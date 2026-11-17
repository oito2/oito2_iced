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

//! Draggable: content the user moves with the pointer or, once focused,
//! with the arrow keys.
//!
//! [`draggable`] fills its parent, usually a layer of a
//! [`stack`](iced::widget::stack()), and draws its content with the top-left
//! corner at `position`, clipped to its bounds. The position lives in the
//! application: a pointer drag publishes the new position at every move
//! through [`Draggable::on_move`], as does each arrow key press while the
//! content has the keyboard focus (it moves by
//! [`Draggable::keyboard_step`]). Without [`Draggable::on_move`] the content
//! stays still and is skipped by Tab.
//!
//! Outside its content the draggable lets the pointer through to the
//! layers below. [`Move`] and [`hint`] give the localized names of the
//! moves and the hint that the arrow keys move the content, for menus or
//! help text that offer the same moves.
//!
//! ```no_run
//! use iced::Point;
//! use iced::widget::{container, stack};
//! use oito2_iced::widget::draggable;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Moved(Point),
//! }
//!
//! # fn view<'a>(note: Point) -> oito2_iced::Element<'a, Message> {
//! stack![
//!     container("Board").width(400).height(300),
//!     draggable::draggable(note, container("Note").padding(8)).on_move(Message::Moved),
//! ]
//! .into()
//! # }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Element as IcedElement, Event, Length, Point, Rectangle, Size, Vector, mouse, touch};

use crate::l10n::Localizations;
use crate::tokens::radius;
use crate::{Element, Theme};

/// The default distance one arrow key press moves the content.
pub const KEYBOARD_STEP: f32 = 8.0;

/// How far a mouse moves before a press becomes a drag (the precise
/// pointer pan slop of the OITO2 design).
const MOUSE_SLOP: f32 = 2.0;

/// How far a finger moves before a touch becomes a drag (the touch pan
/// slop of the OITO2 design).
const TOUCH_SLOP: f32 = 36.0;

/// A move of one keyboard step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    /// Up.
    Up,
    /// Down.
    Down,
    /// Left.
    Left,
    /// Right.
    Right,
}

impl Move {
    /// Every move, in the order up, down, left, right.
    pub const ALL: [Move; 4] = [Move::Up, Move::Down, Move::Left, Move::Right];

    /// The localized name of the move, such as "Move up".
    pub fn label(self, localizations: &Localizations) -> &'static str {
        match self {
            Move::Up => localizations.move_up,
            Move::Down => localizations.move_down,
            Move::Left => localizations.move_left,
            Move::Right => localizations.move_right,
        }
    }

    /// The offset of the move by `step` pixels.
    pub fn offset(self, step: f32) -> Vector {
        match self {
            Move::Up => Vector::new(0.0, -step),
            Move::Down => Vector::new(0.0, step),
            Move::Left => Vector::new(-step, 0.0),
            Move::Right => Vector::new(step, 0.0),
        }
    }

    /// The move of an arrow key, if `key` is one.
    fn of(key: &keyboard::Key) -> Option<Move> {
        match key {
            keyboard::Key::Named(key::Named::ArrowUp) => Some(Move::Up),
            keyboard::Key::Named(key::Named::ArrowDown) => Some(Move::Down),
            keyboard::Key::Named(key::Named::ArrowLeft) => Some(Move::Left),
            keyboard::Key::Named(key::Named::ArrowRight) => Some(Move::Right),
            _ => None,
        }
    }
}

/// The localized hint that the arrow keys move the content.
pub fn hint(localizations: &Localizations) -> &'static str {
    localizations.move_with_arrow_keys
}

/// Content the user moves. See [`draggable`].
pub struct Draggable<'a, Message> {
    position: Point,
    content: Element<'a, Message>,
    on_move: Option<Box<dyn Fn(Point) -> Message + 'a>>,
    on_drag_start: Option<Message>,
    on_drag_end: Option<Message>,
    keyboard_step: f32,
    id: Option<Id>,
}

/// Draws `content` with its top-left corner at `position`, relative to the
/// top-left corner of the draggable. Without [`Draggable::on_move`] it
/// does not move.
pub fn draggable<'a, Message>(
    position: Point,
    content: impl Into<Element<'a, Message>>,
) -> Draggable<'a, Message> {
    Draggable {
        position,
        content: content.into(),
        on_move: None,
        on_drag_start: None,
        on_drag_end: None,
        keyboard_step: KEYBOARD_STEP,
        id: None,
    }
}

impl<'a, Message> Draggable<'a, Message> {
    /// Sets the function that turns a new position into a message, which
    /// lets the user move the content.
    #[must_use]
    pub fn on_move(mut self, on_move: impl Fn(Point) -> Message + 'a) -> Self {
        self.on_move = Some(Box::new(on_move));
        self
    }

