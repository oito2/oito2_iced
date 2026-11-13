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

//! Search fields: a [text field](super::text_field) with a search icon and
//! a clear button, and the search title field of header bars, which
//! switches between a title and a search field.
//!
//! A search field shows the localized "Search" hint and, while it has text,
//! a clear button with the localized "Clear text" tooltip. Escape clears
//! the text and keeps the focus in the field; Escape on an empty field
//! publishes the [`SearchField::on_escape`] message.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::widget::search;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Query(String),
//!     Close,
//! }
//!
//! # fn view<'a>(query: &'a str) -> oito2_iced::Element<'a, Message> {
//! search::search_field(&Localizations::en(), query)
//!     .on_input(Message::Query)
//!     .on_escape(Message::Close)
//!     .into()
//! # }
//! ```

#[cfg(feature = "svg")]
use std::borrow::Cow;
#[cfg(feature = "svg")]
use std::rc::Rc;

#[cfg(feature = "svg")]
use iced::advanced::widget::Id;
#[cfg(feature = "svg")]
use iced::keyboard::{self, key};
#[cfg(feature = "svg")]
use iced::widget::text::{IntoFragment, LineHeight};
#[cfg(feature = "svg")]
use iced::widget::{Row, Space, Stack, container, text_input};
#[cfg(feature = "svg")]
use iced::{Event, Length, Padding};

#[cfg(feature = "svg")]
use self::host::{Effects, Host, Msg, Reconcile, Signal, Target};
#[cfg(feature = "svg")]
use super::{button, text_field};
#[cfg(feature = "svg")]
use crate::icon::lucide;
#[cfg(feature = "svg")]
use crate::l10n::Localizations;
#[cfg(feature = "svg")]
use crate::theme::text_input::Class as InputClass;
#[cfg(feature = "svg")]
use crate::tokens::{spacing, typography};
#[cfg(feature = "svg")]
use crate::{Element, Theme, density};

/// The width of a [`search_title_field`] in both of its states, unless set
/// with [`SearchTitleField::width`].
pub const TITLE_FIELD_WIDTH: f32 = 260.0;

/// A search field showing `value`, with the localized "Search" hint and
/// clear button tooltip of `localizations`. Without
/// [`SearchField::on_input`] it is disabled.
#[cfg(feature = "svg")]
pub fn search_field<'a, Message: Clone + 'a>(
    localizations: &Localizations,
    value: &str,
) -> SearchField<'a, Message> {
    SearchField {
        value: value.to_owned(),
        placeholder: Cow::Borrowed(localizations.material.search_field),
        clear_label: localizations.material.clear_text,
        on_input: None,
        on_submit: None,
        on_clear: None,
        on_escape: None,
        id: None,
        width: Length::Fill,
    }
}

/// A search field. See [`search_field`].
#[cfg(feature = "svg")]
pub struct SearchField<'a, Message> {
    value: String,
    placeholder: Cow<'a, str>,
    clear_label: &'static str,
    on_input: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    on_clear: Option<Message>,
    on_escape: Option<Message>,
    id: Option<Id>,
    width: Length,
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> SearchField<'a, Message> {
    /// Sets the message produced from the new text on every edit, on the
    /// clear button and on Escape; without it the field is disabled.
    #[must_use]
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Rc::new(on_input));
        self
    }

    /// Sets the message published on Enter.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Sets the message published after the clear button or Escape empties
    /// the field.
    #[must_use]
    pub fn on_clear(mut self, message: Message) -> Self {
        self.on_clear = Some(message);
        self
    }

    /// Sets the message published when Escape is pressed on an empty field.
    #[must_use]
    pub fn on_escape(mut self, message: Message) -> Self {
        self.on_escape = Some(message);
        self
    }

    /// Replaces the localized "Search" hint.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Sets the widget [`Id`] of the input.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

/// The own events of the search fields.
#[cfg(feature = "svg")]
#[derive(Debug, Clone)]
enum Search {
    /// The clear button was pressed.
    Clear,
    /// The toggle of the title field was pressed.
    Toggle,
}

