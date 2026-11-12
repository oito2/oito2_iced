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

//! The Dracula and Alucard palette.

use iced::Color;

const fn hex(rgb: u32) -> Color {
    Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

/// The colors of one Dracula variant.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dracula {
    /// The `background` color.
    pub background: Color,
    /// The `currentLine` color.
    pub current_line: Color,
    /// The `foreground` color.
    pub foreground: Color,
    /// The `comment` color.
    pub comment: Color,
    /// The `cyan` color.
    pub cyan: Color,
    /// The `green` color.
    pub green: Color,
    /// The `orange` color.
    pub orange: Color,
    /// The `pink` color.
    pub pink: Color,
    /// The `purple` color.
    pub purple: Color,
    /// The `red` color.
    pub red: Color,
    /// The `yellow` color.
    pub yellow: Color,
}

impl Dracula {
    /// The dark Dracula variant.
    pub const DRACULA: Dracula = Dracula {
        background: hex(0x282A36),
        current_line: hex(0x44475A),
        foreground: hex(0xF8F8F2),
        comment: hex(0x6272A4),
        cyan: hex(0x8BE9FD),
        green: hex(0x50FA7B),
        orange: hex(0xFFB86C),
        pink: hex(0xFF79C6),
        purple: hex(0xBD93F9),
        red: hex(0xFF5555),
        yellow: hex(0xF1FA8C),
    };

    /// The light Alucard variant.
    pub const ALUCARD: Dracula = Dracula {
        background: hex(0xFFFBEB),
        current_line: hex(0x6C664B),
        foreground: hex(0x1F1F1F),
        comment: hex(0x6C664B),
        cyan: hex(0x036A96),
        green: hex(0x14710A),
        orange: hex(0xA34D14),
        pink: hex(0xA3144D),
        purple: hex(0x644AC9),
        red: hex(0xCB3A2A),
        yellow: hex(0x846E15),
    };
}
