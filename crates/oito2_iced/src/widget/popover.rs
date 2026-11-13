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

//! A popover: a non-modal panel anchored to an element, such as a picker or
//! a short form.
//!
//! [`popover`] is an [`anchored`](crate::overlay::anchored()) panel on the
//! popover surface (the surface container, outlined, with the medium radius
//! and a shadow), with a medium padding, at most [`MAX_WIDTH`] wide, below
//! the anchor and aligned with its start by default. Every option of
//! [`Anchored`] applies.
//!
//! The application keeps whether the popover is open: a button in the
//! anchor typically toggles it, and the [`Popover::on_dismiss`] message,
//! published on Escape and on a mouse press outside the anchor and the
//! panel, closes it. Actions inside the panel close it by publishing a
//! message that sets the state to closed.
//!
//! When the panel opens, the first focusable element inside it takes the
//! keyboard focus (see [`Popover::focus_on_open`]) and Tab cycles through
//! the elements of the panel. When it closes with the focus inside, the
//! focus returns to the first focusable element of the anchor.
//!
//! ```no_run
//! use iced::widget::{button, text};
//! use oito2_iced::widget::popover::popover;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Toggle,
//!     Close,
//! }
//!
//! # fn view<'a>(open: bool) -> oito2_iced::Element<'a, Message> {
//! popover(
//!     button("Share").on_press(Message::Toggle),
//!     text("Anyone with the link can view."),
//! )
//! .open(open)
//! .on_dismiss(Message::Close)
//! .into()
//! # }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::focusable::{Count, count};
use iced::advanced::widget::operation::{self, Focusable, Operation, Outcome};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Element, Event, Length, Padding, Rectangle, Size, Vector, mouse};

use crate::Theme;
pub use crate::overlay::anchored::{Alignment, Anchored, Placement};
use crate::tokens::spacing;
use crate::widget::helpers::ReadingDirection;

/// The largest width of a popover panel, unless set with
/// [`Popover::max_width`].
pub const MAX_WIDTH: f32 = 360.0;

/// An `anchor` with a popover `panel` that opens next to it while open.
pub fn popover<'a, Message, Renderer>(
    anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
    panel: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Popover<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    Popover {
        anchored: crate::overlay::anchored(anchor, panel)
            .menu()
            .padding(spacing::MD)
            .max_width(MAX_WIDTH),
        open: false,
        focus_on_open: true,
        close_on_click_outside: true,
    }
}

/// A popover. See [`popover`].
pub struct Popover<'a, Message, Renderer = iced::Renderer> {
    anchored: Anchored<'a, Message, Renderer>,
    open: bool,
    focus_on_open: bool,
    close_on_click_outside: bool,
}

impl<Message, Renderer> Popover<'_, Message, Renderer> {
    /// Shows or hides the panel.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self.anchored = self.anchored.open(open);
        self
    }

    /// Sets the message published on Escape and, unless turned off with
    /// [`Popover::close_on_click_outside`], on a mouse press outside the
    /// panel and the anchor.
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.anchored = self.anchored.on_dismiss(message);
        self
    }

    /// Sets whether a mouse press outside the panel and the anchor
    /// publishes the dismiss message (true by default).
    #[must_use]
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Sets whether the first focusable element of the panel takes the
    /// focus when the panel opens (true by default).
    #[must_use]
    pub fn focus_on_open(mut self, focus: bool) -> Self {
        self.focus_on_open = focus;
        self
    }

    /// Sets the preferred side (below by default).
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.anchored = self.anchored.placement(placement);
        self
    }

    /// Sets the alignment along the anchor edge (the start by default).
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.anchored = self.anchored.alignment(alignment);
        self
    }

    /// Sets the reading direction that maps the start and end placements and
    /// alignments to physical sides.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.anchored = self.anchored.direction(direction);
        self
    }

    /// Sets the distance between the anchor and the panel or its arrow tip.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.anchored = self.anchored.gap(gap);
        self
    }

    /// Sets the space kept free at the edges of the window.
    #[must_use]
    pub fn margin(mut self, margin: f32) -> Self {
        self.anchored = self.anchored.margin(margin);
        self
    }

    /// Sets the padding inside the panel (the medium spacing by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.anchored = self.anchored.padding(padding);
        self
    }

    /// Draws an arrow from the panel to the anchor.
    #[must_use]
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.anchored = self.anchored.arrow(arrow);
        self
    }

    /// Sets the largest width of the panel ([`MAX_WIDTH`] by default).
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.anchored = self.anchored.max_width(width);
        self
    }

    /// Sets the largest height of the panel.
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.anchored = self.anchored.max_height(height);
        self
    }

    /// Gives a panel above or below the anchor the width of the anchor.
    #[must_use]
    pub fn match_anchor_width(mut self, matches: bool) -> Self {
        self.anchored = self.anchored.fit_anchor_width(matches);
        self
    }
}

