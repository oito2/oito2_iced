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

//! Toggle groups: a joined row (or column) of buttons that turn on and off,
//! such as text formatting or alignment buttons.
//!
//! With [`ToggleGroup::multiple`] each item turns on and off by itself;
//! otherwise turning one on turns the others off, and
//! [`ToggleGroup::deselectable`] says whether the one that is on can be
//! turned off. Items that are on are raised from the track.
//!
//! The group is one stop in keyboard navigation: the arrow keys of its
//! orientation move the focus between the enabled items, wrapping at the
//! ends, Home and End move it to the first and last enabled item, and Space
//! or Enter toggles the focused one. The selection lives in the
//! application: the group shows `selected` and publishes the new selection
//! through [`ToggleGroup::on_change`].
//!
//! ```no_run
//! use oito2_iced::widget::toggle_group;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Align(Vec<&'static str>),
//! }
//!
//! # fn view<'a>(align: &[&'static str]) -> oito2_iced::Element<'a, Message> {
//! toggle_group::toggle_group(
//!     [
//!         toggle_group::item("left", "Left"),
//!         toggle_group::item("center", "Center"),
//!         toggle_group::item("right", "Right"),
//!     ],
//!     align,
//! )
//! .deselectable(false)
//! .on_change(Message::Align)
//! .into()
//! # }
//! ```

use std::borrow::Cow;
use std::time::{Duration, Instant};

use iced::advanced::layout::{self, Layout, flex};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::button::{Status, Style};
use iced::widget::container;
use iced::widget::text::IntoFragment;
use iced::{
    Alignment, Background, Border, Color, Event, Length, Padding, Rectangle, Shadow, Size, Vector,
    mouse, window,
};

use super::min_size::{Minimum, min_size};
use super::{styled_text, tooltip};
use crate::animation::ease_out_cubic;
#[cfg(feature = "svg")]
use crate::icon::{Icon, icon};
use crate::theme::button::Class;
use crate::tokens::typography::{self, TextStyle};
use crate::tokens::{radius, spacing};
use crate::{Element, Theme};

/// The text style of group items: the medium body size in the medium
/// weight.
pub(crate) const ITEM_TEXT: TextStyle = TextStyle {
    weight: iced::font::Weight::Medium,
    ..typography::BODY_MEDIUM
};

/// The opacity of a disabled item.
pub(crate) const DISABLED_ITEM_OPACITY: f32 = 0.5;

/// A button of a [`ToggleGroup`]. Built by [`item`], [`icon_item`] and
/// [`custom_item`].
pub struct Item<'a, T, Message> {
    value: T,
    content: Element<'a, Message>,
    tooltip: Option<Cow<'a, str>>,
    enabled: bool,
}

/// An item for `value` showing `label`.
pub fn item<'a, T, Message: 'a>(value: T, label: impl IntoFragment<'a>) -> Item<'a, T, Message> {
    custom_item(value, styled_text(label, ITEM_TEXT))
}

/// An item for `value` showing `icon`; give it a [`Item::tooltip`], which
/// also names it.
#[cfg(feature = "svg")]
pub fn icon_item<'a, T, Message: 'a>(value: T, icon: Icon) -> Item<'a, T, Message> {
    custom_item(value, self::icon(icon))
}

/// An item for `value` showing any `content`, drawn in the item's text
/// color.
pub fn custom_item<'a, T, Message>(
    value: T,
    content: impl Into<Element<'a, Message>>,
) -> Item<'a, T, Message> {
    Item {
        value,
        content: content.into(),
        tooltip: None,
        enabled: true,
    }
}

impl<'a, T, Message> Item<'a, T, Message> {
    /// Shows `label` below the item while the pointer rests on it.
    #[must_use]
    pub fn tooltip(mut self, label: impl IntoFragment<'a>) -> Self {
        self.tooltip = Some(label.into_fragment());
        self
    }

    /// Sets whether the item can be toggled and focused (true by default);
    /// a disabled item is dimmed.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// A joined row of buttons that turn on and off. See [`toggle_group`].
pub struct ToggleGroup<'a, T, Message> {
    items: Vec<Item<'a, T, Message>>,
    selected: Vec<T>,
    on_change: Option<Box<dyn Fn(Vec<T>) -> Message + 'a>>,
    multiple: bool,
    deselectable: bool,
    vertical: bool,
}

