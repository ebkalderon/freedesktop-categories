# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2019-02-15
### Added
* Add crate LICENSE file (#5).

### Changed
* Update `map.rs` to reflect the latest spec version.
* Upgrade `phf` dependency in main crate to version 0.7.24.
* Upgrade `amxml` dependency in codegen crate to version 0.5.3.
* Upgrade `curl` dependency in codegen crate to version 0.4.19.
* Upgrade `phf_codegen` dependency in codegen crate to version 0.7.24.

### Fixed
* Update incorrect repo URL in codegen crate (#7).
* Eliminate unused `Result` warning with `writeln!` in codegen crate.

## 0.1.0 - 2018-09-29
### Added
* Initial crate release.
* Static hash map based on the latest Desktop Menu specification.
* Codegen crate for automatic parsing of the upstream DocBook XML.

[Unreleased]: https://github.com/ebkalderon/freedesktop-categories/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/ebkalderon/freedesktop-categories/compare/v0.1.0...v0.2.0
