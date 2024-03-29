# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] <!-- ReleaseDate -->

## [2.3.0]  - 2023-08-24

- add optional postcard MaxSize
- add optional serde Serialize, Deserialize

## [2.2.0]  - 2023-08-24

- enums: Eq + PartialEq

## [2.1.0]  - 2023-08-24

- bump dependencies
- expose `JrkG2error`

## [2.0.0] - 2022-01-10

- remove HEADER
- rename `JrkG2Serial` → `Serial`
- rename `JrkG2I2c` → `I2c`
- rename `JrkG2BlockingI2c` → `BlockingI2c`
- rename `jrk-g2-rs` → `jrk-g2`
- ufmt optional
- bump edition to 2021
- clean

## [1.0.1] - 2020-12-30

- Update metadata

## [1.0.0] - 2020-12-28

- First release
- Split main struct into a JrkG2 trait implemented by JrkG2I2c / JrkG2BlockingI2c / JrkG2Serial
- Added examples for i²c / serial with stm32 / rpi / arduino

<!-- next-url -->
[Unreleased]: https://github.com/nim65s/jrk-g2-rs/compare/v2.3.0...HEAD
[2.3.0]: https://github.com/nim65s/jrk-g2-rs/compare/v2.2.0...v2.3.0
[2.2.0]: https://github.com/nim65s/jrk-g2-rs/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/nim65s/jrk-g2-rs/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/nim65s/jrk-g2-rs/compare/v1.0.1...v2.0.0
[1.0.1]: https://github.com/nim65s/jrk-g2-rs/compare/v1.0.0...v1.0.1
[1.0.0]: https://github.com/nim65s/jrk-g2-rs/releases/tag/v1.0.0
