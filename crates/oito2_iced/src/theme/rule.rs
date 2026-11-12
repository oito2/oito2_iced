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

//! Divider (rule) classes.

use iced::widget::rule::{Catalog, FillMode, Style, StyleFn};

use super::Theme;

/// The style class of a divider.
pub enum Class<'a> {
    /// A 1-pixel line in the divider color across the whole length.
    Default,
    /// A line in the control border color: separates controls.
    Strong,
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
            Class::Default => roles.outline_variant,
            Class::Strong => roles.outline,
            Class::Custom(style) => return style(self),
        };

        Style {
            color,
            radius: 0.0.into(),
            fill_mode: FillMode::Full,
            snap: true,
        }
    }
}
