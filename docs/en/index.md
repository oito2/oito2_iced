# Documentation — oito2_iced

🌐 [Português](../pt-br/index.md) | **English** | 🏠 [Back to README](../../README.md)

---

oito2_iced is a design system for native Rust desktop apps built on Iced 0.14, Linux first, with Windows and macOS on a best-effort basis. It gives apps without a webview the same visual identity as the Flutter design system oito2_ui and the Tauri design system oito2_uikit — GTK-style neutrals, a 34 px desktop density, Material 3 color roles, 13 presets and WCAG-checked contrast — with 84 core components that all take keyboard focus, client-side window chrome in six styles, and satellite crates for system settings, presets, syntax highlighting, extras and files.

---

## 🚀 Getting Started

- [Prerequisites](./getting-started/prerequisites.md) — the Rust toolchain and the system packages per operating system.
- [Installation](./getting-started/installation.md) — adding the crates to an app and building the showcase from source, per operating system.
- [Quickstart](./getting-started/quickstart.md) — the shortest path to an app with the OITO2 theme, fonts, keyboard focus and window chrome.

---

## 🧠 Concepts

- [Why oito2_iced](./concepts/why-oito2_iced.md) — the problem it solves, why Iced, and when to choose oito2_uikit instead.
- [How oito2_iced works](./concepts/how-oito2_iced-works.md) — a tour from the theme to widgets, focus, overlays, window chrome and system settings.
- [Architecture overview](./concepts/architecture.md) — data flow, crate layout, main dependencies and key design decisions.
- [Glossary](./concepts/glossary.md) — the project's terms, from color roles to goldens.

---

## 🏗️ Architecture (for contributors)

- [Theme and tokens](./architecture/theme-and-tokens.md) — tokens, color roles, contrast, the style catalogs, presets and fonts.
- [Focus and keyboard](./architecture/focus-and-keyboard.md) — the focusable wrapper, Tab traversal, roving focus, focus rings and shortcuts.
- [Overlay](./architecture/overlay.md) — the anchored panel behind popovers, menus and tooltips, and the root hosts for dialogs, sheets, toasts and tours.
- [Window chrome](./architecture/window-chrome.md) — title bar, header bar, window controls, resize handles and the window state.
- [Animation](./architecture/animation.md) — Material curves and durations, and how widgets animate without subscriptions.
- [System settings](./architecture/system-settings.md) — from the desktop stores to the theme, through `oito2_settings` and the subscription.
- [Parity and generators](./architecture/parity-and-generators.md) — parity fixtures, the icon and font generators, and the CI workflows.
- [Testing and goldens](./architecture/testing-and-goldens.md) — the simulator, golden images, audits and the test layout.

---

## 🧭 Guides

- [Theming](./guides/theming.md) — accents, light and dark, high contrast, presets, custom roles and style classes.
- [Layout and density](./guides/layout-and-density.md) — pointer and touch sizes, spacing tokens and responsive layouts.
- [Localization](./guides/localization.md) — English and Portuguese strings, formats, dates and reading direction.
- [Files](./guides/files.md) — file chooser, drop zone, upload validation and downloads.
- [Desktop integration](./guides/desktop-integration.md) — undecorated windows, window chrome, system settings and application ids.
- [Recipes](./guides/recipes/index.md) — tray icons with `iced::daemon`, notifications, autostart and packaging.

---

## 📚 Reference

- [Crates and features](./reference/crates-and-features.md) — every crate, its modules, Cargo features and dependencies.
- [Components](./reference/components.md) — every component page of the showcase and the Rust module behind it.
- [Showcase pages](./reference/showcase-pages.md) — every page of the showcase, by group.
- [Repository tools](./reference/repository-tools.md) — Cargo commands, tools, environment variables and CI workflows.

---

## 🛠️ Troubleshooting

- [Common issues](./troubleshooting/common-issues.md) — build, rendering, fonts, files and test problems, indexed by symptom.

---

## 🤝 Contributing

- [Contributing guide](../../CONTRIBUTING.md) — development environment, checks, tests and goldens, and the release process.
- [Code of Conduct](../../CODE_OF_CONDUCT.md) and [Security Policy](../../SECURITY.md).
