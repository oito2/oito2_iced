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

//! The ScrollArea page: scrolling up and down, across and both ways.

use iced::widget::{column, row, text};
use iced::{Element, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::scroll_area::{self, Direction};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("scroll_area.rs");

/// A paragraph longer than the examples.
const TEXT: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod \
                    tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
                    veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
                    commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
                    velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
                    cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
                    est laborum.";

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {}
}

/// A frame of `width` × `height` around an example.
fn frame<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
    width: f32,
    height: f32,
) -> Element<'a, Message, Theme> {
    border_container(content).width(width).height(height).into()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        frame(
            // #region preview
            scroll_area::scroll_area(column![text(TEXT), text(TEXT)].spacing(spacing::MD))
                .padding(spacing::MD),
            // #endregion
            280.0,
            180.0,
        ),
    );

    let undershoot = doc::example(
        ctx,
        SOURCE,
        &["undershoot"],
        frame(
            // #region undershoot
            scroll_area::scroll_area(column![text(TEXT), text(TEXT)].spacing(spacing::MD))
                .undershoot(true)
                .padding(spacing::MD),
            // #endregion
            280.0,
            180.0,
        ),
    );

    let horizontal = doc::example(
        ctx,
        SOURCE,
        &["horizontal"],
        frame(
            // #region horizontal
            scroll_area::scroll_area(
                row((1..=12).map(|item| {
                    border_container(text(format!("Item {item}")))
                        .padding([spacing::XS, spacing::MD])
                        .into()
                }))
                .spacing(spacing::SM),
            )
            .direction(Direction::Horizontal)
            .padding(spacing::SM),
            // #endregion
            280.0,
            80.0,
        ),
    );

    let both = doc::example(
        ctx,
        SOURCE,
        &["both"],
        frame(
            // #region both
            scroll_area::scroll_area(
                column((0..6).map(|line| {
                    row((0..8).map(|cell| {
                        border_container(text(format!("{}", line * 8 + cell)))
                            .width(76.0)
                            .height(76.0)
                            .padding(spacing::SM)
                            .into()
                    }))
                    .spacing(spacing::XS)
                    .into()
                }))
                .spacing(spacing::XS),
            )
            .direction(Direction::Both),
            // #endregion
            280.0,
            200.0,
        ),
    );

    doc::page(
        "ScrollArea",
        "A scrollable box for content larger than it, with the themed scroll bars on the axes \
         it scrolls in.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Wrap content that may not fit in `scroll_area`. The area fills the available \
                 space, so place it where its size is bounded, or set `width` and `height`. \
                 It scrolls up and down by default; `direction` chooses across or both ways. \
                 `padding` is the space around the content, inside the scrolled part.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::scroll_area::{self, Direction};",
            ),
            doc::prose(
                "The scroll bars, a 10-pixel rail with a 6-pixel scroller, show on the axes \
                 the area scrolls in while the content overflows them. `on_scroll` publishes \
                 the new viewport whenever the content scrolls, and `id` names the scrollable \
                 for the scroll operations of `iced::widget::operation`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Undershoot shadow"),
            doc::prose(
                "With `undershoot(true)`, a soft shadow marks the edges with more content \
                 beyond them.",
            ),
            undershoot,
            doc::subheading("Across"),
            doc::prose("`Direction::Horizontal` scrolls a row from side to side."),
            horizontal,
            doc::subheading("Both ways"),
            doc::prose(
                "`Direction::Both` scrolls large content in both directions, with a scroll \
                 bar on each axis. With `undershoot`, the shadows follow the vertical axis \
                 only.",
            ),
            both,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::scroll_area::scroll_area",
                    "oito2_iced::widget::scroll_area::ScrollArea",
                    "oito2_iced::widget::scroll_area::Direction",
                    "oito2_iced::widget::scroll_area::scrollbar",
                    "oito2_iced::widget::scroll_area::SCROLLBAR_WIDTH",
                    "oito2_iced::widget::scroll_area::SCROLLER_WIDTH",
                ],
            ),
        ],
    )
}
