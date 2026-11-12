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

//! Radio button classes.

use iced::Background;
use iced::widget::radio::{Catalog, Status, Style, StyleFn};

use super::{HOVER_OPACITY, TRANSPARENT, Theme, state_layer};

/// The style class of a radio button.
pub enum Class<'a> {
    /// Indicator dot and ring when selected, outline ring otherwise.
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

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        if let Class::Custom(style) = class {
            return style(self, status);
        }

        let roles = self.roles();

        let (is_selected, hovered) = match status {
            Status::Active { is_selected } => (is_selected, false),
            Status::Hovered { is_selected } => (is_selected, true),
        };

        let ring = if is_selected {
            roles.indicator()
        } else {
            roles.outline
        };

        let background = if hovered {
            state_layer(TRANSPARENT, roles.on_surface, HOVER_OPACITY)
        } else {
            TRANSPARENT
        };

        Style {
            background: Background::Color(background),
            dot_color: roles.indicator(),
            border_width: 2.0,
            border_color: ring,
            text_color: Some(roles.on_surface),
        }
    }
}
