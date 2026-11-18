🌐 [Português](../../pt-br/guides/theming.md) | **English** | 🏠 [Index](../index.md)

---

# Theming

How to choose the colors of an app: accents, light and dark, high contrast, presets, the desktop settings, and per-widget classes.

## Give Iced the theme

Return an `oito2_iced::Theme` from the application's theme function. Views use it as the theme type of their elements (`Element<'_, Message, Theme>`, or the alias `oito2_iced::Element<'_, Message>`).

```rust
use oito2_iced::Theme;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::Accent;

fn theme(state: &App) -> Theme {
    if state.dark {
        Theme::new(Brightness::Dark, Accent::Green)
    } else {
        Theme::new(Brightness::Light, Accent::Green)
    }
}
```

| Constructor | Use it for |
|---|---|
| `Theme::light()`, `Theme::dark()` | The OITO2 base themes with the blue accent |
| `Theme::new(brightness, Accent)` | One of the nine accents: blue, teal, green, yellow, orange, red, pink, purple, slate |
| `Theme::base(brightness, color, Contrast)` | Any accent color, standard or high contrast |
| `Preset::theme(Contrast)` | A named preset of `oito2_iced_themes` |
| `oito2_iced_settings::theme(&settings, accent, fallback)` | The desktop accent, light or dark preference and contrast |
| `Theme::from_palette`, `Theme::from_roles` | Your own color family or roles |

## Follow the desktop

With `oito2_iced_settings`, keep the `SystemSettings` in the model, update them from `subscription()`, and build the theme from them. Passing `Some(color)` as the accent overrides the desktop accent; the fallback brightness applies when the desktop states no preference.

```rust
fn theme(&self) -> Theme {
    oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
}
```

See [Desktop integration](./desktop-integration.md) for the subscription.

## High contrast

`Contrast::High` raises text and outline contrast on every surface. `oito2_iced_settings::contrast(&settings)` returns it when the desktop asks for high contrast; pass it to `Theme::base` or to a preset.

## Presets

```rust
use oito2_iced::theme::Contrast;
use oito2_iced_themes::Preset;

let theme = Preset::Dracula.theme(Contrast::Standard);
```

| Family | Presets |
|---|---|
| Everforest | `EverforestLight`, `EverforestDark` |
| Rosé Pine | `RosePine`, `RosePineMoon`, `RosePineDawn` |
| Nord | `NordDark`, `NordLight` |
| Dracula | `Dracula` (dark), `Alucard` (light) |
| Catppuccin | `CatppuccinLatte`, `CatppuccinFrappe`, `CatppuccinMacchiato`, `CatppuccinMocha` |

`Preset::ALL` lists them, `label()` gives a display name, `name()` and `from_name()` a stable identifier for settings files, and `Preset::with_brightness(brightness)` the presets of one brightness.

## Read the colors

`theme.roles()` returns the `ColorRoles`: every Material 3 role (`primary`, `surface_container_high`, `outline_variant`…) plus:

| Group | Colors |
|---|---|
| `roles.chrome` | `background`, `sidebar`, `mainbar`, `title_bar`, `dialog`: the window surfaces |
| `roles.semantic` | `primary_text`, `error_text`, `success`, `warning`, `info`, `link`: text-safe tones |
| `roles.syntax` | The code colors used by `oito2_iced_syntax` |

```rust
use iced::widget::text;
use oito2_iced::Theme;

let saved = text("Saved").style(|theme: &Theme| text::Style {
    color: Some(theme.roles().semantic.success),
});
```

## Classes

Each native Iced widget picks a class of the OITO2 catalog with `.class(…)`; OITO2 builders such as `button::filled` already choose one.

| Catalog | Classes |
|---|---|
| `theme::button::Class` | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`, `Danger`, `Chrome`, `Fab`, `Icon` |
| `theme::container::Class` | `Transparent`, `Background`, `TitleBar`, `Sidebar`, `Mainbar`, `Card`, `Dialog`, `Popover`, `Menu`, `Tooltip`, `Inverse` |
| `theme::text::Class` | `Default`, `Variant`, `Primary`, `Error` |

```rust
use iced::widget::{container, text};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;

let sidebar = container(text("Library").class(TextClass::Variant)).class(ContainerClass::Sidebar);
```

Every class enum also has `Custom`, and accepts a closure through `From`, so Iced's `.style(…)` builder keeps working for one-off styles.

## Contrast helpers

`oito2_iced::theme::contrast` checks your own color pairs with the WCAG formulas: `ratio(a, b)`, `foreground_for(background)` and `ensure(color, &backgrounds, target)`, with the targets `TEXT_AA` (4.5), `NON_TEXT_AA` (3.0) and `TEXT_AAA` (7.0).

## Density

The theme carries a density (`with_density`); see [Layout and density](./layout-and-density.md).
