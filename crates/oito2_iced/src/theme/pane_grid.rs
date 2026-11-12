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

//! Pane grid classes.

use iced::widget::pane_grid::{Catalog, Highlight, Line, Style, StyleFn};
use iced::{Background, Border};

use super::Theme;
use crate::tokens::{radius, sizes};

/// The style class of a pane grid.
pub enum Class<'a> {
    /// The indicator color for the drop region and the dragged split; the
    /// control border color for a hovered split.
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
        let indicator = roles.indicator();

        Style {
            hovered_region: Highlight {
                background: Background::Color(indicator.scale_alpha(0.12)),
                border: Border::default()
                    .color(indicator)
                    .width(sizes::FOCUS_BORDER_WIDTH)
                    .rounded(radius::MEDIUM),
            },
            picked_split: Line {
                color: indicator,
                width: sizes::FOCUS_BORDER_WIDTH,
            },
            hovered_split: Line {
                color: roles.outline,
                width: sizes::FOCUS_BORDER_WIDTH,
            },
        }
    }
}
