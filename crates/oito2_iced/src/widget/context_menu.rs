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

//! A region that opens a menu on a secondary click or from the keyboard.
//!
//! [`context_menu`] wraps an element: a right click on it, or Shift+F10 or
//! the Menu key while a widget inside it has the focus, publishes the open
//! request. The menu opens where the pointer was, or over the center of the
//! region from the keyboard with its first enabled item highlighted. The
//! menu is the one of [`menu`](super::menu), with the same keys and items.
//!
//! ```no_run
//! use oito2_iced::widget::{context_menu, menu};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Menu(bool),
//!     Copy,
//! }
//!
//! # fn view<'a>(open: bool) -> oito2_iced::Element<'a, Message> {
//! context_menu::context_menu(
//!     iced::widget::text("Right click here"),
//!     [menu::item("Copy").on_press(Message::Copy)],
//! )
//! .open(open)
//! .on_open_change(Message::Menu)
//! .into()
//! # }
//! ```

use std::cell::Cell;
use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::Space;
use iced::{Event, Length, Point, Rectangle, Size, Vector, mouse};

use super::menu::{Item, List, anchored_menu};
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// A closure that turns the requested open state into a message.
type OnOpenChange<'a, Message> = Box<dyn Fn(bool) -> Message + 'a>;

/// An element with a context menu. Built by [`context_menu`].
pub struct ContextMenu<'a, Message> {
    content: Element<'a, Message>,
    items: Vec<Item<'a, Message>>,
    open: bool,
    on_open_change: Option<OnOpenChange<'a, Message>>,
    direction: ReadingDirection,
}

/// Wraps `content` with a menu of `items` that opens on a right click, or
/// on Shift+F10 or the Menu key while a widget inside `content` has the
/// focus.
pub fn context_menu<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> ContextMenu<'a, Message> {
    ContextMenu {
        content: content.into(),
        items: items.into_iter().collect(),
        open: false,
        on_open_change: None,
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> ContextMenu<'a, Message> {
    /// Shows or hides the menu.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the closure that turns the requested open state into a message:
    /// `true` on a right click, Shift+F10 or the Menu key; `false` on
    /// Escape, a click outside the menu, Tab, or after an item that closes
    /// the menu. Without it the menu never opens.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Sets the reading direction, which places the menu and mirrors its
    /// items.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<ContextMenu<'a, Message>> for Element<'a, Message> {
    fn from(menu: ContextMenu<'a, Message>) -> Self {
        let ContextMenu {
            content,
            items,
            open,
            on_open_change,
            direction,
        } = menu;

        let enabled = on_open_change.is_some();
        let open = open && enabled;
        let request = |value: bool| on_open_change.as_ref().map(|f| f(value));
        let close = request(false);
        let focus_first = Rc::new(Cell::new(false));

        let panel = List::new(items, open, close.clone(), direction)
            .focus_first(Rc::clone(&focus_first))
            .into_panel();
        let pin = anchored_menu(Space::new(), panel, open, close, direction);

        Element::new(Region {
            children: [content, pin.into()],
            on_open: if open { None } else { request(true) },
            focus_first,
        })
    }
}

/// The tree state of a [`Region`].
#[derive(Debug, Default)]
struct State {
    /// Where the menu opens, from the top-left corner of the region.
    point: Point,
    /// Whether the menu was opened from the keyboard.
    from_keyboard: bool,
}

/// The content and a zero-size anchor at the point where the menu opens.
struct Region<'a, Message> {
    children: [Element<'a, Message>; 2],
    on_open: Option<Message>,
    focus_first: Rc<Cell<bool>>,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for Region<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        self.focus_first
            .set(tree.state.downcast_ref::<State>().from_keyboard);
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        self.children[0].as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.children[0].as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let point = tree.state.downcast_ref::<State>().point;
        let [content, pin] = &mut self.children;
        let content = content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let pin = pin
            .as_widget_mut()
            .layout(
                &mut tree.children[1],
                renderer,
                &layout::Limits::new(Size::ZERO, Size::ZERO),
            )
            .move_to(point);

        layout::Node::with_children(content.size(), vec![content, pin])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(content_layout) = layout.children().next() {
            self.children[0].as_widget_mut().operate(
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
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.children[0].as_widget_mut().update(
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

        let Some(message) = self.on_open.clone() else {
            return;
        };

        let bounds = layout.bounds();

        let opened_at = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)) => cursor
                .position_over(bounds)
                .map(|position| (position - bounds.position(), false)),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if matches!(
                (named, modifiers.shift()),
                (key::Named::F10, true) | (key::Named::ContextMenu, _)
            ) && crate::focus::run(
                &mut self.children[0],
                &mut tree.children[0],
                content_layout,
                renderer,
                crate::focus::operations::has_focus(),
            )
            .unwrap_or(false) =>
            {
                Some((Vector::new(bounds.width / 2.0, bounds.height / 2.0), true))
            }
            _ => None,
        };

        if let Some((offset, from_keyboard)) = opened_at {
            let state = tree.state.downcast_mut::<State>();
            state.point = Point::ORIGIN + offset;
            state.from_keyboard = from_keyboard;

            shell.publish(message);
            shell.capture_event();
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
        if let Some(content_layout) = layout.children().next() {
            self.children[0].as_widget().draw(
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
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        layout
            .children()
            .next()
            .map(|content_layout| {
                self.children[0].as_widget().mouse_interaction(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::menu::{self, find_highlight};

    #[allow(dead_code)]
    #[derive(Debug, Clone)]
    enum Message {
        Menu(bool),
        Copy,
    }

    fn view(open: bool) -> Element<'static, Message> {
        context_menu(
            iced::widget::text("Region"),
            [menu::item("Copy").on_press(Message::Copy)],
        )
        .open(open)
        .on_open_change(Message::Menu)
        .into()
    }

    #[test]
    fn a_menu_opened_from_the_keyboard_highlights_its_first_item() {
        for (from_keyboard, highlight) in [(true, Some(0)), (false, None)] {
            let closed = view(false);
            let mut tree = Tree::new(closed.as_widget());
            tree.state.downcast_mut::<State>().from_keyboard = from_keyboard;

            let open = view(true);
            open.as_widget().diff(&mut tree);

            assert_eq!(find_highlight(&tree), Some(highlight));
        }
    }
}
