🌐 [Português](../../../pt-br/guides/recipes/packaging.md) | **English** | 🏠 [Index](../../index.md)

---

# Packaging an Application

How an application built on `oito2_iced` is packaged for Linux as a `.deb`, an `.rpm` and a Flatpak, with its desktop entry, AppStream metainfo and icon.

> **These are templates.** `oito2_iced` is a library: neither the project nor its CI builds any package, and nothing on this page is tested by CI. Replace every placeholder — the crate `my-app`, the binary `my-app`, the application id `io.github.example.MyApp`, names, URLs, versions — and check each tool's current documentation before relying on a key or a flag.

Tool versions checked when this guide was written: `cargo-deb` 3.8, `cargo-generate-rpm` 0.21, Flatpak runtime `org.freedesktop.Platform` 25.08.

## Files the package installs

| File | Installed at | Purpose |
|---|---|---|
| Binary | `/usr/bin/my-app` | The application. |
| Desktop entry | `/usr/share/applications/io.github.example.MyApp.desktop` | Menu entry and launcher. |
| AppStream metainfo | `/usr/share/metainfo/io.github.example.MyApp.metainfo.xml` | Description for software centers. |
| Icon | `/usr/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg` | Menu, launcher and window icon. |

Keep the source files in a folder of your crate, for example `packaging/`. The **application id** names all three metadata files and must match the Wayland application id of the window, which an Iced application sets in its window settings on Linux:

```rust
#[cfg(target_os = "linux")]
fn platform_specific() -> iced::window::settings::PlatformSpecific {
    iced::window::settings::PlatformSpecific {
        application_id: "io.github.example.MyApp".into(),
        ..Default::default()
    }
}
```

The OITO2 showcase does the same in [`apps/showcase/src/main.rs`](../../../../apps/showcase/src/main.rs). The OITO2 fonts are embedded in the binary by `oito2_iced`, so the package needs no font dependency.

## Desktop entry

`packaging/io.github.example.MyApp.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=My App
Comment=One-line description of My App
Exec=my-app
Icon=io.github.example.MyApp
Terminal=false
Categories=Utility;
StartupWMClass=io.github.example.MyApp
```

Check it with `desktop-file-validate packaging/io.github.example.MyApp.desktop` (package `desktop-file-utils`). The [autostart recipe](autostart.md) writes a smaller entry of the same format.

## AppStream metainfo

`packaging/io.github.example.MyApp.metainfo.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>io.github.example.MyApp</id>
  <name>My App</name>
  <summary>One-line description of My App</summary>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>GPL-3.0-or-later</project_license>
  <developer id="io.github.example">
    <name>Example Developer</name>
  </developer>
  <description>
    <p>A paragraph that describes what My App does.</p>
  </description>
  <launchable type="desktop-id">io.github.example.MyApp.desktop</launchable>
  <url type="homepage">https://github.com/example/my-app</url>
  <content_rating type="oars-1.1"/>
  <releases>
    <release version="1.0.0" date="2026-01-01"/>
  </releases>
</component>
```

Check it with `appstreamcli validate packaging/io.github.example.MyApp.metainfo.xml` (package `appstream`). Flathub also requires screenshots and branding colors; see its submission guidelines.

## Icon

Ship a scalable SVG at `share/icons/hicolor/scalable/apps/<app id>.svg`. Add PNG sizes (for example `256x256/apps/<app id>.png`) for desktops that do not render SVG icons.

## `.deb` with cargo-deb

Install the tool with `cargo install cargo-deb`. Add to the `Cargo.toml` of the application crate:

```toml
[package.metadata.deb]
maintainer = "Example Developer <dev@example.com>"
copyright = "2026, Example Developer"
license-file = ["LICENSE", "0"]
extended-description = "A longer description of My App."
section = "utils"
priority = "optional"
depends = "$auto"
assets = [
    # cargo-deb replaces target/release with the real target directory.
    ["target/release/my-app", "usr/bin/", "755"],
    ["packaging/io.github.example.MyApp.desktop", "usr/share/applications/", "644"],
    ["packaging/io.github.example.MyApp.metainfo.xml", "usr/share/metainfo/", "644"],
    ["packaging/io.github.example.MyApp.svg", "usr/share/icons/hicolor/scalable/apps/", "644"],
]
```

