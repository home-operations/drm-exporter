//! Translation of [`GpuSample`]s into OpenTelemetry metric instruments.
//!
//! Every metric name carries the `drm_` prefix and Prometheus base units encoded
//! in the name (`_bytes`, `_hertz`, `_watts`, `_celsius`, `_rpm`); engine
//! utilization is a `0.0..=1.0` ratio (`_ratio`). Instruments are left *without*
//! an OTel unit so `opentelemetry-prometheus` does not append a unit suffix —
//! the names are exposed verbatim. Static device attributes ride on a constant
//! `drm_info` series (the kube-state-metrics "info" pattern).
//!
//! [`Metrics::record`] is a total function of its input: it records one series
//! per reading present in the sample, so a missing section (e.g. no fans) simply
//! emits nothing for it.

use opentelemetry::KeyValue;
use opentelemetry::metrics::{Gauge, Meter};

use crate::sample::GpuSample;

/// The exported gauges. All series share a `device` (PCI slot) attribute.
pub struct Metrics {
    info: Gauge<f64>,
    memory_used: Gauge<f64>,
    memory_total: Gauge<f64>,
    engine: Gauge<f64>,
    frequency: Gauge<f64>,
    power: Gauge<f64>,
    temperature: Gauge<f64>,
    fan: Gauge<f64>,
}

impl Metrics {
    /// Create the instruments from `meter`.
    pub fn new(meter: &Meter) -> Self {
        Self {
            info: meter
                .f64_gauge("drm_info")
                .with_description(
                    "Constant 1 series labeled with GPU identity (vendor, model, driver, PCI id, device node).",
                )
                .build(),
            memory_used: meter
                .f64_gauge("drm_memory_used_bytes")
                .with_description("GPU memory in use, by pool.")
                .build(),
            memory_total: meter
                .f64_gauge("drm_memory_total_bytes")
                .with_description("Total GPU memory, by pool.")
                .build(),
            engine: meter
                .f64_gauge("drm_engine_utilization_ratio")
                .with_description("GPU engine busy fraction (0-1), by engine.")
                .build(),
            frequency: meter
                .f64_gauge("drm_frequency_hertz")
                .with_description("GPU clock frequency in hertz, by domain and kind (actual/max).")
                .build(),
            power: meter
                .f64_gauge("drm_power_watts")
                .with_description("GPU power draw in watts, by domain (gpu/package).")
                .build(),
            temperature: meter
                .f64_gauge("drm_temperature_celsius")
                .with_description("GPU temperature in celsius, by sensor.")
                .build(),
            fan: meter
                .f64_gauge("drm_fan_speed_rpm")
                .with_description("GPU fan speed in RPM, by fan.")
                .build(),
        }
    }

    /// Record the current value of every reading in each sample.
    pub fn record(&self, samples: &[GpuSample]) {
        for s in samples {
            self.record_one(s);
        }
    }

