🌐 [Português](../../pt-br/guides/desktop-integration.md) | **English** | 🏠 [Index](../index.md)

---

# Desktop integration

How an Iced app with oito2_iced fits into the desktop: an undecorated window with OITO2 chrome, the desktop settings followed live, an application id, keyboard shortcuts, and where to go for tray icons and packaging. The [quickstart](../getting-started/quickstart.md) puts the first three together in one file.

## The window

Open the window without system decorations and give it an application id on Linux, which Wayland compositors and X11 window managers use to group the window and match it with its desktop entry:

```rust
use iced::window;

fn settings() -> window::Settings {
    window::Settings {
        size: iced::Size::new(960.0, 640.0),
        min_size: Some(iced::Size::new(480.0, 360.0)),
        decorations: false,
        platform_specific: platform_specific(),
        ..window::Settings::default()
    }
}

#[cfg(target_os = "linux")]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        application_id: "org.example.MyApp".into(),
        ..Default::default()
    }
}

#[cfg(not(target_os = "linux"))]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
}
```

Pass it to the application with `.window(settings())`. Use the same id as the name of the app's `.desktop` file.

## The chrome

Draw the title bar and the resize borders yourself:

1. Put a `window::header_bar(Message::Window)` (or `window::title_bar`) at the top of the view, with the app's controls in its `start` and `end` slots.
2. Wrap the whole view in `window::resize_handles(view, Message::Window)`.
3. In `update`, run `oito2_iced::window::perform_latest(action)` for `Message::Window(action)` (or `perform(id, action)` with several windows).
4. Keep a `WindowState` from `iced::window::events()` and pass it to the bar with `.state(…)`, so it shows the restore glyph when maximized and dims when the window loses focus.

Drag, double click to maximize and the right-click window menu work on the bar's empty area.

## The desktop settings

With `oito2_iced_settings`:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
        oito2_iced_settings::subscription().map(Message::Settings),
        iced::window::events().map(|(id, event)| Message::WindowEvent(id, event)),
    ])
}
```

and use the settings in three places:

| Setting | Use |
|---|---|
| `accent_color`, `color_scheme`, `high_contrast` | `oito2_iced_settings::theme(&settings, None, Brightness::Light)` |
| `platform.density()` | `density::set(…)` in `view`, and `theme.with_density(…)` |
| `window_control_style`, `window_button_layout` | `.control_style(…)` and `.button_layout(…)` of the header bar |

The chrome then looks like GNOME on GNOME, Breeze on KDE Plasma, Xfwm on Xfce, COSMIC on COSMIC, and follows the button side and order each desktop is set to.

## Keyboard shortcuts

Iced reports key presses through `iced::keyboard::listen()`. Turn them into chords and look them up in a `CommandRegistry`, as the showcase does for Ctrl+K and Ctrl+,:

```rust
use oito2_iced::shortcuts::{Command, CommandRegistry, KeyChord, OperatingSystem, Shortcut};

let mut commands = CommandRegistry::new();
if let Ok(shortcut) = Shortcut::parse("Mod+K") {
    commands.register(
        Command::new("palette", "Command palette", Message::OpenPalette).with_shortcut(shortcut),
    );
}

// In update, for Message::Key(event) from keyboard::listen():
if let Some(chord) = KeyChord::from_keyboard_event(&event)
    && let Some(command) = commands.find_by_chord(&chord, OperatingSystem::current())
{
    let action = command.action.clone();
    return self.update(action);
}
```

`Mod` is Command on macOS and Control elsewhere. `widget::command_palette` shows the same registry as a searchable list.

## Fonts

Register the embedded fonts, so text looks the same on every system:

```rust
let app = font::INTER_FONTS
    .into_iter()
    .chain(oito2_iced_syntax::font::JETBRAINS_MONO_FONTS) // only with code blocks
    .fold(app, |app, bytes| app.font(bytes))
    .default_font(font::INTER);
```

## Tray icons, notifications, autostart and packaging

Resident apps, notifications, autostart entries and packages are outside the design system; the [recipes](./recipes/index.md) show how to build them around oito2_iced, including a tray app with `iced::daemon`.

## Platform notes

- **Wayland**: no file drag-and-drop in winit 0.30 (see [Files](./files.md)).
- **Windows and macOS**: best effort. The chrome is drawn in their style; there is no native macOS traffic-light integration.
- **No accessibility tree**: Iced 0.14 exposes nothing to screen readers.
