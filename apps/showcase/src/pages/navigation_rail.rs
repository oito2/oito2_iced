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

//! The NavigationRail page: the three rail styles, a trailing element
//! and a single rail destination.

use iced::widget::{column, container, row, rule};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::navigation_rail::{
    NavigationItem, NavigationRailStyle, item, navigation_rail, rail_item,
};
use oito2_iced::widget::segment_group::{segment, segment_group};
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("navigation_rail.rs");

/// The names of the destinations.
const NAMES: [&str; 5] = ["Inbox", "Sent", "Drafts", "Archive", "Trash"];

// #region items
/// The destinations of the examples.
fn items() -> [NavigationItem<'static>; 5] {
    [
        item(lucide::INBOX, "Inbox"),
        item(lucide::SEND, "Sent"),
        item(lucide::FILE_PEN, "Drafts"),
        item(lucide::ARCHIVE, "Archive"),
        item(lucide::TRASH, "Trash"),
    ]
}
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    selected: usize,
    style: NavigationRailStyle,
    styled: usize,
    starred: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            selected: 0,
            style: NavigationRailStyle::Labelled,
            styled: 0,
            starred: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A destination of the preview was chosen.
    Select(usize),
    /// A rail style was chosen.
    Style(NavigationRailStyle),
    /// A destination of the styles example was chosen.
    Styled(usize),
    /// The settings button was pressed.
    Settings,
    /// The single destination was pressed.
    Starred,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Select(index) => state.selected = index,
        // #endregion
        Message::Style(style) => state.style = style,
        Message::Styled(index) => state.styled = index,
        Message::Settings => shell.toast("Settings", InfoType::Information),
        Message::Starred => state.starred = !state.starred,
    }

    Task::none()
}

/// A bordered box with `rail` at the start and the name of the
/// destination at `index` beside it.
fn frame<'a>(
    rail: impl Into<Element<'a, Message, Theme>>,
    index: usize,
) -> Element<'a, Message, Theme> {
    border_container(row![
        rail.into(),
        rule::vertical(1),
        container(styled_text(
            NAMES.get(index).copied().unwrap_or_default(),
            typography::HEADLINE_SMALL,
        ))
        .center(Length::Fill),
    ])
    .width(Length::Fixed(520.0))
    .height(Length::Fixed(380.0))
    .clip(true)
    .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["items", "preview", "update"],
        frame(
            // #region preview
            navigation_rail(items(), Some(state.selected))
                .on_select(Message::Select)
                .trailing(
                    button::icon_button(lucide::SETTINGS)
                        .tooltip("Settings")
                        .on_press(Message::Settings),
                ),
            // #endregion
            state.selected,
        ),
    );

    let styles = doc::example(
        ctx,
        SOURCE,
        &["styles"],
        column![
            segment_group(
                [
                    segment(NavigationRailStyle::Compact, "Compact"),
                    segment(NavigationRailStyle::Labelled, "Labelled"),
                    segment(NavigationRailStyle::LabelledExtended, "Extended"),
                ],
                Some(state.style),
            )
            .on_change(Message::Style),
            frame(
                // #region styles
                navigation_rail(items(), Some(state.styled))
                    .style(state.style)
                    .on_select(Message::Styled),
                // #endregion
                state.styled,
            ),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let single = doc::example(
        ctx,
        SOURCE,
        &["single"],
        // #region single
        rail_item(
            item(lucide::STAR, "Starred"),
            state.starred,
            NavigationRailStyle::Labelled,
        )
        .on_press(Message::Starred),
        // #endregion
    );

    doc::page(
        "NavigationRail",
        "A vertical bar of destinations at the start side of a window, with icons only, \
         labels below the icons or labels beside them.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the destinations as `NavigationItem`s made by `item` from an icon and a \
                 label, and give `navigation_rail` the index of the selected one, or `None`. \
                 The selection lives in the application: `on_select` publishes the index of a \
                 chosen destination, and without it the rail is disabled. The selected \
                 destination sits on a pill in the secondary container color.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::navigation_rail::{item, navigation_rail};",
            ),
            doc::note(
                "For a whole window that switches between a bottom bar and a rail by width, \
                 use `navigation_page`, which builds this rail.",
            ),
            doc::prose(
                "The destinations scroll with the mouse wheel when they don't fit the height, \
                 and the rail keeps the selected and the focused destination in view. \
                 `leading` and `trailing` stay at the top and the bottom. The rail takes the \
                 chrome sidebar color unless `background` is given; `padding` sets the space \
                 around the destinations. A destination shows `selected_icon` while selected, \
                 and `tooltip` where its label is hidden.",
            ),
            doc::heading("Examples"),
            doc::subheading("Styles"),
            doc::prose(
                "`style` picks a `NavigationRailStyle`: `Compact` shows icons only, with the \
                 label as a tooltip, 64 pixels wide; `Labelled` adds a small label below each \
                 icon, 88 pixels wide; and `LabelledExtended` puts the label beside the icon \
                 in a rail 240 pixels wide.",
            ),
            styles,
            doc::subheading("A single destination"),
            doc::prose(
                "`rail_item` lays out one destination as in a rail of a style, with the pill, \
                 the focus ring and the hand cursor, for rails built by hand.",
            ),
            single,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab`",
                        "Focuses the selected destination; the rail is one stop.",
                    ],
                    vec![
                        "`Up` / `Down`",
                        "Move the focus to the previous or next destination, stopping at the \
                         ends.",
                    ],
                    vec![
                        "`Home` / `End`",
                        "Move the focus to the first or last destination.",
                    ],
                    vec!["`Enter` / `Space`", "Selects the focused destination."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::navigation_rail::navigation_rail",
                    "oito2_iced::widget::navigation_rail::NavigationRail",
                    "oito2_iced::widget::navigation_rail::item",
                    "oito2_iced::widget::navigation_rail::NavigationItem",
                    "oito2_iced::widget::navigation_rail::NavigationRailStyle",
                    "oito2_iced::widget::navigation_rail::rail_item",
                    "oito2_iced::widget::navigation_rail::NavigationRailItem",
                ],
            ),
        ],
    )
}
