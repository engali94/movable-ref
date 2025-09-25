# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.0] - 2025-09-25

### Added
- Optional `debug-guards` feature flag that enables runtime assertions for
  relative pointer integrity without impacting release builds.
- Safe accessor APIs (`try_as_ref`, `try_as_mut`) and the `SelfRefGuard` RAII type
  to provide ergonomic, checked access to self-referential data.
- Documentation for Miri and AddressSanitizer workflows, including explicit
  failure modes and nightly command snippets.
- `SelfRefCell::try_get` and `SelfRefCell::try_get_mut` so callers can detect
  uninitialised state without triggering undefined behaviour.
- Miri support for testing self-referential data structures.

### Changed
- `SelfRef` equality now compares offset and metadata rather than struct
  addresses, aligning behaviour with user expectations.
- Debug assertions now validate absolute pointers only when explicitly
  requested via `from_parts_with_target`, ensuring regular builds stay lean.

### Fixed
- Guard handling during mutable access ensures pointers are re-sealed on drop,
  preventing accidental reuse of stale offsets after interior mutation.

## [0.1.0] - 2025-06-25

### Added
- Initial public release with `SelfRef`, `SelfRefCell`, metadata support for
  sized and unsized types, and Criterion benchmarks.

[0.1.0]: https://github.com/engali94/movable-ref/releases/tag/v0.1.0
