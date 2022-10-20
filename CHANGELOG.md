# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

<<<<<<< HEAD
## [1.3.0] - 2022-02-04

### Changed

-   Updated arbitration-dlib
-   Updated machine-solidity-step
-   Updated logger
-   Updated machine-manager image

## [1.2.2] - 2021-09-29

-   Fixed build issue due to changes in Rust Cargo dependencies

=======
- Fixed build issue due to changes in Rust Cargo dependencies
## [1.2.1] - 2021-07-24

### Changed

-   Removed 1K hard limit for direct input drives

## [1.2.0] - 2021-06-27

### Added

-   Support for input drives with no assigned provider (a.k.a. "off-chain drives")
-   Docker Compose template supporting known networks, which allows testing Cartesi Compute with local nodes pointing at those networks

### Changed

-   Challenger node now by default submits a confirmation transaction when results match, which may incur in additional fees but speeds up execution
-   Fixed off-chain support for downloading Logger data from Matic Testnet
-   No longer trying to download Logger data from IPFS when no IPFS configuration is specified

## [1.1.1] - 2021-03-28

### Changed

-   Fixed Docker image configuration so that off-chain dispatcher services can use supported testnet deployments

## [1.1.0] - 2021-03-26

### Added

-   Support for Avalanche FUJI C-Chain Testnet

### Changed

-   Fixed handling of WaitingReveals state in CartesiCompute.getResult

## [1.0.0] - 2021-01-20

-   First release

### Added

-   Multi-party support: now any number of peers can be specified for a Cartesi Compute computation, although it is still strongly recommended to be just a few to avoid risking long periods of time for disputes (e.g., around 6-8 peers, reviously only 2 peers were allowed, claimer and challenger).
-   IPFS drives: if parties cooperate, larger volumes of data can now be uploaded only to IPFS (i.e., it is no longer required that data must always be posted on-chain). The drive provider will still need to post the data to the Logger if there is disagreement on the IPFS data.

### Changed

-   Upgraded on-chain code to Solidity 0.7
-   Packaging and deployment tooling migrated from Truffle to Hardhat

[unreleased]: https://github.com/cartesi/descartes/compare/v1.3.0...HEAD
[1.3.0]: https://github.com/cartesi/descartes/releases/tag/v1.3.0
[1.2.2]: https://github.com/cartesi/descartes/releases/tag/v1.2.2
[1.2.1]: https://github.com/cartesi/descartes/releases/tag/v1.2.1
[1.2.0]: https://github.com/cartesi/descartes/releases/tag/v1.2.0
[1.1.1]: https://github.com/cartesi/descartes/releases/tag/v1.1.1
[1.1.0]: https://github.com/cartesi/descartes/releases/tag/v1.1.0
[1.0.0]: https://github.com/cartesi/descartes/releases/tag/v1.0.0
