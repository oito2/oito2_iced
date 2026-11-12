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

//! The Everforest palette.

use iced::Color;

const fn hex(rgb: u32) -> Color {
    Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

/// The colors of one Everforest variant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Everforest {
    /// The `bgDim` color.
    pub bg_dim: Color,
    /// The `bg0` color.
    pub bg0: Color,
    /// The `bg1` color.
    pub bg1: Color,
    /// The `bg2` color.
    pub bg2: Color,
    /// The `bg3` color.
    pub bg3: Color,
    /// The `bg4` color.
    pub bg4: Color,
    /// The `fg` color.
    pub fg: Color,
    /// The `red` color.
    pub red: Color,
    /// The `orange` color.
    pub orange: Color,
    /// The `yellow` color.
    pub yellow: Color,
    /// The `green` color.
    pub green: Color,
    /// The `aqua` color.
    pub aqua: Color,
    /// The `blue` color.
    pub blue: Color,
    /// The `purple` color.
    pub purple: Color,
    /// The `grey0` color.
    pub grey0: Color,
    /// The `grey1` color.
    pub grey1: Color,
    /// The `grey2` color.
    pub grey2: Color,
}

impl Everforest {
    /// The dark variant.
    pub const DARK: Everforest = Everforest {
        bg_dim: hex(0x232A2E),
        bg0: hex(0x2D353B),
        bg1: hex(0x343F44),
        bg2: hex(0x3D484D),
        bg3: hex(0x475258),
        bg4: hex(0x4F585E),
        fg: hex(0xD3C6AA),
        red: hex(0xE67E80),
        orange: hex(0xE69875),
        yellow: hex(0xDBBC7F),
        green: hex(0xA7C080),
        aqua: hex(0x83C092),
        blue: hex(0x7FBBB3),
        purple: hex(0xD699B6),
        grey0: hex(0x7A8478),
        grey1: hex(0x859289),
        grey2: hex(0x9DA9A0),
    };

    /// The light variant.
    pub const LIGHT: Everforest = Everforest {
        bg_dim: hex(0xEFEBD4),
        bg0: hex(0xFDF6E3),
        bg1: hex(0xF4F0D9),
        bg2: hex(0xEFEBD4),
        bg3: hex(0xE6E2CC),
        bg4: hex(0xE0DCC7),
        fg: hex(0x5C6A72),
        red: hex(0xF85552),
        orange: hex(0xF57D26),
        yellow: hex(0xDFA000),
        green: hex(0x8DA101),
        aqua: hex(0x35A77C),
        blue: hex(0x3A94C5),
        purple: hex(0xDF69BA),
        grey0: hex(0xA6B0A0),
        grey1: hex(0x939F91),
        grey2: hex(0x829181),
    };
}
