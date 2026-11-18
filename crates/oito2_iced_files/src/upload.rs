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

//! A file upload: a [drop zone](crate::drop_zone::drop_zone) that adds
//! files to a [`Controller`] owned by the application, the files refused by
//! the last addition with the reasons, and the list of accepted files with
//! their size, a preview of images and a remove button.
//!
//! The [`Controller`] checks the files against the accepted types, the size
//! limit and the largest number of files as they are added; directories
//! are always refused. [`file_upload`] publishes [`Event`]s that the
//! application hands to [`Controller::update`], which applies them and
//! returns the [`Task`] of the file chooser when the zone is activated.
//!
//! ```no_run
//! use iced::Task;
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_files::file::TypeGroup;
//! use oito2_iced_files::upload::{self, Controller};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Upload(upload::Event),
//! }
//!
//! struct App {
//!     images: Controller,
//! }
//!
//! impl App {
//!     fn new() -> Self {
//!         let images = TypeGroup::new()
//!             .label("PNG, JPEG, WebP")
//!             .extensions(["png", "jpg", "jpeg", "webp"])
//!             .mime_types(["image/png", "image/jpeg", "image/webp"]);
//!
//!         Self {
//!             images: Controller::new()
//!                 .accept([images])
//!                 .max_files(4)
//!                 .max_file_size(5 * 1000 * 1000),
//!         }
//!     }
//!
//!     fn update(&mut self, message: Message) -> Task<Message> {
//!         match message {
//!             Message::Upload(event) => self.images.update(event).map(Message::Upload),
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         upload::file_upload(&self.images, &Localizations::en())
//!             .on_event(Message::Upload)
//!             .into()
//!     }
//! }
//! ```

use std::borrow::Cow;

use iced::font::Weight;
use iced::widget::text::{IntoFragment, Wrapping};
use iced::widget::{Column, Row, column, container, image};
use iced::{Alignment, Border, ContentFit, Length, Task};
use oito2_iced::format::{ByteUnits, Format};
use oito2_iced::icon::{icon, lucide};
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::button::{close_button, icon_button};
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::info::{InfoType, info_box};
use oito2_iced::widget::styled_text;
use oito2_iced::{Element, Theme};

use crate::drop_zone::{MIN_HEIGHT, drop_zone};
use crate::file::{File, TypeGroup, matches_types};

/// The side of the thumbnail of a listed file.
pub const THUMBNAIL_SIZE: f32 = 40.0;

/// The digits after the decimal separator of byte sizes.
const SIZE_FRACTION_DIGITS: usize = 1;

/// Why a [`Controller`] refused a file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rejection {
    /// The file type is not one of the accepted types.
    Type,
    /// The file is larger than the size limit.
    Size,
    /// The upload already holds the largest number of files.
    Count,
    /// The item is a directory, not a file.
    Directory,
}

/// A file a [`Controller`] refused, with the reason.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RejectedFile {
    /// The refused file.
    pub file: File,
    /// Why it was refused.
    pub reason: Rejection,
}

/// What happened in a [`file_upload`], for [`Controller::update`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// The drop zone was activated: the file chooser should open.
    Choose,
    /// Files were dropped or chosen.
    Add(Vec<File>),
    /// The remove button of the file at this index was pressed.
    Remove(usize),
    /// The box of refused files was closed.
    ClearRejected,
}

/// The files of a file upload, checked against the [accepted
/// types](Controller::accept), the [size limit](Controller::max_file_size)
/// and the [largest number of files](Controller::max_files) as they are
/// added; directories are always refused.
///
/// [`add`](Controller::add) keeps the files that pass the checks and lists
/// the others in [`rejected`](Controller::rejected), which holds the
/// refusals of the last addition only.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Controller {
    accept: Vec<TypeGroup>,
    max_files: Option<usize>,
    max_file_size: Option<u64>,
    files: Vec<File>,
    rejected: Vec<RejectedFile>,
}

impl Controller {
    /// A controller without limits and without files.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the accepted file types; none accepts every file.
    #[must_use]
    pub fn accept(mut self, accept: impl IntoIterator<Item = TypeGroup>) -> Self {
        self.accept = accept.into_iter().collect();
        self
    }

    /// Sets the largest number of files; zero counts as one.
    #[must_use]
    pub fn max_files(mut self, max_files: usize) -> Self {
        self.max_files = Some(max_files.max(1));
        self
    }

