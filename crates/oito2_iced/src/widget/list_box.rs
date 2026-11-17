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

//! List boxes with single or multiple selection and type-ahead.
//!
//! A list box shows the items of the
//! [`ListCollection`](crate::collections::ListCollection) of a
//! [`SelectionController`] and lets the user select from them. The list
//! takes the focus as a whole and marks its active row. The arrow keys,
//! Home and End move the active row, which a single-selection list also
//! selects. In a multiple-selection list Space toggles the active row,
//! Shift with the arrows extends the selection and Control (or the logo
//! key) with A selects all; clicks follow the desktop conventions of
//! [`SelectionController::click`]. Typing letters jumps to the next
//! matching label. Enter or a double click (two clicks on the same row
//! within [`DOUBLE_CLICK_TIMEOUT`]) activates an item. Grouped items show
//! under their group headings; disabled items can't be selected or
//! activated.
//!
//! The selection lives in the application: the list box shows the
//! controller and publishes each gesture as a [`SelectionChange`], which
//! the application applies to its controller with
//! [`SelectionChange::apply`]. Without [`ListBox::on_change`] the list is
//! read-only and skipped by Tab.
//!
//! Every row is built, so it suits lists of up to a few hundred items.
//!
//! ```no_run
//! use oito2_iced::collections::SelectionController;
//! use oito2_iced::widget::list_box::{self, SelectionChange};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Selection(SelectionChange),
//!     Open(String),
//! }
//!
//! # fn view<'a>(
//! #     fruits: &'a SelectionController<&'static str>,
//! # ) -> oito2_iced::Element<'a, Message> {
//! list_box::list_box(fruits)
//!     .on_change(Message::Selection)
//!     .on_activate(|fruit| Message::Open(fruit.to_string()))
//!     .max_height(240.0)
//!     .into()
//! # }
//! ```
//!
//! [`DOUBLE_CLICK_TIMEOUT`]: super::helpers::DOUBLE_CLICK_TIMEOUT

use std::time::Duration;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, LineJoin, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, Modifiers, key};
use iced::time::Instant;
use iced::widget::scrollable::Direction;
use iced::widget::text::IntoFragment;
use iced::widget::{Column, Row, Space, container, scrollable, text};
use iced::{
    Alignment, Background, Border, Color, Element as IcedElement, Event, Length, Padding, Point,
    Rectangle, Size, Vector, mouse, touch,
};

use super::helpers::DoubleClick;
use super::min_size::{Minimum, min_size};
use super::{scroll_area, styled_text};
use crate::collections::{SelectionController, SelectionMode};
use crate::theme::{DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY};
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme};

/// The smallest height of a row (a literal in the OITO2 design).
pub const ROW_MIN_HEIGHT: f32 = 32.0;

/// The space between two rows (a literal in the OITO2 design).
const ROW_SPACING: f32 = 2.0;

/// The side of the check mark of a selected row in a multiple-selection
/// list (a literal in the OITO2 design).
const CHECK_SIZE: f32 = 16.0;

/// The pause after which the text typed for type-ahead starts over.
pub const TYPE_AHEAD_TIMEOUT: Duration = Duration::from_millis(600);

/// A selection gesture of a [`ListBox`], applied to the application's
/// [`SelectionController`] with [`SelectionChange::apply`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionChange {
    /// A click on `value`: with `extend` (Shift) it selects the range from
    /// the anchor, with `toggle` (Control or the logo key) it toggles the
    /// value; see [`SelectionController::click`].
    Click {
        /// The clicked item.
        value: String,
        /// Whether Shift was held.
        extend: bool,
        /// Whether Control or the logo key was held.
        toggle: bool,
    },
    /// Makes `value` the only selected item: the arrows of a single
    /// selection list.
    Replace(String),
    /// Selects the range from `anchor` to `target`: Shift with the arrows.
    Extend {
        /// The item the range starts at.
        anchor: String,
        /// The item the range ends at.
        target: String,
    },
    /// Toggles `value`: Space in a multiple-selection list.
    Toggle(String),
    /// Selects every enabled item: Control+A in a multiple-selection list.
    SelectAll,
}

