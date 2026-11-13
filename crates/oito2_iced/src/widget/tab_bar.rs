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

//! Horizontal and vertical tab bars.
//!
//! A [`TabBar`] is a row of tabs with a rounded marker behind the selected
//! one that slides when the selection changes; a [`VerticalTabBar`] is the
//! same as a column, and [`VerticalTabs`] puts a vertical bar beside the
//! content of the selected tab. [`TabBarStyle`] picks the marker look: a
//! pill in the secondary container color, or a faint accent tint.
//!
//! A bar is one stop in keyboard navigation: Tab focuses the selected tab,
//! the arrow keys of its orientation select the previous or next tab
//! (stopping at the ends), Home and End select the first and last tab, and
//! Space or Enter publish the focused tab again. Tabs that don't fit the
//! bar scroll with the mouse wheel, and the bar scrolls to keep the
//! selected and the focused tab in view. The selection lives in the
//! application: the bar shows `selected` and publishes the index of a
//! chosen tab through [`TabBar::on_select`].
//!
//! ```no_run
//! use oito2_iced::widget::tab_bar;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Tab(usize),
//! }
//!
//! # fn view<'a>(selected: usize) -> oito2_iced::Element<'a, Message> {
//! tab_bar::tab_bar(
//!     [tab_bar::tab("General"), tab_bar::tab("About")],
//!     selected,
//! )
//! .on_select(Message::Tab)
//! .into()
//! # }
//! ```

use std::borrow::Cow;
use std::time::Instant;

#[cfg(feature = "svg")]
use iced::Alignment;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::text::{IntoFragment, Wrapping};
use iced::widget::{container, row};
use iced::{
    Border, Color, Event, Length, Padding, Point, Rectangle, Size, Vector, mouse, touch, window,
};

use super::min_size::{Minimum, min_size};
use super::styled_text;
use crate::animation::{duration, ease_in_out_cubic};
use crate::icon::Icon;
#[cfg(feature = "svg")]
use crate::icon::icon;
use crate::theme::DISABLED_CONTENT_OPACITY;
#[cfg(feature = "svg")]
use crate::tokens::sizes;
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// The opacity of the hover layer on a tab.
const TAB_HOVER_OPACITY: f32 = 0.06;

/// The opacity of the accent tint behind the selected tab of the subtle
/// style.
const SUBTLE_FILL_OPACITY: f32 = 0.12;

/// The default width of the tab column of [`VerticalTabs`].
pub const VERTICAL_TABS_WIDTH: f32 = 200.0;

/// The distance one line of a mouse wheel scrolls.
const WHEEL_LINE: f32 = 60.0;

/// The look of a tab bar.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TabBarStyle {
    /// A pill in the secondary container color behind the selected tab, as
    /// in GNOME view switchers.
    #[default]
    Pill,
    /// The selected tab tinted with a faint accent fill and accent text,
    /// the others plain; the tabs keep their own width and start at the
    /// leading edge.
    Subtle,
}

/// A tab of a tab bar: a label with an optional icon before it. Built by
/// [`tab`].
#[derive(Debug, Clone)]
pub struct Tab<'a> {
    label: Cow<'a, str>,
    icon: Option<Icon>,
}

/// A tab showing `label`.
pub fn tab<'a>(label: impl IntoFragment<'a>) -> Tab<'a> {
    Tab {
        label: label.into_fragment(),
        icon: None,
    }
}

impl Tab<'_> {
    /// Places `icon` before the label, at the icon size.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }
}

/// How a tab is aligned in its slot.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Placement {
    /// Its own width, for a scrolling row.
    Natural,
    /// Centered in an equal share of the row.
    Centered,
    /// The full width of a column, content at the start.
    Start,
}

/// The content of `tab`: the icon and the label in a slot as tall as a
/// control, with the medium horizontal padding.
fn tab_content<'a, Message: 'a>(tab: Tab<'a>, placement: Placement) -> Element<'a, Message> {
    let label = styled_text(tab.label, typography::LABEL_LARGE).wrapping(Wrapping::None);

    #[cfg(feature = "svg")]
    let content: Element<'a, Message> = match tab.icon {
        Some(glyph) => row![icon(glyph).size(sizes::ICON_SIZE), label]
            .spacing(spacing::SM)
            .align_y(Alignment::Center)
            .into(),
        None => label.into(),
    };
    #[cfg(not(feature = "svg"))]
    let content: Element<'a, Message> = {
        let _ = tab.icon;
        label.into()
    };

    let content = min_size(content, Minimum::None, Minimum::Control(0.0));
    let slot = container(content).padding(Padding::from([0.0, spacing::MD]));

    match placement {
        Placement::Natural => slot.into(),
        Placement::Centered => slot.center_x(Length::Fill).into(),
        Placement::Start => slot.align_left(Length::Fill).into(),
    }
}

