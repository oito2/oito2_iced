🌐 [Português](../../../pt-br/guides/recipes/tray.md) | **English** | 🏠 [Index](../../index.md)

---

# Tray Icon

A **resident application** starts without a window and lives in the system tray; its tray menu opens and closes an OITO2-styled window and quits. Closing the window keeps the application running.

Source: [`examples/desktop/src/bin/tray.rs`](../../../../examples/desktop/src/bin/tray.rs).

```bash
cargo run -p oito2_iced_desktop_example --bin tray
```

## How it works

| Piece | What it does |
|---|---|
| `iced::daemon` | Runs the application with no window at start. Unlike `iced::application`, it does not exit when the last window closes. |
| `ksni` | Publishes the tray icon as a [StatusNotifierItem](https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/) over D-Bus. Its service runs on its own thread. |
| A channel and a `Subscription` | Carry what the tray menu asks for to the daemon as messages. |
| `window::open` / `window::close` | Open and close the window on request. |

### 1. Start with no window

`iced::daemon` takes the same boot, update and view functions as `iced::application`, but `view`, `title` and `theme` also receive the `window::Id`, because a daemon can own several windows:

```rust
let daemon = iced::daemon(Resident::default, Resident::update, Resident::view)
    .title("OITO2 Iced tray")
    .theme(Resident::theme)
    .subscription(Resident::subscription);
```

The OITO2 fonts are registered with `.font(...)` for each entry of `oito2_iced::font::INTER_FONTS`, and `.default_font(oito2_iced::font::INTER)`, as in any OITO2 application.

### 2. From the tray thread to the daemon

`ksni` calls the tray (`impl ksni::Tray`) from the thread of its D-Bus service. The tray holds the sending half of an `iced::futures::channel::mpsc` channel, and every menu entry sends an event:

```rust
StandardItem {
    label: "Open window".into(),
    icon_name: "window-new".into(),
    activate: Box::new(|tray: &mut Self| tray.send(TrayEvent::Open)),
    ..Default::default()
}
.into(),
```

`iced::stream::channel` builds the stream: it hands its sender to the tray, starts the tray service and keeps the service handle alive as long as the stream runs. `Subscription::run` turns the stream into messages:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
        Subscription::run(tray_events).map(Message::Tray),
        window::close_events().map(Message::Closed),
    ])
}
```

`window::close_events()` reports a window closed by the window manager, so the state forgets it.

### 3. Open, close and quit

- **Open**: `window::open(settings)` returns the new `window::Id` and a task; the daemon stores the id. When the window is already open, `window::gain_focus(id)` raises it instead.
- **Close**: `window::close(id)`.
- **Quit**: `iced::exit()`.

If the tray service cannot start (no StatusNotifierItem host, no session bus), the stream sends `TrayEvent::Unavailable` and the daemon opens the window with the reason and a **Quit** button, so the application never runs without a way to end it.

## Dependency

```toml
[target.'cfg(target_os = "linux")'.dependencies]
ksni = { version = "0.3.7", default-features = false, features = ["async-io"] }
```

- `ksni` talks D-Bus through the pure-Rust `zbus`; no `libdbus` is needed at build or run time.
- The `async-io` feature replaces the default `tokio` runtime: `ksni` runs its service on its own executor thread, independent of the Iced executor.
- `ksni` is released under the **Unlicense**; the workspace `deny.toml` allows it for this crate only.

## Platforms and desktops

- **Linux only.** On Windows and macOS the binary prints a message and exits; a tray icon there needs another crate (for example `tray-icon`), which this recipe does not cover.
- KDE Plasma, Xfce, LXQt, Cinnamon and Budgie show StatusNotifierItems out of the box. **GNOME** needs the *AppIndicator and KStatusNotifierItem Support* extension.
- Inside a Flatpak sandbox, start the tray with `.disable_dbus_name(true)` (see the `ksni` documentation) and grant `--talk-name=org.kde.StatusNotifierWatcher` — see [Packaging](packaging.md).
