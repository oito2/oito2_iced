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

//! Top-level navigation between pages that adapts to the available width
//! and the input.
//!
//! - With the touch [`density`] set, below the compact
//!   breakpoint, the destinations are a bottom navigation bar.
//! - Otherwise they are a [navigation rail](super::navigation_rail) at the
//!   start side: compact (icons only) below the compact breakpoint,
//!   labelled up to the extended breakpoint, and extended (labels beside
//!   the icons) from it. [`NavigationPage::rail_style`] forces one style.
//!
//! The layout is chosen from the width at layout time, so the page keeps
//! its state when the window crosses a breakpoint. The selection lives in
//! the application: the page shows `selected`, the view of the selected
//! destination, and publishes the index of a chosen destination through
//! [`NavigationPage::on_select`].
//!
//! ```no_run
//! use oito2_iced::icon::lucide;
//! use oito2_iced::widget::{navigation_page, navigation_rail};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Go(usize),
//! }
//!
//! # fn view<'a>(selected: usize) -> oito2_iced::Element<'a, Message> {
//! navigation_page::navigation_page(
//!     [
//!         navigation_rail::item(lucide::HOUSE, "Home"),
//!         navigation_rail::item(lucide::SETTINGS, "Settings"),
//!     ],
//!     selected,
//!     iced::widget::text(format!("Page {selected}")),
//! )
//! .on_select(Message::Go)
//! .into()
//! # }
//! ```

use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::widget::{column, container, space};
use iced::{
    Alignment, Background, Color, Event, Length, Padding, Point, Rectangle, Size, Vector, mouse,
};

use super::navigation_rail::{
    NavigationItem, NavigationRailStyle, glyph, label_text, rail_background, rail_strip,
};
use super::tab_bar::{Look, Strip, Target};
use crate::density::{self, Density};
use crate::theme::HOVER_OPACITY;
use crate::tokens::typography;
use crate::{Element, Theme};

/// The default width below which touch layouts use a bottom bar and
/// pointer layouts a compact rail.
pub const COMPACT_BREAKPOINT: f32 = 600.0;

/// The height of the bottom navigation bar.
pub const BAR_HEIGHT: f32 = 80.0;

/// The size of the pill behind the selected destination of the bottom bar.
const BAR_INDICATOR: Size = Size::new(64.0, 32.0);

/// The width of the line between the rail and the page.
const SEPARATOR_WIDTH: f32 = 1.0;

/// How the destinations of a [`NavigationPage`] are shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Arrangement {
    /// A bottom navigation bar below the page.
    Bar,
    /// A rail in the given style beside the page.
    Rail(NavigationRailStyle),
}

/// The arrangement for a page `width` wide: a bar on `touch` layouts below
/// `compact`, otherwise a rail compact below `compact`, extended from
/// `extended` and labelled between; `forced` picks the rail style instead.
pub(crate) fn arrangement(
    width: f32,
    touch: bool,
    compact: f32,
    extended: Option<f32>,
    forced: Option<NavigationRailStyle>,
) -> Arrangement {
    if let Some(style) = forced {
        return Arrangement::Rail(style);
    }

    if width < compact {
        if touch {
            Arrangement::Bar
        } else {
            Arrangement::Rail(NavigationRailStyle::Compact)
        }
    } else if extended.is_some_and(|extended| width >= extended) {
        Arrangement::Rail(NavigationRailStyle::LabelledExtended)
    } else {
        Arrangement::Rail(NavigationRailStyle::Labelled)
    }
}

/// Pages with navigation that adapts to the width. See
/// [`navigation_page`].
pub struct NavigationPage<'a, Message> {
    items: Vec<NavigationItem<'a>>,
    selected: usize,
    page: Element<'a, Message>,
    on_select: Option<Rc<dyn Fn(usize) -> Message + 'a>>,
    leading: Option<Element<'a, Message>>,
    trailing: Option<Element<'a, Message>>,
    compact_breakpoint: f32,
    extended_breakpoint: Option<f32>,
    rail_style: Option<NavigationRailStyle>,
    rail_background: Option<Color>,
    rail_padding: Padding,
    separator: bool,
}

/// Navigation between the destinations `items`, with the one at `selected`
/// selected and its view `page` beside or above the destinations. Without
/// [`NavigationPage::on_select`] the destinations are disabled.
pub fn navigation_page<'a, Message>(
    items: impl IntoIterator<Item = NavigationItem<'a>>,
    selected: usize,
    page: impl Into<Element<'a, Message>>,
) -> NavigationPage<'a, Message> {
    NavigationPage {
        items: items.into_iter().collect(),
        selected,
        page: page.into(),
        on_select: None,
        leading: None,
        trailing: None,
        compact_breakpoint: COMPACT_BREAKPOINT,
        extended_breakpoint: None,
        rail_style: None,
        rail_background: None,
        rail_padding: Padding::from([8.0, 0.0]),
        separator: true,
    }
}