/// A toggle group of `items` with the items of the `selected` values on.
/// Without [`ToggleGroup::on_change`] the group is disabled.
pub fn toggle_group<'a, T, Message>(
    items: impl IntoIterator<Item = Item<'a, T, Message>>,
    selected: &[T],
) -> ToggleGroup<'a, T, Message>
where
    T: Clone,
{
    ToggleGroup {
        items: items.into_iter().collect(),
        selected: selected.to_vec(),
        on_change: None,
        multiple: false,
        deselectable: true,
        vertical: false,
    }
}

impl<'a, T, Message> ToggleGroup<'a, T, Message> {
    /// Sets the function that turns the new selection, in item order, into
    /// a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Vec<T>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or disables the group with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(Vec<T>) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(Vec<T>) -> Message + 'a>);
        self
    }

    /// Sets whether several items can be on at once (false by default).
    #[must_use]
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Sets whether the only item that is on can be turned off, leaving
    /// none on (true by default); ignored when [`ToggleGroup::multiple`].
    #[must_use]
    pub fn deselectable(mut self, deselectable: bool) -> Self {
        self.deselectable = deselectable;
        self
    }

    /// Runs the items down instead of across; they share the width of the
    /// widest one.
    #[must_use]
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}

/// The selection after the item of `value` is toggled while `selected` is
/// on, with the values kept in the order of `order`; `None` when nothing
/// changes.
pub(crate) fn toggled<T: Clone + PartialEq>(
    order: &[T],
    selected: &[T],
    value: &T,
    multiple: bool,
    deselectable: bool,
) -> Option<Vec<T>> {
    let on = selected.contains(value);

    if multiple {
        let next = order
            .iter()
            .filter(|candidate| {
                if *candidate == value {
                    !on
                } else {
                    selected.contains(candidate)
                }
            })
            .cloned()
            .collect();

        Some(next)
    } else if on {
        deselectable.then(Vec::new)
    } else {
        Some(vec![value.clone()])
    }
}

/// The style of a toggle item: transparent when off and tinted on hover,
/// raised on the surface when on; dimmed when disabled.
fn item_style(theme: &Theme, status: Status, on: bool, enabled: bool) -> Style {
    let roles = theme.roles();
    let background = if on {
        Some(roles.surface)
    } else if enabled && matches!(status, Status::Hovered | Status::Pressed) {
        Some(roles.surface_container_highest)
    } else {
        None
    };
    let opacity = if enabled { 1.0 } else { DISABLED_ITEM_OPACITY };

    Style {
        background: background.map(|color| Background::Color(color.scale_alpha(opacity))),
        text_color: roles.on_surface.scale_alpha(opacity),
        border: Border::default().rounded(radius::CHECK),
        shadow: if on {
            raised_shadow(roles.shadow.scale_alpha(opacity))
        } else {
            Shadow::default()
        },
        snap: true,
    }
}

/// The faint shadow under a raised item or marker.
pub(crate) fn raised_shadow(shadow: Color) -> Shadow {
    Shadow {
        color: shadow.scale_alpha(0.12),
        offset: Vector::new(0.0, 1.0),
        blur_radius: 2.0,
    }
}

/// The style of a recessed group track with `radius` corners.
pub(crate) fn track_style(theme: &Theme, radius: f32) -> container::Style {
    let roles = theme.roles();

    container::Style {
        background: Some(Background::Color(roles.surface_container_high)),
        border: Border::default()
            .color(roles.outline_variant)
            .width(1.0)
            .rounded(radius),
        ..container::Style::default()
    }
}

impl<'a, T, Message> From<ToggleGroup<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(group: ToggleGroup<'a, T, Message>) -> Self {
        let order: Vec<T> = group.items.iter().map(|item| item.value.clone()).collect();
        let interactive = group.on_change.is_some();
        let vertical = group.vertical;

        let mut children = Vec::with_capacity(group.items.len());
        let mut enabled = Vec::with_capacity(group.items.len());
        let mut messages = Vec::with_capacity(group.items.len());

