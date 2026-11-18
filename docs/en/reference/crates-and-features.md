🌐 [Português](../../pt-br/reference/crates-and-features.md) | **English** | 🏠 [Index](../index.md)

---

# Crates and features

Every crate of the workspace, its public modules, Cargo features and dependencies. All crates share version `0.0.0`, edition 2024, `rust-version = "1.97"` and the GPL-3.0-or-later license, and none is published yet (`publish = false`). The rustdoc of each item is the full API reference: `cargo doc --workspace --no-deps --all-features --open`.

## `oito2_iced`

Path `crates/oito2_iced`. The core crate.

| Module | Contents |
|---|---|
| `theme` | `Theme`, `ColorRoles` (with `ChromeColors`, `SemanticColors`, `SyntaxColors`), `Palette`, `FamilyColors`, `Brightness`, `Contrast`, `contrast` helpers, one catalog submodule per native widget with its `Class` |
| `tokens` | `radius`, `spacing`, `sizes`, `typography`, `Accent`, `Neutrals`, `Status` |
| `font` | The eight embedded Inter faces (`INTER_FONTS`), `INTER`, `inter(weight)`, `inter_italic(weight)`, `INTER_FAMILY` |
| `icon` | `Icon`, `lucide` (every Lucide icon, generated), `Freedesktop` (freedesktop.org names), `icon` and `Glyph` (with `svg`), `placeholder`, `animated` |
| `density` | `set`, `get`, `control_height`, `icon_size` and the `Density` re-export |
| `focus` | `navigation`, `focusable`, `ring`, `edge_interceptor`, `operations` |
| `overlay` | `anchored`, `place`, `Placement`, `Alignment`, `Side` |
| `widget` | Every OITO2 widget, one module per family; see [Components](./components.md) |
| `window` | `title_bar`, `header_bar`, `controls`, `window_control`, `resize_handles`, `Action`, `perform`, `perform_latest`, `WindowState`, `Labels` |
| `animation` | Material easing curves and durations over Iced's `Animation` |
| `collections` | `ListCollection`, `SelectionController`, `TreeCollection`, `AsyncListController`, `fold_text` |
| `format` | `Format`: byte sizes, compact numbers, relative times |
| `shortcuts` | `KeyChord`, `Shortcut`, `Command`, `CommandRegistry`, `SequenceMatcher` |
| `l10n` | `Localizations` in English and Brazilian Portuguese |
| `date` | `Date` and `Time` |
| `foundation` | Re-export of `oito2_foundation`: platform, desktop environment, window control style and button layout |

The crate root also exports `Theme` and the alias `Element<'a, Message>` (an Iced element with the OITO2 theme).

### Features

| Feature | Default | Enables | Needed by |
|---|---|---|---|
| `svg` | Yes | Iced's `svg`: the SVG style catalog and the Lucide glyphs | Icon buttons and FAB, `icon::icon`, copy button and field, password and search fields, calendar, date picker and date/time entries, pagination, and the icons of menus, tabs, toasts, info boxes and other widgets |
| `image` | No | Iced's `image` | Photos in `avatar` |
| `qr_code` | No | Iced's `qr_code` | The QR code style catalog (used by `oito2_iced_extras`) |
| `markdown` | No | Iced's `markdown` | The Markdown style catalog |

Iced is a dependency without default features plus `advanced`, `canvas`, `x11`, `wayland` and `thread-pool`; the app enables a renderer (`wgpu` and/or `tiny-skia`, both in Iced's defaults).

## `oito2_iced_settings`

Path `crates/oito2_iced_settings`. Depends on `oito2_iced` and `oito2_settings` (feature `watch`).

| Item | Description |
|---|---|
| `read()` | The current `SystemSettings` |
| `subscription()` | The settings at once and after every change |
| `theme(&settings, accent, fallback)` | The base theme that follows the settings |
| `brightness`, `contrast`, `accent` | Each rule alone |
| `SystemSettings`, `ColorScheme`, `Platform` | Re-exports of `oito2_settings` |

## `oito2_iced_themes`

Path `crates/oito2_iced_themes`. Depends on `oito2_iced` and `catppuccin`.

| Item | Description |
|---|---|
| `Preset` | The 13 presets: `ALL`, `label`, `name`, `from_name`, `is_dark`, `brightness`, `with_brightness`, `family`, `palette`, `theme(contrast)` |
| `palettes` | The official palettes of Catppuccin, Dracula/Alucard, Everforest, Nord and Rosé Pine |

## `oito2_iced_syntax`

Path `crates/oito2_iced_syntax`. Depends on `oito2_iced`, `syntect` and `two-face`.

| Module | Contents |
|---|---|
| `code_block` | `code_block(&l10n, code)`: line numbers, copy button, horizontal scrolling, JetBrains Mono |
| `highlighter` | `highlight`, `highlight_cached`, `Highlighter` for Iced's `text_editor::highlight_with` |
| `language` | `LANGUAGES` (27 languages with aliases), `find`, `supports` |
| `role` | `Role` (11 token roles) and the TextMate scope table |
| `font` | Four JetBrains Mono faces (`JETBRAINS_MONO_FONTS`), `JETBRAINS_MONO`, `JETBRAINS_MONO_LICENSE` |

## `oito2_iced_extras`

Path `crates/oito2_iced_extras`. Depends on `oito2_iced` (features `image` and `qr_code`), `qrcode` and `image`.

| Module | Contents |
|---|---|
| `qr_code` | `qr_code`: square or rounded modules, center image, error levels |
| `signature_pad` | `signature_pad` and the `Signature` model, with SVG and PNG export |
| `image_cropper` | `image_cropper`, `preview` and the `Crop` model; `crop()` returns the cropped image |
| `navigation_menu` | `navigation_menu` with link and panel items, and the `MenuState` model |

## `oito2_iced_files`

Path `crates/oito2_iced_files`. Depends on `oito2_iced` and `rfd` (features `xdg-portal` and `wayland`).

| Item | Contents |
|---|---|
| `pick` | The file chooser, as a `Task` |
| `file` | `File`, `TypeGroup`, `matches_types` |
| `drop_zone` | `drop_zone`, `can_drop_files` |
| `upload` | `Controller`, `file_upload`, `RejectedFile`, `Rejection` |
| `download` | `save`, `download_trigger`, `is_supported`, `Error` |

## Not published

| Crate | Path | Purpose |
|---|---|---|
| `oito2_iced_test_utils` | `crates/oito2_iced_test_utils` | Simulator settings with the embedded fonts, snapshot pixels, tolerant goldens (`golden!`), cursor reading |
| `oito2_iced_showcase` | `apps/showcase` | The showcase app |
| `oito2_iced_desktop_example` | `examples/desktop` | The code of the desktop [recipes](../guides/recipes/index.md): tray icon, notifications, autostart |
| `oito2_iced_parity` | `tools/parity` | Parity fixtures |
| `oito2_iced_icons` | `tools/icons` | Lucide icon module |
| `oito2_iced_gen_docs` | `tools/gen_docs` | API reference of the showcase |
