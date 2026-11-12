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

//! Vector image (SVG) classes.

use iced::widget::svg::{Catalog, Status, Style, StyleFn};

use super::Theme;

/// The style class of a vector image.
pub enum Class<'a> {
    /// The original colors of the image.
    Original,
    /// Recolored with the text color: monochrome icons.
    Icon,
    /// Recolored with the secondary text color.
    Variant,
    /// Recolored with the indicator color.
    Primary,
    /// Recolored with the error text color.
    Error,
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
        Class::Original
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let roles = self.roles();

        let color = match class {
            Class::Original => None,
            Class::Icon => Some(roles.on_surface),
            Class::Variant => Some(roles.on_surface_variant),
            Class::Primary => Some(roles.indicator()),
            Class::Error => Some(roles.semantic.error_text),
            Class::Custom(style) => return style(self, status),
        };

        Style { color }
    }
}
