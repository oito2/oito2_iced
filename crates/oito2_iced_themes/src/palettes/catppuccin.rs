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

//! The Catppuccin palette, read from the `catppuccin` crate.

use iced::Color;

fn color(color: &catppuccin::Color) -> Color {
    Color::from_rgb8(color.rgb.r, color.rgb.g, color.rgb.b)
}

/// The colors of one Catppuccin flavor.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flavor {
    /// The `rosewater` color.
    pub rosewater: Color,
    /// The `flamingo` color.
    pub flamingo: Color,
    /// The `pink` color.
    pub pink: Color,
    /// The `mauve` color.
    pub mauve: Color,
    /// The `red` color.
    pub red: Color,
    /// The `maroon` color.
    pub maroon: Color,
    /// The `peach` color.
    pub peach: Color,
    /// The `yellow` color.
    pub yellow: Color,
    /// The `green` color.
    pub green: Color,
    /// The `teal` color.
    pub teal: Color,
    /// The `sky` color.
    pub sky: Color,
    /// The `sapphire` color.
    pub sapphire: Color,
    /// The `blue` color.
    pub blue: Color,
    /// The `lavender` color.
    pub lavender: Color,
    /// The `text` color.
    pub text: Color,
    /// The `subtext1` color.
    pub subtext1: Color,
    /// The `subtext0` color.
    pub subtext0: Color,
    /// The `overlay2` color.
    pub overlay2: Color,
    /// The `overlay1` color.
    pub overlay1: Color,
    /// The `overlay0` color.
    pub overlay0: Color,
    /// The `surface2` color.
    pub surface2: Color,
    /// The `surface1` color.
    pub surface1: Color,
    /// The `surface0` color.
    pub surface0: Color,
    /// The `base` color.
    pub base: Color,
    /// The `mantle` color.
    pub mantle: Color,
    /// The `crust` color.
    pub crust: Color,
}

impl Flavor {
    fn from_catppuccin(flavor: &catppuccin::Flavor) -> Self {
        let colors = &flavor.colors;

        Self {
            rosewater: color(&colors.rosewater),
            flamingo: color(&colors.flamingo),
            pink: color(&colors.pink),
            mauve: color(&colors.mauve),
            red: color(&colors.red),
            maroon: color(&colors.maroon),
            peach: color(&colors.peach),
            yellow: color(&colors.yellow),
            green: color(&colors.green),
            teal: color(&colors.teal),
            sky: color(&colors.sky),
            sapphire: color(&colors.sapphire),
            blue: color(&colors.blue),
            lavender: color(&colors.lavender),
            text: color(&colors.text),
            subtext1: color(&colors.subtext1),
            subtext0: color(&colors.subtext0),
            overlay2: color(&colors.overlay2),
            overlay1: color(&colors.overlay1),
            overlay0: color(&colors.overlay0),
            surface2: color(&colors.surface2),
            surface1: color(&colors.surface1),
            surface0: color(&colors.surface0),
            base: color(&colors.base),
            mantle: color(&colors.mantle),
            crust: color(&colors.crust),
        }
    }
}

/// The light flavor.
pub fn latte() -> Flavor {
    Flavor::from_catppuccin(&catppuccin::PALETTE.latte)
}

/// The lightest dark flavor.
pub fn frappe() -> Flavor {
    Flavor::from_catppuccin(&catppuccin::PALETTE.frappe)
}

/// The middle dark flavor.
pub fn macchiato() -> Flavor {
    Flavor::from_catppuccin(&catppuccin::PALETTE.macchiato)
}

/// The darkest dark flavor.
pub fn mocha() -> Flavor {
    Flavor::from_catppuccin(&catppuccin::PALETTE.mocha)
}
