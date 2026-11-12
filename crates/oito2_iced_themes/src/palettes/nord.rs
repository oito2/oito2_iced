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

//! The Nord palette.

use iced::Color;

const fn hex(rgb: u32) -> Color {
    Color::from_rgb8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

/// The `nord0` color.
pub const NORD0: Color = hex(0x2E3440);

/// The `nord1` color.
pub const NORD1: Color = hex(0x3B4252);

/// The `nord2` color.
pub const NORD2: Color = hex(0x434C5E);

/// The `nord3` color.
pub const NORD3: Color = hex(0x4C566A);

/// The `nord4` color.
pub const NORD4: Color = hex(0xD8DEE9);

/// The `nord5` color.
pub const NORD5: Color = hex(0xE5E9F0);

/// The `nord6` color.
pub const NORD6: Color = hex(0xECEFF4);

/// The `nord7` color.
pub const NORD7: Color = hex(0x8FBCBB);

/// The `nord8` color.
pub const NORD8: Color = hex(0x88C0D0);

/// The `nord9` color.
pub const NORD9: Color = hex(0x81A1C1);

/// The `nord10` color.
pub const NORD10: Color = hex(0x5E81AC);

/// The `nord11` color.
pub const NORD11: Color = hex(0xBF616A);

/// The `nord12` color.
pub const NORD12: Color = hex(0xD08770);

/// The `nord13` color.
pub const NORD13: Color = hex(0xEBCB8B);

/// The `nord14` color.
pub const NORD14: Color = hex(0xA3BE8C);

/// The `nord15` color.
pub const NORD15: Color = hex(0xB48EAD);
