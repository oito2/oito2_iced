🌐 [Português](../../pt-br/concepts/why-oito2_iced.md) | **English** | 🏠 [Index](../index.md)

---

# Why oito2_iced

## The problem

OITO2 apps share one visual identity across three runtimes: Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)), Rust with Tauri v2 and Svelte ([oito2_uikit](https://github.com/oito2/oito2_uikit)), and pure Rust. The first two had a design system; pure Rust desktop apps did not. A Rust app without a webview had to pick between a toolkit that brings another desktop's design and building every widget by hand.

There was also no complete Material 3 design system for Iced: the native widgets ship with a small built-in theme, and the ecosystem around them has no catalogue of desktop components with keyboard focus, overlays and window chrome.

## What oito2_iced does

- Brings the OITO2 identity to Iced: the same tokens, color roles, presets, contrast rules, strings and window chrome as oito2_ui, checked against its parity fixtures in the tests.
- Styles every native Iced widget through one `Theme`, so native and OITO2 widgets mix.
- Adds 84 core components, from buttons and entries to tree views, tours and floating panels, each with keyboard focus and Tab traversal.
- Draws client-side window chrome in the style of GNOME, KDE (Breeze), Xfce (Xfwm), COSMIC, Windows and macOS, and follows the desktop settings live.

## Why Iced

| Toolkit | Why it was or wasn't chosen |
|---|---|
| **Iced 0.14** | **Chosen.** 100 % Rust, the Elm architecture, its own renderer (wgpu with a tiny-skia software fallback), full control of every pixel, and a style `Catalog` per widget that a custom theme can implement. The COSMIC desktop builds on a fork of it. |
| Slint | Its own markup language (a fourth language for the project) and fewer escape hatches for overlays. |
| GPUI | Made for the Zed editor, with an unstable API and almost no accessibility. |
| egui | Immediate mode; a faithful Material 3 look is hard. |
| Xilem, Dioxus Native | Experimental, with few widgets. |
| gtk4-rs + libadwaita, libcosmic | Mature, but they bring the design of another desktop. |

## Trade-offs

- **No accessibility tree.** Iced 0.14 has no AccessKit integration upstream, so screen readers can't read the widgets. Keyboard use is complete: every control takes the focus, Tab moves it and the focus ring shows where it is.
- **Pre-1.0 toolkit.** Iced changes its API between minor versions. The workspace pins Iced 0.14 in `Cargo.lock` and moves to a new minor version as a block.
- **Linux first.** Windows and macOS build and pass the tests in CI, with the window chrome drawn in their style, but there is no promise of native integration (no native macOS traffic lights, for example).
- **Not published yet.** The crates are used as Git dependencies until the first release.

## oito2_iced or oito2_uikit?

Both give a Rust desktop app the OITO2 look; the choice depends on the app:

| Prefer | When |
|---|---|
| **oito2_uikit** (Tauri v2 + Svelte) | The user interface is large and accessibility matters: the components run in a webview, with screen reader support. |
| **oito2_iced** (Iced) | The app is resident and light, such as a tray daemon with windows on demand; it should run without a webview; or the target distributions don't guarantee WebKitGTK. |

## When not to use it

- The app needs screen reader support today: use oito2_uikit or oito2_ui.
- The app targets mobile or the web: oito2_iced is desktop only; oito2_ui covers mobile and the web.
- The app should look like its desktop (GNOME, KDE) rather than like OITO2: use that desktop's toolkit.
