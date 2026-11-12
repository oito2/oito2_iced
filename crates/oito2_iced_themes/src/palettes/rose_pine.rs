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

//! The Rosé Pine palette.

use iced::Color;

const fn hex(rgb: u32) -> Color {
    Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

/// The colors of one Rosé Pine variant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RosePine {
    /// The `base` color.
    pub base: Color,
    /// The `surface` color.
    pub surface: Color,
    /// The `overlay` color.
    pub overlay: Color,
    /// The `muted` color.
    pub muted: Color,
    /// The `subtle` color.
    pub subtle: Color,
    /// The `text` color.
    pub text: Color,
    /// The `love` color.
    pub love: Color,
    /// The `gold` color.
    pub gold: Color,
    /// The `rose` color.
    pub rose: Color,
    /// The `pine` color.
    pub pine: Color,
    /// The `foam` color.
    pub foam: Color,
    /// The `iris` color.
    pub iris: Color,
}

impl RosePine {
    /// The main (dark) variant.
    pub const MAIN: RosePine = RosePine {
        base: hex(0x191724),
        surface: hex(0x1F1D2E),
        overlay: hex(0x26233A),
        muted: hex(0x6E6A86),
        subtle: hex(0x908CAA),
        text: hex(0xE0DEF4),
        love: hex(0xEB6F92),
        gold: hex(0xF6C177),
        rose: hex(0xEBBCBA),
        pine: hex(0x31748F),
        foam: hex(0x9CCFD8),
        iris: hex(0xC4A7E7),
    };

    /// The moon (dark) variant.
    pub const MOON: RosePine = RosePine {
        base: hex(0x232136),
        surface: hex(0x2A273F),
        overlay: hex(0x393552),
        muted: hex(0x6E6A86),
        subtle: hex(0x908CAA),
        text: hex(0xE0DEF4),
        love: hex(0xEB6F92),
        gold: hex(0xF6C177),
        rose: hex(0xEA9A97),
        pine: hex(0x3E8FB0),
        foam: hex(0x9CCFD8),
        iris: hex(0xC4A7E7),
    };

    /// The dawn (light) variant.
    pub const DAWN: RosePine = RosePine {
        base: hex(0xFAF4ED),
        surface: hex(0xFFFAF3),
        overlay: hex(0xF2E9E1),
        muted: hex(0x9893A5),
        subtle: hex(0x797593),
        text: hex(0x464261),
        love: hex(0xB4637A),
        gold: hex(0xEA9D34),
        rose: hex(0xD7827E),
        pine: hex(0x286983),
        foam: hex(0x56949F),
        iris: hex(0x907AA9),
    };
}
