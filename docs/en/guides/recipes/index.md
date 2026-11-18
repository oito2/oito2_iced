🌐 [Português](../../../pt-br/guides/recipes/index.md) | **English** | 🏠 [Index](../../index.md)

---

# Desktop Integration Recipes

Recipes for the parts of a desktop application that are **not** part of the OITO2 design system: a tray icon, notifications, starting at login and packaging. `oito2_iced` styles the windows; these recipes show how an application built on it talks to the desktop.

The code recipes are compiled by CI as the workspace crate [`examples/desktop`](../../../../examples/desktop/Cargo.toml) (package `oito2_iced_desktop_example`, not published). Each recipe is one small binary:

| Recipe | Binary | Platforms | Crates |
|---|---|---|---|
| [Tray icon](tray.md) | `tray` | Linux (StatusNotifierItem) | `ksni` |
| [Notifications](notifications.md) | `notify` | Linux (freedesktop notifications) | `notify-rust` |
| [Autostart](autostart.md) | `autostart` | Linux (XDG autostart) | standard library only |
| [Packaging](packaging.md) | — (guide only) | Linux | `cargo-deb`, `cargo-generate-rpm`, Flatpak |

Run a recipe from the repository root:

```bash
cargo run -p oito2_iced_desktop_example --bin tray
cargo run -p oito2_iced_desktop_example --bin notify
cargo run -p oito2_iced_desktop_example --bin autostart
```

The binaries also build on Windows and macOS, where they explain that the recipe is Linux-only. The pieces they share — the OITO2 theme, the window settings and a page layout with keyboard focus traversal — live in [`examples/desktop/src/lib.rs`](../../../../examples/desktop/src/lib.rs).
