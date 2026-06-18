# Changelog

## [0.2.3](https://github.com/home-operations/drm-exporter/compare/0.2.2...0.2.3) (2026-06-18)


### Features

* **chart:** polish the GPU dashboard ([#21](https://github.com/home-operations/drm-exporter/issues/21)) ([8db0f29](https://github.com/home-operations/drm-exporter/commit/8db0f294b9611b6ffd58b1d69f72f2d3a73af069))

## [0.2.2](https://github.com/home-operations/drm-exporter/compare/0.2.1...0.2.2) (2026-06-18)


### Features

* **chart:** distinguish nodes in the Grafana dashboard ([#19](https://github.com/home-operations/drm-exporter/issues/19)) ([802313b](https://github.com/home-operations/drm-exporter/commit/802313b3253113a8f416e8e5a2d928ac0b2de961))

## [0.2.1](https://github.com/home-operations/drm-exporter/compare/0.2.0...0.2.1) (2026-06-18)


### Features

* change the default metrics port to 8081 ([#18](https://github.com/home-operations/drm-exporter/issues/18)) ([edeb6dc](https://github.com/home-operations/drm-exporter/commit/edeb6dca10a1d5255ccb12d4114487f2bd0c24b8))


### Code Refactoring

* **chart:** rename templates to .tpl ([#16](https://github.com/home-operations/drm-exporter/issues/16)) ([6de9988](https://github.com/home-operations/drm-exporter/commit/6de9988be4042bf33b8034175d4e694b5337fbf5))

## [0.2.0](https://github.com/home-operations/drm-exporter/compare/0.1.1...0.2.0) (2026-06-18)


### ⚠ BREAKING CHANGES

* **chart:** DRA ResourceClaimTemplate + require Kubernetes 1.34+ ([#13](https://github.com/home-operations/drm-exporter/issues/13))
* **github-action:** Update action actions/checkout (v6.0.3 → v7.0.0) ([#14](https://github.com/home-operations/drm-exporter/issues/14))

### Features

* **chart:** DRA ResourceClaimTemplate + require Kubernetes 1.34+ ([#13](https://github.com/home-operations/drm-exporter/issues/13)) ([5f9dfa9](https://github.com/home-operations/drm-exporter/commit/5f9dfa95d867583400fb5b873c6aabbed2b71290))


### Continuous Integration

* **github-action:** Update action actions/checkout (v6.0.3 → v7.0.0) ([#14](https://github.com/home-operations/drm-exporter/issues/14)) ([d1e9391](https://github.com/home-operations/drm-exporter/commit/d1e939138ecf1ceb0ee05c73dffcf0f239726711))

## [0.1.1](https://github.com/home-operations/drm-exporter/compare/0.1.0...0.1.1) (2026-06-18)


### Features

* **deps:** update opentelemetry crates to 0.32 ([#12](https://github.com/home-operations/drm-exporter/issues/12)) ([b5d8735](https://github.com/home-operations/drm-exporter/commit/b5d8735e223bf894d185cd8d19f50ce13d49007a))


### Documentation

* correct Talos MSR guidance (Talos ships no msr module) ([#10](https://github.com/home-operations/drm-exporter/issues/10)) ([78b226d](https://github.com/home-operations/drm-exporter/commit/78b226d15ed089e6c0c7fc204683d7b9ccf85207))


### Miscellaneous Chores

* **mise:** update tool aqua:embarkstudios/cargo-deny (0.19.8 → 0.19.9) ([#2](https://github.com/home-operations/drm-exporter/issues/2)) ([1438cb6](https://github.com/home-operations/drm-exporter/commit/1438cb656786eaa7480c1ae2ac190b67a30e0061))
* **mise:** update tool helm (4.2.1 → 4.2.2) ([#3](https://github.com/home-operations/drm-exporter/issues/3)) ([ae801ff](https://github.com/home-operations/drm-exporter/commit/ae801ff6cdf32bc56a35e56936168f0cf2e52cce))
* **mise:** update tool oxfmt (0.54.0 → 0.55.0) ([#4](https://github.com/home-operations/drm-exporter/issues/4)) ([02dc4a9](https://github.com/home-operations/drm-exporter/commit/02dc4a9069d7dd71f4d967ac694bdc11d102763b))

## 0.1.0 (2026-06-18)


### Features

* GPU metrics exporter for Intel and AMD with Helm chart ([56cfb5b](https://github.com/home-operations/drm-exporter/commit/56cfb5bbf0455f33b81df070c6c2b25c2c278321))


### Performance Improvements

* use mimalloc as the global allocator ([3b4c160](https://github.com/home-operations/drm-exporter/commit/3b4c160874bfb64503d8388bcbf8d5ab565c4bc5))


### Miscellaneous Chores

* remove images workflow ([58997be](https://github.com/home-operations/drm-exporter/commit/58997beac321ed226a22dee6f5610ce96dec0567))

## Changelog
