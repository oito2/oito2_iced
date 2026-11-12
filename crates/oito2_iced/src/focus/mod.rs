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

//! Keyboard focus traversal.
//!
//! Iced only moves the focus when the application runs a focus operation.
//! [`navigation`] wraps the root of a view and runs
//! [`focus_next`](iced::advanced::widget::operation::focusable::focus_next)
//! on Tab and
//! [`focus_previous`](iced::advanced::widget::operation::focusable::focus_previous)
//! on Shift+Tab, over every [`focusable`] and text
//! input of the view, in layout order.
//!
//! [`ring`] draws the focus ring around a group while a widget inside it has
//! the focus; [`edge_interceptor`] moves the focus inside a group with the
//! arrow keys and stops at its ends; [`operations`] holds the focus
//! operations they use.

mod edge;
pub mod operations;
mod ring;

pub use crate::widget::{Focusable, focusable};
pub use edge::{EdgeInterceptor, edge_interceptor};
pub(crate) use ring::draw_ring;
pub use ring::{Ring, ring};

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation, Outcome};
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Element, Event, Length, Rectangle, Size, Vector, mouse};

/// Wraps the root of a view so Tab and Shift+Tab move the keyboard focus.
pub fn navigation<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Navigation<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Navigation {
        content: content.into(),
    }
}

/// The root wrapper that moves the focus with Tab. See [`navigation`].
pub struct Navigation<'a, Message, Theme = crate::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

/// Runs an operation, and the operations it chains, over `content`, and
/// returns its result.
pub(crate) fn run<T, Message, Theme, Renderer>(
    content: &mut Element<'_, Message, Theme, Renderer>,
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
        content.as_widget_mut().operate(
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

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Navigation<'_, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);

        layout::Node::with_children(content.size(), vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            operation,
        );
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

        if shell.is_event_captured() {
            return;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::Tab),
            modifiers,
            ..
        }) = event
        {
            let content = &mut self.content;
            let tree = &mut tree.children[0];

            if modifiers.shift() {
                let _ = run(
                    content,
                    tree,
                    content_layout,
                    renderer,
                    operation::focusable::focus_previous::<()>(),
                );
            } else {
                let _ = run(
                    content,
                    tree,
                    content_layout,
                    renderer,
                    operation::focusable::focus_next::<()>(),
                );
            }

            shell.capture_event();
            shell.request_redraw();
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
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
        layout
            .children()
            .next()
            .map(|content_layout| {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                )
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

impl<'a, Message, Theme, Renderer> From<Navigation<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(navigation: Navigation<'a, Message, Theme, Renderer>) -> Self {
        Element::new(navigation)
    }
}
