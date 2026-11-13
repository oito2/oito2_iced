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

//! Popup menus, menu items and selects.
//!
//! A menu is a list of [`Item`]s on the menu surface, anchored below an
//! element: plain items ([`item`]), items with a check mark for single
//! choices ([`checked_item`]), items with a check box for multiple choices
//! ([`multi_select_item`]), group headers ([`header`]) and separators
//! ([`separator`]). [`anchor`] opens a menu below any element,
//! [`popup_menu_button`] below a button that looks like a field, and
//! [`select`] lists the values of a [`ListCollection`] with a check mark on
//! the selected one.
//!
//! While a menu is open, the Up and Down arrows move the highlight between
//! the enabled items and stop at the ends, Home and End jump to the first
//! and the last one, and Enter or Space activates the highlighted item;
//! moving the pointer over an item highlights it. Activating a plain or
//! checked item publishes its message and closes the menu; a multi-select
//! item keeps it open. Escape, a click outside and Tab close the menu.
//!
//! The open state lives in the application: the menu shows while it is
//! rendered open and publishes the close message to be rendered closed.
//!
//! ```no_run
//! use oito2_iced::widget::menu;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     MenuOpen(bool),
//!     Sort(u8),
//! }
//!
//! # fn view<'a>(open: bool, sort: u8) -> oito2_iced::Element<'a, Message> {
//! menu::popup_menu_button(
//!     "Sort",
//!     [
//!         menu::checked_item(sort == 0, "Name").on_press(Message::Sort(0)),
//!         menu::checked_item(sort == 1, "Date").on_press(Message::Sort(1)),
//!     ],
//! )
//! .open(open)
//! .on_open_change(Message::MenuOpen)
//! .into()
//! # }
//! ```

use std::cell::Cell;
use std::rc::Rc;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Id, Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::font::Weight;
use iced::keyboard::{self, key};
use iced::widget::text::{self, Fragment, IntoFragment};
use iced::widget::{Column, Row, Space, button, scrollable};
use iced::{
    Alignment, Background, Border, Color, Element as IcedElement, Event, Length, Padding, Point,
    Rectangle, Size, Vector, mouse,
};

use super::min_size::{Minimum, min_size};
use super::{focusable, styled_text, toggles, tooltip};
use crate::collections::ListCollection;
use crate::density::{self, Density};
#[cfg(feature = "svg")]
use crate::icon::{Icon, icon};
use crate::overlay::anchored::{self, Anchored};
use crate::theme::{
    DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY, TRANSPARENT, state_layer,
};
use crate::tokens::typography::{self, TextStyle};
use crate::tokens::{radius, spacing};
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// Minimum width of a menu item and of a popup menu button.
pub const MIN_WIDTH: f32 = 64.0;

/// State layer opacity of the item highlighted from the keyboard.
const FOCUSED_OPACITY: f32 = 0.10;

/// The label style of a popup menu button: the label style of buttons at
/// the regular weight.
const BUTTON_LABEL: TextStyle = TextStyle {
    weight: Weight::Normal,
    ..typography::LABEL_LARGE
};

/// Space between the leading mark of an item and its label: 10 pixels for
/// a pointer and 12 for touch.
fn label_gap() -> f32 {
    match density::get() {
        Density::Pointer => spacing::MD - 2.0,
        Density::Touch => spacing::MD,
    }
}

/// What an [`Item`] shows before its label.
enum Leading {
    /// Nothing.
    None,
    /// A check mark, or the room of one when unchecked.
    Check(bool),
    /// A check box.
    Checkbox,
    /// An icon.
    #[cfg(feature = "svg")]
    Icon(Icon),
}

/// The kind of an [`Item`].
enum Kind<'a> {
    /// An item that can be activated.
    Action {
        label: Fragment<'a>,
        leading: Leading,
        closes: bool,
    },
    /// A group name.
    Header(Fragment<'a>),
    /// A line between groups.
    Separator,
}

/// An entry of a menu. Built by [`item`], [`checked_item`],
/// [`multi_select_item`], [`header`] and [`separator`].
pub struct Item<'a, Message> {
    kind: Kind<'a>,
    checked: bool,
    on_press: Option<Message>,
}

/// A plain item with `label`; it closes the menu when activated.
pub fn item<'a, Message>(label: impl IntoFragment<'a>) -> Item<'a, Message> {
    Item {
        kind: Kind::Action {
            label: label.into_fragment(),
            leading: Leading::None,
            closes: true,
        },
        checked: false,
        on_press: None,
    }
}

/// An item with `label` that shows a check mark when `checked`, for single
/// choices; it closes the menu when activated.
pub fn checked_item<'a, Message>(checked: bool, label: impl IntoFragment<'a>) -> Item<'a, Message> {
    Item {
        kind: Kind::Action {
            label: label.into_fragment(),
            leading: Leading::Check(checked),
            closes: true,
        },
        checked,
        on_press: None,
    }
}