impl<'a, Message> NavigationPage<'a, Message> {
    /// Sets the function that turns the index of a chosen destination into
    /// a message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Rc::new(on_select));
        self
    }

    /// Places `leading` above the rail destinations.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Places `trailing` below the rail destinations.
    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Sets the width below which touch layouts use a bottom bar and
    /// pointer layouts a compact rail ([`COMPACT_BREAKPOINT`] by default).
    #[must_use]
    pub fn compact_breakpoint(mut self, width: f32) -> Self {
        self.compact_breakpoint = width;
        self
    }

    /// Sets the width from which the rail shows labels beside the icons;
    /// `None`, the default, never extends it.
    #[must_use]
    pub fn extended_breakpoint(mut self, width: Option<f32>) -> Self {
        self.extended_breakpoint = width;
        self
    }

    /// Forces a rail style instead of choosing the layout by width.
    #[must_use]
    pub fn rail_style(mut self, style: Option<NavigationRailStyle>) -> Self {
        self.rail_style = style;
        self
    }

    /// Sets the rail background, instead of the chrome sidebar color.
    #[must_use]
    pub fn rail_background(mut self, color: Color) -> Self {
        self.rail_background = Some(color);
        self
    }

    /// Sets the space around the rail destinations (8 pixels above and
    /// below by default).
    #[must_use]
    pub fn rail_padding(mut self, padding: impl Into<Padding>) -> Self {
        self.rail_padding = padding.into();
        self
    }

    /// Sets whether a line separates the rail from the page (true by
    /// default).
    #[must_use]
    pub fn separator(mut self, separator: bool) -> Self {
        self.separator = separator;
        self
    }
}

/// The content color of a bottom bar destination icon.
fn bar_icon_color(theme: &Theme, selected: bool) -> Color {
    let roles = theme.roles();

    if selected {
        roles.on_secondary_container
    } else {
        roles.on_surface_variant
    }
}

/// The pill behind the selected bottom bar destination.
fn bar_indicator(theme: &Theme) -> Color {
    theme.roles().secondary_container
}

/// The hover layer of a bottom bar destination.
fn bar_hover(theme: &Theme) -> Color {
    theme.roles().on_surface.scale_alpha(HOVER_OPACITY)
}

/// The bottom navigation bar of `items`.
fn bottom_bar<'a, Message: Clone + 'a>(
    items: &[NavigationItem<'a>],
    selected: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
) -> Element<'a, Message> {
    let look = Look {
        marker: Some(bar_indicator),
        marker_inset: 0.0,
        slide: false,
        radius: BAR_INDICATOR.height / 2.0,
        hover: bar_hover,
        text: bar_icon_color,
        target: Target::FirstChild,
    };

    let enabled = on_select.is_some();
    let mut strip = Strip::new(
        items
            .iter()
            .enumerate()
            .map(|(index, item)| {
                let selected = index == selected;
                let color: fn(&Theme) -> Color = if selected {
                    |theme| theme.roles().on_surface
                } else {
                    |theme| theme.roles().on_surface_variant
                };

                column![
                    container(glyph(item.glyph(selected), density::icon_size()))
                        .center_x(BAR_INDICATOR.width)
                        .center_y(BAR_INDICATOR.height),
                    label_text(
                        item.label().to_owned().into(),
                        typography::LABEL_MEDIUM,
                        color,
                        enabled,
                    ),
                ]
                .spacing(4.0)
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into()
            })
            .collect(),
        look,
    );
    strip.selected = Some(selected);
    strip.on_select = on_select;
    strip.fill = true;
    strip.main = Length::Fill;

    container(strip)
        .width(Length::Fill)
        .center_y(BAR_HEIGHT)
        .style(|theme| rail_background(theme, None))
        .into()
}

