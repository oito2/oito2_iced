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

//! Arrow-key focus navigation inside a group of widgets.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Element, Event, Length, Rectangle, Size, Vector, mouse};

use super::operations::{focus_next_clamped, focus_previous_clamped, has_focus};
use super::run;

/// Moves the keyboard focus inside `content` with the arrow keys and stops
/// at the ends of the group instead of leaving it.
///
/// Down and Right move to the next focusable widget, Up and Left to the
/// previous one, while a widget inside the group has the focus. Tab and
/// Shift+Tab still move to the widgets around the group. Useful for
/// toolbars and grids navigated with the arrow keys.
pub fn edge_interceptor<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> EdgeInterceptor<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    EdgeInterceptor {
        content: content.into(),
    }
}

/// Arrow-key focus navigation inside a group. See [`edge_interceptor`].
pub struct EdgeInterceptor<'a, Message, Theme = crate::Theme, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

/// The direction an arrow key moves the focus in.
enum Step {
    Next,
    Previous,
}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for EdgeInterceptor<'_, Message, Theme, Renderer>
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
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

        let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(named),
            modifiers,
            ..
        }) = event
        else {
            return;
        };

        if !modifiers.is_empty() {
            return;
        }

        let step = match named {
            key::Named::ArrowDown | key::Named::ArrowRight => Step::Next,
            key::Named::ArrowUp | key::Named::ArrowLeft => Step::Previous,
            _ => return,
        };

        let content = &mut self.content;
        let tree = &mut tree.children[0];

        if run(content, tree, content_layout, renderer, has_focus()) != Some(true) {
            return;
        }

        let _ = match step {
            Step::Next => run(
                content,
                tree,
                content_layout,
                renderer,
                focus_next_clamped::<()>(),
            ),
            Step::Previous => run(
                content,
                tree,
                content_layout,
                renderer,
                focus_previous_clamped::<()>(),
            ),
        };

        shell.capture_event();
        shell.request_redraw();
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

impl<'a, Message, Theme, Renderer> From<EdgeInterceptor<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(interceptor: EdgeInterceptor<'a, Message, Theme, Renderer>) -> Self {
        Element::new(interceptor)
    }
}
