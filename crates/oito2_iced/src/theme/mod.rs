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
pub mod float;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod menu;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
#[cfg(feature = "qr_code")]
pub mod qr_code;
pub mod radio;
mod roles;
pub mod rule;
pub mod scrollable;
pub mod slider;
#[cfg(feature = "svg")]
pub mod svg;
pub mod table;
pub mod text;
pub mod text_editor;
pub mod text_input;
pub mod toggler;

use std::sync::Arc;

use iced::theme::{self as iced_theme, Base, Mode, Style};
use iced::{Color, color};

use oito2_foundation::Density;

use crate::tokens::{Accent, sizes};

pub use roles::{
    ChromeColors, ColorRoles, FamilyColors, Palette, SemanticColors, SyntaxColors, tonal_spot,
};

/// Hover state layer opacity.
pub const HOVER_OPACITY: f32 = 0.08;
/// Pressed state layer opacity.
pub const PRESSED_OPACITY: f32 = 0.10;
/// Opacity of the background of a disabled control.
pub const DISABLED_CONTAINER_OPACITY: f32 = 0.12;
/// Opacity of the content of a disabled control.
pub const DISABLED_CONTENT_OPACITY: f32 = 0.38;
/// Opacity of the primary color behind selected text.
pub const SELECTION_OPACITY: f32 = 0.35;

/// Light or dark.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Brightness {
    /// Dark text on light surfaces.
    #[default]
    Light,
    /// Light text on dark surfaces.
    Dark,
}

/// The contrast level of a theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Contrast {
    /// Text reaches 4.5:1 and control borders 3:1 against every surface.
    #[default]
    Standard,
    /// Text reaches 7:1 and control borders 4.5:1 against every surface;
    /// dividers take the control border color.
    High,
}

/// The OITO2 theme. Cheap to clone: the resolved roles are shared.
#[derive(Debug, Clone, PartialEq)]
pub struct Theme {
    inner: Arc<Inner>,
}

#[derive(Debug, PartialEq)]
struct Inner {
    name: String,
    roles: ColorRoles,
    density: Density,
}

impl Theme {
    /// The base theme for a brightness and an accent, at standard contrast.
    pub fn new(brightness: Brightness, accent: Accent) -> Self {
        Self::base(brightness, accent.color(), Contrast::Standard).with_name(base_name(
            accent.name(),
            brightness,
            Contrast::Standard,
        ))
    }

    /// The base theme for a brightness, any accent color and a contrast
    /// level, such as the accent color of the desktop.
    pub fn base(brightness: Brightness, accent: Color, contrast: Contrast) -> Self {
        let [r, g, b, _] = accent.into_rgba8();
        let name = base_name(&format!("#{r:02X}{g:02X}{b:02X}"), brightness, contrast);

        Self::from_roles(name, ColorRoles::base(brightness, accent, contrast))
    }

    /// The theme of a palette, such as a preset color theme, resolved at a
    /// contrast level.
    pub fn from_palette(name: impl Into<String>, palette: &Palette, contrast: Contrast) -> Self {
        Self::from_roles(name, ColorRoles::resolve(palette, contrast))
    }

    /// The theme of already resolved roles.
    pub fn from_roles(name: impl Into<String>, roles: ColorRoles) -> Self {
        Self {
            inner: Arc::new(Inner {
                name: name.into(),
                roles,
                density: Density::Pointer,
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

    /// Returns the theme with another name.
    pub fn with_name(self, name: impl Into<String>) -> Self {
        let inner = &self.inner;

        Self {
            inner: Arc::new(Inner {
                name: name.into(),
                roles: inner.roles,
                density: inner.density,
            }),
        }
    }

    /// Returns the theme with another density: pointer or touch sizes for
    /// controls and icons.
    pub fn with_density(self, density: Density) -> Self {
        let inner = &self.inner;

        Self {
            inner: Arc::new(Inner {
                name: inner.name.clone(),
                roles: inner.roles,
                density,
            }),
        }
    }

    /// The brightness of the theme.
    pub fn brightness(&self) -> Brightness {
        self.inner.roles.brightness
    }

    /// The contrast level of the theme.
    pub fn contrast(&self) -> Contrast {
        if self.inner.roles.high_contrast {
            Contrast::High
        } else {
            Contrast::Standard
        }
    }

    /// The density of the theme.
    pub fn density(&self) -> Density {
        self.inner.density
    }

    /// The resolved color roles.
    pub fn roles(&self) -> &ColorRoles {
        &self.inner.roles
    }

    /// Minimum height of buttons and single-line fields.
    pub fn control_height(&self) -> f32 {
        match self.inner.density {
            Density::Pointer => sizes::BUTTON_HEIGHT,
            Density::Touch => sizes::TOUCH_BUTTON_HEIGHT,
        }
    }

    /// Size of icons inside controls.
    pub fn icon_size(&self) -> f32 {
        match self.inner.density {
            Density::Pointer => sizes::ICON_SIZE,
            Density::Touch => 24.0,
        }
    }

    /// Color of the focus ring drawn around a focused control.
    pub fn focus_ring(&self) -> Color {
        self.inner.roles.indicator()
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light()
    }
}

/// The name of a base theme: "oito2 <accent> <light|dark>", with " high
/// contrast" appended at the high contrast level.
fn base_name(accent: &str, brightness: Brightness, contrast: Contrast) -> String {
    let mode = match brightness {
        Brightness::Light => "light",
        Brightness::Dark => "dark",
    };
    let contrast = match contrast {
        Contrast::Standard => "",
        Contrast::High => " high contrast",
    };

    format!("oito2 {accent} {mode}{contrast}")
}

impl Base for Theme {
    fn default(preference: Mode) -> Self {
        match preference {
            Mode::Dark => Self::dark(),
            Mode::Light | Mode::None => Self::light(),
        }
    }

    fn mode(&self) -> Mode {
        match self.inner.roles.brightness {
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

    fn palette(&self) -> Option<iced_theme::Palette> {
        let roles = &self.inner.roles;

        Some(iced_theme::Palette {
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
