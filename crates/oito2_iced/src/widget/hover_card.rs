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

//! Hover cards: a card of extra detail that opens over an element while the
//! pointer rests on it or the keyboard focus is inside it, such as a
//! profile preview on a user name.
//!
//! The card opens [`OPEN_DELAY`] after the pointer enters the anchor or the
//! focus moves inside it, and closes [`CLOSE_DELAY`] after the pointer
//! leaves both the anchor and the card and the focus leaves the anchor.
//! Escape and a mouse press outside the anchor and the card close it at
//! once. The card is on the popover surface, below the anchor, centered on
//! it and pointing at it with an arrow by default, at most [`MAX_WIDTH`]
//! wide. The card never takes the focus, so its content must only repeat
//! or preview what the anchor leads to.
//!
//! The delays run on the redraw requests of the window: the card asks for
//! redraws only while a change is pending.
//!
//! ```no_run
//! use iced::widget::{button, text};
//! use oito2_iced::widget::hover_card::hover_card;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     OpenProfile,
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! hover_card(
//!     button("Ada Lovelace").on_press(Message::OpenProfile),
//!     text("Mathematician, London"),
//! )
//! .into()
//! # }
//! ```

use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::{Event, Length, Rectangle, Size, Vector, mouse, window};

use super::popover::{Alignment, Placement, run_widget};
use crate::overlay::anchored::Anchored;
use crate::tokens::spacing;
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// The wait before the card opens, unless set with
/// [`HoverCard::open_delay`].
pub const OPEN_DELAY: Duration = Duration::from_millis(500);

/// The wait before the card closes, unless set with
/// [`HoverCard::close_delay`].
pub const CLOSE_DELAY: Duration = Duration::from_millis(300);

/// The largest width of the card, unless set with [`HoverCard::max_width`].
pub const MAX_WIDTH: f32 = 320.0;

/// An `anchor` that opens `card` while the pointer rests on it or the
/// focus is inside it.
pub fn hover_card<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    card: impl Into<Element<'a, Message>>,
) -> HoverCard<'a, Message> {
    HoverCard {
        anchored: crate::overlay::anchored(anchor, card)
            .menu()
            .padding(spacing::MD)
            .placement(Placement::Bottom)
            .alignment(Alignment::Center)
            .arrow(true)
            .max_width(MAX_WIDTH),
        open_delay: OPEN_DELAY,
        close_delay: CLOSE_DELAY,
    }
}

/// A hover card. See [`hover_card`].
pub struct HoverCard<'a, Message> {
    anchored: Anchored<'a, Message>,
    open_delay: Duration,
    close_delay: Duration,
}

impl<Message> HoverCard<'_, Message> {
    /// Sets the wait before the card opens ([`OPEN_DELAY`] by default).
    #[must_use]
    pub fn open_delay(mut self, delay: Duration) -> Self {
        self.open_delay = delay;
        self
    }

    /// Sets the wait before the card closes ([`CLOSE_DELAY`] by default).
    #[must_use]
    pub fn close_delay(mut self, delay: Duration) -> Self {
        self.close_delay = delay;
        self
    }

    /// Sets the preferred side (below by default).
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.anchored = self.anchored.placement(placement);
        self
    }

    /// Sets the alignment along the anchor edge (centered by default).
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

    /// Sets whether the card points at the anchor with an arrow (true by
    /// default).
    #[must_use]
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.anchored = self.anchored.arrow(arrow);
        self
    }

    /// Sets the largest width of the card ([`MAX_WIDTH`] by default).
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.anchored = self.anchored.max_width(width);
        self
    }
}

/// A change of the open state waiting for its delay.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Pending {
    /// Whether the change opens the card.
    open: bool,
    /// When the change applies, once a redraw request gave the time.
    at: Option<Instant>,
}

/// The state of a [`HoverCard`] in the widget tree.
#[derive(Debug, Default)]
struct State {
    open: bool,
    over_anchor: bool,
    over_card: bool,
    focused: bool,
    wanted: bool,
    pending: Option<Pending>,
}