/// Publishes the empty text and the clear message.
#[cfg(feature = "svg")]
fn clear<'a, Message: Clone>(
    on_input: Option<&Rc<dyn Fn(String) -> Message + 'a>>,
    on_clear: Option<&Message>,
    effects: &mut Effects<Message>,
) {
    if let Some(on_input) = on_input {
        effects.publish(on_input(String::new()));
    }

    if let Some(message) = on_clear {
        effects.publish(message.clone());
    }
}

/// Returns whether `event` is a press of Escape.
#[cfg(feature = "svg")]
fn is_escape(event: &Event) -> bool {
    matches!(
        event,
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(key::Named::Escape),
            ..
        })
    )
}

/// The clear button of a search field, publishing the own clear event.
#[cfg(feature = "svg")]
fn clear_button<'a, Message: Clone + 'a>(label: &'static str) -> Element<'a, Msg<Message, Search>> {
    button::icon_button(lucide::X)
        .tooltip(label)
        .on_press(Msg::Own(Search::Clear))
        .into()
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> From<SearchField<'a, Message>> for Element<'a, Message> {
    fn from(field: SearchField<'a, Message>) -> Self {
        let has_text = !field.value.is_empty();

        let mut input = text_field::text_field(&field.placeholder, &field.value)
            .leading(lucide::SEARCH)
            .width(field.width);

        if let Some(on_input) = &field.on_input {
            let on_input = Rc::clone(on_input);
            input = input.on_input(move |text| Msg::App(on_input(text)));

            if has_text {
                input = input.trailing(clear_button(field.clear_label));
            }
        }

        if let Some(message) = field.on_submit {
            input = input.on_submit(Msg::App(message));
        }

        if let Some(id) = field.id {
            input = input.id(id);
        }

        let on_input = field.on_input;
        let on_clear = field.on_clear;
        let on_escape = field.on_escape;

        Host::fixed(
            input.into(),
            move |_: &mut (), signal: Signal<'_, Search>, effects: &mut Effects<Message>| {
                match signal {
                    Signal::Own(Search::Clear) => {
                        clear(on_input.as_ref(), on_clear.as_ref(), effects);
                    }
                    Signal::After(event, context) if context.input_focused && is_escape(event) => {
                        effects.capture();

                        if has_text {
                            clear(on_input.as_ref(), on_clear.as_ref(), effects);
                            effects.focus(Target::Input);
                        } else if let Some(message) = &on_escape {
                            effects.publish(message.clone());
                        }
                    }
                    _ => {}
                }
            },
        )
        .into()
    }
}

/// A title bar element that switches between `title` and a search field
/// showing `value`, with a toggle button at its start, as in GNOME apps.
///
/// The element is as wide as [`TITLE_FIELD_WIDTH`] and as tall as a field
/// in both states. While the search is closed, the title follows the
/// button; while it is open, the search field fills the box and the button
/// serves as its search icon. The toggle publishes
/// `on_active_change(!active)`; Escape on an empty field publishes
/// `on_active_change(false)`. Opening focuses the field; closing from the
/// toggle or with Escape returns the focus to the toggle.
#[cfg(feature = "svg")]
pub fn search_title_field<'a, Message: Clone + 'a>(
    localizations: &Localizations,
    title: impl Into<Element<'a, Message>>,
    value: &str,
    active: bool,
    on_active_change: impl Fn(bool) -> Message + 'a,
) -> SearchTitleField<'a, Message> {
    SearchTitleField {
        title: title.into(),
        value: value.to_owned(),
        active,
        on_active_change: Rc::new(on_active_change),
        placeholder: Cow::Borrowed(localizations.material.search_field),
        toggle_label: Cow::Borrowed(localizations.material.search_field),
        clear_label: localizations.material.clear_text,
        on_input: None,
        on_submit: None,
        id: None,
        width: TITLE_FIELD_WIDTH,
    }
}

/// A search title field. See [`search_title_field`].
#[cfg(feature = "svg")]
pub struct SearchTitleField<'a, Message> {
    title: Element<'a, Message>,
    value: String,
    active: bool,
    on_active_change: Rc<dyn Fn(bool) -> Message + 'a>,
    placeholder: Cow<'a, str>,
    toggle_label: Cow<'a, str>,
    clear_label: &'static str,
    on_input: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    id: Option<Id>,
    width: f32,
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> SearchTitleField<'a, Message> {
    /// Sets the message produced from the new search text on every edit,
    /// on the clear button and on Escape.
    #[must_use]
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Rc::new(on_input));
        self
    }

    /// Sets the message published on Enter in the search field.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    /// Replaces the localized "Search" hint of the field.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = placeholder.into_fragment();
        self
    }

    /// Replaces the localized "Search" tooltip of the toggle.
    #[must_use]
    pub fn toggle_label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.toggle_label = label.into_fragment();
        self
    }

    /// Sets the widget [`Id`] of the search input.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width of the element, in both states.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
}

