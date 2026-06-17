# drm-exporter

![Version: 0.0.0](https://img.shields.io/badge/Version-0.0.0-informational?style=flat-square) ![Type: application](https://img.shields.io/badge/Type-application-informational?style=flat-square) ![AppVersion: 0.0.0](https://img.shields.io/badge/AppVersion-0.0.0-informational?style=flat-square)

A Prometheus exporter for Intel and AMD GPU metrics — utilization, memory,
frequency, power, and thermals — deployed as a per-node DaemonSet. It reads the
kernel DRM interfaces via [qmlib](https://github.com/ulissesf/qmassa) and serves
metrics on `/metrics`, with a `/health` endpoint for probes.

**Homepage:** <https://github.com/home-operations/drm-exporter>

## Installing

The chart is published as a Cosign-signed OCI artifact:

```bash
helm install drm-exporter oci://ghcr.io/home-operations/charts/drm-exporter --version <version>
```

Verify the signature (keyless, GitHub Actions OIDC):

```bash
cosign verify ghcr.io/home-operations/charts/drm-exporter:<version> \
  --certificate-identity-regexp '^https://github.com/home-operations/drm-exporter/\.github/workflows/release\.yaml@.*$' \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com
```

## Host access and privileges

Reading GPU telemetry needs host access, so the chart mounts host paths and
grants a narrow set of capabilities (`hostAccess` + `securityContext`):

- **`/dev/dri`** — the DRM device nodes, to discover and read GPUs.
- **`/sys`** (read-only) — frequency, memory, AMD engine, and hwmon
  (temperature/fan/power) stats.
- **`CAP_PERFMON`** — Intel `i915`/`xe` engine utilization, and the preferred
  Intel RAPL power path, via the perf PMU.
- **`/dev/cpu`** (read-only) + **`CAP_SYS_RAWIO`** — the per-CPU MSR devices,
  used for Intel **iGPU package temperature** and as the fallback for RAPL
  package power when the perf PMU can't supply it. Unused on AMD GPUs and on
  Intel discrete GPUs (those read temperature/power from hwmon).

The default runs as root with those capabilities (not privileged, read-only
root filesystem). AMD GPUs and all sysfs-based stats need none of this — to run
unprivileged on AMD-only nodes, drop the capabilities and set a non-root
`podSecurityContext`.

**The MSR path needs the host's `msr` kernel module.** Most distros autoload it,
but Talos does not — add it via machine config, or Intel **iGPU temperature**
goes missing (power then comes from the perf PMU only; everything else is
unaffected):

```yaml
machine:
  kernel:
    modules:
      - name: msr
```

## Scoping to GPU nodes

Each pod **exits if its node has no GPU**, so on a cluster with non-GPU nodes,
restrict scheduling with `nodeSelector` (or `affinity`). The image is published
for `linux/amd64` only (the supported GPUs are x86_64 hardware), so on a
mixed-architecture cluster also pin `kubernetes.io/arch` to avoid an
`ImagePullBackOff` on arm64 nodes. With
[Node Feature Discovery](https://github.com/kubernetes-sigs/node-feature-discovery)
labelling PCI devices, for example:

```yaml
nodeSelector:
  kubernetes.io/arch: amd64
  feature.node.kubernetes.io/pci-0300_8086.present: "true" # Intel display controller (NFD)
```

## Configuration

The exporter is configured through environment variables; this chart exposes
them as the structured `config.*` values below, each mapping to a
`DRM_EXPORTER_*` variable (or `RUST_LOG`) the binary reads.

## Maintainers

| Name | Email | Url |
| ---- | ------ | --- |
| home-operations | <contact@home-operations.com> |  |

## Source Code

* <https://github.com/home-operations/drm-exporter>

## Requirements

Kubernetes: `>=1.25.0-0`

## Values

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| affinity | object | `{}` | Affinity rules for pod scheduling (templated). |
| config.devices | list | `[]` | Restrict export to specific PCI slots, e.g. `["0000:03:00.0"]` (DRM_EXPORTER_DEVICES); empty exports every GPU discovered. |
| config.intervalSeconds | int | `5` | Seconds between GPU stat refreshes (DRM_EXPORTER_INTERVAL_SECONDS). Intel engine utilization is sampled across this window, so keep it at or below the Prometheus scrape interval. |
| config.logLevel | string | `"info"` | Log level (RUST_LOG): error, warn, info, debug, or trace. |
| config.port | int | `9090` | Metrics HTTP listen port (DRM_EXPORTER_PORT); also the container and Service port. |
| daemonsetAnnotations | object | `{}` | Annotations added to the DaemonSet (workload) metadata, e.g. `reloader.stakater.com/auto: "true"`. |
| env | object | `{}` | Extra environment variables for the container, as a map (templated). |
| envFrom | list | `[]` | Sources of environment variables for the container (templated). |
| extraEnv | list | `[]` | Extra environment variables for the container, as a raw list (templated). |
| fullnameOverride | string | `""` | Override the generated name used for every resource's `metadata.name` (the chart "fullname"). |
| hostAccess.devCpu | string | `"/dev/cpu"` | Host path for the per-CPU MSR nodes, mounted read-only, for Intel iGPU package temperature and the RAPL power fallback. Needs the host `msr` kernel module (Talos: load it via machine config). Harmless (unused) on AMD GPUs; set to "" to skip. |
| hostAccess.devDri | string | `"/dev/dri"` | Host path for the DRM device nodes, mounted read-write; required to discover and read GPUs. |
| hostAccess.hostNetwork | bool | `false` | Use host networking (scrape via the node IP:port instead of through the Service). |
| hostAccess.sys | string | `"/sys"` | Host path for sysfs, mounted read-only; required for frequency, memory, AMD engine, and hwmon (temp/fan/power) stats. |
| image.digest | string | `""` | Pin the image by digest (sha256:…); set by the release pipeline. When set, it overrides the tag. |
| image.pullPolicy | string | `"IfNotPresent"` | Image pull policy. |
| image.repository | string | `"ghcr.io/home-operations/drm-exporter"` | Image repository. |
| image.tag | string | `""` | Overrides the image tag; defaults to the chart appVersion. |
| imagePullSecrets | list | `[]` | Image pull secrets for private registries. |
| initContainers | list | `[]` | Additional init containers (templated). |
| livenessProbe | object | `{"httpGet":{"path":"/health","port":"metrics"},"initialDelaySeconds":5,"periodSeconds":20}` | Liveness probe. `/health` is the exporter's lightweight readiness endpoint (returns `OK`). |
| monitoring.dashboards.annotations | object | `{}` | Annotations added to the dashboard ConfigMap (templated). |
| monitoring.dashboards.enabled | bool | `false` | Render the Grafana dashboard ConfigMap (for the kube-prometheus-stack sidecar or grafana-operator). |
| monitoring.dashboards.grafanaOperator.allowCrossNamespaceImport | bool | `true` | Allow a Grafana in any namespace to import this GrafanaDashboard. |
| monitoring.dashboards.grafanaOperator.enabled | bool | `false` | Render a GrafanaDashboard CR (grafana-operator) referencing the ConfigMap, instead of relying on the sidecar label. |
| monitoring.dashboards.grafanaOperator.folder | string | `""` | Folder to create the dashboard in; empty uses the Grafana default. |
| monitoring.dashboards.grafanaOperator.matchLabels | object | `{}` | Label selector matching the target Grafana instance (required when grafanaOperator is enabled). |
| monitoring.dashboards.grafanaOperator.resyncPeriod | string | `"10m"` | Resync period for the operator to re-check the dashboard. |
| monitoring.dashboards.labels | object | `{}` | Labels added to the dashboard ConfigMap. |
| monitoring.dashboards.namespace | string | `""` | Namespace for the dashboard objects; defaults to the release namespace. |
| monitoring.serviceMonitor.annotations | object | `{}` | ServiceMonitor annotations. |
| monitoring.serviceMonitor.enabled | bool | `false` | Create a Prometheus Operator ServiceMonitor (requires its CRDs). |
| monitoring.serviceMonitor.interval | string | `"30s"` | Scrape interval. |
| monitoring.serviceMonitor.labels | object | `{}` | ServiceMonitor labels. |
| monitoring.serviceMonitor.metricRelabelings | list | `[]` | Prometheus metric relabelings. |
| monitoring.serviceMonitor.path | string | `"/metrics"` | Metrics path. |
| monitoring.serviceMonitor.relabelings | list | `[]` | Prometheus relabelings. |
| monitoring.serviceMonitor.scrapeTimeout | string | `"10s"` | Scrape timeout. |
| nameOverride | string | `""` | Override the chart name used in the `app.kubernetes.io/name` label. |
| nodeSelector | object | `{}` | Node selector for pod scheduling (templated). In mixed clusters, scope to GPU nodes — e.g. a Node Feature Discovery label like `feature.node.kubernetes.io/pci-0300_8086.present: "true"`. See the README. |
| podAnnotations | object | `{}` | Annotations added to the pod. |
| podLabels | object | `{}` | Labels added to the pod. |
| podSecurityContext | object | `{"runAsGroup":0,"runAsNonRoot":false,"runAsUser":0,"seccompProfile":{"type":"RuntimeDefault"}}` | Pod-level securityContext. The exporter runs as root by default because GPU telemetry needs MSR access (Intel package power) and the perf PMU on locked-down kernels; see `securityContext` for the (narrow) capability set. Override to run unprivileged where your nodes permit it. |
| priorityClassName | string | `""` | PriorityClass for the pod (templated); empty uses the cluster default. |
| readinessProbe | object | `{"httpGet":{"path":"/health","port":"metrics"},"initialDelaySeconds":2,"periodSeconds":10}` | Readiness probe. |
| resources | object | `{}` | Exporter container resource requests/limits. |
| securityContext | object | `{"allowPrivilegeEscalation":false,"capabilities":{"add":["PERFMON","SYS_RAWIO"],"drop":["ALL"]},"privileged":false,"readOnlyRootFilesystem":true}` | Exporter container securityContext. Drops ALL capabilities, then adds only what GPU telemetry needs: PERFMON (Intel i915/xe engine utilization via the perf PMU) and SYS_RAWIO (Intel iGPU package temperature + the RAPL power fallback via /dev/cpu/*/msr, which needs the host msr kernel module). AMD and all sysfs-based stats need neither. Read-only root filesystem, no privilege escalation, not privileged. |
| service.annotations | object | `{}` | Service annotations. |
| service.type | string | `"ClusterIP"` | Service type. |
| serviceAccount.annotations | object | `{}` | ServiceAccount annotations. |
| serviceAccount.automount | bool | `false` | Mount the API token. The exporter never calls the Kubernetes API, so this is off by default. |
| serviceAccount.create | bool | `true` | Create a ServiceAccount. |
| serviceAccount.name | string | `""` | ServiceAccount name; empty uses the chart fullname. |
| terminationGracePeriodSeconds | int | `30` | Grace period for a clean shutdown. |
| tests.image.pullPolicy | string | `"IfNotPresent"` | `helm test` image pull policy. |
| tests.image.repository | string | `"mirror.gcr.io/curlimages/curl"` | `helm test` connection-pod image; a gcr-mirrored curl, so the test never pulls from Docker Hub. |
| tests.image.tag | string | `"8.20.0@sha256:b3f1fb2a51d923260350d21b8654bbc607164a987e2f7c84a0ac199a67df812a"` | `helm test` image, pinned as `tag@sha256:digest` so Renovate bumps the tag and its digest together. |
| tolerations | list | `[]` | Tolerations for pod scheduling (templated). |
| updateStrategy | object | `{"type":"RollingUpdate"}` | DaemonSet update strategy. |
| volumeMounts | list | `[]` | Additional volume mounts on the exporter container (templated). |
| volumes | list | `[]` | Additional volumes on the pod, beyond the host mounts (templated). |

---

_This README is generated by [helm-docs](https://github.com/norwoodj/helm-docs) from `Chart.yaml` and `values.yaml`. Edit those (or `README.md.gotmpl`) and run `mise run helm-docs`._
