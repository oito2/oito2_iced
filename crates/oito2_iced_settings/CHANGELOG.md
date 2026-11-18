# Changelog

All notable changes to `oito2_iced_settings` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `read` returns the current desktop settings: accent color, light or dark preference, high contrast, window control style and button layout.
- `subscription` yields the settings at once and again after every change, on Linux, Windows and macOS.
- `theme` builds the OITO2 base theme that follows the settings, with an optional accent of the app and a fallback brightness, at the density of the platform.
- `brightness`, `contrast` and `accent` expose each rule on its own.
- Re-exports `SystemSettings`, `ColorScheme` and `Platform`.