/// An item with `label` and a check box showing `checked`, for multiple
/// choices; the menu stays open when it is activated, so several items can
/// be toggled in a row.
pub fn multi_select_item<'a, Message>(
    checked: bool,
    label: impl IntoFragment<'a>,
) -> Item<'a, Message> {
    Item {
        kind: Kind::Action {
            label: label.into_fragment(),
            leading: Leading::Checkbox,
            closes: false,
        },
        checked,
        on_press: None,
    }
}

/// A group name in the small label style and the variant text color; it
/// cannot be activated.
pub fn header<'a, Message>(label: impl IntoFragment<'a>) -> Item<'a, Message> {
    Item {
        kind: Kind::Header(label.into_fragment()),
        checked: false,
        on_press: None,
    }
}

/// A one-pixel line in the divider color between groups of items.
pub fn separator<'a, Message>() -> Item<'a, Message> {
    Item {
        kind: Kind::Separator,
        checked: false,
        on_press: None,
    }
}

impl<'a, Message> Item<'a, Message> {
    /// Sets the message published when the item is activated. Without it
    /// the item is disabled: dimmed and skipped by the arrow keys.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the activation message, or disables the item with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets the activation message from the checked state the item takes
    /// when activated: the opposite of its current one.
    #[must_use]
    pub fn on_change(mut self, on_change: impl FnOnce(bool) -> Message) -> Self {
        self.on_press = Some(on_change(!self.checked));
        self
    }

    /// Shows `icon` before the label of a plain item, in the variant text
    /// color.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        if let Kind::Action { leading, .. } = &mut self.kind {
            *leading = Leading::Icon(icon);
        }
        self
    }
}

/// A vector mark drawn with the strokes of the Lucide icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Shape {
    /// A check mark.
    Check,
    /// A chevron pointing down.
    ChevronDown,
    /// A chevron pointing up.
    ChevronUp,
    /// Nothing: the room of a mark.
    Blank,
}

/// A mark as large as the icon size of the density, drawn in the text color
/// of its parent or in the variant text color.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Mark {
    shape: Shape,
    variant: bool,
}

impl Mark {
    /// A mark of `shape` in the text color of its parent.
    pub(crate) fn new(shape: Shape) -> Self {
        Self {
            shape,
            variant: false,
        }
    }

    /// Draws the mark in the variant text color of the theme.
    pub(crate) fn variant(mut self) -> Self {
        self.variant = true;
        self
    }
}

impl<Message, Renderer> Widget<Message, Theme, Renderer> for Mark
where
    Renderer: geometry::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = density::icon_size();

        layout::atomic(limits, side, side)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let points: &[(f32, f32)] = match self.shape {
            Shape::Check => &[(20.0, 6.0), (9.0, 17.0), (4.0, 12.0)],
            Shape::ChevronDown => &[(6.0, 9.0), (12.0, 15.0), (18.0, 9.0)],
            Shape::ChevronUp => &[(6.0, 15.0), (12.0, 9.0), (18.0, 15.0)],
            Shape::Blank => return,
        };

        let color = if self.variant {
            theme.roles().on_surface_variant
        } else {
            style.text_color
        };

        let bounds = layout.bounds();
        let unit = bounds.width / 24.0;
        let mut frame = Frame::new(renderer, bounds.size());
        let path = Path::new(|builder| {
            for (index, &(x, y)) in points.iter().enumerate() {
                let point = Point::new(x * unit, y * unit);

                if index == 0 {
                    builder.move_to(point);
                } else {
                    builder.line_to(point);
                }
            }
        });

        frame.stroke(
            &path,
            Stroke::default()
                .with_color(color)
                .with_width(2.0 * unit)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round),
        );

        let geometry = frame.into_geometry();
        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message, Renderer> From<Mark> for IcedElement<'a, Message, Theme, Renderer>
where
    Renderer: geometry::Renderer + 'a,
{
    fn from(mark: Mark) -> Self {
        IcedElement::new(mark)
    }
}

/// An entry of a [`List`], pointing at its elements.
enum Entry<Message> {
    /// An item that can be activated, enabled while it has a message.
    Action {
        leading: Option<usize>,
        label: usize,
        on_press: Option<Message>,
        closes: bool,
    },
    /// A group name.
    Header { label: usize },
    /// A line between groups.
    Separator,
}

impl<Message> Entry<Message> {
    fn is_enabled(&self) -> bool {
        matches!(
            self,
            Entry::Action {
                on_press: Some(_),
                ..
            }
        )
    }
}