/// The state of a search title field: the last state it showed and whether
/// it asked to close.
#[cfg(feature = "svg")]
#[derive(Debug, Default)]
struct TitleState {
    shown: Option<bool>,
    closing: bool,
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> From<SearchTitleField<'a, Message>> for Element<'a, Message> {
    fn from(field: SearchTitleField<'a, Message>) -> Self {
        let height = density::control_height();
        let active = field.active;
        let has_text = !field.value.is_empty();

        let base: Element<'a, Msg<Message, Search>> = if active {
            let mut input = text_input(&field.placeholder, &field.value);

            if let Some(on_input) = &field.on_input {
                let on_input = Rc::clone(on_input);
                input = input.on_input(move |text| Msg::App(on_input(text)));
            }

            if let Some(message) = field.on_submit {
                input = input.on_submit(Msg::App(message));
            }

            if let Some(id) = field.id {
                input = input.id(id);
            }

            let trailing = if has_text && field.on_input.is_some() {
                vec![clear_button(field.clear_label)]
            } else {
                Vec::new()
            };

            field_control(input, height, trailing, false)
        } else {
            container(field.title.map(Msg::App))
                .padding(Padding {
                    left: height + spacing::SM,
                    ..Padding::ZERO
                })
                .center_y(Length::Fill)
                .width(Length::Fill)
                .into()
        };

        let toggle = button::icon_button(lucide::SEARCH)
            .selected(active)
            .tooltip(field.toggle_label)
            .on_press(Msg::Own(Search::Toggle));

        let content = container(Stack::new().push(base).push(toggle).width(Length::Fill))
            .width(Length::Fixed(field.width))
            .height(Length::Fixed(height))
            .into();

        let on_input = field.on_input;
        let on_active_change = Rc::clone(&field.on_active_change);

        Host::fixed(
            content,
            move |state: &mut TitleState,
                  signal: Signal<'_, Search>,
                  effects: &mut Effects<Message>| {
                match signal {
                    Signal::Own(Search::Toggle) => {
                        if active {
                            state.closing = true;
                        }

                        effects.publish(on_active_change(!active));
                    }
                    Signal::Own(Search::Clear) => {
                        clear(on_input.as_ref(), None, effects);
                    }
                    Signal::After(event, context)
                        if active && context.input_focused && is_escape(event) =>
                    {
                        effects.capture();

                        if has_text {
                            clear(on_input.as_ref(), None, effects);
                            effects.focus(Target::Input);
                        } else {
                            state.closing = true;
                            effects.publish(on_active_change(false));
                        }
                    }
                    _ => {}
                }
            },
        )
        .reconcile(move |state: &mut TitleState, _| {
            let previous = state.shown.replace(active);

            let focus = match previous {
                Some(shown) if shown == active => None,
                _ if active => Some(Target::Input),
                Some(_) if std::mem::take(&mut state.closing) => Some(Target::Button),
                _ => None,
            };

            Reconcile {
                rebuild: false,
                focus,
            }
        })
        .into()
    }
}

