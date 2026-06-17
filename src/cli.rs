//! Command-line and environment configuration.

use std::net::IpAddr;
use std::time::Duration;

use clap::Parser;

/// Prometheus exporter for Intel and AMD GPU utilization, memory, power, and
/// thermals. Every flag has a `DRM_EXPORTER_*` environment-variable equivalent
/// for container deployments.
#[derive(Parser, Debug)]
#[command(version = crate::VERSION, about, long_about = None)]
pub struct Args {
    /// Restrict export to these PCI slots (comma-separated, e.g.
    /// `0000:03:00.0`). Default: every DRM GPU discovered.
    #[arg(
        short = 'd',
        long,
        value_delimiter = ',',
        env = "DRM_EXPORTER_DEVICES",
        value_name = "PCI_SLOT"
    )]
    pub devices: Vec<String>,

    /// Address the metrics HTTP server binds to.
    #[arg(short, long, default_value = "0.0.0.0", env = "DRM_EXPORTER_ADDRESS")]
    pub address: IpAddr,

    /// Port the metrics HTTP server listens on.
    #[arg(short, long, default_value_t = 9090, env = "DRM_EXPORTER_PORT")]
    pub port: u16,

    /// Seconds between GPU stat refreshes. Intel engine utilization is sampled
    /// across this window, so keep it at or below the Prometheus scrape
    /// interval.
    #[arg(
        short,
        long,
        default_value_t = 5,
        env = "DRM_EXPORTER_INTERVAL_SECONDS",
        value_name = "SECONDS"
    )]
    pub interval_seconds: u64,

    /// Advanced: extra qmlib driver options as `driver=key=value` (repeatable),
    /// e.g. `i915=power=msr` to force MSR power. The built-in defaults suit most hosts.
    #[arg(long = "driver-option", value_name = "DRIVER=KEY=VALUE")]
    pub driver_options: Vec<String>,
}

impl Args {
    /// The refresh interval as a [`Duration`].
    pub const fn interval(&self) -> Duration {
        Duration::from_secs(self.interval_seconds)
    }
}

#[cfg(test)]
mod tests {
    use super::Args;
    use clap::{CommandFactory, Parser};
    use std::time::Duration;

    #[test]
    fn cli_definition_is_valid() {
        // clap's own lint for conflicting flags, bad defaults, etc.
        Args::command().debug_assert();
    }

    #[test]
    fn defaults_match_the_documented_values() {
        let args = Args::parse_from(["drm-exporter"]);
        assert_eq!(args.address.to_string(), "0.0.0.0");
        assert_eq!(args.port, 9090);
        assert_eq!(args.interval_seconds, 5);
        assert!(args.devices.is_empty());
        assert!(args.driver_options.is_empty());
    }

    #[test]
    fn devices_split_on_commas_and_interval_converts() {
        let args = Args::parse_from([
            "drm-exporter",
            "-d",
            "0000:03:00.0,0000:00:02.0",
            "-i",
            "12",
        ]);
        assert_eq!(args.devices, ["0000:03:00.0", "0000:00:02.0"]);
        assert_eq!(args.interval(), Duration::from_secs(12));
    }
}