    /// Sets the move function, or keeps the content still with `None`.
    #[must_use]
    pub fn on_move_maybe(mut self, on_move: Option<impl Fn(Point) -> Message + 'a>) -> Self {
        self.on_move = on_move.map(|f| Box::new(f) as Box<dyn Fn(Point) -> Message + 'a>);
        self
    }

    /// Sets the message published when a pointer drag starts.
    #[must_use]
    pub fn on_drag_start(mut self, message: Message) -> Self {
        self.on_drag_start = Some(message);
        self
    }

    /// Sets the message published when a pointer drag ends, also when the
    /// pointer leaves the window mid-drag.
    #[must_use]
    pub fn on_drag_end(mut self, message: Message) -> Self {
        self.on_drag_end = Some(message);
        self
    }

    /// Sets how far one arrow key press moves the content, in logical
    /// pixels ([`KEYBOARD_STEP`] by default).
    #[must_use]
    pub fn keyboard_step(mut self, step: f32) -> Self {
        self.keyboard_step = step;
        self
    }

    /// Sets the widget [`Id`], used to focus the content programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// A pointer press on the content, which becomes a drag past the slop.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Drag {
    /// Where the pointer was pressed.
    origin: Point,
    /// The position of the content at the press.
    start: Point,
    dragging: bool,
    touch: bool,
}

/// The tree state of a [`Draggable`].
#[derive(Debug, Clone, Copy, Default)]
struct State {
    focused: bool,
    drag: Option<Drag>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl<Message: Clone> Draggable<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_move.is_some()
    }

    /// Publishes the end of a drag in progress and forgets the press.
    fn end(&self, state: &mut State, shell: &mut Shell<'_, Message>) {
        if let Some(drag) = state.drag.take()
            && drag.dragging
            && let Some(message) = &self.on_drag_end
        {
            shell.publish(message.clone());
        }
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Draggable<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &limits.loose())
            .move_to(self.position);
        let reach = content.bounds();
        let size = limits.resolve(
            Length::Fill,
            Length::Fill,
            Size::new(
                (reach.x + reach.width).max(0.0),
                (reach.y + reach.height).max(0.0),
            ),
        );

        layout::Node::with_children(size, vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };
        let state = tree.state.downcast_mut::<State>();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), content_layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), content_layout.bounds());
        }

        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
        });
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let state = tree.state.downcast_mut::<State>();
        let content = content_layout.bounds();

        let Some(on_move) = &self.on_move else {
            state.focused = false;
            state.drag = None;
            return;
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let touch = matches!(event, Event::Touch(_));

                match cursor.position_over(content) {
                    Some(origin) if !shell.is_event_captured() => {
                        state.drag = Some(Drag {
                            origin,
                            start: self.position,
                            dragging: false,
                            touch,
                        });
                        shell.capture_event();
                    }
                    Some(_) => {}
                    None => {
                        if state.focused {
                            state.focused = false;
                            shell.request_redraw();
                        }
                    }
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. })
            | Event::Touch(touch::Event::FingerMoved { .. }) => {
                let Some(drag) = &mut state.drag else {
                    return;
                };
                let Some(position) = cursor.position() else {
                    return;
                };
                let delta = position - drag.origin;

                if !drag.dragging {
                    let slop = if drag.touch { TOUCH_SLOP } else { MOUSE_SLOP };

                    if delta.x.hypot(delta.y) <= slop {
                        return;
                    }

                    drag.dragging = true;

                    if let Some(message) = &self.on_drag_start {
                        shell.publish(message.clone());
                    }
                }

                shell.publish(on_move(drag.start + delta));
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                if state.drag.is_some_and(|drag| drag.dragging) {
                    shell.capture_event();
                }
                self.end(state, shell);
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, .. })
                if state.focused && !shell.is_event_captured() =>
            {
                if let Some(step) = Move::of(key) {
                    shell.publish(on_move(self.position + step.offset(self.keyboard_step)));
                    shell.capture_event();
                }
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        renderer.with_layer(bounds, |renderer| {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );

            if state.focused && self.enabled() {
                crate::focus::draw_ring(renderer, theme, content_layout.bounds(), radius::MEDIUM);
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let Some(content_layout) = layout.children().next() else {
            return mouse::Interaction::None;
        };
        let state = tree.state.downcast_ref::<State>();

        if self.enabled() && state.drag.is_some_and(|drag| drag.dragging) {
            return mouse::Interaction::Grabbing;
        }

        if !cursor.is_over(content_layout.bounds()) {
            return mouse::Interaction::None;
        }

        let inner = self.content.as_widget().mouse_interaction(
            &tree.children[0],
            content_layout,
            cursor,
            viewport,
            renderer,
        );

        match inner {
            mouse::Interaction::None | mouse::Interaction::Idle if self.enabled() => {
                mouse::Interaction::Grab
            }
            mouse::Interaction::None => mouse::Interaction::Idle,
            other => other,
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let content_layout = layout.children().next()?;

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: Clone + 'a> From<Draggable<'a, Message>> for Element<'a, Message> {
    fn from(draggable: Draggable<'a, Message>) -> Self {
        IcedElement::new(draggable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn moves_name_and_offset_each_direction() {
        let strings = Localizations::en();

        assert_eq!(Move::Up.label(&strings), "Move up");
        assert_eq!(Move::Right.offset(8.0), Vector::new(8.0, 0.0));
        assert_eq!(Move::Up.offset(8.0), Vector::new(0.0, -8.0));
        assert_eq!(
            hint(&Localizations::pt_br()),
            "Use as setas do teclado para mover"
        );
    }
}
