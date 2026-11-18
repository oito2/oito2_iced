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

//! The files the widgets of the crate hand over, the groups of accepted
//! file types and the check of a file against them.
//!
//! ```no_run
//! use oito2_iced_files::file::{File, TypeGroup, matches_types};
//!
//! let images = TypeGroup::new()
//!     .label("Images")
//!     .extensions(["png", "jpg"])
//!     .mime_types(["image/*"]);
//!
//! assert!(matches_types(&File::new("photo.PNG", 1024), &[images]));
//! ```

use std::io;
use std::path::PathBuf;

/// The file extensions of the image formats that get a preview.
const IMAGE_EXTENSIONS: [&str; 6] = ["png", "jpg", "jpeg", "gif", "webp", "bmp"];

/// A chosen or dropped file: its path, name, size in bytes, MIME type when
/// known, and whether it is a directory.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct File {
    /// The path of the file in the file system.
    pub path: PathBuf,
    /// The name of the file, the last component of its path.
    pub name: String,
    /// The size of the file, in bytes.
    pub size: u64,
    /// The MIME type of the file, when known.
    pub mime_type: Option<String>,
    /// Whether the item is a directory instead of a file.
    pub directory: bool,
}

impl File {
    /// Describes a file named `name` of `size` bytes, whose path is its
    /// name, without a MIME type.
    #[must_use]
    pub fn new(name: impl Into<String>, size: u64) -> Self {
        let name = name.into();

        Self {
            path: PathBuf::from(&name),
            name,
            size,
            mime_type: None,
            directory: false,
        }
    }

    /// Reads the size of the item at `path` and whether it is a directory;
    /// the name is the last component of the path.
    ///
    /// # Errors
    ///
    /// Returns the error of reading the metadata, such as
    /// [`io::ErrorKind::NotFound`] for a missing path.
    pub fn from_path(path: impl Into<PathBuf>) -> io::Result<Self> {
        let path = path.into();
        let metadata = std::fs::metadata(&path)?;
        let name = path.file_name().map_or_else(
            || path.display().to_string(),
            |name| name.to_string_lossy().into_owned(),
        );

        Ok(Self {
            size: if metadata.is_dir() { 0 } else { metadata.len() },
            directory: metadata.is_dir(),
            name,
            path,
            mime_type: None,
        })
    }

    /// Sets the MIME type.
    #[must_use]
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    /// Sets the path.
    #[must_use]
    pub fn path(mut self, path: impl Into<PathBuf>) -> Self {
        self.path = path.into();
        self
    }

    /// Marks the item as a directory, which a file upload refuses.
    #[must_use]
    pub fn as_directory(mut self) -> Self {
        self.directory = true;
        self
    }

    /// The text after the last dot of the name, in lowercase, or `None`
    /// when the name has no dot.
    #[must_use]
    pub fn extension(&self) -> Option<String> {
        let name = self.name.to_lowercase();

        name.rfind('.').map(|dot| name[dot + 1..].to_owned())
    }

    /// Whether the file is an image that gets a preview: by its MIME type
    /// when known (any `image/` type but SVG), otherwise by the PNG, JPEG,
    /// GIF, WebP and BMP extensions.
    #[must_use]
    pub fn is_image(&self) -> bool {
        match &self.mime_type {
            Some(mime) => mime.starts_with("image/") && mime != "image/svg+xml",
            None => self
                .extension()
                .is_some_and(|extension| IMAGE_EXTENSIONS.contains(&extension.as_str())),
        }
    }
}

/// A group of accepted file types: a label for hints and file choosers,
/// file extensions and MIME types, where `image/*` stands for every image
/// type.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TypeGroup {
    /// The name of the group, such as "Images".
    pub label: Option<String>,
    /// The extensions, with or without a leading dot, in any case.
    pub extensions: Vec<String>,
    /// The MIME types, including wildcards such as `image/*`.
    pub mime_types: Vec<String>,
}