/// The marker color of the pill style.
fn pill_marker(theme: &Theme) -> Color {
    theme.roles().secondary_container
}

/// The marker color of the subtle style.
fn subtle_marker(theme: &Theme) -> Color {
    theme
        .roles()
        .semantic
        .primary_text
        .scale_alpha(SUBTLE_FILL_OPACITY)
}

/// The label color of a pill style tab.
fn pill_text(theme: &Theme, selected: bool) -> Color {
    let roles = theme.roles();

    if selected {
        roles.on_secondary_container
    } else {
        roles.on_surface_variant
    }
}

/// The label color of a subtle style tab.
fn subtle_text(theme: &Theme, selected: bool) -> Color {
    let roles = theme.roles();

    if selected {
        roles.semantic.primary_text
    } else {
        roles.on_surface_variant
    }
}

/// The hover layer of a tab.
fn tab_hover(theme: &Theme) -> Color {
    theme.roles().on_surface.scale_alpha(TAB_HOVER_OPACITY)
}

/// The look of the tabs of `style`, with the subtle marker inset by
/// `inset` on the sides.
fn tab_look(style: TabBarStyle, inset: f32) -> Look {
    let look = Look {
        marker: Some(pill_marker),
        marker_inset: 0.0,
        slide: true,
        radius: radius::MEDIUM,
        hover: tab_hover,
        text: pill_text,
        target: Target::Item,
    };

    match style {
        TabBarStyle::Pill => look,
        TabBarStyle::Subtle => Look {
            marker: Some(subtle_marker),
            marker_inset: inset,
            text: subtle_text,
            ..look
        },
    }
}

/// A row of tabs. See [`tab_bar`].
pub struct TabBar<'a, Message> {
    tabs: Vec<Tab<'a>>,
    selected: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    scrollable: bool,
    style: TabBarStyle,
}

/// A row of `tabs` with the tab at `selected` selected. Without
/// [`TabBar::on_select`] the bar is disabled.
pub fn tab_bar<'a, Message>(
    tabs: impl IntoIterator<Item = Tab<'a>>,
    selected: usize,
) -> TabBar<'a, Message> {
    TabBar {
        tabs: tabs.into_iter().collect(),
        selected,
        on_select: None,
        scrollable: false,
        style: TabBarStyle::Pill,
    }
}

impl<'a, Message> TabBar<'a, Message> {
    /// Sets the function that turns the index of a chosen tab into a
    /// message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or disables the bar with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets whether the tabs keep their own width, start at the leading
    /// edge and scroll, instead of sharing the bar width equally (false by
    /// default). The subtle style always scrolls.
    #[must_use]
    pub fn scrollable(mut self, scrollable: bool) -> Self {
        self.scrollable = scrollable;
        self
    }

    /// Sets the look of the selected tab marker.
    #[must_use]
    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }
}

impl<'a, Message> From<TabBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: TabBar<'a, Message>) -> Self {
        let scrollable = bar.scrollable || bar.style == TabBarStyle::Subtle;
        let placement = if scrollable {
            Placement::Natural
        } else {
            Placement::Centered
        };

        let mut strip = Strip::new(
            bar.tabs
                .into_iter()
                .map(|tab| tab_content(tab, placement))
                .collect(),
            tab_look(bar.style, spacing::XXS),
        );
        strip.selected = Some(bar.selected);
        strip.on_select = bar.on_select;
        strip.fill = !scrollable;
        strip.main = Length::Fill;
        strip.select_on_move = true;

        container(strip).padding(spacing::XS).into()
    }
}

/// A column of tabs. See [`vertical_tab_bar`].
pub struct VerticalTabBar<'a, Message> {
    tabs: Vec<Tab<'a>>,
    selected: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    style: TabBarStyle,
}

