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

//! The ScrollViewUndershoot page: edge shadows on vertical and horizontal
//! scroll views.

use iced::widget::{column, container, row, scrollable, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::scroll_area::{self, Axis};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("scroll_view_undershoot.rs");

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

/// A list of `count` rows labelled `label` and their number.
fn rows<'a>(label: &'a str, count: u32) -> Element<'a, Message, Theme> {
    column((1..=count).map(|index| {
        container(text(format!("{label} {index}")))
            .padding([spacing::SM, spacing::MD])
            .width(Length::Fill)
            .into()
    }))
    .into()
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
            scroll_area::undershoot(scrollable(rows("Item", 20))),
            // #endregion
            260.0,
            200.0,
        ),
    );

    let horizontal = doc::example(
        ctx,
        SOURCE,
        &["horizontal"],
        frame(
            // #region horizontal
            scroll_area::undershoot(
                scrollable(
                    row((1..=12).map(|tag| {
                        border_container(text(format!("Tag {tag}")))
                            .padding([spacing::XS, spacing::MD])
                            .into()
                    }))
                    .spacing(spacing::SM)
                    .padding(spacing::SM),
                )
                .direction(scrollable::Direction::Horizontal(scroll_area::scrollbar()))
                .height(Length::Fill),
            )
            .axis(Axis::Horizontal),
            // #endregion
            320.0,
            72.0,
        ),
    );

    let edges = doc::example(
        ctx,
        SOURCE,
        &["edges"],
        frame(
            // #region edges
            scroll_area::undershoot(scrollable(rows("Message", 20)))
                .start(false)
                .extent(32.0),
            // #endregion
            260.0,
            200.0,
        ),
    );

    doc::page(
        "ScrollViewUndershoot",
        "Fades soft shadows in at the edges of a scroll view while there is more content \
         beyond them.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Wrap a scroll view, such as a `scrollable` built by hand, in `undershoot`. A \
                 shadow fades in at each edge with content hidden beyond it, and out when the \
                 scroll reaches that edge. Only the first scroll view found inside counts, \
                 not the scroll views nested in it.",
            ),
            doc::code(ctx, "use oito2_iced::widget::scroll_area::{self, Axis};"),
            doc::note(
                "A `scroll_area` draws the same shadows with `undershoot(true)`, following \
                 its direction.",
            ),
            doc::heading("Examples"),
            doc::subheading("Horizontal"),
            doc::prose(
                "`axis(Axis::Horizontal)` puts the shadows at the left and right edges of a \
                 horizontal scroll view. Along that axis the vertical mouse wheel scrolls the \
                 content sideways; `pointer_scrolling(false)` turns that off.",
            ),
            horizontal,
            doc::subheading("One edge and depth"),
            doc::prose(
                "`start` and `end` turn the shadow of each edge on or off, and `extent` sets \
                 how deep the shadows reach, 12 pixels by default.",
            ),
            edges,
            doc::note(
                "The shadows are decorative and ignore the pointer. They fade for 150 \
                 milliseconds and show their final state on the first frame; \
                 `animated(false)` makes them appear and disappear at once, for reduced \
                 motion.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::scroll_area::undershoot",
                    "oito2_iced::widget::scroll_area::Undershoot",
                    "oito2_iced::widget::scroll_area::Axis",
                    "oito2_iced::widget::scroll_area::UNDERSHOOT_EXTENT",
                    "oito2_iced::widget::scroll_area::UNDERSHOOT_FADE",
                ],
            ),
        ],
    )
}
