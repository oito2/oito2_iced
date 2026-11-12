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

//! Follows the desktop settings in OITO2 Iced apps: the accent color, the
//! light or dark preference, high contrast, the window control style and the
//! window button layout.
//!
//! [`read`] returns the current settings; [`subscription`] yields them once
//! right away and again after every change, so an app keeps them in its
//! model and rebuilds its theme with [`theme`]:
//!
//! ```no_run
//! use iced::{Element, Subscription};
//! use oito2_iced::Theme;
//! use oito2_iced::theme::Brightness;
//! use oito2_iced_settings::SystemSettings;
//!
//! struct App {
//!     settings: SystemSettings,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Settings(SystemSettings),
//! }
//!
//! impl App {
//!     fn subscription(&self) -> Subscription<Message> {
//!         oito2_iced_settings::subscription().map(Message::Settings)
//!     }
//!
//!     fn theme(&self) -> Theme {
//!         oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
//!     }
//! #   fn view(&self) -> Element<'_, Message, Theme> { iced::widget::text("").into() }
//! }
//! ```

#![deny(missing_docs)]

use iced::Subscription;
use iced::futures::channel::mpsc;
use iced::futures::{SinkExt, StreamExt};
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;

pub use oito2_settings::{ColorScheme, Platform, SystemSettings};

/// The current desktop settings of the running platform.
pub fn read() -> SystemSettings {
    oito2_settings::read_system_settings()
}

/// Yields the desktop settings once right away and again after every
/// change, until the subscription is dropped.
///
/// When the platform cannot be watched, only the first reading is yielded.
pub fn subscription() -> Subscription<SystemSettings> {
    Subscription::run(watch)
}

/// The stream behind [`subscription`]: the current settings, then the
/// settings after each change reported by the platform watcher, which lives
/// as long as the stream.
fn watch() -> impl iced::futures::Stream<Item = SystemSettings> {
    iced::stream::channel(8, async |mut output| {
        let _ = output.send(read()).await;

        let (sender, mut changes) = mpsc::unbounded();
        let platform = Platform::current();

        let watcher = oito2_settings::watch(move |raw| {
            let _ = sender.unbounded_send(SystemSettings::compose(raw, platform));
        });

        let Ok(_watcher) = watcher else {
            return;
        };

        while let Some(settings) = changes.next().await {
            if output.send(settings).await.is_err() {
                break;
            }
        }
    })
}

/// The brightness the settings ask for, or `fallback` when the system
/// states no preference.
pub fn brightness(settings: &SystemSettings, fallback: Brightness) -> Brightness {
    match settings.color_scheme {
        Some(ColorScheme::Light) => Brightness::Light,
        Some(ColorScheme::Dark) => Brightness::Dark,
        None => fallback,
    }
}

/// The contrast level the settings ask for.
pub fn contrast(settings: &SystemSettings) -> Contrast {
    if settings.high_contrast {
        Contrast::High
    } else {
        Contrast::Standard
    }
}

/// The accent color of the settings, parsed from `#RRGGBB`, or `None` when
/// the system has none or it is not a valid color.
pub fn accent(settings: &SystemSettings) -> Option<iced::Color> {
    let text = settings.accent_color.as_deref()?.strip_prefix('#')?;

    if text.len() != 6 {
        return None;
    }

    let value = u32::from_str_radix(text, 16).ok()?;
    let [_, r, g, b] = value.to_be_bytes();

    Some(iced::Color::from_rgb8(r, g, b))
}

/// The base theme that follows the settings: `accent`, or the system accent
/// color, or the fallback accent; the system light or dark preference, or
/// `fallback`; the system high-contrast preference; and the control density
/// of the platform.
pub fn theme(
    settings: &SystemSettings,
    accent: Option<iced::Color>,
    fallback: Brightness,
) -> Theme {
    let color = accent
        .or_else(|| self::accent(settings))
        .unwrap_or_else(|| Accent::default().color());

    Theme::base(brightness(settings, fallback), color, contrast(settings))
        .with_density(settings.platform.density())
}

#[cfg(test)]
mod tests {
    use super::*;
    use oito2_settings::foundation::{DesktopEnvironment, OperatingSystem};

    fn settings() -> SystemSettings {
        SystemSettings::fallback(
            Platform::new(OperatingSystem::Linux, false),
            Some(DesktopEnvironment::Gnome),
        )
    }

    #[test]
    fn without_preferences_the_fallbacks_apply() {
        let settings = settings();
        let theme = theme(&settings, None, Brightness::Dark);

        assert_eq!(theme.brightness(), Brightness::Dark);
        assert_eq!(theme.contrast(), Contrast::Standard);
        assert_eq!(
            theme,
            Theme::base(Brightness::Dark, Accent::Blue.color(), Contrast::Standard)
        );
    }

    #[test]
    fn the_system_preferences_drive_the_theme() {
        let settings = SystemSettings {
            accent_color: Some("#2190A4".to_owned()),
            color_scheme: Some(ColorScheme::Light),
            high_contrast: true,
            ..settings()
        };
        let theme = theme(&settings, None, Brightness::Dark);

        assert_eq!(theme.brightness(), Brightness::Light);
        assert_eq!(theme.contrast(), Contrast::High);
        assert_eq!(
            theme.roles(),
            Theme::base(Brightness::Light, Accent::Teal.color(), Contrast::High).roles()
        );
    }

    #[test]
    fn an_explicit_accent_wins_over_the_system_one() {
        let settings = SystemSettings {
            accent_color: Some("#2190A4".to_owned()),
            ..settings()
        };
        let theme = theme(&settings, Some(Accent::Purple.color()), Brightness::Light);

        assert_eq!(
            theme.roles(),
            Theme::base(
                Brightness::Light,
                Accent::Purple.color(),
                Contrast::Standard
            )
            .roles()
        );
    }

    #[test]
    fn invalid_accents_are_ignored() {
        for text in ["", "2190A4", "#2190A", "#XYZXYZ"] {
            let settings = SystemSettings {
                accent_color: Some(text.to_owned()),
                ..settings()
            };

            assert_eq!(accent(&settings), None, "{text}");
        }
    }
}
