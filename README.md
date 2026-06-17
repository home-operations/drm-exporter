# drm-exporter

A Prometheus exporter for **Intel** and **AMD** GPU metrics — engine
utilization, memory, clock frequency, power, and thermals. It reads the kernel
DRM interfaces through [qmlib](https://github.com/ulissesf/qmassa) (the library
behind [qmassa](https://github.com/ulissesf/qmassa)) and serves the metrics over
HTTP, intended to run as a per-node Kubernetes DaemonSet.

## Supported GPUs

Driver support comes from qmlib (see its
[DRIVERS.md](https://github.com/ulissesf/qmassa/blob/main/qmlib/DRIVERS.md) for
the authoritative, per-kernel-version matrix):

| Driver   | GPUs                              | Engines | Memory | Frequency | Power | Temps / Fans |
| -------- | --------------------------------- | :-----: | :----: | :-------: | :---: | :----------: |
| `i915`   | Older Intel (Gen ≤ 12, e.g. UHD/Iris Xe) | ✅ perf PMU | ✅ | ✅ | ✅ iGPU via PMU/MSR, dGPU via hwmon | ✅ |
| `xe`     | Newer Intel (Xe, Arc, Lunar Lake+) | ✅ perf PMU | ✅ | ✅ | ✅ iGPU via PMU/MSR, dGPU via hwmon | ✅ |
| `amdgpu` | AMD integrated and discrete       | ✅ sysfs | ✅ | ✅ | ✅ dGPU via hwmon | ✅ dGPU via hwmon |

The exporter reads engine utilization from the perf PMU on Intel (`i915`/`xe`)
and from sysfs on AMD. This assumes a current kernel — in particular **Linux
≥ 6.16 for Intel `xe`** (its perf-PMU support landed there); `i915` and `amdgpu`
work on older kernels too. Legacy-kernel fallbacks are intentionally not
supported.

## Metrics

All series are gauges prefixed `drm_`, with a `device` label (the PCI slot, e.g.
`0000:03:00.0`) identifying the GPU:

| Metric | Extra labels | Meaning |
| ------ | ------------ | ------- |
| `drm_info` | `pci_id`, `vendor`, `model`, `revision`, `driver`, `type`, `dev_node` | Constant `1`; carries the GPU's static identity for PromQL joins |
| `drm_memory_used_bytes` / `drm_memory_total_bytes` | `pool` (`smem`/`vram`) | Memory usage by pool (VRAM only on discrete GPUs) |
| `drm_engine_utilization_ratio` | `engine` | Per-engine busy fraction, `0.0`–`1.0` |
| `drm_frequency_hertz` | `domain`, `kind` (`actual`/`max`) | Clock frequency |
| `drm_power_watts` | `domain` (`gpu`/`package`) | Power draw |
| `drm_temperature_celsius` | `sensor` | Temperature |
| `drm_fan_speed_rpm` | `fan` | Fan speed |

A section is simply omitted for a device/driver that does not expose it (e.g.
no `drm_power_watts` where power is unavailable). `GET /health` returns `OK` for
liveness probes; metrics are served on `GET /metrics` (and any other path).

Metrics are instrumented on the OpenTelemetry API and exposed through
`opentelemetry-prometheus` + the `prometheus` crate — the same stack as the
org's other Rust project (kopiur). Set `OTEL_EXPORTER_OTLP_ENDPOINT` to *also*
push metrics over OTLP/gRPC (plaintext, like kopiur); leave it unset for
Prometheus pull only, in which case no async runtime is created (the OTLP path
spins up a small tokio runtime just for the tonic channel when enabled). As a
consequence of the OTel→Prometheus bridge, every series also carries an
`otel_scope_name` label and a `target_info` metric is exposed; neither affects
the `drm_*` queries the bundled dashboard uses.

## Configuration

Every flag has a `DRM_EXPORTER_*` environment variable equivalent:

| Flag | Env | Default | Description |
| ---- | --- | ------- | ----------- |
| `-a`, `--address` | `DRM_EXPORTER_ADDRESS` | `0.0.0.0` | Metrics HTTP bind address |
| `-p`, `--port` | `DRM_EXPORTER_PORT` | `9090` | Metrics HTTP port |
| `-i`, `--interval-seconds` | `DRM_EXPORTER_INTERVAL_SECONDS` | `5` | Seconds between GPU stat refreshes |
| `-d`, `--devices` | `DRM_EXPORTER_DEVICES` | _(all)_ | Comma-separated PCI slots to export |
| `--driver-option` | — | _(driver defaults)_ | Advanced qmlib `driver=key=value` options (repeatable) |

`RUST_LOG` controls log verbosity (default `info`); qmlib's own logs are bridged
into the same output.

## Running

### Container

```bash
docker run --rm \
  --device /dev/dri \
  -v /sys:/sys:ro \
  --cap-add PERFMON --cap-add SYS_RAWIO \
  -p 9090:9090 \
  ghcr.io/home-operations/drm-exporter:latest
```

The image is `distroless/cc` based; the binary links `libudev` (bundled) and
glibc. It is published for **`linux/amd64` only** — the supported GPUs (Intel
`i915`/`xe`, AMD `amdgpu`) are x86_64-only hardware, so there is no arm64 host
for it to run on.

### Helm (Kubernetes)

The chart deploys a DaemonSet with the host mounts and capabilities the exporter
needs. It is published as a Cosign-signed OCI artifact:

```bash
helm install drm-exporter \
  oci://ghcr.io/home-operations/charts/drm-exporter --version <version>
```

See the [chart README](charts/drm-exporter/README.md) for the full
values reference, the privilege/host-access requirements, and how to scope the
DaemonSet to GPU-only nodes (each pod exits if its node has no GPU).

A Grafana dashboard ships with the chart. Enable it with
`monitoring.dashboards.enabled=true` to render a sidecar ConfigMap (labelled
`grafana_dashboard: "1"` for the kube-prometheus-stack sidecar), or set
`monitoring.dashboards.grafanaOperator.enabled=true` for a grafana-operator
`GrafanaDashboard`. It charts engine utilization, memory, frequency, power, and
temperature per GPU, with a `device` selector.

## Privileges

Reading GPU telemetry requires host access:

- **`/dev/dri`** — discover and read GPUs.
- **`/sys`** (read-only) — frequency, memory, AMD engines, hwmon (temp/fan/power).
- **`CAP_PERFMON`** — Intel engine utilization, and the preferred RAPL power path, via the perf PMU.
- **`/dev/cpu`** (read-only) + **`CAP_SYS_RAWIO`** — per-CPU MSRs, for Intel **iGPU package temperature** and the RAPL power fallback. **Requires the host `msr` kernel module** — autoloaded on most distros, but on Talos add it via machine config (`machine.kernel.modules: [{name: msr}]`); without it, Intel iGPU temperature is unavailable.

AMD GPUs and all sysfs-based stats need none of these. The Helm chart's default
runs as root with just those two capabilities (not privileged, read-only root
filesystem).

## Development

The toolchain and tasks are managed by [mise](https://mise.jdx.dev):

```bash
mise run build        # compile (debug)
mise run test         # unit tests
mise run clippy       # lint (warnings = errors)
mise run fmt          # format
mise run ci           # fmt-check + clippy + build + test
mise run helm-lint    # lint + render the chart
mise run helm-test    # chart unit tests (helm-unittest)
mise run helm-docs    # regenerate the chart README + values schema
mise run image        # build the container image locally
```

The GPU collector (qmlib) only compiles on Linux, so on a non-Linux host
`build`/`test` exercise the platform-independent core (the sample model and the
metric recorder); the full binary is built and tested on Linux in CI and in the
container image. Building on Linux requires `pkg-config` and `libudev` headers
(`apt-get install pkg-config libudev-dev`).

## License

[AGPL-3.0-only](LICENSE).