impl<'a, Message> From<NavigationPage<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(page: NavigationPage<'a, Message>) -> Self {
        let selected = page.selected.min(page.items.len().saturating_sub(1));
        let callback = |on_select: &Option<Rc<dyn Fn(usize) -> Message + 'a>>| {
            on_select.clone().map(|on_select| {
                Box::new(move |index: usize| on_select(index)) as Box<dyn Fn(usize) -> Message + 'a>
            })
        };

        let arrangements: Vec<Arrangement> = match page.rail_style {
            Some(style) => vec![Arrangement::Rail(style)],
            None => {
                let mut list = vec![
                    Arrangement::Rail(NavigationRailStyle::Compact),
                    Arrangement::Rail(NavigationRailStyle::Labelled),
                ];
                if page.extended_breakpoint.is_some() {
                    list.push(Arrangement::Rail(NavigationRailStyle::LabelledExtended));
                }
                list.push(Arrangement::Bar);
                list
            }
        };

        let mut children: Vec<Element<'a, Message>> = arrangements
            .iter()
            .map(|arrangement| match *arrangement {
                Arrangement::Bar => bottom_bar(&page.items, selected, callback(&page.on_select)),
                Arrangement::Rail(style) => rail_strip(
                    &page.items,
                    Some(selected),
                    style,
                    callback(&page.on_select),
                )
                .into(),
            })
            .collect();

        children.push(page.leading.unwrap_or_else(|| space().into()));
        children.push(page.trailing.unwrap_or_else(|| space().into()));
        children.push(page.page);

        Element::new(Adaptive {
            children,
            arrangements,
            compact: page.compact_breakpoint,
            extended: page.extended_breakpoint,
            forced: page.rail_style,
            background: page.rail_background,
            padding: page.rail_padding,
            separator: page.separator,
        })
    }
}

/// The layout of a [`NavigationPage`]: one destination view for each
/// arrangement it can take, then the leading and trailing rail elements and
/// the page. Only the arrangement chosen for the width takes part.
struct Adaptive<'a, Message> {
    children: Vec<Element<'a, Message>>,
    arrangements: Vec<Arrangement>,
    compact: f32,
    extended: Option<f32>,
    forced: Option<NavigationRailStyle>,
    background: Option<Color>,
    padding: Padding,
    separator: bool,
}

/// The arrangement chosen at the last layout.
#[derive(Debug, Clone, Copy, Default)]
struct State {
    active: usize,
}

