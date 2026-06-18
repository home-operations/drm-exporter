//! drm-exporter — a Prometheus exporter for Intel and AMD GPU metrics.
//!
//! Discovers DRM GPUs through `qmlib`, then on a fixed interval refreshes their
//! stats and records them as OpenTelemetry gauges, exposed on a Prometheus
//! `/metrics` endpoint (plus `/health`). See [`metrics`] for the exported series,
//! [`telemetry`] for the OTel→Prometheus wiring, and [`cli`] for the flags.

mod cli;
mod collector;
mod metrics;
mod sample;
mod telemetry;

use std::net::SocketAddr;
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use clap::Parser;
use tracing::{info, warn};
use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;

use crate::cli::Args;
use crate::collector::{Collector, Options};
use crate::metrics::Metrics;
use crate::telemetry::{Telemetry, render};

// Use mimalloc as the global allocator, matching the org's other Rust service
// (kopiur). This is a long-running per-node DaemonSet; glibc's malloc is slow to
// hand freed pages back to the OS, so even modest periodic allocation can let RSS
// drift above the working set, while mimalloc decays dirty pages back. The gain
// is smaller than on an allocation-heavy server, but it is near-free here:
// mimalloc links statically (nothing added to the distroless library closure) and
// builds with the C compiler already present in the image.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

/// Version stamped at build time — the release image passes `DRM_EXPORTER_VERSION`
/// (the release tag) as a build env var; local builds fall back to the crate
/// version.
pub const VERSION: &str = match option_env!("DRM_EXPORTER_VERSION") {
    Some(v) => v,
    None => env!("CARGO_PKG_VERSION"),
};

fn main() -> Result<()> {
    let args = Args::parse();
    init_tracing();

    info!(version = VERSION, "starting drm-exporter");

    // Build the OTel meter + Prometheus exporter, then serve /metrics + /health
    // on a background thread (the refresh loop owns the main thread).
    let telemetry = Telemetry::new().context("initializing metrics")?;
    let metrics = Metrics::new(telemetry.meter());

    let addr = SocketAddr::new(args.address, args.port);
    let server = tiny_http::Server::http(addr)
        .map_err(|e| anyhow!("binding the metrics HTTP listener on {addr}: {e}"))?;
    let registry = telemetry.registry();
    thread::spawn(move || serve(server, &registry));

    let mut collector = Collector::new(&Options {
        devices: args.devices.clone(),
        driver_options: args.driver_options.clone(),
    })
    .context("initializing the GPU collector")?;

    info!(%addr, interval_seconds = args.interval_seconds, "serving GPU metrics on /metrics");

    run(&mut collector, &metrics, args.interval())?;
    telemetry.shutdown();
    Ok(())
}

/// Serve the Prometheus exposition on every request except `/health`, a
/// lightweight liveness endpoint that returns `OK`. Runs until the process exits.
fn serve(server: tiny_http::Server, registry: &prometheus::Registry) {
    for request in server.incoming_requests() {
        let response = if request.url() == "/health" {
            tiny_http::Response::from_string("OK")
        } else {
            let header = tiny_http::Header::from_bytes(
                &b"Content-Type"[..],
                &b"text/plain; version=0.0.4"[..],
            )
            .expect("static content-type header");
            tiny_http::Response::from_data(render(registry)).with_header(header)
        };
        let _ = request.respond(response);
    }
}

/// Refresh-and-record loop, exiting cleanly on SIGINT/SIGTERM so Kubernetes sees
/// a graceful shutdown.
fn run(collector: &mut Collector, metrics: &Metrics, interval: Duration) -> Result<()> {
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(()); // a failed send just means we already signalled
    })
    .context("installing the termination signal handler")?;

    loop {
        match collector.collect() {
            Ok(samples) => metrics.record(&samples),
            // A transient read error (e.g. a perf-counter hiccup) shouldn't kill
            // the exporter; log it and keep serving the last good values.
            Err(err) => warn!(error = %err, "GPU stats refresh failed; serving stale metrics"),
        }

        // Wait one interval, waking immediately on a signal. A timeout means
        // it's time for the next refresh; anything else (a signal, or the
        // sender being dropped) ends the loop.
        if !matches!(rx.recv_timeout(interval), Err(RecvTimeoutError::Timeout)) {
            break;
        }
    }

    info!("received termination signal, shutting down");
    Ok(())
}

/// Initialize tracing. `RUST_LOG` controls verbosity, defaulting to `info` and
/// keeping that floor even when `RUST_LOG` targets a specific module (so our own
/// logs survive e.g. `RUST_LOG=qmlib=debug`); invalid directives are ignored
/// rather than aborting startup. The fmt subscriber's `tracing-log` bridge also
/// captures `qmlib`'s `log` records.
fn init_tracing() {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy();
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
}
