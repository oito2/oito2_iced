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

//! The PlaceholderIcon page: placeholders at several sizes and radii, and
//! in a list.

use iced::widget::{column, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::placeholder;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::list_tile::list_tile;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("placeholder_icon.rs");

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

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        placeholder(),
        // #endregion
    );

    let sizes = doc::example(
        ctx,
        SOURCE,
        &["sizes"],
        // #region sizes
        row![
            placeholder().size(24.0).radius(radius::CHECK),
            placeholder().size(32.0).radius(radius::SMALL),
            placeholder(),
            placeholder().size(96.0).radius(radius::LARGE),
        ]
        .spacing(spacing::LG)
        .align_y(Alignment::Center),
        // #endregion
    );

    let round = doc::example(
        ctx,
        SOURCE,
        &["round"],
        // #region round
        placeholder().size(64.0).radius(32.0),
        // #endregion
    );

    let list = doc::example(
        ctx,
        SOURCE,
        &["list"],
        column(["Album one", "Album two"].map(|title| {
            // #region list
            list_tile()
                .leading(placeholder().size(40.0))
                .title(title)
                .subtitle("Cover not loaded")
                // #endregion
                .into()
        }))
        .width(Length::Fixed(360.0)),
    );

    doc::page(
        "PlaceholderIcon",
        "A decorative square that stands in for a missing picture, such as an album cover or \
         an app icon that has not loaded.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`placeholder` fills a rounded square with the highest surface container tone \
                 and draws a picture outline in its center. `size` sets the width and height, \
                 48 pixels by default, and `radius` the corners, the medium radius by default.",
            ),
            doc::code(ctx, "use oito2_iced::icon::placeholder;"),
            doc::heading("Examples"),
            doc::subheading("Sizes"),
            doc::prose(
                "The outline scales with the square. Match the radius to the size, smaller \
                 corners for smaller squares.",
            ),
            sizes,
            doc::subheading("Round"),
            doc::prose("A radius of half the size makes a circle."),
            round,
            doc::subheading("In a list"),
            doc::prose(
                "As the `leading` element of a list row, it keeps the layout of rows whose \
                 picture is missing.",
            ),
            list,
            doc::note(
                "The placeholder is decorative: describe the missing picture in the text \
                 around it when it matters.",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::icon::placeholder",
                    "oito2_iced::icon::Placeholder",
                ],
            ),
        ],
    )
}