impl SelectionChange {
    /// Applies the gesture to `controller`; returns whether the selection
    /// may have changed.
    pub fn apply<T: 'static>(&self, controller: &mut SelectionController<T>) -> bool {
        match self {
            SelectionChange::Click {
                value,
                extend,
                toggle,
            } => controller.click(value, *extend, *toggle),
            SelectionChange::Replace(value) => controller.replace(Some(value)),
            SelectionChange::Extend { anchor, target } => controller.extend(anchor, target),
            SelectionChange::Toggle(value) => controller.toggle(value),
            SelectionChange::SelectAll => controller.select_all(),
        }
    }
}

/// The view of the content of a row: the item and whether it is selected.
type ItemView<'a, T, Message> = Box<dyn Fn(&T, bool) -> Element<'a, Message> + 'a>;

/// The message of an activated item.
type OnActivate<'a, T, Message> = Box<dyn Fn(&T) -> Message + 'a>;

/// A list of items to select from. See [`list_box`].
pub struct ListBox<'a, T, Message> {
    controller: &'a SelectionController<T>,
    item_view: Option<ItemView<'a, T, Message>>,
    on_change: Option<Box<dyn Fn(SelectionChange) -> Message + 'a>>,
    on_activate: Option<OnActivate<'a, T, Message>>,
    max_height: f32,
    width: Length,
    id: Option<Id>,
}

/// A list box showing the items of the collection of `controller`, with
/// its selection. Each row shows the item label unless
/// [`ListBox::item_view`] says otherwise.
pub fn list_box<'a, T: 'static, Message>(
    controller: &'a SelectionController<T>,
) -> ListBox<'a, T, Message> {
    ListBox {
        controller,
        item_view: None,
        on_change: None,
        on_activate: None,
        max_height: f32::INFINITY,
        width: Length::Fill,
        id: None,
    }
}

impl<'a, T: 'static, Message> ListBox<'a, T, Message> {
    /// Sets the view of the content of a row, given the item and whether
    /// it is selected; the row draws the selection fill and sets the text
    /// color around it.
    #[must_use]
    pub fn item_view(mut self, view: impl Fn(&T, bool) -> Element<'a, Message> + 'a) -> Self {
        self.item_view = Some(Box::new(view));
        self
    }

    /// Sets the function that turns a selection gesture into a message.
    /// Without it the list is read-only and skipped by Tab.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(SelectionChange) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the message published with an enabled item on Enter or a
    /// double click.
    #[must_use]
    pub fn on_activate(mut self, on_activate: impl Fn(&T) -> Message + 'a) -> Self {
        self.on_activate = Some(Box::new(on_activate));
        self
    }

    /// Sets the largest height; the list scrolls beyond it (unbounded by
    /// default).
    #[must_use]
    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height.max(0.0);
        self
    }

    /// Sets the width (fills the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the widget [`Id`], used to focus the list programmatically and
    /// to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, T: 'static, Message: 'a> From<ListBox<'a, T, Message>> for Element<'a, Message> {
    fn from(list: ListBox<'a, T, Message>) -> Self {
        let collection = list.controller.collection();
        let multiple = list.controller.options().mode == SelectionMode::Multiple;
        let mut children: Vec<Element<'a, Message>> = Vec::new();
        let mut slots = Vec::new();

        for group in collection.groups() {
            if let Some(name) = group.group {
                children.push(heading(name));
                slots.push(None);
            }

            for item in group.items {
                let value = collection.value_of(item);
                let selected = list.controller.is_selected(&value);
                let disabled = collection.is_disabled(item);
                let content = match &list.item_view {
                    Some(view) => view(item, selected),
                    None => styled_text(collection.label_of(item), typography::BODY_MEDIUM)
                        .width(Length::Fill)
                        .into(),
                };

                children.push(row(content, selected, disabled, multiple));
                slots.push(Some(value));
            }
        }

        let column = Column::with_children(children)
            .spacing(ROW_SPACING)
            .padding(spacing::XS)
            .width(Length::Fill);
        let content = scrollable(column)
            .direction(Direction::Vertical(scroll_area::scrollbar()))
            .width(Length::Fill);

        IcedElement::new(Inner {
            controller: list.controller,
            content: content.into(),
            slots,
            multiple,
            on_change: list.on_change,
            on_activate: list.on_activate,
            max_height: list.max_height,
            width: list.width,
            id: list.id,
        })
    }
}

