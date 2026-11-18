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
//! The DiagonalClip page: two clips that split a square, and each of the
//! four corners.

use iced::widget::{Space, column, container, row, stack, text};
use iced::{Alignment, Color, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::diagonal_clip::{DiagonalCorner, diagonal_clip};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("diagonal_clip.rs");

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

// #region colored
/// A box that fills its space with `color`.
fn colored<'a>(color: Color) -> Element<'a, Message, Theme> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &Theme| container::Style::default().background(color))
        .into()
}

// #endregion

// #region primary
/// A box that fills its space with the primary color of the theme.
fn primary<'a>() -> Element<'a, Message, Theme> {
    container(Space::new())
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style::default().background(theme.roles().primary))
        .into()
}
// #endregion

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["colored", "preview"],
        border_container(
            // #region preview
            stack![
                diagonal_clip(
                    DiagonalCorner::TopLeft,
                    colored(Color::from_rgb8(0xFA, 0xFA, 0xFA))
                ),
                diagonal_clip(
                    DiagonalCorner::BottomRight,
                    colored(Color::from_rgb8(0x24, 0x24, 0x24)),
                ),
            ],
            // #endregion
        )
        .width(160)
        .height(160)
        .clip(true),
    );

    let corners = doc::example(
        ctx,
        SOURCE,
        &["primary", "corners"],
        row([
            (DiagonalCorner::TopLeft, "TopLeft"),
            (DiagonalCorner::TopRight, "TopRight"),
            (DiagonalCorner::BottomLeft, "BottomLeft"),
            (DiagonalCorner::BottomRight, "BottomRight"),
        ]
        .map(|(corner, name)| {
            column![
                border_container(
                    // #region corners
                    diagonal_clip(corner, primary()),
                    // #endregion
                )
                .width(96)
                .height(96)
                .clip(true),
                text(name),
            ]
            .spacing(spacing::SM)
            .align_x(Alignment::Center)
            .into()
        }))
        .spacing(spacing::LG)
        .wrap()
        .vertical_spacing(spacing::LG),
    );

    doc::page(
        "DiagonalClip",
        "Clips content along a diagonal, keeping the triangle of one corner, for example to \
         split a swatch between two themes.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`diagonal_clip` cuts its content along the diagonal between the two \
                 corners next to the given `DiagonalCorner`, and keeps the triangle on the \
                 side of that corner. Stack two clips of opposite corners to split an area \
                 in two, as in the preview of a light and a dark theme.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::widget::diagonal_clip::{DiagonalCorner, diagonal_clip};",
            ),
            doc::note(
                "The corners are physical: `TopLeft` stays at the top left in right-to-left \
                 text too. The pointer reaches the content only inside the triangle; \
                 elsewhere it passes to the layers below in the stack.",
            ),
            doc::heading("Examples"),
            doc::subheading("Corners"),
            doc::prose("Each `DiagonalCorner` keeps a different half of the square."),
            corners,
            doc::heading("Rendering"),
            doc::prose(
                "Iced 0.14 clips drawing to rectangles only, so the triangle is drawn as a \
                 staircase of horizontal strips one logical pixel tall, each a rectangular \
                 clip of the content. The diagonal edge has no anti-aliasing, and the \
                 content is drawn once per strip, at most `MAX_STRIPS` times: keep the \
                 content simple, such as a color or a small preview.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::diagonal_clip::diagonal_clip",
                    "oito2_iced::widget::diagonal_clip::DiagonalClip",
                    "oito2_iced::widget::diagonal_clip::DiagonalCorner",
                    "oito2_iced::widget::diagonal_clip::MAX_STRIPS",
                ],
            ),
        ],
    )
}