impl TypeGroup {
    /// An empty group, which accepts every file.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the label.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the extensions.
    #[must_use]
    pub fn extensions<S: Into<String>>(mut self, extensions: impl IntoIterator<Item = S>) -> Self {
        self.extensions = extensions.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the MIME types.
    #[must_use]
    pub fn mime_types<S: Into<String>>(mut self, mime_types: impl IntoIterator<Item = S>) -> Self {
        self.mime_types = mime_types.into_iter().map(Into::into).collect();
        self
    }

    /// The extensions in lowercase, without a leading dot.
    #[must_use]
    pub fn clean_extensions(&self) -> Vec<String> {
        self.extensions
            .iter()
            .map(|extension| {
                let lower = extension.to_lowercase();

                lower.strip_prefix('.').unwrap_or(&lower).to_owned()
            })
            .collect()
    }

    /// The text that names the group in a hint: the label, or else the
    /// extensions separated by commas, or `None` without both.
    #[must_use]
    pub fn display_name(&self) -> Option<String> {
        match &self.label {
            Some(label) => Some(label.clone()),
            None if self.extensions.is_empty() => None,
            None => Some(self.extensions.join(", ")),
        }
    }
}

/// Whether `file` is one of the `accept`ed types.
///
/// A group matches when the file name ends with one of its extensions
/// (ignoring case and a leading dot), or when the MIME type of the file is
/// one of its MIME types, where `image/*` matches every image type. No
/// groups, or a group without extensions and MIME types, accept every file.
#[must_use]
pub fn matches_types(file: &File, accept: &[TypeGroup]) -> bool {
    if accept.is_empty() {
        return true;
    }

    let extension = file.extension();
    let mime = file.mime_type.as_ref().map(|mime| mime.to_lowercase());
    let mime_matches = |pattern: &String| {
        let wanted = pattern.to_lowercase();

        match &mime {
            None => false,
            Some(mime) => match wanted.strip_suffix('*') {
                Some(prefix) if prefix.ends_with('/') => mime.starts_with(prefix),
                _ => *mime == wanted,
            },
        }
    };

    accept.iter().any(|group| {
        if group.extensions.is_empty() && group.mime_types.is_empty() {
            return true;
        }

        group
            .clean_extensions()
            .iter()
            .any(|wanted| extension.as_deref() == Some(wanted.as_str()))
            || group.mime_types.iter().any(mime_matches)
    })
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

    #[test]
    fn matches_extensions_mime_types_and_wildcards() {
        assert!(matches_types(&File::new("a.PNG", 1), &[images()]));
        assert!(matches_types(
            &File::new("a", 1).mime_type("image/webp"),
            &[images()]
        ));
        assert!(!matches_types(&File::new("a.pdf", 1), &[images()]));
        assert!(matches_types(
            &File::new("a.txt", 1),
            &[TypeGroup::new().extensions([".TXT"])]
        ));
        assert!(!matches_types(
            &File::new("a", 1).mime_type("text/plain"),
            &[images()]
        ));
        assert!(matches_types(
            &File::new("a", 1).mime_type("TEXT/Plain"),
            &[TypeGroup::new().mime_types(["text/plain"])]
        ));
    }

    #[test]
    fn no_groups_or_a_group_without_filters_accept_every_file() {
        assert!(matches_types(&File::new("a.pdf", 1), &[]));
        assert!(matches_types(
            &File::new("a.pdf", 1),
            &[TypeGroup::new().label("All")]
        ));
    }

    #[test]
    fn the_extension_is_the_text_after_the_last_dot() {
        assert_eq!(File::new("a.tar.GZ", 1).extension().as_deref(), Some("gz"));
        assert_eq!(File::new("README", 1).extension(), None);
        assert_eq!(File::new("a.", 1).extension().as_deref(), Some(""));
    }

    #[test]
    fn images_are_recognized_by_mime_type_or_extension() {
        assert!(File::new("a.webp", 1).is_image());
        assert!(!File::new("a.svg", 1).is_image());
        assert!(File::new("a", 1).mime_type("image/png").is_image());
        assert!(!File::new("a.png", 1).mime_type("image/svg+xml").is_image());
        assert!(!File::new("a.png", 1).mime_type("text/plain").is_image());
    }

    #[test]
    fn display_names_prefer_the_label() {
        assert_eq!(images().display_name().as_deref(), Some("Images"));
        assert_eq!(
            TypeGroup::new()
                .extensions(["png", "jpg"])
                .display_name()
                .as_deref(),
            Some("png, jpg")
        );
        assert_eq!(TypeGroup::new().display_name(), None);
    }

    #[test]
    fn from_path_reads_size_and_directories() {
        let directory =
            std::env::temp_dir().join(format!("oito2_files_unit_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&directory);
        std::fs::create_dir_all(directory.join("photos")).expect("create the directories");
        std::fs::write(directory.join("dot.png"), [1, 2, 3]).expect("write the file");

        let file = File::from_path(directory.join("dot.png")).expect("the file exists");
        assert_eq!(
            (file.name.as_str(), file.size, file.directory),
            ("dot.png", 3, false)
        );

        let folder = File::from_path(directory.join("photos")).expect("the folder exists");
        assert_eq!((folder.name.as_str(), folder.directory), ("photos", true));

        assert!(File::from_path(directory.join("missing.txt")).is_err());

        let _ = std::fs::remove_dir_all(&directory);
    }
}
