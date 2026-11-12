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
//! Inter is embedded as four static instances (Regular, Medium, SemiBold and
//! Bold) of the family `Inter`. Register every entry of [`INTER_FONTS`] with
//! the application (`iced::application(..).font(..)` once per entry) and use
//! [`INTER`] as the default font; weights are selected with
//! [`iced::font::Weight`]. Weights other than those four resolve to the
//! nearest embedded instance.

use iced::Font;
use iced::font::Weight;

/// The bytes of `Inter-Regular.ttf` (weight 400, SIL Open Font License 1.1).
pub const INTER_REGULAR: &[u8] = include_bytes!("../assets/fonts/Inter-Regular.ttf");

/// The bytes of `Inter-Medium.ttf` (weight 500, SIL Open Font License 1.1).
pub const INTER_MEDIUM: &[u8] = include_bytes!("../assets/fonts/Inter-Medium.ttf");

/// The bytes of `Inter-SemiBold.ttf` (weight 600, SIL Open Font License 1.1).
pub const INTER_SEMIBOLD: &[u8] = include_bytes!("../assets/fonts/Inter-SemiBold.ttf");

/// The bytes of `Inter-Bold.ttf` (weight 700, SIL Open Font License 1.1).
pub const INTER_BOLD: &[u8] = include_bytes!("../assets/fonts/Inter-Bold.ttf");

/// Every embedded Inter instance, to register with the application.
pub const INTER_FONTS: [&[u8]; 4] = [INTER_REGULAR, INTER_MEDIUM, INTER_SEMIBOLD, INTER_BOLD];

/// Inter at regular weight.
pub const INTER: Font = Font::with_name("Inter");

/// Inter at the given weight.
pub const fn inter(weight: Weight) -> Font {
    Font { weight, ..INTER }
}
