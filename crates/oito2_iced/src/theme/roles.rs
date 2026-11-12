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

//! The resolved color roles of a theme: Material 3 roles, window chrome and
//! semantic text colors.

use iced::Color;
use material_colors::color::Rgb;
use material_colors::hct::Hct;
use material_colors::scheme::Scheme;
use material_colors::scheme::variant::SchemeTonalSpot;

use super::Brightness;
use super::contrast::{self, NON_TEXT_AA, TEXT_AA};
use crate::tokens::{Accent, Neutrals, Status};

/// Every color role a theme resolves.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRoles {
    /// Window background.
    pub background: Color,
    /// Primary color: filled buttons, selection, focus.
    pub primary: Color,
    /// Text and icons on `primary`.
    pub on_primary: Color,
    /// Primary container.
    pub primary_container: Color,
    /// Text and icons on `primary_container`.
    pub on_primary_container: Color,
    /// Secondary color.
    pub secondary: Color,
    /// Text and icons on `secondary`.
    pub on_secondary: Color,
    /// Secondary container: tonal buttons.
    pub secondary_container: Color,
    /// Text and icons on `secondary_container`.
    pub on_secondary_container: Color,
    /// Tertiary color.
    pub tertiary: Color,
    /// Text and icons on `tertiary`.
    pub on_tertiary: Color,
    /// Tertiary container.
    pub tertiary_container: Color,
    /// Text and icons on `tertiary_container`.
    pub on_tertiary_container: Color,
    /// Error color.
    pub error: Color,
    /// Text and icons on `error`.
    pub on_error: Color,
    /// Error container.
    pub error_container: Color,
    /// Text and icons on `error_container`.
    pub on_error_container: Color,
    /// Surface of cards and content.
    pub surface: Color,
    /// Text and icons on surfaces.
    pub on_surface: Color,
    /// Secondary text and icons on surfaces.
    pub on_surface_variant: Color,
    /// Lowest surface container.
    pub surface_container_lowest: Color,
    /// Low surface container.
    pub surface_container_low: Color,
    /// Surface container.
    pub surface_container: Color,
    /// High surface container.
    pub surface_container_high: Color,
    /// Highest surface container.
    pub surface_container_highest: Color,
    /// Borders of controls.
    pub outline: Color,
    /// Dividers and decorative borders.
    pub outline_variant: Color,
    /// Inverse surface: toasts and tooltips.
    pub inverse_surface: Color,
    /// Text on `inverse_surface`.
    pub on_inverse_surface: Color,
    /// Primary color on `inverse_surface`.
    pub inverse_primary: Color,
    /// Window chrome colors.
    pub chrome: ChromeColors,
    /// Semantic text colors.
    pub semantic: SemanticColors,
}

/// Background colors of the window chrome.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChromeColors {
    /// Window background.
    pub background: Color,
    /// Sidebar.
    pub sidebar: Color,
    /// Main bar.
    pub mainbar: Color,
    /// Title bar.
    pub title_bar: Color,
    /// Dialogs.
    pub dialog: Color,
}

/// Text colors that reach the contrast target on every surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticColors {
    /// Primary color used as text.
    pub primary_text: Color,
    /// Error color used as text.
    pub error_text: Color,
    /// Success.
    pub success: Color,
    /// Warning.
    pub warning: Color,
    /// Information.
    pub info: Color,
    /// Links.
    pub link: Color,
}

impl ColorRoles {
    /// Resolves the roles of the base theme for a brightness and an accent at
    /// standard contrast: GTK neutrals for the surfaces, the accent raised to
    /// AA against white as the primary color, and the remaining Material
    /// roles from a tonal-spot scheme seeded by that primary color.
    pub fn base(brightness: Brightness, accent: Accent) -> Self {
        let is_dark = brightness == Brightness::Dark;
        let neutrals = if is_dark {
            Neutrals::DARK
        } else {
            Neutrals::LIGHT
        };
        let status = if is_dark { Status::DARK } else { Status::LIGHT };

        let primary = contrast::ensure(accent.color(), &[Color::WHITE], TEXT_AA);
        let seed = tonal_spot(primary, is_dark);

        let chrome = ChromeColors {
            background: neutrals.surface_container_low,
            sidebar: neutrals.surface_container,
            mainbar: neutrals.surface_container_highest,
            title_bar: neutrals.title_bar,
            dialog: if is_dark {
                neutrals.surface_container_high
            } else {
                neutrals.surface
            },
        };

        let surfaces = [
            chrome.background,
            neutrals.surface,
            neutrals.surface_container_lowest,
            neutrals.surface_container_low,
            neutrals.surface_container,
            neutrals.surface_container_high,
            neutrals.surface_container_highest,
            chrome.title_bar,
            chrome.sidebar,
            chrome.mainbar,
            chrome.dialog,
        ];
        let text = |color| contrast::ensure(color, &surfaces, TEXT_AA);

        let on_surface = text(neutrals.on_surface);
        let primary_text = text(primary);

        Self {
            background: chrome.background,
            primary,
            on_primary: Color::WHITE,
            primary_container: color(seed.primary_container),
            on_primary_container: color(seed.on_primary_container),
            secondary: color(seed.secondary),
            on_secondary: color(seed.on_secondary),
            secondary_container: color(seed.secondary_container),
            on_secondary_container: color(seed.on_secondary_container),
            tertiary: color(seed.tertiary),
            on_tertiary: color(seed.on_tertiary),
            tertiary_container: color(seed.tertiary_container),
            on_tertiary_container: color(seed.on_tertiary_container),
            error: status.error,
            on_error: contrast::foreground_for(status.error),
            error_container: color(seed.error_container),
            on_error_container: color(seed.on_error_container),
            surface: neutrals.surface,
            on_surface,
            on_surface_variant: text(neutrals.on_surface_variant),
            surface_container_lowest: neutrals.surface_container_lowest,
            surface_container_low: neutrals.surface_container_low,
            surface_container: neutrals.surface_container,
            surface_container_high: neutrals.surface_container_high,
            surface_container_highest: neutrals.surface_container_highest,
            outline: contrast::ensure(neutrals.outline, &surfaces, NON_TEXT_AA),
            outline_variant: neutrals.outline_variant,
            inverse_surface: on_surface,
            on_inverse_surface: neutrals.surface,
            inverse_primary: contrast::ensure(primary, &[on_surface], TEXT_AA),
            chrome,
            semantic: SemanticColors {
                primary_text,
                error_text: text(status.error),
                success: text(status.success),
                warning: text(status.warning),
                info: text(status.info),
                link: primary_text,
            },
        }
    }
}

/// The Material 3 tonal-spot scheme of a seed color, at standard contrast.
pub fn tonal_spot(seed: Color, is_dark: bool) -> Scheme {
    let [r, g, b, _] = seed.into_rgba8();
    let hct = Hct::new(Rgb::new(r, g, b));

    Scheme::from(SchemeTonalSpot::new(hct, is_dark, Some(0.0)).scheme)
}

/// Converts a material-colors color into an Iced color.
fn color(rgb: Rgb) -> Color {
    Color::from_rgb8(rgb.red, rgb.green, rgb.blue)
}