/// A change of the open state that the focus still has to follow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Change {
    Opened,
    Closed,
}

/// The state of a [`Popover`] in the widget tree.
#[derive(Debug)]
struct State {
    open: bool,
    change: Option<Change>,
    focus_inside: bool,
}

impl<Message, Renderer> Popover<'_, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn inner(&self) -> &dyn Widget<Message, Theme, Renderer> {
        &self.anchored
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for Popover<'a, Message, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            open: self.open,
            change: self.open.then_some(Change::Opened),
            focus_inside: false,
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.inner())]
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();

        if state.open != self.open {
            state.open = self.open;
            state.change = Some(if self.open {
                Change::Opened
            } else {
                Change::Closed
            });
        }

        if let Some(child) = tree.children.first_mut() {
            child.diff(self.inner());
        }
    }

    fn size(&self) -> Size<Length> {
        self.anchored.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.anchored.size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let Some(child) = children.first_mut() else {
            return layout::Node::new(Size::ZERO);
        };

        let node = self.anchored.layout(child, renderer, limits);
        let layout = Layout::new(&node);

        match state.change {
            Some(Change::Opened) if self.focus_on_open => {
                let _ = run_widget(&mut self.anchored, child, layout, renderer, unfocus::<()>());
            }
            Some(Change::Closed) => {
                if state.focus_inside {
                    let _ = run_widget(
                        &mut self.anchored,
                        child,
                        layout,
                        renderer,
                        crate::focus::operations::focus_first::<()>(),
                    );
                }

                state.focus_inside = false;
                state.change = None;
            }
            _ => {}
        }

        node
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.anchored.operate(child, layout, renderer, operation);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Some(child) = tree.children.first_mut() {
            self.anchored.update(
                child, event, layout, cursor, renderer, clipboard, shell, viewport,
            );
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        if let Some(child) = tree.children.first() {
            self.anchored
                .draw(child, renderer, theme, style, layout, cursor, viewport);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        tree.children
            .first()
            .map(|child| {
                self.anchored
                    .mouse_interaction(child, layout, cursor, viewport, renderer)
            })
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let child = children.first_mut()?;
        let anchor = layout.bounds() + translation;

        let inner = self
            .anchored
            .overlay(child, layout, renderer, viewport, translation)?;

        if !self.open {
            return Some(inner);
        }

        Some(overlay::Element::new(Box::new(Panel {
            inner,
            state,
            anchor,
            focus_on_open: self.focus_on_open,
            close_on_click_outside: self.close_on_click_outside,
        })))
    }
}

impl<'a, Message, Renderer> From<Popover<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(popover: Popover<'a, Message, Renderer>) -> Self {
        Element::new(popover)
    }
}

/// The open panel of a [`Popover`]: moves the focus into it once it opens,
/// keeps Tab inside it and records whether it holds the focus.
struct Panel<'b, Message, Renderer> {
    inner: overlay::Element<'b, Message, Theme, Renderer>,
    state: &'b mut State,
    anchor: Rectangle,
    focus_on_open: bool,
    close_on_click_outside: bool,
}

