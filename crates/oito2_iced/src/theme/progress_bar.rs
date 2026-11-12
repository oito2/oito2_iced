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

//! Progress bar classes.

use iced::widget::progress_bar::{Catalog, Style, StyleFn};
use iced::{Background, Border, Color};

use super::Theme;

/// The style class of a progress bar.
pub enum Class<'a> {
    /// The selected fill over the highest surface container.
    Default,
    /// Success color over the highest surface container.
    Success,
    /// Warning color over the highest surface container.
    Warning,
    /// Error color over the highest surface container.
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

        let bar: Color = match class {
            Class::Default => roles.selected_fill(),
            Class::Success => roles.semantic.success,
            Class::Warning => roles.semantic.warning,
            Class::Error => roles.semantic.error_text,
            Class::Custom(style) => return style(self),
        };

        Style {
            background: Background::Color(roles.surface_container_highest),
            bar: Background::Color(bar),
            border: Border::default().rounded(2.0),
        }
    }
}
