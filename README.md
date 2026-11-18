# oito2_iced

A Linux-first design system for native Rust desktop apps built on [Iced](https://iced.rs) 0.14, with the identity of [oito2_ui](https://github.com/oito2/oito2_ui): Material 3 color roles, a 34 px desktop density, 13 color presets, WCAG-checked contrast and client-side window chrome in six styles.

[![CI](https://github.com/oito2/oito2_iced/actions/workflows/ci.yml/badge.svg)](https://github.com/oito2/oito2_iced/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.97%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Iced](https://img.shields.io/badge/iced-0.14-3645FF)](https://iced.rs)
[![Platform](https://img.shields.io/badge/platform-Linux-informational?logo=linux)](docs/en/getting-started/installation.md)
[![License](https://img.shields.io/github/license/oito2/oito2_iced?label=license)](LICENSE)

🌐 **Language:** English · [Português](docs/pt-br/leiame.md)

## Table of Contents

- [Overview](#overview)
- [Prerequisites & Quick Installation](#prerequisites--quick-installation)
- [Update & Maintenance](#update--maintenance)
- [Documentation](#documentation)
- [License](#license)

## Overview

OITO2 apps are written in Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)), in Rust with Tauri v2 and Svelte ([oito2_uikit](https://github.com/oito2/oito2_uikit)) and in pure Rust. oito2_iced covers the last case: desktop apps with no webview, drawn by Iced, that look like the other two. oito2_ui stays the design source of truth; its parity fixtures (tokens, color schemes, contrast, strings, formats, syntax colors, QR matrices) are copied into this repository and checked by the tests, and a weekly workflow compares them with oito2_ui `main`.

What it offers:

- **A theme for every Iced widget** — `oito2_iced::Theme` implements the style catalog of every native Iced widget, with the Material 3 color roles, the window surfaces, status colors and syntax colors of oito2_ui, light and dark, nine accents, standard and high contrast.
- **84 components in the core crate** — buttons, toggles, entries, pickers, menus, popovers, dialogs, sheets, toasts, tabs, navigation rail, master-detail, list box, tree view, carousel, tour, floating panel and more, each with keyboard focus, Tab traversal and a focus ring.
- **Window chrome** — title bar, header bar, window controls in the GNOME, Breeze, Xfwm, COSMIC, Windows and macOS styles, and resize handles for undecorated windows.
- **System settings** — accent color, light or dark preference, high contrast, window control style and button layout, read from the desktop and followed while the app runs.
- **Presets** — Everforest, Rosé Pine, Nord, Dracula and Alucard, and Catppuccin, 13 in all.
- **Satellite crates** — syntax highlighting with code blocks, QR codes, a signature pad, an image cropper, a navigation menu, and file upload, drop zone and download.
- **No assistive technology support yet** — Iced 0.14 has no accessibility tree (no AccessKit upstream), so screen readers can't read the widgets. Keyboard use is fully supported.

oito2_iced is licensed under the GPL-3.0-or-later: an app that uses it must be distributed under a compatible license.

![The oito2_iced showcase](docs/img/showcase.png)
<!-- TODO: capture a screenshot of the showcase main window -->

### Crates

| Crate | Contents |
|---|---|
| [`oito2_iced`](crates/oito2_iced) | Theme, tokens, fonts, icons, focus, overlay, widgets, window chrome, animation, collections, formats, shortcuts and strings |
| [`oito2_iced_settings`](crates/oito2_iced_settings) | Desktop settings as an Iced subscription, and the theme that follows them |
| [`oito2_iced_themes`](crates/oito2_iced_themes) | The 13 color presets |
| [`oito2_iced_syntax`](crates/oito2_iced_syntax) | Syntax highlighting, the code block and the JetBrains Mono font |
| [`oito2_iced_extras`](crates/oito2_iced_extras) | QR code, signature pad, image cropper and navigation menu |
| [`oito2_iced_files`](crates/oito2_iced_files) | Drop zone, file upload, file chooser and download |

## Prerequisites & Quick Installation

### Supported systems

| OS | Status | Minimum version / distributions | Runtime dependencies |
|---|---|---|---|
| Linux | Supported (primary) | X11 or Wayland; built and tested on Ubuntu (GitHub `ubuntu-latest`) | `libxkbcommon`, GLib, a Vulkan or OpenGL driver (or the software renderer) |
| Windows | Best effort | Builds and passes the tests in CI (`windows-latest`); no promise of native integration | — |
| macOS | Best effort | Builds and passes the tests in CI (`macos-latest`); no promise of native integration | — |

The project produces no installers or binaries yet: the crates are used as Git dependencies and the showcase is built from source.

### Requirements

- [rustup](https://rustup.rs): the repository pins Rust **1.99.0** in `rust-toolchain.toml`, which rustup installs on first use. The crates build with Rust **1.97** or newer.
- On Debian and Ubuntu, the development files of `libxkbcommon` and GLib:

  ```bash
  sudo apt install libxkbcommon-dev libglib2.0-dev
  ```

The [installation guide](docs/en/getting-started/installation.md) covers every system and the [quickstart](docs/en/getting-started/quickstart.md) builds a first app.

### Try the showcase

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

Set `ICED_BACKEND=tiny-skia` to force the software renderer.

### Add the crates to an app

The crates aren't on crates.io yet; add them from the Git repository:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }
# Optional crates, as you need them:
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_themes = { git = "https://github.com/oito2/oito2_iced" }
```

Register the embedded fonts, use the OITO2 theme and wrap the view in the focus navigation:

```rust
use iced::Element;
use oito2_iced::{Theme, focus, font};
use oito2_iced::widget::button;

fn main() -> iced::Result {
    font::INTER_FONTS
        .into_iter()
        .fold(iced::application(App::default, App::update, App::view), |app, bytes| {
            app.font(bytes)
        })
        .default_font(font::INTER)
        .theme(|_: &App| Theme::light())
        .run()
}

#[derive(Default)]
struct App {
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        focus::navigation(
            button::filled(format!("Clicked {} times", self.count)).on_press(Message::Increment),
        )
        .into()
    }
}
```

## Update & Maintenance

- **An app** that depends on the Git repository moves to the newest commit with `cargo update -p oito2_iced` (one `-p` per OITO2 crate it uses).
- **The showcase** built from a clone updates with `git pull` followed by `cargo run -p oito2_iced_showcase`.

Each crate records its changes in its own `CHANGELOG.md` (for example [`crates/oito2_iced/CHANGELOG.md`](crates/oito2_iced/CHANGELOG.md)).

## Documentation

- [Documentation index](docs/en/index.md) — everything below, and more.
- [Installation](docs/en/getting-started/installation.md) and [Quickstart](docs/en/getting-started/quickstart.md).
- [Why oito2_iced](docs/en/concepts/why-oito2_iced.md) — including when to choose oito2_uikit instead.
- [Architecture overview](docs/en/concepts/architecture.md).
- [Guides](docs/en/index.md#-guides) — theming, layout and density, localization, files and desktop integration.
- [Components](docs/en/reference/components.md) — every showcase page and the Rust module behind it.
- [Contributing](CONTRIBUTING.md), [Code of Conduct](CODE_OF_CONDUCT.md) and [Security Policy](SECURITY.md).

## License

GPL-3.0 — see [LICENSE](LICENSE).
