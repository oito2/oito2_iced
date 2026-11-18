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

//! Autostart recipe: a switch in an OITO2 window writes or removes the XDG
//! autostart entry that starts this program at login, in
//! `$XDG_CONFIG_HOME/autostart` or `~/.config/autostart`.
//!
//! XDG autostart is a convention of Linux desktops: on other platforms the
//! switch is disabled and the window says why.

use std::path::PathBuf;

use iced::Task;
use iced::widget::column;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::list_tile::switch_list_tile;
use oito2_iced::widget::{styled_text, toggles};
use oito2_iced::{Element, Theme, font};
use oito2_iced_desktop_example as example;
use oito2_iced_desktop_example::autostart::{self, Entry};

fn main() -> iced::Result {
    let application = iced::application(Autostart::new, Autostart::update, Autostart::view)
        .title("OITO2 Iced autostart")
        .theme(Autostart::theme);

    font::INTER_FONTS
        .into_iter()
        .fold(application, |application, bytes| application.font(bytes))
        .default_font(font::INTER)
        .window(example::window_settings(460.0, 260.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Toggled(bool),
}

/// Where the entry goes, and what the window reports.
struct Autostart {
    /// The autostart directory and the entry of this program, when both are
    /// known.
    target: Result<(PathBuf, Entry), String>,
    enabled: bool,
    status: Option<String>,
}

impl Autostart {
    fn new() -> Self {
        let target = target();
        let enabled = target
            .as_ref()
            .is_ok_and(|(directory, entry)| entry.is_enabled(directory));

        Self {
            target,
            enabled,
            status: None,
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        let Message::Toggled(enable) = message;
        let Ok((directory, entry)) = &self.target else {
            return Task::none();
        };

        // A single small file in the user's configuration: written in place.
        let result = if enable {
            entry.enable(directory).map(|_path| ())
        } else {
            entry.disable(directory)
        };

        match result {
            Ok(()) => {
                self.enabled = enable;
                self.status = None;
            }
            Err(error) => self.status = Some(format!("The entry was not changed: {error}")),
        }

        Task::none()
    }

    fn theme(&self) -> Theme {
        example::theme()
    }

    fn view(&self) -> Element<'_, Message> {
        let (subtitle, available) = match &self.target {
            Ok((directory, entry)) => (entry.path(directory).display().to_string(), true),
            Err(reason) => (reason.clone(), false),
        };

        let switch = switch_list_tile(
            toggles::switch(self.enabled)
                .on_toggle(Message::Toggled)
                .enabled(available),
        )
        .title("Start at login")
        .subtitle(subtitle);

        let mut content = column![switch].spacing(spacing::MD);
        if let Some(status) = &self.status {
            content = content.push(
                styled_text(status.as_str(), typography::BODY_MEDIUM).class(TextClass::Error),
            );
        }

        example::page(
            "Autostart",
            "The desktop session starts the programs listed in its autostart directory at login.",
            content,
        )
    }
}

/// The autostart directory and the entry that runs this executable, or why
/// they are not available.
fn target() -> Result<(PathBuf, Entry), String> {
    if !cfg!(target_os = "linux") {
        return Err("XDG autostart is a convention of Linux desktops.".into());
    }

    let directory = autostart::directory_from_env()
        .ok_or("Neither XDG_CONFIG_HOME nor HOME holds an absolute path.")?;
    let program = std::env::current_exe()
        .map_err(|error| format!("The path of this program is unknown: {error}"))?;

    let entry = Entry {
        id: "io.github.oito2.oito2_iced.autostart_example".into(),
        name: "OITO2 Iced autostart example".into(),
        program,
        args: Vec::new(),
    };

    Ok((directory, entry))
}
