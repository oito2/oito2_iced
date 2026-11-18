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

//! The files page: file uploads with a drop zone, a file chooser and a
//! download trigger.

use iced::widget::{column, text};
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced_files::download;
use oito2_iced_files::drop_zone::drop_zone;
use oito2_iced_files::file::{File, TypeGroup};
use oito2_iced_files::upload::{self, Controller};

use super::{Gallery, Message, card, page};

/// The text the download trigger saves.
const GREETING: &str = "Hello from the OITO2 Iced showcase.\n";

/// The state of the files page.
#[derive(Debug)]
pub struct Files {
    pub(super) images: Controller,
    pub(super) document: Controller,
    pub(super) chosen: Vec<File>,
    pub(super) saving: bool,
    pub(super) saved: Option<Result<bool, download::Error>>,
}

impl Default for Files {
    fn default() -> Self {
        Self {
            images: Controller::new()
                .accept([TypeGroup::new()
                    .label("PNG, JPEG, WebP")
                    .extensions(["png", "jpg", "jpeg", "webp"])
                    .mime_types(["image/png", "image/jpeg", "image/webp"])])
                .max_files(4)
                .max_file_size(5 * 1000 * 1000),
            document: Controller::new()
                .accept([TypeGroup::new().label("PDF").extensions(["pdf"])])
                .max_files(1),
            chosen: Vec::new(),
            saving: false,
            saved: None,
        }
    }
}

impl Files {
    /// Applies a change, returning the task it starts.
    pub(super) fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Images(event) => return self.images.update(event).map(Message::Images),
            Message::Document(event) => {
                return self.document.update(event).map(Message::Document);
            }
            Message::ChooseFiles => {
                return oito2_iced_files::pick(&[], true).map(Message::FilesChosen);
            }
            Message::FilesChosen(files) => self.chosen = files,
            Message::Download => {
                self.saving = true;
                return download::save("greeting.txt", GREETING).map(Message::Saved);
            }
            Message::Saved(result) => {
                self.saving = false;
                self.saved = Some(result);
            }
            _ => {}
        }

        Task::none()
    }
}

/// The files page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let files = &gallery.files;

    // Iced reports dropped files for the whole window, so only the image
    // upload receives drops.
    let images = card(
        "File upload",
        upload::file_upload(&files.images, l10n).on_event(Message::Images),
    );
    let document = card(
        "Single file",
        upload::file_upload(&files.document, l10n)
            .show_previews(false)
            .accepts_drops(false)
            .zone_min_height(120.0)
            .on_event(Message::Document),
    );

    let chosen = if files.chosen.is_empty() {
        "No files chosen".to_owned()
    } else {
        files
            .chosen
            .iter()
            .map(|file| file.name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };
    let chooser = card(
        "Drop zone",
        column![
            drop_zone(l10n)
                .min_height(120.0)
                .accepts_drops(false)
                .on_choose(Message::ChooseFiles),
            text(chosen).class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    );

    let saved = match &files.saved {
        None => "Nothing saved yet".to_owned(),
        Some(Ok(true)) => "Saved.".to_owned(),
        Some(Ok(false)) => "Canceled.".to_owned(),
        Some(Err(error)) => error.to_string(),
    };
    let download = card(
        "Download trigger",
        column![
            download::download_trigger(l10n)
                .busy(files.saving)
                .on_press(Message::Download),
            download::download_trigger::<Message>(l10n).label("Export CSV"),
            text(saved).class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    );

    page(
        "Files",
        "Uploads with drag and drop, a file chooser and downloads.",
        [images, document, chooser, download],
    )
}