    /// Sets the largest file size, in bytes; zero counts as one.
    #[must_use]
    pub fn max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = Some(max_file_size.max(1));
        self
    }

    /// Sets the files held at first, unchecked.
    #[must_use]
    pub fn files(mut self, files: impl IntoIterator<Item = File>) -> Self {
        self.files = files.into_iter().collect();
        self
    }

    /// The accepted file types; empty accepts every file.
    #[must_use]
    pub fn accepted_types(&self) -> &[TypeGroup] {
        &self.accept
    }

    /// The largest number of files, or `None` for no limit.
    #[must_use]
    pub fn file_limit(&self) -> Option<usize> {
        self.max_files
    }

    /// The largest file size in bytes, or `None` for no limit.
    #[must_use]
    pub fn size_limit(&self) -> Option<u64> {
        self.max_file_size
    }

    /// The accepted files, in the order they were added.
    #[must_use]
    pub fn accepted(&self) -> &[File] {
        &self.files
    }

    /// The files refused by the last [`add`](Controller::add).
    #[must_use]
    pub fn rejected(&self) -> &[RejectedFile] {
        &self.rejected
    }

    /// Whether more files can be added.
    #[must_use]
    pub fn can_add_more(&self) -> bool {
        self.max_files.is_none_or(|max| self.files.len() < max)
    }

    /// Whether the file chooser takes several files: unless the controller
    /// holds one file at most.
    #[must_use]
    pub fn multiple(&self) -> bool {
        self.max_files != Some(1)
    }

    /// Adds the files of `incoming` that pass the checks, in order, and
    /// lists the others in [`rejected`](Controller::rejected), replacing
    /// the refusals of the previous addition.
    ///
    /// A file is refused when it is a directory, else when its type is not
    /// accepted, else when it is larger than the size limit, else when the
    /// upload is full.
    pub fn add(&mut self, incoming: impl IntoIterator<Item = File>) {
        let mut rejected = Vec::new();

        for file in incoming {
            let reason = if file.directory {
                Some(Rejection::Directory)
            } else if !matches_types(&file, &self.accept) {
                Some(Rejection::Type)
            } else if self.max_file_size.is_some_and(|max| file.size > max) {
                Some(Rejection::Size)
            } else if self.max_files.is_some_and(|max| self.files.len() >= max) {
                Some(Rejection::Count)
            } else {
                None
            };

            match reason {
                Some(reason) => rejected.push(RejectedFile { file, reason }),
                None => self.files.push(file),
            }
        }

        self.rejected = rejected;
    }

    /// Removes the file at `index`, if there is one.
    pub fn remove(&mut self, index: usize) {
        if index < self.files.len() {
            self.files.remove(index);
        }
    }

    /// Removes every file and refusal.
    pub fn clear(&mut self) {
        self.files.clear();
        self.rejected.clear();
    }

    /// Forgets the refusals of the last addition.
    pub fn clear_rejected(&mut self) {
        self.rejected.clear();
    }

    /// Applies `event`: [`Event::Choose`] returns the task of the file
    /// chooser, which yields an [`Event::Add`] with the chosen files;
    /// [`Event::Add`] adds the files, ignoring an empty list (a canceled
    /// chooser); the others remove a file or the refusals.
    pub fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::Choose => crate::pick(&self.accept, self.multiple()).map(Event::Add),
            Event::Add(files) => {
                if !files.is_empty() {
                    self.add(files);
                }

                Task::none()
            }
            Event::Remove(index) => {
                self.remove(index);
                Task::none()
            }
            Event::ClearRejected => {
                self.clear_rejected();
                Task::none()
            }
        }
    }

    /// The localized accepted types and size limit, such as "Accepted:
    /// Images · Up to 1 kB each", or `None` without both.
    #[must_use]
    pub fn hint(&self, localizations: &Localizations) -> Option<String> {
        let format = Format::new(localizations.locale_name);
        let mut parts = Vec::new();

        if !self.accept.is_empty() {
            let names: Vec<String> = self
                .accept
                .iter()
                .filter_map(TypeGroup::display_name)
                .collect();

            parts.push(localizations.accepted_file_types(&names.join(", ")));
        }

        if let Some(max) = self.max_file_size {
            parts.push(localizations.max_file_size_hint(&format.bytes(
                i64::try_from(max).unwrap_or(i64::MAX),
                ByteUnits::Si,
                SIZE_FRACTION_DIGITS,
            )));
        }

        (!parts.is_empty()).then(|| parts.join(" · "))
    }

    /// The localized message that explains why `rejected` was refused.
    #[must_use]
    pub fn reason(&self, rejected: &RejectedFile, localizations: &Localizations) -> String {
        let name = &rejected.file.name;

        match rejected.reason {
            Rejection::Type => localizations.file_type_not_accepted(name),
            Rejection::Size => localizations.file_too_large(
                name,
                &size_text(self.max_file_size.unwrap_or(0), localizations),
            ),
            Rejection::Count => localizations.too_many_files(
                name,
                i64::try_from(self.max_files.unwrap_or(0)).unwrap_or(i64::MAX),
            ),
            Rejection::Directory => localizations.folder_not_accepted(name),
        }
    }
}

