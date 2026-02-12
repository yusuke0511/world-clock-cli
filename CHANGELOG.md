# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-02-11

### Added
- Initial release
- Card display mode with 4-column layout
- Simple list display mode (`-s` flag)
- Real-time clock updates (1 second interval)
- Configuration file support (`~/.wclock/config.toml`)
- Support for 30+ country flags
- Graceful handling of invalid timezones
- Current timezone display at the top
- Default city list (20 major cities)
- Exit via Ctrl+C in both modes

### Features
- Beautiful Unicode box-drawing characters for card borders
- Country flag emojis for visual identification
- Modular code structure (config, display, timezone modules)
- IANA timezone database support via chrono-tz
- Crossterm-based terminal UI for cross-platform support
