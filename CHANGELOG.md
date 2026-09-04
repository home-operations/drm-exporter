# Changelog

## [0.3.4](https://github.com/home-operations/drm-exporter/compare/0.3.3...0.3.4) (2026-09-04)


### Features

* **container:** update image mirror.gcr.io/curlimages/curl (8.21.0 → 8.22.0) ([#107](https://github.com/home-operations/drm-exporter/issues/107)) ([1f9dc44](https://github.com/home-operations/drm-exporter/commit/1f9dc443f38fb42f4a27e24921d908459b0016ce))


### Bug Fixes

* **chart:** keep the env example out of the extraEnv schema description ([#96](https://github.com/home-operations/drm-exporter/issues/96)) ([9545c45](https://github.com/home-operations/drm-exporter/commit/9545c451d26472e79153105b6de20bd93043ee8b))
* **ci:** fail the merge gate on cancelled jobs ([#77](https://github.com/home-operations/drm-exporter/issues/77)) ([fd4503a](https://github.com/home-operations/drm-exporter/commit/fd4503a71f0c288ca08e4553fd6a6217eac187ab))
* **deps:** update h2 to 0.4.19 for RUSTSEC-2026-0258 ([#102](https://github.com/home-operations/drm-exporter/issues/102)) ([0517dc7](https://github.com/home-operations/drm-exporter/commit/0517dc70f5eb6d6bd2b6ceafbea936a67bf8b7cd))
* **rust:** update crate clap (4.6.5 → 4.6.6) ([#80](https://github.com/home-operations/drm-exporter/issues/80)) ([6566d7d](https://github.com/home-operations/drm-exporter/commit/6566d7dbaa493c6c208ad40735e58b95f4ac1859))


### Continuous Integration

* **github-action:** Update action docker/github-builder (v1.14.0 → v1.15.0) ([#76](https://github.com/home-operations/drm-exporter/issues/76)) ([931d92b](https://github.com/home-operations/drm-exporter/commit/931d92bd6668fec42e47a539f5c31c303e9326ad))
* **github-action:** Update action docker/github-builder (v1.15.0 → v1.16.0) ([#90](https://github.com/home-operations/drm-exporter/issues/90)) ([f8ef9a9](https://github.com/home-operations/drm-exporter/commit/f8ef9a9fe823670aa2ec1e66ea754a06bd48a557))
* **github-action:** Update action docker/login-action (v4.5.1 → v4.5.2) ([#79](https://github.com/home-operations/drm-exporter/issues/79)) ([cd6606a](https://github.com/home-operations/drm-exporter/commit/cd6606a648a33480117ae03f3b56fae71fcc7484))
* **github-action:** Update action docker/login-action (v4.5.2 → v4.6.0) ([#83](https://github.com/home-operations/drm-exporter/issues/83)) ([0da4e61](https://github.com/home-operations/drm-exporter/commit/0da4e61d3a175e86bbf88d77fa2a307aae41d54c))
* **github-action:** Update action home-operations/.github/actions/workflow-lint (v1.0.2 → v1.0.3) ([#87](https://github.com/home-operations/drm-exporter/issues/87)) ([74ed17e](https://github.com/home-operations/drm-exporter/commit/74ed17e3ef6c6bcea004cd7df1fc98a251091c2c))
* **github-action:** Update action jdx/mise-action (v4.2.3 → v4.2.4) ([#88](https://github.com/home-operations/drm-exporter/issues/88)) ([a5da76c](https://github.com/home-operations/drm-exporter/commit/a5da76c7ef4896ebd59e86bee0c5a9410ee45e2c))
* **github-action:** Update action Swatinem/rust-cache (v2.9.1 → v2.9.2) ([#92](https://github.com/home-operations/drm-exporter/issues/92)) ([cc3b601](https://github.com/home-operations/drm-exporter/commit/cc3b6017db5269a1a326b9c39ea56c9af650b014))
* **github-action:** update workflow-lint action (1.0.0 → v1.0.2) ([#85](https://github.com/home-operations/drm-exporter/issues/85)) ([426a892](https://github.com/home-operations/drm-exporter/commit/426a8928174ca88ae61977be484d019ee27b449e))


### Miscellaneous Chores

* **github-action:** update action docker/github-builder (v1.16.0 → v1.17.0) ([#101](https://github.com/home-operations/drm-exporter/issues/101)) ([a46bf28](https://github.com/home-operations/drm-exporter/commit/a46bf2855b5d961e4358d2cbc2ed2e37b39f29ce))
* **github-action:** update action jdx/mise-action (v4.2.4 → v4.2.5) ([#94](https://github.com/home-operations/drm-exporter/issues/94)) ([1bb06e3](https://github.com/home-operations/drm-exporter/commit/1bb06e3af8acd25ddb112575eb558d7b28874d3c))
* **github-action:** update action jdx/mise-action (v4.2.5 → v4.3.0) ([#105](https://github.com/home-operations/drm-exporter/issues/105)) ([cb0de87](https://github.com/home-operations/drm-exporter/commit/cb0de87e4b3fb4d9b775899eb38051dd500544d3))
* **mise:** prune lockfile to used platforms ([#86](https://github.com/home-operations/drm-exporter/issues/86)) ([637b367](https://github.com/home-operations/drm-exporter/commit/637b3676796bfd1084e6ede802c8888567840c20))
* **mise:** update mise tools ([#100](https://github.com/home-operations/drm-exporter/issues/100)) ([aec4be6](https://github.com/home-operations/drm-exporter/commit/aec4be6e9c8387760d2d43d380fa2153b11bf143))
* **mise:** update mise tools ([#97](https://github.com/home-operations/drm-exporter/issues/97)) ([9ed98fe](https://github.com/home-operations/drm-exporter/commit/9ed98fe175ff0f812533f6ffccdfac54599f03e4))
* **mise:** update tool aqua:dadav/helm-schema (0.23.4 → 0.23.5) ([#103](https://github.com/home-operations/drm-exporter/issues/103)) ([2d277ce](https://github.com/home-operations/drm-exporter/commit/2d277cee2a1ee36f168fd51a0b52c7b2f35977e3))
* **mise:** Update tool cosign (3.1.2 → 3.1.3) ([#91](https://github.com/home-operations/drm-exporter/issues/91)) ([f8bd86e](https://github.com/home-operations/drm-exporter/commit/f8bd86ed2b1bcdf1db00b2474ba7b6fd80368eae))
* **mise:** update tool helm (4.2.3 → 4.2.4) ([#95](https://github.com/home-operations/drm-exporter/issues/95)) ([78f58a5](https://github.com/home-operations/drm-exporter/commit/78f58a520fae10b51d5cdb04f2d2ec22366476d7))
* **mise:** update tool lefthook (2.1.11 → 2.1.12) ([#106](https://github.com/home-operations/drm-exporter/issues/106)) ([2db4ba7](https://github.com/home-operations/drm-exporter/commit/2db4ba7af288e4641c17468d76edc1c1c417bd72))
* **mise:** Update tool oxfmt (0.61.0 → 0.62.0) ([#89](https://github.com/home-operations/drm-exporter/issues/89)) ([5858583](https://github.com/home-operations/drm-exporter/commit/58585834b04bfb30f04cbbfb8b0413ed51bf8ee3))
* **mise:** Update tool oxfmt (0.62.0 → 0.63.0) ([#93](https://github.com/home-operations/drm-exporter/issues/93)) ([63250c0](https://github.com/home-operations/drm-exporter/commit/63250c01d82edb44dc4c572dd8b95e57f297eb4c))
* **mise:** update tool oxfmt (0.64.0 → 0.65.0) ([#104](https://github.com/home-operations/drm-exporter/issues/104)) ([7cf0594](https://github.com/home-operations/drm-exporter/commit/7cf0594d705ab91d668a2b471ae6416f1c807779))
* **mise:** update tool oxfmt (0.65.0 → 0.66.0) ([#109](https://github.com/home-operations/drm-exporter/issues/109)) ([995e895](https://github.com/home-operations/drm-exporter/commit/995e895558d695cdc71124ee33d8b35b681bf6d3))
* **mise:** update tool rust (1.97.1 → 1.98.0) ([#99](https://github.com/home-operations/drm-exporter/issues/99)) ([ca31e1c](https://github.com/home-operations/drm-exporter/commit/ca31e1c330dd11230fc638ebe58d8bfe7e852ccd))
* **mise:** update tool yq (4.53.3 → 4.53.4) ([#98](https://github.com/home-operations/drm-exporter/issues/98)) ([36b59fd](https://github.com/home-operations/drm-exporter/commit/36b59fd8446d7456a350510474d57b560111f33e))
* **mise:** Update tool zizmor (1.28.0 → 1.29.0) ([#84](https://github.com/home-operations/drm-exporter/issues/84)) ([b1a9bba](https://github.com/home-operations/drm-exporter/commit/b1a9bba4e9236d4259ef350eedf7e617ad5c82d4))
* **mise:** update tool zizmor (1.29.0 → 1.30.0) ([#108](https://github.com/home-operations/drm-exporter/issues/108)) ([e40d023](https://github.com/home-operations/drm-exporter/commit/e40d023eb7fb445548d742486a69a6181d46b4bd))
* **release-please:** standardize the release pull request title pattern ([#82](https://github.com/home-operations/drm-exporter/issues/82)) ([43b1a36](https://github.com/home-operations/drm-exporter/commit/43b1a36dee38eb9f3e2c881358933bc10464a294))
* **rust:** lock file maintenance crate (cargo) ([#81](https://github.com/home-operations/drm-exporter/issues/81)) ([3c287b4](https://github.com/home-operations/drm-exporter/commit/3c287b41b284c7c880b18e911008b64244976443))

## [0.3.3](https://github.com/home-operations/drm-exporter/compare/0.3.2...0.3.3) (2026-07-28)


### Bug Fixes

* **rust:** update crate qmlib (2.2.1 → 2.2.2) ([#67](https://github.com/home-operations/drm-exporter/issues/67)) ([44df613](https://github.com/home-operations/drm-exporter/commit/44df613466e631df360f9d551b0110437dcea3c0))


### Build System

* **mise:** add actionlint and refresh the lockfile ([#68](https://github.com/home-operations/drm-exporter/issues/68)) ([cc77a8c](https://github.com/home-operations/drm-exporter/commit/cc77a8c659486ecf24677160a0c95a0a3abddcb5))


### Continuous Integration

* gate pull requests on a single Build Success check ([#66](https://github.com/home-operations/drm-exporter/issues/66)) ([6b8b370](https://github.com/home-operations/drm-exporter/commit/6b8b370c4d4ec9feef119304c9ebe42e576e9fee))
* **github-action:** Update action docker/login-action (v4.5.0 → v4.5.1) ([#72](https://github.com/home-operations/drm-exporter/issues/72)) ([e427427](https://github.com/home-operations/drm-exporter/commit/e4274274813694c188a627805e4f6989480756cc))
* **github-action:** Update action jdx/mise-action (v4.2.1 → v4.2.2) ([#71](https://github.com/home-operations/drm-exporter/issues/71)) ([e34037a](https://github.com/home-operations/drm-exporter/commit/e34037ab3dbe48289f9f1d8e425ce129b0afecae))
* **github-action:** Update action jdx/mise-action (v4.2.2 → v4.2.3) ([#74](https://github.com/home-operations/drm-exporter/issues/74)) ([bfb474f](https://github.com/home-operations/drm-exporter/commit/bfb474f5e0cacb01236089677702cdac1a712fa8))
* lint workflows with the shared composite action ([#70](https://github.com/home-operations/drm-exporter/issues/70)) ([5bb3065](https://github.com/home-operations/drm-exporter/commit/5bb3065a458f55e54155f02a6652736a3ac8b813))
* skip release-please version-bump PRs in checks ([#65](https://github.com/home-operations/drm-exporter/issues/65)) ([13e7ef3](https://github.com/home-operations/drm-exporter/commit/13e7ef33c1918b520872aeaa734d799d9cf01f8e))


### Miscellaneous Chores

* **mise:** Update tool oxfmt (0.60.0 → 0.61.0) ([#73](https://github.com/home-operations/drm-exporter/issues/73)) ([eb1624b](https://github.com/home-operations/drm-exporter/commit/eb1624b7d7ad6c16c29cfd470ada8352facaf1f8))
* standardize release-please changelog sections ([#75](https://github.com/home-operations/drm-exporter/issues/75)) ([02b30d3](https://github.com/home-operations/drm-exporter/commit/02b30d3a87fdd6e2dbb67009f2b7998bc475a9b1))

## [0.3.2](https://github.com/home-operations/drm-exporter/compare/0.3.1...0.3.2) (2026-07-25)


### Features

* **deps:** update rust crate tokio (1.52.3 → 1.53.0) ([#51](https://github.com/home-operations/drm-exporter/issues/51)) ([2646ce9](https://github.com/home-operations/drm-exporter/commit/2646ce93503d429be8bd78c60c68590c8785065b))


### Bug Fixes

* **deps:** update rust crate anyhow (1.0.103 → 1.0.104) ([#55](https://github.com/home-operations/drm-exporter/issues/55)) ([437b4aa](https://github.com/home-operations/drm-exporter/commit/437b4aa016023d2457b8d40efef48a29ecf166ef))
* **deps:** update rust crate clap (4.6.1 → 4.6.2) ([#49](https://github.com/home-operations/drm-exporter/issues/49)) ([dee47b2](https://github.com/home-operations/drm-exporter/commit/dee47b255fbb576093810342fff1d73f80c812ea))
* **deps:** update rust crate clap (4.6.2 → 4.6.3) ([#56](https://github.com/home-operations/drm-exporter/issues/56)) ([e14feba](https://github.com/home-operations/drm-exporter/commit/e14febafff69fc63bc76c4cd4dcba34dac427b96))
* **deps:** update rust crate clap (4.6.3 → 4.6.4) ([#59](https://github.com/home-operations/drm-exporter/issues/59)) ([e5734c7](https://github.com/home-operations/drm-exporter/commit/e5734c72268037e549ddbfb418bbd118b7c67e0e))
* **deps:** update rust crate tokio (1.53.0 → 1.53.1) ([#58](https://github.com/home-operations/drm-exporter/issues/58)) ([fa3ca1c](https://github.com/home-operations/drm-exporter/commit/fa3ca1ceb29cf17614b8d5509f2a06e99286eb3d))
* **helm:** stamp Chart.yaml version on release ([#64](https://github.com/home-operations/drm-exporter/issues/64)) ([8309502](https://github.com/home-operations/drm-exporter/commit/8309502ea02eb6546cdce767db9eb8d67d66749d))


### Miscellaneous Chores

* add .editorconfig from the org standard ([#57](https://github.com/home-operations/drm-exporter/issues/57)) ([f611e31](https://github.com/home-operations/drm-exporter/commit/f611e311aedb2a1b578b0eb2c4decbf90362482f))
* **github-release:** Update release helm-unittest/helm-unittest (v1.1.1 → v1.1.2) ([#63](https://github.com/home-operations/drm-exporter/issues/63)) ([43a55cf](https://github.com/home-operations/drm-exporter/commit/43a55cf0ad82334eda5f39c3b8a7de0be6865dd9))
* **mise:** Update tool cosign (3.1.1 → 3.1.2) ([#54](https://github.com/home-operations/drm-exporter/issues/54)) ([b04620e](https://github.com/home-operations/drm-exporter/commit/b04620e81b5d6aaaf1e203f34f799751cff17093))
* **mise:** Update tool oxfmt (0.59.0 → 0.60.0) ([#61](https://github.com/home-operations/drm-exporter/issues/61)) ([ba4896c](https://github.com/home-operations/drm-exporter/commit/ba4896c17b4a59f7a68013cf653bfdd756d9d47f))
* **mise:** Update tool rust (1.97.0 → 1.97.1) ([#50](https://github.com/home-operations/drm-exporter/issues/50)) ([436fdd6](https://github.com/home-operations/drm-exporter/commit/436fdd601af2a671a1d433e7b5a1be08a5b220a7))
* **mise:** Update tool zizmor (1.27.0 → 1.28.0) ([#60](https://github.com/home-operations/drm-exporter/issues/60)) ([c75e9c7](https://github.com/home-operations/drm-exporter/commit/c75e9c7925b8a07955ea0b8ecd4d981c829c90ed))

## [0.3.1](https://github.com/home-operations/drm-exporter/compare/0.3.0...0.3.1) (2026-07-14)


### Miscellaneous Chores

* **mise:** Update tool aqua:EmbarkStudios/cargo-deny (0.19.9 → 0.20.2) ([#45](https://github.com/home-operations/drm-exporter/issues/45)) ([885c7b4](https://github.com/home-operations/drm-exporter/commit/885c7b4a9a64baacfe258b28252ba929098e984b))
* **mise:** Update tool helm (4.2.2 → 4.2.3) ([#46](https://github.com/home-operations/drm-exporter/issues/46)) ([a379bb8](https://github.com/home-operations/drm-exporter/commit/a379bb8b3d5eef7b7dacd86aa0e599bb9d7444dc))
* **mise:** Update tool lefthook (2.1.9 → 2.1.10) ([#43](https://github.com/home-operations/drm-exporter/issues/43)) ([aac0f94](https://github.com/home-operations/drm-exporter/commit/aac0f94c0fd8f589f315f6f937a81112b9832d6f))
* **mise:** Update tool oxfmt (0.57.0 → 0.58.0) ([#41](https://github.com/home-operations/drm-exporter/issues/41)) ([c9a1e47](https://github.com/home-operations/drm-exporter/commit/c9a1e47367987a14eb63ee3b9cf36bb2fabf4979))
* **mise:** Update tool oxfmt (0.58.0 → 0.59.0) ([#47](https://github.com/home-operations/drm-exporter/issues/47)) ([847de01](https://github.com/home-operations/drm-exporter/commit/847de014eadf9e1c8bd18c751db044bab534bdce))
* **mise:** Update tool rust (1.96.1 → 1.97.0) ([#44](https://github.com/home-operations/drm-exporter/issues/44)) ([d489afe](https://github.com/home-operations/drm-exporter/commit/d489afe6727398aa0d434957a7894b7c2887d474))
* **mise:** Update tool zizmor (1.26.1 → 1.27.0) ([#48](https://github.com/home-operations/drm-exporter/issues/48)) ([95404e3](https://github.com/home-operations/drm-exporter/commit/95404e31e6d0273a318473991f8a804b57d15647))

## [0.3.0](https://github.com/home-operations/drm-exporter/compare/0.2.6...0.3.0) (2026-07-04)


### ⚠ BREAKING CHANGES

* adopt the /healthz + /readyz pair and a dual-stack default bind ([#39](https://github.com/home-operations/drm-exporter/issues/39))

### Features

* adopt the /healthz + /readyz pair and a dual-stack default bind ([#39](https://github.com/home-operations/drm-exporter/issues/39)) ([ef69995](https://github.com/home-operations/drm-exporter/commit/ef69995111071bddd603b4ee6ba3c106e01f6cc8))

## [0.2.6](https://github.com/home-operations/drm-exporter/compare/0.2.5...0.2.6) (2026-07-03)


### Miscellaneous Chores

* **mise:** Lock file maintenance tool ([#38](https://github.com/home-operations/drm-exporter/issues/38)) ([669c8ef](https://github.com/home-operations/drm-exporter/commit/669c8efb87a7c9408e67588bb7a81f05ed39954b))
* **mise:** Update tool oxfmt (0.56.0 → 0.57.0) ([#36](https://github.com/home-operations/drm-exporter/issues/36)) ([8c367ce](https://github.com/home-operations/drm-exporter/commit/8c367cebdc163c3da92ad93e0512d006f93e323d))
* **mise:** Update tool rust (1.96.0 → 1.96.1) ([#37](https://github.com/home-operations/drm-exporter/issues/37)) ([de36de2](https://github.com/home-operations/drm-exporter/commit/de36de2219372590d190371c72b5cca2e9328811))
* **renovate:** inherit shared chart-docs postUpgradeTasks preset ([#34](https://github.com/home-operations/drm-exporter/issues/34)) ([2eb2990](https://github.com/home-operations/drm-exporter/commit/2eb2990f7c30c6997ee9df6326713c1476ff2358))
* update .renovaterc.json5 with cargo update packageRules ([508c8e3](https://github.com/home-operations/drm-exporter/commit/508c8e37b65d7d02f0946baf6f139dc2781bcc1b))

## [0.2.5](https://github.com/home-operations/drm-exporter/compare/0.2.4...0.2.5) (2026-06-27)


### Features

* **container:** update image mirror.gcr.io/curlimages/curl (8.20.0 → 8.21.0) ([#30](https://github.com/home-operations/drm-exporter/issues/30)) ([12f8ffa](https://github.com/home-operations/drm-exporter/commit/12f8ffadc3e4c4fe0f5cd068cbb206be6777e150))


### Bug Fixes

* **deps:** update qmlib digest (799a63b → 4b0dbec) (#undefined) ([22fe84a](https://github.com/home-operations/drm-exporter/commit/22fe84a4316e012ad8c02ea9e301513894b9177a))
* **deps:** update rust crate anyhow (1.0.102 → 1.0.103) ([#31](https://github.com/home-operations/drm-exporter/issues/31)) ([7036ef4](https://github.com/home-operations/drm-exporter/commit/7036ef46289ce7c527545b1c9f5d5ea022cf6aee))


### Miscellaneous Chores

* **mise:** Update tool oxfmt (0.55.0 → 0.56.0) ([#27](https://github.com/home-operations/drm-exporter/issues/27)) ([0bf9719](https://github.com/home-operations/drm-exporter/commit/0bf9719f9812c3b71949654c5a9ac724f9042398))

## [0.2.4](https://github.com/home-operations/drm-exporter/compare/0.2.3...0.2.4) (2026-06-21)


### Bug Fixes

* **deps:** update qmlib to qmassa main (799a63b) ([#25](https://github.com/home-operations/drm-exporter/issues/25)) ([60b2cbe](https://github.com/home-operations/drm-exporter/commit/60b2cbe60eb383117354a2a27e4d102ce78ef4e5))


### Miscellaneous Chores

* **mise:** Update tool zizmor (1.25.2 → 1.26.1) ([#24](https://github.com/home-operations/drm-exporter/issues/24)) ([d107a74](https://github.com/home-operations/drm-exporter/commit/d107a74bd55cd5cd5aca89c654cf5ca99da9a6fb))

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
