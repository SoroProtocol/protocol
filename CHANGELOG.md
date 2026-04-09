# Changelog

## [Unreleased]
- Pausable streams (#27)
- NFT receipts for active streams (#23)
- Governance module for fee parameters (#31)

## [0.2.0] - 2026-03-01
### Added
- Vesting contract with cliff support
- Distributor contract for batch streams
- Integration test suite
### Fixed
- `balance_of` now caps elapsed time at `stop_time` (#7)

## [0.1.0] - 2026-01-25
### Added
- Core stream contract: create, withdraw, cancel
- Stream events: StreamCreated, Withdrawn, Cancelled