        for item in group.items {
            let on = group.selected.contains(&item.value);
            let can_toggle = interactive && item.enabled;
            let message = if can_toggle {
                group.on_change.as_ref().and_then(|on_change| {
                    toggled(
                        &order,
                        &group.selected,
                        &item.value,
                        group.multiple,
                        group.deselectable,
                    )
                    .map(on_change)
                })
            } else {
                None
            };

            let content = min_size(
                item.content,
                Minimum::Control(2.0 * spacing::SM),
                Minimum::Control(0.0),
            );
            let content: Element<'a, Message> = if vertical {
                container(content).center_x(Length::Fill).into()
            } else {
                content.into()
            };
            let enabled_look = item.enabled;
            let button = iced::widget::Button::new(content)
                .padding(Padding::from([0.0, spacing::SM]))
                .width(if vertical {
                    Length::Fill
                } else {
                    Length::Shrink
                })
                .on_press_maybe(message.clone())
                .class(Class::Custom(Box::new(move |theme, status| {
                    item_style(theme, status, on, enabled_look)
                })));

            let child: Element<'a, Message> = match item.tooltip {
                Some(label) => tooltip(button, label).into(),
                None => button.into(),
            };

            children.push(child);
            enabled.push(can_toggle);
            messages.push(message);
        }

        let roving = Roving {
            items: children,
            enabled,
            messages,
            vertical,
            spacing: 1.0,
            stop: None,
            select_on_move: false,
            marker: None,
            ring_radius: radius::CHECK,
        };

        container(roving)
            .padding(spacing::XXS + 1.0)
            .style(|theme| track_style(theme, radius::SMALL))
            .into()
    }
}

/// How long the selection marker takes to slide to a new item.
const MARKER_DURATION: Duration = Duration::from_millis(150);

/// A row or column of items that is one stop in keyboard navigation, with
/// the focus moving between its items. Shared by the toggle and segment
/// groups.
pub(crate) struct Roving<'a, Message> {
    /// The items, in order.
    pub(crate) items: Vec<Element<'a, Message>>,
    /// Whether each item can be focused and activated.
    pub(crate) enabled: Vec<bool>,
    /// The message each item publishes on Space or Enter.
    pub(crate) messages: Vec<Option<Message>>,
    /// Whether the items run down instead of across.
    pub(crate) vertical: bool,
    /// The space between items.
    pub(crate) spacing: f32,
    /// The item that takes the focus when the group gains it; when set, the
    /// focus always rests on it.
    pub(crate) stop: Option<usize>,
    /// Whether moving the focus with the keyboard also publishes the
    /// message of the item it lands on.
    pub(crate) select_on_move: bool,
    /// The item a raised marker sits behind, sliding when it changes.
    pub(crate) marker: Option<usize>,
    /// The corner radius of the focus ring and of the marker.
    pub(crate) ring_radius: f32,
}

/// The sliding marker of a [`Roving`] group.
#[derive(Debug, Clone, Copy)]
struct Marker {
    index: usize,
    /// The rectangle the marker slides from, relative to the group.
    from: Option<Rectangle>,
    started: Option<Instant>,
    progress: f32,
}

/// The focus and marker state of a [`Roving`] group.
#[derive(Debug, Clone, Copy, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the ring.
    keyboard: bool,
    index: Option<usize>,
    marker: Option<Marker>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// The first enabled item after `from`, going forward or back and wrapping
/// at the ends; `None` when no other item is enabled.
pub(crate) fn next_enabled(enabled: &[bool], from: usize, forward: bool) -> Option<usize> {
    let count = enabled.len();

    (1..count)
        .map(|offset| {
            if forward {
                (from + offset) % count
            } else {
                (from + count - offset) % count
            }
        })
        .find(|&index| enabled[index])
}

