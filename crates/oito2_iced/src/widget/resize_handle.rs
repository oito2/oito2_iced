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

//! Draggable handles that resize panes, and the widget that lays panes out
//! along an axis with a one-pixel divider and a resize handle between each
//! two.
//!
//! A handle is [`spacing::SM`] thick across the axis, centered on its
//! divider, and fills the other direction. It shows the resize cursor while
//! hovered or dragged, takes part in focus traversal, moves by a keyboard
//! step on the arrow keys of its axis and reports Enter, Space, a double
//! click, Home and End. Its line is drawn in the primary text color while
//! hovered, dragged or focused, two pixels thick while focused, and in the
//! `outline_variant` role otherwise.

use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse::click;
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay};
use iced::keyboard::{self, key::Named};
use iced::mouse;
use iced::{Color, Event, Length, Point, Rectangle, Size, Vector, window};

use super::splitter::Axis;
use crate::tokens::spacing;
use crate::{Element, Theme};

/// Thickness of the divider between two panes.
pub(crate) const DIVIDER: f32 = 1.0;

/// Thickness of the handle line while focused.
const FOCUSED_LINE: f32 = 2.0;

/// What a handle asks its owner to do.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Request {
    /// Moves the divider by the given pixels toward the end of the axis in
    /// screen coordinates: right for a horizontal axis, down for a vertical
    /// one.
    Move(f32),
    /// Collapses or restores a pane: Enter, Space or a double click.
    Toggle,
    /// Moves the divider to the start limit: the Home key.
    Home,
    /// Moves the divider to the end limit: the End key.
    End,
}

/// How the owner of a handle handled a [`Request`].
pub(crate) enum Outcome<Message> {
    /// The request does not apply; a key press stays available to others.
    Ignored,
    /// The request applies but changed nothing.
    Unchanged,
    /// The request changed the sizes; the message reports them.
    Changed(Message),
}

/// The interaction state of one handle.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct Handle {
    focused: bool,
    drag: Option<f32>,
    last_click: Option<click::Click>,
    drawn: Option<(bool, bool)>,
}

impl operation::Focusable for Handle {
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

/// The coordinate of `point` along `axis`.
fn along(axis: Axis, point: Point) -> f32 {
    match axis {
        Axis::Horizontal => point.x,
        Axis::Vertical => point.y,
    }
}

impl Handle {
    /// Whether the line is highlighted: hovered, dragged or focused.
    fn highlighted(&self, bounds: Rectangle, cursor: mouse::Cursor) -> bool {
        self.focused || self.drag.is_some() || cursor.is_over(bounds)
    }

    /// Whether a drag is in progress.
    pub(crate) fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }

    /// Removes the focus and ends any drag.
    pub(crate) fn reset(&mut self) {
        self.focused = false;
        self.drag = None;
    }

    /// Handles `event` for a handle at `bounds` moving along `axis`.
    ///
    /// A press on the handle starts a drag and captures the event; while
    /// dragging, every pointer movement is captured and reported as a
    /// [`Request::Move`] until the button is released, wherever the pointer
    /// is. A press elsewhere removes the focus. Key presses on the focused
    /// handle become requests and are left for the owner to capture.
    pub(crate) fn update<Message>(
        &mut self,
        axis: Axis,
        bounds: Rectangle,
        keyboard_step: f32,
        event: &Event,
        cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) -> Option<Request> {
        let request = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_over(bounds) {
                    self.drag = Some(along(axis, position));
                    let click = click::Click::new(position, mouse::Button::Left, self.last_click);
                    self.last_click = Some(click);
                    shell.capture_event();

                    (click.kind() == click::Kind::Double).then_some(Request::Toggle)
                } else {
                    self.focused = false;
                    None
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                if !cursor.is_over(bounds) {
                    self.focused = false;
                }
                None
            }
            Event::Mouse(mouse::Event::CursorMoved { position }) => match self.drag {
                Some(last) => {
                    let current = along(axis, *position);
                    self.drag = Some(current);
                    shell.capture_event();

                    (current != last).then_some(Request::Move(current - last))
                }
                None => None,
            },
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if self.drag.take().is_some() {
                    shell.capture_event();
                }
                None
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if self.focused && modifiers.is_empty() && !shell.is_event_captured() => {
                key_request(axis, *named, keyboard_step)
            }
            _ => None,
        };

