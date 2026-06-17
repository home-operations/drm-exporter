//! GPU stats collection.
//!
//! The real collector wraps `qmlib`'s `DrmDevices`, which reads the kernel DRM
//! interfaces (udev, the perf PMU, and sysfs) and only exists on Linux. It is
//! compiled behind `cfg(target_os = "linux")`; a stub stands in elsewhere so
//! the crate (and the platform-independent core it shares) still builds and
//! tests on any host.
//!
//! The unit conversions and the discrete-VRAM policy — the only logic with a
//! way to be wrong — live in [`convert`], which is platform-independent and
//! unit-tested directly. The Linux glue is then a thin, total field mapping.

/// Collector configuration, built from the CLI. Platform-independent so the
/// CLI can populate it unconditionally; only the Linux collector reads it (off
/// Linux the stub ignores it, hence the targeted dead-code allow).
#[derive(Debug)]
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub struct Options {
    /// Restrict to these PCI slots; empty means every DRM GPU.
    pub devices: Vec<String>,
    /// Extra `driver=key=value` qmlib driver options.
    pub driver_options: Vec<String>,
}

/// Pure conversions from raw qmlib readings to the Prometheus base units used
/// by [`crate::sample::GpuSample`]. Free of any qmlib type so they run anywhere
/// (off Linux only the tests call them, hence the targeted dead-code allow).
#[cfg_attr(not(target_os = "linux"), allow(dead_code))]
pub mod convert {
    use crate::sample::MemoryPool;

    /// qmlib reports clock frequencies in MHz; Prometheus wants hertz.
    pub fn mhz_to_hertz(mhz: u64) -> f64 {
        mhz as f64 * 1e6
    }

    /// qmlib reports engine utilization as a 0..100 percentage; the
    /// Prometheus-idiomatic form is a 0..1 ratio.
    pub fn percent_to_ratio(percent: f64) -> f64 {
        percent / 100.0
    }

    /// The memory pools to export: system memory always, plus VRAM only for
    /// discrete GPUs. Integrated GPUs alias system memory and report no
    /// meaningful VRAM, so emitting a VRAM pool for them would be misleading.
    pub fn memory_pools(
        smem_used: f64,
        smem_total: f64,
        vram_used: f64,
        vram_total: f64,
        discrete: bool,
    ) -> Vec<MemoryPool> {
        let mut pools = vec![MemoryPool {
            pool: "smem".into(),
            used_bytes: smem_used,
            total_bytes: smem_total,
        }];
        if discrete {
            pools.push(MemoryPool {
                pool: "vram".into(),
                used_bytes: vram_used,
                total_bytes: vram_total,
            });
        }
        pools
    }
}

// Re-export the platform-appropriate collector under one name.
#[cfg(target_os = "linux")]
pub use linux::Collector;
#[cfg(not(target_os = "linux"))]
pub use stub::Collector;

#[cfg(target_os = "linux")]
mod linux {
    use std::collections::HashMap;

    use anyhow::{Context, Result, bail};
    use qmlib::drm_devices::{DrmDeviceInfo, DrmDevices};

    use super::{Options, convert};
    use crate::sample::{EngineUtilization, Fan, Frequency, GpuSample, PowerDraw, Temperature};

    /// Holds the discovered DRM GPUs and re-reads their stats on each collect.
    pub struct Collector {
        devices: DrmDevices,
    }

    impl Collector {
        /// Discover the DRM GPUs matching `opts`; errors if none are found.
        pub fn new(opts: &Options) -> Result<Self> {
            let slots: Vec<&str> = opts.devices.iter().map(String::as_str).collect();

            // Intel (i915/xe) take engine usage from the perf PMU, AMD from
            // sysfs — the device-level sources that work on current kernels.
            let mut drv_opts: HashMap<&str, Vec<&str>> = HashMap::new();
            drv_opts.insert("xe", vec!["engines=pmu"]);
            drv_opts.insert("i915", vec!["engines=pmu"]);
            drv_opts.insert("amdgpu", vec!["engines=sysfs"]);
            // `driver=key=value` -> drv_opts["driver"] += "key=value" (qmlib
            // parses the remainder), matching qmmd's `-o` handling.
            for opt in &opts.driver_options {
                if let Some((drv, kv)) = opt.split_once('=') {
                    drv_opts.entry(drv).or_default().push(kv);
                }
            }

            let devices =
                DrmDevices::find_devices(&slots, &drv_opts).context("discovering DRM devices")?;
            if devices.is_empty() {
                bail!("no DRM GPU devices found (is /dev/dri present and /sys mounted?)");
            }

            Ok(Self { devices })
        }

