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
//! The Expandable page: a collapsed summary, the button at the start, a
//! custom header and a section opened from outside.

use iced::widget::{column, container, row, text};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::controls::switch_button;
use oito2_iced::widget::expandable::{ButtonPosition, expandable, expandable_with};
use oito2_iced::widget::toggles::switch;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("expandable.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    advanced: bool,
    notes: bool,
    storage: bool,
    details: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            advanced: false,
            notes: true,
            storage: false,
            details: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The advanced options opened or closed.
    Advanced(bool),
    /// The release notes opened or closed.
    Notes(bool),
    /// The storage section opened or closed.
    Storage(bool),
    /// The details opened or closed, from the header or the switch.
    Details(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Advanced(open) => state.advanced = open,
        Message::Notes(open) => state.notes = open,
        Message::Storage(open) => state.storage = open,
        Message::Details(open) => state.details = open,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let width = Length::Fixed(420.0);

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        border_container(
            // #region preview
            expandable(
                "Advanced options",
                column![
                    text("Enable experimental features"),
                    text("Send usage statistics"),
                    text("Use legacy renderer"),
                ],
                state.advanced,
            )
            .collapsed(doc::caption("3 options hidden"))
            .on_toggle(Message::Advanced),
            // #endregion
        )
        .padding(spacing::MD)
        .width(width),
    );

    let start = doc::example(
        ctx,
        SOURCE,
        &["start"],
        container(
            // #region start
            expandable(
                "Release notes",
                text("Fixes the window size on the second monitor."),
                state.notes,
            )
            .button_position(ButtonPosition::Start)
            .on_toggle(Message::Notes),
            // #endregion
        )
        .width(width),
    );

    let custom = doc::example(
        ctx,
        SOURCE,
        &["custom"],
        container(
            // #region custom
            expandable_with(
                row![icon(lucide::HARD_DRIVE), text("Storage")]
                    .spacing(spacing::SM)
                    .align_y(Alignment::Center),
                text("42 GB used of 256 GB."),
                state.storage,
            )
            .gap(spacing::MD)
            .on_toggle(Message::Storage),
            // #endregion
        )
        .width(width),
    );

    let controlled = doc::example(
        ctx,
        SOURCE,
        &["controlled"],
        container(
            column![
                // #region controlled
                switch_button(
                    switch(state.details).on_toggle(Message::Details),
                    "Show details"
                ),
                expandable(
                    "Details",
                    text("Size 2.4 MB, modified today."),
                    state.details,
                )
                .on_toggle(Message::Details),
                // #endregion
            ]
            .spacing(spacing::MD),
        )
        .width(width),
    );

    doc::page(
        "Expandable",
        "A header that shows or hides the content under it, optionally with a summary while \
         it is hidden.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `expandable` a header text, the content it shows and whether it is \
                 expanded. The whole header row toggles the content, with a chevron that \
                 turns over as it opens, and the content grows open. While closed, the \
                 optional `collapsed` element shows in its place, such as a summary. `gap` \
                 sets the space between the header and the content.",
            ),
            doc::code(ctx, "use oito2_iced::widget::expandable::expandable;"),
            doc::prose(
                "The expanded state lives in the application: activating the header \
                 publishes the requested state through `on_toggle`, and the content \
                 follows the state it is given. Without `on_toggle` the header is \
                 disabled.",
            ),
            doc::note(
                "For several sections in one bordered box, where opening one can close the \
                 others, use `expansion_panel`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Button at the start"),
            doc::prose(
                "`button_position(ButtonPosition::Start)` puts the chevron before the \
                 header. This section starts open.",
            ),
            start,
            doc::subheading("Custom header"),
            doc::prose("`expandable_with` takes any header element, such as an icon and a text."),
            custom,
            doc::subheading("Opened from outside"),
            doc::prose(
                "Since the state is the application's, anything can open the section. Here \
                 a switch outside it opens and closes it too.",
            ),
            controlled,
            doc::subheading("Animation"),
            doc::prose(
                "The content grows and shrinks in `DURATION`, 200 milliseconds. With \
                 `animated(false)`, for reduced motion, it shows and hides at once.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the header."],
                    vec!["`Enter` / `Space`", "Shows or hides the content."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::expandable::expandable",
                    "oito2_iced::widget::expandable::expandable_with",
                    "oito2_iced::widget::expandable::Expandable",
                    "oito2_iced::widget::expandable::ButtonPosition",
                    "oito2_iced::widget::expandable::DURATION",
                ],
            ),
        ],
    )
}
