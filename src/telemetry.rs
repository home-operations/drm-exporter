//! OpenTelemetry metrics wiring.
//!
//! Mirrors the org's other Rust project (kopiur): instruments are created on the
//! OpenTelemetry metrics API and exposed as a Prometheus pull endpoint via
//! `opentelemetry-prometheus`, backed by a `prometheus::Registry`. Optionally —
//! when `OTEL_EXPORTER_OTLP_ENDPOINT` is set — metrics are *also* pushed over
//! OTLP/gRPC (plaintext, like kopiur).
//!
//! The default (no OTLP endpoint) path is fully synchronous. OTLP/gRPC needs a
//! tokio runtime for its tonic channel, so [`Telemetry`] creates a small one
//! **only** when OTLP is configured and keeps it alive for the channel's
//! lifetime; the metrics SDK's `PeriodicReader` runs the export on its own thread.
//!
//! `opentelemetry-prometheus` adds an `otel_scope_name` label to every series and
//! a `target_info` metric; these are harmless to the `drm_*{device=…}` queries the
//! bundled dashboard uses, and match kopiur's exposition.

use anyhow::{Context, Result};
use opentelemetry::metrics::{Meter, MeterProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::Resource;
use opentelemetry_sdk::metrics::{PeriodicReader, SdkMeterProvider};
use prometheus::{Encoder, Registry, TextEncoder};

/// Owns the meter provider and the Prometheus registry it feeds. Instruments are
/// created from [`Telemetry::meter`]; the `/metrics` endpoint serves [`render`]
/// over [`Telemetry::registry`].
pub struct Telemetry {
    provider: SdkMeterProvider,
    registry: Registry,
    meter: Meter,
    // Kept alive for the OTLP tonic channel it backs; `None` when OTLP is off.
    // Dropped after the provider on shutdown, so the final push can complete.
    _otlp_runtime: Option<tokio::runtime::Runtime>,
}

impl Telemetry {
    /// Build a meter provider with the Prometheus pull exporter, plus an OTLP/gRPC
    /// push reader when `OTEL_EXPORTER_OTLP_ENDPOINT` is set.
    pub fn new() -> Result<Self> {
        Self::build(otlp_endpoint())
    }

    fn build(otlp: Option<String>) -> Result<Self> {
        let registry = Registry::new();
        let prometheus = opentelemetry_prometheus::exporter()
            .with_registry(registry.clone())
            .build()
            .context("building the Prometheus exporter")?;

        let mut builder = SdkMeterProvider::builder()
            .with_reader(prometheus)
            .with_resource(
                Resource::builder()
                    .with_service_name(env!("CARGO_PKG_NAME"))
                    .build(),
            );

        // OTLP/gRPC push, when configured. The tonic channel captures the tokio
        // runtime it is built under, so build it inside `runtime.enter()` and keep
        // the runtime alive; the PeriodicReader's own thread drives exports on it.
        let mut otlp_runtime = None;
        if let Some(endpoint) = otlp {
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()
                .context("building the OTLP export runtime")?;
            let reader = {
                let _guard = runtime.enter();
                let exporter = opentelemetry_otlp::MetricExporter::builder()
                    .with_tonic()
                    .with_endpoint(endpoint.clone())
                    .build()
                    .context("building the OTLP metric exporter")?;
                PeriodicReader::builder(exporter).build()
            };
            builder = builder.with_reader(reader);
            otlp_runtime = Some(runtime);
            tracing::info!(otlp_endpoint = %endpoint, "OTLP metric push enabled");
        }

        let provider = builder.build();
        let meter = provider.meter(env!("CARGO_PKG_NAME"));
        Ok(Self {
            provider,
            registry,
            meter,
            _otlp_runtime: otlp_runtime,
        })
    }

    /// The meter for creating instruments.
    pub const fn meter(&self) -> &Meter {
        &self.meter
    }

    /// A handle to the registry. Cheap to clone (internally reference-counted), so
    /// the HTTP server thread can `gather()` the series the meter feeds.
    pub fn registry(&self) -> Registry {
        self.registry.clone()
    }

    /// Flush and shut down the meter provider (drains a final OTLP push).
    pub fn shutdown(&self) {
        let _ = self.provider.shutdown();
    }
}

/// The OTLP endpoint from the standard `OTEL_EXPORTER_OTLP_ENDPOINT` env var, if
/// set and non-empty. Absent → OTLP push is disabled (Prometheus pull only).
fn otlp_endpoint() -> Option<String> {
    std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .ok()
        .map(|v| v.trim().to_string())
        .filter(|v| !v.is_empty())
}

/// Render the Prometheus text exposition for `registry`.
pub fn render(registry: &Registry) -> Vec<u8> {
    let mut buf = Vec::new();
    // Encoding only fails on a broken writer; a Vec never errors.
    let _ = TextEncoder::new().encode(&registry.gather(), &mut buf);
    buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_prometheus_only_without_otlp() {
        let t = Telemetry::build(None).expect("build offline");
        t.meter().f64_gauge("drm_probe").build().record(1.0, &[]);
        let text = String::from_utf8(render(&t.registry())).expect("utf8");
        assert!(text.contains("drm_probe"), "missing series: {text}");
    }

    #[test]
    fn builds_with_otlp_endpoint_and_still_serves_prometheus() {
        // A bogus endpoint is fine: the tonic channel connects lazily, so the
        // build succeeds and the Prometheus pull path is unaffected. This checks
        // the OTLP reader wires in (and the runtime is created) without a collector.
        let t = Telemetry::build(Some("http://127.0.0.1:4317".into())).expect("build with otlp");
        t.meter().f64_gauge("drm_probe").build().record(2.0, &[]);
        let text = String::from_utf8(render(&t.registry())).expect("utf8");
        assert!(text.contains("drm_probe"), "missing series: {text}");
        t.shutdown();
    }
}
