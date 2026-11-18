// Copyright (C) 2026  OITO2
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Saving bytes as a file through the "Save as" dialog, and the button that
//! starts it.
//!
//! [`save`] asks where to save with the dialog of the platform and writes
//! the bytes there, as a [`Task`] the application runs. A
//! [`download_trigger`] is an outlined button with a download icon that
//! publishes its press message; the application builds the bytes, runs
//! [`save`] and marks the trigger [`busy`](DownloadTrigger::busy)
//! meanwhile, which shows a spinner and ignores presses. Where
//! [`is_supported`] is false the trigger is disabled and its tooltip says
//! why.
//!
//! ```no_run
//! use iced::Task;
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_files::download;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Download,
//!     Saved(Result<bool, download::Error>),
//! }
//!
//! struct App {
//!     busy: bool,
//!     text: String,
//! }
//!
//! impl App {
//!     fn update(&mut self, message: Message) -> Task<Message> {
//!         match message {
//!             Message::Download => {
//!                 self.busy = true;
//!                 download::save("hello.txt", self.text.clone().into_bytes())
//!                     .map(Message::Saved)
//!             }
//!             Message::Saved(_result) => {
//!                 self.busy = false;
//!                 Task::none()
//!             }
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         download::download_trigger(&Localizations::en())
//!             .busy(self.busy)
//!             .on_press(Message::Download)
//!             .into()
//!     }
//! }
//! ```

use std::borrow::Cow;
use std::fmt;
use std::path::Path;

use iced::Alignment;
use iced::Task;
use iced::advanced::widget::Id;
use iced::widget::row;
use iced::widget::text::IntoFragment;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::{button, progress, styled_text};
use oito2_iced::{Element, density};

/// Why saving a file failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// This platform has no "Save as" dialog.
    Unsupported,
    /// Writing the file failed, with the message of the error.
    Write(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unsupported => f.write_str("saving files is not supported on this platform"),
            Self::Write(message) => write!(f, "the file could not be written: {message}"),
        }
    }
}

impl std::error::Error for Error {}

/// Whether this platform has a "Save as" dialog: Linux and the other free
/// Unix systems, Windows and macOS.
#[must_use]
pub const fn is_supported() -> bool {
    cfg!(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "dragonfly",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "windows",
        target_os = "macos"
    ))
}

/// Asks where to save `bytes` with the "Save as" dialog, suggesting
/// `file_name`, and writes them there.
///
/// Yields `Ok(false)` when the dialog is canceled and `Ok(true)` once the
/// file is written; [`Error::Unsupported`] when not [`is_supported`] and
/// [`Error::Write`] when writing fails.
pub fn save(file_name: impl Into<String>, bytes: impl Into<Vec<u8>>) -> Task<Result<bool, Error>> {
    if !is_supported() {
        return Task::done(Err(Error::Unsupported));
    }

    let dialog = rfd::AsyncFileDialog::new().set_file_name(file_name);
    let bytes = bytes.into();

    Task::future(async move {
        let location = dialog.save_file().await;

        write_to(location.as_ref().map(rfd::FileHandle::path), &bytes)
    })
}

/// Writes `bytes` to `location`; `Ok(false)` without a location.
pub(crate) fn write_to(location: Option<&Path>, bytes: &[u8]) -> Result<bool, Error> {
    let Some(location) = location else {
        return Ok(false);
    };

    std::fs::write(location, bytes)
        .map(|()| true)
        .map_err(|error| Error::Write(error.to_string()))
}

/// A button that starts a download. See [`download_trigger`].
pub struct DownloadTrigger<'a, Message> {
    label: Cow<'a, str>,
    unavailable: Cow<'a, str>,
    on_press: Option<Message>,
    busy: bool,
    supported: bool,
    id: Option<Id>,
}

/// An outlined button with a download icon and the localized "Download"
/// label, disabled until it has a press message.
pub fn download_trigger<'a, Message>(
    localizations: &Localizations,
) -> DownloadTrigger<'a, Message> {
    DownloadTrigger {
        label: Cow::Borrowed(localizations.download),
        unavailable: Cow::Borrowed(localizations.download_unavailable),
        on_press: None,
        busy: false,
        supported: is_supported(),
        id: None,
    }
}

impl<'a, Message> DownloadTrigger<'a, Message> {
    /// Replaces the localized "Download" label.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = label.into_fragment();
        self
    }

    /// Sets the message published on a click, Enter or Space. Without it
    /// the button is disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or disables the button with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets whether the file is being built or saved: the icon turns into a
    /// spinner and presses are ignored.
    #[must_use]
    pub fn busy(mut self, busy: bool) -> Self {
        self.busy = busy;
        self
    }

    /// Sets whether the platform can save files, [`is_supported`] by
    /// default; when false the button is disabled and its tooltip says
    /// that downloads are not available.
    #[must_use]
    pub fn supported(mut self, supported: bool) -> Self {
        self.supported = supported;
        self
    }

    /// Sets the widget [`Id`], used to focus the button programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<DownloadTrigger<'a, Message>> for Element<'a, Message> {
    fn from(trigger: DownloadTrigger<'a, Message>) -> Self {
        let size = density::icon_size();
        let leading: Element<'a, Message> = if trigger.busy {
            progress::circular_indeterminate().size(size).into()
        } else {
            icon(lucide::DOWNLOAD).size(size).into()
        };
        let content = row![leading, styled_text(trigger.label, typography::LABEL_LARGE)]
            .spacing(spacing::SM)
            .align_y(Alignment::Center);
        let on_press = trigger
            .on_press
            .filter(|_| trigger.supported && !trigger.busy);

        let mut button = button::button(content)
            .class(button::Class::Outlined)
            .on_press_maybe(on_press);

        if !trigger.supported {
            button = button.tooltip(trigger.unavailable);
        }

        if let Some(id) = trigger.id {
            button = button.id(id);
        }

        button.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writing_without_a_location_reports_a_cancel() {
        assert_eq!(write_to(None, b"hello"), Ok(false));
    }

    #[test]
    fn writing_saves_the_bytes_and_reports_failures() {
        let directory =
            std::env::temp_dir().join(format!("oito2_download_unit_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(&directory).expect("create the directory");

        let path = directory.join("report.txt");
        assert_eq!(write_to(Some(&path), b"hello"), Ok(true));
        assert_eq!(std::fs::read(&path).expect("read the file"), b"hello");

        let failure = write_to(Some(&directory.join("missing/report.txt")), b"x");
        assert!(matches!(failure, Err(Error::Write(_))));

        let _ = std::fs::remove_dir_all(&directory);
    }

    #[test]
    fn desktop_platforms_are_supported() {
        assert!(is_supported());
    }
}
