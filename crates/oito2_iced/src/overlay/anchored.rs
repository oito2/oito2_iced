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
//! [`anchored`] keeps the anchor in the layout and, while open, shows the
//! panel in an overlay next to it: on the preferred side when the panel fits
//! there, otherwise on the opposite side when it has more room; aligned with
//! the anchor and moved to stay inside the window, a margin away from its
//! edges. A mouse press outside the panel and the anchor, or Escape,
//! publishes the dismiss message; the application closes the panel by
//! rendering it with `open(false)`.
//!
//! Panels nest: an anchored panel inside another opens above it, and a press
//! inside the inner panel does not dismiss the outer one.

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::container;
use iced::{Element, Event, Length, Padding, Point, Rectangle, Size, Vector, mouse};

use crate::Theme;
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{radius, spacing};
use crate::widget::helpers::ReadingDirection;

/// The side of the anchor where the panel opens. `Start` and `End` follow
/// the reading direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Placement {
    /// Above the anchor.
    Top,
    /// Below the anchor.
    #[default]
    Bottom,
    /// Before the anchor in the reading order.
    Start,
    /// After the anchor in the reading order.
    End,
}

/// How the panel lines up with the anchor along the edge it opens on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// With the start of the anchor (its top for side placements).
    #[default]
    Start,
    /// Centered on the anchor.
    Center,
    /// With the end of the anchor (its bottom for side placements).
    End,
}

/// A physical side of the anchor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Side {
    /// Above.
    Top,
    /// Below.
    Bottom,
    /// To the left.
    Left,
    /// To the right.
    Right,
}

impl Side {
    /// The side across the anchor.
    pub fn opposite(self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
            Side::Right => Side::Left,
        }
    }

    /// Whether the panel sits above or below the anchor.
    pub fn is_vertical(self) -> bool {
        matches!(self, Side::Top | Side::Bottom)
    }

    /// The physical side of a placement in a reading direction.
    pub fn of(placement: Placement, direction: ReadingDirection) -> Self {
        let ltr = direction == ReadingDirection::LeftToRight;

        match placement {
            Placement::Top => Side::Top,
            Placement::Bottom => Side::Bottom,
            Placement::Start if ltr => Side::Left,
            Placement::Start => Side::Right,
            Placement::End if ltr => Side::Right,
            Placement::End => Side::Left,
        }
    }
}

/// The geometry of a placed panel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placed {
    /// Top-left corner of the panel.
    pub position: Point,
    /// The side of the anchor the panel opened on.
    pub side: Side,
    /// The center of the arrow along the panel edge that faces the anchor,
    /// from the start of that edge.
    pub arrow_center: f32,
}

/// The options of [`place`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Geometry {
    /// The preferred side.
    pub side: Side,
    /// The alignment along the anchor edge.
    pub alignment: Alignment,
    /// The reading direction, which swaps the start and end alignments of
    /// panels above and below the anchor.
    pub direction: ReadingDirection,
    /// Space between the anchor and the panel or its arrow tip.
    pub gap: f32,
    /// Space kept free at the edges of the viewport.
    pub margin: f32,
    /// The arrow base width and height, if any.
    pub arrow: Option<Size>,
    /// The corner radius of the panel, kept free of the arrow.
    pub radius: f32,
}

impl Geometry {
    /// The room for the panel on `side` of `anchor` inside `viewport`, after
    /// the gap, the arrow and the margin; never negative.
    pub fn space(&self, side: Side, anchor: Rectangle, viewport: Size) -> f32 {
        let reach = self.reach() + self.margin;

        let space = match side {
            Side::Top => anchor.y - reach,
            Side::Bottom => viewport.height - (anchor.y + anchor.height) - reach,
            Side::Left => anchor.x - reach,
            Side::Right => viewport.width - (anchor.x + anchor.width) - reach,
        };

        space.max(0.0)
    }

    /// The distance from the anchor to the panel: the gap plus the arrow.
    fn reach(&self) -> f32 {
        self.gap + self.arrow.map_or(0.0, |arrow| arrow.height)
    }

    /// The placement of a panel already laid out at `panel`: the side of the
    /// anchor it sits on and the center of its arrow.
    pub fn placed_at(&self, anchor: Rectangle, panel: Rectangle) -> Placed {
        let side = if panel.y >= anchor.y + anchor.height {
            Side::Bottom
        } else if panel.y + panel.height <= anchor.y {
            Side::Top
        } else if panel.x >= anchor.x + anchor.width {
            Side::Right
        } else {
            Side::Left
        };
        let half = self.arrow.map_or(0.0, |arrow| arrow.width / 2.0);
        let (center, extent) = if side.is_vertical() {
            (anchor.center_x() - panel.x, panel.width)
        } else {
            (anchor.center_y() - panel.y, panel.height)
        };

        Placed {
            position: panel.position(),
            side,
            arrow_center: center
                .min(extent - self.radius - half)
                .max(self.radius + half),
        }
    }

