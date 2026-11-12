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
    /// Page background.
    Background,
    /// Title bar of the window.
    TitleBar,
    /// Narrow sidebar column, such as an icon rail.
    Sidebar,
    /// Main bar: the pane beside a sidebar.
    Mainbar,
    /// Card on the page background: surface with an outline.
    Card,
    /// Dialogs, sheets and drawers.
    Dialog,
    /// Anchored panels: popovers and hover cards.
    Popover,
    /// Menus: the surface container with an outline and a shadow.
    Menu,
    /// Tooltips: the inverse surface with the small radius.
    Tooltip,
    /// The inverse surface with the medium radius: toasts and snack bars.
    Inverse,
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
        let outlined = |radius: f32| {
            Border::default()
                .color(roles.outline_variant)
                .width(1.0)
                .rounded(radius)
        };

        match class {
            Class::Transparent => Style::default(),
            Class::Background => filled(roles.chrome.background, roles.on_surface),
            Class::TitleBar => filled(roles.chrome.title_bar, roles.on_surface),
            Class::Sidebar => filled(roles.chrome.sidebar, roles.on_surface),
            Class::Mainbar => filled(roles.chrome.mainbar, roles.on_surface),
            Class::Card => Style {
                border: outlined(radius::LARGE),
                ..filled(roles.surface, roles.on_surface)
            },
            Class::Dialog => Style {
                border: outlined(radius::WINDOW),
                shadow: elevation(roles.shadow, 6.0),
                ..filled(roles.chrome.dialog, roles.on_surface)
            },
            Class::Popover => Style {
                border: outlined(radius::LARGE),
                shadow: elevation(roles.shadow, 4.0),
                ..filled(roles.surface_container, roles.on_surface)
            },
            Class::Menu => Style {
                border: outlined(radius::MEDIUM),
                shadow: elevation(roles.shadow, 4.0),
                ..filled(roles.surface_container, roles.on_surface)
            },
            Class::Tooltip => Style {
                border: Border::default().rounded(radius::SMALL),
                ..filled(roles.inverse_surface, roles.on_inverse_surface)
            },
            Class::Inverse => Style {
                border: Border::default().rounded(radius::MEDIUM),
                ..filled(roles.inverse_surface, roles.on_inverse_surface)
            },
            Class::Custom(style) => style(self),
        }
    }
}

/// A background with its text color.
fn filled(background: Color, text: Color) -> Style {
    Style {
        background: Some(Background::Color(background)),
        text_color: Some(text),
        ..Style::default()
    }
}

/// The shadow of a surface raised to `level`: the offset grows with half
/// the level and the blur with twice the level.
pub fn elevation(shadow: Color, level: f32) -> Shadow {
    Shadow {
        color: shadow.scale_alpha(0.2),
        offset: Vector::new(0.0, level / 2.0),
        blur_radius: level * 2.0,
    }
}
