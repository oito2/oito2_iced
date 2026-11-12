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

//! Container classes.

use iced::widget::container::{Catalog, Style, StyleFn};
use iced::{Background, Border, Color, Shadow, Vector};

use super::Theme;
use crate::tokens::radius;

/// The style class of a container.
pub enum Class<'a> {
    /// No background: inherits the surface below.
    Transparent,
    /// Window background.
    Background,
    /// Title bar of the window.
    TitleBar,
    /// Sidebar.
    Sidebar,
    /// Card on the window background: surface with an outline.
    Card,
    /// Floating panel: popovers and menus, with a shadow.
    Popover,
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
        Class::Transparent
    }

    fn style(&self, class: &Self::Class<'_>) -> Style {
        let roles = self.roles();

        match class {
            Class::Transparent => Style::default(),
            Class::Background => filled(roles.chrome.background, roles.on_surface),
            Class::TitleBar => filled(roles.chrome.title_bar, roles.on_surface),
            Class::Sidebar => filled(roles.chrome.sidebar, roles.on_surface),
            Class::Card => Style {
                border: Border::default()
                    .color(roles.outline_variant)
                    .width(1.0)
                    .rounded(radius::LARGE),
                ..filled(roles.surface, roles.on_surface)
            },
            Class::Popover => Style {
                border: Border::default()
                    .color(roles.outline_variant)
                    .width(1.0)
                    .rounded(radius::LARGE),
                shadow: Shadow {
                    color: Color::BLACK.scale_alpha(0.2),
                    offset: Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
                ..filled(roles.surface_container, roles.on_surface)
            },
            Class::Custom(style) => style(self),
        }
    }
}

fn filled(background: Color, text: Color) -> Style {
    Style {
        background: Some(Background::Color(background)),
        text_color: Some(text),
        ..Style::default()
    }
}
