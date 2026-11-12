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

//! Text classes.

use iced::widget::text::{Catalog, Style, StyleFn};

use super::Theme;

/// The style class of a text.
pub enum Class<'a> {
    /// Inherits the color of the parent.
    Default,
    /// Secondary text.
    Variant,
    /// Primary color as text.
    Primary,
    /// Error text.
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
        Class::Default
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let roles = self.roles();

        let color = match class {
            Class::Default => None,
            Class::Variant => Some(roles.on_surface_variant),
            Class::Primary => Some(roles.semantic.primary_text),
            Class::Error => Some(roles.semantic.error_text),
            Class::Custom(style) => return style(self),
        };

        Style { color }
    }
}