/// `bytes` formatted in the locale of `localizations`, such as "1.5 kB".
fn size_text(bytes: u64, localizations: &Localizations) -> String {
    Format::new(localizations.locale_name).bytes(
        i64::try_from(bytes).unwrap_or(i64::MAX),
        ByteUnits::Si,
        SIZE_FRACTION_DIGITS,
    )
}

/// A file upload. See [`file_upload`].
pub struct FileUpload<'a, Message> {
    controller: &'a Controller,
    localizations: Localizations,
    on_event: Option<Box<dyn Fn(Event) -> Message + 'a>>,
    hint: Option<Cow<'a, str>>,
    show_previews: bool,
    enabled: bool,
    zone_min_height: f32,
    accepts_drops: Option<bool>,
    direction: ReadingDirection,
}

/// A file upload for `controller`, filling the available width: a drop
/// zone, the box of files refused by the last addition with the reasons
/// and a close button, and a row per accepted file with a thumbnail (a
/// preview for images) or a file icon, the name, the size and a remove
/// button.
///
/// The zone hint defaults to [`Controller::hint`]; the zone is disabled
/// once the controller holds its largest number of files. Without
/// [`on_event`](FileUpload::on_event) nothing can be added or removed.
pub fn file_upload<'a, Message>(
    controller: &'a Controller,
    localizations: &Localizations,
) -> FileUpload<'a, Message> {
    FileUpload {
        controller,
        localizations: *localizations,
        on_event: None,
        hint: None,
        show_previews: true,
        enabled: true,
        zone_min_height: MIN_HEIGHT,
        accepts_drops: None,
        direction: ReadingDirection::default(),
    }
}

impl<'a, Message> FileUpload<'a, Message> {
    /// Sets the message that receives the [`Event`]s of the upload.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(Event) -> Message + 'a) -> Self {
        self.on_event = Some(Box::new(on_event));
        self
    }

    /// Replaces the default zone hint, which lists the accepted types and
    /// the size limit.
    #[must_use]
    pub fn hint(mut self, hint: impl IntoFragment<'a>) -> Self {
        self.hint = Some(hint.into_fragment());
        self
    }

    /// Sets whether image files show a thumbnail (true by default).
    #[must_use]
    pub fn show_previews(mut self, show_previews: bool) -> Self {
        self.show_previews = show_previews;
        self
    }

    /// Sets whether files can be added and removed (true by default).
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the smallest height of the drop zone, in logical pixels.
    #[must_use]
    pub fn zone_min_height(mut self, min_height: f32) -> Self {
        self.zone_min_height = min_height;
        self
    }

    /// Sets whether the zone takes dropped files; see
    /// [`DropZone::accepts_drops`](crate::drop_zone::DropZone::accepts_drops).
    #[must_use]
    pub fn accepts_drops(mut self, accepts_drops: bool) -> Self {
        self.accepts_drops = Some(accepts_drops);
        self
    }

    /// Sets the reading direction, which places the thumbnail at the start
    /// and the remove button at the end of each row.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<FileUpload<'a, Message>> for Element<'a, Message> {
    fn from(upload: FileUpload<'a, Message>) -> Self {
        let controller = upload.controller;
        let localizations = upload.localizations;
        let on_event: Option<std::rc::Rc<dyn Fn(Event) -> Message + 'a>> =
            upload.on_event.map(std::rc::Rc::from);

        let mut zone = drop_zone(&localizations)
            .multiple(controller.multiple())
            .enabled(upload.enabled && controller.can_add_more())
            .min_height(upload.zone_min_height);

        if let Some(accepts_drops) = upload.accepts_drops {
            zone = zone.accepts_drops(accepts_drops);
        }

        match upload.hint {
            Some(hint) => zone = zone.hint(hint),
            None => {
                if let Some(hint) = controller.hint(&localizations) {
                    zone = zone.hint(hint);
                }
            }
        }

        if let Some(on_event) = &on_event {
            let on_drop = on_event.clone();

            zone = zone
                .on_choose(on_event(Event::Choose))
                .on_drop(move |files| on_drop(Event::Add(files)));
        }

        let mut list = Column::new()
            .push(zone)
            .spacing(spacing::SM)
            .width(Length::Fill);

        if !controller.rejected().is_empty() {
            let reasons = Column::with_children(controller.rejected().iter().map(|rejected| {
                styled_text(
                    controller.reason(rejected, &localizations),
                    typography::BODY_MEDIUM,
                )
                .into()
            }));

            list = list.push(
                info_box(InfoType::Danger).content(reasons).trailing(
                    close_button(&localizations)
                        .on_press_maybe(on_event.as_ref().map(|f| f(Event::ClearRejected))),
                ),
            );
        }

        for (index, file) in controller.accepted().iter().enumerate() {
            let on_remove = on_event
                .as_ref()
                .filter(|_| upload.enabled)
                .map(|f| f(Event::Remove(index)));

            list = list.push(file_row(
                file,
                upload.show_previews && file.is_image(),
                on_remove,
                &localizations,
                upload.direction,
            ));
        }

        list.into()
    }
}

