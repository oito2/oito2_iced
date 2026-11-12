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

//! A panel anchored to an element.
//!
//! [`popover`] keeps the anchor in the layout and, while open, shows the
//! panel in an overlay next to it: below the anchor by default, above it
//! when the space below is too short, and moved horizontally to stay inside
//! the window. A mouse press outside the panel and the anchor, or Escape,
//! publishes the dismiss message; the application closes the popover by
//! rendering it with `open(false)`.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::container;
use iced::{Element, Event, Length, Padding, Point, Rectangle, Size, Vector, mouse};

use crate::Theme;
use crate::theme::container::Class as ContainerClass;
use crate::tokens::spacing;

/// Where the panel opens relative to the anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    /// Below the anchor; above it when the space below is too short.
    #[default]
    Bottom,
    /// Above the anchor; below it when the space above is too short.
    Top,
}

/// An `anchor` with a `panel` that opens next to it. See the
/// [module documentation](self).
pub fn popover<'a, Message, Renderer>(
    anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
    panel: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Popover<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    Popover {
        anchor: anchor.into(),
        panel: panel.into(),
        open: false,
        on_dismiss: None,
        placement: Placement::Bottom,
        gap: spacing::XS,
        padding: Padding::new(spacing::SM),
    }
}

/// An anchored panel. See [`popover`].
pub struct Popover<'a, Message, Renderer = iced::Renderer> {
    anchor: Element<'a, Message, Theme, Renderer>,
    panel: Element<'a, Message, Theme, Renderer>,
    open: bool,
    on_dismiss: Option<Message>,
    placement: Placement,
    gap: f32,
    padding: Padding,
}

impl<Message, Renderer> Popover<'_, Message, Renderer> {
    /// Shows or hides the panel.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the message published on a mouse press outside the panel and
    /// the anchor, or on Escape.
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }

    /// Sets the preferred placement.
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the distance between the anchor and the panel.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the padding inside the panel.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Popover<'_, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.anchor), Tree::new(&self.panel)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.anchor, &self.panel]);
    }

    fn size(&self) -> Size<Length> {
        self.anchor.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.anchor.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.anchor
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation,
    ) {
        self.anchor
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        self.anchor.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
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
        self.anchor.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.anchor.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        let [anchor_tree, panel_tree] = tree.children.as_mut_slice() else {
            return None;
        };

        if !self.open {
            return self.anchor.as_widget_mut().overlay(
                anchor_tree,
                layout,
                renderer,
                viewport,
                translation,
            );
        }

        Some(overlay::Element::new(Box::new(Panel {
            panel: &mut self.panel,
            tree: panel_tree,
            anchor: layout.bounds() + translation,
            placement: self.placement,
            gap: self.gap,
            padding: self.padding,
            on_dismiss: self.on_dismiss.clone(),
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

/// The overlay that shows the panel of an open [`Popover`].
struct Panel<'a, 'b, Message, Renderer> {
    panel: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    anchor: Rectangle,
    placement: Placement,
    gap: f32,
    padding: Padding,
    on_dismiss: Option<Message>,
}

/// Places a panel of `size` next to `anchor` inside `viewport`: on the
/// preferred side when it fits, otherwise on the side with more room; then
/// aligned with the start of the anchor and moved to stay inside the
/// viewport.
pub fn place(
    anchor: Rectangle,
    size: Size,
    viewport: Size,
    placement: Placement,
    gap: f32,
) -> Point {
    let space_below = viewport.height - (anchor.y + anchor.height + gap);
    let space_above = anchor.y - gap;

    let below = match placement {
        Placement::Bottom => size.height <= space_below || space_below >= space_above,
        Placement::Top => size.height > space_above && space_below > space_above,
    };

    let y = if below {
        anchor.y + anchor.height + gap
    } else {
        anchor.y - gap - size.height
    };

    let x = anchor.x.min(viewport.width - size.width).max(0.0);

    Point::new(x, y.max(0.0))
}

impl<Message, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Panel<'_, '_, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let limits = layout::Limits::new(Size::ZERO, bounds).shrink(self.padding);
        let content = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits)
            .translate(Vector::new(self.padding.left, self.padding.top));

        let size = content.size().expand(self.padding);
        let position = place(self.anchor, size, bounds, self.placement, self.gap);

        layout::Node::with_children(size, vec![content]).move_to(position)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let panel_style = <Theme as container::Catalog>::style(theme, &ContainerClass::Popover);
        container::draw_background(renderer, &panel_style, layout.bounds());

        let style = renderer::Style {
            text_color: panel_style.text_color.unwrap_or(style.text_color),
        };

        if let Some(content_layout) = layout.children().next() {
            self.panel.as_widget().draw(
                self.tree,
                renderer,
                theme,
                &style,
                content_layout,
                cursor,
                &layout.bounds(),
            );
        }
    }

    fn operate(&mut self, layout: Layout<'_>, renderer: &Renderer, operation: &mut dyn Operation) {
        if let Some(content_layout) = layout.children().next() {
            self.panel
                .as_widget_mut()
                .operate(self.tree, content_layout, renderer, operation);
        }
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
        let bounds = layout.bounds();

        if let Some(content_layout) = layout.children().next() {
            self.panel.as_widget_mut().update(
                self.tree,
                event,
                content_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                &bounds,
            );
        }

        if shell.is_event_captured() {
            return;
        }

        let dismiss = match event {
            Event::Mouse(mouse::Event::ButtonPressed(_)) => {
                !cursor.is_over(bounds) && !cursor.is_over(self.anchor)
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key::Named::Escape),
                ..
            }) => true,
            _ => false,
        };

        if dismiss && let Some(message) = &self.on_dismiss {
            shell.publish(message.clone());
            shell.capture_event();
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        let bounds = layout.bounds();

        layout
            .children()
            .next()
            .map(|content_layout| {
                self.panel.as_widget().mouse_interaction(
                    self.tree,
                    content_layout,
                    cursor,
                    &bounds,
                    renderer,
                )
            })
            .unwrap_or_default()
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, Renderer>> {
        let content_layout = layout.children().next()?;
        let bounds = layout.bounds();

        self.panel.as_widget_mut().overlay(
            self.tree,
            content_layout,
            renderer,
            &bounds,
            Vector::ZERO,
        )
    }

    fn index(&self) -> f32 {
        2.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEWPORT: Size = Size::new(800.0, 600.0);
    const PANEL: Size = Size::new(200.0, 100.0);

    #[test]
    fn opens_below_when_it_fits() {
        let anchor = Rectangle::new(Point::new(10.0, 10.0), Size::new(80.0, 34.0));
        let position = place(anchor, PANEL, VIEWPORT, Placement::Bottom, 4.0);

        assert_eq!(position, Point::new(10.0, 48.0));
    }

    #[test]
    fn flips_above_when_the_space_below_is_short() {
        let anchor = Rectangle::new(Point::new(10.0, 540.0), Size::new(80.0, 34.0));
        let position = place(anchor, PANEL, VIEWPORT, Placement::Bottom, 4.0);

        assert_eq!(position, Point::new(10.0, 436.0));
    }

    #[test]
    fn stays_inside_the_viewport_horizontally() {
        let anchor = Rectangle::new(Point::new(700.0, 10.0), Size::new(80.0, 34.0));
        let position = place(anchor, PANEL, VIEWPORT, Placement::Bottom, 4.0);

        assert_eq!(position.x, 600.0);
    }
}
