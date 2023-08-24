# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog], and this project adheres to
[Semantic Versioning].

## [Unreleased]


## [0.2.0] - 2023-08-24

### Added
- add features: `nightly_docs`, `alloc`.
- add changelog.

### Changed
- rename reexported `log` crate to `log_crate` and uninline it.
- remove `safe` from default features
- update manifest categories.
- bump MSRV to `1.71.1`.

### Fixed
- remove unneeded wraning allowances.
- update CI and `check.sh` script.

## [0.1.0] - 2023-07-06

### Added

### Removed
- delete duplicated `log_enabled` macro.
- add features `unsafe`, `no-std`.
- add new `all` root module.
- add `check.sh` script.

### Changed
- bump deps versions.
- bump MSRV to `1.65.0`
- update aliases.
- made modules public.

### Fixes
- refactor manifest and reexported modules
- fix doc compilation error.
- update examples, CI.

## [0.0.3] - 2023-03-16

### Changes
- bump MSRV to `1.63.0`.

### Fixes
- fix no-std build.
- update licenses, CI, readme.

## [0.0.2] - 2023-02-09

### Added
- re-export the `log` crate.

### Changes
- rename `Error` & `Result` types.

### Fixes
- fix macros.

## [0.0.1] - 2023-02-07

### Added
- add `Logger` and `MultiLogger`.
- add `ScopeTime` and related functions.
- add `timeit!` macro and related functions.
- add examples.


[unreleased]: https://github.com/andamira/depura/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/andamira/depura/releases/tag/v0.2.0
[0.1.0]: https://github.com/andamira/depura/releases/tag/v0.1.0
[0.0.3]: https://github.com/andamira/depura/releases/tag/v0.0.3
[0.0.2]: https://github.com/andamira/depura/releases/tag/v0.0.2
[0.0.1]: https://github.com/andamira/depura/releases/tag/v0.0.1

[Keep a Changelog]: https://keepachangelog.com/en/1.0.0/
[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
