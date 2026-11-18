# Changelog

All notable changes to `oito2_iced_syntax` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `code_block`: code with the syntax colors of the theme, line numbers, a copy button, horizontal scrolling and right-to-left support; recolors on theme changes without highlighting again.
- `Highlighter` for Iced's `text_editor`, resuming from the changed line.
- `highlight` and `highlight_cached` return runs of tokens with their role.
- 27 languages with aliases, through the TextMate grammars of `two-face` on `syntect`.
- `Role` with the 11 token roles of oito2_ui and the table from TextMate scopes to roles.
- JetBrains Mono embedded as four static faces (Regular, Bold and italics) in the family `OITO2 JetBrains Mono`, with its license.
