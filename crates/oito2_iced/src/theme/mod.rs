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

//! The OITO2 [`Theme`]: resolved color roles plus the style catalogs of the
//! native Iced widgets.
//!
//! Every catalog exposes its classes as an enum with the OITO2 variants and a
//! `Custom` variant that holds a styling closure, so the `style` builder of
//! each widget keeps working.

pub mod button;
pub mod checkbox;
pub mod container;
pub mod contrast;
mod roles;
pub mod text;
pub mod text_input;

use std::sync::Arc;

use iced::theme::{Base, Mode, Palette, Style};
use iced::{Color, color};

use crate::tokens::Accent;

pub use roles::{ChromeColors, ColorRoles, SemanticColors, tonal_spot};

/// Hover state layer opacity.
pub const HOVER_OPACITY: f32 = 0.08;
/// Pressed state layer opacity.
pub const PRESSED_OPACITY: f32 = 0.10;
/// Opacity of the background of a disabled control.
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
/// Opacity of the content of a disabled control.
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;

/// Light or dark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Brightness {
    /// Dark text on light surfaces.
    #[default]
    Light,
    /// Light text on dark surfaces.
    Dark,
}

/// The OITO2 theme. Cheap to clone: the resolved roles are shared.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    inner: Arc<Inner>,
}

#[derive(Debug, PartialEq)]
struct Inner {
    name: String,
    brightness: Brightness,
    accent: Accent,
    roles: ColorRoles,
}

impl Theme {
    /// The base theme for a brightness and an accent.
    pub fn new(brightness: Brightness, accent: Accent) -> Self {
        let mode = match brightness {
            Brightness::Light => "light",
            Brightness::Dark => "dark",
        };

        Self {
            inner: Arc::new(Inner {
                name: format!("oito2 {} {mode}", accent.name()),
                brightness,
                accent,
                roles: ColorRoles::base(brightness, accent),
            }),
        }
    }

    /// The light theme with the fallback accent.
    pub fn light() -> Self {
        Self::new(Brightness::Light, Accent::default())
    }

    /// The dark theme with the fallback accent.
    pub fn dark() -> Self {
        Self::new(Brightness::Dark, Accent::default())
    }

    /// The brightness of the theme.
    pub fn brightness(&self) -> Brightness {
        self.inner.brightness
    }

    /// The accent of the theme.
    pub fn accent(&self) -> Accent {
        self.inner.accent
    }

    /// The resolved color roles.
    pub fn roles(&self) -> &ColorRoles {
        &self.inner.roles
    }

    /// Color of the focus ring drawn around a focused control.
    pub fn focus_ring(&self) -> Color {
        self.inner.roles.semantic.primary_text
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

impl Base for Theme {
    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Dark => Self::dark(),
            Mode::Light | Mode::None => Self::light(),
        }
    }

    fn mode(&self) -> Mode {
        match self.inner.brightness {
            Brightness::Light => Mode::Light,
            Brightness::Dark => Mode::Dark,
        }
    }

    fn base(&self) -> Style {
        Style {
            background_color: self.inner.roles.background,
            text_color: self.inner.roles.on_surface,
        }
    }

    fn palette(&self) -> Option<Palette> {
        let roles = &self.inner.roles;

        Some(Palette {
            background: roles.background,
            text: roles.on_surface,
            primary: roles.primary,
            success: roles.semantic.success,
            warning: roles.semantic.warning,
            danger: roles.error,
        })
    }

    fn name(&self) -> &str {
        &self.inner.name
    }
}

/// Paints `layer` over `base` with the given opacity.
pub fn state_layer(base: Color, layer: Color, opacity: f32) -> Color {
    let alpha = opacity * layer.a;

    Color {
        r: base.r + (layer.r - base.r) * alpha,
        g: base.g + (layer.g - base.g) * alpha,
        b: base.b + (layer.b - base.b) * alpha,
        a: base.a + (1.0 - base.a) * alpha,
    }
}

/// Fully transparent black.
pub const TRANSPARENT: Color = color!(0x000000, 0.0);
