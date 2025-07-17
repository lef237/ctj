# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).

## [Unreleased]

There are no changes yet.

## [0.1.8] - 2025-07-17

### Added
- Support for piped stdin input - you can now pipe CSV data directly to ctj (e.g., `cat file.csv | ctj`)
- Short option `-n` for `--no-header` flag to improve usability
- Comprehensive test coverage for stdin functionality (6 new tests)

### Changed
- Input handling now supports both file input and stdin seamlessly
- Updated README.md with stdin usage examples and documentation
- Enhanced CLI argument parsing to handle optional input files
- Updated README.md to reflect the new `-n` shorthand option
- Updated usage examples to demonstrate stdin functionality and the `-n` option

### Technical
- Refactored input processing to use `Reader<Box<dyn Read>>` for unified file/stdin handling
- Modified `Config` struct to use `Option<String>` for input field
- All existing functionality preserved with no breaking changes