/// A group heading.
fn heading<'a, Message: 'a>(name: impl IntoFragment<'a>) -> Element<'a, Message> {
    container(
        styled_text(name, typography::LABEL_MEDIUM).style(|theme: &Theme| text::Style {
            color: Some(theme.roles().on_surface_variant),
        }),
    )
    .padding(Padding {
        top: spacing::SM,
        right: spacing::SM,
        bottom: spacing::XXS,
        left: spacing::SM,
    })
    .into()
}

/// The row around `content`, filled while `selected`, with room for the
/// check mark in a `multiple` selection list.
fn row<'a, Message: 'a>(
    content: Element<'a, Message>,
    selected: bool,
    disabled: bool,
    multiple: bool,
) -> Element<'a, Message> {
    let mut line = Row::new()
        .push(container(content).width(Length::Fill))
        .spacing(spacing::SM)
        .align_y(Alignment::Center)
        .width(Length::Fill);

    if multiple {
        line = line.push(Space::new().width(CHECK_SIZE));
    }

    container(min_size(
        line,
        Minimum::None,
        Minimum::Fixed(ROW_MIN_HEIGHT - 2.0 * spacing::XS),
    ))
    .padding([spacing::XS, spacing::SM])
    .width(Length::Fill)
    .style(move |theme: &Theme| {
        let roles = theme.roles();
        let foreground = row_foreground(theme, selected, disabled);

        container::Style {
            text_color: Some(foreground),
            background: selected.then_some(Background::Color(roles.secondary_container)),
            border: Border::default().rounded(radius::SMALL),
            ..container::Style::default()
        }
    })
    .into()
}

/// The text and icon color of a row.
fn row_foreground(theme: &Theme, selected: bool, disabled: bool) -> Color {
    let roles = theme.roles();

    if disabled {
        roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
    } else if selected {
        roles.on_secondary_container
    } else {
        roles.on_surface
    }
}

/// The list box widget: the scrollable rows and the selection behavior.
struct Inner<'a, T, Message> {
    controller: &'a SelectionController<T>,
    content: Element<'a, Message>,
    /// The value of the row behind each child of the column, `None` for a
    /// heading.
    slots: Vec<Option<String>>,
    multiple: bool,
    on_change: Option<Box<dyn Fn(SelectionChange) -> Message + 'a>>,
    on_activate: Option<OnActivate<'a, T, Message>>,
    max_height: f32,
    width: Length,
    id: Option<Id>,
}

/// The tree state of a list box.
#[derive(Debug, Clone, Default)]
struct State {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the active row.
    keyboard: bool,
    /// The value of the active row, or `None` before the list is used.
    active: Option<String>,
    /// The row the left button went down on.
    pressed: Option<String>,
    modifiers: Modifiers,
    /// The text typed for type-ahead and when it was last typed.
    typed: String,
    typed_at: Option<Instant>,
    clicks: DoubleClick<String>,
    /// The scroll offset of the rows.
    translation: Vector,
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

/// What a key press did.
enum KeyOutcome {
    /// Nothing: the key is not one the list takes.
    Ignored,
    /// The key was handled without moving the active row.
    Handled,
    /// The active row moved to this value, which scrolls into view.
    Moved(String),
}

impl<T: 'static, Message> Inner<'_, T, Message> {
    fn enabled(&self) -> bool {
        self.on_change.is_some()
    }

    /// The value of the active row: the remembered one while it is still
    /// an item, otherwise the first selected or the first enabled item.
    fn active(&self, state: &State) -> Option<String> {
        let collection = self.controller.collection();

        state
            .active
            .clone()
            .filter(|value| collection.has(value))
            .or_else(|| self.controller.first_selected_value())
            .or_else(|| collection.first_value().map(str::to_owned))
    }