/// The first enabled index of `enabled`.
fn first(enabled: &[bool]) -> Option<usize> {
    enabled.iter().position(|&enabled| enabled)
}

/// The last enabled index of `enabled`.
fn last(enabled: &[bool]) -> Option<usize> {
    enabled.iter().rposition(|&enabled| enabled)
}

/// The enabled index after `from` (or before it when not `forward`); the
/// first (or last) one when nothing is highlighted, and `from` itself at
/// the end of the list.
fn step(enabled: &[bool], from: Option<usize>, forward: bool) -> Option<usize> {
    let Some(from) = from else {
        return if forward {
            first(enabled)
        } else {
            last(enabled)
        };
    };

    let next = if forward {
        enabled
            .iter()
            .enumerate()
            .skip(from + 1)
            .find(|(_, enabled)| **enabled)
            .map(|(index, _)| index)
    } else {
        enabled
            .iter()
            .enumerate()
            .take(from)
            .rev()
            .find(|(_, enabled)| **enabled)
            .map(|(index, _)| index)
    };

    next.or(Some(from))
}

/// The tree state of a [`List`].
#[derive(Debug, Default)]
struct State {
    highlight: Option<usize>,
    hovered: Option<usize>,
    pressed: Option<usize>,
    was_open: bool,
    rows: Vec<Rectangle>,
}

impl State {
    /// Clears the highlight while closed, and sets it when the list opens:
    /// on the first enabled item when `focus_first`, on nothing otherwise.
    fn sync(&mut self, open: bool, focus_first: bool, enabled: &[bool]) {
        if !open {
            self.highlight = None;
            self.hovered = None;
            self.pressed = None;
            self.was_open = false;
        } else if !self.was_open {
            self.was_open = true;
            self.highlight = if focus_first { first(enabled) } else { None };
        } else if self
            .highlight
            .is_some_and(|index| !enabled.get(index).copied().unwrap_or(false))
        {
            self.highlight = None;
        }
    }
}

/// The items of an open menu. See the [module documentation](self).
pub(crate) struct List<'a, Message> {
    elements: Vec<Element<'a, Message>>,
    entries: Vec<Entry<Message>>,
    open: bool,
    on_close: Option<Message>,
    direction: ReadingDirection,
    focus_first: Option<Rc<Cell<bool>>>,
}

impl<'a, Message: Clone + 'a> List<'a, Message> {
    /// The list of `items`, shown while `open`, publishing `on_close` after
    /// an item that closes the menu and on Tab.
    pub(crate) fn new(
        items: impl IntoIterator<Item = Item<'a, Message>>,
        open: bool,
        on_close: Option<Message>,
        direction: ReadingDirection,
    ) -> Self {
        let mut elements: Vec<Element<'a, Message>> = Vec::new();
        let mut entries = Vec::new();

        for item in items {
            let Item {
                kind,
                checked,
                on_press,
            } = item;

            match kind {
                Kind::Action {
                    label,
                    leading,
                    closes,
                } => {
                    let leading: Option<Element<'a, Message>> = match leading {
                        Leading::None => None,
                        Leading::Check(checked) => Some(
                            Mark::new(if checked { Shape::Check } else { Shape::Blank }).into(),
                        ),
                        Leading::Checkbox => {
                            let message = on_press.clone();
                            let checkbox = match message {
                                Some(message) => {
                                    toggles::checkbox(checked).on_change(move |_| message.clone())
                                }
                                None => toggles::checkbox(checked),
                            };

                            Some(checkbox.into_toggle().passive().into())
                        }
                        #[cfg(feature = "svg")]
                        Leading::Icon(glyph) => Some(icon(glyph).into()),
                    };

                    let leading = leading.map(|element| {
                        elements.push(element);
                        elements.len() - 1
                    });

                    elements.push(styled_text(label, typography::BODY_MEDIUM).into());

                    entries.push(Entry::Action {
                        leading,
                        label: elements.len() - 1,
                        on_press,
                        closes,
                    });
                }
                Kind::Header(label) => {
                    elements.push(styled_text(label, typography::LABEL_SMALL).into());
                    entries.push(Entry::Header {
                        label: elements.len() - 1,
                    });
                }
                Kind::Separator => entries.push(Entry::Separator),
            }
        }

        Self {
            elements,
            entries,
            open,
            on_close,
            direction,
            focus_first: None,
        }
    }

    /// Highlights the first enabled item when the list opens while the
    /// shared flag is set.
    pub(crate) fn focus_first(mut self, flag: Rc<Cell<bool>>) -> Self {
        self.focus_first = Some(flag);
        self
    }

    /// The list in a vertical scrollable, for menus taller than the window.
    pub(crate) fn into_panel(self) -> Element<'a, Message> {
        scrollable(Element::new(self)).into()
    }
}