        let drawn = (self.highlighted(bounds, cursor), self.focused);

        if let Event::Window(window::Event::RedrawRequested(_)) = event {
            self.drawn = Some(drawn);
        } else if self.drawn.is_some_and(|previous| previous != drawn) {
            shell.request_redraw();
        }

        request
    }
}

/// The request of the named key on a focused handle moving along `axis`.
fn key_request(axis: Axis, key: Named, step: f32) -> Option<Request> {
    match (axis, key) {
        (Axis::Horizontal, Named::ArrowRight) | (Axis::Vertical, Named::ArrowDown) => {
            Some(Request::Move(step))
        }
        (Axis::Horizontal, Named::ArrowLeft) | (Axis::Vertical, Named::ArrowUp) => {
            Some(Request::Move(-step))
        }
        (_, Named::Enter | Named::Space) => Some(Request::Toggle),
        (_, Named::Home) => Some(Request::Home),
        (_, Named::End) => Some(Request::End),
        _ => None,
    }
}

/// The resize cursor of a handle moving along `axis`.
pub(crate) fn interaction(axis: Axis) -> mouse::Interaction {
    match axis {
        Axis::Horizontal => mouse::Interaction::ResizingHorizontally,
        Axis::Vertical => mouse::Interaction::ResizingVertically,
    }
}

/// A one-pixel divider starting at `start` along `axis`, across `bounds`.
fn divider_bounds(axis: Axis, bounds: Rectangle, start: f32, thickness: f32) -> Rectangle {
    match axis {
        Axis::Horizontal => Rectangle::new(
            Point::new(bounds.x + start, bounds.y),
            Size::new(thickness, bounds.height),
        ),
        Axis::Vertical => Rectangle::new(
            Point::new(bounds.x, bounds.y + start),
            Size::new(bounds.width, thickness),
        ),
    }
}

/// The handle of the divider starting at `start`: [`spacing::SM`] thick,
/// centered on the divider.
fn handle_bounds(axis: Axis, bounds: Rectangle, start: f32) -> Rectangle {
    divider_bounds(
        axis,
        bounds,
        start + DIVIDER / 2.0 - spacing::SM / 2.0,
        spacing::SM,
    )
}

/// Decides the sizes of the panes of a [`Split`] and applies the requests
/// of its handles.
pub(crate) trait Sizing<Message> {
    /// The size of each child along the axis, in logical order, for an
    /// `extent` along the axis that includes the dividers.
    fn sizes(&mut self, extent: f32) -> Vec<f32>;

    /// Whether the child at `index` is hidden: laid out empty, neither drawn
    /// nor reachable.
    fn hidden(&self, _index: usize) -> bool {
        false
    }

    /// Applies `request` to the divider after the child at `divider`, with
    /// moves already in logical direction (toward the last child).
    fn apply(&mut self, divider: usize, request: Request) -> Outcome<Message>;
}

/// Panes along an axis with a divider and a resize handle between each two.
pub(crate) struct Split<'a, Message> {
    children: Vec<Element<'a, Message>>,
    axis: Axis,
    mirrored: bool,
    resizable: bool,
    keyboard_step: f32,
    sizing: Box<dyn Sizing<Message> + 'a>,
    dividers: Vec<f32>,
    overflows: bool,
}

/// The widget state of a [`Split`]: one [`Handle`] per divider.
#[derive(Debug, Default)]
struct State {
    handles: Vec<Handle>,
}

impl<'a, Message> Split<'a, Message> {
    /// Lays `children` out along `axis` with sizes from `sizing`. With
    /// `mirrored`, a horizontal split runs from right to left. Without
    /// `resizable`, the dividers have no handle.
    pub(crate) fn new(
        children: Vec<Element<'a, Message>>,
        axis: Axis,
        mirrored: bool,
        resizable: bool,
        keyboard_step: f32,
        sizing: impl Sizing<Message> + 'a,
    ) -> Self {
        Self {
            children,
            axis,
            mirrored: mirrored && axis == Axis::Horizontal,
            resizable,
            keyboard_step,
            sizing: Box::new(sizing),
            dividers: Vec::new(),
            overflows: false,
        }
    }

