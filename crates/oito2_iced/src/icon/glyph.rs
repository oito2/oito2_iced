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

//! The widget that draws an [`Icon`] in the text color.

use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer;
use iced::advanced::svg::{self, Svg};
use iced::advanced::widget::{Tree, Widget};
use iced::{Color, Element, Length, Rectangle, Size, mouse};

use super::Icon;
use crate::Theme;

/// Draws `icon` as a square of the icon size of the current
/// [density](crate::density), in the text color of its parent: the label
/// color inside a button, the content color inside a container.
pub fn icon(icon: Icon) -> Glyph {
    Glyph {
        icon,
        size: None,
        color: None,
    }
}

/// An icon drawn as a widget. See [`icon`].
#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    icon: Icon,
    size: Option<f32>,
    color: Option<Color>,
}

impl Glyph {
    /// Sets the width and height, instead of the icon size of the density.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the color, instead of the text color of the parent.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    fn side(&self) -> f32 {
        self.size.unwrap_or_else(crate::density::icon_size)
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Glyph
where
    Renderer: svg::Renderer,
{
    fn size(&self) -> Size<Length> {
        let side = Length::Fixed(self.side());

        Size::new(side, side)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = self.side();

        layout::Node::new(limits.resolve(side, side, Size::new(side, side)))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };

        let color = self.color.unwrap_or(style.text_color);

        // The color filter of the renderers keeps the alpha of the image, so
        // the alpha of the color is applied as opacity.
        let svg = Svg::new(self.icon.handle())
            .color(Color { a: 1.0, ..color })
            .opacity(color.a);

        renderer.draw_svg(svg, bounds, clip);
    }
}

impl<'a, Message, Renderer> From<Glyph> for Element<'a, Message, Theme, Renderer>
where
    Renderer: svg::Renderer + 'a,
{
    fn from(glyph: Glyph) -> Self {
        Element::new(glyph)
    }
}
