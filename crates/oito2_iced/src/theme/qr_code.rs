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

//! QR code classes.

use iced::widget::qr_code::{Catalog, Style, StyleFn};

use super::Theme;

/// The style class of a QR code.
pub enum Class<'a> {
    /// Black cells on white, readable by every scanner in light and dark
    /// themes.
    Default,
    /// Cells in the text color on the surface.
    Themed,
    /// A styling closure.
    Custom(StyleFn<'a, Theme>),
}

impl<'a> From<StyleFn<'a, Theme>> for Class<'a> {
    fn from(style: StyleFn<'a, Theme>) -> Self {
        Class::Custom(style)
    }
}

impl Catalog for Theme {
    type Class<'a> = Class<'a>;

    fn default<'a>() -> Self::Class<'a> {
        Class::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let roles = self.roles();

        match class {
            Class::Default => Style {
                cell: iced::Color::BLACK,
                background: iced::Color::WHITE,
            },
            Class::Themed => Style {
                cell: roles.on_surface,
                background: roles.surface,
            },
            Class::Custom(style) => style(self),
        }
    }
}