impl<Message> Adaptive<'_, Message> {
    fn leading(&self) -> usize {
        self.arrangements.len()
    }

    fn trailing(&self) -> usize {
        self.arrangements.len() + 1
    }

    fn page(&self) -> usize {
        self.arrangements.len() + 2
    }

    /// The children that take part in the `active` arrangement, in focus
    /// order.
    fn visible(&self, active: usize) -> Vec<usize> {
        match self.arrangements.get(active) {
            Some(Arrangement::Bar) => vec![self.page(), active],
            _ => vec![self.leading(), active, self.trailing(), self.page()],
        }
    }

    /// The bounds of the rail, if the `active` arrangement has one.
    fn rail_bounds(&self, active: usize, bounds: Rectangle) -> Option<Rectangle> {
        match self.arrangements.get(active)? {
            Arrangement::Rail(style) => Some(Rectangle {
                width: style.width(),
                ..bounds
            }),
            Arrangement::Bar => None,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Adaptive<'_, Message> {
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
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.max();
        let touch = density::get() == Density::Touch;
        let chosen = arrangement(size.width, touch, self.compact, self.extended, self.forced);
        let active = self
            .arrangements
            .iter()
            .position(|candidate| *candidate == chosen)
            .unwrap_or(0);
        tree.state.downcast_mut::<State>().active = active;

        let mut nodes: Vec<layout::Node> = self
            .children
            .iter()
            .map(|_| layout::Node::new(Size::ZERO))
            .collect();
        let (leading, trailing, page) = (self.leading(), self.trailing(), self.page());
        let mut place = |index: usize, limits: layout::Limits, at: &dyn Fn(Size) -> Point| {
            if let (Some(child), Some(state)) =
                (self.children.get_mut(index), tree.children.get_mut(index))
            {
                let node = child.as_widget_mut().layout(state, renderer, &limits);
                let point = at(node.size());
                nodes[index] = node.move_to(point);
                nodes[index].size()
            } else {
                Size::ZERO
            }
        };
        let exact = |size: Size| layout::Limits::new(size, size);

        match self.arrangements.get(active).copied() {
            Some(Arrangement::Bar) => {
                let bar = place(
                    active,
                    layout::Limits::new(Size::ZERO, size),
                    &|bar: Size| Point::new(0.0, (size.height - bar.height).max(0.0)),
                );
                place(
                    page,
                    exact(Size::new(size.width, (size.height - bar.height).max(0.0))),
                    &|_| Point::ORIGIN,
                );
            }
            Some(Arrangement::Rail(style)) => {
                let padding = self.padding;
                let width = style.width();
                let inner = (width - padding.left - padding.right).max(0.0);
                let extended = style == NavigationRailStyle::LabelledExtended;
                let x = move |child: Size| {
                    if extended {
                        padding.left
                    } else {
                        padding.left + (inner - child.width).max(0.0) / 2.0
                    }
                };
                let free = layout::Limits::new(Size::ZERO, Size::new(inner, size.height));

                let top = place(leading, free, &|child| Point::new(x(child), padding.top));
                let bottom = place(trailing, free, &|child| {
                    Point::new(x(child), size.height - padding.bottom - child.height)
                });
                let strip = Size::new(
                    inner,
                    (size.height - padding.top - padding.bottom - top.height - bottom.height)
                        .max(0.0),
                );
                place(active, exact(strip), &|_| {
                    Point::new(padding.left, padding.top + top.height)
                });

                let separator = if self.separator { SEPARATOR_WIDTH } else { 0.0 };
                let offset = width + separator;
                place(
                    page,
                    exact(Size::new((size.width - offset).max(0.0), size.height)),
                    &|_| Point::new(offset, 0.0),
                );
            }
            None => {
                place(page, exact(size), &|_| Point::ORIGIN);
            }
        }

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let visible = self.visible(tree.state.downcast_ref::<State>().active);
        let layouts: Vec<Layout<'_>> = layout.children().collect();

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for &index in &visible {
                if let (Some(child), Some(state), Some(layout)) = (
                    self.children.get_mut(index),
                    tree.children.get_mut(index),
                    layouts.get(index),
                ) {
                    child
                        .as_widget_mut()
                        .operate(state, *layout, renderer, operation);
                }
            }
        });
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
        let visible = self.visible(tree.state.downcast_ref::<State>().active);

        for (index, ((child, state), layout)) in self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
        {
            if visible.contains(&index) {
                child.as_widget_mut().update(
                    state, event, layout, cursor, renderer, clipboard, shell, viewport,
                );
            }
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
        use iced::advanced::Renderer as _;

        let active = tree.state.downcast_ref::<State>().active;
        let visible = self.visible(active);
        let bounds = layout.bounds();

        if let Some(rail) = self.rail_bounds(active, bounds) {
            let background = rail_background(theme, self.background)
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT));

            renderer.fill_quad(
                renderer::Quad {
                    bounds: rail,
                    ..renderer::Quad::default()
                },
                background,
            );

            if self.separator {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: rail.x + rail.width,
                            width: SEPARATOR_WIDTH,
                            ..rail
                        },
                        ..renderer::Quad::default()
                    },
                    theme.roles().outline_variant,
                );
            }
        }

        for (index, ((child, state), layout)) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .enumerate()
        {
            if visible.contains(&index) {
                child
                    .as_widget()
                    .draw(state, renderer, theme, style, layout, cursor, viewport);
            }
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
        let visible = self.visible(tree.state.downcast_ref::<State>().active);

        self.children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .enumerate()
            .filter(|(index, _)| visible.contains(index))
            .map(|(_, ((child, state), layout))| {
                child
                    .as_widget()
                    .mouse_interaction(state, layout, cursor, viewport, renderer)
            })
            .max()
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
        let visible = self.visible(tree.state.downcast_ref::<State>().active);

        let overlays: Vec<_> = self
            .children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .enumerate()
            .filter(|(index, _)| visible.contains(index))
            .filter_map(|(_, ((child, state), layout))| {
                child
                    .as_widget_mut()
                    .overlay(state, layout, renderer, viewport, translation)
            })
            .collect();

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPACT: NavigationRailStyle = NavigationRailStyle::Compact;
    const LABELLED: NavigationRailStyle = NavigationRailStyle::Labelled;
    const EXTENDED: NavigationRailStyle = NavigationRailStyle::LabelledExtended;

    #[test]
    fn narrow_layouts_use_a_bar_on_touch_and_a_compact_rail_on_pointer() {
        assert_eq!(
            arrangement(400.0, true, 600.0, None, None),
            Arrangement::Bar
        );
        assert_eq!(
            arrangement(400.0, false, 600.0, None, None),
            Arrangement::Rail(COMPACT)
        );
    }

    #[test]
    fn the_rail_extends_from_the_extended_breakpoint() {
        assert_eq!(
            arrangement(800.0, false, 600.0, None, None),
            Arrangement::Rail(LABELLED)
        );
        assert_eq!(
            arrangement(900.0, true, 600.0, Some(900.0), None),
            Arrangement::Rail(EXTENDED)
        );
        assert_eq!(
            arrangement(899.0, false, 600.0, Some(900.0), None),
            Arrangement::Rail(LABELLED)
        );
    }

    #[test]
    fn a_forced_style_wins_over_the_width() {
        assert_eq!(
            arrangement(300.0, true, 600.0, None, Some(EXTENDED)),
            Arrangement::Rail(EXTENDED)
        );
    }
}
