🌐 [Português](../../../pt-br/guides/recipes/autostart.md) | **English** | 🏠 [Index](../../index.md)

---

# Start at Login

A switch in an OITO2-styled window writes or removes an **XDG autostart entry**: a desktop entry file in the autostart directory, which freedesktop.org sessions (GNOME, KDE Plasma, Xfce, Cinnamon, MATE, LXQt) run at login.

Sources:

- [`examples/desktop/src/autostart.rs`](../../../../examples/desktop/src/autostart.rs) — the logic, standard library only, with unit tests.
- [`examples/desktop/src/bin/autostart.rs`](../../../../examples/desktop/src/bin/autostart.rs) — the window.

```bash
cargo run -p oito2_iced_desktop_example --bin autostart
```

## The autostart directory

Following the XDG Base Directory specification, the directory is:

| Condition | Directory |
|---|---|
| `XDG_CONFIG_HOME` is set to an absolute path | `$XDG_CONFIG_HOME/autostart` |
| otherwise (unset, empty or relative) | `$HOME/.config/autostart` |

`autostart::directory(xdg_config_home, home)` takes both values as arguments, so it is tested without touching the environment; `autostart::directory_from_env()` reads them from the process.

## The entry

`autostart::Entry` holds the file id, the name and the command. `Entry::contents()` writes:

```ini
[Desktop Entry]
Type=Application
Name=OITO2 Iced autostart example
Exec=/path/to/autostart
Terminal=false
```

- The `Exec` arguments follow the Desktop Entry quoting rules: `%` is doubled, and an argument with a space or another reserved character is wrapped in double quotes with `"`, `` ` ``, `$` and `\` escaped.
- The file name is `<id>.desktop`; use your application id in reverse DNS notation, the same one as in its `.desktop` file and AppStream metainfo.
- The example points `Exec` at the running executable (`std::env::current_exe()`). A packaged application should use its installed command instead, for example `Exec=my-app --minimized`.

## Turning it on and off

| Call | Effect |
|---|---|
| `entry.enable(&dir)` | Creates the directory when missing and writes the file. |
| `entry.disable(&dir)` | Removes the file; a missing file is not an error. |
| `entry.is_enabled(&dir)` | Whether the file exists. |

Every function takes the directory, so the unit tests run on a temporary directory and never touch the real home:

```bash
cargo test -p oito2_iced_desktop_example
```

## Platforms and sandboxes

- **Linux only.** On Windows and macOS the switch is disabled and the window says why; those systems use other mechanisms (the `Run` registry key, login items) that this recipe does not cover.
- **Flatpak**: an app in the sandbox cannot write to `~/.config/autostart`; it asks the *Background* portal (`org.freedesktop.portal.Background`, `RequestBackground` with `autostart`) instead.
- Removing the application should remove the entry too: list `~/.config/autostart/<id>.desktop` in your uninstall instructions.
