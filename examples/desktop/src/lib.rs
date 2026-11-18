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

//! Desktop integration recipes for applications built on [`oito2_iced`]: a
//! tray icon, desktop notifications and an autostart entry. They are not
//! part of the design system; each recipe is a binary of this crate
//! (`src/bin`), and this library holds what they share.

pub mod autostart;

use iced::widget::{column, container};
use iced::{Length, Size, window};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::styled_text;
use oito2_iced::{Element, Theme, focus};

/// The theme of every example window: the OITO2 light theme.
pub fn theme() -> Theme {
    Theme::light()
}

/// The settings of an example window of the given size.
pub fn window_settings(width: f32, height: f32) -> window::Settings {
    window::Settings {
        size: Size::new(width, height),
        min_size: Some(Size::new(320.0, 240.0)),
        ..window::Settings::default()
    }
}

/// A window page: `title` and `description` above `content`, on the page
/// background, with Tab and Shift+Tab moving the keyboard focus.
pub fn page<'a, Message: 'a>(
    title: &'a str,
    description: &'a str,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    let body = column![
        styled_text(title, typography::TITLE_LARGE),
        styled_text(description, typography::BODY_MEDIUM).class(TextClass::Variant),
        content.into(),
    ]
    .spacing(spacing::LG);

    focus::navigation(
        container(body)
            .padding(spacing::PAGE)
            .width(Length::Fill)
            .height(Length::Fill)
            .class(ContainerClass::Background),
    )
    .into()
}
