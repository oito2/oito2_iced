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

//! Autocomplete: a [text field](super::text_field) that suggests items of
//! a [`ListCollection`] as the user types, as a combobox.
//!
//! By default the suggestions are the enabled items whose label contains
//! the typed text, ignoring case and accents
//! ([`ListCollection::suggestions`]); none show while the field is empty
//! unless [`Autocomplete::show_all_when_empty`] is set. The application may
//! pass its own suggestions with [`Autocomplete::suggestions`] instead.
//!
//! While the field has the focus and there are suggestions, they show in a
//! panel below the field, as wide as the field and at most
//! [`OPTIONS_MAX_HEIGHT`] tall before it scrolls, each label with the typed
//! text highlighted. The Up and Down arrows move the highlighted option
//! while the focus stays in the field, Enter picks it and Escape closes the
//! panel until the text changes; a click picks an option too. Picking
//! publishes the label of the option as the new text and the
//! [`Autocomplete::on_select`] message, and closes the panel until the text
//! changes again.
//!
//! ```no_run
//! use oito2_iced::collections::ListCollection;
//! use oito2_iced::widget::autocomplete::autocomplete;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     City(String),
//!     Picked(&'static str),
//! }
//!
//! # fn view<'a>(
//! #     cities: &'a ListCollection<&'static str>,
//! #     city: &'a str,
//! # ) -> oito2_iced::Element<'a, Message> {
//! autocomplete("City", city, cities)
//!     .on_input(Message::City)
//!     .on_select(Message::Picked)
//!     .into()
//! # }
//! ```

use std::collections::HashSet;
use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::{Id, Operation, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::{button, column, scrollable};
use iced::{Event, Length, Padding, Rectangle, Size, Vector, mouse};

use super::highlighted_text::highlighted_text;
use super::popover::run_widget;
use super::text_field;
use crate::collections::ListCollection;
use crate::theme::button::Class as ButtonClass;
use crate::theme::{HOVER_OPACITY, PRESSED_OPACITY, state_layer};
use crate::tokens::spacing;
use crate::{Element, Theme};

/// The tallest the suggestion panel grows before it scrolls, unless set
/// with [`Autocomplete::options_max_height`].
pub const OPTIONS_MAX_HEIGHT: f32 = 240.0;

/// The width of the panel outline, kept free so the highlighted option
/// does not cover it.
const OUTLINE_WIDTH: f32 = 1.0;

/// A text field showing `value`, with `placeholder` while it is empty,
/// that suggests the items of `collection` matching the text. Without
/// [`Autocomplete::on_input`] it is disabled.
pub fn autocomplete<'a, T, Message>(
    placeholder: &str,
    value: &str,
    collection: &'a ListCollection<T>,
) -> Autocomplete<'a, T, Message>
where
    T: 'static,
{
    Autocomplete {
        placeholder: placeholder.to_owned(),
        value: value.to_owned(),
        collection,
        suggestions: None,
        show_all_when_empty: false,
        on_input: None,
        on_select: None,
        on_submit: None,
        id: None,
        label: None,
        helper: None,
        width: Length::Fill,
        options_max_height: OPTIONS_MAX_HEIGHT,
        options_width: None,
    }
}

/// An autocomplete field. See [`autocomplete`].
pub struct Autocomplete<'a, T, Message> {
    placeholder: String,
    value: String,
    collection: &'a ListCollection<T>,
    suggestions: Option<Vec<&'a T>>,
    show_all_when_empty: bool,
    on_input: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    on_select: Option<Rc<dyn Fn(T) -> Message + 'a>>,
    on_submit: Option<Message>,
    id: Option<Id>,
    label: Option<String>,
    helper: Option<String>,
    width: Length,
    options_max_height: f32,
    options_width: Option<f32>,
}

