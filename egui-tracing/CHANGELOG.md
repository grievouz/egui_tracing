# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Upgrade egui, egui_extras and the eframe examples from 0.36 to 0.37. Consumers must use compatible egui 0.37 types.
- Follow egui 0.37 font defaults: applications choose their fonts; egui-default no longer implicitly enables bundled fonts.

## [0.4.0](https://github.com/grievouz/egui_tracing/compare/egui_tracing-v0.3.0...egui_tracing-v0.4.0) - 2026-11-19

### Changed

- Upgrade egui, egui_extras and the eframe examples from 0.34 to 0.36. Consumers must use compatible egui 0.36 types.

### Added

- Add generic timestamp collectors and log views, with an optional `time` backend and support for custom clocks. Existing chrono-based aliases and defaults remain available.

### Fixed

- Enable browser clock support automatically on wasm32 targets; the `wasmbind` feature remains available for compatibility.

## [0.3.0](https://github.com/grievouz/egui_tracing/compare/egui_tracing-v0.2.6...egui_tracing-v0.3.0) - 2026-04-10

### Added

- improve labels and UI component initialization
- add internationalization support for labels
- improve logging UI and collector configuration
- add expandable log rows to show full event details
- allow target menu to close on click outside

### Other

- update preview image
- update dependency versioning ([#40](https://github.com/grievouz/egui_tracing/pull/40))
- disable resizing and use default frame for detail panel
- improve logging performance and UI responsiveness
- optimize event collection and filtering with caching
- Replace custom table with egui_extras
- update egui and eframe to 0.34

## [0.2.6](https://github.com/grievouz/egui_tracing/compare/egui_tracing-v0.2.5...egui_tracing-v0.2.6) - 2024-09-27

### Added

- add toggle for default features of sub-dependencies

### Other

- remove Box and dyn trait objects for improved performance

## [0.2.5](https://github.com/grievouz/egui_tracing/compare/egui_tracing-v0.2.4...egui_tracing-v0.2.5) - 2024-09-17

### Added

- enhance hover text with more comprehensive content ([#22](https://github.com/grievouz/egui_tracing/pull/22))

### Fixed

- remove color override and use workspace dependencies ([#21](https://github.com/grievouz/egui_tracing/pull/21))

## [0.2.4](https://github.com/grievouz/egui_tracing/compare/egui_tracing-v0.2.3...egui_tracing-v0.2.4) - 2024-09-15

### Other

- add symlink to readme