    /// The handle bounds of every divider inside `bounds`.
    fn handles(&self, bounds: Rectangle) -> impl Iterator<Item = Rectangle> + '_ {
        self.dividers
            .iter()
            .map(move |start| handle_bounds(self.axis, bounds, *start))
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Split<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            handles: vec![Handle::default(); self.children.len().saturating_sub(1)],
        })
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
        tree.state
            .downcast_mut::<State>()
            .handles
            .resize(self.children.len().saturating_sub(1), Handle::default());
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
        let size = limits.resolve(Length::Fill, Length::Fill, Size::ZERO);
        let (extent, cross) = match self.axis {
            Axis::Horizontal => (size.width, size.height),
            Axis::Vertical => (size.height, size.width),
        };
        let sizes = self.sizing.sizes(extent);
        let count = self.children.len();

        self.dividers.clear();
        let mut position = 0.0;
        let mut nodes = Vec::with_capacity(count);

        for (index, (child, child_tree)) in
            self.children.iter_mut().zip(&mut tree.children).enumerate()
        {
            let length = sizes.get(index).copied().unwrap_or(0.0).max(0.0);
            let hidden = self.sizing.hidden(index);
            let max = if hidden {
                Size::ZERO
            } else {
                match self.axis {
                    Axis::Horizontal => Size::new(length, cross),
                    Axis::Vertical => Size::new(cross, length),
                }
            };
            let node = child.as_widget_mut().layout(
                child_tree,
                renderer,
                &layout::Limits::new(Size::ZERO, max),
            );
            let start = if self.mirrored {
                extent - position - length
            } else {
                position
            };
            let origin = match self.axis {
                Axis::Horizontal => Point::new(start, 0.0),
                Axis::Vertical => Point::new(0.0, start),
            };
            nodes.push(node.move_to(origin));
            position += length;

            if index + 1 < count {
                self.dividers.push(if self.mirrored {
                    extent - position - DIVIDER
                } else {
                    position
                });
                position += DIVIDER;
            }
        }

        self.overflows = position > extent + 0.01;

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let bounds = layout.bounds();
        operation.container(None, bounds);

        let handles: Vec<Rectangle> = self.handles(bounds).collect();
        let resizable = self.resizable;
        let sizing = &self.sizing;
        let children = &mut self.children;
        let (state, trees) = (&mut tree.state, &mut tree.children);

        operation.traverse(&mut |operation| {
            for (index, ((child, child_tree), child_layout)) in children
                .iter_mut()
                .zip(trees.iter_mut())
                .zip(layout.children())
                .enumerate()
            {
                if !sizing.hidden(index) {
                    child
                        .as_widget_mut()
                        .operate(child_tree, child_layout, renderer, operation);
                }
            }

            for (handle, bounds) in state
                .downcast_mut::<State>()
                .handles
                .iter_mut()
                .zip(&handles)
            {
                if resizable {
                    operation.focusable(None, *bounds, handle);
                } else {
                    handle.reset();
                }
            }
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
        let bounds = layout.bounds();
        let mut on_handle = false;

        if self.resizable {
            let handles: Vec<Rectangle> = self.handles(bounds).collect();
            let state = tree.state.downcast_mut::<State>();

            for (divider, (handle, handle_bounds)) in
                state.handles.iter_mut().zip(handles).enumerate()
            {
                let request = handle.update(
                    self.axis,
                    handle_bounds,
                    self.keyboard_step,
                    event,
                    cursor,
                    shell,
                );
                on_handle |= handle.is_dragging() || cursor.is_over(handle_bounds);

                let Some(request) = request else {
                    continue;
                };
                let request = match request {
                    Request::Move(delta) if self.mirrored => Request::Move(-delta),
                    request => request,
                };

                match self.sizing.apply(divider, request) {
                    Outcome::Ignored => {}
                    Outcome::Unchanged => shell.capture_event(),
                    Outcome::Changed(message) => {
                        shell.publish(message);
                        shell.capture_event();
                    }
                }
            }
        }

        if shell.is_event_captured() {
            return;
        }

        let cursor = if on_handle {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };

        for (index, ((child, child_tree), child_layout)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
        {
            if self.sizing.hidden(index) {
                continue;
            }

            child.as_widget_mut().update(
                child_tree,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
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
        let bounds = layout.bounds();
        let roles = theme.roles();
        let state = tree.state.downcast_ref::<State>();
        let dragging = state.handles.iter().any(Handle::is_dragging);
        let on_handle = self.resizable && self.handles(bounds).any(|h| cursor.is_over(h));
        let child_cursor = if dragging || on_handle {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };

        let draw = |renderer: &mut iced::Renderer| {
            for (index, ((child, child_tree), child_layout)) in self
                .children
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .enumerate()
            {
                if self.sizing.hidden(index) {
                    continue;
                }

                child.as_widget().draw(
                    child_tree,
                    renderer,
                    theme,
                    style,
                    child_layout,
                    child_cursor,
                    viewport,
                );
            }

            for (divider, start) in self.dividers.iter().enumerate() {
                let handle = state.handles.get(divider).copied().unwrap_or_default();
                let handle_bounds = handle_bounds(self.axis, bounds, *start);
                let highlighted = self.resizable && handle.highlighted(handle_bounds, cursor);
                let focused = self.resizable && handle.focused;

                fill(
                    renderer,
                    divider_bounds(self.axis, bounds, *start, DIVIDER),
                    roles.outline_variant,
                );

                if highlighted {
                    let thickness = if focused { FOCUSED_LINE } else { DIVIDER };
                    let line_start = *start + DIVIDER / 2.0 - thickness / 2.0;
                    fill(
                        renderer,
                        divider_bounds(self.axis, bounds, line_start, thickness),
                        roles.semantic.primary_text,
                    );
                }
            }
        };

        if self.overflows {
            iced::advanced::Renderer::with_layer(renderer, bounds, draw);
        } else {
            draw(renderer);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();

        if self.resizable {
            let state = tree.state.downcast_ref::<State>();
            let dragging = state.handles.iter().any(Handle::is_dragging);

            if dragging || self.handles(bounds).any(|handle| cursor.is_over(handle)) {
                return interaction(self.axis);
            }
        }

        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .enumerate()
            .filter(|(index, _)| !self.sizing.hidden(*index))
            .map(|(_, ((child, child_tree), child_layout))| {
                child.as_widget().mouse_interaction(
                    child_tree,
                    child_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        overlay::from_children(
            &mut self.children,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

/// Fills `bounds` with `color`.
fn fill(renderer: &mut iced::Renderer, bounds: Rectangle, color: Color) {
    iced::advanced::Renderer::fill_quad(
        renderer,
        Quad {
            bounds,
            ..Quad::default()
        },
        color,
    );
}

impl<'a, Message: 'a> From<Split<'a, Message>> for Element<'a, Message> {
    fn from(split: Split<'a, Message>) -> Self {
        Element::new(split)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrow_keys_follow_the_axis() {
        assert_eq!(
            key_request(Axis::Horizontal, Named::ArrowRight, 16.0),
            Some(Request::Move(16.0))
        );
        assert_eq!(
            key_request(Axis::Horizontal, Named::ArrowLeft, 16.0),
            Some(Request::Move(-16.0))
        );
        assert_eq!(key_request(Axis::Horizontal, Named::ArrowDown, 16.0), None);
        assert_eq!(
            key_request(Axis::Vertical, Named::ArrowDown, 8.0),
            Some(Request::Move(8.0))
        );
        assert_eq!(
            key_request(Axis::Vertical, Named::ArrowUp, 8.0),
            Some(Request::Move(-8.0))
        );
        assert_eq!(
            key_request(Axis::Vertical, Named::Enter, 8.0),
            Some(Request::Toggle)
        );
        assert_eq!(
            key_request(Axis::Vertical, Named::Home, 8.0),
            Some(Request::Home)
        );
    }

    #[test]
    fn the_handle_is_centered_on_the_divider() {
        let bounds = Rectangle::new(Point::new(10.0, 0.0), Size::new(300.0, 100.0));
        let handle = handle_bounds(Axis::Horizontal, bounds, 100.0);

        assert_eq!(handle.width, spacing::SM);
        assert_eq!(handle.center_x(), 10.0 + 100.0 + DIVIDER / 2.0);
        assert_eq!(handle.height, 100.0);
    }
}
