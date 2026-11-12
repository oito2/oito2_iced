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

//! Classes of the drop-down menu used by pick lists and combo boxes.

use iced::overlay::menu::{Catalog, Style, StyleFn};
use iced::{Background, Border};

use super::container::elevation;
use super::{HOVER_OPACITY, Theme, state_layer};
use crate::tokens::radius;

/// The style class of a drop-down menu.
pub enum Class<'a> {
    /// The surface container with an outline and a shadow; the selected
    /// option takes a hover layer.
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

    fn default<'a>() -> <Self as Catalog>::Class<'a> {
        Class::Default
    }

    fn style(&self, class: &<Self as Catalog>::Class<'_>) -> Style {
        if let Class::Custom(style) = class {
            return style(self);
        }

        let roles = self.roles();

        Style {
            background: Background::Color(roles.surface_container),
            border: Border::default()
                .color(roles.outline_variant)
                .width(1.0)
                .rounded(radius::MEDIUM),
            text_color: roles.on_surface,
            selected_text_color: roles.on_surface,
            selected_background: Background::Color(state_layer(
                roles.surface_container,
                roles.on_surface,
                HOVER_OPACITY,
            )),
            shadow: elevation(roles.shadow, 4.0),
        }
    }
}
