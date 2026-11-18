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

//! The RadioButton page: radios with a clickable label.

use iced::widget::column;
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::widget::{controls, toggles};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("radio_button.rs");

/// The sort orders of the preview, with their labels.
const SORTS: [(&str, &str); 3] = [
    ("name", "Sort by name"),
    ("size", "Sort by size"),
    ("date", "Sort by date"),
];

/// The filters of the toggleable example, with their labels.
const FILTERS: [(&str, &str); 2] = [("images", "Only images"), ("videos", "Only videos")];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    sort: &'static str,
    quality: &'static str,
    filter: Option<&'static str>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            sort: "name",
            quality: "high",
            filter: Some("images"),
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A sort order was chosen.
    Sort(&'static str),
    /// A quality was chosen.
    Quality(&'static str),
    /// A filter was chosen, or the filters cleared.
    Filter(Option<&'static str>),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Sort(sort) => state.sort = sort,
        Message::Quality(quality) => state.quality = quality,
        Message::Filter(filter) => state.filter = filter,
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
        column(SORTS.map(|(value, label)| {
            controls::radio_button(
                toggles::radio(value, Some(state.sort)).on_change(move |_| Message::Sort(value)),
                label,
            )
            .into()
        })),
        // #endregion
    );

    let subtitle = doc::example(
        ctx,
        SOURCE,
        &["subtitle"],
        // #region subtitle
        column![
            controls::radio_button(
                toggles::radio("high", Some(state.quality)).on_change(|_| Message::Quality("high")),
                "High quality",
            )
            .subtitle("Larger files"),
            controls::radio_button(
                toggles::radio("low", Some(state.quality)).on_change(|_| Message::Quality("low")),
                "Low quality",
            )
            .subtitle("Smaller files"),
        ],
        // #endregion
    );

    let toggleable = doc::example(
        ctx,
        SOURCE,
        &["toggleable"],
        // #region toggleable
        column(FILTERS.map(|(value, label)| {
            controls::radio_button(
                toggles::radio(value, state.filter)
                    .toggleable(true)
                    .on_change(Message::Filter),
                label,
            )
            .into()
        })),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        controls::radio_button(
            toggles::radio::<_, Message>("type", Some("name")),
            "Sort by type",
        ),
        // #endregion
    );

    let end = doc::example(
        ctx,
        SOURCE,
        &["end"],
        // #region end
        controls::radio_button(
            toggles::radio("name", Some(state.sort)).on_change(|_| Message::Sort("name")),
            "Sort by name",
        )
        .control_at_end(true),
        // #endregion
    );

    doc::page(
        "RadioButton",
        "A radio with a clickable label, for picking one option of a group.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`radio_button` puts a `radio` before a title. Give each radio of a group \
                 its own value and the same selection; a click anywhere on the row, or \
                 Enter or Space while it has the focus, publishes the message of the \
                 radio's `on_change`. A radio without `on_change` disables the row and \
                 dims its text.",
            ),
            doc::code(ctx, "use oito2_iced::widget::{controls, toggles};"),
            doc::note(
                "For a few short options on one line, consider a segment group; in a list \
                 of tiles, use `list_tile::radio_list_tile`.",
            ),
            doc::heading("Examples"),
            doc::subheading("With subtitles"),
            doc::prose("`subtitle` adds secondary text under the title."),
            subtitle,
            doc::subheading("Toggleable"),
            doc::prose(
                "With a `toggleable` radio, activating the selected row clears the group: \
                 `on_change` receives `None`. Without it, activating the selected row does \
                 nothing.",
            ),
            toggleable,
            doc::subheading("Disabled"),
            doc::prose(
                "Without `on_change`, the row keeps showing whether it is selected but \
                 can't be picked.",
            ),
            disabled,
            doc::subheading("Control at the end"),
            doc::prose("`control_at_end(true)` places the radio after the text."),
            end,
            doc::note(
                "The row is a single focus stop with the focus ring around itself; the \
                 radio inside never takes the focus, so each row is its own Tab stop.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next row."],
                    vec!["Enter, Space", "Selects the focused row."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::controls::radio_button",
                    "oito2_iced::widget::controls::ToggleButton",
                    "oito2_iced::widget::toggles::radio",
                ],
            ),
        ],
    )
}
