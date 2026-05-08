# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.1.1...v0.2.0) - 2026-05-08

### Documentation

- configure docs.rs all-features and add README badges ([#42](https://github.com/bulletxyz/bullet-exchange-interface/pull/42))

### Features

- enums are non-exhaustive to not break semver with new variants ([#45](https://github.com/bulletxyz/bullet-exchange-interface/pull/45))
- *(security)* added pyth signature support ([#26](https://github.com/bulletxyz/bullet-exchange-interface/pull/26))

## [0.1.0](https://github.com/bulletxyz/bullet-exchange-interface/releases/tag/v0.1.0) - 2026-05-04

### Bug Fixes

- update to the latest rust-decimal version to get more accurate calculations downstream ([#16](https://github.com/bulletxyz/bullet-exchange-interface/pull/16))
- trim example failed due to microseconds time struct

### Features

- convert from Actions to CallMessage to simplify client code ([#28](https://github.com/bulletxyz/bullet-exchange-interface/pull/28))
- add CancelReason and V1 cancel events ([#27](https://github.com/bulletxyz/bullet-exchange-interface/pull/27))
- add AdminAction::InitProtocolVault variant ([#21](https://github.com/bulletxyz/bullet-exchange-interface/pull/21))
- new v1 events for initialising assets and markets ([#25](https://github.com/bulletxyz/bullet-exchange-interface/pull/25))
- make the universal-wallet schema an optional feature ([#24](https://github.com/bulletxyz/bullet-exchange-interface/pull/24))
- use the published version of the sov-universal-wallet crate ([#23](https://github.com/bulletxyz/bullet-exchange-interface/pull/23))
- add DelegateUserV2 and DelegateVaultUserV2 with expiry and flags ([#17](https://github.com/bulletxyz/bullet-exchange-interface/pull/17))
- 10 fee tier enums ([#20](https://github.com/bulletxyz/bullet-exchange-interface/pull/20))
- iso margin handlers and new helpers for timestamp ([#19](https://github.com/bulletxyz/bullet-exchange-interface/pull/19))
- add MAX_TX_SIZE support ([#13](https://github.com/bulletxyz/bullet-exchange-interface/pull/13))
- add fee tier Tier5 ([#15](https://github.com/bulletxyz/bullet-exchange-interface/pull/15))
- add bank transfers ([#10](https://github.com/bulletxyz/bullet-exchange-interface/pull/10))
- iso markets risk engine ([#9](https://github.com/bulletxyz/bullet-exchange-interface/pull/9))
- add TX transaction size constant ([#8](https://github.com/bulletxyz/bullet-exchange-interface/pull/8))
- derive strum::FromRepr for all u8 enums ([#4](https://github.com/bulletxyz/bullet-exchange-interface/pull/4))
- make delegation better ([#1](https://github.com/bulletxyz/bullet-exchange-interface/pull/1))
