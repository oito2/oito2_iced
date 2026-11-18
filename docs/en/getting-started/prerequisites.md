🌐 [Português](../../pt-br/getting-started/prerequisites.md) | **English** | 🏠 [Index](../index.md)

---

# Prerequisites

What a machine needs to build an app with oito2_iced, or the showcase. Linux is the primary platform; Windows and macOS are best effort: the crates build and pass the tests there in CI, with no promise of native integration.

## Rust

| Tool | Version | Why |
|---|---|---|
| [rustup](https://rustup.rs) | any | Installs the toolchain the repository pins |
| Rust | **1.99.0** in this repository; **1.97** or newer for apps | `rust-toolchain.toml` pins 1.99.0 with `rustfmt` and `clippy`; the crates declare `rust-version = "1.97"`, the minimum `material-colors` 0.5 needs |

Inside a clone of the repository, the first `cargo` command installs Rust 1.99.0 through rustup. An app that uses the crates needs Rust 1.97 or newer and the 2024 edition support that comes with it.

## Linux

Building needs the development files of `libxkbcommon` (keyboard handling of the windowing library) and GLib (GIO, which reads the desktop settings). These are the packages the CI installs on Ubuntu:

```bash
sudo apt install libxkbcommon-dev libglib2.0-dev
```

On Fedora the same files come from:

```bash
sudo dnf install libxkbcommon-devel glib2-devel
```

At run time an app needs:

- an X11 or Wayland session;
- a Vulkan or OpenGL driver for the default `wgpu` renderer, or the software renderer (`ICED_BACKEND=tiny-skia`) when the GPU path fails;
- for the file chooser and "Save as" dialogs of `oito2_iced_files`, a running [XDG desktop portal](https://flatpak.github.io/xdg-desktop-portal/) with a backend for the desktop, which most desktop environments install by default.

## Windows

Best effort. Rust needs the MSVC build tools that rustup asks for during its installation; no other system package is needed. The CI builds and tests the workspace on `windows-latest`.

## macOS

Best effort. Rust needs the Xcode Command Line Tools (`xcode-select --install`); no other system package is needed. The CI builds and tests the workspace on `macos-latest`.

## For contributors

The checks of the [contributing guide](../../../CONTRIBUTING.md#checks) also need `cargo-deny` and the 1.97 toolchain:

```bash
cargo install --locked cargo-deny
rustup toolchain install 1.97 --profile minimal
```

Regenerating the fonts needs Python 3 with [fontTools](https://github.com/fonttools/fonttools); see [Repository tools](../reference/repository-tools.md).
