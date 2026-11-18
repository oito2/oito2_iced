🌐 [Português](../../pt-br/guides/files.md) | **English** | 🏠 [Index](../index.md)

---

# Files

How to let users choose, drop, upload and save files with `oito2_iced_files`. Add the crate first:

```toml
[dependencies]
oito2_iced_files = { git = "https://github.com/oito2/oito2_iced" }
```

The widgets never open a dialog by themselves: dialogs are `iced::Task`s that the app returns from `update`.

## Choose files

`oito2_iced_files::pick(&types, multiple)` opens the platform's file chooser and yields the chosen `File`s (an empty list when canceled). On Linux it goes through the XDG desktop portal; on Windows and macOS through the native dialog.

```rust
use iced::Task;
use oito2_iced_files::file::{File, TypeGroup};

#[derive(Debug, Clone)]
enum Message {
    Choose,
    Chosen(Vec<File>),
}

fn update(message: Message) -> Task<Message> {
    match message {
        Message::Choose => {
            let pdf = TypeGroup::new().label("PDF").extensions(["pdf"]);
            oito2_iced_files::pick(&[pdf], true).map(Message::Chosen)
        }
        Message::Chosen(files) => {
            println!("{} files", files.len());
            Task::none()
        }
    }
}
```

`File` holds the name, size, optional MIME type and path, and whether it is a directory; `TypeGroup` a label, extensions and MIME types.

## Drop zone

`drop_zone::drop_zone(&l10n)` is a dashed box that publishes `on_choose` on a click, Enter or Space, and `on_drop` with the files dropped on the window.

```rust
use oito2_iced_files::drop_zone::drop_zone;

let zone = drop_zone(&l10n)
    .hint("PNG or JPEG, up to 5 MB")
    .on_choose(Message::Choose)
    .on_drop(Message::Files);
```

Things to know about drops:

- The platform reports dropped files without a position, so a zone can't know where the files land: every zone with `on_drop` receives the drops of the **whole window**. Give `on_drop` to one zone per window.
- Drops arrive on Windows, macOS and Linux under X11. **Under Wayland the windowing library (winit 0.30) reports no file drops**; `drop_zone::can_drop_files()` is then false and the zone only invites a click.
- Dropped paths are read when the drop ends: items that no longer exist are skipped and directories are marked as such.

## Upload with validation

`upload::Controller` holds the accepted files and checks every addition against the accepted types, a size limit and a largest number of files; directories are always refused. `upload::file_upload` shows the zone, the refusals with their reasons, and the accepted files with sizes, image previews and remove buttons.

```rust
use iced::Task;
use oito2_iced::l10n::Localizations;
use oito2_iced_files::file::TypeGroup;
use oito2_iced_files::upload::{self, Controller};

#[derive(Debug, Clone)]
enum Message {
    Upload(upload::Event),
}

struct App {
    images: Controller,
}

impl App {
    fn new() -> Self {
        let images = TypeGroup::new()
            .label("PNG, JPEG, WebP")
            .extensions(["png", "jpg", "jpeg", "webp"])
            .mime_types(["image/png", "image/jpeg", "image/webp"]);

        Self {
            images: Controller::new()
                .accept([images])
                .max_files(4)
                .max_file_size(5 * 1000 * 1000),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Upload(event) => self.images.update(event).map(Message::Upload),
        }
    }

    fn view(&self) -> oito2_iced::Element<'_, Message> {
        upload::file_upload(&self.images, &Localizations::en())
            .on_event(Message::Upload)
            .into()
    }
}
```

`Controller::update` applies each event and returns the file chooser task when the zone is activated. `accepted()` and `rejected()` give the current lists; `reason(&rejected, &l10n)` the localized message of a refusal.

## Save a file

`download::save(file_name, bytes)` asks where to save with the "Save as" dialog and writes the bytes; it yields `Ok(true)` once written, `Ok(false)` when canceled, or an `Error`. `download::download_trigger(&l10n)` is an outlined button with a download icon; mark it `busy(true)` while saving, which shows a spinner and ignores presses.

```rust
Message::Download => {
    self.busy = true;
    download::save("report.txt", self.report.clone().into_bytes()).map(Message::Saved)
}
Message::Saved(_result) => {
    self.busy = false;
    Task::none()
}
```

## Platform notes

| Platform | Chooser and "Save as" | Drops |
|---|---|---|
| Linux (X11) | XDG desktop portal | Yes |
| Linux (Wayland) | XDG desktop portal | No (winit 0.30) |
| Windows | Native dialog | Yes |
| macOS | Native dialog | Yes |

On Linux the dialogs need a running `xdg-desktop-portal` with a backend for the desktop; see [Common issues](../troubleshooting/common-issues.md).