    /// The value and bounds of each row, at their unscrolled position.
    fn rows<'b>(&'b self, layout: Layout<'b>) -> impl Iterator<Item = (&'b str, Rectangle)> + 'b {
        layout
            .children()
            .next()
            .and_then(|scroll| scroll.children().next())
            .into_iter()
            .flat_map(|column| column.children())
            .zip(&self.slots)
            .filter_map(|(row, slot)| slot.as_deref().map(|value| (value, row.bounds())))
    }

    /// The bounds of the scrolled viewport.
    fn viewport(layout: Layout<'_>) -> Rectangle {
        layout
            .children()
            .next()
            .map_or(layout.bounds(), |scroll| scroll.bounds())
    }

    /// The value of the enabled row under `cursor`.
    fn row_at(&self, layout: Layout<'_>, state: &State, cursor: mouse::Cursor) -> Option<String> {
        let position = cursor.position_over(Self::viewport(layout))? + state.translation;

        self.rows(layout)
            .find(|(_, bounds)| bounds.contains(position))
            .map(|(value, _)| value.to_owned())
            .filter(|value| self.controller.collection().is_enabled(value))
    }

    fn publish(&self, shell: &mut Shell<'_, Message>, change: SelectionChange) {
        if let Some(on_change) = &self.on_change {
            shell.publish(on_change(change));
        }
    }

    /// Publishes the activation of `value` when it is an enabled item.
    fn activate(&self, shell: &mut Shell<'_, Message>, value: &str) -> bool {
        let collection = self.controller.collection();

        match (&self.on_activate, collection.find(value)) {
            (Some(on_activate), Some(item)) if !collection.is_disabled(item) => {
                shell.publish(on_activate(item));
                true
            }
            _ => false,
        }
    }

    /// Makes `value` the active row; a single-selection list also selects
    /// it, and a multiple one with `extend` extends the selection from the
    /// anchor to it.
    fn move_to(
        &self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        value: &str,
        extend: bool,
    ) -> KeyOutcome {
        let anchor = self
            .controller
            .anchor_value()
            .map(str::to_owned)
            .or_else(|| self.active(state));
        state.active = Some(value.to_owned());

        if !self.multiple {
            if self.controller.can_select(value) && !self.controller.is_selected(value) {
                self.publish(shell, SelectionChange::Replace(value.to_owned()));
            }
        } else if extend && let Some(anchor) = anchor {
            self.publish(
                shell,
                SelectionChange::Extend {
                    anchor,
                    target: value.to_owned(),
                },
            );
        }

        KeyOutcome::Moved(value.to_owned())
    }

    /// Handles navigation, selection, activation and type-ahead keys.
    fn key(
        &self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        key: &keyboard::Key,
        modifiers: Modifiers,
        text: Option<&str>,
    ) -> KeyOutcome {
        let collection = self.controller.collection();
        let shift = modifiers.shift();
        let command = modifiers.control() || modifiers.logo();
        let active = self.active(state);
        state.active.clone_from(&active);

        let target = |next: bool| -> Option<String> {
            match &active {
                None if next => collection.first_value(),
                None => collection.last_value(),
                Some(active) if next => collection
                    .next_value(active, false)
                    .or(Some(active.as_str())),
                Some(active) => collection
                    .previous_value(active, false)
                    .or(Some(active.as_str())),
            }
            .map(str::to_owned)
        };

        let moved = match key {
            keyboard::Key::Named(key::Named::ArrowDown) => Some(target(true)),
            keyboard::Key::Named(key::Named::ArrowUp) => Some(target(false)),
            keyboard::Key::Named(key::Named::Home) => {
                Some(collection.first_value().map(str::to_owned))
            }
            keyboard::Key::Named(key::Named::End) => {
                Some(collection.last_value().map(str::to_owned))
            }
            _ => None,
        };

        if let Some(value) = moved {
            return match value {
                Some(value) => self.move_to(state, shell, &value, shift),
                None => KeyOutcome::Handled,
            };
        }

        match key {
            keyboard::Key::Named(key::Named::Space) if self.multiple => {
                if let Some(active) = active {
                    self.publish(shell, SelectionChange::Toggle(active));
                    return KeyOutcome::Handled;
                }
                return KeyOutcome::Ignored;
            }
            keyboard::Key::Character(character)
                if command && self.multiple && character.eq_ignore_ascii_case("a") =>
            {
                self.publish(shell, SelectionChange::SelectAll);
                return KeyOutcome::Handled;
            }
            keyboard::Key::Named(key::Named::Enter) => {
                return match active {
                    Some(active) if self.activate(shell, &active) => KeyOutcome::Handled,
                    _ => KeyOutcome::Ignored,
                };
            }
            _ => {}
        }

        let Some(character) = text else {
            return KeyOutcome::Ignored;
        };
        if command || character.trim().is_empty() || character.chars().any(char::is_control) {
            return KeyOutcome::Ignored;
        }

        let now = Instant::now();
        if state
            .typed_at
            .is_none_or(|at| now.saturating_duration_since(at) >= TYPE_AHEAD_TIMEOUT)
        {
            state.typed.clear();
        }
        state.typed_at = Some(now);

        let repeat = state.typed == character;
        if repeat {
            state.typed = character.to_owned();
        } else {
            state.typed.push_str(character);
        }

        let from = (repeat || state.typed.chars().count() == 1)
            .then_some(active.as_deref())
            .flatten();
        let found = collection
            .search(&state.typed, from)
            .or_else(|| from.and_then(|_| collection.search(&state.typed, None)))
            .map(str::to_owned);

        match found {
            Some(value) => self.move_to(state, shell, &value, false),
            None => KeyOutcome::Handled,
        }
    }

    /// Scrolls the rows so the row of `value` shows whole.
    fn reveal(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        value: &str,
    ) {
        let Some(row) = self
            .rows(layout)
            .find(|(row, _)| *row == value)
            .map(|(_, bounds)| bounds)
        else {
            return;
        };
        let Some(scroll_layout) = layout.children().next() else {
            return;
        };
        let mut reveal = Reveal { row, done: false };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            scroll_layout,
            renderer,
            &mut reveal,
        );
    }

    /// Reads the scroll offset of the rows into `state`.
    fn probe(&mut self, tree: &mut Tree, layout: Layout<'_>, renderer: &iced::Renderer) {
        let Some(scroll_layout) = layout.children().next() else {
            return;
        };
        let mut probe = Probe { translation: None };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            scroll_layout,
            renderer,
            &mut probe,
        );

        if let Some(translation) = probe.translation {
            tree.state.downcast_mut::<State>().translation = translation;
        }
    }
}