impl State {
    /// Schedules opening or closing when the pointer or the focus entered or
    /// left, and cancels a pending change that no longer applies.
    fn schedule<Message>(&mut self, shell: &mut Shell<'_, Message>) {
        let wanted = self.over_anchor || self.over_card || self.focused;

        if wanted == self.wanted {
            return;
        }

        self.wanted = wanted;

        if wanted == self.open {
            self.pending = None;
        } else {
            self.pending = Some(Pending {
                open: wanted,
                at: None,
            });
            shell.request_redraw();
        }
    }

    /// Applies the pending change once its time has come at `now`, or asks
    /// for a redraw at that time.
    fn tick<Message>(
        &mut self,
        now: Instant,
        delays: (Duration, Duration),
        shell: &mut Shell<'_, Message>,
    ) {
        let Some(pending) = &mut self.pending else {
            return;
        };
        let delay = if pending.open { delays.0 } else { delays.1 };
        let at = *pending.at.get_or_insert(now + delay);

        if now >= at {
            self.open = pending.open;
            self.pending = None;
            shell.invalidate_layout();
            shell.request_redraw();
        } else {
            shell.request_redraw_at(at);
        }
    }

    /// Closes the card at once.
    fn close<Message>(&mut self, shell: &mut Shell<'_, Message>) {
        self.open = false;
        self.pending = None;
        self.over_card = false;
        shell.invalidate_layout();
        shell.request_redraw();
    }
}

impl<'a, Message: Clone + 'a> Widget<Message, Theme, iced::Renderer> for HoverCard<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(
            &self.anchored as &dyn Widget<Message, Theme, iced::Renderer>,
        )]
    }

    fn diff(&self, tree: &mut Tree) {
        if let Some(child) = tree.children.first_mut() {
            child.diff(&self.anchored as &dyn Widget<Message, Theme, iced::Renderer>);
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
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        match tree.children.first_mut() {
            Some(child) => self.anchored.layout(child, renderer, limits),
            None => layout::Node::new(Size::ZERO),
        }
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
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
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let Some(child) = children.first_mut() else {
            return;
        };

        self.anchored.update(
            child, event, layout, cursor, renderer, clipboard, shell, viewport,
        );

        if let Event::Mouse(_) = event {
            state.over_anchor = cursor.is_over(layout.bounds());
        }

        state.focused = run_widget(
            &mut self.anchored,
            child,
            layout,
            renderer,
            crate::focus::operations::has_focus(),
        )
        .unwrap_or(false);

        state.schedule(shell);

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.tick(*now, (self.open_delay, self.close_delay), shell);
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
        renderer: &iced::Renderer,
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
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let child = children.first_mut()?;
        let anchor = layout.bounds() + translation;

        self.anchored.set_open(state.open);

        let inner = self
            .anchored
            .overlay(child, layout, renderer, viewport, translation)?;

        if !state.open {
            return Some(inner);
        }

        Some(overlay::Element::new(Box::new(Card {
            inner,
            state,
            anchor,
        })))
    }
}

impl<'a, Message: Clone + 'a> From<HoverCard<'a, Message>> for Element<'a, Message> {
    fn from(card: HoverCard<'a, Message>) -> Self {
        Element::new(card)
    }
}

/// The open card of a [`HoverCard`]: keeps it open while hovered and closes
/// it on Escape and on a press outside.
struct Card<'b, Message> {
    inner: overlay::Element<'b, Message, Theme, iced::Renderer>,
    state: &'b mut State,
    anchor: Rectangle,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Card<'_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        self.inner.as_overlay_mut().layout(renderer, bounds)
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.inner
            .as_overlay()
            .draw(renderer, theme, style, layout, cursor);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.inner
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let bounds = layout.bounds();

        match event {
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            }) => {
                self.state.close(shell);
                shell.capture_event();
                return;
            }
            Event::Mouse(mouse::Event::ButtonPressed(_))
                if cursor.position().is_some_and(|point| {
                    !bounds.contains(point) && !self.anchor.contains(point)
                }) =>
            {
                self.state.close(shell);
                return;
            }
            _ => {}
        }

        self.inner
            .as_overlay_mut()
            .update(event, layout, cursor, renderer, clipboard, shell);

        if let Event::Mouse(_) = event {
            self.state.over_card = cursor.is_over(bounds);
            self.state.schedule(shell);
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.inner
            .as_overlay()
            .mouse_interaction(layout, cursor, renderer)
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
        self.inner.as_overlay_mut().overlay(layout, renderer)
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}
