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
//! Inter is embedded as its variable font. Register [`INTER_VARIABLE`] with
//! the application (`iced::application(..).font(INTER_VARIABLE)`) and use
//! [`INTER`] as the default font; weights are selected with
//! [`iced::font::Weight`].

use iced::Font;
use iced::font::Weight;

/// The bytes of `InterVariable.ttf` (SIL Open Font License 1.1).
pub const INTER_VARIABLE: &[u8] = include_bytes!("../assets/fonts/InterVariable.ttf");

/// Inter at regular weight.
pub const INTER: Font = Font::with_name("Inter Variable");

/// Inter at the given weight.
pub const fn inter(weight: Weight) -> Font {
    Font { weight, ..INTER }
}
