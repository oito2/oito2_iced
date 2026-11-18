# Styling

oito2_iced is styled through the Iced theme. One `oito2_iced::Theme` styles every OITO2 component and the native Iced widgets alike, so an app changes its look in one place: the theme function of the application.

## Build the theme

`Theme` resolves every color role from an accent, a brightness and a contrast level. Its density, which sets the control sizes, comes from the platform or from the app.

| Builder | Use it for |
|---|---|
| `Theme::light()` / `Theme::dark()` | The OITO2 base scheme with the blue accent |
| `Theme::new(brightness, Accent::Green)` | One of the nine OITO2 accents |
| `Theme::base(brightness, color, Contrast::High)` | Any accent color, standard or high contrast |
| `oito2_iced_settings::theme(&settings, None, fallback)` | The system accent, color scheme and contrast, read by `oito2_iced_settings` |
| `Preset::Dracula.theme(contrast)` | A named preset of `oito2_iced_themes` |

```rust
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;

fn theme(dark: bool) -> Theme {
    let brightness = if dark { Brightness::Dark } else { Brightness::Light };
    Theme::base(brightness, Accent::Green.color(), Contrast::Standard)
}
```

## Color schemes

A theme holds `ColorRoles`: the Material 3 roles plus the OITO2 extras, with every text role checked for WCAG AA contrast. `Theme::from_palette` builds one from the dozen colors a theme family defines; the `oito2_iced_themes` crate ships 13 named presets built that way.

```rust
use oito2_iced::theme::Contrast;
use oito2_iced_themes::Preset;

let theme = Preset::CatppuccinMocha.theme(Contrast::Standard);
```

## Theme colors

Colors the Material roles don't cover live in groups of `ColorRoles`. Read them from the theme in a style closure:

| Group | Colors |
|---|---|
| `roles().semantic` | `primary_text`, `error_text`, `success`, `warning`, `info`, `link`: text-safe status and accent tones |
| `roles().chrome` | `background`, `sidebar`, `mainbar`, `title_bar`, `dialog`: the window surfaces |
| `roles().syntax` | The code colors of the code block and the JSON tree |

```rust
use iced::widget::text;
use oito2_iced::Theme;

let saved = text("Saved").style(|theme: &Theme| text::Style {
    color: Some(theme.roles().semantic.success),
});
```

## Design tokens

Spacing, radii and sizes are constants, so custom layouts line up with the components.

| Token module | Examples |
|---|---|
| `tokens::spacing` | `XXS` (2), `XS` (4), `SM` (8), `MD` (12), `LG` (16), `PAGE` (20), `XL` (24), `XXL` (32) |
| `tokens::radius` | `SMALL` (6), `MEDIUM` (8), `LARGE` (12), `WINDOW` (14) |
| `tokens::sizes` | `BUTTON_HEIGHT` (34), `TOUCH_BUTTON_HEIGHT` (40), `ICON_SIZE` (20) |

```rust
use iced::widget::{container, text};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::tokens::spacing;

let card = container(text("Storage"))
    .padding(spacing::MD)
    .class(ContainerClass::Card);
```

## Component variants

Components offer their variations as builder methods instead of separate widgets: a style enum, a variant function or a color.

```rust
use oito2_iced::widget::button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::tab_bar::{self, TabBarStyle};
use oito2_iced::widget::tags_field::{self, TagsFieldStyle};

let save = button::tonal("Save").on_press(Message::Save);
let tabs = tab_bar::tab_bar([tab_bar::tab("Files"), tab_bar::tab("Shared")], selected)
    .style(TabBarStyle::Subtle)
    .on_select(Message::Tab);
let tinted = border_container(content).tint(theme.roles().semantic.success);
let labels = tags_field::tags_field(&tags)
    .style(TagsFieldStyle::Subtle)
    .on_change(Message::Tags);
```

## Override one component

Every native Iced widget picks a class of the OITO2 catalog with `.class(…)`; every class enum also has a `Custom` variant, and Iced's `.style(…)` builder still takes a closure for a one-off look.

| Catalog | Classes |
|---|---|
| `theme::button::Class` | `Filled`, `Tonal`, `Elevated`, `Outlined`, `Text`, `Danger`, `Chrome`, `Fab`, `Icon` |
| `theme::container::Class` | `Transparent`, `Background`, `TitleBar`, `Sidebar`, `Mainbar`, `Card`, `Dialog`, `Popover`, `Menu`, `Tooltip`, `Inverse` |
| `theme::text::Class` | `Default`, `Variant`, `Primary`, `Error` |

```rust
use iced::widget::{button, text};
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::text::Class as TextClass;

let chrome = button(text("Menu")).class(ButtonClass::Chrome);
let hint = text("Optional").class(TextClass::Variant);
```
