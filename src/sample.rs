//! Platform-independent snapshot model for a single GPU.
//!
//! [`GpuSample`] is the boundary type between the collector (which reads the
//! kernel's DRM interfaces through `qmlib`, Linux-only) and the metric recorder
//! (which translates samples into Prometheus series). Keeping it free of any
//! `qmlib` type lets the metric-emission logic — the part most prone to
//! regressions — be exercised on any platform, without GPU hardware.
//!
//! Values are pre-converted to the Prometheus base units the recorder emits
//! (bytes, hertz, watts), so the recorder stays a dumb, total translation.

/// A point-in-time snapshot of one GPU's identity and readings.
#[derive(Debug, PartialEq)]
pub struct GpuSample {
    /// PCI slot, e.g. `0000:03:00.0`. Used as the `device` label on every
    /// metric — the stable identifier for a GPU across scrapes.
    pub pci_dev: String,
    /// Four-hex-digit PCI vendor id, e.g. `8086` (Intel) or `1002` (AMD).
    pub vendor_id: String,
    /// Four-hex-digit PCI device id.
    pub device_id: String,
    /// Human-readable vendor name, or the vendor id when the hwdb lookup is
    /// unavailable (e.g. no hwdb in the runtime image).
    pub vendor: String,
    /// Human-readable model name, or the device id when the lookup is
    /// unavailable.
    pub model: String,
    /// PCI revision.
    pub revision: String,
    /// Kernel DRM driver backing the device: `i915`, `xe`, or `amdgpu`.
    pub driver: String,
    /// Device class, e.g. `Integrated`, `Discrete`, or `Discrete (PF)`.
    pub dev_type: String,
    /// DRM device nodes for this GPU, e.g. `/dev/dri/card0`.
    pub dev_nodes: Vec<String>,
    /// Memory pools: always system memory, plus VRAM for discrete GPUs.
    pub memory: Vec<MemoryPool>,
    /// Per-engine busy fraction.
    pub engines: Vec<EngineUtilization>,
    /// Per-domain clock frequencies.
    pub frequencies: Vec<Frequency>,
    /// Per-domain power draw.
    pub power: Vec<PowerDraw>,
    /// Temperature sensors.
    pub temperatures: Vec<Temperature>,
    /// Fans.
    pub fans: Vec<Fan>,
}

/// A GPU memory pool and its usage, in bytes.
#[derive(Debug, PartialEq)]
pub struct MemoryPool {
    /// Pool name: `smem` (system memory) or `vram` (discrete video memory).
    pub pool: String,
    pub used_bytes: f64,
    pub total_bytes: f64,
}

/// A single GPU engine's busy fraction.
#[derive(Debug, PartialEq)]
pub struct EngineUtilization {
    /// Engine name as reported by the driver, e.g. `rcs`/`bcs`/`vcs` (Intel) or
    /// `gfx`/`compute` (AMD).
    pub engine: String,
    /// Busy fraction in the range `0.0..=1.0`.
    pub ratio: f64,
}

/// A clock-frequency domain's actual and maximum frequency, in hertz.
#[derive(Debug, PartialEq)]
pub struct Frequency {
    /// Frequency domain id, e.g. `gt0`.
    pub domain: String,
    pub actual_hertz: f64,
    pub max_hertz: f64,
}

/// Power draw for one domain, in watts.
#[derive(Debug, PartialEq)]
pub struct PowerDraw {
    /// Power domain: `gpu` (the GPU rail) or `package` (the whole package).
    pub domain: String,
    pub watts: f64,
}

/// A temperature sensor reading, in degrees Celsius.
#[derive(Debug, PartialEq)]
pub struct Temperature {
    pub sensor: String,
    pub celsius: f64,
}

/// A fan's speed, in RPM.
#[derive(Debug, PartialEq)]
pub struct Fan {
    pub fan: String,
    pub rpm: f64,
}
