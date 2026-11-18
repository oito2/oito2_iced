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

//! The FileUpload page: uploads with type, size and count checks, a single
//! file, a disabled upload and the bare drop zone.

use iced::widget::{Column, column, container, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::format::{ByteUnits, Format};
use oito2_iced::tokens::spacing;
use oito2_iced_files::drop_zone::{self, drop_zone};
use oito2_iced_files::file::{File, TypeGroup};
use oito2_iced_files::upload::{self, Controller, file_upload};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("file_upload.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    images: Controller,
    document: Controller,
    locked: Controller,
    received: Vec<File>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            // #region images-data
            images: Controller::new()
                .accept([TypeGroup::new()
                    .label("PNG, JPEG, WebP")
                    .extensions(["png", "jpg", "jpeg", "webp"])
                    .mime_types(["image/png", "image/jpeg", "image/webp"])])
                .max_files(4)
                .max_file_size(5 * 1000 * 1000),
            // #endregion
            // #region document-data
            document: Controller::new().max_files(1),
            // #endregion
            locked: Controller::new(),
            received: Vec::new(),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// An event of the image upload.
    Images(upload::Event),
    /// An event of the single file upload.
    Document(upload::Event),
    /// An event of the disabled upload.
    Locked(upload::Event),
    /// The bare drop zone asks for the file chooser.
    Choose,
    /// Files were chosen in the bare drop zone.
    Received(Vec<File>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Images(event) => state.images.update(event).map(Message::Images),
        // #endregion
        Message::Document(event) => state.document.update(event).map(Message::Document),
        // #region zone-update
        Message::Choose => oito2_iced_files::pick(&[], true).map(Message::Received),
        Message::Received(files) => {
            state.received = files;
            Task::none()
        }
        // #endregion
        Message::Locked(event) => state.locked.update(event).map(Message::Locked),
    }
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let width = Length::Fixed(420.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["images-data", "preview", "update"],
        container(
            // #region preview
            file_upload(&state.images, l10n).on_event(Message::Images),
            // #endregion
        )
        .width(width),
    );

    let document = doc::example(
        ctx,
        SOURCE,
        &["document-data", "document"],
        container(
            // #region document
            file_upload(&state.document, l10n)
                .show_previews(false)
                .accepts_drops(false)
                .zone_min_height(120.0)
                .on_event(Message::Document),
            // #endregion
        )
        .width(width),
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        container(
            // #region disabled
            file_upload(&state.locked, l10n)
                .enabled(false)
                .accepts_drops(false)
                .zone_min_height(120.0)
                .on_event(Message::Locked),
            // #endregion
        )
        .width(width),
    );

    let format = Format::new(l10n.locale_name);
    let received = Column::with_children(state.received.iter().map(|file| {
        let size = format.bytes(
            i64::try_from(file.size).unwrap_or(i64::MAX),
            ByteUnits::Si,
            1,
        );
        text(format!("{} — {size}", file.name)).into()
    }));

    let zone = doc::example(
        ctx,
        SOURCE,
        &["zone", "zone-update"],
        column![
            // #region zone
            drop_zone(l10n)
                .min_height(120.0)
                .accepts_drops(false)
                .hint("Click to choose files")
                .on_choose(Message::Choose),
            // #endregion
            received,
        ]
        .spacing(spacing::SM)
        .width(width),
    );

    let drops = if drop_zone::can_drop_files() {
        "Files dragged from the desktop reach this window."
    } else {
        "Files dragged from the desktop don't reach this window here."
    };

    doc::page(
        "FileUpload",
        "Adds files from the file chooser or a drop, checks their type, size and count, and \
         lists them with previews and remove buttons.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "An upload `Controller` holds the files and their limits: the `accept`ed \
                 `TypeGroup`s, `max_files` and `max_file_size` in bytes, each unlimited when \
                 left out. `file_upload` shows it and publishes `upload::Event`s through \
                 `on_event`; `Controller::update` applies them and returns the task of the file \
                 chooser when the zone is activated. Read the accepted files from `accepted`.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_files::file::TypeGroup;\n\
                 use oito2_iced_files::upload::{self, Controller, file_upload};",
            ),
            doc::prose(
                "`add` keeps the files that pass the checks and lists the others, with the \
                 reason, in `rejected`, which holds the refusals of the last addition only. The \
                 upload shows those refusals above the list, and disables its zone once the \
                 controller holds `max_files` files. The zone hint defaults to the accepted \
                 types and the size limit.",
            ),
            doc::note(
                "Iced reports dropped files for the whole window, not for the widget under the \
                 pointer, so only one zone of a window may accept drops; give the others \
                 `accepts_drops(false)`, as on this page. `drop_zone::can_drop_files` says \
                 whether files dragged from the desktop reach the window: not under Wayland. \
                 Dropped folders are refused.",
            ),
            doc::caption(drops),
            doc::heading("Examples"),
            doc::subheading("One file of any type"),
            doc::prose(
                "With `max_files(1)` the file chooser picks one file. `show_previews(false)` \
                 drops the image thumbnails, and `zone_min_height` makes the zone shorter.",
            ),
            document,
            doc::subheading("Disabled"),
            doc::prose(
                "With `enabled(false)` files can be neither added nor removed, and the zone \
                 fades.",
            ),
            disabled,
            doc::subheading("Bare drop zone"),
            doc::prose(
                "`drop_zone` is the dashed zone alone: `on_choose` asks the application to open \
                 the file chooser, for example with `oito2_iced_files::pick`, and `on_drop` \
                 receives dropped files unchecked, for your own handling. `multiple` says \
                 whether it takes several files.",
            ),
            zone,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the zone and the remove buttons."],
                    vec![
                        "Enter, Space",
                        "Opens the file chooser from the zone, or removes the focused file.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_files::upload::file_upload",
                    "oito2_iced_files::upload::FileUpload",
                    "oito2_iced_files::upload::Controller",
                    "oito2_iced_files::upload::Event",
                    "oito2_iced_files::upload::Rejection",
                    "oito2_iced_files::upload::RejectedFile",
                    "oito2_iced_files::drop_zone::drop_zone",
                    "oito2_iced_files::drop_zone::DropZone",
                    "oito2_iced_files::drop_zone::can_drop_files",
                    "oito2_iced_files::file::File",
                    "oito2_iced_files::file::TypeGroup",
                    "oito2_iced_files::file::matches_types",
                    "oito2_iced_files::pick",
                ],
            ),
        ],
    )
}
