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

//! Checkbox classes.

use iced::widget::checkbox::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

use super::{DISABLED_CONTENT_OPACITY, HOVER_OPACITY, TRANSPARENT, Theme, state_layer};
use crate::tokens::radius;

/// The style class of a checkbox.
pub enum Class<'a> {
    /// Primary fill when checked, outline when unchecked.
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

        let (is_checked, hovered, disabled) = match status {
            Status::Active { is_checked } => (is_checked, false, false),
            Status::Hovered { is_checked } => (is_checked, true, false),
            Status::Disabled { is_checked } => (is_checked, false, true),
        };

        let (background, border_color) = match (is_checked, disabled) {
            (true, false) => (roles.primary, roles.primary),
            (true, true) => {
                let faded = roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY);
                (faded, TRANSPARENT)
            }
            (false, false) => (TRANSPARENT, roles.outline),
            (false, true) => (
                TRANSPARENT,
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
            ),
        };

        let background = if hovered {
            state_layer(background, roles.on_surface, HOVER_OPACITY)
        } else {
            background
        };

        Style {
            background: Background::Color(background),
            icon_color: if disabled {
                roles.surface
            } else {
                roles.on_primary
            },
            border: Border::default()
                .color(border_color)
                .width(if is_checked { 0.0 } else { 2.0 })
                .rounded(radius::CHECK),
            text_color: Some(if disabled {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            } else {
                roles.on_surface
            }),
        }
    }
}