impl<Message> List<'_, Message> {
    fn enabled(&self) -> Vec<bool> {
        self.entries.iter().map(Entry::is_enabled).collect()
    }

    fn focus_first_now(&self) -> bool {
        self.focus_first.as_ref().is_some_and(|flag| flag.get())
    }

    /// The index of the enabled entry under `cursor`.
    fn entry_at(&self, state: &State, bounds: Rectangle, cursor: mouse::Cursor) -> Option<usize> {
        let position = cursor.position()?;
        let offset = Vector::new(bounds.x, bounds.y);

        state
            .rows
            .iter()
            .position(|row| (*row + offset).contains(position))
            .filter(|&index| self.entries.get(index).is_some_and(Entry::is_enabled))
    }

    /// Publishes the message of the entry at `index`, then the close message
    /// when the entry closes the menu.
    fn activate(&self, index: usize, shell: &mut Shell<'_, Message>)
    where
        Message: Clone,
    {
        if let Some(Entry::Action {
            on_press: Some(message),
            closes,
            ..
        }) = self.entries.get(index)
        {
            shell.publish(message.clone());

            if *closes && let Some(close) = &self.on_close {
                shell.publish(close.clone());
            }
        }
    }
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for List<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let mut state = State::default();
        state.sync(self.open, self.focus_first_now(), &self.enabled());

        tree::State::new(state)
    }

    fn children(&self) -> Vec<Tree> {
        self.elements.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.elements);

        let enabled = self.enabled();
        let state = tree.state.downcast_mut::<State>();
        state.sync(self.open, self.focus_first_now(), &enabled);
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
        let padding = spacing::MD;
        let gap = label_gap();
        let control_height = density::control_height();
        let max_width = limits.max().width;
        let content_limits = layout::Limits::new(
            Size::ZERO,
            Size::new((max_width - 2.0 * padding).max(0.0), f32::INFINITY),
        );

        let mut nodes: Vec<layout::Node> = Vec::with_capacity(self.elements.len());
        let mut heights = Vec::with_capacity(self.entries.len());
        let mut content_width: f32 = 0.0;

        for entry in &self.entries {
            match entry {
                Entry::Action { leading, label, .. } => {
                    let leading_size = match leading {
                        Some(index) => {
                            let node = self.elements[*index].as_widget_mut().layout(
                                &mut tree.children[*index],
                                renderer,
                                &content_limits,
                            );
                            let size = node.size();
                            nodes.push(node);
                            Some(size)
                        }
                        None => None,
                    };
                    let reserved = leading_size.map_or(0.0, |size| size.width + gap);
                    let node = self.elements[*label].as_widget_mut().layout(
                        &mut tree.children[*label],
                        renderer,
                        &content_limits.shrink(Size::new(reserved, 0.0)),
                    );
                    let label_size = node.size();
                    nodes.push(node);

                    content_width = content_width.max(reserved + label_size.width);
                    heights.push(
                        control_height
                            .max(label_size.height)
                            .max(leading_size.map_or(0.0, |size| size.height)),
                    );
                }
                Entry::Header { label } => {
                    let node = self.elements[*label].as_widget_mut().layout(
                        &mut tree.children[*label],
                        renderer,
                        &content_limits,
                    );
                    let size = node.size();
                    nodes.push(node);

                    content_width = content_width.max(size.width);
                    heights.push(size.height + spacing::SM + spacing::XXS);
                }
                Entry::Separator => heights.push(1.0),
            }
        }

        let width = (content_width + 2.0 * padding)
            .max(MIN_WIDTH)
            .max(limits.min().width)
            .min(max_width);
        let rtl = self.direction == ReadingDirection::RightToLeft;
        let place = |node: &mut layout::Node, x: f32, y: f32| {
            let x = if rtl {
                width - x - node.size().width
            } else {
                x
            };
            node.move_to_mut(Point::new(x, y));
        };

        let mut rows = Vec::with_capacity(self.entries.len());
        let mut nodes_iter = nodes.iter_mut();
        let mut y = 0.0;

        for (entry, height) in self.entries.iter().zip(heights) {
            match entry {
                Entry::Action { leading, .. } => {
                    let mut x = padding;

                    if leading.is_some()
                        && let Some(node) = nodes_iter.next()
                    {
                        let size = node.size();
                        place(node, x, y + (height - size.height) / 2.0);
                        x += size.width + gap;
                    }

                    if let Some(node) = nodes_iter.next() {
                        let size = node.size();
                        place(node, x, y + (height - size.height) / 2.0);
                    }
                }
                Entry::Header { .. } => {
                    if let Some(node) = nodes_iter.next() {
                        place(node, padding, y + spacing::SM);
                    }
                }
                Entry::Separator => {}
            }

            rows.push(Rectangle::new(Point::new(0.0, y), Size::new(width, height)));
            y += height;
        }

        tree.state.downcast_mut::<State>().rows = rows;

        layout::Node::with_children(Size::new(width, y), nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            for ((element, child), child_layout) in self
                .elements
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
            {
                element
                    .as_widget_mut()
                    .operate(child, child_layout, renderer, operation);
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
        for ((element, child), child_layout) in self
            .elements
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
        {
            element.as_widget_mut().update(
                child,
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let bounds = layout.bounds();
        let enabled = self.enabled();
        let state = tree.state.downcast_mut::<State>();
        let under = self.entry_at(state, bounds, cursor);

        match event {
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if under != state.hovered {
                    state.hovered = under;

                    if under.is_some() {
                        state.highlight = under;
                    }

                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(index) = under {
                    state.pressed = Some(index);
                    state.highlight = Some(index);
                    shell.capture_event();
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                if let Some(pressed) = state.pressed.take() {
                    shell.request_redraw();

                    if under == Some(pressed) {
                        self.activate(pressed, shell);
                        shell.capture_event();
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if !shell.is_event_captured() => {
                let target = match named {
                    key::Named::ArrowDown => Some(step(&enabled, state.highlight, true)),
                    key::Named::ArrowUp => Some(step(&enabled, state.highlight, false)),
                    key::Named::Home => Some(first(&enabled)),
                    key::Named::End => Some(last(&enabled)),
                    _ => None,
                };

                if let Some(target) = target {
                    if target.is_some() {
                        state.highlight = target;
                    }
                    shell.capture_event();
                    shell.request_redraw();
                    return;
                }

                match named {
                    key::Named::Enter | key::Named::Space if modifiers.is_empty() => {
                        if let Some(index) = state.highlight {
                            self.activate(index, shell);
                            shell.capture_event();
                        }
                    }
                    key::Named::Tab => {
                        if let Some(close) = &self.on_close {
                            shell.publish(close.clone());
                        }
                    }
                    _ => {}
                }
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
        let roles = theme.roles();
        let bounds = layout.bounds();
        let offset = Vector::new(bounds.x, bounds.y);
        let children: Vec<Layout<'_>> = layout.children().collect();
        let disabled = roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY);

        let draw_child = |renderer: &mut iced::Renderer, index: usize, color: Color| {
            if let (Some(element), Some(child), Some(child_layout)) = (
                self.elements.get(index),
                tree.children.get(index),
                children.get(index),
            ) {
                element.as_widget().draw(
                    child,
                    renderer,
                    theme,
                    &renderer::Style { text_color: color },
                    *child_layout,
                    cursor,
                    viewport,
                );
            }
        };

        for (index, (entry, row)) in self.entries.iter().zip(&state.rows).enumerate() {
            let row = *row + offset;

            match entry {
                Entry::Separator => {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds: Rectangle::new(row.position(), Size::new(row.width, 1.0)),
                            ..renderer::Quad::default()
                        },
                        Background::Color(roles.outline_variant),
                    );
                }
                Entry::Header { label } => draw_child(renderer, *label, roles.on_surface_variant),
                Entry::Action {
                    leading,
                    label,
                    on_press,
                    ..
                } => {
                    let enabled = on_press.is_some();

                    if enabled {
                        let hovered = cursor.is_over(row);
                        let opacity = if hovered && state.pressed == Some(index) {
                            PRESSED_OPACITY
                        } else if hovered {
                            HOVER_OPACITY
                        } else if state.highlight == Some(index) {
                            FOCUSED_OPACITY
                        } else {
                            0.0
                        };

                        if opacity > 0.0 {
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds: row,
                                    border: Border::default().rounded(radius::SMALL),
                                    ..renderer::Quad::default()
                                },
                                Background::Color(roles.on_surface.scale_alpha(opacity)),
                            );
                        }
                    }

                    let (text_color, icon_color) = if enabled {
                        (roles.on_surface, roles.on_surface_variant)
                    } else {
                        (disabled, disabled)
                    };

                    if let Some(leading) = leading {
                        draw_child(renderer, *leading, icon_color);
                    }

                    draw_child(renderer, *label, text_color);
                }
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();

        if self.entry_at(state, layout.bounds(), cursor).is_some() {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }
}

impl<'a, Message: Clone + 'a> From<List<'a, Message>> for Element<'a, Message> {
    fn from(list: List<'a, Message>) -> Self {
        Element::new(list)
    }
}

/// Anchors the menu `panel` to `anchor` with the menu surface: below it,
/// aligned with its start, with the vertical padding of menus.
pub(crate) fn anchored_menu<'a, Message: Clone + 'a>(
    anchor: impl Into<Element<'a, Message>>,
    panel: Element<'a, Message>,
    open: bool,
    on_close: Option<Message>,
    direction: ReadingDirection,
) -> Anchored<'a, Message> {
    let anchored = crate::overlay::anchored(anchor, panel)
        .menu()
        .open(open)
        .direction(direction)
        .placement(anchored::Placement::Bottom)
        .alignment(anchored::Alignment::Start)
        .gap(0.0)
        .padding(Padding::from([spacing::XS, 0.0]));

    match on_close {
        Some(message) => anchored.on_dismiss(message),
        None => anchored,
    }
}

/// An element with a menu of items below it. Built by [`anchor`].
pub struct MenuAnchor<'a, Message> {
    anchor: Element<'a, Message>,
    items: Vec<Item<'a, Message>>,
    open: bool,
    on_close: Option<Message>,
    direction: ReadingDirection,
}

/// Opens a menu of `items` below `anchor` while rendered open; the anchor
/// is usually a button that publishes the message that opens the menu.
pub fn anchor<'a, Message>(
    anchor: impl Into<Element<'a, Message>>,
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> MenuAnchor<'a, Message> {
    MenuAnchor {
        anchor: anchor.into(),
        items: items.into_iter().collect(),
        open: false,
        on_close: None,
        direction: ReadingDirection::LeftToRight,
    }
}

impl<Message> MenuAnchor<'_, Message> {
    /// Shows or hides the menu.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the message published when the menu closes: on Escape, a click
    /// outside, Tab, or after an item that closes it.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
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

impl<'a, Message: Clone + 'a> From<MenuAnchor<'a, Message>> for Element<'a, Message> {
    fn from(menu: MenuAnchor<'a, Message>) -> Self {
        let panel =
            List::new(menu.items, menu.open, menu.on_close.clone(), menu.direction).into_panel();

        anchored_menu(menu.anchor, panel, menu.open, menu.on_close, menu.direction).into()
    }
}

/// The style of a popup menu button: the high surface container with the
/// outline, as tall as a text field.
fn field_style(theme: &Theme, status: button::Status) -> button::Style {
    let roles = theme.roles();
    let background = roles.surface_container_high;

    let (background, text_color) = match status {
        button::Status::Active => (background, roles.on_surface),
        button::Status::Hovered => (
            state_layer(background, roles.on_surface, HOVER_OPACITY),
            roles.on_surface,
        ),
        button::Status::Pressed => (
            state_layer(background, roles.on_surface, PRESSED_OPACITY),
            roles.on_surface,
        ),
        button::Status::Disabled => (
            TRANSPARENT,
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
        ),
    };

    button::Style {
        background: (background != TRANSPARENT).then_some(Background::Color(background)),
        text_color,
        border: Border::default()
            .color(roles.outline)
            .width(1.0)
            .rounded(radius::MEDIUM),
        ..button::Style::default()
    }
}

/// A closure that turns the requested open state into a message.
type OnOpenChange<'a, Message> = Box<dyn Fn(bool) -> Message + 'a>;

/// A button that looks like a field and opens a menu. Built by
/// [`popup_menu_button`].
pub struct PopupMenuButton<'a, Message> {
    label: Element<'a, Message>,
    leading: Option<Element<'a, Message>>,
    items: Vec<Item<'a, Message>>,
    open: bool,
    on_open_change: Option<OnOpenChange<'a, Message>>,
    tooltip: Option<Fragment<'a>>,
    id: Option<Id>,
    width: Length,
    direction: ReadingDirection,
}

/// A button showing `label` and a chevron that opens a menu of `items`
/// below it, for choices that don't fit a segmented button.
///
/// The button is as tall as the text fields, on the high surface container
/// with the outline. A click, Enter, Space or the Down arrow while it has
/// the focus publishes the open request; the chevron points up while the
/// menu is open.
pub fn popup_menu_button<'a, Message: 'a>(
    label: impl IntoFragment<'a>,
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> PopupMenuButton<'a, Message> {
    PopupMenuButton {
        label: styled_text(label, BUTTON_LABEL).into(),
        leading: None,
        items: items.into_iter().collect(),
        open: false,
        on_open_change: None,
        tooltip: None,
        id: None,
        width: Length::Shrink,
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> PopupMenuButton<'a, Message> {
    /// Shows or hides the menu.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the closure that turns the requested open state into a message:
    /// `true` on a click, Enter, Space or Down while closed; `false` on a
    /// click while open, Escape, a click outside, Tab, or after an item
    /// that closes the menu. Without it the button is disabled.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Places `leading`, usually an icon, before the label.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Shows `label` below the button while the pointer rests on it.
    #[must_use]
    pub fn tooltip(mut self, label: impl IntoFragment<'a>) -> Self {
        self.tooltip = Some(label.into_fragment());
        self
    }

    /// Sets the widget [`Id`] of the button, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width of the button; the content stays at its start.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the reading direction, which orders the content and places the
    /// menu.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<PopupMenuButton<'a, Message>> for Element<'a, Message> {
    fn from(popup: PopupMenuButton<'a, Message>) -> Self {
        let PopupMenuButton {
            label,
            leading,
            items,
            open,
            on_open_change,
            tooltip: tooltip_label,
            id,
            width,
            direction,
        } = popup;

        let enabled = on_open_change.is_some();
        let open = open && enabled;
        let request = |value: bool| on_open_change.as_ref().map(|f| f(value));
        let toggle = request(!open);
        let close = request(false);
        let open_request = if open { None } else { request(true) };
        let rtl = direction == ReadingDirection::RightToLeft;

        let mut parts: Vec<Element<'a, Message>> = Vec::new();
        if let Some(leading) = leading {
            parts.push(leading);
        }
        parts.push(label);
        parts.push(
            Mark::new(if open {
                Shape::ChevronUp
            } else {
                Shape::ChevronDown
            })
            .variant()
            .into(),
        );
        if rtl {
            parts.reverse();
        }

        let content = Row::from_vec(parts)
            .spacing(spacing::SM)
            .align_y(Alignment::Center);
        let content = Column::new()
            .push(content)
            .push(Space::new().width(MIN_WIDTH - spacing::MD - spacing::SM))
            .align_x(if rtl {
                Alignment::End
            } else {
                Alignment::Start
            });
        let content = min_size(content, Minimum::None, Minimum::Control(0.0));

        let (start, end) = (spacing::MD, spacing::SM);
        let padding = if rtl {
            Padding::ZERO.left(end).right(start)
        } else {
            Padding::ZERO.left(start).right(end)
        };

        let native = button(content)
            .style(field_style)
            .padding(padding)
            .width(width)
            .on_press_maybe(toggle.clone());

        let mut wrapper = focusable(native).radius(radius::MEDIUM).enabled(enabled);
        if let Some(message) = toggle {
            wrapper = wrapper.on_activate(message);
        }
        if let Some(id) = id {
            wrapper = wrapper.id(id);
        }

        let opener = KeyOpener {
            content: wrapper.into(),
            message: open_request,
        };

        let anchor: Element<'a, Message> = match tooltip_label {
            Some(label) => tooltip(Element::new(opener), label).into(),
            None => Element::new(opener),
        };

        let panel = List::new(items, open, close.clone(), direction).into_panel();

        anchored_menu(anchor, panel, open, close, direction).into()
    }
}

/// Publishes `message` on the Down arrow while a widget inside `content`
/// has the focus.
struct KeyOpener<'a, Message> {
    content: Element<'a, Message>,
    message: Option<Message>,
}

impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for KeyOpener<'_, Message> {
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
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
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
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::ArrowDown),
            modifiers,
            ..
        }) = event
            && modifiers.is_empty()
            && let Some(message) = &self.message
            && crate::focus::run(
                &mut self.content,
                &mut tree.children[0],
                layout,
                renderer,
                crate::focus::operations::has_focus(),
            )
            .unwrap_or(false)
        {
            shell.publish(message.clone());
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
        self.content.as_widget().draw(
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
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
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
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

/// One choice of a [`Select`].
struct Choice {
    value: String,
    label: String,
    disabled: bool,
}

/// A popup menu button that picks one value of a collection. Built by
/// [`select`].
pub struct Select<'a, Message> {
    groups: Vec<(Option<String>, Vec<Choice>)>,
    value: Option<String>,
    selected_label: Option<String>,
    placeholder: Option<Fragment<'a>>,
    leading: Option<Element<'a, Message>>,
    on_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    open: bool,
    on_open_change: Option<OnOpenChange<'a, Message>>,
    id: Option<Id>,
    width: Length,
    direction: ReadingDirection,
}

/// A popup menu button that picks one value of `collection`: the button
/// shows the label of the item whose value is `value` (or the
/// placeholder), and the menu lists the items under their group names, with
/// a check mark on the selected one and disabled items dimmed.
pub fn select<'a, T: 'static, Message>(
    collection: &ListCollection<T>,
    value: Option<&str>,
) -> Select<'a, Message> {
    let groups = collection
        .groups()
        .into_iter()
        .map(|group| {
            let choices = group
                .items
                .into_iter()
                .map(|item| Choice {
                    value: collection.value_of(item),
                    label: collection.label_of(item),
                    disabled: collection.is_disabled(item),
                })
                .collect();

            (group.group, choices)
        })
        .collect();

    Select {
        groups,
        value: value.map(str::to_owned),
        selected_label: value
            .and_then(|value| collection.find(value))
            .map(|item| collection.label_of(item)),
        placeholder: None,
        leading: None,
        on_change: None,
        open: false,
        on_open_change: None,
        id: None,
        width: Length::Shrink,
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> Select<'a, Message> {
    /// Sets the closure that turns the chosen value into a message. Without
    /// it the select is disabled.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Shows or hides the menu.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the closure that turns the requested open state into a message,
    /// as [`PopupMenuButton::on_open_change`]. Without it the select is
    /// disabled.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Sets the text shown in the variant text color while nothing is
    /// selected.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = Some(placeholder.into_fragment());
        self
    }

    /// Places `leading`, usually an icon, before the label.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Sets the widget [`Id`] of the button.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width of the button; the content stays at its start.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the reading direction, which orders the content and places the
    /// menu.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<Select<'a, Message>> for Element<'a, Message> {
    fn from(select: Select<'a, Message>) -> Self {
        let Select {
            groups,
            value,
            selected_label,
            placeholder,
            leading,
            on_change,
            open,
            on_open_change,
            id,
            width,
            direction,
        } = select;

        let mut items = Vec::new();

        for (group, choices) in groups {
            if let Some(name) = group {
                items.push(header(name));
            }

            for choice in choices {
                let message = on_change
                    .as_ref()
                    .filter(|_| !choice.disabled)
                    .map(|f| f(choice.value.clone()));
                let checked = value.as_deref() == Some(choice.value.as_str());

                items.push(checked_item(checked, choice.label).on_press_maybe(message));
            }
        }

        let label: Element<'a, Message> = match selected_label {
            Some(label) => styled_text(label, BUTTON_LABEL).into(),
            None => styled_text(placeholder.unwrap_or_default(), BUTTON_LABEL)
                .style(|theme: &Theme| text::Style {
                    color: Some(theme.roles().on_surface_variant),
                })
                .into(),
        };

        let on_open_change = on_open_change.filter(|_| on_change.is_some());

        PopupMenuButton {
            label,
            leading,
            items,
            open,
            on_open_change,
            tooltip: None,
            id,
            width,
            direction,
        }
        .into()
    }
}

