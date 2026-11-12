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

//! Scrollable classes.

use iced::widget::container;
use iced::widget::scrollable::{AutoScroll, Catalog, Rail, Scroller, Status, Style, StyleFn};
use iced::{Background, Border, Shadow, Vector};

use super::Theme;
use crate::tokens::radius;

/// Opacity of a scroller at rest.
const SCROLLER_OPACITY: f32 = 0.3;
/// Opacity of a hovered scroller.
const SCROLLER_HOVERED_OPACITY: f32 = 0.45;
/// Opacity of a dragged scroller.
const SCROLLER_DRAGGED_OPACITY: f32 = 0.6;

/// The style class of a scrollable.
pub enum Class<'a> {
    /// Rounded, semi-transparent scrollers that darken on hover and drag.
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

        let (horizontal, vertical) = match status {
            Status::Active { .. } => (SCROLLER_OPACITY, SCROLLER_OPACITY),
            Status::Hovered {
                is_horizontal_scrollbar_hovered,
                is_vertical_scrollbar_hovered,
                ..
            } => (
                hovered(is_horizontal_scrollbar_hovered),
                hovered(is_vertical_scrollbar_hovered),
            ),
            Status::Dragged {
                is_horizontal_scrollbar_dragged,
                is_vertical_scrollbar_dragged,
                ..
            } => (
                dragged(is_horizontal_scrollbar_dragged),
                dragged(is_vertical_scrollbar_dragged),
            ),
        };

        let rail = |opacity: f32| Rail {
            background: None,
            border: Border::default(),
            scroller: Scroller {
                background: Background::Color(roles.on_surface.scale_alpha(opacity)),
                border: Border::default().rounded(radius::CHECK),
            },
        };

        Style {
            container: container::Style::default(),
            vertical_rail: rail(vertical),
            horizontal_rail: rail(horizontal),
            gap: None,
            auto_scroll: AutoScroll {
                background: Background::Color(roles.surface_container),
                border: Border::default()
                    .color(roles.outline_variant)
                    .width(1.0)
                    .rounded(f32::MAX),
                shadow: Shadow {
                    color: roles.shadow.scale_alpha(0.2),
                    offset: Vector::new(0.0, 1.0),
                    blur_radius: 4.0,
                },
                icon: roles.on_surface,
            },
        }
    }
}

/// The scroller opacity for a hovered scrollable.
fn hovered(is_hovered: bool) -> f32 {
    if is_hovered {
        SCROLLER_HOVERED_OPACITY
    } else {
        SCROLLER_OPACITY
    }
}

/// The scroller opacity for a dragged scrollable.
fn dragged(is_dragged: bool) -> f32 {
    if is_dragged {
        SCROLLER_DRAGGED_OPACITY
    } else {
        SCROLLER_OPACITY
    }
}
