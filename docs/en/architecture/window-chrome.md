🌐 [Português](../../pt-br/architecture/window-chrome.md) | **English** | 🏠 [Index](../index.md)

---

# Window chrome

How an undecorated Iced window gets a title bar, window controls and resize borders in the style of each desktop. Code: `crates/oito2_iced/src/window/`.

## Actions instead of side effects

The chrome never touches the window itself. Every builder takes an `on_action: impl Fn(window::Action) -> Message` mapping and publishes:

| `Action` | Published by | Window task |
|---|---|---|
| `Drag` | Press on the empty area of the bar | `iced::window::drag` |
| `ToggleMaximize` | Double click on the bar, maximize/restore button | `iced::window::toggle_maximize` |
| `ShowSystemMenu` | Right press on the bar | `iced::window::show_system_menu` |
| `Minimize` | Minimize button | `iced::window::minimize` |
| `Close` | Close button | `iced::window::close` |
| `Resize(Direction)` | Press on one of the eight resize handles | `iced::window::drag_resize` |

The app turns them into tasks with `window::perform(id, action)`, or `window::perform_latest(action)` for a single-window app. Keeping the side effect in `update` lets the simulator tests check exactly which action each gesture publishes.

## Title bar and header bar

`window::title_bar(title, start, on_action)` and `window::header_bar(on_action)` build a `TitleBar`:

- `title` or `title_element` in the middle, replaced by `search` while the app shows a search field;
- `start` items after the leading window buttons and `end` items before the trailing ones;
- `control_style(WindowControlStyle)` and `button_layout(WindowButtonLayout)` from the desktop settings (or chosen by the app);
- `state(WindowState)` for the maximize/restore glyph and the inactive look;
- `labels(Labels::new(&localizations))` for the button tooltips;
- `center_title`, `leading_inset`, `height` (46 by default) and `radius`.

The title is centered except in the Windows style. While the window is not focused, the title and the buttons are drawn at `INACTIVE_OPACITY` (0.7). A line in the `outline_variant` role runs along the bottom edge.

## Window controls

`window::controls` and `window::window_control` draw the buttons of seven `WindowControlStyle`s — GNOME, Breeze, Xfwm, COSMIC, Windows, macOS, and none — as canvas geometry (which is why the core crate enables Iced's `canvas` feature). `WindowButtonLayout` (from `oito2_foundation`) sets which buttons appear and on which side, parsed from the desktop's own setting where there is one (for example GNOME's `button-layout`).

## Resize handles

Iced upstream has no resize border for undecorated windows. `window::resize_handles(content, on_action)` wraps the whole view with eight invisible handles — edges `RESIZE_BORDER` (6) pixels thick and corners `RESIZE_CORNER` (12) pixels wide — that show the matching resize cursor and publish `Action::Resize(direction)`. The content stays fully usable under them.

## Window state

`WindowState { maximized, focused, fullscreen }` lives in the app model:

1. Subscribe to `iced::window::events()`.
2. Call `state.update(&event)`; it returns `true` when the placement may have changed.
3. Then run `WindowState::query(id)` and apply the resulting `Placement` with `state.apply(placement)`.

## Platform notes

- **Linux** is the primary target on X11 and Wayland, on every desktop environment. Set the Wayland application id through `window::Settings::platform_specific` (the showcase uses `io.github.oito2.oito2_iced.showcase`).
- **Windows** gets the Windows control style with the title at the start.
- **macOS** gets the macOS control style drawn by the chrome; there are no native traffic lights.

The real behavior in each window manager (drag, double click, system menu, resize cursors) is checked by hand; the automated tests check the published actions in the simulator.
