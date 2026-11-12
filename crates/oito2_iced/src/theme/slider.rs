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

//! Slider classes.

use iced::widget::slider::{Catalog, Handle, HandleShape, Rail, Status, Style, StyleFn};
use iced::{Background, Border};

use super::{PRESSED_OPACITY, TRANSPARENT, Theme, state_layer};

/// The style class of a slider.
pub enum Class<'a> {
    /// Indicator track and handle over the highest surface container.
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
        let indicator = roles.indicator();

        let (handle_border_width, handle_border_color) = match status {
            Status::Active => (0.0, TRANSPARENT),
            Status::Hovered => (4.0, indicator.scale_alpha(0.12)),
            Status::Dragged => (
                6.0,
                state_layer(TRANSPARENT, indicator, PRESSED_OPACITY * 2.0),
            ),
        };

        Style {
            rail: Rail {
                backgrounds: (
                    Background::Color(indicator),
                    Background::Color(roles.surface_container_highest),
                ),
                width: 4.0,
                border: Border::default().rounded(2.0),
            },
            handle: Handle {
                shape: HandleShape::Circle { radius: 10.0 },
                background: Background::Color(indicator),
                border_width: handle_border_width,
                border_color: handle_border_color,
            },
        }
    }
}
