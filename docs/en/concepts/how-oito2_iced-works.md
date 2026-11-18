🌐 [Português](../../pt-br/concepts/how-oito2_iced-works.md) | **English** | 🏠 [Index](../index.md)

---

# How oito2_iced works

A tour of the pieces an app touches, from the theme to the window. The [architecture overview](./architecture.md) covers the data flow and the design decisions; the [architecture pages](../architecture/theme-and-tokens.md) go into each subsystem.

![The showcase with a component page open](../../img/showcase-component.png)
<!-- TODO: capture the showcase with a component page open, light theme -->

## 1. The theme

`oito2_iced::Theme` is the Iced theme of the app. It is built from a brightness, an accent and a contrast level — or from a preset of `oito2_iced_themes`, or from the desktop settings with `oito2_iced_settings::theme` — and resolves every color role: the Material 3 roles, the window surfaces (`chrome`), the status colors (`semantic`) and the code colors (`syntax`).

The theme implements the style `Catalog` of every native Iced widget. Each widget picks a **class**: `button::Class::Filled`, `container::Class::Sidebar`, `text::Class::Variant`, and so on, or `Custom` with a closure for a one-off style.

```rust
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;

let theme = Theme::base(Brightness::Dark, Accent::Green.color(), Contrast::High);
let success = theme.roles().semantic.success;
```

## 2. Tokens and density

`oito2_iced::tokens` holds the numbers every widget shares: `spacing` (2 to 48 pixels), `radius` (6, 8, 12 and the window radius 14), `sizes` (34-pixel controls for a pointer, 40 on touch) and the `typography` scale. Iced lays widgets out without the theme, so the density lives in `oito2_iced::density`: the app calls `density::set` at the start of `view` and the widgets read it while they measure.

## 3. Widgets

The widgets live in `oito2_iced::widget`, one module per family (`button`, `toggles`, `entries`, `menu`, `tab_bar`…). Each is a function that returns a builder:

```rust
use oito2_iced::widget::button;

let save = button::filled("Save").on_press(Message::Save);
```

The app owns every value: a widget shows what it receives and publishes messages (`on_press`, `on_change`, `on_toggle`…). State with several parts lives in plain structs the app keeps in its model, such as `SelectionController`, `TreeState`, `Tour` or `Toasts`, which the app updates from the events the widgets publish.

## 4. Keyboard focus

Iced 0.14 gives keyboard focus only to text inputs. oito2_iced adds it to everything:

- `focus::focusable` wraps any element so it takes the focus, draws the focus ring and publishes a message on Enter and Space; every OITO2 control uses it.
- `focus::navigation` wraps the root of the view and moves the focus with Tab and Shift+Tab.
- Groups such as tab bars, segment groups and list boxes are a single Tab stop whose items move with the arrow keys.

## 5. Overlays and root hosts

Popovers, menus, selects, hover cards, tooltips, autocomplete lists and tour cards open in an **anchored** overlay next to the element they belong to, flip to the other side when there is no room, stay inside the window and close on Escape or a click outside.

Surfaces that cover the whole window wrap the root of the view instead: `toast::toaster`, `dialog::host`, `sheet::host`, `tour::host` and the command palette. The showcase stacks them like this:

```text
resize_handles
└── focus::navigation
    └── toast::toaster
        └── command_palette
            └── dialog::host
                └── sheet::host
                    └── tour::host
                        └── header bar + page
```

## 6. Window chrome

An app opens its window with `decorations: false` and draws the chrome itself: `window::header_bar` (title, start and end slots, search) or `window::title_bar`, window controls in six styles, and `window::resize_handles` around the whole view. The chrome publishes `window::Action`s that the app runs with `window::perform_latest`, and follows the `WindowState` the app keeps from the window events.

![The header bar in each window control style](../../img/window-controls.png)
<!-- TODO: capture the header bar in the GNOME, Breeze, Xfwm, COSMIC, Windows and macOS styles -->

## 7. System settings

`oito2_iced_settings` reads the desktop through `oito2_settings`: accent color, light or dark preference, high contrast, window control style and button layout. `subscription()` delivers them at once and after every change, so the theme and the chrome follow the desktop while the app runs.

## 8. The showcase

`apps/showcase` is an Iced app with one page per component, grouped like the Flutter showcase (Overview, Guides, Components, Extras, Files, Utilities, Theme), a search field, a command palette (Ctrl+K) and a settings dialog (Ctrl+,) for the theme, preset, contrast, density, language and simulated desktop.

![The showcase settings dialog](../../img/showcase-settings.png)
<!-- TODO: capture the showcase settings dialog -->