    fn record_one(&self, s: &GpuSample) {
        // The `device` attribute is on every series; rebuild per use since
        // `record` borrows the attribute slice.
        let device = || KeyValue::new("device", s.pci_dev.clone());

        // One constant-1 info series per device node, carrying the static identity.
        let pci_id = format!("{}:{}", s.vendor_id, s.device_id);
        for node in &s.dev_nodes {
            self.info.record(
                1.0,
                &[
                    device(),
                    KeyValue::new("pci_id", pci_id.clone()),
                    KeyValue::new("vendor", s.vendor.clone()),
                    KeyValue::new("model", s.model.clone()),
                    KeyValue::new("revision", s.revision.clone()),
                    KeyValue::new("driver", s.driver.clone()),
                    KeyValue::new("type", s.dev_type.clone()),
                    KeyValue::new("dev_node", node.clone()),
                ],
            );
        }

        for m in &s.memory {
            let pool = || KeyValue::new("pool", m.pool.clone());
            self.memory_used.record(m.used_bytes, &[device(), pool()]);
            self.memory_total.record(m.total_bytes, &[device(), pool()]);
        }

        for e in &s.engines {
            self.engine.record(
                e.ratio,
                &[device(), KeyValue::new("engine", e.engine.clone())],
            );
        }

        for f in &s.frequencies {
            let domain = || KeyValue::new("domain", f.domain.clone());
            self.frequency.record(
                f.actual_hertz,
                &[device(), domain(), KeyValue::new("kind", "actual")],
            );
            self.frequency.record(
                f.max_hertz,
                &[device(), domain(), KeyValue::new("kind", "max")],
            );
        }

        for p in &s.power {
            self.power.record(
                p.watts,
                &[device(), KeyValue::new("domain", p.domain.clone())],
            );
        }

        for t in &s.temperatures {
            self.temperature.record(
                t.celsius,
                &[device(), KeyValue::new("sensor", t.sensor.clone())],
            );
        }

        for f in &s.fans {
            self.fan
                .record(f.rpm, &[device(), KeyValue::new("fan", f.fan.clone())]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sample::{EngineUtilization, Fan, Frequency, MemoryPool, PowerDraw, Temperature};
    use crate::telemetry::{Telemetry, render};

    /// Record `samples` through a fresh OTel meter + Prometheus exporter and
    /// return the rendered exposition. Each call builds its own provider, so
    /// tests stay isolated and parallel-safe.
    fn render_samples(samples: &[GpuSample]) -> String {
        let telemetry = Telemetry::new().expect("build telemetry");
        let metrics = Metrics::new(telemetry.meter());
        metrics.record(samples);
        String::from_utf8(render(&telemetry.registry())).expect("utf8 exposition")
    }

    /// Value of the series named `name` whose label set contains every
    /// `(key, value)` in `labels`. Robust to label ordering and to the extra
    /// `otel_scope_*` labels the OTel→Prometheus bridge adds.
    fn series_value(out: &str, name: &str, labels: &[(&str, &str)]) -> Option<f64> {
        out.lines().find_map(|line| {
            let rest = line.strip_prefix(name)?;
            if !rest.starts_with('{') {
                return None;
            }
            if !labels
                .iter()
                .all(|(k, v)| rest.contains(&format!("{k}=\"{v}\"")))
            {
                return None;
            }
            rest.rsplit_once(' ')?.1.trim().parse::<f64>().ok()
        })
    }

    /// How many series share the given metric name.
    fn series_count(out: &str, name: &str) -> usize {
        out.lines()
            .filter(|l| l.strip_prefix(name).is_some_and(|r| r.starts_with('{')))
            .count()
    }

    fn discrete_gpu() -> GpuSample {
        GpuSample {
            pci_dev: "0000:03:00.0".into(),
            vendor_id: "1002".into(),
            device_id: "744c".into(),
            vendor: "Advanced Micro Devices, Inc. [AMD/ATI]".into(),
            model: "Navi 31".into(),
            revision: "c8".into(),
            driver: "amdgpu".into(),
            dev_type: "Discrete".into(),
            dev_nodes: vec!["/dev/dri/card0".into(), "/dev/dri/renderD128".into()],
            memory: vec![
                MemoryPool {
                    pool: "smem".into(),
                    used_bytes: 1024.0,
                    total_bytes: 8192.0,
                },
                MemoryPool {
                    pool: "vram".into(),
                    used_bytes: 2_000_000.0,
                    total_bytes: 24_000_000_000.0,
                },
            ],
            engines: vec![
                EngineUtilization {
                    engine: "gfx".into(),
                    ratio: 0.42,
                },
                EngineUtilization {
                    engine: "compute".into(),
                    ratio: 0.0,
                },
            ],
            frequencies: vec![Frequency {
                domain: "gt0".into(),
                actual_hertz: 2_400_000_000.0,
                max_hertz: 2_500_000_000.0,
            }],
            power: vec![
                PowerDraw {
                    domain: "gpu".into(),
                    watts: 35.5,
                },
                PowerDraw {
                    domain: "package".into(),
                    watts: 60.0,
                },
            ],
            temperatures: vec![Temperature {
                sensor: "edge".into(),
                celsius: 48.0,
            }],
            fans: vec![Fan {
                fan: "fan1".into(),
                rpm: 1200.0,
            }],
        }
    }

    fn integrated_igpu() -> GpuSample {
        GpuSample {
            pci_dev: "0000:00:02.0".into(),
            vendor_id: "8086".into(),
            device_id: "9a49".into(),
            vendor: "Intel Corporation".into(),
            model: "TigerLake-LP GT2 [Iris Xe Graphics]".into(),
            revision: "01".into(),
            driver: "i915".into(),
            dev_type: "Integrated".into(),
            dev_nodes: vec!["/dev/dri/card1".into()],
            memory: vec![MemoryPool {
                pool: "smem".into(),
                used_bytes: 500.0,
                total_bytes: 16_000_000_000.0,
            }],
            engines: vec![EngineUtilization {
                engine: "rcs".into(),
                ratio: 0.10,
            }],
            frequencies: vec![Frequency {
                domain: "gt0".into(),
                actual_hertz: 900_000_000.0,
                max_hertz: 1_300_000_000.0,
            }],
            power: vec![],
            temperatures: vec![],
            fans: vec![],
        }
    }

    #[test]
    fn info_series_is_one_per_device_node() {
        let out = render_samples(&[discrete_gpu()]);
        assert_eq!(series_count(&out, "drm_info"), 2);
        assert_eq!(
            series_value(
                &out,
                "drm_info",
                &[("device", "0000:03:00.0"), ("dev_node", "/dev/dri/card0")]
            ),
            Some(1.0)
        );
    }

    #[test]
    fn info_carries_identity_labels() {
        let out = render_samples(&[discrete_gpu()]);
        assert!(out.contains("pci_id=\"1002:744c\""));
        assert!(out.contains("driver=\"amdgpu\""));
        assert!(out.contains("model=\"Navi 31\""));
        assert!(out.contains("type=\"Discrete\""));
    }

    #[test]
    fn memory_used_and_total_per_pool() {
        let out = render_samples(&[discrete_gpu()]);
        assert_eq!(
            series_value(&out, "drm_memory_used_bytes", &[("pool", "vram")]),
            Some(2_000_000.0)
        );
        assert_eq!(
            series_value(&out, "drm_memory_total_bytes", &[("pool", "vram")]),
            Some(24_000_000_000.0)
        );
        assert_eq!(
            series_value(&out, "drm_memory_used_bytes", &[("pool", "smem")]),
            Some(1024.0)
        );
    }

    #[test]
    fn engine_ratio_including_zero() {
        let out = render_samples(&[discrete_gpu()]);
        assert_eq!(
            series_value(&out, "drm_engine_utilization_ratio", &[("engine", "gfx")]),
            Some(0.42)
        );
        assert_eq!(
            series_value(
                &out,
                "drm_engine_utilization_ratio",
                &[("engine", "compute")]
            ),
            Some(0.0)
        );
    }

    #[test]
    fn frequency_splits_into_actual_and_max() {
        let out = render_samples(&[discrete_gpu()]);
        assert_eq!(
            series_value(
                &out,
                "drm_frequency_hertz",
                &[("domain", "gt0"), ("kind", "actual")]
            ),
            Some(2_400_000_000.0)
        );
        assert_eq!(
            series_value(
                &out,
                "drm_frequency_hertz",
                &[("domain", "gt0"), ("kind", "max")]
            ),
            Some(2_500_000_000.0)
        );
        assert_eq!(series_count(&out, "drm_frequency_hertz"), 2);
    }

    #[test]
    fn power_temp_and_fan() {
        let out = render_samples(&[discrete_gpu()]);
        assert_eq!(
            series_value(&out, "drm_power_watts", &[("domain", "package")]),
            Some(60.0)
        );
        assert_eq!(
            series_value(&out, "drm_temperature_celsius", &[("sensor", "edge")]),
            Some(48.0)
        );
        assert_eq!(
            series_value(&out, "drm_fan_speed_rpm", &[("fan", "fan1")]),
            Some(1200.0)
        );
    }

    #[test]
    fn absent_sections_emit_no_series() {
        let out = render_samples(&[integrated_igpu()]);
        assert_eq!(series_count(&out, "drm_power_watts"), 0);
        assert_eq!(series_count(&out, "drm_temperature_celsius"), 0);
        assert_eq!(series_count(&out, "drm_fan_speed_rpm"), 0);
        assert_eq!(series_count(&out, "drm_memory_used_bytes"), 1);
    }

    #[test]
    fn multiple_devices_are_disambiguated_by_device_label() {
        let out = render_samples(&[discrete_gpu(), integrated_igpu()]);
        assert_eq!(
            series_value(
                &out,
                "drm_engine_utilization_ratio",
                &[("device", "0000:03:00.0"), ("engine", "gfx")]
            ),
            Some(0.42)
        );
        assert_eq!(
            series_value(
                &out,
                "drm_engine_utilization_ratio",
                &[("device", "0000:00:02.0"), ("engine", "rcs")]
            ),
            Some(0.10)
        );
        assert_eq!(series_count(&out, "drm_info"), 3);
    }

    #[test]
    fn empty_input_records_nothing() {
        let out = render_samples(&[]);
        assert_eq!(series_count(&out, "drm_info"), 0);
        assert_eq!(series_count(&out, "drm_memory_used_bytes"), 0);
    }
}
