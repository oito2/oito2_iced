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

//! Diagonal clip: content cut along a diagonal, keeping the triangle on
//! the side of a [`DiagonalCorner`]. Stack two of them to split an area
//! diagonally, for example to preview light and dark themes in one swatch.
//!
//! The pointer reaches the content only inside the triangle; elsewhere the
//! clip lets it through to the layers below in a
//! [`stack`](iced::widget::stack()).
//!
//! Iced 0.14 clips drawing to rectangles only, so the triangle is drawn as
//! a staircase of horizontal strips one logical pixel tall (taller strips
//! for content over [`MAX_STRIPS`] pixels tall), each a rectangular clip of
//! the content: the diagonal edge has no anti-aliasing, and the content is
//! drawn once per strip.
//!
//! ```no_run
//! use iced::widget::{container, stack};
//! use oito2_iced::widget::diagonal_clip::{DiagonalCorner, diagonal_clip};
//!
//! # fn view<'a>(light: oito2_iced::Element<'a, ()>, dark: oito2_iced::Element<'a, ()>)
//! # -> oito2_iced::Element<'a, ()> {
//! container(stack![
//!     diagonal_clip(DiagonalCorner::TopLeft, light),
//!     diagonal_clip(DiagonalCorner::BottomRight, dark),
//! ])
//! .width(64)
//! .height(64)
//! .into()
//! # }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::{Element as IcedElement, Event, Length, Point, Rectangle, Size, Vector, mouse};

use crate::{Element, Theme};

/// The most strips the triangle is drawn with.
pub const MAX_STRIPS: usize = 512;

/// The corner whose triangle a [`DiagonalClip`] keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DiagonalCorner {
    /// The top-left corner.
    TopLeft,
    /// The top-right corner.
    TopRight,
    /// The bottom-left corner.
    BottomLeft,
    /// The bottom-right corner.
    BottomRight,
}

impl DiagonalCorner {
    /// The horizontal span, as `(start, end)` fractions of the width, that
    /// the triangle covers at the height fraction `y` (0 at the top, 1 at
    /// the bottom).
    fn span(self, y: f32) -> (f32, f32) {
        let y = y.clamp(0.0, 1.0);

        match self {
            DiagonalCorner::TopLeft => (0.0, 1.0 - y),
            DiagonalCorner::TopRight => (y, 1.0),
            DiagonalCorner::BottomLeft => (0.0, y),
            DiagonalCorner::BottomRight => (1.0 - y, 1.0),
        }
    }

    /// Whether `point` lies in the triangle of this corner inside `bounds`.
    pub fn contains(self, bounds: Rectangle, point: Point) -> bool {
        if !bounds.contains(point) || bounds.width <= 0.0 || bounds.height <= 0.0 {
            return false;
        }

        let (start, end) = self.span((point.y - bounds.y) / bounds.height);
        let x = (point.x - bounds.x) / bounds.width;

        x >= start && x <= end
    }
}

/// Content clipped to a triangle. See [`diagonal_clip`].
pub struct DiagonalClip<'a, Message> {
    corner: DiagonalCorner,
    content: Element<'a, Message>,
}

/// Clips `content` along the diagonal between the two corners next to
/// `corner`, keeping the triangle of `corner`.
pub fn diagonal_clip<'a, Message>(
    corner: DiagonalCorner,
    content: impl Into<Element<'a, Message>>,
) -> DiagonalClip<'a, Message> {
    DiagonalClip {
        corner,
        content: content.into(),
    }
}

impl<Message> DiagonalClip<'_, Message> {
    /// `cursor`, or no cursor when it is outside the triangle.
    fn cursor(&self, bounds: Rectangle, cursor: mouse::Cursor) -> mouse::Cursor {
        match cursor.position() {
            Some(position) if self.corner.contains(bounds, position) => cursor,
            _ => mouse::Cursor::Unavailable,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for DiagonalClip<'_, Message> {
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
        renderer: &iced::Renderer,
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
        renderer: &iced::Renderer,
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
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let cursor = self.cursor(layout.bounds(), cursor);

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
        let bounds = layout.bounds();

        if bounds.width <= 0.0 || bounds.height <= 0.0 {
            return;
        }

        let cursor = self.cursor(bounds, cursor);
        let strips = (bounds.height.ceil() as usize).clamp(1, MAX_STRIPS);
        let strip = bounds.height / strips as f32;

        for index in 0..strips {
            let top = index as f32 * strip;
            // The span at the middle of the strip keeps the pixels that are
            // at least half inside the triangle.
            let (start, end) = self.corner.span((top + strip / 2.0) / bounds.height);
            let width = (end - start) * bounds.width;

            if width <= 0.0 {
                continue;
            }

            let clip = Rectangle {
                x: bounds.x + start * bounds.width,
                y: bounds.y + top,
                width,
                height: strip,
            };

            renderer.with_layer(clip, |renderer| {
                self.content.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    style,
                    content_layout,
                    cursor,
                    viewport,
                );
            });
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
        let cursor = self.cursor(layout.bounds(), cursor);

        if cursor.position().is_none() {
            return mouse::Interaction::None;
        }

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

impl<'a, Message: 'a> From<DiagonalClip<'a, Message>> for Element<'a, Message> {
    fn from(clip: DiagonalClip<'a, Message>) -> Self {
        IcedElement::new(clip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_corner_keeps_its_triangle() {
        let bounds = Rectangle::new(Point::ORIGIN, Size::new(100.0, 100.0));
        let near =
            |corner: DiagonalCorner, x: f32, y: f32| corner.contains(bounds, Point::new(x, y));

        assert!(near(DiagonalCorner::TopLeft, 10.0, 10.0));
        assert!(!near(DiagonalCorner::TopLeft, 90.0, 90.0));
        assert!(near(DiagonalCorner::TopRight, 90.0, 10.0));
        assert!(!near(DiagonalCorner::TopRight, 10.0, 90.0));
        assert!(near(DiagonalCorner::BottomLeft, 10.0, 90.0));
        assert!(!near(DiagonalCorner::BottomLeft, 90.0, 10.0));
        assert!(near(DiagonalCorner::BottomRight, 90.0, 90.0));
        assert!(!near(DiagonalCorner::BottomRight, 10.0, 10.0));
        assert!(!near(DiagonalCorner::TopLeft, 110.0, 10.0));
    }
}
