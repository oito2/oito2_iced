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

//! Icons: every Lucide icon as an [`Icon`] constant in [`lucide`], the
//! freedesktop.org icon names in [`Freedesktop`], the [`icon`] widget that
//! draws an icon in the text color, and the [`placeholder`] that stands in
//! for a missing picture.
//!
//! ```no_run
//! use oito2_iced::icon::{self, Freedesktop, lucide};
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! # let _: oito2_iced::Element<'a, ()> =
//! icon::icon(lucide::FOLDER_OPEN).into();
//! icon::icon(Freedesktop::DocumentSave.icon()).size(24.0).into()
//! # }
//! ```

mod freedesktop;
#[cfg(feature = "svg")]
mod glyph;
#[rustfmt::skip]
pub mod lucide;
mod placeholder;

pub use freedesktop::Freedesktop;
#[cfg(feature = "svg")]
pub use glyph::{Glyph, icon};
pub use placeholder::{Placeholder, placeholder};

/// A monochrome vector icon on a 24-unit grid: its name and its SVG source,
/// drawn with `currentColor` strokes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Icon {
    name: &'static str,
    svg: &'static str,
}

impl Icon {
    /// An icon named `name` with the SVG source `svg`.
    pub const fn new(name: &'static str, svg: &'static str) -> Self {
        Self { name, svg }
    }

    /// The name of the icon, such as `folder-open`.
    pub const fn name(self) -> &'static str {
        self.name
    }

    /// The SVG source of the icon.
    pub const fn svg(self) -> &'static str {
        self.svg
    }

    /// A handle to the SVG source, for the native `svg` widget.
    #[cfg(feature = "svg")]
    pub fn handle(self) -> iced::widget::svg::Handle {
        iced::widget::svg::Handle::from_memory(self.svg.as_bytes())
    }
}
