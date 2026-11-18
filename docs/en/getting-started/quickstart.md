🌐 [Português](../../pt-br/getting-started/quickstart.md) | **English** | 🏠 [Index](../index.md)

---

# Quickstart

The shortest path to an Iced app with the OITO2 look: the embedded fonts, a theme that follows the desktop settings, an undecorated window with the OITO2 header bar and resize handles, keyboard focus, and a first component. It assumes the [prerequisites](./prerequisites.md) are installed.

## 1. Create the app

```bash
cargo new hello-oito2
cd hello-oito2
```

Add the dependencies to `Cargo.toml`:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
```

## 2. Write the app

Replace `src/main.rs`:

```rust
use iced::widget::{column, container};
use iced::{Element, Length, Subscription, Task, window};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::{controls, toggles};
use oito2_iced::window::{Labels, Placement, WindowState};
use oito2_iced::{Theme, density, focus, font};
use oito2_iced_settings::SystemSettings;

fn main() -> iced::Result {
    let app = iced::application(App::new, App::update, App::view)
        .title("Hello OITO2")
        .theme(App::theme)
        .subscription(App::subscription);

    // The embedded Inter faces, so every weight renders in the OITO2 font.
    font::INTER_FONTS
        .into_iter()
        .fold(app, |app, bytes| app.font(bytes))
        .default_font(font::INTER)
        .window(window::Settings {
            decorations: false,
            ..window::Settings::default()
        })
        .run()
}

struct App {
    settings: SystemSettings,
    window: WindowState,
    wifi: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Settings(SystemSettings),
    Window(oito2_iced::window::Action),
    WindowEvent(window::Id, window::Event),
    Placement(Placement),
    Wifi(bool),
}

impl App {
    fn new() -> Self {
        Self {
            settings: oito2_iced_settings::read(),
            window: WindowState::default(),
            wifi: true,
        }
    }

    fn theme(&self) -> Theme {
        oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            oito2_iced_settings::subscription().map(Message::Settings),
            window::events().map(|(id, event)| Message::WindowEvent(id, event)),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Settings(settings) => self.settings = settings,
            Message::Window(action) => return oito2_iced::window::perform_latest(action),
            Message::WindowEvent(id, event) => {
                if self.window.update(&event) {
                    return WindowState::query(id).map(Message::Placement);
                }
            }
            Message::Placement(placement) => self.window.apply(placement),
            Message::Wifi(wifi) => self.wifi = wifi,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        // The widgets measure themselves with the density of the theme.
        density::set(self.settings.platform.density());

        let header = oito2_iced::window::header_bar(Message::Window)
            .title("Hello OITO2")
            .control_style(self.settings.window_control_style)
            .button_layout(self.settings.window_button_layout.clone())
            .state(self.window)
            .labels(Labels::new(&Localizations::en()));

        let content = container(controls::switch_button(
            toggles::switch(self.wifi).on_toggle(Message::Wifi),
            "Wi-Fi",
        ))
        .padding(spacing::PAGE)
        .width(Length::Fill)
        .height(Length::Fill);

        oito2_iced::window::resize_handles(
            focus::navigation(column![header, content]),
            Message::Window,
        )
    }
}
```

## 3. Run it

```bash
cargo run
```

The window opens without system decorations, with the header bar in the style and button layout of your desktop. Drag the header to move the window, double-click it to maximize, and drag the edges to resize. Press **Tab** to move the focus to the switch, then **Space** to toggle it. Change the accent color, the dark style or high contrast in the desktop settings: the app follows them while it runs.

## What each piece does

| Piece | Role |
|---|---|
| `font::INTER_FONTS`, `font::INTER` | Registers the embedded static Inter faces (family `OITO2 Inter`) and makes them the default font |
| `oito2_iced_settings::theme` | Builds the theme from the desktop accent, light or dark preference and high contrast |
| `oito2_iced_settings::subscription` | Delivers the settings right away and after every change |
| `density::set` | Tells the widgets the pointer or touch sizes they lay out with |
| `focus::navigation` | Moves the keyboard focus with Tab and Shift+Tab |
| `window::header_bar`, `resize_handles` | The client-side chrome of the undecorated window |
| `WindowState` | Whether the window is maximized and focused, for the chrome |

## Next steps

- Browse the components in the showcase (`cargo run -p oito2_iced_showcase` in a clone of the repository); the [component reference](../reference/components.md) maps each page to its module.
- Read [Theming](../guides/theming.md) and [Desktop integration](../guides/desktop-integration.md).
