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

//! Table classes.

use iced::Background;
use iced::widget::table::{Catalog, Style, StyleFn};

use super::Theme;

/// The style class of a table.
pub enum Class<'a> {
    /// Separators in the divider color.
    Default,
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
        if let Class::Custom(style) = class {
            return style(self);
        }

        let separator = Background::Color(self.roles().outline_variant);

        Style {
            separator_x: separator,
            separator_y: separator,
        }
    }
}
