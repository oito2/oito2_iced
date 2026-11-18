🌐 [Português](../../pt-br/reference/showcase-pages.md) | **English** | 🏠 [Index](../index.md)

---

# Showcase pages

The showcase (`apps/showcase`, package `oito2_iced_showcase`) lists its pages in `apps/showcase/src/catalog.rs`, in seven groups, in the order of the sidebar. Each page is a module of `apps/showcase/src/pages/` with the same name as its identifier, which the command palette also uses (`Go to <page>`).

```bash
cargo run -p oito2_iced_showcase
```

## Overview

| Page | Identifier | Content |
|---|---|---|
| Getting Started | `getting_started` | Setting up an app (`apps/showcase/assets/docs/getting_started.md`) |
| Changelog | `changelog` | The `CHANGELOG.md` of every published crate |
| About | `about` | The project (`apps/showcase/assets/docs/about.md`) |

## Guides

The patterns every component shares, from `apps/showcase/assets/docs/guides/`.

| Page | Identifier |
|---|---|
| Styling | `styling` |
| Composition | `composition` |
| Component State | `component_state` |
| Animations | `animations` |
| Forms | `forms` |
| Refs | `refs` |

## Components, Extras and Files

84 component pages for `oito2_iced`, four for `oito2_iced_extras` and two for `oito2_iced_files`. [Components](./components.md) lists them all with their Rust modules.

## Utilities

| Page | Identifier | Rust module |
|---|---|---|
| Collections | `collections` | `oito2_iced::collections` |
| Commands | `commands` | `oito2_iced::shortcuts`, `oito2_iced::widget::command_palette` |
| Format | `format` | `oito2_iced::format` |
| FreedesktopIcons | `freedesktop_icons` | `oito2_iced::icon::Freedesktop` |

## Theme

| Page | Identifier | Rust module |
|---|---|---|
| Material theme | `material_theme` | `oito2_iced::theme`, `oito2_iced::tokens` |
| Foundation | `foundation` | `oito2_iced::foundation` (re-export of `oito2_foundation`) |

## Window-wide features

| Feature | How to open it |
|---|---|
| Search of the sidebar | The field at the top of the sidebar filters pages by title |
| Command palette | Ctrl+K (Command+K on macOS) |
| Settings dialog | Ctrl+, (Command+, on macOS), or the Settings button of the header bar: theme, color theme (preset), contrast, density, language and simulated desktop |
| Software renderer | Start with `ICED_BACKEND=tiny-skia` |
