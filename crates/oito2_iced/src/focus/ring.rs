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

//! A focus ring around a group of widgets.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::{Border, Element, Event, Length, Rectangle, Size, Vector, keyboard, mouse};

use super::operations::has_focus;
use super::run;
use crate::Theme;
use crate::tokens::{radius, sizes};

/// Draws the focus ring of the theme around `content` while a widget inside
/// it has the keyboard focus.
///
/// The ring shows only while the keyboard drives the interface: a mouse
/// press hides it until the next key press. It is drawn outside the bounds
/// of the content and does not affect the layout.
pub fn ring<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Ring<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    Ring {
        content: content.into(),
        radius: radius::MEDIUM,
        focused: None,
    }
}

/// A focus ring around a group of widgets. See [`ring`].
pub struct Ring<'a, Message, Renderer = iced::Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    radius: f32,
    focused: Option<bool>,
}

impl<Message, Renderer> Ring<'_, Message, Renderer> {
    /// Sets the corner radius of the ring (medium radius by default).
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Forces the ring on or off; by default it follows the keyboard focus.
    #[must_use]
    pub fn focused(mut self, focused: bool) -> Self {
        self.focused = Some(focused);
        self
    }
}

/// Whether a widget inside has the focus, and whether the keyboard drives
/// the interface.
#[derive(Debug, Clone, Copy)]
struct State {
    has_focus: bool,
    keyboard: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            has_focus: false,
            keyboard: true,
        }
    }
}

impl<Message, Renderer> Ring<'_, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Records whether a widget inside the content has the focus.
    fn refresh(&mut self, tree: &mut Tree, layout: Layout<'_>, renderer: &Renderer) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        let has_focus = run(
            &mut self.content,
            &mut tree.children[0],
            content_layout,
            renderer,
            has_focus(),
        )
        .unwrap_or(false);

        tree.state.downcast_mut::<State>().has_focus = has_focus;
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Ring<'_, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
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

        self.refresh(tree, layout, renderer);
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
        if let Some(content_layout) = layout.children().next() {
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
        }

        let before = *tree.state.downcast_ref::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                tree.state.downcast_mut::<State>().keyboard = false;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { .. }) => {
                tree.state.downcast_mut::<State>().keyboard = true;
            }
            _ => {}
        }

        if matches!(event, Event::Mouse(_) | Event::Keyboard(_)) {
            self.refresh(tree, layout, renderer);
        }

        let after = *tree.state.downcast_ref::<State>();

        if (before.has_focus && before.keyboard) != (after.has_focus && after.keyboard) {
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

        let state = tree.state.downcast_ref::<State>();
        let show = self.focused.unwrap_or(state.has_focus && state.keyboard);

        if show {
            draw_ring(renderer, theme, layout.bounds(), self.radius);
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

impl<'a, Message, Renderer> From<Ring<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(ring: Ring<'a, Message, Renderer>) -> Self {
        Element::new(ring)
    }
}

/// Draws the focus ring outside `bounds`: a border of the focus border width
/// in the focus color, with `radius` grown by that width so it follows the
/// corners of the content.
pub(crate) fn draw_ring<Renderer>(
    renderer: &mut Renderer,
    theme: &Theme,
    bounds: Rectangle,
    radius: f32,
) where
    Renderer: renderer::Renderer,
{
    let width = sizes::FOCUS_BORDER_WIDTH;

    renderer.fill_quad(
        renderer::Quad {
            bounds: bounds.expand(width),
            border: Border::default()
                .color(theme.focus_ring())
                .width(width)
                .rounded(radius + width),
            ..renderer::Quad::default()
        },
        iced::Color::TRANSPARENT,
    );
}
