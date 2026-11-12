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

//! Switch (toggler) classes.

use iced::Background;
use iced::widget::toggler::{Catalog, Status, Style, StyleFn};

use super::{
    DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, TRANSPARENT, Theme,
    state_layer,
};

/// The style class of a switch.
pub enum Class<'a> {
    /// Indicator track with an on-indicator thumb when on; outlined track
    /// with an outline thumb when off.
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

        let (is_toggled, hovered, disabled) = match status {
            Status::Active { is_toggled } => (is_toggled, false, false),
            Status::Hovered { is_toggled } => (is_toggled, true, false),
            Status::Disabled { is_toggled } => (is_toggled, false, true),
        };

        let (track, thumb, track_border) = match (is_toggled, disabled) {
            (true, false) => (roles.indicator(), roles.on_indicator(), TRANSPARENT),
            (false, false) => (
                roles.surface_container_highest,
                roles.outline,
                roles.outline,
            ),
            (true, true) => (
                roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY),
                roles.surface,
                TRANSPARENT,
            ),
            (false, true) => (
                roles
                    .surface_container_highest
                    .scale_alpha(DISABLED_CONTAINER_OPACITY),
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
                roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY),
            ),
        };

        let track = if hovered {
            state_layer(track, roles.on_surface, HOVER_OPACITY)
        } else {
            track
        };

        Style {
            background: Background::Color(track),
            background_border_width: if track_border == TRANSPARENT {
                0.0
            } else {
                2.0
            },
            background_border_color: track_border,
            foreground: Background::Color(thumb),
            foreground_border_width: 0.0,
            foreground_border_color: TRANSPARENT,
            text_color: Some(if disabled {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            } else {
                roles.on_surface
            }),
            border_radius: None,
            padding_ratio: 0.15,
        }
    }
}
