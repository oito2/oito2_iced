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
//! The BorderContainer page: the default and custom colors, rows inside
//! it, and the tinted variations as they grow with long text.

use iced::widget::{column, container, text};
use iced::{Color, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::tokens::{Accent, radius, spacing};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::list_tile::list_tile;
use oito2_iced::widget::slider::slider;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("border_container.rs");

/// The sample text of the tinted containers.
const LOREM: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                     tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
                     veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea \
                     commodo consequat. Duis aute irure dolor in reprehenderit in voluptate \
                     velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat \
                     cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id \
                     est laborum.";

// #region tints
/// The tints of the examples, named by what they stand for.
const TINTS: [(&str, Color); 5] = [
    ("Accent", Accent::Blue.color()),
    ("Success", Accent::Green.color()),
    ("Warning", Accent::Yellow.color()),
    ("Error", Accent::Red.color()),
    ("Info", Accent::Teal.color()),
];
// #endregion

/// The state of the page.
#[derive(Debug)]
pub struct State {
    length: f32,
    opened: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            length: 80.0,
            opened: 0,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The text length slider moved.
    Length(f32),
    /// The row inside the container was pressed.
    Open,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Length(length) => state.length = length,
        Message::Open => state.opened += 1,
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
        border_container(text("Default surface"))
            .width(200)
            .height(120)
            .padding(spacing::MD),
        // #endregion
    );

    let color = doc::example(
        ctx,
        SOURCE,
        &["color"],
        // #region color
        border_container(text("Custom colors"))
            .width(200)
            .height(120)
            .padding(spacing::MD)
            .color(Accent::Slate.color().scale_alpha(0.15))
            .border_color(Accent::Slate.color())
            .radius(radius::MEDIUM),
        // #endregion
    );

    let rows = doc::example(
        ctx,
        SOURCE,
        &["rows"],
        container(
            // #region rows
            border_container(
                list_tile()
                    .leading(icon(lucide::FOLDER))
                    .title("Open folder")
                    .square()
                    .on_press(Message::Open),
            )
            .width(Length::Fill)
            .clip(true),
            // #endregion
        )
        .width(Length::Fixed(320.0)),
    );

    let sample: String = LOREM.chars().take(state.length.round() as usize).collect();

    let tinted = column(TINTS.iter().map(|(name, tint)| {
        // #region tinted
        border_container(text(format!("{name}: {sample}")))
            .tint(*tint)
            .padding(spacing::MD)
            .width(Length::Fill)
            // #endregion
            .into()
    }))
    .spacing(spacing::MD);

    let tinted = doc::example(
        ctx,
        SOURCE,
        &["tints", "tinted"],
        container(
            column![
                slider(state.length)
                    .range(1.0, LOREM.chars().count() as f32)
                    .divisions(LOREM.chars().count() as u32 - 1)
                    .label("Text length")
                    .on_change(Message::Length),
                border_container(text(sample.clone()))
                    .padding(spacing::MD)
                    .width(Length::Fill),
                tinted,
            ]
            .spacing(spacing::MD),
        )
        .width(Length::Fixed(480.0)),
    );

    doc::page(
        "BorderContainer",
        "A surface with a thin border and large rounded corners, the basic card of OITO2 \
         layouts.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`border_container` draws its content on the theme surface with a 1-pixel \
                 border in the divider color and the large corner radius. Without `width` \
                 and `height` it sizes to its content; `padding` adds space inside the \
                 border and `margin` outside it.",
            ),
            doc::code(ctx, "use oito2_iced::widget::containers::border_container;"),
            doc::heading("Examples"),
            doc::subheading("Custom color"),
            doc::prose(
                "`color` replaces the background, `border_color` the border, and `radius` \
                 the corners.",
            ),
            color,
            doc::subheading("Rows inside"),
            doc::prose(
                "The hover and pressed shapes of rows inside the container show on its \
                 surface. A square row with `clip(true)` keeps them inside the rounded \
                 corners; the container does not clip by default, so the focus rings of \
                 its children stay visible.",
            ),
            rows,
            doc::subheading("Tinted"),
            doc::prose(
                "`tint` uses a color, such as a status color, for the border and, at \
                 `tint_opacity` (10 % by default), for the fill. Like every container, it \
                 grows with its content: drag the slider to change the text length.",
            ),
            tinted,
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::containers::border_container",
                    "oito2_iced::widget::containers::BorderContainer",
                    "oito2_iced::widget::containers::TINT_OPACITY",
                ],
            ),
        ],
    )
}
