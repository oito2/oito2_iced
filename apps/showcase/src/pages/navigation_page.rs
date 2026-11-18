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

//! The NavigationPage page: the adaptive layout, a fixed rail style and a
//! destination chosen from inside a page.

use iced::widget::container;
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{self, lucide};
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::navigation_page::navigation_page;
use oito2_iced::widget::navigation_rail::{NavigationItem, NavigationRailStyle, item};
use oito2_iced::widget::styled_text;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("navigation_page.rs");

/// The names of the destinations.
const NAMES: [&str; 3] = ["Home", "Library", "Settings"];

// #region items
/// The destinations of the examples.
fn items() -> [NavigationItem<'static>; 3] {
    [
        item(lucide::HOUSE, "Home"),
        item(lucide::LIBRARY, "Library"),
        item(lucide::SETTINGS, "Settings"),
    ]
}
// #endregion

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    preview: usize,
    fixed: usize,
    linked: usize,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A destination of the preview was chosen.
    Preview(usize),
    /// A destination of the fixed rail example was chosen.
    Fixed(usize),
    /// A destination of the linked example was chosen.
    Linked(usize),
    /// The about button was pressed.
    About,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        // #region update
        Message::Preview(index) => state.preview = index,
        // #endregion
        Message::Fixed(index) => state.fixed = index,
        Message::Linked(index) => state.linked = index,
        Message::About => shell.toast("Navigation page example", InfoType::Information),
    }

    Task::none()
}

/// The page of the destination at `index`: its name, centered.
fn page<'a>(index: usize) -> Element<'a, Message, Theme> {
    container(styled_text(
        NAMES.get(index).copied().unwrap_or_default(),
        typography::HEADLINE_SMALL,
    ))
    .center(Length::Fill)
    .into()
}

/// A bordered frame of `width` by 300 pixels around an example.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: Length,
) -> Element<'a, Message, Theme> {
    border_container(content)
        .width(width)
        .height(Length::Fixed(300.0))
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
            navigation_page(items(), state.preview, page(state.preview))
                .compact_breakpoint(560.0)
                .extended_breakpoint(Some(700.0))
                .on_select(Message::Preview),
            // #endregion
            Length::Fill,
        ),
    );

    let fixed = doc::example(
        ctx,
        SOURCE,
        &["rail-style"],
        frame(
            // #region rail-style
            navigation_page(items(), state.fixed, page(state.fixed))
                .rail_style(Some(NavigationRailStyle::Compact))
                .leading(container(icon::icon(lucide::BOOK_OPEN)).padding([spacing::SM, 0.0]))
                .trailing(
                    button::icon_button(lucide::INFO)
                        .tooltip("About")
                        .on_press(Message::About),
                )
                .on_select(Message::Fixed),
            // #endregion
            Length::Fixed(520.0),
        ),
    );

    // #region linked
    let linked: Element<'a, Message, Theme> = if state.linked == 2 {
        page(2)
    } else {
        container(button::outlined("Open settings").on_press(Message::Linked(2)))
            .center(Length::Fill)
            .into()
    };
    let linked = navigation_page(items(), state.linked, linked)
        .rail_style(Some(NavigationRailStyle::Labelled))
        .on_select(Message::Linked);
    // #endregion
    let linked = doc::example(
        ctx,
        SOURCE,
        &["linked"],
        frame(linked, Length::Fixed(520.0)),
    );

    doc::page(
        "NavigationPage",
        "Top-level navigation between pages that turns from a compact rail into a labelled \
         and then an extended rail as the area widens, or into a bottom bar on a touch screen.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "List the destinations as `NavigationItem`s, each made by \
                 `navigation_rail::item` from an icon and a label, and give `navigation_page` \
                 the index of the selected one and its page. The selection lives in the \
                 application: `on_select` publishes the index of a chosen destination.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::navigation_page::navigation_page;\nuse \
                 oito2_iced::widget::navigation_rail::item;",
            ),
            doc::prose(
                "The layout follows the width at layout time, so the page keeps its state when \
                 the window crosses a breakpoint. For a pointer the destinations are a \
                 navigation rail: icons only below `compact_breakpoint` (600 pixels by \
                 default), icons with labels below them up to `extended_breakpoint`, and \
                 labels beside the icons from it; without an extended breakpoint the rail \
                 never extends. With the touch density, below the compact breakpoint, they \
                 are a bottom bar instead. The preview extends from 700 pixels, so it is \
                 extended in a wide window and compact in a narrow one.",
            ),
            doc::note(
                "`rail_background`, `rail_padding` and `separator` set the rail color, the \
                 space around its destinations and the line beside it.",
            ),
            doc::heading("Examples"),
            doc::subheading("Fixed rail style"),
            doc::prose(
                "`rail_style` forces one `NavigationRailStyle` whatever the width, and also \
                 keeps the rail on a touch screen. `leading` and `trailing` sit above and \
                 below the destinations.",
            ),
            fixed,
            doc::subheading("Choosing a destination from a page"),
            doc::prose(
                "Since the application keeps the selection, a page can move to another \
                 destination by publishing the same message, such as from a link inside it.",
            ),
            linked,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Focuses the selected destination of the rail."],
                    vec![
                        "`Up` / `Down`, `Home` / `End`",
                        "Move the focus between the destinations.",
                    ],
                    vec!["`Enter` / `Space`", "Selects the focused destination."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::navigation_page::navigation_page",
                    "oito2_iced::widget::navigation_page::NavigationPage",
                    "oito2_iced::widget::navigation_page::COMPACT_BREAKPOINT",
                    "oito2_iced::widget::navigation_rail::item",
                    "oito2_iced::widget::navigation_rail::NavigationItem",
                    "oito2_iced::widget::navigation_rail::NavigationRailStyle",
                ],
            ),
        ],
    )
}
