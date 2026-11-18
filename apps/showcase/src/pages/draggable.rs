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
//! The Draggable page: content moved inside a stack with the pointer or
//! the arrow keys, and its move reports.

use iced::widget::{Space, container, row, stack, text};
use iced::{Alignment, Element, Length, Point, Task};

use oito2_iced::Theme;
use oito2_iced::icon::{icon, lucide};
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::containers::border_container;
use oito2_iced::widget::draggable::{self, Move, draggable};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("draggable.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    chip: Point,
    note: Point,
    dragging: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            chip: Point::new(24.0, 40.0),
            note: Point::new(24.0, 24.0),
            dragging: false,
        }
    }
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The chip moved.
    Chip(Point),
    /// The note moved.
    Note(Point),
    /// A drag of the note started (true) or ended (false).
    Dragging(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Chip(point) => state.chip = point,
        Message::Note(point) => state.note = point,
        Message::Dragging(dragging) => state.dragging = dragging,
    }

    Task::none()
}

/// A board of 420 by 220 pixels around `content`.
fn board<'a>(content: impl Into<Element<'a, Message, Theme>>) -> Element<'a, Message, Theme> {
    border_container(content)
        .width(420)
        .height(220)
        .clip(true)
        .into()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        board(
            // #region preview
            stack![
                Space::new().width(Length::Fill).height(Length::Fill),
                draggable(
                    state.chip,
                    container(
                        row![icon(lucide::MOVE), text("Drag me or use arrows")]
                            .spacing(spacing::SM)
                            .align_y(Alignment::Center),
                    )
                    .padding([spacing::XS, spacing::MD])
                    .class(ContainerClass::Popover),
                )
                .on_move(Message::Chip),
            ],
            // #endregion
        ),
    );

    let position = format!(
        "{}, {}{}",
        state.note.x.round(),
        state.note.y.round(),
        if state.dragging { " · dragging" } else { "" }
    );

    let callbacks = doc::example(
        ctx,
        SOURCE,
        &["callbacks"],
        board(
            // #region callbacks
            stack![
                container(text(position))
                    .padding(spacing::SM)
                    .align_right(Length::Fill)
                    .align_bottom(Length::Fill),
                draggable(
                    state.note,
                    container(text("Note"))
                        .padding(spacing::MD)
                        .class(ContainerClass::Card),
                )
                .keyboard_step(16.0)
                .on_move(Message::Note)
                .on_drag_start(Message::Dragging(true))
                .on_drag_end(Message::Dragging(false)),
            ],
            // #endregion
        ),
    );

    let moves = [Move::Up, Move::Down, Move::Left, Move::Right]
        .map(|step| step.label(l10n))
        .join(", ");

    doc::page(
        "Draggable",
        "Content the user can move inside a stack with the pointer or the arrow keys.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`draggable` fills its parent, usually a layer of a `stack`, and draws its \
                 content with the top-left corner at the given position, clipped to its \
                 bounds, also in right-to-left text. The position lives in the \
                 application: a pointer drag publishes the new position at every move \
                 through `on_move`, as does each arrow key press while the content has the \
                 focus. Without `on_move` the content stays still and Tab skips it.",
            ),
            doc::code(ctx, "use oito2_iced::widget::draggable::draggable;"),
            doc::note(
                "The position is not limited to the parent: the content can be moved past \
                 its edges, where it is clipped. Clamp the position in `update` to keep it \
                 inside. Outside its content the draggable lets the pointer through to the \
                 layers below.",
            ),
            doc::heading("Examples"),
            doc::subheading("Reporting moves"),
            doc::prose(
                "`on_drag_start` and `on_drag_end` mark a pointer drag. `keyboard_step` sets \
                 how far one arrow key press moves the content, 8 pixels by default.",
            ),
            callbacks,
            doc::subheading("Move names"),
            doc::prose(
                "`Move` names the four moves in the language of the application, and `hint` \
                 tells that the arrow keys move the content, for menus or help text that \
                 offer the same moves:",
            ),
            doc::caption(moves),
            doc::caption(draggable::hint(l10n)),
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["`Tab`", "Moves the focus to the content."],
                    vec![
                        "Arrow keys",
                        "Move the content by `keyboard_step` in the direction of the arrow; \
                         holding a key repeats the move.",
                    ],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced::widget::draggable::draggable",
                    "oito2_iced::widget::draggable::Draggable",
                    "oito2_iced::widget::draggable::Move",
                    "oito2_iced::widget::draggable::hint",
                    "oito2_iced::widget::draggable::KEYBOARD_STEP",
                ],
            ),
        ],
    )
}