/// A column of `tabs` with the tab at `selected` selected, for side
/// navigation between views of the same content. The tabs take the full
/// width, line up at the leading edge and are separated by the extra small
/// spacing. Without [`VerticalTabBar::on_select`] the bar is disabled.
pub fn vertical_tab_bar<'a, Message>(
    tabs: impl IntoIterator<Item = Tab<'a>>,
    selected: usize,
) -> VerticalTabBar<'a, Message> {
    VerticalTabBar {
        tabs: tabs.into_iter().collect(),
        selected,
        on_select: None,
        style: TabBarStyle::Pill,
    }
}

impl<'a, Message> VerticalTabBar<'a, Message> {
    /// Sets the function that turns the index of a chosen tab into a
    /// message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or disables the bar with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets the look of the selected tab marker.
    #[must_use]
    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.style = style;
        self
    }

    /// The tab column as a strip, `main` tall with `padding` around it.
    fn strip(self, main: Length, padding: f32) -> Strip<'a, Message>
    where
        Message: 'a,
    {
        let mut strip = Strip::new(
            self.tabs
                .into_iter()
                .map(|tab| tab_content(tab, Placement::Start))
                .collect(),
            tab_look(self.style, 0.0),
        );
        strip.selected = Some(self.selected);
        strip.on_select = self.on_select;
        strip.vertical = true;
        strip.spacing = spacing::XXS;
        strip.padding = Padding::new(padding);
        strip.main = main;
        strip.select_on_move = true;
        strip
    }
}

impl<'a, Message> From<VerticalTabBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: VerticalTabBar<'a, Message>) -> Self {
        Element::new(bar.strip(Length::Shrink, 0.0))
    }
}

/// A vertical tab bar beside the content of the selected tab. See
/// [`vertical_tabs`].
pub struct VerticalTabs<'a, Message> {
    bar: VerticalTabBar<'a, Message>,
    content: Element<'a, Message>,
    width: f32,
}

/// A [`vertical_tab_bar`] of `tabs` at the start side, with `content`, the
/// view of the tab at `selected`, filling the rest. The tab column is
/// [`VERTICAL_TABS_WIDTH`] wide and scrolls when the tabs don't fit its
/// height.
pub fn vertical_tabs<'a, Message>(
    tabs: impl IntoIterator<Item = Tab<'a>>,
    selected: usize,
    content: impl Into<Element<'a, Message>>,
) -> VerticalTabs<'a, Message> {
    VerticalTabs {
        bar: vertical_tab_bar(tabs, selected),
        content: content.into(),
        width: VERTICAL_TABS_WIDTH,
    }
}

impl<'a, Message> VerticalTabs<'a, Message> {
    /// Sets the function that turns the index of a chosen tab into a
    /// message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.bar = self.bar.on_select(on_select);
        self
    }

    /// Sets the select function, or disables the bar with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.bar = self.bar.on_select_maybe(on_select);
        self
    }

    /// Sets the look of the selected tab marker.
    #[must_use]
    pub fn style(mut self, style: TabBarStyle) -> Self {
        self.bar = self.bar.style(style);
        self
    }

    /// Sets the width of the tab column.
    #[must_use]
    pub fn tab_bar_width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

impl<'a, Message> From<VerticalTabs<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(tabs: VerticalTabs<'a, Message>) -> Self {
        let bar = container(tabs.bar.strip(Length::Fill, spacing::XS))
            .width(tabs.width)
            .height(Length::Fill);

        row![
            bar,
            container(tabs.content)
                .width(Length::Fill)
                .height(Length::Fill)
        ]
        .height(Length::Fill)
        .into()
    }
}

/// What the marker, the hover layer and the focus ring of a strip item
/// cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Target {
    /// The whole item.
    Item,
    /// The first child of the item, such as the pill above a label.
    FirstChild,
}

/// The colors and shapes of a [`Strip`].
#[derive(Debug, Clone, Copy)]
pub(crate) struct Look {
    /// The color of the marker behind the selected item; `None` draws no
    /// marker.
    pub(crate) marker: Option<fn(&Theme) -> Color>,
    /// How far the marker is inset from the sides of its target.
    pub(crate) marker_inset: f32,
    /// Whether the marker slides to a newly selected item.
    pub(crate) slide: bool,
    /// The corner radius of the marker, the hover layer and the focus ring.
    pub(crate) radius: f32,
    /// The color of the hover layer.
    pub(crate) hover: fn(&Theme) -> Color,
    /// The content color of an item, given whether it is selected.
    pub(crate) text: fn(&Theme, bool) -> Color,
    /// What the marker, the hover layer and the focus ring cover.
    pub(crate) target: Target,
}