/// Lays out `input` as the control of a text field: as tall as the control
/// height of the density, with `leading` pixels of free room at its start
/// for an element drawn over it and each `trailing` element in a square box
/// at its end.
#[cfg(feature = "svg")]
pub(crate) fn field_control<'a, Message: Clone + 'a>(
    input: text_input::TextInput<'a, Message, Theme>,
    leading: f32,
    trailing: Vec<Element<'a, Message>>,
    error: bool,
) -> Element<'a, Message> {
    let style = typography::BODY_LARGE;
    let height = density::control_height();
    let vertical = ((height - style.line_height) / 2.0).max(0.0);
    let count = trailing.len() as f32;

    let padding = Padding {
        top: vertical,
        bottom: vertical,
        left: if leading > 0.0 { leading } else { spacing::MD },
        right: if trailing.is_empty() {
            spacing::MD
        } else {
            height * count
        },
    };

    let input = input
        .padding(padding)
        .size(style.size)
        .line_height(LineHeight::Absolute(style.line_height.into()))
        .width(Length::Fill)
        .class(if error {
            InputClass::Error
        } else {
            InputClass::Default
        });

    let mut decorations = Row::new()
        .height(Length::Fixed(height))
        .push(Space::new().width(Length::Fill));

    for element in trailing {
        decorations = decorations.push(
            container(element)
                .center_x(Length::Fixed(height))
                .center_y(Length::Fixed(height)),
        );
    }

    Stack::new()
        .push(input)
        .push(decorations)
        .width(Length::Fill)
        .into()
}

/// A wrapper that keeps the state of a composed field in the widget tree,
/// rebuilds the composition from it and observes the events the native
/// text input does not report, such as Escape, the arrows or the wheel.
#[cfg_attr(not(feature = "svg"), allow(dead_code))]
pub(crate) mod host {
    use iced::advanced::layout::{self, Layout};
    use iced::advanced::widget::operation::{Focusable, Operation, TextInput};
    use iced::advanced::widget::{Id, Tree, Widget, tree};
    use iced::advanced::{Clipboard, Shell, overlay, renderer};
    use iced::{Event, Length, Rectangle, Size, Vector, mouse};

    use crate::{Element, Theme};

    /// A message of the composition: a message of the app, published as it
    /// is, or an own event handed to the host.
    #[derive(Debug, Clone)]
    pub(crate) enum Msg<Message, Own> {
        /// A message of the app.
        App(Message),
        /// An event handled by the host.
        Own(Own),
    }

