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

//! Pick list classes.

use iced::widget::pick_list::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

use super::{HOVER_OPACITY, Theme, state_layer};
use crate::tokens::{radius, sizes};

/// The style class of a pick list.
pub enum Class<'a> {
    /// Filled field with an outline that turns into the focus color while
    /// the menu is open.
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

    fn style(&self, class: &<Self as Catalog>::Class<'_>, status: Status) -> Style {
        if let Class::Custom(style) = class {
            return style(self, status);
        }

        let roles = self.roles();

        let (border, hovered) = match status {
            Status::Active => (Border::default().color(roles.outline).width(1.0), false),
            Status::Hovered => (Border::default().color(roles.outline).width(1.0), true),
            Status::Opened { is_hovered } => (
                Border::default()
                    .color(roles.indicator())
                    .width(sizes::FOCUS_BORDER_WIDTH),
                is_hovered,
            ),
        };

        let background = if hovered {
            state_layer(
                roles.surface_container_high,
                roles.on_surface,
                HOVER_OPACITY,
            )
        } else {
            roles.surface_container_high
        };

        Style {
            text_color: roles.on_surface,
            placeholder_color: roles.on_surface_variant,
            handle_color: roles.on_surface_variant,
            background: Background::Color(background),
            border: border.rounded(radius::MEDIUM),
        }
    }
}
