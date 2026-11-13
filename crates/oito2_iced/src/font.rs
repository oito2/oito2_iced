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

//! Embedded fonts.
//!
//! Inter is embedded as eight static instances: Regular, Medium, SemiBold
//! and Bold, upright and italic, all in the family `OITO2 Inter`, so an
//! Inter installed on the system never takes the place of the embedded
//! faces. Register every
//! entry of [`INTER_FONTS`] with the application
//! (`iced::application(..).font(..)` once per entry) and use [`INTER`] as
//! the default font; weights are selected with [`iced::font::Weight`] and
//! italics with [`iced::font::Style`]. Weights other than those four
//! resolve to the nearest embedded instance.

use iced::Font;
use iced::font::{Style, Weight};

macro_rules! inter_file {
    ($file:literal) => {
        include_bytes!(concat!("../assets/fonts/", $file))
    };
}

/// The bytes of `Inter-Regular.ttf` (weight 400, SIL Open Font License 1.1).
pub const INTER_REGULAR: &[u8] = inter_file!("Inter-Regular.ttf");

/// The bytes of `Inter-Medium.ttf` (weight 500, SIL Open Font License 1.1).
pub const INTER_MEDIUM: &[u8] = inter_file!("Inter-Medium.ttf");

/// The bytes of `Inter-SemiBold.ttf` (weight 600, SIL Open Font License 1.1).
pub const INTER_SEMIBOLD: &[u8] = inter_file!("Inter-SemiBold.ttf");

/// The bytes of `Inter-Bold.ttf` (weight 700, SIL Open Font License 1.1).
pub const INTER_BOLD: &[u8] = inter_file!("Inter-Bold.ttf");

/// The bytes of `Inter-Italic.ttf` (weight 400, SIL Open Font License 1.1).
pub const INTER_ITALIC: &[u8] = inter_file!("Inter-Italic.ttf");

/// The bytes of `Inter-MediumItalic.ttf` (weight 500, SIL Open Font License
/// 1.1).
pub const INTER_MEDIUM_ITALIC: &[u8] = inter_file!("Inter-MediumItalic.ttf");

/// The bytes of `Inter-SemiBoldItalic.ttf` (weight 600, SIL Open Font
/// License 1.1).
pub const INTER_SEMIBOLD_ITALIC: &[u8] = inter_file!("Inter-SemiBoldItalic.ttf");

/// The bytes of `Inter-BoldItalic.ttf` (weight 700, SIL Open Font License
/// 1.1).
pub const INTER_BOLD_ITALIC: &[u8] = inter_file!("Inter-BoldItalic.ttf");

/// Every embedded Inter instance, to register with the application.
pub const INTER_FONTS: [&[u8]; 8] = [
    INTER_REGULAR,
    INTER_MEDIUM,
    INTER_SEMIBOLD,
    INTER_BOLD,
    INTER_ITALIC,
    INTER_MEDIUM_ITALIC,
    INTER_SEMIBOLD_ITALIC,
    INTER_BOLD_ITALIC,
];

/// The family name of the embedded Inter instances.
pub const INTER_FAMILY: &str = "OITO2 Inter";

/// Inter at regular weight.
pub const INTER: Font = Font::with_name(INTER_FAMILY);

/// Inter at the given weight.
pub const fn inter(weight: Weight) -> Font {
    Font { weight, ..INTER }
}

/// Inter italic at the given weight.
pub const fn inter_italic(weight: Weight) -> Font {
    Font {
        weight,
        style: Style::Italic,
        ..INTER
    }
}
