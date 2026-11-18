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

//! The Toaster page: toasts of every kind, an action, persistent and
//! loading toasts, placements and dismissing them all.

use iced::widget::{column, container, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::toast::{self, Placement, Toast, ToastId, Toasts};

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("toaster.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    /// The toasts of the example area, apart from the toasts of the
    /// application.
    toasts: Toasts<Message>,
    upload: Option<ToastId>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// Shows a toast of the application.
    Show(Option<InfoType>),
    /// Shows a toast of the application with an Undo action.
    ShowUndo,
    /// Shows a persistent toast in the example area.
    Persistent,
    /// Starts the upload of the example area.
    StartUpload,
    /// Finishes the upload of the example area.
    FinishUpload,
    /// Moves the stack of the example area to the next placement.
    NextPlacement,
    /// Dismisses every toast of the example area.
    DismissAll,
    /// An event of the toaster of the example area.
    Toast(toast::Event),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // #region area-update
        Message::Persistent => {
            state
                .toasts
                .push(Toast::new("Stays until dismissed").with_duration(None));
        }
        Message::StartUpload => {
            let toast = Toast::new("Uploading…").with_loading(true);
            state.upload = Some(state.toasts.push(toast));
        }
        Message::FinishUpload => {
            if let Some(id) = state.upload.take() {
                let done = Toast::new("3 files uploaded").with_kind(InfoType::Success);
                state.toasts.replace(id, done);
            }
        }
        Message::NextPlacement => {
            let index = Placement::ALL
                .iter()
                .position(|placement| *placement == state.toasts.placement())
                .unwrap_or_default();
            let next = Placement::ALL[(index + 1) % Placement::ALL.len()];
            state.toasts.set_placement(next);
        }
        Message::DismissAll => state.toasts.dismiss_all(),
        Message::Toast(event) => {
            if let Some(action) = state.toasts.update(event) {
                return update(state, action, shell);
            }
        }
        // #endregion
        Message::Show(kind) => shell.request(Action::Toast {
            title: kind
                .map_or("Settings saved", |_| "Something happened")
                .to_owned(),
            description: Some(match kind {
                Some(kind) => format!("A {kind:?} message."),
                None => "They apply to every window.".to_owned(),
            }),
            kind: kind.unwrap_or(InfoType::Success),
            undo: false,
        }),
        Message::ShowUndo => shell.request(Action::Toast {
            title: "Message deleted".to_owned(),
            description: None,
            kind: InfoType::Information,
            undo: true,
        }),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        button::outlined("Show a toast").on_press(Message::Show(None)),
        // #endregion
    );

    let kinds = doc::example(
        ctx,
        SOURCE,
        &["kinds"],
        // #region kinds
        row(InfoType::ALL.into_iter().map(|kind| {
            button::outlined(format!("{kind:?}"))
                .on_press(Message::Show(Some(kind)))
                .into()
        }))
        .spacing(spacing::SM)
        .wrap(),
        // #endregion
    );

    let action = doc::example(
        ctx,
        SOURCE,
        &["action"],
        // #region action
        button::outlined("With an action").on_press(Message::ShowUndo),
        // #endregion
    );

    let upload = if state.upload.is_some() {
        button::outlined("Finish upload").on_press(Message::FinishUpload)
    } else {
        button::outlined("Start upload").on_press(Message::StartUpload)
    };

    let area = doc::example(
        ctx,
        SOURCE,
        &["area", "area-update"],
        container(
            // #region area
            toast::toaster(
                container(
                    column![
                        row![
                            button::outlined("Persistent").on_press(Message::Persistent),
                            upload,
                        ]
                        .spacing(spacing::SM),
                        row![
                            button::outlined(format!("Placement: {:?}", state.toasts.placement()))
                                .on_press(Message::NextPlacement),
                            button::text("Dismiss all").on_press(Message::DismissAll),
                        ]
                        .spacing(spacing::SM),
                    ]
                    .spacing(spacing::SM)
                    .align_x(Alignment::Center),
                )
                .center(Length::Fill),
                &state.toasts,
                &ctx.l10n,
                Message::Toast,
            ),
            // #endregion
        )
        .height(Length::Fixed(320.0)),
    );

    doc::page(
        "Toaster",
        "A short notification that appears over the application for a few seconds, \
         optionally with an action.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "The application owns a `Toasts` queue and wraps the root of its view in a \
                 `toaster`, which draws the visible toasts over it and publishes \
                 `toast::Event`s for `Toasts::update`. `push` adds a `Toast` and returns its \
                 `ToastId`, `replace` changes it and `dismiss` closes it.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::toast::{self, Toast, Toasts};\n\n\
                 // In `update`:\n\
                 Message::Save => {\n    \
                     self.toasts.push(Toast::new(\"Saved\").with_action(\"Undo\", Message::Undo));\n\
                 }\n\
                 Message::Toast(event) => {\n    \
                     if let Some(action) = self.toasts.update(event) {\n        \
                         return self.update(action);\n    \
                     }\n\
                 }\n\n\
                 // In `view`:\n\
                 toast::toaster(content, &self.toasts, &l10n, Message::Toast)",
            ),
            doc::prose(
                "A toast stays for its duration, 4 seconds by default, and its time stops \
                 while the pointer is over the stack or the focus is in a toast. At most 3 \
                 toasts show at a time (`Toasts::with_max_visible`), the oldest nearest the \
                 edge of the `Placement`, which is the bottom center by default; newer ones \
                 wait for room. The toaster reads the clock from the redraw events, so the \
                 application needs no subscription.",
            ),
            doc::heading("Examples"),
            doc::subheading("Kinds"),
            doc::prose(
                "`with_kind` shows a colored icon for the kind of message; a toast without a \
                 kind has no icon. `with_description` adds a line of detail under the bold \
                 title. The buttons above show the toasts of the showcase window.",
            ),
            kinds,
            doc::subheading("Action"),
            doc::prose(
                "`with_action` adds a button with a label and a message. Pressing it closes \
                 the toast, and `Toasts::update` returns the message for the application to \
                 run.",
            ),
            action,
            doc::subheading("Persistent, loading and placement"),
            doc::prose(
                "The toaster below has its own queue. `with_duration(None)` keeps a toast \
                 until it is dismissed, and `with_dismissible(false)` removes its dismiss \
                 button and Escape, so only code closes it. `with_loading(true)` shows a \
                 spinner and keeps the toast until `replace` or `dismiss`. `set_placement` \
                 moves the stack and `dismiss_all` closes every toast, shown or waiting.",
            ),
            area,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the action and dismiss buttons."],
                    vec![
                        "Escape",
                        "Dismisses the focused toast, when it is dismissible.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::toast::toaster",
                    "oito2_iced::widget::toast::Toaster",
                    "oito2_iced::widget::toast::Toasts",
                    "oito2_iced::widget::toast::Toast",
                    "oito2_iced::widget::toast::ToastId",
                    "oito2_iced::widget::toast::Event",
                    "oito2_iced::widget::toast::Placement",
                    "oito2_iced::widget::toast::DEFAULT_DURATION",
                    "oito2_iced::widget::toast::DEFAULT_MAX_VISIBLE",
                ],
            ),
        ],
    )
}
