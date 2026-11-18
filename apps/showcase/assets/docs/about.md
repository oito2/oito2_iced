# About

oito2_iced is the design system of the OITO2 apps written in pure Rust with Iced: the same look and feel as oito2_ui for Flutter and oito2_uikit for Tauri, for desktop apps that run without a webview.

## Principles

- **Desktop first, touch ready.** Density, control sizes and targets follow the platform: compact 34-pixel controls with a pointer, 40-pixel ones on touch screens.
- **One design, checked.** Tokens, colors, contrast, strings and formats come from oito2_ui, and the tests compare them with its exported fixtures; text and controls reach the WCAG 2.2 AA contrast ratios on every scheme.
- **Keyboard first.** Every control takes the keyboard focus, shows a focus ring and works with Tab, the arrow keys, Enter, Space and Escape. Iced 0.14 has no accessibility tree yet, so screen readers can't read the components.
- **Iced underneath.** The theme styles the native Iced widgets too, so an app can mix them with OITO2 components.
- **Native window chrome.** Title bars and window controls follow GNOME, KDE, Xfce, COSMIC, Windows and macOS, and the theme follows the desktop accent, dark style and high contrast while the app runs.

## Platforms

| Platform | Status |
|---|---|
| Linux (X11 and Wayland, every desktop environment) | Supported, primary |
| Windows | Best effort: builds and passes the tests in CI |
| macOS | Best effort: builds and passes the tests in CI |

## Inspiration

The catalogue and its behavior follow oito2_ui, whose components follow the yaru.dart widgets of Ubuntu and the Ark UI headless components; this showcase is organized like the oito2_ui showcase.

## License

oito2_iced is free software under the GNU General Public License, version 3 or later. The Inter and JetBrains Mono fonts it bundles are under the SIL Open Font License, and the Lucide icons under the ISC License.

## Source

The source code, the issue tracker and the documentation are on [GitHub](https://github.com/oito2/oito2_iced).
