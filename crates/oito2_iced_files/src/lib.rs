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

//! File widgets for the OITO2 Iced design system: drop zone, file upload
//! and download trigger.
//!
//! - [`drop_zone`](drop_zone::drop_zone) is a dashed box that asks for the
//!   file chooser when activated and receives files dropped on the window.
//! - [`upload`] holds the [`Controller`](upload::Controller) that checks
//!   files against accepted types, a size limit and a largest number of
//!   files, and the [`file_upload`](upload::file_upload) that shows the
//!   zone, the refusals and the accepted files.
//! - [`download`] saves bytes through the "Save as" dialog and holds the
//!   [`download_trigger`](download::download_trigger) button.
//! - [`pick`] opens the file chooser; [`file`](mod@file) describes the files and the
//!   accepted types.
//!
//! Dialogs are returned as [`iced::Task`]s that the application runs; the
//! widgets never open them by themselves.
//!
//! ```no_run
//! use iced::Task;
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_files::upload::{self, Controller};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Upload(upload::Event),
//! }
//!
//! fn update(controller: &mut Controller, message: Message) -> Task<Message> {
//!     match message {
//!         Message::Upload(event) => controller.update(event).map(Message::Upload),
//!     }
//! }
//!
//! fn view(controller: &Controller) -> oito2_iced::Element<'_, Message> {
//!     upload::file_upload(controller, &Localizations::en())
//!         .on_event(Message::Upload)
//!         .into()
//! }
//! ```

#![deny(missing_docs)]

mod dialog;
pub mod download;
pub mod drop_zone;
pub mod file;
pub mod upload;

pub use dialog::pick;
