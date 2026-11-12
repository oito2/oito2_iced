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

//! The appearance choices of the showcase settings dialog and the theme and
//! window chrome they resolve to.

use std::fmt;

use oito2_iced::Theme;
use oito2_iced::foundation::{Density, WindowButtonLayout, WindowControlStyle};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::Accent;
use oito2_iced_settings::{ColorScheme, SystemSettings};
use oito2_iced_themes::Preset;

/// Light, dark or the system preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mode {
    /// Follows the system.
    #[default]
    System,
    /// Always light.
    Light,
    /// Always dark.
    Dark,
}

impl Mode {
    /// Every mode.
    pub const ALL: [Mode; 3] = [Mode::System, Mode::Light, Mode::Dark];
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Mode::System => "System",
            Mode::Light => "Light",
            Mode::Dark => "Dark",
        })
    }
}

/// The color theme: the OITO2 base theme or a preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Family(pub Option<Preset>);

impl Family {
    /// The base theme and every preset.
    pub fn all() -> Vec<Family> {
        std::iter::once(Family(None))
            .chain(Preset::ALL.into_iter().map(|preset| Family(Some(preset))))
            .collect()
    }
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            None => f.write_str("OITO2"),
            Some(preset) => f.write_str(preset.label()),
        }
    }
}

/// The contrast level: the system preference or a fixed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ContrastChoice {
    /// Follows the system high-contrast preference.
    #[default]
    System,
    /// Standard contrast.
    Standard,
    /// High contrast.
    High,
}

impl ContrastChoice {
    /// Every choice.
    pub const ALL: [ContrastChoice; 3] = [
        ContrastChoice::System,
        ContrastChoice::Standard,
        ContrastChoice::High,
    ];
}

impl fmt::Display for ContrastChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ContrastChoice::System => "System",
            ContrastChoice::Standard => "Standard",
            ContrastChoice::High => "High",
        })
    }
}

/// The control density: the platform one or a fixed one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DensityChoice {
    /// The density of the platform.
    #[default]
    System,
    /// Pointer sizes.
    Pointer,
    /// Touch sizes.
    Touch,
}

impl DensityChoice {
    /// Every choice.
    pub const ALL: [DensityChoice; 3] = [
        DensityChoice::System,
        DensityChoice::Pointer,
        DensityChoice::Touch,
    ];
}

impl fmt::Display for DensityChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            DensityChoice::System => "System",
            DensityChoice::Pointer => "Pointer",
            DensityChoice::Touch => "Touch",
        })
    }
}

/// The language of the interface strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Language {
    /// English.
    #[default]
    English,
    /// Brazilian Portuguese.
    Portuguese,
}

impl Language {
    /// Every language.
    pub const ALL: [Language; 2] = [Language::English, Language::Portuguese];

    /// The strings of the language.
    pub fn localizations(self) -> Localizations {
        match self {
            Language::English => Localizations::en(),
            Language::Portuguese => Localizations::pt_br(),
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Language::English => "English",
            Language::Portuguese => "Português (Brasil)",
        })
    }
}

/// The window chrome: the one of the desktop or a simulated desktop style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Desktop(pub Option<WindowControlStyle>);

impl Desktop {
    /// The system chrome and every control style.
    pub fn all() -> Vec<Desktop> {
        std::iter::once(Desktop(None))
            .chain(
                WindowControlStyle::ALL
                    .into_iter()
                    .map(|style| Desktop(Some(style))),
            )
            .collect()
    }
}

impl fmt::Display for Desktop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            None => "System",
            Some(WindowControlStyle::Gnome) => "GNOME",
            Some(WindowControlStyle::Breeze) => "KDE (Breeze)",
            Some(WindowControlStyle::Xfwm) => "Xfce (Xfwm)",
            Some(WindowControlStyle::Cosmic) => "COSMIC",
            Some(WindowControlStyle::Windows) => "Windows",
            Some(WindowControlStyle::Macos) => "macOS",
            Some(WindowControlStyle::None) => "No buttons",
        })
    }
}

/// Every appearance choice of the settings dialog.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Appearance {
    /// Light, dark or system.
    pub mode: Mode,
    /// The base theme or a preset.
    pub family: Family,
    /// An accent that replaces the system one in the base theme.
    pub accent: Option<Accent>,
    /// The contrast level.
    pub contrast: ContrastChoice,
    /// The control density.
    pub density: DensityChoice,
    /// The interface language.
    pub language: Language,
    /// The window chrome.
    pub desktop: Desktop,
}

impl Appearance {
    /// The theme of these choices under the system `settings`.
    pub fn theme(&self, settings: &SystemSettings) -> Theme {
        let mut settings = settings.clone();

        match self.mode {
            Mode::System => {}
            Mode::Light => settings.color_scheme = Some(ColorScheme::Light),
            Mode::Dark => settings.color_scheme = Some(ColorScheme::Dark),
        }

        match self.contrast {
            ContrastChoice::System => {}
            ContrastChoice::Standard => settings.high_contrast = false,
            ContrastChoice::High => settings.high_contrast = true,
        }

        let density = match self.density {
            DensityChoice::System => settings.platform.density(),
            DensityChoice::Pointer => Density::Pointer,
            DensityChoice::Touch => Density::Touch,
        };

        let theme = match self.family.0 {
            Some(preset) => preset.theme(oito2_iced_settings::contrast(&settings)),
            None => oito2_iced_settings::theme(
                &settings,
                self.accent.map(Accent::color),
                Brightness::Light,
            ),
        };

        theme.with_density(density)
    }

    /// The window control style and button layout of these choices under
    /// the system `settings`.
    pub fn chrome(&self, settings: &SystemSettings) -> (WindowControlStyle, WindowButtonLayout) {
        match self.desktop.0 {
            Some(style) => (style, WindowButtonLayout::default_for(style)),
            None => (
                settings.window_control_style,
                settings.window_button_layout.clone(),
            ),
        }
    }
}
