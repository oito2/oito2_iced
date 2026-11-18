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

//! The SelectableContainer page: a style picker and a size picker.

use iced::widget::{column, row, text};
use iced::{Color, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::{Accent, radius, spacing};
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::selectable;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("selectable_container.rs");

/// The styles of the preview, with the color of their sample; the system
/// style shows the surface of the current theme.
const STYLES: [(&str, Option<Color>); 3] = [
    ("Light", Some(Color::from_rgb8(0xFA, 0xFA, 0xFA))),
    ("Dark", Some(Color::from_rgb8(0x24, 0x24, 0x24))),
    ("System", None),
];

/// The sizes of the second example.
const SIZES: [&str; 3] = ["S", "M", "L"];

/// The state of the page.
#[derive(Debug)]
pub struct State {
    style: usize,
    size: usize,
    compact: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            style: 0,
            size: 1,
            compact: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A style was chosen.
    Style(usize),
    /// A size was chosen.
    Size(usize),
    /// The compact option was toggled.
    Compact(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Style(style) => state.style = style,
        Message::Size(size) => state.size = size,
        Message::Compact(compact) => state.compact = compact,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        row(STYLES.iter().enumerate().map(|(index, (name, color))| {
            // #region preview
            let mut sample = border_container(text("")).height(72.0).width(Length::Fill);
            if let Some(color) = color {
                sample = sample.color(*color);
            }

            selectable::selectable_container(
                column![sample, text(*name)].spacing(spacing::SM),
                state.style == index,
            )
            .on_press(Message::Style(index))
            .width(150.0)
            .into()
            // #endregion
        }))
        .spacing(spacing::MD)
        .wrap(),
    );

    let toggle = doc::example(
        ctx,
        SOURCE,
        &["toggle"],
        // #region toggle
        selectable::selectable_container(text("Compact layout"), state.compact)
            .on_toggle(Message::Compact),
        // #endregion
    );

    let color = doc::example(
        ctx,
        SOURCE,
        &["color"],
        row(SIZES.iter().enumerate().map(|(index, name)| {
            // #region color
            selectable::selectable_container(text(*name), state.size == index)
                .selection_color(Accent::Purple.color().scale_alpha(0.2))
                .padding([spacing::SM, spacing::LG])
                .radius(radius::MEDIUM)
                .on_press(Message::Size(index))
                .into()
            // #endregion
        }))
        .spacing(spacing::SM),
    );

    doc::page(
        "SelectableContainer",
        "A clickable container that shows when it is selected, such as an option in a grid of \
         choices.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`selectable_container` is controlled: give it its content and whether it is \
                 selected, and change the selection in the message of `on_press`. A selected \
                 container takes a tinted background and a 2-pixel border in the indicator \
                 color. With a message, hovering shows a thin border; without one the \
                 container is static and skipped by focus traversal.",
            ),
            doc::code(ctx, "use oito2_iced::widget::selectable;"),
            doc::note(
                "For a card that promotes content, with a visible border at rest, use a \
                 banner.",
            ),
            doc::heading("Examples"),
            doc::subheading("A toggle"),
            doc::prose(
                "`on_toggle` publishes the opposite of the current selection, for a \
                 container that turns an option on and off.",
            ),
            toggle,
            doc::subheading("Selection color and shape"),
            doc::prose(
                "`selection_color` replaces the selected tint, the primary color at 12 % \
                 opacity by default. `padding` sets the space around the content, 8 pixels by \
                 default, and `radius` the corners, the large radius by default.",
            ),
            color,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the next container."],
                    vec!["Enter, Space", "Publishes the message of the container."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::selectable::selectable_container",
                    "oito2_iced::widget::selectable::SelectableContainer",
                    "oito2_iced::widget::selectable::SELECTION_TINT_OPACITY",
                ],
            ),
        ],
    )
}
