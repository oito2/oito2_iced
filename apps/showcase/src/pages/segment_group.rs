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

//! The SegmentGroup page: across, down, with icons and with a disabled
//! segment.

use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::widget::segment_group::{self, segment};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("segment_group.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    framework: &'static str,
    view: &'static str,
    period: &'static str,
    plan: &'static str,
}

impl Default for State {
    fn default() -> Self {
        Self {
            framework: "iced",
            view: "grid",
            period: "week",
            plan: "pro",
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A framework was chosen.
    Framework(&'static str),
    /// A view was chosen.
    View(&'static str),
    /// A period was chosen.
    Period(&'static str),
    /// A plan was chosen.
    Plan(&'static str),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Framework(value) => state.framework = value,
        Message::View(value) => state.view = value,
        Message::Period(value) => state.period = value,
        Message::Plan(value) => state.plan = value,
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
        segment_group::segment_group(
            [
                segment("iced", "Iced"),
                segment("gtk", "GTK"),
                segment("qt", "Qt"),
                segment("slint", "Slint"),
            ],
            Some(state.framework),
        )
        .on_change(Message::Framework),
        // #endregion
    );

    let icons = doc::example(
        ctx,
        SOURCE,
        &["icons"],
        // #region icons
        segment_group::segment_group(
            [
                segment("grid", "Grid").icon(lucide::LAYOUT_GRID),
                segment("list", "List").icon(lucide::LIST),
            ],
            Some(state.view),
        )
        .on_change(Message::View),
        // #endregion
    );

    let vertical = doc::example(
        ctx,
        SOURCE,
        &["vertical"],
        // #region vertical
        segment_group::segment_group(
            [
                segment("day", "Today"),
                segment("week", "This week"),
                segment("month", "This month"),
            ],
            Some(state.period),
        )
        .vertical(true)
        .on_change(Message::Period),
        // #endregion
    );

    let disabled = doc::example(
        ctx,
        SOURCE,
        &["disabled"],
        // #region disabled
        segment_group::segment_group(
            [
                segment("free", "Free"),
                segment("pro", "Pro"),
                segment("team", "Team").enabled(false),
            ],
            Some(state.plan),
        )
        .on_change(Message::Plan),
        // #endregion
    );

    doc::page(
        "SegmentGroup",
        "Picks one of a few options on a recessed track, with a raised marker that slides to \
         the selected one.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the choices as `segment`s, each with a value and a label. The group is \
                 controlled: pass the selected value, or `None`, and update it in the message \
                 of `on_change`. Without `on_change` the group is disabled.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::segment_group::{self, segment};",
            ),
            doc::note(
                "Use a segment group for two to five short, mutually exclusive options that \
                 change a view, such as a period or a layout. For tabs with their own \
                 content, use a tab bar; for options that turn on and off, a toggle group.",
            ),
            doc::heading("Examples"),
            doc::subheading("With icons"),
            doc::prose("A segment takes an `icon` before its label."),
            icons,
            doc::subheading("Vertical"),
            doc::prose(
                "With `vertical(true)` the segments stack, as wide as the widest one, with \
                 their labels at the start.",
            ),
            vertical,
            doc::subheading("Disabled segment"),
            doc::prose(
                "A segment with `enabled(false)` stays visible, dimmed, but can't be picked, \
                 and the arrow keys skip it.",
            ),
            disabled,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus into the group, and out of it."],
                    vec![
                        "Right, Left",
                        "Selects the next or previous enabled segment, wrapping around.",
                    ],
                    vec!["Down, Up", "The same in a vertical group."],
                    vec!["Home, End", "Select the first or the last enabled segment."],
                    vec!["Enter, Space", "Selects the focused segment."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::segment_group::segment_group",
                    "oito2_iced::widget::segment_group::SegmentGroup",
                    "oito2_iced::widget::segment_group::segment",
                    "oito2_iced::widget::segment_group::Segment",
                ],
            ),
        ],
    )
}