impl<'a, T, Message> Autocomplete<'a, T, Message>
where
    T: 'static,
{
    /// Sets the message produced from the new text on every edit and when an
    /// option is picked; without it the field is disabled.
    #[must_use]
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Rc::new(on_input));
        self
    }

    /// Sets the message produced from the picked item.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(T) -> Message + 'a) -> Self {
        self.on_select = Some(Rc::new(on_select));
        self
    }

    /// Sets the message published on Enter while no suggestion shows.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Shows `suggestions`, in order and as given, instead of the items of
    /// the collection that match the text.
    #[must_use]
    pub fn suggestions(mut self, suggestions: impl IntoIterator<Item = &'a T>) -> Self {
        self.suggestions = Some(suggestions.into_iter().collect());
        self
    }

    /// Suggests every enabled item while the field is empty (false by
    /// default).
    #[must_use]
    pub fn show_all_when_empty(mut self, show: bool) -> Self {
        self.show_all_when_empty = show;
        self
    }

    /// Sets the widget [`Id`] of the text input.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Shows `helper` below the field.
    #[must_use]
    pub fn helper(mut self, helper: impl Into<String>) -> Self {
        self.helper = Some(helper.into());
        self
    }

    /// Sets the width of the field (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the tallest the suggestion panel grows before it scrolls
    /// ([`OPTIONS_MAX_HEIGHT`] by default).
    #[must_use]
    pub fn options_max_height(mut self, height: f32) -> Self {
        self.options_max_height = height;
        self
    }

    /// Sets the width of the suggestion panel (the width of the field by
    /// default).
    #[must_use]
    pub fn options_width(mut self, width: f32) -> Self {
        self.options_width = Some(width);
        self
    }

    /// The items to suggest for the current text.
    fn items(&self) -> Vec<&'a T> {
        if let Some(suggestions) = &self.suggestions {
            return suggestions.clone();
        }

        if self.value.is_empty() && !self.show_all_when_empty {
            return Vec::new();
        }

        self.collection.suggestions(&self.value, &HashSet::new())
    }
}

/// A message of the composition: a message of the app, or the pick of the
/// option at an index.
#[derive(Debug, Clone)]
enum Inner<Message> {
    App(Message),
    Pick(usize),
}

/// What the composition shows: whether the panel is open and which option
/// is highlighted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Key {
    open: bool,
    highlight: usize,
}

type View<'a, Message> = Box<dyn Fn(Key) -> Element<'a, Inner<Message>> + 'a>;
type Pick<'a, Message> = Box<dyn Fn(usize) -> Vec<Message> + 'a>;

impl<'a, T, Message> From<Autocomplete<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + 'static,
    Message: Clone + 'a,
{
    fn from(field: Autocomplete<'a, T, Message>) -> Self {
        let items = field.items();
        let labels: Rc<[String]> = items
            .iter()
            .map(|item| field.collection.label_of(item))
            .collect();

        let pick: Pick<'a, Message> = {
            let labels = Rc::clone(&labels);
            let on_input = field.on_input.clone();
            let on_select = field.on_select.clone();

            Box::new(move |index| {
                let mut messages = Vec::new();

                if let (Some(item), Some(label)) = (items.get(index), labels.get(index)) {
                    if let Some(on_input) = &on_input {
                        messages.push(on_input(label.clone()));
                    }

                    if let Some(on_select) = &on_select {
                        messages.push(on_select((*item).clone()));
                    }
                }

                messages
            })
        };

        let value = field.value.clone();
        let view: View<'a, Message> = {
            let labels = Rc::clone(&labels);

            Box::new(move |key| {
                let mut input =
                    text_field::text_field(&field.placeholder, &field.value).width(field.width);

                if let Some(on_input) = &field.on_input {
                    let on_input = Rc::clone(on_input);
                    input = input.on_input(move |text| Inner::App(on_input(text)));
                }

                if let Some(message) = &field.on_submit {
                    input = input.on_submit(Inner::App(message.clone()));
                }

                if let Some(id) = &field.id {
                    input = input.id(id.clone());
                }

                if let Some(label) = &field.label {
                    input = input.label(label.clone());
                }

                if let Some(helper) = &field.helper {
                    input = input.helper(helper.clone());
                }

                #[cfg(feature = "svg")]
                {
                    input = input.trailing(chevrons());
                }

                let panel = options(&labels, &field.value, key.highlight);

                let anchored = crate::overlay::anchored(input, panel)
                    .menu()
                    .padding(Padding::new(OUTLINE_WIDTH))
                    .max_height(field.options_max_height)
                    .open(key.open);

                match field.options_width {
                    Some(width) => anchored.max_width(width).into(),
                    None => anchored.fit_anchor_width(true).into(),
                }
            })
        };

        Element::new(Combo {
            view,
            content: None,
            pick,
            labels,
            value,
        })
    }
}

/// The trailing icon of the field: the up and down chevrons of a combobox.
#[cfg(feature = "svg")]
fn chevrons<'a, Message: 'a>() -> Element<'a, Message> {
    use crate::icon::{icon, lucide};
    use crate::theme::container::Class as ContainerClass;
    use crate::tokens::sizes;
    use iced::widget::container;

    container(icon(lucide::CHEVRONS_UP_DOWN).size(sizes::ICON_SIZE))
        .class(ContainerClass::Custom(Box::new(|theme: &Theme| {
            container::Style::default().color(theme.roles().on_surface_variant)
        })))
        .into()
}