/// A file of the upload list: a thumbnail or file icon, the name and size,
/// and a remove button when `on_remove` is set.
fn file_row<'a, Message: Clone + 'a>(
    file: &File,
    preview: bool,
    on_remove: Option<Message>,
    localizations: &Localizations,
    direction: ReadingDirection,
) -> Element<'a, Message> {
    let thumbnail: Element<'a, Message> = if preview {
        image(image::Handle::from_path(&file.path))
            .width(THUMBNAIL_SIZE)
            .height(THUMBNAIL_SIZE)
            .content_fit(ContentFit::Cover)
            .border_radius(radius::SMALL)
            .into()
    } else {
        icon(lucide::FILE).into()
    };
    let thumbnail = container(thumbnail)
        .center(THUMBNAIL_SIZE)
        .style(|theme: &Theme| container::Style {
            text_color: Some(theme.roles().on_surface_variant),
            background: Some(theme.roles().surface_container_highest.into()),
            border: Border::default().rounded(radius::SMALL),
            ..container::Style::default()
        });

    let name = styled_text(
        file.name.clone(),
        typography::TextStyle {
            weight: Weight::Medium,
            ..typography::BODY_MEDIUM
        },
    )
    .wrapping(Wrapping::None);
    let size = styled_text(size_text(file.size, localizations), typography::BODY_SMALL).style(
        |theme: &Theme| iced::widget::text::Style {
            color: Some(theme.roles().on_surface_variant),
        },
    );
    let text = container(column![name, size])
        .width(Length::Fill)
        .clip(true);

    let mut children: Vec<Element<'a, Message>> = vec![thumbnail.into(), text.into()];

    if let Some(message) = on_remove {
        children.push(
            icon_button(lucide::X)
                .tooltip(localizations.remove_item(&file.name))
                .on_press(message)
                .into(),
        );
    }

    if direction == ReadingDirection::RightToLeft {
        children.reverse();
    }

    container(
        Row::with_children(children)
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
    )
    .padding(spacing::SM)
    .width(Length::Fill)
    .style(|theme: &Theme| container::Style {
        border: Border::default()
            .color(theme.roles().outline_variant)
            .width(1.0)
            .rounded(radius::MEDIUM),
        ..container::Style::default()
    })
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn images() -> TypeGroup {
        TypeGroup::new()
            .label("Images")
            .extensions(["png", "jpg"])
            .mime_types(["image/*"])
    }

    fn names(files: &[File]) -> Vec<&str> {
        files.iter().map(|file| file.name.as_str()).collect()
    }

    #[test]
    fn refuses_files_by_type_size_and_count() {
        let mut controller = Controller::new()
            .accept([images()])
            .max_files(2)
            .max_file_size(100);

        controller.add([
            File::new("a.png", 10),
            File::new("b.pdf", 10),
            File::new("c.png", 500),
            File::new("d.jpg", 10),
            File::new("e.png", 10),
        ]);

        assert_eq!(names(controller.accepted()), ["a.png", "d.jpg"]);
        let rejected: Vec<(&str, Rejection)> = controller
            .rejected()
            .iter()
            .map(|rejected| (rejected.file.name.as_str(), rejected.reason))
            .collect();
        assert_eq!(
            rejected,
            [
                ("b.pdf", Rejection::Type),
                ("c.png", Rejection::Size),
                ("e.png", Rejection::Count),
            ]
        );
        assert!(!controller.can_add_more());
    }

    #[test]
    fn directories_are_refused_before_any_other_check() {
        let mut controller = Controller::new().accept([images()]);

        controller.add([File::new("photos", 0).as_directory()]);

        assert!(controller.accepted().is_empty());
        assert_eq!(controller.rejected()[0].reason, Rejection::Directory);
    }

    #[test]
    fn remove_clear_rejected_and_clear() {
        let mut controller = Controller::new().accept([images()]);

        controller.add([File::new("a.png", 1), File::new("b.pdf", 1)]);
        controller.remove(0);
        controller.remove(5);
        assert!(controller.accepted().is_empty());
        controller.clear_rejected();
        assert!(controller.rejected().is_empty());

        controller.add([File::new("c.png", 1), File::new("d.pdf", 1)]);
        controller.clear();
        assert!(controller.accepted().is_empty());
        assert!(controller.rejected().is_empty());
    }

    #[test]
    fn a_later_add_replaces_the_refusals() {
        let mut controller = Controller::new().accept([images()]);

        controller.add([File::new("b.pdf", 1)]);
        controller.add([File::new("c.png", 1)]);

        assert!(controller.rejected().is_empty());
        assert_eq!(names(controller.accepted()), ["c.png"]);
    }

    #[test]
    fn successive_adds_respect_the_file_limit() {
        let mut controller = Controller::new().max_files(1).max_file_size(100);

        controller.add([File::new("a.txt", 1), File::new("b.txt", 1)]);
        controller.add([File::new("c.txt", 1)]);

        assert_eq!(names(controller.accepted()), ["a.txt"]);
        assert_eq!(controller.rejected()[0].file.name, "c.txt");
        assert!(!controller.multiple());
    }

    #[test]
    fn events_update_the_files() {
        let mut controller = Controller::new().accept([images()]);

        let _ = controller.update(Event::Add(vec![
            File::new("a.png", 1),
            File::new("b.pdf", 1),
        ]));
        assert_eq!(controller.rejected().len(), 1);
        let _ = controller.update(Event::Add(Vec::new()));
        assert_eq!(controller.rejected().len(), 1);
        let _ = controller.update(Event::ClearRejected);
        assert!(controller.rejected().is_empty());
        let _ = controller.update(Event::Remove(0));
        assert!(controller.accepted().is_empty());
    }

    #[test]
    fn the_hint_lists_the_types_and_the_size_limit() {
        let en = Localizations::en();
        let controller = Controller::new().accept([images()]).max_file_size(1000);

        assert_eq!(
            controller.hint(&en).as_deref(),
            Some("Accepted: Images · Up to 1 kB each")
        );
        assert_eq!(
            Controller::new()
                .accept([TypeGroup::new().extensions(["png", "jpg"])])
                .hint(&Localizations::pt_br())
                .as_deref(),
            Some("Aceitos: png, jpg")
        );
        assert_eq!(Controller::new().hint(&en), None);
        assert_eq!(
            Controller::new().max_file_size(1500).hint(&en).as_deref(),
            Some("Up to 1.5 kB each")
        );
    }

    #[test]
    fn reasons_explain_each_refusal() {
        let en = Localizations::en();
        let controller = Controller::new().max_files(1).max_file_size(5_000_000);
        let reason = |file: &str, reason| {
            controller.reason(
                &RejectedFile {
                    file: File::new(file, 1),
                    reason,
                },
                &en,
            )
        };

        assert_eq!(
            reason("notes.pdf", Rejection::Type),
            "notes.pdf: this file type is not accepted"
        );
        assert_eq!(
            reason("big.png", Rejection::Size),
            "big.png: larger than 5 MB"
        );
        assert_eq!(
            reason("b.txt", Rejection::Count),
            "b.txt: only 1 file can be added"
        );
        assert_eq!(
            reason("photos", Rejection::Directory),
            "photos: folders are not accepted"
        );
    }
}
