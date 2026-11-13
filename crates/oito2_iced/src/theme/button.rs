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

//! Button classes.

use iced::widget::button::{Catalog, Status, Style, StyleFn};
use iced::{Background, Border, Color, Shadow, Vector};

use super::container::elevation;
use super::{
    DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY,
    TRANSPARENT, Theme, state_layer,
};
use crate::tokens::radius;

/// The style class of a button.
pub enum Class<'a> {
    /// Primary background: the main action of a view.
    Filled,
    /// Secondary container background: secondary actions.
    Tonal,
    /// Raised neutral surface that gains a shadow on hover: the neutral
    /// desktop button.
    Elevated,
    /// Transparent with an outline.
    Outlined,
    /// Transparent, primary text: low-emphasis actions.
    Text,
    /// Error background: destructive actions.
    Danger,
    /// Transparent, surface text, fully rounded: window controls and title
    /// bar buttons.
    Chrome,
    /// Primary container background, large radius and a low shadow: the
    /// floating action button.
    Fab,
    /// Transparent with the variant content color: icon buttons.
    Icon,
    /// Secondary container background: a selected toggle icon button.
    IconSelected,
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
        Class::Filled
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        let roles = self.roles();

        let (background, foreground, border_color) = match class {
            Class::Filled => (roles.primary, roles.on_primary, None),
            Class::Tonal => (
                roles.secondary_container,
                roles.on_secondary_container,
                None,
            ),
            Class::Elevated => (roles.surface_container_high, roles.on_surface, None),
            Class::Outlined => (
                TRANSPARENT,
                roles.semantic.primary_text,
                Some(roles.outline),
            ),
            Class::Text => (TRANSPARENT, roles.semantic.primary_text, None),
            Class::Danger => (roles.error, roles.on_error, None),
            Class::Chrome => (TRANSPARENT, roles.on_surface, None),
            Class::Fab => (roles.primary_container, roles.on_primary_container, None),
            Class::Icon => (TRANSPARENT, roles.on_surface_variant, None),
            Class::IconSelected => (
                roles.secondary_container,
                roles.on_secondary_container,
                None,
            ),
            Class::Custom(style) => return style(self, status),
        };

        let radius = match class {
            Class::Chrome => f32::MAX,
            Class::Fab => radius::LARGE,
            _ => radius::MEDIUM,
        };

        let disabled = status == Status::Disabled;

        let background = match status {
            Status::Active => background,
            Status::Hovered => state_layer(background, foreground, HOVER_OPACITY),
            Status::Pressed => state_layer(background, foreground, PRESSED_OPACITY),
            Status::Disabled if background == TRANSPARENT => TRANSPARENT,
            Status::Disabled => roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY),
        };

        let text_color = if disabled {
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
        } else {
            foreground
        };

        let border = match border_color {
            Some(_) if disabled => border(
                roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY),
                radius,
            ),
            Some(color) => border(color, radius),
            None => Border::default().rounded(radius),
        };

        let shadow = match (class, status) {
            (Class::Elevated, Status::Hovered) => Shadow {
                color: roles.shadow.scale_alpha(0.2),
                offset: Vector::new(0.0, 1.0),
                blur_radius: 3.0,
            },
            (Class::Fab, Status::Hovered) => elevation(roles.shadow, 3.0),
            (Class::Fab, Status::Active | Status::Pressed) => elevation(roles.shadow, 2.0),
            _ => Shadow::default(),
        };

        Style {
            background: (background != TRANSPARENT).then_some(Background::Color(background)),
            text_color,
            border,
            shadow,
            ..Style::default()
        }
    }
}

fn border(color: Color, radius: f32) -> Border {
    Border::default().color(color).width(1.0).rounded(radius)
}