/// A row or column of selectable items that is one stop in keyboard
/// navigation, with a marker behind the selected item and scrolling along
/// its axis when the items don't fit. Shared by the tab bars and the
/// navigation rails.
pub(crate) struct Strip<'a, Message> {
    /// The items, in order.
    pub(crate) items: Vec<Element<'a, Message>>,
    /// Turns the index of a chosen item into a message; `None` disables the
    /// strip.
    pub(crate) on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    /// The selected item.
    pub(crate) selected: Option<usize>,
    /// Whether the items run down instead of across.
    pub(crate) vertical: bool,
    /// The space between items.
    pub(crate) spacing: f32,
    /// The space around the items, scrolled with them.
    pub(crate) padding: Padding,
    /// Whether the items share the length of the strip equally instead of
    /// keeping their own and scrolling.
    pub(crate) fill: bool,
    /// The length of the strip along its axis.
    pub(crate) main: Length,
    /// Whether moving the focus with the keyboard also selects.
    pub(crate) select_on_move: bool,
    /// The colors and shapes.
    pub(crate) look: Look,
}

impl<'a, Message> Strip<'a, Message> {
    /// A disabled horizontal strip of `items` with nothing selected.
    pub(crate) fn new(items: Vec<Element<'a, Message>>, look: Look) -> Self {
        Self {
            items,
            on_select: None,
            selected: None,
            vertical: false,
            spacing: 0.0,
            padding: Padding::ZERO,
            fill: false,
            main: Length::Shrink,
            select_on_move: false,
            look,
        }
    }
}

/// The sliding marker of a [`Strip`].
#[derive(Debug, Clone, Copy)]
struct Marker {
    index: usize,
    /// The rectangle the marker slides from, relative to the strip.
    from: Option<Rectangle>,
    started: Option<Instant>,
    progress: f32,
}

/// The focus, scroll and marker state of a [`Strip`].
#[derive(Debug, Clone, Copy, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the ring.
    keyboard: bool,
    /// The item the focus moved to since the strip gained the focus.
    index: Option<usize>,
    /// The item a press started on.
    pressed: Option<usize>,
    /// How far the items are scrolled.
    offset: f32,
    /// The selection the strip last scrolled into view.
    revealed: Option<Option<usize>>,
    /// An item to scroll into view at the next layout.
    reveal: Option<usize>,
    marker: Option<Marker>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
        self.index = None;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// The scroll offset that brings the span from `start` to `end` fully into
/// a viewport `viewport` long scrolled by `offset`, moving it as little as
/// possible.
pub(crate) fn reveal_offset(offset: f32, viewport: f32, start: f32, end: f32) -> f32 {
    if start < offset {
        start
    } else if end > offset + viewport {
        (end - viewport).min(start)
    } else {
        offset
    }
}

/// The index `forward` or back from `current` among `count` items, stopping
/// at the ends.
pub(crate) fn step(current: usize, count: usize, forward: bool) -> usize {
    if forward {
        (current + 1).min(count.saturating_sub(1))
    } else {
        current.saturating_sub(1)
    }
}

