# Changelog

## [0.3.0](https://github.com/sentdm/sent-dm-rust/compare/v0.2.0...v0.3.0) (2026-11-12)


### ⚠ BREAKING CHANGES

* **api:** sync OpenAPI spec from production

### Features

* **api:** sync OpenAPI spec from production ([817f2ba](https://github.com/sentdm/sent-dm-rust/commit/817f2ba5141af359dc8854c5e96fadb5eaad170a))


### Bug Fixes

* **sdk:** methods for the US 10DLC campaigns, numbers and market compliance ([2652f98](https://github.com/sentdm/sent-dm-rust/commit/2652f98e3556990e75bfd05705a38da4bb862b95))

## [0.2.0](https://github.com/sentdm/sent-dm-rust/compare/v0.1.0...v0.2.0) (2026-11-06)


### ⚠ BREAKING CHANGES

* **sdk:** stable names for shared models; typed methods for sender profiles, channels, compliance requirements and message cancel

### Features

* **api:** drop the resend response's duplicate schema from the snapshot ([157c5f5](https://github.com/sentdm/sent-dm-rust/commit/157c5f5f625e81991e3d9b8635ea83996ed719f9))
* **api:** sync OpenAPI spec from production ([7e1db9c](https://github.com/sentdm/sent-dm-rust/commit/7e1db9c7662378b6c468d74675d6b2204945ef39))
* **api:** sync OpenAPI spec from production (x-enum-descriptions as arrays, sent-dm-services [#2608](https://github.com/sentdm/sent-dm-rust/issues/2608)) ([f715bf4](https://github.com/sentdm/sent-dm-rust/commit/f715bf425cb647c038c8a7f057eac6334c88c64d))


### Bug Fixes

* **sdk:** add messages.resend to resend a message on its id ([9c74e2b](https://github.com/sentdm/sent-dm-rust/commit/9c74e2b9131947ef88aeb910a5fdf5d53c0bb4de))
* **sdk:** stable names for shared models; typed methods for sender profiles, channels, compliance requirements and message cancel ([e21a629](https://github.com/sentdm/sent-dm-rust/commit/e21a629641166c61899876e594931d730afd1bbd))

## 0.1.0 (2026-11-02)


### Features

* the Sent REST API from Rust, rendered from the spec ([c6dc581](https://github.com/sentdm/sent-dm-rust/commit/c6dc581b132a0dafc0d8384ba0464104a901c4e0))
