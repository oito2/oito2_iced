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

//! The FocusBorder page: the focus layer of OITO2, with the `focusable`
//! wrapper, the focus ring of a group, arrow-key groups and the focus
//! traversal of the view.

use iced::widget::{column, container, mouse_area, row, text};
use iced::{Element, Length, Task, mouse};

use oito2_iced::Theme;
use oito2_iced::focus;
use oito2_iced::icon::lucide;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::focusable;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("focus_border.rs");

/// The labels of the focusable boxes.
const BOXES: [&str; 3] = ["First", "Second", "Third"];

/// The tools of the arrow-key toolbar, with their tooltips.
const TOOLS: [(oito2_iced::icon::Icon, &str); 4] = [
    (lucide::TEXT_ALIGN_START, "Align start"),
    (lucide::TEXT_ALIGN_CENTER, "Center"),
    (lucide::TEXT_ALIGN_END, "Align end"),
    (lucide::TEXT_ALIGN_JUSTIFY, "Justify"),
];

/// The state of the page.
#[derive(Debug, Default)]
pub struct State {
    picked: Option<usize>,
    tool: Option<usize>,
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A box was clicked or activated with the keyboard.
    Pick(usize),
    /// A tool of the toolbar was pressed.
    Tool(usize),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Pick(index) => state.picked = Some(index),
        Message::Tool(index) => state.tool = Some(index),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let boxes = row(BOXES.iter().enumerate().map(|(index, label)| {
        // #region preview
        focusable(
            mouse_area(
                border_container(container(text(*label)).center(Length::Fill))
                    .width(Length::Fixed(140.0))
                    .height(Length::Fixed(72.0)),
            )
            .on_press(Message::Pick(index))
            .interaction(mouse::Interaction::Pointer),
        )
        .on_activate(Message::Pick(index))
        // #endregion
        .into()
    }))
    .spacing(spacing::MD);

    let picked = state.picked.and_then(|index| BOXES.get(index)).map_or_else(
        || "Press Tab to move the focus ring between the boxes.".to_owned(),
        |label| format!("Picked: {label}"),
    );

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        column![boxes, doc::caption(picked)]
            .spacing(spacing::MD)
            .align_x(iced::Alignment::Center),
    );

    let forced = doc::example(
        ctx,
        SOURCE,
        &["forced"],
        // #region forced
        focus::ring(
            border_container(container(text("Always on")).center(Length::Fill))
                .width(Length::Fixed(140.0))
                .height(Length::Fixed(72.0))
                .radius(radius::LARGE),
        )
        .focused(true)
        .radius(radius::LARGE),
        // #endregion
    );

    let toolbar = doc::example(
        ctx,
        SOURCE,
        &["toolbar"],
        column![
            // #region toolbar
            focus::edge_interceptor(
                row(TOOLS.iter().enumerate().map(|(index, (icon, label))| {
                    icon_button(*icon)
                        .tooltip(*label)
                        .selected(state.tool == Some(index))
                        .on_press(Message::Tool(index))
                        .into()
                }))
                .spacing(spacing::XS),
            ),
            // #endregion
            doc::caption("Focus a tool, then move with the arrow keys."),
        ]
        .spacing(spacing::MD)
        .align_x(iced::Alignment::Center),
    );

    doc::page(
        "FocusBorder",
        "The focus layer of OITO2: keyboard focus for any element, the focus ring and the \
         focus traversal of the view.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Iced gives the keyboard focus only to text inputs: its buttons, check boxes \
                 and custom widgets never receive it. OITO2 adds a focus layer on top. \
                 `focusable` wraps any element so it takes part in focus traversal; while \
                 focused it draws a 2-pixel ring in the focus color of the theme just outside \
                 the content, and publishes its `on_activate` message on `Enter` or `Space`. \
                 The ring takes no space, so the layout does not move when it appears.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::focus;\nuse oito2_iced::widget::focusable;",
            ),
            doc::prose(
                "Iced moves the focus only when the application runs a focus operation, so \
                 wrap the root of the view in `focus::navigation`: it runs the next and \
                 previous focus operations on `Tab` and `Shift+Tab` over every focusable \
                 element and text input of the view, in layout order.",
            ),
            doc::code(
                ctx,
                "fn view(&self) -> Element<'_, Message> {\n    \
                 focus::navigation(self.content()).into()\n}",
            ),
            doc::prose(
                "A click does not focus a `focusable`, and a press outside it removes the \
                 focus. To make custom content clickable as well, put a `mouse_area` inside, \
                 as the boxes above do; `enabled(false)` takes the element out of the \
                 traversal.",
            ),
            doc::note(
                "Use `focusable` for custom controls that draw their own surface. The OITO2 \
                 widgets, such as the buttons and `list_tile`, already include it.",
            ),
            doc::heading("Examples"),
            doc::subheading("Ring around a group"),
            doc::prose(
                "`focus::ring` draws the ring around a group while any widget inside it has \
                 the focus, and only while the keyboard drives the interface: a mouse press \
                 hides it until the next key press. `focused` forces it on or off, for \
                 example to mirror a selection kept elsewhere, and `radius` sets its corners.",
            ),
            forced,
            doc::subheading("Arrow keys inside a group"),
            doc::prose(
                "`focus::edge_interceptor` moves the focus inside its content with the arrow \
                 keys and stops at the ends of the group instead of leaving it, for toolbars \
                 and grids. `Tab` and `Shift+Tab` keep moving through the view.",
            ),
            toolbar,
            doc::subheading("Focus operations"),
            doc::prose(
                "`focus::operations` adds operations to the ones of Iced: `focus_first`, \
                 `focus_last`, `focus_next_clamped` and `focus_previous_clamped`, which stop \
                 at the ends, and `has_focus`. Run them with \
                 `iced::advanced::widget::operate`; `focus::operations::focus` focuses a \
                 widget by its `Id`.",
            ),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec![
                        "`Tab` / `Shift+Tab`",
                        "Move the focus to the next or previous element.",
                    ],
                    vec![
                        "`Enter` / `Space`",
                        "Publish the activation message of the focused element.",
                    ],
                    vec![
                        "Arrow keys",
                        "Move the focus inside an `edge_interceptor` group.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::focusable",
                    "oito2_iced::widget::Focusable",
                    "oito2_iced::focus::navigation",
                    "oito2_iced::focus::ring",
                    "oito2_iced::focus::edge_interceptor",
                    "oito2_iced::focus::operations::focus_first",
                    "oito2_iced::focus::operations::focus_last",
                    "oito2_iced::focus::operations::focus_next_clamped",
                    "oito2_iced::focus::operations::focus_previous_clamped",
                    "oito2_iced::focus::operations::has_focus",
                ],
            ),
        ],
    )
}
