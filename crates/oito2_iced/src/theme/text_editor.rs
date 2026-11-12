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

//! Multi-line text editor classes.

use iced::widget::text_editor::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border};

use super::{DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, SELECTION_OPACITY, Theme};
use crate::tokens::{radius, sizes};

/// The style class of a text editor.
pub enum Class<'a> {
    /// Filled field with an outline that turns into the focus color.
    Default,
    /// Field with an invalid value: error outline.
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

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let roles = self.roles();

        let (outline, focused_outline) = match class {
            Class::Default => (roles.outline, roles.indicator()),
            Class::Error => (roles.semantic.error_text, roles.semantic.error_text),
            Class::Custom(style) => return style(self, status),
        };

        let border = match status {
            Status::Focused { .. } => Border::default()
                .color(focused_outline)
                .width(sizes::FOCUS_BORDER_WIDTH),
            Status::Disabled => Border::default()
                .color(roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY))
                .width(1.0),
            Status::Active | Status::Hovered => Border::default().color(outline).width(1.0),
        }
        .rounded(radius::MEDIUM);

        Style {
            background: Background::Color(roles.surface_container_high),
            border,
            placeholder: roles.on_surface_variant,
            value: if status == Status::Disabled {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            } else {
                roles.on_surface
            },
            selection: roles.primary.scale_alpha(SELECTION_OPACITY),
        }
    }
}