impl<Message> Strip<'_, Message> {
    fn enabled(&self) -> bool {
        self.on_select.is_some() && !self.items.is_empty()
    }

    fn selected(&self) -> Option<usize> {
        self.selected.filter(|&index| index < self.items.len())
    }

    /// The item that has the focus while the strip has it.
    fn current(&self, state: &State) -> Option<usize> {
        if !self.enabled() {
            return None;
        }

        state
            .index
            .filter(|&index| index < self.items.len())
            .or_else(|| self.selected())
            .or(Some(0))
    }

    /// The enabled item under the cursor, inside the strip.
    fn item_at(&self, layout: Layout<'_>, cursor: mouse::Cursor) -> Option<usize> {
        if !self.enabled() || !cursor.is_over(layout.bounds()) {
            return None;
        }

        layout
            .children()
            .position(|child| cursor.is_over(child.bounds()))
    }

    /// The bounds the marker, hover layer and ring cover for the item at
    /// `index`.
    fn target(&self, layout: Layout<'_>, index: usize) -> Option<Rectangle> {
        let item = layout.children().nth(index)?;

        Some(match self.look.target {
            Target::Item => item.bounds(),
            Target::FirstChild => item
                .children()
                .next()
                .map_or(item.bounds(), |child| child.bounds()),
        })
    }

    /// The bounds of the marker behind the item at `index`, inset.
    fn marker_target(&self, layout: Layout<'_>, index: usize) -> Option<Rectangle> {
        self.target(layout, index).map(|bounds| Rectangle {
            x: bounds.x + self.look.marker_inset,
            width: (bounds.width - 2.0 * self.look.marker_inset).max(0.0),
            ..bounds
        })
    }

    /// The bounds of the marker, between its old and new place while it
    /// slides.
    fn marker_bounds(&self, state: &State, layout: Layout<'_>) -> Option<Rectangle> {
        let index = self.selected()?;
        let target = self.marker_target(layout, index)?;

        let Some(Marker {
            from: Some(from),
            progress,
            ..
        }) = state.marker.filter(|marker| marker.index == index)
        else {
            return Some(target);
        };

        let origin = layout.position();
        let t = ease_in_out_cubic(progress.clamp(0.0, 1.0));
        let lerp = |a: f32, b: f32| a + (b - a) * t;

        Some(Rectangle {
            x: lerp(origin.x + from.x, target.x),
            y: lerp(origin.y + from.y, target.y),
            width: lerp(from.width, target.width),
            height: lerp(from.height, target.height),
        })
    }

    /// Starts sliding the marker when the selection changed since the last
    /// event; the first marker shows at once.
    fn sync_marker(&self, state: &mut State, layout: Layout<'_>, shell: &mut Shell<'_, Message>) {
        let Some(target) = self.selected() else {
            state.marker = None;
            return;
        };

        match state.marker {
            Some(marker) if marker.index == target => {}
            Some(marker) if self.look.slide => {
                let origin = layout.position();
                let from = self
                    .marker_target(layout, marker.index)
                    .map(|bounds| Rectangle {
                        x: bounds.x - origin.x,
                        y: bounds.y - origin.y,
                        ..bounds
                    });

                state.marker = Some(Marker {
                    index: target,
                    from,
                    started: None,
                    progress: 0.0,
                });
                shell.request_redraw();
            }
            _ => {
                state.marker = Some(Marker {
                    index: target,
                    from: None,
                    started: None,
                    progress: 1.0,
                });
            }
        }
    }

    /// Splits `size` into its length along the strip and across it.
    fn split(&self, size: Size) -> (f32, f32) {
        if self.vertical {
            (size.height, size.width)
        } else {
            (size.width, size.height)
        }
    }

    /// Joins a length along the strip and across it into a size.
    fn join(&self, main: f32, cross: f32) -> Size {
        if self.vertical {
            Size::new(cross, main)
        } else {
            Size::new(main, cross)
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Strip<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        self.items.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.items);
    }

    fn size(&self) -> Size<Length> {
        if self.vertical {
            Size::new(Length::Fill, self.main)
        } else {
            Size::new(self.main, Length::Shrink)
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let padding = self.padding;
        let (pad_start, pad_end, pad_cross) = if self.vertical {
            (padding.top, padding.bottom, (padding.left, padding.right))
        } else {
            (padding.left, padding.right, (padding.top, padding.bottom))
        };
        let (max_main, max_cross) = self.split(limits.max());
        let available_cross = (max_cross - pad_cross.0 - pad_cross.1).max(0.0);
        let count = self.items.len();
        #[allow(clippy::cast_precision_loss)]
        let share = (self.fill && max_main.is_finite() && count > 0).then(|| {
            ((max_main - pad_start - pad_end - self.spacing * (count - 1) as f32) / count as f32)
                .max(0.0)
        });

        let item_limits = layout::Limits::new(
            Size::ZERO,
            self.join(share.unwrap_or(f32::INFINITY), available_cross),
        );
        let mut nodes: Vec<layout::Node> = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(item, tree)| item.as_widget_mut().layout(tree, renderer, &item_limits))
            .collect();

        let cross = if self.vertical && available_cross.is_finite() {
            available_cross
        } else {
            nodes
                .iter()
                .map(|node| self.split(node.size()).1)
                .fold(0.0, f32::max)
        };

        let mut starts = Vec::with_capacity(count);
        let mut position = pad_start;
        for (index, node) in nodes.iter().enumerate() {
            if index > 0 {
                position += self.spacing;
            }
            starts.push(position);
            position += self.split(node.size()).0;
        }
        let content = position + pad_end;

        let viewport = match self.main {
            Length::Shrink => content.min(max_main),
            _ if max_main.is_finite() => max_main,
            _ => content,
        };
        let max_offset = (content - viewport).max(0.0);

        let state = tree.state.downcast_mut::<State>();
        let reveal = |offset: f32, index: usize| -> f32 {
            let Some((&start, node)) = starts.get(index).zip(nodes.get(index)) else {
                return offset;
            };
            let start = if index == 0 { 0.0 } else { start };
            let end = if index + 1 == count {
                content
            } else {
                start + self.split(node.size()).0
            };

            reveal_offset(offset, viewport, start, end)
        };

        let selected = self.selected();
        if state.revealed != Some(selected) {
            state.revealed = Some(selected);
            if let Some(index) = selected {
                state.offset = reveal(state.offset, index);
            }
        }
        if let Some(index) = state.reveal.take() {
            state.offset = reveal(state.offset, index);
        }
        state.offset = state.offset.clamp(0.0, max_offset);
        let offset = state.offset;

        for (node, start) in nodes.iter_mut().zip(&starts) {
            let item_cross = self.split(node.size()).1;
            let along = start - offset;
            let across = pad_cross.0 + (cross - item_cross) / 2.0;
            let point = if self.vertical {
                Point::new(across, along)
            } else {
                Point::new(along, across)
            };
            node.move_to_mut(point);
        }

        layout::Node::with_children(
            self.join(viewport, cross + pad_cross.0 + pad_cross.1),
            nodes,
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let current = self.current(tree.state.downcast_ref::<State>());
        let state = tree.state.downcast_mut::<State>();

        match current.and_then(|index| layout.children().nth(index)) {
            Some(item) => operation.focusable(None, item.bounds(), state),
            None => state.focused = false,
        }

        operation.traverse(&mut |operation| {
            self.items
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget_mut()
                        .operate(state, layout, renderer, operation);
                });
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
        let bounds = layout.bounds();
        let inner_cursor = if cursor.is_over(bounds) {
            cursor
        } else {
            mouse::Cursor::Unavailable
        };
        let inner_viewport = bounds.intersection(viewport).unwrap_or(bounds);

        for ((child, state), child_layout) in self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            child.as_widget_mut().update(
                state,
                event,
                child_layout,
                inner_cursor,
                renderer,
                clipboard,
                shell,
                &inner_viewport,
            );
        }

        self.sync_marker(tree.state.downcast_mut::<State>(), layout, shell);

        let state = tree.state.downcast_mut::<State>();

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                match self.item_at(layout, cursor) {
                    Some(index) => {
                        state.pressed = Some(index);
                        state.index = Some(index);
                        state.focused = true;
                        state.keyboard = false;
                        shell.capture_event();
                        shell.request_redraw();
                    }
                    None if state.focused && !cursor.is_over(bounds) => {
                        state.focused = false;
                        shell.request_redraw();
                    }
                    None => {}
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(pressed) = state.pressed.take()
                    && self.item_at(layout, cursor) == Some(pressed)
                    && let Some(on_select) = &self.on_select
                {
                    shell.publish(on_select(pressed));
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if shell.is_event_captured() || !cursor.is_over(bounds) {
                    return;
                }

                let (x, y) = match *delta {
                    mouse::ScrollDelta::Lines { x, y } => (x * WHEEL_LINE, y * WHEEL_LINE),
                    mouse::ScrollDelta::Pixels { x, y } => (x, y),
                };
                let amount = if self.vertical || x == 0.0 { y } else { x };
                let before = state.offset;
                state.offset = (state.offset - amount).max(0.0);

                if (state.offset - before).abs() > f32::EPSILON || amount != 0.0 {
                    shell.invalidate_layout();
                    shell.request_redraw();
                    shell.capture_event();
                }
            }
            Event::Window(window::Event::RedrawRequested(now)) => {
                if let Some(marker) = &mut state.marker
                    && marker.from.is_some()
                {
                    let started = *marker.started.get_or_insert(*now);
                    marker.progress = now.saturating_duration_since(started).as_secs_f32()
                        / duration::LONG.as_secs_f32();

                    if marker.progress >= 1.0 {
                        marker.from = None;
                    } else {
                        shell.request_redraw();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if !state.focused || shell.is_event_captured() || !modifiers.is_empty() {
                    return;
                }

                let Some(current) = self.current(state) else {
                    return;
                };
                let keyboard::Key::Named(named) = key else {
                    return;
                };
                let (back, forward) = if self.vertical {
                    (key::Named::ArrowUp, key::Named::ArrowDown)
                } else {
                    (key::Named::ArrowLeft, key::Named::ArrowRight)
                };
                let count = self.items.len();

                let target = match *named {
                    key::Named::Enter | key::Named::Space => {
                        state.index = Some(current);
                        state.keyboard = true;
                        if let Some(on_select) = &self.on_select {
                            shell.publish(on_select(current));
                        }
                        shell.capture_event();
                        shell.request_redraw();
                        return;
                    }
                    named if named == back => step(current, count, false),
                    named if named == forward => step(current, count, true),
                    key::Named::Home => 0,
                    key::Named::End => count - 1,
                    _ => return,
                };

                if target != current {
                    state.index = Some(target);
                    state.reveal = Some(target);
                    shell.invalidate_layout();

                    if self.select_on_move
                        && let Some(on_select) = &self.on_select
                    {
                        shell.publish(on_select(target));
                    }
                }

                state.keyboard = true;
                shell.capture_event();
                shell.request_redraw();
            }
            _ => {}
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();
        let Some(visible) = bounds.intersection(viewport) else {
            return;
        };
        let enabled = self.enabled();
        let selected = self.selected();
        let hovered = self
            .item_at(layout, cursor)
            .and_then(|index| self.target(layout, index));
        let border = Border::default().rounded(self.look.radius);

        renderer.with_layer(visible, |renderer| {
            if let (Some(color), Some(marker)) =
                (self.look.marker, self.marker_bounds(state, layout))
            {
                let color = color(theme);

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: marker,
                        border,
                        ..renderer::Quad::default()
                    },
                    if enabled {
                        color
                    } else {
                        color.scale_alpha(DISABLED_CONTENT_OPACITY)
                    },
                );
            }

            if let Some(target) = hovered {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: target,
                        border,
                        ..renderer::Quad::default()
                    },
                    (self.look.hover)(theme),
                );
            }

            for (index, ((child, child_state), child_layout)) in self
                .items
                .iter()
                .zip(&tree.children)
                .zip(layout.children())
                .enumerate()
            {
                if !child_layout.bounds().intersects(&visible) {
                    continue;
                }

                let color = (self.look.text)(theme, selected == Some(index));
                let text_color = if enabled {
                    color
                } else {
                    color.scale_alpha(DISABLED_CONTENT_OPACITY)
                };

                child.as_widget().draw(
                    child_state,
                    renderer,
                    theme,
                    &renderer::Style { text_color },
                    child_layout,
                    cursor,
                    &visible,
                );
            }
        });

        if state.focused
            && state.keyboard
            && let Some(ring) = self
                .current(state)
                .and_then(|index| self.target(layout, index))
                .filter(|ring| ring.intersects(&visible))
        {
            crate::focus::draw_ring(renderer, theme, ring, self.look.radius);
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
        if self.item_at(layout, cursor).is_some() {
            return mouse::Interaction::Pointer;
        }

        self.items
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
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
        overlay::from_children(
            &mut self.items,
            tree,
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message> From<Strip<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(strip: Strip<'a, Message>) -> Self {
        Element::new(strip)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn revealing_moves_the_offset_as_little_as_possible() {
        assert_eq!(reveal_offset(0.0, 100.0, 20.0, 60.0), 0.0);
        assert_eq!(reveal_offset(0.0, 100.0, 120.0, 160.0), 60.0);
        assert_eq!(reveal_offset(80.0, 100.0, 20.0, 60.0), 20.0);
        assert_eq!(reveal_offset(0.0, 100.0, 50.0, 300.0), 50.0, "start wins");
    }

    #[test]
    fn steps_stop_at_the_ends() {
        assert_eq!(step(0, 3, false), 0);
        assert_eq!(step(0, 3, true), 1);
        assert_eq!(step(2, 3, true), 2);
        assert_eq!(step(2, 3, false), 1);
    }
}