impl<Message> Roving<'_, Message> {
    fn is_enabled(&self, index: usize) -> bool {
        self.enabled.get(index).copied().unwrap_or(false)
    }

    fn first_enabled(&self) -> Option<usize> {
        self.enabled.iter().position(|enabled| *enabled)
    }

    fn last_enabled(&self) -> Option<usize> {
        self.enabled.iter().rposition(|enabled| *enabled)
    }

    /// The item that has the focus while the group has it.
    fn current(&self, state: &State) -> Option<usize> {
        let preferred = match self.stop {
            Some(stop) => Some(stop),
            None => state.index,
        };

        preferred
            .filter(|&index| self.is_enabled(index))
            .or_else(|| self.first_enabled())
    }

    /// The item under the cursor, if enabled.
    fn item_at(&self, layout: Layout<'_>, cursor: mouse::Cursor) -> Option<usize> {
        layout
            .children()
            .position(|child| cursor.is_over(child.bounds()))
            .filter(|&index| self.is_enabled(index))
    }

    /// The bounds of the marker, if any.
    fn marker_bounds(&self, state: &State, layout: Layout<'_>) -> Option<Rectangle> {
        marker_rect(self.marker?, state.marker, layout)
    }

    /// Moves the focus to `index` from the keyboard, publishing its message
    /// when the group selects on move.
    fn move_to(&self, state: &mut State, index: usize, shell: &mut Shell<'_, Message>)
    where
        Message: Clone,
    {
        state.index = Some(index);
        state.keyboard = true;

        if self.select_on_move
            && let Some(Some(message)) = self.messages.get(index)
        {
            shell.publish(message.clone());
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Roving<'_, Message>
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
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        if !self.vertical {
            return flex::resolve(
                flex::Axis::Horizontal,
                renderer,
                limits,
                Length::Shrink,
                Length::Shrink,
                Padding::ZERO,
                self.spacing,
                Alignment::Center,
                &mut self.items,
                &mut tree.children,
            );
        }

        // A column measures its items at their natural width first, then
        // lays them all out at the widest one.
        let max = limits.max();
        let natural = layout::Limits::with_compression(Size::ZERO, max, Size::new(true, true));
        let width = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .map(|(item, tree)| {
                item.as_widget_mut()
                    .layout(tree, renderer, &natural)
                    .size()
                    .width
            })
            .fold(0.0, f32::max);

        let shared = layout::Limits::new(Size::ZERO, Size::new(width, max.height));
        let mut y = 0.0;
        let nodes = self
            .items
            .iter_mut()
            .zip(&mut tree.children)
            .enumerate()
            .map(|(index, (item, tree))| {
                if index > 0 {
                    y += self.spacing;
                }

                let node = item
                    .as_widget_mut()
                    .layout(tree, renderer, &shared)
                    .move_to(iced::Point::new(0.0, y));
                y += node.size().height;
                node
            })
            .collect();

        layout::Node::with_children(Size::new(width, y), nodes)
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
        let pressed_item = match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(iced::touch::Event::FingerPressed { .. }) => {
                Some(self.item_at(layout, cursor))
            }
            _ => None,
        };

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
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        self.sync_marker(tree.state.downcast_mut::<State>(), layout, shell);

        let state = tree.state.downcast_mut::<State>();

        if let Some(item) = pressed_item {
            match item {
                Some(index) => {
                    state.index = Some(index);
                    state.focused = true;
                    state.keyboard = false;
                    shell.request_redraw();
                }
                None if state.focused && !cursor.is_over(layout.bounds()) => {
                    state.focused = false;
                    shell.request_redraw();
                }
                None => {}
            }
            return;
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            if let Some(marker) = &mut state.marker
                && marker.from.is_some()
            {
                let started = *marker.started.get_or_insert(*now);
                marker.progress = now.saturating_duration_since(started).as_secs_f32()
                    / MARKER_DURATION.as_secs_f32();

                if marker.progress >= 1.0 {
                    marker.from = None;
                } else {
                    shell.request_redraw();
                }
            }
            return;
        }

        let Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) = event else {
            return;
        };

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

        let target = match *named {
            key::Named::Enter | key::Named::Space => {
                state.index = Some(current);
                state.keyboard = true;
                if let Some(Some(message)) = self.messages.get(current) {
                    shell.publish(message.clone());
                }
                shell.capture_event();
                shell.request_redraw();
                return;
            }
            named if named == back => next_enabled(&self.enabled, current, false),
            named if named == forward => next_enabled(&self.enabled, current, true),
            key::Named::Home => self.first_enabled(),
            key::Named::End => self.last_enabled(),
            _ => return,
        };

        if let Some(target) = target.filter(|&target| target != current) {
            self.move_to(state, target, shell);
        }

        state.keyboard = true;
        shell.capture_event();
        shell.request_redraw();
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

        let state = tree.state.downcast_ref::<State>();

        if let Some(bounds) = self.marker_bounds(state, layout) {
            let roles = theme.roles();

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border::default().rounded(self.ring_radius),
                    shadow: raised_shadow(roles.shadow),
                    ..renderer::Quad::default()
                },
                roles.surface,
            );
        }

        for ((child, state), child_layout) in
            self.items.iter().zip(&tree.children).zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                child_layout,
                cursor,
                viewport,
            );
        }

        if state.focused
            && state.keyboard
            && let Some(item) = self
                .current(state)
                .and_then(|index| layout.children().nth(index))
        {
            crate::focus::draw_ring(renderer, theme, item.bounds(), self.ring_radius);
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

impl<Message> Roving<'_, Message> {
    /// Starts sliding the marker when the marked item changed since the
    /// last event; the first marker shows at once.
    fn sync_marker(&self, state: &mut State, layout: Layout<'_>, shell: &mut Shell<'_, Message>) {
        let Some(target) = self.marker else {
            state.marker = None;
            return;
        };

        match state.marker {
            Some(marker) if marker.index == target => {}
            Some(marker) => {
                let origin = layout.position();
                let from =
                    marker_rect(marker.index, Some(marker), layout).map(|bounds| Rectangle {
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
            None => {
                state.marker = Some(Marker {
                    index: target,
                    from: None,
                    started: None,
                    progress: 1.0,
                });
            }
        }
    }
}

impl<'a, Message> From<Roving<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(roving: Roving<'a, Message>) -> Self {
        Element::new(roving)
    }
}

/// The bounds of a marker behind the item at `index`: the item itself, or
/// a point between the previous and that item while `marker` slides.
fn marker_rect(index: usize, marker: Option<Marker>, layout: Layout<'_>) -> Option<Rectangle> {
    let target = layout.children().nth(index)?.bounds();

    let Some(Marker {
        from: Some(from),
        progress,
        ..
    }) = marker.filter(|marker| marker.index == index)
    else {
        return Some(target);
    };

    let origin = layout.position();
    let t = ease_out_cubic(progress.clamp(0.0, 1.0));
    let lerp = |a: f32, b: f32| a + (b - a) * t;

    Some(Rectangle {
        x: lerp(origin.x + from.x, target.x),
        y: lerp(origin.y + from.y, target.y),
        width: lerp(from.width, target.width),
        height: lerp(from.height, target.height),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const ORDER: [&str; 3] = ["b", "i", "u"];

    #[test]
    fn single_selection_replaces_and_clears() {
        assert_eq!(toggled(&ORDER, &[], &"b", false, true), Some(vec!["b"]));
        assert_eq!(toggled(&ORDER, &["b"], &"i", false, true), Some(vec!["i"]));
        assert_eq!(toggled(&ORDER, &["i"], &"i", false, true), Some(vec![]));
        assert_eq!(toggled(&ORDER, &["i"], &"i", false, false), None);
    }

    #[test]
    fn multiple_selection_toggles_in_item_order() {
        assert_eq!(
            toggled(&ORDER, &["u"], &"b", true, false),
            Some(vec!["b", "u"])
        );
        assert_eq!(
            toggled(&ORDER, &["b", "u"], &"b", true, false),
            Some(vec!["u"])
        );
    }

    #[test]
    fn the_next_enabled_item_wraps_and_skips_disabled_ones() {
        let enabled = [true, true, false, true];

        assert_eq!(next_enabled(&enabled, 1, true), Some(3));
        assert_eq!(next_enabled(&enabled, 3, true), Some(0));
        assert_eq!(next_enabled(&enabled, 0, false), Some(3));
        assert_eq!(next_enabled(&enabled, 3, false), Some(1));
        assert_eq!(next_enabled(&[true], 0, true), None);
    }
}