    /// The side the panel opens on: the preferred one, or the opposite one
    /// when the panel does not fit and the opposite one has more room.
    pub fn side_for(&self, size: Size, anchor: Rectangle, viewport: Size) -> Side {
        let extent = |side: Side| {
            if side.is_vertical() {
                size.height
            } else {
                size.width
            }
        };
        let preferred = self.side;
        let opposite = preferred.opposite();

        if extent(preferred) > self.space(preferred, anchor, viewport)
            && self.space(opposite, anchor, viewport) > self.space(preferred, anchor, viewport)
        {
            opposite
        } else {
            preferred
        }
    }
}

/// Places a panel of `size` on `side` of `anchor` inside `viewport`: aligned
/// with the anchor, then moved to stay a margin away from the viewport edges.
pub fn place(
    anchor: Rectangle,
    size: Size,
    viewport: Size,
    side: Side,
    geometry: &Geometry,
) -> Placed {
    let reach = geometry.reach();
    let margin = geometry.margin;
    let arrow_half = geometry.arrow.map_or(0.0, |arrow| arrow.width / 2.0);

    let align = |start: f32, end: f32, extent: f32, flip: bool| {
        let alignment = match geometry.alignment {
            Alignment::Start if flip => Alignment::End,
            Alignment::End if flip => Alignment::Start,
            other => other,
        };

        match alignment {
            Alignment::Start => start,
            Alignment::Center => (start + end - extent) / 2.0,
            Alignment::End => end - extent,
        }
    };

    let clamp = |value: f32, min: f32, max: f32| value.min(max).max(min);

    if side.is_vertical() {
        let y = if side == Side::Bottom {
            anchor.y + anchor.height + reach
        } else {
            anchor.y - reach - size.height
        };
        let rtl = geometry.direction == ReadingDirection::RightToLeft;
        let x = clamp(
            align(anchor.x, anchor.x + anchor.width, size.width, rtl),
            margin,
            viewport.width - margin - size.width,
        );
        let arrow_center = clamp(
            anchor.center_x() - x,
            geometry.radius + arrow_half,
            size.width - geometry.radius - arrow_half,
        );

        Placed {
            position: Point::new(x, y),
            side,
            arrow_center,
        }
    } else {
        let x = if side == Side::Right {
            anchor.x + anchor.width + reach
        } else {
            anchor.x - reach - size.width
        };
        let y = clamp(
            align(anchor.y, anchor.y + anchor.height, size.height, false),
            margin,
            viewport.height - margin - size.height,
        );
        let arrow_center = clamp(
            anchor.center_y() - y,
            geometry.radius + arrow_half,
            size.height - geometry.radius - arrow_half,
        );

        Placed {
            position: Point::new(x, y),
            side,
            arrow_center,
        }
    }
}

/// An `anchor` with a `panel` that opens next to it. See the
/// [module documentation](self).
pub fn anchored<'a, Message, Renderer>(
    anchor: impl Into<Element<'a, Message, Theme, Renderer>>,
    panel: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Anchored<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    Anchored {
        anchor: anchor.into(),
        panel: panel.into(),
        open: false,
        on_dismiss: None,
        placement: Placement::Bottom,
        alignment: Alignment::Start,
        direction: ReadingDirection::LeftToRight,
        gap: spacing::XS,
        margin: spacing::SM,
        padding: Padding::new(spacing::SM),
        arrow: false,
        max_width: f32::INFINITY,
        max_height: f32::INFINITY,
        match_anchor_width: false,
        fit_anchor_width: false,
        class: Panel::Popover,
    }
}

/// The surface the panel is drawn on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Panel {
    Popover,
    Menu,
    Tooltip,
}

/// Size of the arrow: base width and height.
const ARROW: Size = Size::new(14.0, 7.0);

/// An anchored panel. See [`anchored`].
pub struct Anchored<'a, Message, Renderer = iced::Renderer> {
    anchor: Element<'a, Message, Theme, Renderer>,
    panel: Element<'a, Message, Theme, Renderer>,
    open: bool,
    on_dismiss: Option<Message>,
    placement: Placement,
    alignment: Alignment,
    direction: ReadingDirection,
    gap: f32,
    margin: f32,
    padding: Padding,
    arrow: bool,
    max_width: f32,
    max_height: f32,
    match_anchor_width: bool,
    fit_anchor_width: bool,
    class: Panel,
}

