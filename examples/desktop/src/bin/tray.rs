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

//! Tray recipe: a resident application. It starts with no window and a
//! tray icon (a StatusNotifierItem over D-Bus, through `ksni`); the tray menu
//! opens and closes an OITO2 window and quits the application. Closing the
//! window keeps the application running.
//!
//! The tray service runs on its own thread and sends what the menu asks for
//! through a channel, which a `Subscription` turns into messages of the
//! daemon.
//!
//! Linux only: other platforms print a message and exit.

#[cfg(target_os = "linux")]
fn main() -> iced::Result {
    linux::run()
}

#[cfg(not(target_os = "linux"))]
fn main() {
    eprintln!(
        "The tray example uses the StatusNotifierItem D-Bus protocol of Linux desktops; \
         it does nothing on this platform."
    );
}

#[cfg(target_os = "linux")]
mod linux {
    use iced::futures::channel::mpsc;
    use iced::futures::{SinkExt, Stream};
    use iced::widget::row;
    use iced::{Subscription, Task, window};
    use ksni::TrayMethods;
    use oito2_iced::tokens::spacing;
    use oito2_iced::widget::button;
    use oito2_iced::{Element, Theme, font};
    use oito2_iced_desktop_example as example;

    /// Runs the daemon: no window at start, the tray icon from the
    /// subscription.
    pub fn run() -> iced::Result {
        let daemon = iced::daemon(Resident::default, Resident::update, Resident::view)
            .title("OITO2 Iced tray")
            .theme(Resident::theme)
            .subscription(Resident::subscription);

        font::INTER_FONTS
            .into_iter()
            .fold(daemon, |daemon, bytes| daemon.font(bytes))
            .default_font(font::INTER)
            .run()
    }

    /// What the tray asks the application for.
    #[derive(Debug, Clone)]
    enum TrayEvent {
        /// Open the window, or focus it when it is open.
        Open,
        /// Close the window.
        Close,
        /// End the application.
        Quit,
        /// The tray service could not start; holds the reason.
        Unavailable(String),
    }

    /// The tray icon. `ksni` calls it from the thread of its D-Bus service;
    /// every menu activation goes to the daemon through `sender`.
    struct TrayIcon {
        sender: mpsc::Sender<TrayEvent>,
    }

    impl TrayIcon {
        /// Queues `event` for the daemon; drops it when the channel is full
        /// or the daemon is gone.
        fn send(&mut self, event: TrayEvent) {
            let _ = self.sender.try_send(event);
        }
    }

    impl ksni::Tray for TrayIcon {
        fn id(&self) -> String {
            "oito2_iced_tray_example".into()
        }

        fn title(&self) -> String {
            "OITO2 Iced tray".into()
        }

        fn icon_name(&self) -> String {
            "preferences-system".into()
        }

        /// A primary click on the icon opens the window.
        fn activate(&mut self, _x: i32, _y: i32) {
            self.send(TrayEvent::Open);
        }

        fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
            use ksni::menu::StandardItem;

            vec![
                StandardItem {
                    label: "Open window".into(),
                    icon_name: "window-new".into(),
                    activate: Box::new(|tray: &mut Self| tray.send(TrayEvent::Open)),
                    ..Default::default()
                }
                .into(),
                StandardItem {
                    label: "Close window".into(),
                    icon_name: "window-close".into(),
                    activate: Box::new(|tray: &mut Self| tray.send(TrayEvent::Close)),
                    ..Default::default()
                }
                .into(),
                ksni::MenuItem::Separator,
                StandardItem {
                    label: "Quit".into(),
                    icon_name: "application-exit".into(),
                    activate: Box::new(|tray: &mut Self| tray.send(TrayEvent::Quit)),
                    ..Default::default()
                }
                .into(),
            ]
        }
    }

    /// Starts the tray service and yields what its menu asks for. The
    /// service handle lives as long as the stream, so the icon stays while
    /// the subscription is active.
    fn tray_events() -> impl Stream<Item = TrayEvent> {
        iced::stream::channel(16, async |mut output: mpsc::Sender<TrayEvent>| {
            let icon = TrayIcon {
                sender: output.clone(),
            };

            match icon.spawn().await {
                Ok(handle) => {
                    let _handle = handle;
                    std::future::pending::<()>().await;
                }
                Err(error) => {
                    let _ = output.send(TrayEvent::Unavailable(error.to_string())).await;
                }
            }
        })
    }

    #[derive(Debug, Clone)]
    enum Message {
        Tray(TrayEvent),
        Closed(window::Id),
    }

    /// The state of the daemon: the open window, if any, and the reason the
    /// tray is unavailable, if it is.
    #[derive(Default)]
    struct Resident {
        window: Option<window::Id>,
        unavailable: Option<String>,
    }

    impl Resident {
        fn update(&mut self, message: Message) -> Task<Message> {
            match message {
                Message::Tray(TrayEvent::Open) => return self.open(),
                Message::Tray(TrayEvent::Close) => {
                    if let Some(id) = self.window.take() {
                        return window::close(id);
                    }
                }
                Message::Tray(TrayEvent::Quit) => return iced::exit(),
                // Without a tray the window is the only way to quit.
                Message::Tray(TrayEvent::Unavailable(reason)) => {
                    self.unavailable = Some(reason);
                    return self.open();
                }
                Message::Closed(id) => {
                    if self.window == Some(id) {
                        self.window = None;
                    }
                }
            }

            Task::none()
        }

        /// Opens the window, or focuses it when it is already open.
        fn open(&mut self) -> Task<Message> {
            if let Some(id) = self.window {
                return window::gain_focus(id);
            }

            let (id, open) = window::open(example::window_settings(460.0, 240.0));
            self.window = Some(id);
            open.discard()
        }

        fn theme(&self, _window: window::Id) -> Theme {
            example::theme()
        }

        fn view(&self, _window: window::Id) -> Element<'_, Message> {
            let description = match &self.unavailable {
                Some(reason) => reason.as_str(),
                None => {
                    "Closing this window keeps the application in the tray; \
                     Quit in the tray menu ends it."
                }
            };

            let actions = row![
                button::outlined("Close window").on_press(Message::Tray(TrayEvent::Close)),
                button::filled("Quit").on_press(Message::Tray(TrayEvent::Quit)),
            ]
            .spacing(spacing::SM);

            example::page("Resident application", description, actions)
        }

        fn subscription(&self) -> Subscription<Message> {
            Subscription::batch([
                Subscription::run(tray_events).map(Message::Tray),
                window::close_events().map(Message::Closed),
            ])
        }
    }
}
