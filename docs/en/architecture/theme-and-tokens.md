🌐 [Português](../../pt-br/architecture/theme-and-tokens.md) | **English** | 🏠 [Index](../index.md)

---

# Theme and tokens

How `oito2_iced` turns an accent or a preset into the colors of every widget, and where the shared numbers live. Code: `crates/oito2_iced/src/{tokens.rs,theme/,font.rs,density.rs}` and `crates/oito2_iced_themes/src/`.

## Tokens

`tokens.rs` holds the values exported by oito2_ui in `tokens.json`, compared by the parity tests:

| Module | Contents |
|---|---|
| `tokens::radius` | `CHECK` 4, `SMALL` 6, `MEDIUM` 8, `LARGE` 12, `WINDOW` 14 |
| `tokens::spacing` | `XXS` 2, `XS` 4, `SM` 8, `MD` 12, `LG` 16, `PAGE` 20, `XL` 24, `XXL` 32, `XXXL` 48 |
| `tokens::sizes` | Button height 34 (touch 40), title bar 46, icon 20, focus border 2, window control 24, progress 6, minimum targets 24 (pointer) and 48 (touch) |
| `tokens::typography` | The 15 Material type styles (`DISPLAY_LARGE` … `LABEL_SMALL`), the emphasis weight and the font fallback list |
| `tokens::Accent` | The nine accents and their colors |
| `tokens::Neutrals`, `tokens::Status` | The GTK-style neutral surfaces and the status colors, light and dark |

## From an accent to the color roles

```text
accent color + brightness + contrast
        │
        ▼
material-colors: SchemeTonalSpot (HCT)    ── tonal palettes from the seed
        │
        ▼
theme::roles: compose the Material 3 roles over the OITO2 neutrals,
              chrome, semantic and syntax colors
        │
        ▼
contrast::ensure: move each text role until it reaches its WCAG target
              against the surfaces it sits on
        │
        ▼
ColorRoles  ──►  Theme
```

- `ColorRoles::base(brightness, accent, contrast)` builds the roles of the OITO2 base themes; `ColorRoles::resolve(&Palette, contrast)` builds them from the raw colors of a family (`Palette`), which is how presets work.
- `theme::contrast` holds the WCAG helpers: `ratio`, `foreground_for`, `min_ratio`, `ensure`, and the targets `TEXT_AA` (4.5), `NON_TEXT_AA` (3.0) and `TEXT_AAA` (7.0).
- The parity tests compare every role of every base theme (nine accents × light and dark × standard and high contrast) with `schemes.json`, and every contrast case with `foundation.json`.

## The `Theme`

`Theme` holds the resolved `ColorRoles`, a name and a density, behind a shared pointer so cloning is cheap.

| Constructor | Builds |
|---|---|
| `Theme::new(brightness, Accent)` | A base theme from one of the nine accents |
| `Theme::base(brightness, Color, Contrast)` | A base theme from any accent color and a contrast level |
| `Theme::light()`, `Theme::dark()` | The base themes with the fallback (blue) accent |
| `Theme::from_palette(name, &Palette, Contrast)` | A theme from the raw colors of a family |
| `Theme::from_roles(name, ColorRoles)` | A theme from roles built by hand |

`with_density`, `with_name`, `brightness`, `contrast`, `density`, `roles`, `control_height`, `icon_size` and `focus_ring` complete it. `Theme` implements `iced::theme::Base`, so Iced can also pick a light or dark default by itself.

## Style catalogs and classes

`theme/` has one submodule per native Iced widget: `button`, `checkbox`, `container`, `float`, `menu`, `pane_grid`, `pick_list`, `progress_bar`, `radio`, `rule`, `scrollable`, `slider`, `table`, `text`, `text_editor`, `text_input`, `toggler`, and behind features `svg`, `qr_code` and `markdown`. Each one implements the widget's `Catalog` for `Theme` with a `Class` enum:

```rust
pub enum Class<'a> {
    Filled,
    Tonal,
    Elevated,
    Outlined,
    Text,
    Danger,
    Chrome,
    Fab,
    Icon,
    Custom(StyleFn<'a, Theme>),
}
```

`impl From<StyleFn> for Class` keeps the `.style(|theme, status| …)` builder of each Iced widget working; choosing a class with `.class(…)` needs Iced's `advanced` feature, which `oito2_iced` enables. State layers follow Material: `HOVER_OPACITY` 0.08, `PRESSED_OPACITY` 0.10, disabled containers at 0.12 and disabled content at 0.38.

## Presets

`oito2_iced_themes::Preset` lists the 13 presets. Each one has its official palette (`palettes::catppuccin` reads the `catppuccin` crate, the others are written out), turned into `FamilyColors`, then into a `Palette` (`Palette::from_family`), then into `ColorRoles` with the same contrast rules as the base themes. The parity tests compare all 13 presets × standard and high contrast with `schemes.json`, and the 212 palette colors.

## Fonts

`font.rs` embeds eight static instances of Inter (Regular, Medium, SemiBold, Bold and their italics) in the family `OITO2 Inter`; `oito2_iced_syntax::font` embeds four of JetBrains Mono (Regular, Bold and italics) in `OITO2 JetBrains Mono`. Static instances are required: Iced's text stack matches a face by exact weight, so the other weights of a variable font fall back to a system font. The own family names keep a copy of Inter installed on the system from replacing them. The fonts are under the SIL Open Font License (`assets/fonts/OFL.txt`).

## Density

`density::set(Density)` stores the pointer or touch density in a thread-local; widgets read `density::control_height()` and `density::icon_size()` while they lay out. The app calls it at the start of `view`, with the density of its theme, so layout and style agree.
