# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
### Changed
- Renamed fields in output validity proof in server-manager interface

## [0.12.0] - 2023-04-28
### Changed
- Changed processed input count in finish epoch request

## [0.11.0] - 2023-04-18
### Added
- Added new proof structure to finish epoch
- Added DeleteEpoch method to server manager
- Added SessionReplaceMemoryRange method on machine-manager
- Added ResetUarchState on Cartesi Machine
- Added GetUarchXAddress on Cartesi Machine
- Added Uarch halt flag on Cartesi Machine

### Changed
- Replaced epoch input index with global input index
- Moved hashes from get epoch status to finish epoch
- Renamed voucher.address to voucher.destination
- Renamed machine Step to StepUarch on Cartesi Machine
- Renamed machine UarchRun to RunUarch on Cartesi Machine
- Removed Uarch ROM on Cartesi Machine

### Removed
- Removed keccak fields from vouchers and notices

## [0.10.0] - 2023-02-16
### Changed
- Removed brkflag CSR
- Replaced minstret by icycleinstret CSR

## [0.9.0] - 2022-11-17
### Added
- Added microarchitecture configs
- Added TLB configs
- Added read/write virtual memory methods
- Added new CSRs related to the RISC-V specification

### Changed
- Removed DHD device

## [Previous Versions]
- [0.8.0]
- [0.7.0]
- [0.6.0]
- [0.5.0]
- [0.4.0]
- [0.3.0]
- [0.2.0]
- [0.1.3]
- [0.1.2]
- [0.1.1]

[Unreleased]: https://github.com/cartesi/grpc-interfaces/compare/v0.12.0...HEAD
[0.12.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.12.0
[0.11.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.11.0
[0.10.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.10.0
[0.9.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.9.0
[0.8.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.8.0
[0.7.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.7.0
[0.6.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.6.0
[0.5.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.5.0
[0.4.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.4.0
[0.3.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.3.0
[0.2.0]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.2.0
[0.1.3]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.3
[0.1.2]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.2
[0.1.1]: https://github.com/cartesi/grpc-interfaces/releases/tag/v0.1.1