impl<Message, Renderer> Anchored<'_, Message, Renderer> {
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

    /// Sets the preferred side.
    #[must_use]
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the alignment along the anchor edge.
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets the reading direction that maps the start and end placements and
    /// alignments to physical sides.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the distance between the anchor and the panel or its arrow tip.
    #[must_use]
    pub fn gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    /// Sets the space kept free at the edges of the window.
    #[must_use]
    pub fn margin(mut self, margin: f32) -> Self {
        self.margin = margin;
        self
    }

    /// Sets the padding inside the panel.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Draws an arrow from the panel to the anchor.
    #[must_use]
    pub fn arrow(mut self, arrow: bool) -> Self {
        self.arrow = arrow;
        self
    }

    /// Sets the largest width of the panel.
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }

    /// Sets the largest height of the panel.
    #[must_use]
    pub fn max_height(mut self, height: f32) -> Self {
        self.max_height = height;
        self
    }

    /// Gives a panel above or below the anchor the width of the anchor.
    #[must_use]
    pub fn match_anchor_width(mut self, matches: bool) -> Self {
        self.match_anchor_width = matches;
        self
    }

    /// Gives a panel above or below the anchor exactly the width of the
    /// anchor: at least as wide, as with [`Anchored::match_anchor_width`],
    /// and no wider, so content that fills the width takes the anchor's.
    #[must_use]
    pub fn fit_anchor_width(mut self, fits: bool) -> Self {
        self.fit_anchor_width = fits;
        self
    }

    /// Shows or hides the panel of an anchored panel already built.
    pub(crate) fn set_open(&mut self, open: bool) {
        self.open = open;
    }

    /// Draws the panel as a menu: the menu surface with the medium radius.
    #[must_use]
    pub fn menu(mut self) -> Self {
        self.class = Panel::Menu;
        self
    }

    /// Draws the panel as a tooltip: the inverse surface with the small
    /// radius.
    #[must_use]
    pub fn tooltip(mut self) -> Self {
        self.class = Panel::Tooltip;
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Anchored<'_, Message, Renderer>
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

        let radius = match self.class {
            Panel::Popover => radius::LARGE,
            Panel::Menu => radius::MEDIUM,
            Panel::Tooltip => radius::SMALL,
        };

        Some(overlay::Element::new(Box::new(Overlay {
            panel: &mut self.panel,
            tree: panel_tree,
            anchor: layout.bounds() + translation,
            geometry: Geometry {
                side: Side::of(self.placement, self.direction),
                alignment: self.alignment,
                direction: self.direction,
                gap: self.gap,
                margin: self.margin,
                arrow: self.arrow.then_some(ARROW),
                radius,
            },
            padding: self.padding,
            max: Size::new(self.max_width, self.max_height),
            match_anchor_width: self.match_anchor_width,
            fit_anchor_width: self.fit_anchor_width,
            class: self.class,
            on_dismiss: self.on_dismiss.clone(),
        })))
    }
}

impl<'a, Message, Renderer> From<Anchored<'a, Message, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: renderer::Renderer + 'a,
{
    fn from(anchored: Anchored<'a, Message, Renderer>) -> Self {
        Element::new(anchored)
    }
}

/// The overlay that shows the panel of an open [`Anchored`].
struct Overlay<'a, 'b, Message, Renderer> {
    panel: &'b mut Element<'a, Message, Theme, Renderer>,
    tree: &'b mut Tree,
    anchor: Rectangle,
    geometry: Geometry,
    padding: Padding,
    max: Size,
    match_anchor_width: bool,
    fit_anchor_width: bool,
    class: Panel,
    on_dismiss: Option<Message>,
}

