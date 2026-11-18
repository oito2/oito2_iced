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

//! The file chooser of the platform, as a [`Task`] the application runs.
//!
//! On Linux the chooser goes through the XDG desktop portal; on Windows and
//! macOS through the native dialog.
//!
//! ```no_run
//! use iced::Task;
//! use oito2_iced_files::file::{File, TypeGroup};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Choose,
//!     Chosen(Vec<File>),
//! }
//!
//! fn update(message: Message) -> Task<Message> {
//!     match message {
//!         Message::Choose => {
//!             let pdf = TypeGroup::new().label("PDF").extensions(["pdf"]);
//!
//!             oito2_iced_files::pick(&[pdf], true).map(Message::Chosen)
//!         }
//!         Message::Chosen(files) => {
//!             println!("{} files", files.len());
//!             Task::none()
//!         }
//!     }
//! }
//! ```

use iced::Task;

use crate::file::{File, TypeGroup};

/// Opens the file chooser for one file, or several when `multiple`, offering
/// the `accept`ed types, and yields the chosen files; nothing when the
/// chooser is canceled.
///
/// The chooser filters by extension only: when a group has no extensions
/// (no filter, or MIME types only) it offers every file. Files whose size
/// can't be read are left out.
pub fn pick(accept: &[TypeGroup], multiple: bool) -> Task<Vec<File>> {
    let mut dialog = rfd::AsyncFileDialog::new();

    for (name, extensions) in filters(accept) {
        dialog = dialog.add_filter(name, &extensions);
    }

    Task::future(async move {
        let handles = if multiple {
            dialog.pick_files().await.unwrap_or_default()
        } else {
            dialog.pick_file().await.into_iter().collect()
        };

        handles
            .iter()
            .filter_map(|handle| File::from_path(handle.path()).ok())
            .collect()
    })
}

/// The extension filters of the file chooser for `accept`: one per group,
/// named by its label or its extensions; none when a group has no
/// extensions, since the chooser can't express its MIME types.
pub(crate) fn filters(accept: &[TypeGroup]) -> Vec<(String, Vec<String>)> {
    if accept.iter().any(|group| group.extensions.is_empty()) {
        return Vec::new();
    }

    accept
        .iter()
        .map(|group| {
            let extensions = group.clean_extensions();
            let name = group.label.clone().unwrap_or_else(|| extensions.join(", "));

            (name, extensions)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_follow_the_groups_with_extensions() {
        let images = TypeGroup::new().label("Images").extensions([".PNG", "jpg"]);
        let text = TypeGroup::new().extensions(["txt"]);

        assert_eq!(
            filters(&[images.clone(), text]),
            vec![
                (
                    "Images".to_owned(),
                    vec!["png".to_owned(), "jpg".to_owned()]
                ),
                ("txt".to_owned(), vec!["txt".to_owned()]),
            ]
        );
        assert!(filters(&[]).is_empty());
        assert!(filters(&[images.clone(), TypeGroup::new().label("All")]).is_empty());
        assert!(filters(&[images, TypeGroup::new().mime_types(["image/*"])]).is_empty());
    }
}