impl<Message, Renderer> overlay::Overlay<Message, Theme, Renderer> for Panel<'_, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let node = self.inner.as_overlay_mut().layout(renderer, bounds);

        if self.state.change == Some(Change::Opened) {
            self.state.change = None;
            let layout = Layout::new(&node);

            if self.focus_on_open {
                let _ = run_overlay(
                    &mut self.inner,
                    layout,
                    renderer,
                    crate::focus::operations::focus_first::<()>(),
                );
                self.state.focus_inside = run_overlay(
                    &mut self.inner,
                    layout,
                    renderer,
                    crate::focus::operations::has_focus(),
                )
                .unwrap_or(false);
            } else {
                let _ = run_overlay(&mut self.inner, layout, renderer, unfocus::<()>());
                self.state.focus_inside = false;
            }
        }

        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.inner
            .as_overlay()
            .draw(renderer, theme, style, layout, cursor);
    }

    fn operate(&mut self, layout: Layout<'_>, renderer: &Renderer, operation: &mut dyn Operation) {
        self.inner
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        if !self.close_on_click_outside
            && let Event::Mouse(mouse::Event::ButtonPressed(_)) = event
            && cursor.position().is_some_and(|point| {
                !layout.bounds().contains(point) && !self.anchor.contains(point)
            })
        {
            return;
        }

        if self.state.focus_inside
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Tab),
                modifiers,
                ..
            }) = event
        {
            let _ = run_overlay(
                &mut self.inner,
                layout,
                renderer,
                cycle::<()>(modifiers.shift()),
            );
            shell.capture_event();
            shell.request_redraw();
            return;
        }

        self.inner
            .as_overlay_mut()
            .update(event, layout, cursor, renderer, clipboard, shell);

        self.state.focus_inside = run_overlay(
            &mut self.inner,
            layout,
            renderer,
            crate::focus::operations::has_focus(),
        )
        .unwrap_or(false);
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.inner
            .as_overlay()
            .mouse_interaction(layout, cursor, renderer)
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        self.inner.as_overlay_mut().overlay(layout, renderer)
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}

/// Runs an operation, and the operations it chains, over `widget`, and
/// returns its result.
pub(crate) fn run_widget<T, Message, Renderer>(
    widget: &mut dyn Widget<Message, Theme, Renderer>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &Renderer,
    operation: impl Operation<T> + 'static,
) -> Option<T>
where
    Renderer: renderer::Renderer,
{
    let mut operation: Box<dyn Operation<T>> = Box::new(operation);

    loop {
        widget.operate(
            tree,
            layout,
            renderer,
            &mut operation::black_box(operation.as_mut()),
        );

        match operation.finish() {
            Outcome::Chain(next) => operation = next,
            Outcome::Some(result) => return Some(result),
            Outcome::None => return None,
        }
    }
}

/// Runs an operation, and the operations it chains, over an overlay, and
/// returns its result.
pub(crate) fn run_overlay<T, Message, Renderer>(
    overlay: &mut overlay::Element<'_, Message, Theme, Renderer>,
    layout: Layout<'_>,
    renderer: &Renderer,
    operation: impl Operation<T> + 'static,
) -> Option<T>
where
    Renderer: renderer::Renderer,
{
    let mut operation: Box<dyn Operation<T>> = Box::new(operation);

    loop {
        overlay.as_overlay_mut().operate(
            layout,
            renderer,
            &mut operation::black_box(operation.as_mut()),
        );

        match operation.finish() {
            Outcome::Chain(next) => operation = next,
            Outcome::Some(result) => return Some(result),
            Outcome::None => return None,
        }
    }
}

/// Focuses the focusable widget at `target`, in layout order, and
/// unfocuses every other one.
struct FocusAt {
    target: usize,
    current: usize,
}

impl<T> Operation<T> for FocusAt {
    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        if self.current == self.target {
            state.focus();
        } else {
            state.unfocus();
        }

        self.current += 1;
    }

    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
        operate(self);
    }
}

/// Unfocuses every focusable widget.
pub(crate) fn unfocus<T>() -> impl Operation<T> {
    FocusAt {
        target: usize::MAX,
        current: 0,
    }
}

/// Moves the focus to the next focusable widget, or to the previous one
/// when `backward` is true, wrapping around at the ends.
fn cycle<T>(backward: bool) -> impl Operation<T>
where
    T: Send + 'static,
{
    let choose: fn(Count) -> FocusAt = if backward {
        |count| FocusAt {
            target: match count.focused {
                Some(focused) => (focused + count.total - 1) % count.total.max(1),
                None => count.total.saturating_sub(1),
            },
            current: 0,
        }
    } else {
        |count| FocusAt {
            target: match count.focused {
                Some(focused) => (focused + 1) % count.total.max(1),
                None => 0,
            },
            current: 0,
        }
    };

    operation::then(count(), choose)
}
