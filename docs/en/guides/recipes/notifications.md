🌐 [Português](../../../pt-br/guides/recipes/notifications.md) | **English** | 🏠 [Index](../../index.md)

---

# Desktop Notifications

A button in an OITO2-styled window sends a desktop notification. The D-Bus call runs as an `iced::Task`, so the window keeps drawing and reacting while the notification server answers.

Source: [`examples/desktop/src/bin/notify.rs`](../../../../examples/desktop/src/bin/notify.rs).

```bash
cargo run -p oito2_iced_desktop_example --bin notify
```

## How it works

The button press returns a task that runs the asynchronous send and turns its result into a message:

```rust
Message::Send => {
    self.sending = true;
    Task::perform(send(), Message::Sent)
}
```

`send` builds the notification with `notify-rust` and awaits `show_async`, which talks to the `org.freedesktop.Notifications` service over D-Bus:

```rust
let mut notification = notify_rust::Notification::new();
notification
    .appname("OITO2 Iced")
    .summary("Hello from OITO2 Iced")
    .body("This notification was sent from an iced::Task.")
    .icon("dialog-information");

notification
    .show_async()
    .await
    .map(|_handle| ())
    .map_err(|error| format!("The notification was not sent: {error}"))
```

- The error becomes a `String` because a message must be `Clone`; the window shows it in the error text style.
- The button is disabled (`on_press_maybe(None)`) while a send is in flight.
- Never call the blocking `show()` from `update`: it would freeze the window until the server replies.

## Dependency

```toml
[target.'cfg(target_os = "linux")'.dependencies]
notify-rust = { version = "4.19", default-features = false, features = ["z"] }
```

- The `z` feature selects the pure-Rust `zbus` backend (the default; the alternative `d` feature links `libdbus`). With default features turned off, nothing else is pulled in — no image decoding, no `chrono`.
- `notify-rust` is dual-licensed MIT or Apache-2.0.

## Platforms

- **Linux** (and other freedesktop.org desktops): any notification server — GNOME Shell, KDE Plasma, `mako`, `dunst`, `xfce4-notifyd`.
- **Windows and macOS**: `notify-rust` supports them with other backends, but this example declares the crate for Linux only, so the Windows and macOS builds need no platform SDK; there the button reports that the recipe is Linux-only. To support them, move `notify-rust` to the general `[dependencies]` and drop the `#[cfg]` around `send`.
- **Flatpak**: grant `--talk-name=org.freedesktop.Notifications` — see [Packaging](packaging.md).
