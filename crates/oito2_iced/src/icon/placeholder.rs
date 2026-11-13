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

//! A square that stands in for a missing picture.

use iced::advanced::graphics::geometry::{self, Fill, Frame, Path};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::widget::{Tree, Widget};
use iced::widget::canvas::path::Builder;
use iced::{Border, Element, Length, Point, Rectangle, Size, mouse};

use crate::Theme;
use crate::tokens::radius;

/// Opacity of the picture outline drawn by a [`Placeholder`].
const INK_OPACITY: f32 = 0.55;

/// A decorative square that stands in for a missing picture, such as an
/// album cover or an app icon that has not loaded: a rounded square in the
/// highest surface container tone with a picture outline in its center.
pub fn placeholder() -> Placeholder {
    Placeholder {
        size: 48.0,
        radius: radius::MEDIUM,
    }
}

/// A placeholder for a missing picture. See [`placeholder`].
#[derive(Debug, Clone, Copy)]
pub struct Placeholder {
    size: f32,
    radius: f32,
}

impl Placeholder {
    /// Sets the width and height (48 by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the corner radius (medium by default).
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }
}

/// The picture outline on a 24-unit grid: a frame with a hole, a sun and two
/// hills.
fn outline(unit: f32, origin: Point) -> Path {
    let at = |x: f32, y: f32| Point::new(origin.x + x * unit, origin.y + y * unit);
    let rounded = |builder: &mut Builder,
                   (left, top, right, bottom, r): (f32, f32, f32, f32, f32)| {
        builder.rounded_rectangle(
            at(left, top),
            Size::new((right - left) * unit, (bottom - top) * unit),
            (r * unit).into(),
        );
    };

    Path::new(|builder| {
        rounded(builder, (2.0, 3.0, 22.0, 21.0, 3.0));
        rounded(builder, (4.0, 5.0, 20.0, 19.0, 1.5));
        builder.circle(at(8.5, 9.5), 1.75 * unit);
        builder.move_to(at(4.0, 19.0));
        builder.line_to(at(10.0, 12.5));
        builder.line_to(at(13.5, 16.0));
        builder.line_to(at(16.0, 13.5));
        builder.line_to(at(20.0, 17.5));
        builder.line_to(at(20.0, 19.0));
        builder.close();
    })
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Placeholder
where
    Renderer: geometry::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(limits.resolve(self.size, self.size, Size::new(self.size, self.size)))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let roles = theme.roles();

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border::default().rounded(self.radius),
                ..Quad::default()
            },
            roles.surface_container_highest,
        );

        let unit = bounds.width.min(bounds.height) / 48.0;
        let origin = Point::new(
            bounds.width / 2.0 - 12.0 * unit,
            bounds.height / 2.0 - 12.0 * unit,
        );

        let mut frame = Frame::new(renderer, bounds.size());
        frame.fill(
            &outline(unit, origin),
            Fill {
                style: geometry::Style::Solid(roles.on_surface_variant.scale_alpha(INK_OPACITY)),
                rule: geometry::fill::Rule::EvenOdd,
            },
        );

        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message, Renderer> From<Placeholder> for Element<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer + 'a,
{
    fn from(placeholder: Placeholder) -> Self {
        Element::new(placeholder)
    }
}