/// An operation that reads the translation of the first scrollable.
struct Probe {
    translation: Option<Vector>,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if self.translation.is_none() {
            operate(self);
        }
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        _bounds: Rectangle,
        _content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.translation.get_or_insert(translation);
    }
}

/// An operation that scrolls the first scrollable just enough to show
/// `row`, given at its unscrolled position, whole.
struct Reveal {
    row: Rectangle,
    done: bool,
}

impl Operation for Reveal {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        if !self.done {
            operate(self);
        }
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        if self.done {
            return;
        }
        self.done = true;

        let top = self.row.y - content_bounds.y;
        let bottom = top + self.row.height;
        let offset = if top < translation.y {
            top
        } else if bottom > translation.y + bounds.height {
            bottom - bounds.height
        } else {
            return;
        };

        state.scroll_to(AbsoluteOffset {
            x: None,
            y: Some(offset.max(0.0)),
        });
    }
}

impl<T: 'static, Message> Widget<Message, Theme, iced::Renderer> for Inner<'_, T, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.width, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).max_height(self.max_height);
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &limits);
        let size = limits.resolve(self.width, Length::Shrink, content.size());

        layout::Node::with_children(size, vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let bounds = layout.bounds();
        let state = tree.state.downcast_mut::<State>();

        if self.enabled() {
            operation.focusable(self.id.as_ref(), bounds, state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), bounds);
        }

        let Some(content_layout) = layout.children().next() else {
            return;
        };

        operation.traverse(&mut |operation| {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                content_layout,
                renderer,
                operation,
            );
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

        let captured = shell.is_event_captured();
        self.probe(tree, layout, renderer);

        let state = tree.state.downcast_mut::<State>();

        if let Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) = event {
            state.modifiers = *modifiers;
        }

        if !self.enabled() {
            state.focused = false;
            state.pressed = None;
            return;
        }

        let mut reveal = None;

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    if !captured && let Some(value) = self.row_at(layout, state, cursor) {
                        state.pressed = Some(value);
                        shell.capture_event();
                        shell.request_redraw();
                    }
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(pressed) = state.pressed.take() {
                    if self.row_at(layout, state, cursor).as_ref() == Some(&pressed) {
                        state.focused = true;
                        state.keyboard = false;
                        state.active = Some(pressed.clone());
                        self.publish(
                            shell,
                            SelectionChange::Click {
                                value: pressed.clone(),
                                extend: state.modifiers.shift(),
                                toggle: state.modifiers.control() || state.modifiers.logo(),
                            },
                        );
                        if state.clicks.click(pressed.clone(), Instant::now()) {
                            let _ = self.activate(shell, &pressed);
                        }
                        shell.capture_event();
                    }
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                if cursor.is_over(layout.bounds()) {
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = None;
                shell.request_redraw();
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                text,
                ..
            }) if state.focused && !captured => {
                match self.key(state, shell, key, *modifiers, text.as_deref()) {
                    KeyOutcome::Ignored => {}
                    KeyOutcome::Handled => {
                        state.keyboard = true;
                        shell.capture_event();
                        shell.request_redraw();
                    }
                    KeyOutcome::Moved(value) => {
                        state.keyboard = true;
                        shell.capture_event();
                        shell.request_redraw();
                        reveal = Some(value);
                    }
                }
            }
            _ => {}
        }

        if let Some(value) = reveal {
            self.reveal(tree, layout, renderer, &value);
            self.probe(tree, layout, renderer);
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
        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let clip = Self::viewport(layout);
        let translation = state.translation;

        if self.enabled()
            && let Some(hovered) = self.row_at(layout, state, cursor)
            && let Some((_, bounds)) = self.rows(layout).find(|(value, _)| *value == hovered)
        {
            let opacity = if state.pressed.as_deref() == Some(hovered.as_str()) {
                PRESSED_OPACITY
            } else {
                HOVER_OPACITY
            };

            renderer.with_layer(clip, |renderer| {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: bounds - translation,
                        border: Border::default().rounded(radius::SMALL),
                        ..renderer::Quad::default()
                    },
                    roles.on_surface.scale_alpha(opacity),
                );
            });
        }

        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
        }

        let active = (state.focused && state.keyboard && self.enabled())
            .then(|| self.active(state))
            .flatten();
        let collection = self.controller.collection();

        renderer.with_layer(clip, |renderer| {
            for (value, bounds) in self.rows(layout) {
                let bounds = bounds - translation;

                if !bounds.intersects(&clip) {
                    continue;
                }

                if self.multiple && self.controller.is_selected(value) {
                    let color = row_foreground(theme, true, !collection.is_enabled(value));
                    let origin = Point::new(
                        bounds.x + bounds.width - spacing::SM - CHECK_SIZE,
                        bounds.center_y() - CHECK_SIZE / 2.0,
                    );

                    draw_check(renderer, origin, color);
                }

                if active.as_deref() == Some(value) {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: Border::default()
                                .color(roles.indicator())
                                .width(sizes::FOCUS_BORDER_WIDTH)
                                .rounded(radius::SMALL),
                            ..renderer::Quad::default()
                        },
                        Color::TRANSPARENT,
                    );
                }
            }
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let content = layout
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
            .unwrap_or_default();

        if content == mouse::Interaction::default()
            && self.enabled()
            && self.row_at(layout, state, cursor).is_some()
        {
            mouse::Interaction::Pointer
        } else {
            content
        }
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

/// Draws the check mark of a selected row, `CHECK_SIZE` square, with its
/// top left corner at `origin`: the Lucide check path on its 24-pixel
/// grid.
fn draw_check(renderer: &mut iced::Renderer, origin: Point, color: Color) {
    let scale = CHECK_SIZE / 24.0;
    let at = |x: f32, y: f32| Point::new(x * scale, y * scale);
    let mut frame = Frame::new(renderer, Size::new(CHECK_SIZE, CHECK_SIZE));
    let path = Path::new(|builder| {
        builder.move_to(at(20.0, 6.0));
        builder.line_to(at(9.0, 17.0));
        builder.line_to(at(4.0, 12.0));
    });

    frame.stroke(
        &path,
        Stroke::default()
            .with_color(color)
            .with_width(2.0 * scale)
            .with_line_cap(LineCap::Round)
            .with_line_join(LineJoin::Round),
    );

    draw_frame(renderer, frame, origin);
}

/// Draws the geometry of `frame` with its origin at `origin`.
fn draw_frame<Renderer>(renderer: &mut Renderer, frame: Frame<Renderer>, origin: Point)
where
    Renderer: geometry::Renderer,
{
    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}
