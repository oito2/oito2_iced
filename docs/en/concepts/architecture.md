🌐 [Português](../../pt-br/concepts/architecture.md) | **English** | 🏠 [Index](../index.md)

---

# Architecture

A high-level overview of oito2_iced: how data flows through it, where each piece lives, which dependencies it uses and the design decisions behind it. The per-subsystem pages for contributors are under [`architecture/`](../architecture/theme-and-tokens.md).

## Data flow

### From the desktop to the theme

```text
Operating system
  (GSettings, configuration files and the settings portal on Linux,
   Windows Runtime, AppKit)
        │  read by
        ▼
oito2_settings (crates.io, from oito2_rs)
  read_system_settings() / watch()  ──►  SystemSettings
        │
        ▼
oito2_iced_settings
  read()          first reading, in the app's `new`
  subscription()  the reading again after every change
        │  Message::Settings(SystemSettings)
        ▼
App model  ──►  oito2_iced_settings::theme(&settings, accent, fallback)
        │                      or Preset::theme(contrast)
        ▼
oito2_iced::Theme  ──►  Iced style catalogs of every widget
        │
        └──►  header bar: settings.window_control_style, window_button_layout
```

### From a widget to the app and back

```text
view(&model)  ──►  widget builders (values from the model)
                       │  user input: click, keys, drag
                       ▼
                  Message (on_press, on_change, Event…)
                       │
update(&mut model, message)
   ├── plain values: assign
   ├── controllers: SelectionChange::apply, Toasts::update, Tour::apply…
   └── tasks: window::perform_latest, pick(), download::save()…
                       │
                       ▼
                  view(&model) again
```

Widgets never keep the value they show: the model is the single source of truth, as in every Elm-style Iced app. What a widget keeps in its own tree state is transient (focus, hover, animation progress, text being typed).

## Crate layout

| Crate | Depends on | Holds |
|---|---|---|
| `oito2_iced` | `iced`, `material-colors`, `oito2_foundation` | Theme, tokens, fonts, icons, density, focus, overlay, widgets, window chrome, animation, collections, formats, shortcuts, strings, dates |
| `oito2_iced_settings` | `oito2_iced`, `oito2_settings` | `read`, `subscription`, `theme` and helpers |
| `oito2_iced_themes` | `oito2_iced`, `catppuccin` | `Preset` and the official palettes |
| `oito2_iced_syntax` | `oito2_iced`, `syntect`, `two-face` | Token roles, languages, highlighter, code block, JetBrains Mono |
| `oito2_iced_extras` | `oito2_iced`, `qrcode`, `image` | QR code, signature pad, image cropper, navigation menu |
| `oito2_iced_files` | `oito2_iced`, `rfd` | Files and type groups, file chooser, drop zone, upload, download |
| `oito2_iced_test_utils` | `oito2_iced`, `iced_test`, `png` | Test helpers; not published |
| `oito2_iced_showcase` (`apps/showcase`) | every crate above | The showcase app |
| `oito2_iced_desktop_example` (`examples/desktop`) | `oito2_iced` and desktop crates | The code of the desktop recipes |
| `oito2_iced_parity`, `oito2_iced_icons`, `oito2_iced_gen_docs` (`tools/`) | `serde_json`, `syn` | Parity fixtures, the Lucide icon module and the showcase API reference |

Inside `oito2_iced`, one module per concern: `theme` (roles, contrast, one submodule per native widget catalog), `tokens`, `font`, `icon`, `density`, `focus`, `overlay`, `widget` (one module per family), `window`, `animation`, `collections`, `format`, `shortcuts`, `l10n`, `date`, and `foundation` (a re-export of `oito2_foundation`).

## Main dependencies

| Dependency | Version | Why |
|---|---|---|
| `iced` | 0.14 | The toolkit. The core crate enables only `advanced`, `canvas`, `x11`, `wayland` and `thread-pool`; apps choose the renderers |
| `material-colors` | 0.5 | Tonal color schemes from a seed (HCT), the base of every role |
| `oito2_foundation`, `oito2_settings` | 0.1.1 | Platform rules and desktop settings shared by every OITO2 Rust interface |
| `catppuccin` | 2.8 | The official Catppuccin palettes |
| `syntect` + `two-face` | 5.3 / 0.5 | TextMate grammars for syntax highlighting, without the Oniguruma C library |
| `rfd` | 0.18 | File dialogs; on Linux through the XDG desktop portal, in pure Rust |
| `qrcode` | 0.13 | QR code bit streams and error correction |
| `image` | 0.25 | Decoding and cropping images |
| `iced_test` | 0.14 | The headless simulator behind the behavior tests |

## Design decisions

| Decision | Rationale |
|---|---|
| oito2_ui is the design source of truth | One design for three runtimes; the parity fixtures catch drift in CI. |
| A custom `Theme` implementing every native `Catalog`, with classes as enums plus `Custom` | Native widgets and OITO2 widgets share one style system; `Custom` keeps an escape hatch, and `From<StyleFn>` keeps Iced's `style` builder working. |
| Widgets at three levels: styled native widget, composition, custom widget | A native widget is used when it does the job; custom widgets only for layout, state or overlays Iced lacks. |
| Own focus layer (`focusable`, `navigation`, roving groups) | Iced 0.14 focuses only text inputs and doesn't bind Tab; the layer is a widget, so the headless simulator exercises it. |
| One anchored overlay foundation | Placement, flip, clamping, arrow, dismiss and nesting are written once and reused by every popup. |
| Client-side window chrome with resize handles | One look on every desktop; Iced has no resize borders for undecorated windows. |
| Density as a thread-local setting | Iced lays widgets out without the theme. |
| Static font instances (`OITO2 Inter`, `OITO2 JetBrains Mono`) | The text stack matches faces by exact weight, so variable fonts fall back to system fonts; the own family name keeps an installed Inter from replacing them. |
| No accessibility tree | Iced 0.14 has no AccessKit integration; declared, not hidden. |
| Dates as own `Date` and `Time` types | No calendar dependency; the formats come from the parity fixtures. |
| Iced pinned to 0.14 in `Cargo.lock` | Minor Iced versions break the API; migrations happen as a block. |