/// The highlight of the first menu list found in `tree`, if any.
#[cfg(test)]
pub(crate) fn find_highlight(tree: &Tree) -> Option<Option<usize>> {
    if tree.tag == tree::Tag::of::<State>() {
        return Some(tree.state.downcast_ref::<State>().highlight);
    }

    tree.children.iter().find_map(find_highlight)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENABLED: [bool; 5] = [false, true, false, true, false];

    #[test]
    fn steps_skip_disabled_items_and_stop_at_the_ends() {
        assert_eq!(step(&ENABLED, None, true), Some(1));
        assert_eq!(step(&ENABLED, None, false), Some(3));
        assert_eq!(step(&ENABLED, Some(1), true), Some(3));
        assert_eq!(step(&ENABLED, Some(3), true), Some(3));
        assert_eq!(step(&ENABLED, Some(3), false), Some(1));
        assert_eq!(step(&ENABLED, Some(1), false), Some(1));
        assert_eq!(step(&[false, false], None, true), None);
    }

    #[test]
    fn first_and_last_skip_disabled_items() {
        assert_eq!(first(&ENABLED), Some(1));
        assert_eq!(last(&ENABLED), Some(3));
        assert_eq!(first(&[]), None);
    }

    #[test]
    fn the_highlight_resets_when_the_list_closes() {
        let mut state = State {
            highlight: Some(3),
            was_open: true,
            ..State::default()
        };

        state.sync(false, false, &ENABLED);
        assert_eq!(state.highlight, None);

        state.sync(true, true, &ENABLED);
        assert_eq!(state.highlight, Some(1));

        state.highlight = Some(3);
        state.sync(true, true, &ENABLED);
        assert_eq!(state.highlight, Some(3), "kept while open");
    }

    #[derive(Debug, Clone)]
    struct Pick;

    #[test]
    fn a_list_opened_with_the_flag_highlights_its_first_enabled_item() {
        let items = || {
            [
                header("Group"),
                item("Disabled"),
                item("Enabled").on_press(Pick),
            ]
        };
        let flagged: Element<'_, Pick> =
            List::new(items(), true, None, ReadingDirection::LeftToRight)
                .focus_first(Rc::new(Cell::new(true)))
                .into();
        let plain: Element<'_, Pick> =
            List::new(items(), true, None, ReadingDirection::LeftToRight).into();

        assert_eq!(
            find_highlight(&Tree::new(flagged.as_widget())),
            Some(Some(2))
        );
        assert_eq!(find_highlight(&Tree::new(plain.as_widget())), Some(None));
    }
}