impl<Message, Renderer> Overlay<'_, '_, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    /// Lays the panel content out within `max`, with the padding around it.
    fn layout_panel(&mut self, renderer: &Renderer, max: Size, side: Side) -> layout::Node {
        let max = Size::new(
            max.width.min(self.max.width),
            max.height.min(self.max.height),
        );
        let fits = self.fit_anchor_width && side.is_vertical();
        let max = if fits {
            Size::new(max.width.min(self.anchor.width), max.height)
        } else {
            max
        };
        let min_width = if (self.match_anchor_width || fits) && side.is_vertical() {
            self.anchor.width.min(max.width)
        } else {
            0.0
        };
        let limits = layout::Limits::new(Size::new(min_width, 0.0), max).shrink(self.padding);
        let content = self
            .panel
            .as_widget_mut()
            .layout(self.tree, renderer, &limits)
            .translate(Vector::new(self.padding.left, self.padding.top));
        let size = content.size().expand(self.padding);
        let size = Size::new(size.width.max(min_width), size.height);

        layout::Node::with_children(size, vec![content])
    }

    /// The surface style of the panel.
    fn style(&self, theme: &Theme) -> container::Style {
        let class = match self.class {
            Panel::Popover => ContainerClass::Popover,
            Panel::Menu => ContainerClass::Menu,
            Panel::Tooltip => ContainerClass::Tooltip,
        };

        <Theme as container::Catalog>::style(theme, &class)
    }
}

impl<Message, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'_, '_, Message, Renderer>
where
    Message: Clone,
    Renderer: renderer::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        let margin = self.geometry.margin;
        let area = Size::new(
            (bounds.width - 2.0 * margin).max(0.0),
            (bounds.height - 2.0 * margin).max(0.0),
        );

        let node = self.layout_panel(renderer, area, self.geometry.side);
        let side = self.geometry.side_for(node.size(), self.anchor, bounds);
        let space = self.geometry.space(side, self.anchor, bounds);

        let too_long = if side.is_vertical() {
            node.size().height > space
        } else {
            node.size().width > space
        };

        let node = if too_long {
            let limit = if side.is_vertical() {
                Size::new(area.width, space)
            } else {
                Size::new(space, area.height)
            };

            self.layout_panel(renderer, limit, side)
        } else if side != self.geometry.side {
            self.layout_panel(renderer, area, side)
        } else {
            node
        };

        let placed = place(self.anchor, node.size(), bounds, side, &self.geometry);

        node.move_to(placed.position)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        let bounds = layout.bounds();
        let panel_style = self.style(theme);

        container::draw_background(renderer, &panel_style, bounds);

        if let Some(arrow) = self.geometry.arrow {
            let placed = self.geometry.placed_at(self.anchor, bounds);

            draw_arrow(renderer, bounds, placed, arrow, &panel_style);
        }

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
                &bounds,
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
            Event::Mouse(mouse::Event::ButtonPressed(_)) => cursor
                .position()
                .is_some_and(|point| !bounds.contains(point) && !self.anchor.contains(point)),
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

        let interaction = layout
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
            .unwrap_or_default();

        // The panel claims the pointer over its whole surface, so the widgets
        // below it and the overlays it is nested in treat the pointer as
        // taken.
        if interaction == mouse::Interaction::None && cursor.is_over(bounds) {
            mouse::Interaction::Idle
        } else {
            interaction
        }
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

