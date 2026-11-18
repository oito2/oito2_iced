🌐 [Português](../../pt-br/troubleshooting/common-issues.md) | **English** | 🏠 [Index](../index.md)

---

# Common issues

Problems that come up when building or running apps with oito2_iced, indexed by symptom.

## Building

### `pkg-config` can't find `glib-2.0`, or the build fails in `glib-sys` / `gio-sys`

`oito2_iced_settings` reads the desktop settings through GIO on Linux, which needs the GLib development files. Install them:

```bash
sudo apt install libglib2.0-dev        # Debian, Ubuntu
sudo dnf install glib2-devel           # Fedora
```

### The build fails looking for `xkbcommon`

The windowing library needs the `libxkbcommon` development files on Linux:

```bash
sudo apt install libxkbcommon-dev      # Debian, Ubuntu
sudo dnf install libxkbcommon-devel    # Fedora
```

### `error: package requires rustc 1.97` or edition 2024 errors

The crates need Rust 1.97 or newer. Update with `rustup update`. Inside a clone of the repository, rustup uses the pinned 1.99.0 automatically.

### `no method named 'class' found` on an Iced widget

Choosing a style class with `.class(…)` is part of Iced's `advanced` feature. `oito2_iced` enables it on its Iced dependency, so the method appears when the app's `iced` is the same 0.14 version Cargo unifies with it; check that the app depends on `iced = "0.14"`, not another minor version.

### The machine freezes or runs out of memory while building the tests

Every test binary links all of Iced, and many linkers running in parallel need a lot of memory. Limit the jobs, for example `cargo test -j 6 …`. Contributors: never add a top-level `tests/<name>.rs`; use `tests/all/<area>.rs` (see [Testing and goldens](../architecture/testing-and-goldens.md)).

## Running

### The window is black, the app crashes at start-up, or GPU errors appear

The default renderer is wgpu (Vulkan, OpenGL, Metal or DirectX 12). On machines without a working GPU driver, in virtual machines or on live systems, use the software renderer:

```bash
ICED_BACKEND=tiny-skia cargo run -p oito2_iced_showcase
```

The app must have Iced's `tiny-skia` feature, which is in Iced's default features.

### Text is in another font, or bold and semibold look the same

The OITO2 widgets expect the embedded Inter faces. Register them and make Inter the default font:

```rust
font::INTER_FONTS
    .into_iter()
    .fold(app, |app, bytes| app.font(bytes))
    .default_font(oito2_iced::font::INTER)
```

Iced's text stack picks a face by exact weight; without the static faces, weights other than regular fall back to a system font. Don't replace them with a variable Inter for the same reason. The embedded family is named `OITO2 Inter`, so an Inter installed on the system doesn't interfere.

### Code blocks show a proportional font

Register the JetBrains Mono faces too: `.font(…)` for each item of `oito2_iced_syntax::font::JETBRAINS_MONO_FONTS`.

### Tab doesn't move the focus

Iced 0.14 doesn't bind Tab. Wrap the root of the view in `oito2_iced::focus::navigation(…)`. A control without its message (`on_press`, `on_toggle`, `on_change`…) is disabled and skipped by Tab.

### Controls have the wrong height after switching between pointer and touch

The widgets read the density while they lay out. Call `oito2_iced::density::set(…)` at the start of `view` with the density of the theme.

### The theme doesn't follow the desktop accent or dark style

- Subscribe to `oito2_iced_settings::subscription()` and rebuild the theme from the latest `SystemSettings`.
- The desktop must expose the setting: on Linux the readers use GSettings, the configuration files of KDE, Xfce and COSMIC, and the settings portal. When a store can't be read the values stay empty and the defaults apply.
- When the platform can't be watched, only the first reading arrives: the app follows changes after a restart.

### The window has no title bar, or two title bars

oito2_iced draws its own chrome. Open the window with `decorations: false` and put a `window::header_bar` or `window::title_bar` in the view; with `decorations: true` the system bar appears above the OITO2 one.

### The window can't be resized

An undecorated window has no system borders. Wrap the whole view in `oito2_iced::window::resize_handles(view, Message::Window)` and run the published actions with `window::perform_latest`.

## Files

### Dropping files on the window does nothing (Wayland)

The Wayland backend of winit 0.30 reports no file drops, so `drop_zone` only offers a click there (`drop_zone::can_drop_files()` is false). Use the file chooser, or run the app through XWayland by unsetting the Wayland variables:

```bash
env -u WAYLAND_DISPLAY -u WAYLAND_SOCKET cargo run -p oito2_iced_showcase
```

### Files dropped anywhere on the window go to one drop zone

The platform reports drops without a position, so every zone with `on_drop` receives the drops of the whole window. Give `on_drop` to one zone per window.

### The file chooser or "Save as" dialog doesn't open on Linux

`oito2_iced_files` opens the dialogs through the XDG desktop portal. Make sure `xdg-desktop-portal` and a backend for your desktop (for example `xdg-desktop-portal-gnome`, `-kde` or `-gtk`) are installed and running.

## Tests

### A golden test fails on a machine where everything else passes

- Run the tests with `ICED_TEST_BACKEND=tiny-skia`: the goldens are made with the software renderer.
- Compare `<name>.actual.png`, written next to the golden, with the golden itself. The tolerance is 0.5 % of the pixels.

### `missing golden … run with OITO2_UPDATE_GOLDENS=1`

A new golden test has no reference image yet. Run it once with `OITO2_UPDATE_GOLDENS=1`, look at the written image, and commit it if it is right.

## Accessibility

### Screen readers don't read the widgets

Iced 0.14 has no accessibility tree (no AccessKit integration upstream), so no assistive technology can read an Iced window. This is a known limit of the toolkit; if your app needs screen reader support, see [oito2_iced or oito2_uikit?](../concepts/why-oito2_iced.md#oito2_iced-or-oito2_uikit).