        /// Refresh every device and snapshot those with a live kernel driver.
        pub fn collect(&mut self) -> Result<Vec<GpuSample>> {
            self.devices
                .refresh()
                .context("refreshing GPU device stats")?;

            Ok(self
                .devices
                .devices()
                .into_iter()
                .filter_map(|slot| self.devices.device_info(slot))
                .filter(|di| di.has_driver())
                .map(sample_from)
                .collect())
        }
    }

    /// Project a qmlib device snapshot onto the platform-independent
    /// [`GpuSample`], delegating the unit/policy decisions to [`convert`].
    fn sample_from(di: &DrmDeviceInfo) -> GpuSample {
        let memory = di
            .mem_info
            .as_ref()
            .map(|mi| {
                convert::memory_pools(
                    mi.smem_used as f64,
                    mi.smem_total as f64,
                    mi.vram_used as f64,
                    mi.vram_total as f64,
                    di.dev_type.is_discrete(),
                )
            })
            .unwrap_or_default();

        let engines = di
            .engines()
            .into_iter()
            .map(|engine| EngineUtilization {
                ratio: convert::percent_to_ratio(di.eng_utilization(&engine)),
                engine,
            })
            .collect();

        // freq_limits and freqs are positional, one entry per frequency domain.
        let frequencies = di
            .freq_limits
            .iter()
            .zip(&di.freqs)
            .map(|(limits, freq)| Frequency {
                domain: limits.name.clone(),
                actual_hertz: convert::mhz_to_hertz(freq.act_freq),
                max_hertz: convert::mhz_to_hertz(limits.maximum),
            })
            .collect();

        let power = di
            .power
            .as_ref()
            .map(|p| {
                vec![
                    PowerDraw {
                        domain: "gpu".into(),
                        watts: p.gpu_cur_power,
                    },
                    PowerDraw {
                        domain: "package".into(),
                        watts: p.pkg_cur_power,
                    },
                ]
            })
            .unwrap_or_default();

        let temperatures = di
            .temps
            .iter()
            .map(|t| Temperature {
                sensor: t.name.clone(),
                celsius: t.temp,
            })
            .collect();

        let fans = di
            .fans
            .iter()
            .map(|f| Fan {
                fan: f.name.clone(),
                rpm: f.speed as f64,
            })
            .collect();

        GpuSample {
            pci_dev: di.pci_dev.clone(),
            vendor_id: di.vendor_id.clone(),
            device_id: di.device_id.clone(),
            vendor: di.vendor.clone(),
            model: di.device.clone(),
            revision: di.revision.clone(),
            driver: di.drv_name.clone(),
            dev_type: di.dev_type.to_string(),
            dev_nodes: di.dev_nodes.iter().map(|n| n.devnode.clone()).collect(),
            memory,
            engines,
            frequencies,
            power,
            temperatures,
            fans,
        }
    }
}

#[cfg(not(target_os = "linux"))]
mod stub {
    use anyhow::{Result, bail};

    use super::Options;
    use crate::sample::GpuSample;

    /// Non-Linux placeholder. GPU collection needs the Linux DRM interfaces, so
    /// construction fails fast; the core metric logic stays testable regardless.
    pub struct Collector;

    impl Collector {
        pub fn new(_opts: &Options) -> Result<Self> {
            bail!(
                "GPU collection is only supported on Linux (this binary was built for a non-Linux target)"
            )
        }

        pub fn collect(&mut self) -> Result<Vec<GpuSample>> {
            Ok(Vec::new()) // unreachable: `new` never returns Ok off Linux
        }
    }
}

#[cfg(test)]
mod tests {
    use super::convert;
    use crate::sample::MemoryPool;

    #[test]
    fn mhz_to_hertz_scales_by_one_million() {
        assert_eq!(convert::mhz_to_hertz(2500), 2_500_000_000.0);
        assert_eq!(convert::mhz_to_hertz(0), 0.0);
    }

    #[test]
    fn percent_to_ratio_maps_0_100_to_0_1() {
        assert_eq!(convert::percent_to_ratio(0.0), 0.0);
        assert_eq!(convert::percent_to_ratio(42.0), 0.42);
        assert_eq!(convert::percent_to_ratio(100.0), 1.0);
    }

    #[test]
    fn discrete_gpu_gets_smem_and_vram() {
        assert_eq!(
            convert::memory_pools(10.0, 100.0, 20.0, 200.0, true),
            vec![
                MemoryPool {
                    pool: "smem".into(),
                    used_bytes: 10.0,
                    total_bytes: 100.0,
                },
                MemoryPool {
                    pool: "vram".into(),
                    used_bytes: 20.0,
                    total_bytes: 200.0,
                },
            ],
        );
    }

    #[test]
    fn integrated_gpu_gets_smem_only() {
        assert_eq!(
            convert::memory_pools(10.0, 100.0, 0.0, 0.0, false),
            vec![MemoryPool {
                pool: "smem".into(),
                used_bytes: 10.0,
                total_bytes: 100.0,
            }],
        );
    }
}
