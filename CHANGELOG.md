# Changelog

All notable changes to GTRusthop will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
### Changed
### Deprecated
### Removed
### Fixed
### Security

## [1.2.1] - 2025-08-04

### Added
- Complete thread-local storage elimination with builder pattern architecture
- Cross-platform development commands and setup guide in `docs/cross-platform-commands.md`
- Comprehensive documentation restructuring with centralized docs/ directory
- Enhanced performance benchmarks comparing recursive vs iterative strategies
- Documentation Index with learning paths and detailed cross-references
- Comprehensive table of contents in main README.md with 40+ clickable navigation links
- Platform-specific development setup for Windows, Linux, and macOS
- Zero compiler warnings requirement with clean, thread-safe design

### Changed
- **BREAKING**: Complete elimination of thread-local storage in favor of builder pattern
- Moved `Rust.md` and `runtime_benchmark.md` to `docs/` directory for better organization
- Updated all documentation links to reflect new file locations in docs/ directory
- Improved README.md with comprehensive navigation and enhanced Documentation section
- Restructured documentation files for optimal GitHub presentation and developer experience
- Enhanced multigoal management system with automatic cleanup and better memory management
- Updated all learning paths to use current documentation structure

### Fixed
- Removed all references to obsolete "TO BE DELETED" directory files
- Corrected broken internal links in documentation (docs/examples.md section numbering)
- Updated migration paths to use current documentation structure
- Fixed relative paths in all cross-references after file reorganization
- Eliminated race conditions and thread safety issues through builder pattern migration
- Resolved documentation flow inconsistencies after removing obsolete references

### Removed
- All thread-local storage (`thread_local!` macros) completely eliminated
- References to deprecated MIGRATION_SUMMARY.md and other obsolete files
- Empty `tests/` directory (testing handled through examples with embedded unit tests)
- Manual cleanup requirements for multigoal management (now automatic)

## [Release Candidate] - 2025-08-03

### Added
- Initial builder pattern implementation
- Enhanced error handling and validation
- Improved planning strategy selection

### Changed
- Refactored core planning algorithms
- Updated domain creation patterns

### Fixed
- Memory management improvements
- Planning verification enhancements

[Unreleased]: https://github.com/gtrusthop/gtrusthop/compare/v1.2.1...HEAD
[1.2.1]: https://github.com/gtrusthop/gtrusthop/compare/release-candidate...v1.2.1
[Release Candidate]: https://github.com/gtrusthop/gtrusthop/releases/tag/release-candidate
