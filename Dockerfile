# drm-exporter image.
#
# The binary links libudev (via qmlib's udev FFI) and glibc, so it is NOT a
# static binary: the runtime is distroless/cc (glibc + CA certs, non-root) with
# libudev.so.1 copied in. PCI vendor/model *names* need the udev hardware
# database, which is not bundled — those `drm_info` labels fall back to the
# numeric vendor/device ids (the PCI id is exported regardless).
#
# syntax=docker/dockerfile:1

ARG RUST_VERSION=1.96.0

FROM rust:${RUST_VERSION}-slim-trixie AS builder
# TARGETARCH is supplied by buildx per platform; the cache-mount ids below are
# keyed on it so a concurrent multi-arch build does not share one cargo cache
# and race on unpacking the registry. VERSION is stamped into `--version`.
ARG TARGETARCH
ARG VERSION
ENV DRM_EXPORTER_VERSION=${VERSION}
WORKDIR /src

# pkg-config + libudev headers to link qmlib's udev FFI. libudev-dev pulls in
# the libudev1 runtime shared object, which the runtime stage copies out.
RUN apt-get update \
  && apt-get install -y --no-install-recommends pkg-config libudev-dev \
  && rm -rf /var/lib/apt/lists/*

# Copy manifests + sources, then build the release binary with locked deps.
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN --mount=type=cache,target=/usr/local/cargo/registry,id=cargo-registry-${TARGETARCH},sharing=locked \
    --mount=type=cache,target=/src/target,id=cargo-target-${TARGETARCH},sharing=locked \
    cargo build --release --locked \
    && cp target/release/drm-exporter /usr/local/bin/drm-exporter

FROM gcr.io/distroless/cc-debian13:nonroot AS runtime
# Ensure the loader searches /usr/lib for the copied libudev (distroless keeps
# glibc under the arch triplet dir; /usr/lib is added here for the flat copy).
ENV LD_LIBRARY_PATH=/usr/lib
COPY --from=builder /usr/local/bin/drm-exporter /usr/local/bin/drm-exporter
# libudev's full non-glibc runtime closure: libudev.so.1 + its one dependency,
# libcap.so.2 (glibc/libgcc come from the cc base).
COPY --from=builder /usr/lib/*/libudev.so.1* /usr/lib/
COPY --from=builder /usr/lib/*/libcap.so.2* /usr/lib/
EXPOSE 8081
ENTRYPOINT ["/usr/local/bin/drm-exporter"]