/// The list of options, with `query` highlighted in each label and the
/// option at `highlight` drawn selected.
fn options<'a, Message: Clone + 'a>(
    labels: &[String],
    query: &str,
    highlight: usize,
) -> Element<'a, Inner<Message>> {
    let rows = labels.iter().enumerate().map(|(index, label)| {
        let selected = index == highlight;

        button(highlighted_text(label.clone(), query))
            .width(Length::Fill)
            .padding([spacing::SM, spacing::MD])
            .class(ButtonClass::Custom(Box::new(
                move |theme: &Theme, status: button::Status| option_style(theme, status, selected),
            )))
            .on_press(Inner::Pick(index))
            .into()
    });

    scrollable(column(rows).padding([spacing::XS, 0.0]).width(Length::Fill))
        .width(Length::Fill)
        .into()
}

/// The style of an option: the secondary container while highlighted,
/// otherwise transparent with a state layer while hovered or pressed.
fn option_style(theme: &Theme, status: button::Status, selected: bool) -> button::Style {
    let roles = theme.roles();

    if selected {
        return button::Style {
            background: Some(roles.secondary_container.into()),
            text_color: roles.on_secondary_container,
            ..button::Style::default()
        };
    }

    let layer = match status {
        button::Status::Hovered => Some(HOVER_OPACITY),
        button::Status::Pressed => Some(PRESSED_OPACITY),
        button::Status::Active | button::Status::Disabled => None,
    };

    button::Style {
        background: layer
            .map(|opacity| state_layer(roles.surface_container, roles.on_surface, opacity).into()),
        text_color: roles.on_surface,
        ..button::Style::default()
    }
}

/// The state of an autocomplete field in the widget tree.
#[derive(Debug, Default)]
struct State {
    /// The text the state was last reconciled with.
    value: Option<String>,
    /// Whether the text input has the focus.
    focused: bool,
    /// Whether Escape closed the panel for the current text.
    hidden: bool,
    /// The label of the last picked option, which keeps the panel closed
    /// while the text equals it.
    picked: Option<String>,
    /// The index of the highlighted option.
    highlight: usize,
    /// Whether the highlighted option must be scrolled into view.
    reveal: bool,
}

impl State {
    /// Resets the panel when the text changed since the last call.
    fn reconcile(&mut self, value: &str) {
        if self.value.as_deref() == Some(value) {
            return;
        }

        if self.value.is_some() {
            self.hidden = false;
            self.highlight = 0;
        }

        if self.picked.as_deref() != Some(value) {
            self.picked = None;
        }

        self.value = Some(value.to_owned());
    }

    /// What the composition shows with `count` options.
    fn key(&self, count: usize) -> Key {
        let open = self.focused
            && !self.hidden
            && count > 0
            && self
                .picked
                .as_ref()
                .is_none_or(|picked| Some(picked.as_str()) != self.value.as_deref());

        Key {
            open,
            highlight: self.highlight.min(count.saturating_sub(1)),
        }
    }
}

/// The widget of an autocomplete field: rebuilds the composition when the
/// panel opens, closes or moves its highlight, and handles the keys of the
/// panel while the text input has the focus.
struct Combo<'a, Message> {
    view: View<'a, Message>,
    content: Option<(Key, Element<'a, Inner<Message>>)>,
    pick: Pick<'a, Message>,
    labels: Rc<[String]>,
    value: String,
}

impl<'a, Message: 'a> Combo<'a, Message> {
    /// Builds the composition for `key` unless it shows it already, and
    /// returns it.
    fn content(&mut self, key: Key, tree: &mut Tree) -> &mut Element<'a, Inner<Message>> {
        let stale = self.content.as_ref().is_none_or(|(built, _)| *built != key);

        if stale {
            let content = (self.view)(key);
            tree.diff(&content);
            self.content = Some((key, content));
        }

        let view = &self.view;
        let (_, content) = self.content.get_or_insert_with(|| (key, view(key)));

        content
    }
}

/// Publishes the messages of a pick and closes the panel until the text
/// changes.
fn pick<Message>(
    state: &mut State,
    index: usize,
    labels: &[String],
    pick: &dyn Fn(usize) -> Vec<Message>,
    shell: &mut Shell<'_, Message>,
) {
    state.picked = labels.get(index).cloned();

    for message in pick(index) {
        shell.publish(message);
    }

    shell.invalidate_layout();
    shell.request_redraw();
}

