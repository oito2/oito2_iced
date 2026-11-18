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

//! The InfoBox page: message boxes and badges in every kind.

use iced::widget::{column, container, row};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{self, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::info::{InfoType, info_badge, info_box};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("info_box.rs");

/// The width of the example boxes.
const WIDTH: f32 = 380.0;

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    dismissed: bool,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The update box was dismissed, or shown again.
    Dismissed(bool),
    /// The restart button was pressed.
    Restart,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Dismissed(dismissed) => state.dismissed = dismissed,
        Message::Restart => shell.toast("Restarting to update", InfoType::Information),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        container(
            // #region preview
            info_box(InfoType::Warning)
                .title("Low disk space")
                .subtitle("Free up space to keep receiving updates."),
            // #endregion
        )
        .width(Length::Fixed(WIDTH)),
    );

    let types = doc::example(
        ctx,
        SOURCE,
        &["types"],
        column(InfoType::ALL.into_iter().map(|kind| {
            // #region types
            info_box(kind)
                .title(kind.label(l10n))
                .subtitle("A short message explaining the state.")
                // #endregion
                .into()
        }))
        .spacing(spacing::SM)
        .width(Length::Fixed(WIDTH)),
    );

    let actions: Element<'a, Message, Theme> = if state.dismissed {
        button::outlined("Show the box again")
            .on_press(Message::Dismissed(false))
            .into()
    } else {
        // #region actions
        info_box(InfoType::Information)
            .icon(icon::icon(lucide::REFRESH_CW))
            .title("Update available")
            .subtitle("Version 2.0 is ready to install.")
            .trailing(
                button::icon_button(lucide::X)
                    .tooltip("Dismiss")
                    .on_press(Message::Dismissed(true)),
            )
            .content(button::outlined("Restart now").on_press(Message::Restart))
            .into()
        // #endregion
    };
    let actions = doc::example(
        ctx,
        SOURCE,
        &["actions"],
        container(actions).center_x(Length::Fixed(WIDTH)),
    );

    let badges = doc::example(
        ctx,
        SOURCE,
        &["badges"],
        row(InfoType::ALL.into_iter().map(|kind| {
            // #region badges
            info_badge(kind, kind.label(l10n))
            // #endregion
        }))
        .spacing(spacing::SM),
    );

    doc::page(
        "InfoBox",
        "Message boxes and small status badges tinted by their kind: information, success, \
         warning, danger or important.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`info_box` takes an `InfoType` and any of a `title`, a `subtitle`, extra \
                 `content` and a `trailing` element. The kind picks the icon and the tint; \
                 the title takes the kind color, adjusted to stay readable on the tinted \
                 background. The box fills the available width.",
            ),
            doc::prose(
                "`info_badge` is a pill-shaped label tinted by its kind, for statuses and \
                 counters.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::info::{InfoType, info_badge, info_box};",
            ),
            doc::heading("Examples"),
            doc::subheading("Types"),
            doc::prose(
                "Each `InfoType` has its own icon and color: `Information`, `Success`, \
                 `Warning`, `Danger`, and `Important` in the accent color. `label` gives the \
                 localized name of a kind, such as \"Warning\".",
            ),
            types,
            doc::subheading("Actions and custom icon"),
            doc::prose(
                "`content` adds content under the subtitle, such as buttons, and `trailing` \
                 puts an element at the end, such as a close button. `icon` replaces the kind \
                 icon, drawn in the kind color.",
            ),
            actions,
            doc::subheading("Badges"),
            doc::prose("`info_badge` takes the same kinds."),
            badges,
            doc::note(
                "Iced 0.14 has no accessibility tree, so the kind of a box reaches the user \
                 only through its icon and color: say the kind in the title when it matters, \
                 above all with a custom `icon`.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::info::info_box",
                    "oito2_iced::widget::info::InfoBox",
                    "oito2_iced::widget::info::info_badge",
                    "oito2_iced::widget::info::InfoType",
                ],
            ),
        ],
    )
}
