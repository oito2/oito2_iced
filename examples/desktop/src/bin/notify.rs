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

//! Notification recipe: a button in an OITO2 window sends a desktop
//! notification with `notify-rust`. The D-Bus call runs as an `iced::Task`,
//! so the window keeps responding while the notification server answers.
//!
//! Linux only: on other platforms the button reports that notifications are
//! not available in this example.

use iced::Task;
use iced::widget::column;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::{button, styled_text};
use oito2_iced::{Element, Theme, font};
use oito2_iced_desktop_example as example;

fn main() -> iced::Result {
    let application = iced::application(Notify::default, Notify::update, Notify::view)
        .title("OITO2 Iced notifications")
        .theme(Notify::theme);

    font::INTER_FONTS
        .into_iter()
        .fold(application, |application, bytes| application.font(bytes))
        .default_font(font::INTER)
        .window(example::window_settings(460.0, 240.0))
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    Send,
    Sent(Result<(), String>),
}

/// Whether a notification is on its way, and the outcome of the last one.
#[derive(Default)]
struct Notify {
    sending: bool,
    status: Option<Result<(), String>>,
}

impl Notify {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Send => {
                self.sending = true;
                Task::perform(send(), Message::Sent)
            }
            Message::Sent(result) => {
                self.sending = false;
                self.status = Some(result);
                Task::none()
            }
        }
    }

    fn theme(&self) -> Theme {
        example::theme()
    }

    fn view(&self) -> Element<'_, Message> {
        let status = match &self.status {
            None => styled_text("", typography::BODY_MEDIUM),
            Some(Ok(())) => styled_text("Notification sent.", typography::BODY_MEDIUM),
            Some(Err(error)) => {
                styled_text(error.as_str(), typography::BODY_MEDIUM).class(TextClass::Error)
            }
        };

        let content = column![
            button::filled("Send notification")
                .on_press_maybe((!self.sending).then_some(Message::Send)),
            status,
        ]
        .spacing(spacing::MD);

        example::page(
            "Desktop notifications",
            "The notification goes to the notification server of the session over D-Bus.",
            content,
        )
    }
}

/// Sends the notification and waits for the server to accept it.
#[cfg(target_os = "linux")]
async fn send() -> Result<(), String> {
    let mut notification = notify_rust::Notification::new();
    notification
        .appname("OITO2 Iced")
        .summary("Hello from OITO2 Iced")
        .body("This notification was sent from an iced::Task.")
        .icon("dialog-information");

    notification
        .show_async()
        .await
        .map(|_handle| ())
        .map_err(|error| format!("The notification was not sent: {error}"))
}

/// Reports that this example sends notifications on Linux only.
#[cfg(not(target_os = "linux"))]
async fn send() -> Result<(), String> {
    Err("This example sends notifications over D-Bus, on Linux only.".into())
}