    /// An element of the composition to focus.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Target {
        /// The text input.
        Input,
        /// The first focusable element that is not the text input.
        Button,
    }

    /// The situation of the composition when an event arrives, before the
    /// composition handles it.
    #[derive(Debug, Clone, Copy)]
    pub(crate) struct Context {
        /// Whether the text input has the focus.
        pub(crate) input_focused: bool,
        /// Whether another element of the composition has the focus.
        pub(crate) other_focused: bool,
        /// The mouse cursor.
        pub(crate) cursor: mouse::Cursor,
        /// The bounds of the whole composition.
        pub(crate) bounds: Rectangle,
        /// The bounds of the text input, when there is one.
        pub(crate) input_bounds: Option<Rectangle>,
        /// Whether the composition captured the event (always false before
        /// it handles it).
        pub(crate) captured: bool,
    }

    /// What the host tells the handler of a composition.
    pub(crate) enum Signal<'e, Own> {
        /// An own event of the composition.
        Own(Own),
        /// The text input gained (true) or lost (false) the focus.
        Focus(bool),
        /// An event, before the composition handles it.
        Before(&'e Event, &'e Context),
        /// An event, after the composition handled it.
        After(&'e Event, &'e Context),
    }

    /// The effects the handler asks the host to apply.
    pub(crate) struct Effects<Message> {
        messages: Vec<Message>,
        capture: bool,
        rebuild: bool,
        focus: Option<Target>,
        select_all: bool,
        hide_cursor: bool,
    }

    impl<Message> Effects<Message> {
        fn new() -> Self {
            Self {
                messages: Vec::new(),
                capture: false,
                rebuild: false,
                focus: None,
                select_all: false,
                hide_cursor: false,
            }
        }

        /// Publishes `message` to the app.
        pub(crate) fn publish(&mut self, message: Message) {
            self.messages.push(message);
        }

        /// Captures the event.
        pub(crate) fn capture(&mut self) {
            self.capture = true;
        }

        /// Rebuilds the composition from the state.
        pub(crate) fn rebuild(&mut self) {
            self.rebuild = true;
        }

        /// Moves the focus to `target` and away from the rest of the
        /// composition.
        pub(crate) fn focus(&mut self, target: Target) {
            self.focus = Some(target);
        }

        /// Selects the whole text of the input after focusing it.
        pub(crate) fn select_all(&mut self) {
            self.select_all = true;
        }

        /// Hands the composition the event without the cursor, during
        /// [`Signal::Before`], so a press ends without a click.
        pub(crate) fn hide_cursor(&mut self) {
            self.hide_cursor = true;
        }
    }

    /// The result of reconciling the state with the current properties.
    pub(crate) struct Reconcile {
        /// Whether the composition must be rebuilt.
        pub(crate) rebuild: bool,
        /// The element to focus after the layout.
        pub(crate) focus: Option<Target>,
    }

    type View<'a, S, Message, Own> = Box<dyn Fn(&S) -> Element<'a, Msg<Message, Own>> + 'a>;
    type Handler<'a, S, Message, Own> =
        Box<dyn Fn(&mut S, Signal<'_, Own>, &mut Effects<Message>) + 'a>;
    type Reconciler<'a, S> = Box<dyn Fn(&mut S, bool) -> Reconcile + 'a>;

    /// Hosts a composition and its state `S`. See the module.
    pub(crate) struct Host<'a, S, Message, Own> {
        content: Element<'a, Msg<Message, Own>>,
        view: Option<View<'a, S, Message, Own>>,
        handler: Handler<'a, S, Message, Own>,
        reconciler: Option<Reconciler<'a, S>>,
        fresh: bool,
    }

    impl<'a, S: Default + 'static, Message: 'a, Own: 'a> Host<'a, S, Message, Own> {
        /// Hosts the composition `view` builds from the state, rebuilt
        /// whenever the handler asks for it.
        pub(crate) fn new(
            view: impl Fn(&S) -> Element<'a, Msg<Message, Own>> + 'a,
            handler: impl Fn(&mut S, Signal<'_, Own>, &mut Effects<Message>) + 'a,
        ) -> Self {
            Self {
                content: view(&S::default()),
                view: Some(Box::new(view)),
                handler: Box::new(handler),
                reconciler: None,
                fresh: true,
            }
        }

        /// Hosts a composition that never changes.
        pub(crate) fn fixed(
            content: Element<'a, Msg<Message, Own>>,
            handler: impl Fn(&mut S, Signal<'_, Own>, &mut Effects<Message>) + 'a,
        ) -> Self {
            Self {
                content,
                view: None,
                handler: Box::new(handler),
                reconciler: None,
                fresh: false,
            }
        }

        /// Reconciles the state with the properties before every layout;
        /// the closure also gets whether the text input has the focus.
        pub(crate) fn reconcile(
            mut self,
            reconciler: impl Fn(&mut S, bool) -> Reconcile + 'a,
        ) -> Self {
            self.reconciler = Some(Box::new(reconciler));
            self
        }
    }

    /// The state of the host in the widget tree.
    struct State<S> {
        user: S,
        input_focused: bool,
        pending: Option<(Target, bool)>,
    }

    /// Finds the text input and the focused elements of a composition.
    #[derive(Default)]
    struct Probe {
        after_input: bool,
        input_focused: bool,
        other_focused: bool,
        input_bounds: Option<Rectangle>,
    }

    impl Operation for Probe {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text_input(&mut self, _id: Option<&Id>, bounds: Rectangle, _state: &mut dyn TextInput) {
            self.after_input = true;
            self.input_bounds.get_or_insert(bounds);
        }

        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            let is_input = std::mem::take(&mut self.after_input);

            if state.is_focused() {
                if is_input {
                    self.input_focused = true;
                } else {
                    self.other_focused = true;
                }
            }
        }
    }

    /// Focuses the target of a composition and unfocuses the rest.
    struct Focus {
        target: Target,
        after_input: bool,
        done: bool,
    }

    impl Operation for Focus {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn TextInput) {
            self.after_input = true;
        }

        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            let is_input = std::mem::take(&mut self.after_input);
            let is_target = match self.target {
                Target::Input => is_input,
                Target::Button => !is_input,
            };

            if is_target && !self.done {
                self.done = true;
                state.focus();
            } else {
                state.unfocus();
            }
        }
    }

    /// Selects the whole text of the first text input.
    #[derive(Default)]
    struct SelectAll {
        done: bool,
    }

    impl Operation for SelectAll {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
            operate(self);
        }

        fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
            if !self.done {
                self.done = true;
                state.select_all();
            }
        }
    }

    /// Runs `operation` on the composition.
    fn run<Message>(
        content: &mut Element<'_, Message>,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        content
            .as_widget_mut()
            .operate(tree, layout, renderer, operation);
    }

    /// Returns what [`Probe`] finds in the composition.
    fn probe<Message>(
        content: &mut Element<'_, Message>,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
    ) -> Probe {
        let mut probe = Probe::default();
        run(content, tree, layout, renderer, &mut probe);
        probe
    }

    /// Focuses `target` in the composition and selects the text of the
    /// input when `select_all` is true.
    fn focus<Message>(
        content: &mut Element<'_, Message>,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        target: Target,
        select_all: bool,
    ) {
        let mut operation = Focus {
            target,
            after_input: false,
            done: false,
        };
        run(content, tree, layout, renderer, &mut operation);

        if select_all {
            run(content, tree, layout, renderer, &mut SelectAll::default());
        }
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

    impl<'a, S, Message, Own> Host<'a, S, Message, Own>
    where
        S: Default + 'static,
        Message: 'a,
        Own: 'a,
    {
        /// Tells the handler that the input focus changed when it did.
        fn track_focus(&self, state: &mut State<S>, focused: bool, effects: &mut Effects<Message>) {
            if focused != state.input_focused {
                state.input_focused = focused;
                (self.handler)(&mut state.user, Signal::Focus(focused), effects);
            }
        }
    }

    impl<'a, S, Message, Own> Widget<Message, Theme, iced::Renderer> for Host<'a, S, Message, Own>
    where
        S: Default + 'static,
        Message: 'a,
        Own: 'a,
    {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<State<S>>()
        }

        fn state(&self) -> tree::State {
            tree::State::new(State {
                user: S::default(),
                input_focused: false,
                pending: None,
            })
        }

        fn children(&self) -> Vec<Tree> {
            vec![Tree::new(self.content.as_widget())]
        }

        fn diff(&self, tree: &mut Tree) {
            match &self.view {
                Some(view) => {
                    let current = view(&tree.state.downcast_ref::<State<S>>().user);
                    tree.diff_children(std::slice::from_ref(&current));
                }
                None => tree.diff_children(std::slice::from_ref(&self.content)),
            }
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
            let Tree {
                state, children, ..
            } = tree;
            let state = state.downcast_mut::<State<S>>();
            let Some(child) = children.first_mut() else {
                return layout::Node::new(Size::ZERO);
            };

            let mut rebuild = std::mem::take(&mut self.fresh);

            if let Some(reconciler) = &self.reconciler {
                let outcome = reconciler(&mut state.user, state.input_focused);
                rebuild |= outcome.rebuild;

                if let Some(target) = outcome.focus {
                    state.pending = Some((target, false));
                }
            }

            if rebuild && let Some(view) = &self.view {
                self.content = view(&state.user);
                child.diff(self.content.as_widget());
            }

            let node = self.content.as_widget_mut().layout(child, renderer, limits);

            if let Some((target, select_all)) = state.pending.take() {
                focus(
                    &mut self.content,
                    child,
                    Layout::new(&node),
                    renderer,
                    target,
                    select_all,
                );
            }

            layout::Node::with_children(node.size(), vec![node])
        }

        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &iced::Renderer,
            operation: &mut dyn Operation,
        ) {
            let Some(content_layout) = layout.children().next() else {
                return;
            };

            operation.container(None, layout.bounds());
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
            let Some(content_layout) = layout.children().next() else {
                return;
            };
            let Tree {
                state, children, ..
            } = tree;
            let state = state.downcast_mut::<State<S>>();
            let Some(child) = children.first_mut() else {
                return;
            };

            let mut effects = Effects::new();

            let before = probe(&mut self.content, child, content_layout, renderer);
            self.track_focus(state, before.input_focused, &mut effects);

            let mut context = Context {
                input_focused: before.input_focused,
                other_focused: before.other_focused,
                cursor,
                bounds: layout.bounds(),
                input_bounds: before.input_bounds,
                captured: false,
            };

            (self.handler)(
                &mut state.user,
                Signal::Before(event, &context),
                &mut effects,
            );

            let content_cursor = if std::mem::take(&mut effects.hide_cursor) {
                mouse::Cursor::Unavailable
            } else {
                cursor
            };

            let mut messages = Vec::new();
            {
                let mut local = Shell::new(&mut messages);

                self.content.as_widget_mut().update(
                    child,
                    event,
                    content_layout,
                    content_cursor,
                    renderer,
                    clipboard,
                    &mut local,
                    viewport,
                );

                context.captured = local.is_event_captured();
                absorb(shell, &local);
            }

            for message in messages {
                match message {
                    Msg::App(message) => shell.publish(message),
                    Msg::Own(own) => {
                        (self.handler)(&mut state.user, Signal::Own(own), &mut effects)
                    }
                }
            }

            (self.handler)(
                &mut state.user,
                Signal::After(event, &context),
                &mut effects,
            );

            let after = probe(&mut self.content, child, content_layout, renderer);
            self.track_focus(state, after.input_focused, &mut effects);

            for message in effects.messages.drain(..) {
                shell.publish(message);
            }

            if effects.capture {
                shell.capture_event();
            }

            let rebuilt = effects.rebuild && self.view.is_some();

            if rebuilt && let Some(view) = &self.view {
                self.content = view(&state.user);
                child.diff(self.content.as_widget());
                shell.invalidate_layout();
                shell.request_redraw();
            }

            if let Some(target) = effects.focus {
                if rebuilt {
                    state.pending = Some((target, effects.select_all));
                } else {
                    focus(
                        &mut self.content,
                        child,
                        content_layout,
                        renderer,
                        target,
                        effects.select_all,
                    );
                    shell.request_redraw();
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
        }

        fn mouse_interaction(
            &self,
            tree: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &iced::Renderer,
        ) -> mouse::Interaction {
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

            self.content
                .as_widget_mut()
                .overlay(
                    &mut tree.children[0],
                    content_layout,
                    renderer,
                    viewport,
                    translation,
                )
                .map(AppOnly::wrap)
        }
    }

    impl<'a, S, Message, Own> From<Host<'a, S, Message, Own>> for Element<'a, Message>
    where
        S: Default + 'static,
        Message: 'a,
        Own: 'a,
    {
        fn from(host: Host<'a, S, Message, Own>) -> Self {
            Element::new(host)
        }
    }

    /// An overlay of the composition, such as a tooltip, that publishes
    /// the app messages of its content and drops its own events.
    struct AppOnly<'b, Message, Own> {
        content: overlay::Element<'b, Msg<Message, Own>, Theme, iced::Renderer>,
    }

    impl<'b, Message: 'b, Own: 'b> AppOnly<'b, Message, Own> {
        /// Wraps `content` in an overlay element of app messages.
        fn wrap(
            content: overlay::Element<'b, Msg<Message, Own>, Theme, iced::Renderer>,
        ) -> overlay::Element<'b, Message, Theme, iced::Renderer> {
            overlay::Element::new(Box::new(Self { content }))
        }
    }

    impl<'b, Message: 'b, Own: 'b> overlay::Overlay<Message, Theme, iced::Renderer>
        for AppOnly<'b, Message, Own>
    {
        fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
            self.content.as_overlay_mut().layout(renderer, bounds)
        }

        fn draw(
            &self,
            renderer: &mut iced::Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
        ) {
            self.content
                .as_overlay()
                .draw(renderer, theme, style, layout, cursor);
        }

        fn operate(
            &mut self,
            layout: Layout<'_>,
            renderer: &iced::Renderer,
            operation: &mut dyn Operation,
        ) {
            self.content
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

                self.content
                    .as_overlay_mut()
                    .update(event, layout, cursor, renderer, clipboard, &mut local);

                absorb(shell, &local);
            }

            for message in messages {
                if let Msg::App(message) = message {
                    shell.publish(message);
                }
            }
        }

        fn mouse_interaction(
            &self,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &iced::Renderer,
        ) -> mouse::Interaction {
            self.content
                .as_overlay()
                .mouse_interaction(layout, cursor, renderer)
        }

        fn overlay<'c>(
            &'c mut self,
            layout: Layout<'c>,
            renderer: &iced::Renderer,
        ) -> Option<overlay::Element<'c, Message, Theme, iced::Renderer>> {
            self.content
                .as_overlay_mut()
                .overlay(layout, renderer)
                .map(AppOnly::wrap)
        }

        fn index(&self) -> f32 {
            self.content.as_overlay().index()
        }
    }
}