```bash
cargo deb -p my-app
```

The package is written to `target/debian/my-app_<version>-1_<arch>.deb`. Keep the `target/release/` prefix in `assets` as written: `cargo-deb` resolves it itself, workspaces included.

`$auto` computes dependencies from the libraries the binary links. Libraries that Iced's windowing loads at run time (such as `libxkbcommon` and the Vulkan/OpenGL drivers) are not linked, so they are not detected: test the package on a clean installation of each distribution you target and add what is missing to `depends`.

## `.rpm` with cargo-generate-rpm

Install the tool with `cargo install cargo-generate-rpm`. It does not build the binary, so build and strip it first. Add to the `Cargo.toml` of the application crate:

```toml
[package.metadata.generate-rpm]
assets = [
    { source = "target/release/my-app", dest = "/usr/bin/my-app", mode = "755" },
    { source = "packaging/io.github.example.MyApp.desktop", dest = "/usr/share/applications/io.github.example.MyApp.desktop", mode = "644" },
    { source = "packaging/io.github.example.MyApp.metainfo.xml", dest = "/usr/share/metainfo/io.github.example.MyApp.metainfo.xml", mode = "644" },
    { source = "packaging/io.github.example.MyApp.svg", dest = "/usr/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg", mode = "644" },
]
```

```bash
cargo build --release -p my-app
strip -s target/release/my-app
cargo generate-rpm -p my-app
```

The package is written to `target/generate-rpm/`. Name, version, license, summary and URL come from `[package]`. Automatic dependencies have the same limit as with `cargo-deb`; add missing ones under `[package.metadata.generate-rpm.requires]`.

## Flatpak

Flatpak builds run offline, so the crates are downloaded beforehand: generate `cargo-sources.json` from `Cargo.lock` with `flatpak-cargo-generator.py` from [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools):

```bash
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```

`io.github.example.MyApp.yml`, a sketch on the freedesktop.org runtime with the Rust SDK extension (an application that needs GNOME libraries uses `org.gnome.Platform` instead):

```yaml
id: io.github.example.MyApp
runtime: org.freedesktop.Platform
runtime-version: '25.08'
sdk: org.freedesktop.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
command: my-app
finish-args:
  - --socket=wayland
  - --socket=fallback-x11
  - --share=ipc
  - --device=dri
  # Only with the notifications recipe:
  - --talk-name=org.freedesktop.Notifications
  # Only with the tray recipe:
  - --talk-name=org.kde.StatusNotifierWatcher
build-options:
  append-path: /usr/lib/sdk/rust-stable/bin
  env:
    CARGO_HOME: /run/build/my-app/cargo
modules:
  - name: my-app
    buildsystem: simple
    build-commands:
      - cargo --offline fetch --manifest-path Cargo.toml --verbose
      - cargo --offline build --release -p my-app
      - install -Dm755 target/release/my-app /app/bin/my-app
      - install -Dm644 packaging/io.github.example.MyApp.desktop /app/share/applications/io.github.example.MyApp.desktop
      - install -Dm644 packaging/io.github.example.MyApp.metainfo.xml /app/share/metainfo/io.github.example.MyApp.metainfo.xml
      - install -Dm644 packaging/io.github.example.MyApp.svg /app/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg
    sources:
      - type: dir
        path: .
      - cargo-sources.json
```

```bash
flatpak-builder --user --install --force-clean build-dir io.github.example.MyApp.yml
flatpak run io.github.example.MyApp
```

Use the current `runtime-version` (check `flatpak remote-info flathub org.freedesktop.Platform//<branch>` or Flathub). Inside the sandbox:

- the tray icon needs `.disable_dbus_name(true)` when spawning the `ksni` tray ([tray recipe](tray.md));
- autostart goes through the Background portal, not `~/.config/autostart` ([autostart recipe](autostart.md)).

## Uninstallation

The package manager removes what it installed (`sudo apt remove my-app`, `sudo dnf remove my-app`, `flatpak uninstall io.github.example.MyApp`). Files the application writes itself stay behind — its configuration and data directories and, with the autostart recipe, `~/.config/autostart/<app id>.desktop`; document them in your uninstallation instructions.
