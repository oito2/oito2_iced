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

//! The Foundation page: the platform, the desktop environment, the window
//! controls and the system settings the application follows.

use std::sync::LazyLock;

use iced::widget::{column, row};
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::foundation::{
    DesktopEnvironment, OperatingSystem, Platform, SystemSettings, WindowButton, WindowControlStyle,
};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("foundation.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    /// The settings shown: the fallback of the running platform until the
    /// system settings are read.
    settings: SystemSettings,
    /// The rows of the settings table.
    rows: Vec<[String; 2]>,
    read: bool,
}

impl Default for State {
    fn default() -> Self {
        let settings = SystemSettings::fallback(Platform::current(), None);

        Self {
            rows: rows(&settings),
            settings,
            read: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// Reads the system settings.
    Read,
    /// The system settings were read.
    Settings(SystemSettings),
    /// Opens the settings of the showcase.
    OpenSettings,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // #region read
        Message::Read => {
            return Task::perform(async { oito2_iced_settings::read() }, Message::Settings);
        }
        Message::Settings(settings) => {
            state.rows = rows(&settings);
            state.settings = settings;
            state.read = true;
        }
        // #endregion
        Message::OpenSettings => shell.request(Action::OpenSettings),
    }

    Task::none()
}

/// The names of `buttons`, or a dash for none.
fn names(buttons: &[WindowButton]) -> String {
    if buttons.is_empty() {
        "—".to_owned()
    } else {
        buttons
            .iter()
            .map(|button| button.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// The rows of the settings table.
fn rows(settings: &SystemSettings) -> Vec<[String; 2]> {
    let platform = settings.platform;
    let layout = &settings.window_button_layout;

    [
        ("Operating system", platform.os.as_str().to_owned()),
        ("Web", platform.is_web.to_string()),
        ("Native desktop", platform.is_desktop_native().to_string()),
        ("Touch input", platform.uses_touch_input().to_string()),
        ("Density", format!("{:?}", platform.density())),
        (
            "Desktop environment",
            settings
                .desktop_environment
                .map_or_else(|| "—".to_owned(), |de| de.as_str().to_owned()),
        ),
        (
            "Window control style",
            settings.window_control_style.as_str().to_owned(),
        ),
        ("Leading window buttons", names(&layout.leading)),
        ("Trailing window buttons", names(&layout.trailing)),
        (
            "Color scheme",
            settings.color_scheme.map_or_else(
                || "— (no preference)".to_owned(),
                |scheme| scheme.as_str().to_owned(),
            ),
        ),
        ("High contrast", settings.high_contrast.to_string()),
        (
            "Accent color",
            settings
                .accent_color
                .clone()
                .unwrap_or_else(|| "— (none reported)".to_owned()),
        ),
    ]
    .map(|(name, value)| [name.to_owned(), value])
    .to_vec()
}

/// Desktop session values and the environment each is detected as.
static DETECTED: LazyLock<Vec<[String; 3]>> = LazyLock::new(|| {
    // #region detect
    [
        (Some("ubuntu:GNOME"), None),
        (Some("KDE"), None),
        (Some("X-Cinnamon"), None),
        (Some("pop:COSMIC"), None),
        (None, Some("xubuntu")),
        (Some("Hyprland"), None),
    ]
    .map(|(xdg, session)| {
        [
            xdg.unwrap_or("—").to_owned(),
            session.unwrap_or("—").to_owned(),
            DesktopEnvironment::detect(xdg, session).as_str().to_owned(),
        ]
    })
    .to_vec()
    // #endregion
});

/// Platforms and environments with the control style that follows.
static STYLES: LazyLock<Vec<[String; 3]>> = LazyLock::new(|| {
    // #region resolve
    let linux = Platform::new(OperatingSystem::Linux, false);
    [
        (linux, Some(DesktopEnvironment::Gnome)),
        (linux, Some(DesktopEnvironment::Kde)),
        (linux, Some(DesktopEnvironment::Xfce)),
        (linux, Some(DesktopEnvironment::Cosmic)),
        (linux, Some(DesktopEnvironment::Mate)),
        (Platform::new(OperatingSystem::Windows, false), None),
        (Platform::new(OperatingSystem::Macos, false), None),
        (Platform::new(OperatingSystem::Linux, true), None),
    ]
    .map(|(platform, environment)| {
        let style = WindowControlStyle::resolve(platform, environment);
        [
            format!(
                "{}{}",
                platform.os.as_str(),
                if platform.is_web { " (web)" } else { "" }
            ),
            environment.map_or_else(|| "—".to_owned(), |de| de.as_str().to_owned()),
            style.as_str().to_owned(),
        ]
    })
    .to_vec()
    // #endregion
});

/// Rows of strings as the cells of a table.
fn cells<const N: usize>(rows: &[[String; N]]) -> Vec<Vec<&str>> {
    rows.iter()
        .map(|row| row.iter().map(String::as_str).collect())
        .collect()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let source = if state.read {
        "Read from the system."
    } else {
        "The fallback of this platform; read the system settings to see yours."
    };

    let preview = doc::example(
        ctx,
        SOURCE,
        &["read"],
        column![
            doc::table(&["Setting", "Value"], &cells(&state.rows)),
            doc::caption(source),
            row![
                button::outlined("Read the system settings").on_press(Message::Read),
                button::text("Open the showcase settings").on_press(Message::OpenSettings),
            ]
            .spacing(spacing::SM),
        ]
        .spacing(spacing::MD),
    );

    let detected = doc::example(
        ctx,
        SOURCE,
        &["detect"],
        doc::table(
            &["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION", "Detected"],
            &cells(&DETECTED),
        ),
    );

    let styles = doc::example(
        ctx,
        SOURCE,
        &["resolve"],
        doc::table(&["Platform", "Desktop", "Control style"], &cells(&STYLES)),
    );

    doc::page(
        "Foundation",
        "The platform, the desktop environment, the window controls and the system settings \
         an application follows.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`oito2_iced::foundation` is the `oito2_foundation` crate: plain types for the \
                 `Platform` and its `Density`, the `DesktopEnvironment`, the \
                 `WindowControlStyle` and `WindowButtonLayout` of the window chrome, and the \
                 `SystemSettings` that gather them with the accent color, the color scheme and \
                 high contrast. `SystemSettings::fallback` derives them from the platform \
                 alone.",
            ),
            doc::prose(
                "`oito2_iced_settings` reads them from the desktop: `read` returns the current \
                 settings, `subscription` yields them once and again after every change, and \
                 `theme` builds the theme they ask for, with the system accent, light or dark \
                 preference, contrast and density.",
            ),
            doc::code(
                ctx,
                "fn subscription(&self) -> Subscription<Message> {\n    \
                     oito2_iced_settings::subscription().map(Message::Settings)\n\
                 }\n\n\
                 fn theme(&self) -> Theme {\n    \
                     oito2_iced_settings::theme(&self.settings, None, Brightness::Light)\n\
                 }",
            ),
            doc::heading("Examples"),
            doc::subheading("Desktop environment"),
            doc::prose(
                "`DesktopEnvironment::detect` reads the values of `XDG_CURRENT_DESKTOP` and \
                 `DESKTOP_SESSION`: the first recognized entry of the colon-separated list \
                 wins, then the session, ignoring case; anything else is `Unknown`.",
            ),
            detected,
            doc::subheading("Window control style"),
            doc::prose(
                "`WindowControlStyle::resolve` picks the look of the window buttons for a \
                 platform and a desktop: Breeze on KDE, Xfwm on Xfce, COSMIC on COSMIC, GNOME \
                 on the other Linux desktops, Windows and macOS on theirs, and none on the web \
                 and touch platforms. The TitleBar page draws each style and the button \
                 layouts the desktops write.",
            ),
            styles,
            doc::note(
                "The window buttons follow the desktop only on Linux; on the web and touch \
                 platforms the browser or the system owns the window, so the title bar shows \
                 no buttons there.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_settings::read",
                    "oito2_iced_settings::subscription",
                    "oito2_iced_settings::theme",
                    "oito2_iced_settings::brightness",
                    "oito2_iced_settings::contrast",
                    "oito2_iced_settings::accent",
                ],
            ),
        ],
    )
}
