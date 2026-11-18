🌐 [Português](../../pt-br/getting-started/installation.md) | **English** | 🏠 [Index](../index.md)

---

# Installation

oito2_iced is a set of Rust crates, not an application: "installing" it means adding the crates to an app. The repository also holds a showcase app, which you build from source. The project produces no binaries, installers or packages yet, and the crates are not on crates.io: every crate is at version `0.0.0` with `publish = false`.

## Platform support

| OS | Status | Notes |
|---|---|---|
| Linux | Supported (primary) | X11 and Wayland; every desktop environment. Built and tested in CI on Ubuntu. |
| Windows | Best effort | Built and tested in CI on `windows-latest`; window chrome in the Windows style; no promise of native integration. |
| macOS | Best effort | Built and tested in CI on `macos-latest`; window chrome in the macOS style, without native traffic lights; no promise of native integration. |

Install the [prerequisites](./prerequisites.md) of your system first.

## Add the crates to an app

Add the crates you need from the Git repository, next to Iced 0.14:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }

# Optional crates, as you need them:
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_themes = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_syntax = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_extras = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_files = { git = "https://github.com/oito2/oito2_iced" }
```

Cargo finds each crate inside the repository and records the commit in `Cargo.lock`. To pin a commit explicitly, add `rev = "<commit>"` to each entry.

| Crate | Add it for |
|---|---|
| `oito2_iced` | The theme, tokens, fonts, icons, focus, overlay, every core widget and the window chrome |
| `oito2_iced_settings` | Following the desktop accent, light or dark preference, high contrast and window buttons |
| `oito2_iced_themes` | The 13 color presets |
| `oito2_iced_syntax` | Code blocks and a highlighter for the Iced text editor |
| `oito2_iced_extras` | QR code, signature pad, image cropper and navigation menu |
| `oito2_iced_files` | Drop zone, file upload, file chooser and download |

### Renderers and features

`oito2_iced` depends on Iced without its default features, so the app chooses the renderers. `iced = "0.14"` with its default features enables both the `wgpu` renderer and the `tiny-skia` software fallback. The core crate has these features of its own:

| Feature | Default | Enables |
|---|---|---|
| `svg` | Yes | The SVG style catalog and the Lucide icon glyphs, used by many widgets |
| `image` | No | Photos in avatars |
| `qr_code` | No | The style catalog of the Iced QR code widget |
| `markdown` | No | The style catalog of the Iced Markdown widget |

See [Crates and features](../reference/crates-and-features.md) for every crate.

## Linux

1. Install the build packages from the [prerequisites](./prerequisites.md#linux) (`libxkbcommon-dev` and `libglib2.0-dev` on Debian and Ubuntu).
2. Add the crates as above and build the app with `cargo build`.

The same steps build the showcase:

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

For an optimized build, run `cargo build --release -p oito2_iced_showcase`; the binary is `target/release/oito2_iced_showcase`. Set `ICED_BACKEND=tiny-skia` to force the software renderer.

## Windows

Best effort: the steps are the same as on Linux, without system packages. In PowerShell:

```powershell
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

The release binary is `target\release\oito2_iced_showcase.exe`.

## macOS

Best effort: the steps are the same as on Linux, without system packages:

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

## What installing leaves on the system

Nothing outside the places Cargo always uses: the clone, its `target/` build directory, and the downloaded dependencies in Cargo's cache (`~/.cargo/registry` and `~/.cargo/git`). The showcase installs no desktop entry, icon or service, and writes no configuration or data files.

To remove the showcase, delete the clone. To remove the crates from an app, delete their lines from its `Cargo.toml`; deleting the `oito2_iced-*` directories under `~/.cargo/git/checkouts` and `~/.cargo/git/db` reclaims the cached Git checkout.