/// Draws the arrow between the panel and the anchor as stacks of one-pixel
/// strips that narrow toward the tip: first its outline in the border color,
/// then its fill in the panel background, which also covers the panel border
/// where the arrow joins it.
fn draw_arrow<Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    placed: Placed,
    arrow: Size,
    style: &container::Style,
) where
    Renderer: renderer::Renderer,
{
    let Some(background) = style.background else {
        return;
    };
    let steps = arrow.height.ceil().max(1.0) as i32;
    let width_at = |depth: f32| (arrow.width * (1.0 - depth / arrow.height)).max(0.0);

    let mut strip = |depth: f32, width: f32, color: iced::Background| {
        let half = width / 2.0;

        let rectangle = match placed.side {
            Side::Bottom => Rectangle::new(
                Point::new(
                    bounds.x + placed.arrow_center - half,
                    bounds.y - depth - 1.0,
                ),
                Size::new(width, 1.0),
            ),
            Side::Top => Rectangle::new(
                Point::new(
                    bounds.x + placed.arrow_center - half,
                    bounds.y + bounds.height + depth,
                ),
                Size::new(width, 1.0),
            ),
            Side::Right => Rectangle::new(
                Point::new(
                    bounds.x - depth - 1.0,
                    bounds.y + placed.arrow_center - half,
                ),
                Size::new(1.0, width),
            ),
            Side::Left => Rectangle::new(
                Point::new(
                    bounds.x + bounds.width + depth,
                    bounds.y + placed.arrow_center - half,
                ),
                Size::new(1.0, width),
            ),
        };

        renderer.fill_quad(
            renderer::Quad {
                bounds: rectangle,
                ..renderer::Quad::default()
            },
            color,
        );
    };

    if style.border.width > 0.0 {
        let outline = iced::Background::Color(style.border.color);

        for step in 0..=steps {
            let depth = step as f32;
            strip(depth, width_at(depth) + 2.0, outline);
        }
    }

    for step in -1..steps {
        let depth = step as f32;
        strip(depth, width_at(depth + 1.0), background);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VIEWPORT: Size = Size::new(800.0, 600.0);
    const PANEL: Size = Size::new(200.0, 100.0);

    fn geometry(side: Side) -> Geometry {
        Geometry {
            side,
            alignment: Alignment::Start,
            direction: ReadingDirection::LeftToRight,
            gap: 4.0,
            margin: 8.0,
            arrow: None,
            radius: 12.0,
        }
    }

    fn anchor(x: f32, y: f32) -> Rectangle {
        Rectangle::new(Point::new(x, y), Size::new(80.0, 34.0))
    }

    fn placed(anchor: Rectangle, geometry: &Geometry) -> Placed {
        let side = geometry.side_for(PANEL, anchor, VIEWPORT);

        place(anchor, PANEL, VIEWPORT, side, geometry)
    }

    #[test]
    fn opens_below_when_it_fits() {
        let placed = placed(anchor(10.0, 10.0), &geometry(Side::Bottom));

        assert_eq!(placed.side, Side::Bottom);
        assert_eq!(placed.position, Point::new(10.0, 48.0));
    }

    #[test]
    fn flips_when_the_preferred_side_is_short() {
        let below = placed(anchor(10.0, 540.0), &geometry(Side::Bottom));
        assert_eq!(below.side, Side::Top);
        assert_eq!(below.position, Point::new(10.0, 436.0));

        let left = placed(anchor(20.0, 200.0), &geometry(Side::Left));
        assert_eq!(left.side, Side::Right);
        assert_eq!(left.position.x, 104.0);
    }

    #[test]
    fn keeps_the_preferred_side_when_the_opposite_is_shorter() {
        let tall = Size::new(200.0, 700.0);
        let geometry = geometry(Side::Bottom);
        let anchor = anchor(10.0, 100.0);

        assert_eq!(geometry.side_for(tall, anchor, VIEWPORT), Side::Bottom);
    }

    #[test]
    fn stays_a_margin_inside_the_viewport() {
        let right = placed(anchor(700.0, 10.0), &geometry(Side::Bottom));
        assert_eq!(right.position.x, 592.0);

        let low = placed(anchor(300.0, 560.0), &geometry(Side::Right));
        assert_eq!(low.position.y, 492.0);
    }

    #[test]
    fn aligns_center_and_end_and_mirrors_in_right_to_left() {
        let anchor = anchor(300.0, 10.0);
        let center = Geometry {
            alignment: Alignment::Center,
            ..geometry(Side::Bottom)
        };
        let end = Geometry {
            alignment: Alignment::End,
            ..geometry(Side::Bottom)
        };
        let start_rtl = Geometry {
            direction: ReadingDirection::RightToLeft,
            ..geometry(Side::Bottom)
        };

        assert_eq!(placed(anchor, &center).position.x, 240.0);
        assert_eq!(placed(anchor, &end).position.x, 180.0);
        assert_eq!(placed(anchor, &start_rtl).position.x, 180.0);
    }

    #[test]
    fn start_and_end_follow_the_reading_direction() {
        let ltr = ReadingDirection::LeftToRight;
        let rtl = ReadingDirection::RightToLeft;

        assert_eq!(Side::of(Placement::Start, ltr), Side::Left);
        assert_eq!(Side::of(Placement::End, ltr), Side::Right);
        assert_eq!(Side::of(Placement::Start, rtl), Side::Right);
        assert_eq!(Side::of(Placement::End, rtl), Side::Left);
    }

    #[test]
    fn the_arrow_points_at_the_anchor_and_clears_the_corners() {
        let geometry = Geometry {
            arrow: Some(ARROW),
            ..geometry(Side::Bottom)
        };

        let middle = placed(anchor(300.0, 10.0), &geometry);
        assert_eq!(middle.position.y, 10.0 + 34.0 + 4.0 + ARROW.height);
        assert_eq!(middle.arrow_center, 40.0);

        let edge = placed(anchor(760.0, 10.0), &geometry);
        assert_eq!(edge.arrow_center, PANEL.width - 12.0 - ARROW.width / 2.0);

        let panel = Rectangle::new(edge.position, PANEL);
        assert_eq!(geometry.placed_at(anchor(760.0, 10.0), panel), edge);
    }
}