/// Copies to `shell` everything but the messages of `local`.
fn absorb<A, B>(shell: &mut Shell<'_, A>, local: &Shell<'_, B>) {
    if local.is_event_captured() {
        shell.capture_event();
    }

    shell.request_redraw_at(local.redraw_request());
    shell.request_input_method(local.input_method());

    if local.is_layout_invalid() {
        shell.invalidate_layout();
    }

    if local.are_widgets_invalid() {
        shell.invalidate_widgets();
    }
}

/// Whether the composition has the focus.
fn has_focus<Message>(
    content: &mut Element<'_, Inner<Message>>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
) -> bool {
    run_widget(
        content.as_widget_mut(),
        tree,
        layout,
        renderer,
        crate::focus::operations::has_focus(),
    )
    .unwrap_or(false)
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Combo<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new((self.view)(Key::default()))]
    }

    fn diff(&self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        state.reconcile(&self.value);
        let content = (self.view)(state.key(self.labels.len()));

        if let Some(child) = children.first_mut() {
            child.diff(&content);
        }
    }

    fn size(&self) -> Size<Length> {
        (self.view)(Key::default()).as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        (self.view)(Key::default()).as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let Some(child) = children.first_mut() else {
            return layout::Node::new(Size::ZERO);
        };
        let count = self.labels.len();

        state.reconcile(&self.value);

        let key = state.key(count);
        let content = self.content(key, child);
        let node = content.as_widget_mut().layout(child, renderer, limits);
        let focused = has_focus(content, child, Layout::new(&node), renderer);

        if focused == state.focused {
            return node;
        }

        state.focused = focused;
        let key = state.key(count);
        let content = self.content(key, child);

        content.as_widget_mut().layout(child, renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let (Some((_, content)), Some(child)) = (&mut self.content, tree.children.first_mut()) {
            content
                .as_widget_mut()
                .operate(child, layout, renderer, operation);
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
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let (Some((_, content)), Some(child)) = (&mut self.content, children.first_mut()) else {
            return;
        };
        let count = self.labels.len();

        let focused = has_focus(content, child, layout, renderer);

        if focused != state.focused {
            state.focused = focused;
            shell.invalidate_layout();
            shell.request_redraw();
        }

        if state.focused
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                ..
            }) = event
        {
            let key = state.key(count);

            match named {
                key::Named::ArrowDown | key::Named::ArrowUp if count > 0 => {
                    if key.open {
                        state.highlight = if *named == key::Named::ArrowDown {
                            (key.highlight + 1) % count
                        } else {
                            (key.highlight + count - 1) % count
                        };
                    } else {
                        state.hidden = false;
                        state.picked = None;
                    }

                    state.reveal = true;
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                    return;
                }
                key::Named::Enter if key.open => {
                    pick(state, key.highlight, &self.labels, &self.pick, shell);
                    shell.capture_event();
                    return;
                }
                key::Named::Escape if key.open => {
                    state.hidden = true;
                    shell.capture_event();
                    shell.invalidate_layout();
                    shell.request_redraw();
                    return;
                }
                _ => {}
            }
        }

        let mut messages = Vec::new();
        {
            let mut local = Shell::new(&mut messages);

            content.as_widget_mut().update(
                child, event, layout, cursor, renderer, clipboard, &mut local, viewport,
            );

            absorb(shell, &local);
        }

        for message in messages {
            match message {
                Inner::App(message) => shell.publish(message),
                Inner::Pick(index) => pick(state, index, &self.labels, &self.pick, shell),
            }
        }

        let focused = has_focus(content, child, layout, renderer);

        if focused != state.focused {
            state.focused = focused;
            shell.invalidate_layout();
            shell.request_redraw();
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
        if let (Some((_, content)), Some(child)) = (&self.content, tree.children.first()) {
            content
                .as_widget()
                .draw(child, renderer, theme, style, layout, cursor, viewport);
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
        match (&self.content, tree.children.first()) {
            (Some((_, content)), Some(child)) => content
                .as_widget()
                .mouse_interaction(child, layout, cursor, viewport, renderer),
            _ => mouse::Interaction::None,
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
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let Self {
            content,
            pick,
            labels,
            ..
        } = self;
        let (_, content) = content.as_mut()?;
        let child = children.first_mut()?;

        let inner =
            content
                .as_widget_mut()
                .overlay(child, layout, renderer, viewport, translation)?;

        Some(overlay::Element::new(Box::new(Options {
            inner,
            state,
            labels,
            pick,
        })))
    }
}

/// An overlay of the composition, such as the option panel: publishes the
/// app messages of its content, handles the picks and scrolls the
/// highlighted option into view.
struct Options<'b, Message> {
    inner: overlay::Element<'b, Inner<Message>, Theme, iced::Renderer>,
    state: &'b mut State,
    labels: &'b [String],
    pick: &'b dyn Fn(usize) -> Vec<Message>,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Options<'_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        let node = self.inner.as_overlay_mut().layout(renderer, bounds);

        if std::mem::take(&mut self.state.reveal) {
            let mut reveal = Reveal {
                index: self.state.highlight,
                count: self.labels.len(),
            };

            self.inner
                .as_overlay_mut()
                .operate(Layout::new(&node), renderer, &mut reveal);
        }

        node
    }

    fn draw(
        &self,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        self.inner
            .as_overlay()
            .draw(renderer, theme, style, layout, cursor);
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.inner
            .as_overlay_mut()
            .operate(layout, renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut messages = Vec::new();
        {
            let mut local = Shell::new(&mut messages);

            self.inner
                .as_overlay_mut()
                .update(event, layout, cursor, renderer, clipboard, &mut local);

            absorb(shell, &local);
        }

        for message in messages {
            match message {
                Inner::App(message) => shell.publish(message),
                Inner::Pick(index) => pick(self.state, index, self.labels, self.pick, shell),
            }
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.inner
            .as_overlay()
            .mouse_interaction(layout, cursor, renderer)
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'c>,
        renderer: &iced::Renderer,
    ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
        let inner = self.inner.as_overlay_mut().overlay(layout, renderer)?;

        Some(overlay::Element::new(Box::new(Options {
            inner,
            state: self.state,
            labels: self.labels,
            pick: self.pick,
        })))
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}

/// Scrolls the option list so the option at `index` of `count` sits in
/// the middle of the panel, when the list is taller than the panel.
struct Reveal {
    index: usize,
    count: usize,
}

impl Reveal {
    /// The vertical offset that centers the option, for a list of
    /// `content` height in a panel of `visible` height.
    fn offset(&self, visible: f32, content: f32) -> Option<f32> {
        if self.count == 0 || content <= visible {
            return None;
        }

        let row = (content - 2.0 * spacing::XS) / self.count as f32;
        let center = spacing::XS + row * (self.index as f32 + 0.5);

        Some((center - visible / 2.0).clamp(0.0, content - visible))
    }
}

impl Operation for Reveal {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        _translation: Vector,
        state: &mut dyn Scrollable,
    ) {
        if let Some(y) = self.offset(bounds.height, content_bounds.height) {
            state.scroll_to(AbsoluteOffset {
                x: None,
                y: Some(y),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_panel_opens_while_focused_with_options() {
        let mut state = State::default();
        state.reconcile("sa");

        assert!(!state.key(2).open, "closed without the focus");

        state.focused = true;
        assert!(state.key(2).open);
        assert!(!state.key(0).open, "closed without options");
    }

    #[test]
    fn escape_and_picks_close_the_panel_until_the_text_changes() {
        let mut state = State {
            focused: true,
            ..State::default()
        };
        state.reconcile("sa");

        state.hidden = true;
        assert!(!state.key(2).open);
        state.reconcile("sal");
        assert!(state.key(2).open, "a new text reopens it");

        state.picked = Some("Salvador".to_owned());
        state.reconcile("Salvador");
        assert!(!state.key(1).open, "the picked label keeps it closed");
        state.reconcile("Salvado");
        assert!(state.key(1).open);
    }

    #[test]
    fn a_new_text_resets_the_highlight_and_the_highlight_stays_in_range() {
        let mut state = State {
            focused: true,
            highlight: 3,
            ..State::default()
        };
        state.reconcile("b");
        assert_eq!(state.key(2).highlight, 1, "clamped to the last option");

        state.reconcile("br");
        assert_eq!(state.key(2).highlight, 0);
    }

    #[test]
    fn the_highlighted_option_scrolls_to_the_middle() {
        let reveal = Reveal {
            index: 9,
            count: 10,
        };
        let content = 2.0 * spacing::XS + 10.0 * 36.0;

        assert_eq!(reveal.offset(240.0, 200.0), None, "no scroll when it fits");
        assert_eq!(reveal.offset(240.0, content), Some(content - 240.0));
        assert_eq!(
            Reveal {
                index: 0,
                count: 10
            }
            .offset(240.0, content),
            Some(0.0)
        );
    }
}
