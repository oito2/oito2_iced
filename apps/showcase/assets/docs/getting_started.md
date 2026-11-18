# Getting Started

oito2_iced is the OITO2 design system for Iced 0.14: a Material 3 theme for every Iced widget, desktop-grade density, GTK-style surfaces, keyboard focus for every control and client-side window chrome, for native Rust apps on Linux, with Windows and macOS on a best-effort basis. This page sets up an app with the theme, the fonts, keyboard navigation and a first component.

## Requirements

| Tool | Version |
|---|---|
| Rust | 1.97 or newer (edition 2024) |
| Iced | 0.14 |
| Linux packages | `libxkbcommon-dev` and `libglib2.0-dev` (Debian and Ubuntu names) |

## Install

The crates aren't on crates.io yet; add them from the Git repository, next to Iced:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }
# Optional crates, as you need them:
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_themes = { git = "https://github.com/oito2/oito2_iced" }
```

| Crate | What it adds |
|---|---|
| `oito2_iced` | The theme, the tokens, the fonts, the icons, keyboard focus, the window chrome and every core component |
| `oito2_iced_settings` | Reads the system accent, color scheme, contrast and window buttons, and follows their changes |
| `oito2_iced_themes` | The named color presets, such as Dracula and Catppuccin |
| `oito2_iced_syntax` | The code block, a highlighter for the text editor and the JetBrains Mono font |
| `oito2_iced_extras` | QR codes, signature pad, image cropper and navigation menu |
| `oito2_iced_files` | File chooser, drop zone, file upload and the download trigger |

## Set up the app

Register the embedded Inter faces and make them the default font, give the application the OITO2 `Theme`, set the density at the start of `view`, and wrap the view in `focus::navigation` so Tab and Shift+Tab move the focus.

```rust
use iced::Element;
use iced::widget::container;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::Accent;
use oito2_iced::{Theme, density, focus, font};

fn main() -> iced::Result {
    let app = iced::application(App::default, App::update, App::view).theme(App::theme);

    font::INTER_FONTS
        .into_iter()
        .fold(app, |app, bytes| app.font(bytes))
        .default_font(font::INTER)
        .run()
}

#[derive(Default)]
struct App {
    wifi: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Wifi(bool),
}

impl App {
    fn theme(&self) -> Theme {
        Theme::new(Brightness::Light, Accent::Blue)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Wifi(wifi) => self.wifi = wifi,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        density::set(self.theme().density());

        focus::navigation(container(home(self.wifi)).padding(20)).into()
    }
}
```

> `oito2_iced::l10n::Localizations` holds the texts of the components, such as tooltips and the labels of the window buttons, in English and Brazilian Portuguese. Widgets that show text of their own take it as an argument.

## Use a component

Components are functions that return a builder. The value lives in your app: the component shows it and publishes a message when the user changes it. A switch with a clickable label, for example:

```rust
use oito2_iced::widget::{controls, toggles};

fn home<'a>(wifi: bool) -> oito2_iced::Element<'a, Message> {
    controls::switch_button(toggles::switch(wifi).on_toggle(Message::Wifi), "Wi-Fi").into()
}
```

## Follow the desktop

With `oito2_iced_settings`, the theme follows the accent color, the dark style and high contrast of the desktop while the app runs:

```rust
use iced::Subscription;
use oito2_iced_settings::SystemSettings;

fn subscription(&self) -> Subscription<Message> {
    oito2_iced_settings::subscription().map(Message::Settings)
}

fn theme(&self) -> Theme {
    oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
}
```

## Next steps

- Read the guides on **Styling**, **Composition** and **Component State** to learn the patterns every component shares.
- Browse the components in the navigation: each page has working examples, their code, and the API reference.
- Run this showcase from a clone of the repository with `cargo run -p oito2_iced_showcase`; set `ICED_BACKEND=tiny-skia` to use the software renderer.
