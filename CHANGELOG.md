# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.12.1](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.12.0...v0.12.1) - 2026-07-13

### Features

- add bank/Mint call-message ([#93](https://github.com/bulletxyz/bullet-exchange-interface/pull/93))

## [0.12.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.11.2...v0.12.0) - 2026-07-07

### Bug Fixes

- [**breaking**] broken schema ([#91](https://github.com/bulletxyz/bullet-exchange-interface/pull/91))

## [0.11.2](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.11.1...v0.11.2) - 2026-07-07

### Bug Fixes

- address again by using a newer upstream version ([#89](https://github.com/bulletxyz/bullet-exchange-interface/pull/89))
- display of Address should not drop leading zeros ([#87](https://github.com/bulletxyz/bullet-exchange-interface/pull/87))

## [0.11.1](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.11.0...v0.11.1) - 2026-07-03

### Features

- *(warp)* add Register and Update admin call messages ([#84](https://github.com/bulletxyz/bullet-exchange-interface/pull/84))

## [0.11.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.10.0...v0.11.0) - 2026-06-24

### Bug Fixes

- wrong WARP discriminant

## [0.10.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.9.3...v0.10.0) - 2026-06-23

### Features

- margin discount and new admin type ([#80](https://github.com/bulletxyz/bullet-exchange-interface/pull/80))

## [0.9.3](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.9.2...v0.9.3) - 2026-06-22

### Features

- add UpdatePremiumIndexV1 to report estimated_funding_rate ([#78](https://github.com/bulletxyz/bullet-exchange-interface/pull/78))

## [0.9.2](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.9.1...v0.9.2) - 2026-06-19

### Features

- add TwapsActive event ([#75](https://github.com/bulletxyz/bullet-exchange-interface/pull/75))

## [0.9.1](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.9.0...v0.9.1) - 2026-06-18

### Bug Fixes

- introduce UpdateAssetInfoV2 to avoid resetting pyth feeds ([#70](https://github.com/bulletxyz/bullet-exchange-interface/pull/70))

## [0.9.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.8.0...v0.9.0) - 2026-06-15

### Features

- non-exhaustive enums have also non-exhaustive enum-discriminants ([#69](https://github.com/bulletxyz/bullet-exchange-interface/pull/69))
- add new mark-price event ([#67](https://github.com/bulletxyz/bullet-exchange-interface/pull/67))

## [0.8.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.7.0...v0.8.0) - 2026-06-12

### Features

- *(transaction)* add warp runtime call support ([#65](https://github.com/bulletxyz/bullet-exchange-interface/pull/65))

## [0.6.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.5.0...v0.6.0) - 2026-05-29

### Features

- post only and close only trading statuses ([#61](https://github.com/bulletxyz/bullet-exchange-interface/pull/61))

## [0.5.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.4.1...v0.5.0) - 2026-05-27

### Features

- derive Default to simplify AdminAction::Update* calls ([#59](https://github.com/bulletxyz/bullet-exchange-interface/pull/59))
- support windowed uniqueness ([#57](https://github.com/bulletxyz/bullet-exchange-interface/pull/57))

## [0.4.1](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.4.0...v0.4.1) - 2026-05-19

### Bug Fixes

- use sov-universal-wallet from crates.io ([#55](https://github.com/bulletxyz/bullet-exchange-interface/pull/55))

## [0.4.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.3.0...v0.4.0) - 2026-05-18

### Features

- rwa-perps ([#51](https://github.com/bulletxyz/bullet-exchange-interface/pull/51))

## [0.3.0](https://github.com/bulletxyz/bullet-exchange-interface/compare/v0.2.0...v0.3.0) - 2026-05-11

### Features

- CancelAndPlace ([#14](https://github.com/bulletxyz/bullet-exchange-interface/pull/14))

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
