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

//! Tags fields: a field of tags shown as chips, with a text input after
//! them, the chips and the input wrapping onto more rows.
//!
//! Enter or a [delimiter](TagsField::delimiters) adds the typed text,
//! trimmed, as a tag; a pasted list is split on the delimiters. A tag is
//! refused when it is empty, repeated (unless
//! [`TagsField::allow_duplicates`]), past [`TagsField::max_tags`] or
//! rejected by [`TagsField::validator`]. A typed tag that is refused stays
//! in the input to be fixed; the refused tags of a pasted list are dropped.
//! Once [`TagsField::max_tags`] is reached the input takes no more text.
//!
//! Every chip takes part in focus traversal, before the input. Backspace in
//! the empty input, or the Left arrow in the empty input, moves the focus
//! to the last chip; on a chip the Left and Right arrows move to the
//! adjacent chip (Right from the last one returns to the input), and
//! Backspace or Delete removes it, moving the focus to the previous chip,
//! or to the input when it was the first. The "x" of a chip removes it too.
//! A click anywhere else in the field focuses the input.
//!
//! With [`TagsField::suggestions`], the items of a [`ListCollection`] that
//! match the typed text and are not tags yet show in a panel below the
//! input, the typed text highlighted: the Up and Down arrows move the
//! highlighted one, Enter or a click adds it, and Escape closes the panel
//! until the text changes. With [`TagsField::clearable`], a button at the
//! end of the field removes every tag at once.
//!
//! The field looks like a [text field](super::text_field): filled with the
//! high surface container, outlined, at least as tall as the control height
//! of the density, with the thick primary outline while any part of it has
//! the focus. The [`TagsFieldStyle`] picks the look of the chips.
//!
//! The tags live in the application: the field shows `tags` and publishes
//! the whole new list through [`TagsField::on_change`]; without it the
//! field is disabled. The text being typed lives in the field.
//!
//! ```no_run
//! use oito2_iced::widget::tags_field;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Tags(Vec<String>),
//! }
//!
//! # fn view<'a>(tags: &'a [String]) -> oito2_iced::Element<'a, Message> {
//! tags_field::tags_field(tags)
//!     .placeholder("Add tag")
//!     .on_change(Message::Tags)
//!     .into()
//! # }
//! ```

use std::borrow::Cow;
use std::collections::HashSet;

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Paragraph as _, Renderer as _};
use iced::advanced::widget::operation::{Focusable, Operation, TextInput};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::widget::text::{IntoFragment, LineHeight};
use iced::widget::text_input::{self, Catalog as _, Status};
use iced::widget::{Column, Row, Space, button, column, container, scrollable};
use iced::{
    Alignment, Border, Color, Event, Length, Padding, Pixels, Point, Rectangle, Size, Vector, mouse,
};

use super::highlighted_text::highlighted_text;
use super::styled_text;
use crate::collections::ListCollection;
use crate::l10n::Localizations;
use crate::theme::button::Class as ButtonClass;
use crate::theme::text::Class as TextClass;
use crate::theme::text_input::Class as InputClass;
use crate::theme::{HOVER_OPACITY, PRESSED_OPACITY, TRANSPARENT, state_layer};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme, density};

/// The narrowest the text input gets.
pub const MIN_INPUT_WIDTH: f32 = 80.0;

/// The width of the suggestion panel.
pub const OPTIONS_WIDTH: f32 = 280.0;

/// The tallest the suggestion panel grows before it scrolls.
pub const OPTIONS_MAX_HEIGHT: f32 = 240.0;

/// The side of the "x" icon of a chip.
#[cfg(feature = "svg")]
const REMOVE_ICON_SIZE: f32 = 14.0;

/// The radius of the circle behind the "x" of a chip while hovered (the
/// literal of the OITO2 widget).
const REMOVE_HOVER_RADIUS: f32 = 12.0;

/// The opacity of the accent fill of the focused chip of the subtle style
/// (the literal of the OITO2 widget).
const SUBTLE_FOCUS_OPACITY: f32 = 0.12;

/// The width of the suggestion panel outline, kept free so the highlighted
/// option does not cover it.
const OUTLINE_WIDTH: f32 = 1.0;

/// The width the caret takes after the text.
const CARET_WIDTH: f32 = 2.0;

/// The text style of the typed text.
const INPUT_STYLE: typography::TextStyle = typography::BODY_LARGE;

/// The look of the chips of a [`TagsField`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagsFieldStyle {
    /// Chips filled with the secondary container color.
    #[default]
    Filled,
    /// Chips on a neutral fill with medium-weight text; the focused chip
    /// takes a faint accent fill and accent text.
    Subtle,
}

type Suggest<'a> = Box<dyn Fn(&str, &HashSet<String>) -> Vec<String> + 'a>;
type Validator<'a> = Box<dyn Fn(&str) -> bool + 'a>;

/// A field showing `tags` as chips. Without [`TagsField::on_change`] it is
/// disabled.
pub fn tags_field<'a, Message, S>(tags: &[S]) -> TagsField<'a, Message>
where
    S: AsRef<str>,
{
    TagsField {
        tags: tags.iter().map(|tag| tag.as_ref().to_owned()).collect(),
        on_change: None,
        delimiters: vec![",".to_owned()],
        max_tags: None,
        validator: None,
        allow_duplicates: false,
        label: None,
        placeholder: String::new(),
        suggest: None,
        style: TagsFieldStyle::Filled,
        clearable: false,
        localizations: Localizations::en(),
        id: None,
        width: Length::Fill,
    }
}

/// A tags field. See [`tags_field`].
pub struct TagsField<'a, Message> {
    tags: Vec<String>,
    on_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    delimiters: Vec<String>,
    max_tags: Option<usize>,
    validator: Option<Validator<'a>>,
    allow_duplicates: bool,
    label: Option<Cow<'a, str>>,
    placeholder: String,
    suggest: Option<Suggest<'a>>,
    style: TagsFieldStyle,
    clearable: bool,
    localizations: Localizations,
    id: Option<Id>,
    width: Length,
}

impl<'a, Message> TagsField<'a, Message> {
    /// Sets the message produced from the new tag list after tags are added
    /// or removed; without it the field is disabled.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Vec<String>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the strings that end a tag while typing or pasting (a comma by
    /// default); with none, only Enter adds a tag.
    #[must_use]
    pub fn delimiters<D: Into<String>>(mut self, delimiters: impl IntoIterator<Item = D>) -> Self {
        self.delimiters = delimiters
            .into_iter()
            .map(Into::into)
            .filter(|delimiter: &String| !delimiter.is_empty())
            .collect();
        self
    }

    /// Sets the largest number of tags (unlimited by default); once
    /// reached, the input takes no more text.
    #[must_use]
    pub fn max_tags(mut self, max_tags: usize) -> Self {
        self.max_tags = Some(max_tags);
        self
    }

    /// Accepts only the trimmed, non-empty tags for which `validator`
    /// returns true.
    #[must_use]
    pub fn validator(mut self, validator: impl Fn(&str) -> bool + 'a) -> Self {
        self.validator = Some(Box::new(validator));
        self
    }

    /// Sets whether the same tag can be added twice (false by default).
    #[must_use]
    pub fn allow_duplicates(mut self, allow: bool) -> Self {
        self.allow_duplicates = allow;
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Shows `placeholder` in the input while there are no tags.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        placeholder.clone_into(&mut self.placeholder);
        self
    }

    /// Offers the enabled items of `collection` whose label contains the
    /// typed text, ignoring case and accents, and that are not tags yet.
    #[must_use]
    pub fn suggestions<T: 'static>(mut self, collection: &'a ListCollection<T>) -> Self {
        self.suggest = Some(Box::new(move |query, exclude| {
            collection
                .suggestions(query, exclude)
                .into_iter()
                .map(|item| collection.label_of(item))
                .collect()
        }));
        self
    }

    /// Sets the look of the chips ([`TagsFieldStyle::Filled`] by default).
    #[must_use]
    pub fn style(mut self, style: TagsFieldStyle) -> Self {
        self.style = style;
        self
    }

    /// Shows a button at the end of the field that removes every tag while
    /// the field is enabled and has tags.
    #[must_use]
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Takes the tooltips of the "x" of the chips and of the clear button
    /// from `localizations` (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the widget [`Id`] of the text input, used to focus it with
    /// [`iced::widget::operation::focus`] and to find it in tests.
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

    /// Whether the field takes changes.
    fn enabled(&self) -> bool {
        self.on_change.is_some()
    }

    /// Whether `tags` holds as many tags as the field allows.
    fn full(&self, tags: &[String]) -> bool {
        self.max_tags.is_some_and(|max| tags.len() >= max)
    }

    /// Whether `tag` can be appended to `tags` under the rules of the field.
    fn accepts(&self, tag: &str, tags: &[String]) -> bool {
        !tag.is_empty()
            && (self.allow_duplicates || !tags.iter().any(|existing| existing == tag))
            && !self.full(tags)
            && self
                .validator
                .as_ref()
                .is_none_or(|validator| validator(tag))
    }

    /// The suggestions for `text`: none while the text is blank.
    fn options(&self, text: &str) -> Vec<String> {
        let query = text.trim();

        match &self.suggest {
            Some(suggest) if !query.is_empty() => {
                let exclude: HashSet<String> = self.tags.iter().cloned().collect();
                suggest(query, &exclude)
            }
            _ => Vec::new(),
        }
    }
}

/// Splits `text` at every occurrence of any of `delimiters`, trying them in
/// order at each position.
fn split<'t>(text: &'t str, delimiters: &[String]) -> Vec<&'t str> {
    let mut parts = Vec::new();
    let mut start = 0;
    let mut index = 0;

    while index < text.len() {
        let rest = &text[index..];
        let found = delimiters
            .iter()
            .find(|delimiter| rest.starts_with(delimiter.as_str()));

        match found {
            Some(delimiter) => {
                parts.push(&text[start..index]);
                index += delimiter.len();
                start = index;
            }
            None => {
                index += rest.chars().next().map_or(1, char::len_utf8);
            }
        }
    }

    parts.push(&text[start..]);
    parts
}

impl<'a, Message: 'a> From<TagsField<'a, Message>> for Element<'a, Message> {
    fn from(field: TagsField<'a, Message>) -> Self {
        let label = field.label.clone();
        let trailing = trailing(&field);
        let width = field.width;

        let control = Element::new(Field {
            field,
            content: None,
            has_trailing: trailing.is_some(),
            trailing: trailing.unwrap_or_else(|| Space::new().into()),
        });

        match label {
            Some(label) => Column::new()
                .spacing(spacing::XS)
                .width(width)
                .push(styled_text(label, typography::BODY_MEDIUM).class(TextClass::Variant))
                .push(control)
                .into(),
            None => control,
        }
    }
}

/// The clear button of a clearable field with tags.
#[cfg(feature = "svg")]
fn trailing<'a, Message>(field: &TagsField<'a, Message>) -> Option<Element<'a, Inner>> {
    use crate::icon::lucide;

    (field.clearable && field.enabled() && !field.tags.is_empty()).then(|| {
        super::button::icon_button(lucide::X)
            .tooltip(field.localizations.clear)
            .on_press(Inner::Clear)
            .into()
    })
}

/// Without icons there is no clear button.
#[cfg(not(feature = "svg"))]
fn trailing<'a, Message>(_field: &TagsField<'a, Message>) -> Option<Element<'a, Inner>> {
    None
}

/// An event of the composition of a field.
#[derive(Debug, Clone)]
enum Inner {
    /// The text of the input changed.
    Input(String),
    /// Enter was pressed in the input.
    Submit,
    /// The chip at the index asks to be removed.
    Remove(usize),
    /// The chip at the index asks to move the focus back.
    Back(usize),
    /// The chip at the index asks to move the focus forward.
    Forward(usize),
    /// The suggestion at the index was picked.
    Pick(usize),
    /// The clear button was pressed.
    #[cfg_attr(not(feature = "svg"), allow(dead_code))]
    Clear,
}

/// The part of a field that has the focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum Focus {
    /// Nothing of the field.
    #[default]
    None,
    /// The chip at the index.
    Chip(usize),
    /// The text input.
    Input,
    /// The clear button.
    Clear,
}

/// A focus to apply once the field shows a given number of tags.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pending {
    target: Focus,
    count: usize,
}

/// What the composition of a field shows, besides the properties.
#[derive(Debug, Clone, PartialEq)]
struct Key {
    text: String,
    width: f32,
    open: bool,
    highlight: usize,
}

/// The state of a field in the widget tree.
#[derive(Debug, Default)]
struct State {
    /// The typed text.
    text: String,
    /// The part of the field with the focus, as last read.
    focus: Focus,
    /// The width of the input for the text, as last measured.
    width: f32,
    /// Whether Escape or Tab closed the panel for the current text.
    hidden: bool,
    /// The index of the highlighted suggestion.
    highlight: usize,
    /// The tags of the application, as last seen.
    seen: Option<Vec<String>>,
    /// The tags last published while the application shows older ones.
    published: Option<Vec<String>>,
    /// The focus to apply once the tags are updated.
    pending: Option<Pending>,
}

impl State {
    /// The tags new changes start from: the last published ones until the
    /// application shows new tags.
    fn base<'s>(&'s self, tags: &'s [String]) -> &'s [String] {
        self.published.as_deref().unwrap_or(tags)
    }

    /// Replaces the typed text, resetting the panel when it changes.
    fn set_text(&mut self, text: String) {
        if text != self.text {
            self.text = text;
            self.hidden = false;
            self.highlight = 0;
        }
    }
}

/// The effects of an event of the composition.
#[derive(Default)]
struct Effects {
    /// The focus to apply to the current composition.
    focus: Option<Focus>,
}

/// The behaviour of a field, shared by the field and its overlays.
impl<Message> TagsField<'_, Message> {
    /// Adds the accepted tags of `raw` and returns the refused texts.
    fn add<'r>(
        &self,
        state: &mut State,
        raw: impl IntoIterator<Item = &'r str>,
        shell: &mut Shell<'_, Message>,
    ) -> Vec<String> {
        let mut tags = state.base(&self.tags).to_vec();
        let before = tags.len();
        let mut refused = Vec::new();

        for text in raw {
            let tag = text.trim();

            if !self.enabled() {
                refused.push(tag.to_owned());
            } else if self.accepts(tag, &tags) {
                tags.push(tag.to_owned());
            } else if !tag.is_empty() {
                refused.push(tag.to_owned());
            }
        }

        if tags.len() > before {
            self.publish(state, tags, Focus::Input, shell);
        }

        refused
    }

    /// Publishes `tags` and focuses `target` once the application shows
    /// them.
    fn publish(
        &self,
        state: &mut State,
        tags: Vec<String>,
        target: Focus,
        shell: &mut Shell<'_, Message>,
    ) {
        let Some(on_change) = &self.on_change else {
            return;
        };

        state.pending = Some(Pending {
            target,
            count: tags.len(),
        });
        shell.publish(on_change(tags.clone()));
        state.published = Some(tags);
        shell.invalidate_layout();
        shell.request_redraw();
    }

    /// Turns every delimited part of `text` into a tag, keeping the text
    /// after the last delimiter in the input.
    fn input(&self, state: &mut State, text: String, shell: &mut Shell<'_, Message>) {
        if !self.enabled() || self.full(state.base(&self.tags)) {
            return;
        }

        if self.delimiters.is_empty() {
            state.set_text(text);
            return;
        }

        let parts = split(&text, &self.delimiters);

        let [complete @ .., last] = parts.as_slice() else {
            return;
        };

        if complete.is_empty() {
            state.set_text(text);
            return;
        }

        let refused = self.add(state, complete.iter().copied(), shell);

        // A single tag refused as its delimiter is typed at the end stays
        // to be fixed, as with Enter; the refused parts of pasted text are
        // dropped, leaving the text after the last delimiter.
        let kept = match refused.as_slice() {
            [single] if complete.len() == 1 && last.is_empty() => single.clone(),
            _ => (*last).to_owned(),
        };

        state.set_text(kept);
    }

    /// Adds the suggestion at `index` of the suggestions for the text.
    fn pick(&self, state: &mut State, index: usize, shell: &mut Shell<'_, Message>) {
        let options = self.options(&state.text);

        if let Some(option) = options.get(index) {
            let _ = self.add(state, [option.as_str()], shell);
            state.set_text(String::new());
            state.pending.get_or_insert(Pending {
                target: Focus::Input,
                count: state.base(&self.tags).len(),
            });
            shell.invalidate_layout();
            shell.request_redraw();
        }
    }

    /// Handles an event of the composition.
    fn handle(
        &self,
        state: &mut State,
        inner: Inner,
        effects: &mut Effects,
        shell: &mut Shell<'_, Message>,
    ) {
        let count = state.base(&self.tags).len();

        match inner {
            Inner::Input(text) => self.input(state, text, shell),
            Inner::Submit => {
                if !state.text.is_empty() {
                    let text = state.text.clone();

                    if self.add(state, [text.as_str()], shell).is_empty() {
                        state.set_text(String::new());
                    }
                }
            }
            Inner::Remove(index) if self.enabled() && index < count => {
                let mut tags = state.base(&self.tags).to_vec();
                tags.remove(index);
                let target = match index.checked_sub(1) {
                    Some(previous) => Focus::Chip(previous),
                    None => Focus::Input,
                };
                self.publish(state, tags, target, shell);
            }
            Inner::Remove(_) => {}
            Inner::Back(index) => {
                if let Some(previous) = index.checked_sub(1) {
                    effects.focus = Some(Focus::Chip(previous));
                }
            }
            Inner::Forward(index) => {
                effects.focus = Some(if index + 1 < count {
                    Focus::Chip(index + 1)
                } else {
                    Focus::Input
                });
            }
            Inner::Pick(index) => self.pick(state, index, shell),
            Inner::Clear => {
                state.set_text(String::new());
                self.publish(state, Vec::new(), Focus::Input, shell);
            }
        }
    }

    /// Whether the suggestion panel shows for `state`.
    fn open(&self, state: &State) -> bool {
        self.enabled()
            && state.focus == Focus::Input
            && !state.hidden
            && !self.options(&state.text).is_empty()
    }

    /// What the composition shows for `state`.
    fn key(&self, state: &State) -> Key {
        let count = self.options(&state.text).len();

        Key {
            text: state.text.clone(),
            width: state.width,
            open: self.open(state),
            highlight: state.highlight.min(count.saturating_sub(1)),
        }
    }
}

/// The style of the text input: the text field style without its fill and
/// outline, which the field draws.
fn bare_input(theme: &Theme, status: Status) -> text_input::Style {
    text_input::Style {
        background: TRANSPARENT.into(),
        border: Border::default(),
        ..theme.style(&InputClass::Default, status)
    }
}

/// Builds the composition of `field` for `key`: the chips and the input in
/// wrapping rows.
fn view<'a, Message: 'a>(field: &TagsField<'a, Message>, key: &Key) -> Element<'a, Inner> {
    let enabled = field.enabled();

    let mut children: Vec<Element<'a, Inner>> = field
        .tags
        .iter()
        .enumerate()
        .map(|(index, tag)| chip(field, index, tag))
        .collect();

    let placeholder = if field.tags.is_empty() {
        field.placeholder.as_str()
    } else {
        ""
    };

    let mut input = text_input::TextInput::new(placeholder, &key.text)
        .padding(0)
        .size(INPUT_STYLE.size)
        .line_height(LineHeight::Absolute(Pixels(INPUT_STYLE.line_height)))
        .width(Length::Fixed(key.width))
        .class(InputClass::Custom(Box::new(bare_input)));

    if enabled {
        input = input.on_input(Inner::Input).on_submit(Inner::Submit);
    }

    if let Some(id) = &field.id {
        input = input.id(id.clone());
    }

    let input: Element<'a, Inner> = if field.suggest.is_some() {
        let options = field.options(&key.text);
        let panel = options_panel(&options, &key.text, key.highlight);

        crate::overlay::anchored(input, panel)
            .menu()
            .padding(Padding::new(OUTLINE_WIDTH))
            .max_width(OPTIONS_WIDTH)
            .max_height(OPTIONS_MAX_HEIGHT)
            .open(key.open)
            .into()
    } else {
        input.into()
    };

    let input = if enabled {
        input
    } else {
        super::controls::exclude_focus(input)
    };

    children.push(
        container(input)
            .padding([0.0, spacing::XS])
            .center_y(Length::Shrink)
            .into(),
    );

    Row::with_children(children)
        .spacing(spacing::XS)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .wrap()
        .vertical_spacing(spacing::XS)
        .into()
}

/// The chip of the tag at `index`.
fn chip<'a, Message>(
    field: &TagsField<'a, Message>,
    index: usize,
    tag: &str,
) -> Element<'a, Inner> {
    let enabled = field.enabled();
    let text_style = match field.style {
        TagsFieldStyle::Filled => typography::BODY_MEDIUM,
        TagsFieldStyle::Subtle => typography::TextStyle {
            weight: iced::font::Weight::Medium,
            ..typography::BODY_MEDIUM
        },
    };

    Element::new(Chip {
        label: styled_text(tag.to_owned(), text_style).into(),
        remove: remove_button(enabled, field.localizations.remove_item(tag)),
        index,
        style: field.style,
        enabled,
    })
}

/// The "x" of a chip with its tooltip, when the field is enabled.
#[cfg(feature = "svg")]
fn remove_button<'a>(enabled: bool, label: String) -> Option<Element<'a, Inner>> {
    use crate::icon::{icon, lucide};

    enabled.then(|| {
        super::tooltip(
            container(icon(lucide::X).size(REMOVE_ICON_SIZE)).padding(spacing::XS),
            label,
        )
        .into()
    })
}

/// Without icons the chips have no "x"; Backspace and Delete remove them.
#[cfg(not(feature = "svg"))]
fn remove_button<'a>(_enabled: bool, _label: String) -> Option<Element<'a, Inner>> {
    None
}

/// The list of suggestions, with `query` highlighted in each label and the
/// one at `highlight` drawn selected.
fn options_panel<'a>(labels: &[String], query: &str, highlight: usize) -> Element<'a, Inner> {
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

/// The style of a suggestion: the secondary container while highlighted,
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

/// Reads which part of a composition has the focus: the chips come first
/// among its focusable widgets, then the text input.
#[derive(Default)]
struct Probe {
    after_input: bool,
    chips: usize,
    focus: Focus,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, _state: &mut dyn TextInput) {
        self.after_input = true;
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        let is_input = std::mem::take(&mut self.after_input);

        if state.is_focused() {
            self.focus = if is_input {
                Focus::Input
            } else {
                Focus::Chip(self.chips)
            };
        }

        if !is_input {
            self.chips += 1;
        }
    }
}

/// Focuses one part of a composition and unfocuses the rest; with
/// `Focus::None` it unfocuses everything.
struct FocusOn {
    target: Focus,
    after_input: bool,
    chips: usize,
}

impl FocusOn {
    fn new(target: Focus) -> Self {
        Self {
            target,
            after_input: false,
            chips: 0,
        }
    }
}

impl Operation for FocusOn {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
        self.after_input = true;

        if self.target == Focus::Input {
            state.move_cursor_to_end();
        }
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        let is_input = std::mem::take(&mut self.after_input);
        let is_target = if is_input {
            self.target == Focus::Input
        } else {
            let index = self.chips;
            self.chips += 1;
            self.target == Focus::Chip(index) || self.target == Focus::Clear
        };

        if is_target {
            state.focus();
        } else {
            state.unfocus();
        }
    }
}

/// Moves the caret of the text input to the end.
struct CaretToEnd;

impl Operation for CaretToEnd {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
        state.move_cursor_to_end();
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

/// The widget of a field: draws the field, hosts the chips, the input and
/// the clear button, and applies the rules of the field to their events.
struct Field<'a, Message> {
    field: TagsField<'a, Message>,
    content: Option<(Key, Element<'a, Inner>)>,
    trailing: Element<'a, Inner>,
    has_trailing: bool,
}

/// The parts of a laid out field.
struct Parts<'l> {
    content: Layout<'l>,
    trailing: Layout<'l>,
}

/// Returns the layouts of the content and the clear button of a field.
fn parts(layout: Layout<'_>) -> Option<Parts<'_>> {
    let mut children = layout.children();

    Some(Parts {
        content: children.next()?,
        trailing: children.next()?,
    })
}

impl<'a, Message: 'a> Field<'a, Message> {
    /// Builds the composition for `key` unless it shows it already.
    fn ensure(&mut self, key: Key, tree: &mut Tree) {
        let stale = self.content.as_ref().is_none_or(|(built, _)| *built != key);

        if stale {
            let content = view(&self.field, &key);
            tree.diff(&content);
            self.content = Some((key, content));
        }
    }

    /// Reads the focus of the content and the clear button.
    fn probe(
        &mut self,
        children: &mut [Tree],
        parts: &Parts<'_>,
        renderer: &iced::Renderer,
    ) -> Focus {
        let [content_tree, trailing_tree] = children else {
            return Focus::None;
        };

        let mut probe = Probe::default();

        if let Some((_, content)) = &mut self.content {
            content
                .as_widget_mut()
                .operate(content_tree, parts.content, renderer, &mut probe);
        }

        if probe.focus != Focus::None {
            return probe.focus;
        }

        let mut trailing = Probe::default();
        self.trailing.as_widget_mut().operate(
            trailing_tree,
            parts.trailing,
            renderer,
            &mut trailing,
        );

        if trailing.focus == Focus::None {
            Focus::None
        } else {
            Focus::Clear
        }
    }

    /// Focuses `target` and unfocuses the rest of the field.
    fn focus(
        &mut self,
        children: &mut [Tree],
        parts: &Parts<'_>,
        renderer: &iced::Renderer,
        target: Focus,
    ) {
        let [content_tree, trailing_tree] = children else {
            return;
        };

        let in_content = if target == Focus::Clear {
            Focus::None
        } else {
            target
        };

        if let Some((_, content)) = &mut self.content {
            content.as_widget_mut().operate(
                content_tree,
                parts.content,
                renderer,
                &mut FocusOn::new(in_content),
            );
        }

        let in_trailing = if target == Focus::Clear {
            Focus::Clear
        } else {
            Focus::None
        };

        self.trailing.as_widget_mut().operate(
            trailing_tree,
            parts.trailing,
            renderer,
            &mut FocusOn::new(in_trailing),
        );
    }

    /// Lays out the content and the clear button in `width`.
    fn arrange(
        &mut self,
        children: &mut [Tree],
        renderer: &iced::Renderer,
        width: f32,
    ) -> layout::Node {
        let height = density::control_height();
        let vertical = ((height - INPUT_STYLE.line_height) / 2.0).max(0.0);
        let right = if self.has_trailing {
            height
        } else {
            spacing::SM
        };
        let inner = (width - spacing::SM - right).max(0.0);

        let [content_tree, trailing_tree] = children else {
            return layout::Node::new(Size::new(width, height));
        };

        let content = match &mut self.content {
            Some((_, content)) => content.as_widget_mut().layout(
                content_tree,
                renderer,
                &layout::Limits::new(Size::ZERO, Size::new(inner, f32::INFINITY)),
            ),
            None => layout::Node::new(Size::ZERO),
        };

        let content_height = content.size().height;
        let total = (content_height + 2.0 * vertical).max(height);
        let content = content.move_to(Point::new(spacing::SM, (total - content_height) / 2.0));

        let trailing = self.trailing.as_widget_mut().layout(
            trailing_tree,
            renderer,
            &layout::Limits::new(Size::ZERO, Size::new(height, height)),
        );
        let trailing_size = trailing.size();
        let trailing = trailing.move_to(Point::new(
            width - height + (height - trailing_size.width) / 2.0,
            (total - trailing_size.height) / 2.0,
        ));

        layout::Node::with_children(Size::new(width, total), vec![content, trailing])
    }

    /// The width of the input for the typed text, or the placeholder while
    /// it shows, between [`MIN_INPUT_WIDTH`] and `available`.
    fn input_width(&self, state: &State, renderer: &iced::Renderer, available: f32) -> f32 {
        let shown = if state.text.is_empty() && self.field.tags.is_empty() {
            self.field.placeholder.as_str()
        } else {
            state.text.as_str()
        };

        let measured = measure::<iced::Renderer>(shown, renderer.default_font());

        let width = (measured + CARET_WIDTH).ceil();

        width
            .max(MIN_INPUT_WIDTH)
            .min(available.max(MIN_INPUT_WIDTH))
    }
}

/// The width of `content` in the text style of the input and `font`.
fn measure<R: text::Renderer>(content: &str, font: R::Font) -> f32 {
    R::Paragraph::with_text(text::Text {
        content,
        bounds: Size::INFINITE,
        size: Pixels(INPUT_STYLE.size),
        line_height: LineHeight::Absolute(Pixels(INPUT_STYLE.line_height)),
        font,
        align_x: text::Alignment::Left,
        align_y: iced::alignment::Vertical::Top,
        shaping: text::Shaping::Advanced,
        wrapping: text::Wrapping::None,
    })
    .min_width()
}

impl<'a, Message: 'a> Widget<Message, Theme, iced::Renderer> for Field<'a, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            width: MIN_INPUT_WIDTH,
            ..State::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        let key = self.field.key(&State {
            width: MIN_INPUT_WIDTH,
            ..State::default()
        });

        vec![
            Tree::new(view(&self.field, &key)),
            Tree::new(&self.trailing),
        ]
    }

    fn diff(&self, tree: &mut Tree) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_ref::<State>();
        let content = view(&self.field, &self.field.key(state));

        if let [content_tree, trailing_tree] = children.as_mut_slice() {
            content_tree.diff(&content);
            trailing_tree.diff(&self.trailing);
        } else {
            *children = vec![Tree::new(content), Tree::new(&self.trailing)];
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(self.field.width, Length::Shrink)
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

        // New tags of the application replace the published ones; a focus
        // waiting for other tags is dropped.
        if state.seen.as_ref() != Some(&self.field.tags) {
            let first = state.seen.is_none();
            state.seen = Some(self.field.tags.clone());
            state.published = None;

            if !first
                && state
                    .pending
                    .is_some_and(|pending| pending.count != self.field.tags.len())
            {
                state.pending = None;
            }
        }

        let width = limits.width(self.field.width).max().width;
        let right = if self.has_trailing {
            density::control_height()
        } else {
            spacing::SM
        };
        let available = width - spacing::SM - right - 2.0 * spacing::XS;
        state.width = self.input_width(state, renderer, available);

        let Some(content_tree) = children.first_mut() else {
            return layout::Node::new(Size::new(width, density::control_height()));
        };
        self.ensure(self.field.key(state), content_tree);

        let mut node = self.arrange(children, renderer, width);

        if let Some(pending) = state.pending
            && pending.count == self.field.tags.len()
        {
            state.pending = None;

            if let Some(parts) = parts(Layout::new(&node)) {
                self.focus(children, &parts, renderer, pending.target);
                state.focus = self.probe(children, &parts, renderer);
            }

            let key = self.field.key(state);

            if self
                .content
                .as_ref()
                .is_some_and(|(built, _)| *built != key)
                && let Some(content_tree) = children.first_mut()
            {
                self.ensure(key, content_tree);
                node = self.arrange(children, renderer, width);
            }
        }

        node
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let Some(parts) = parts(layout) else {
            return;
        };
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            if let ([content_tree, trailing_tree], Some((_, content))) =
                (children.as_mut_slice(), &mut self.content)
            {
                content
                    .as_widget_mut()
                    .operate(content_tree, parts.content, renderer, operation);
                self.trailing.as_widget_mut().operate(
                    trailing_tree,
                    parts.trailing,
                    renderer,
                    operation,
                );
            }
        });

        state.focus = self.probe(children, &parts, renderer);
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
        let Some(parts) = parts(layout) else {
            return;
        };
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();

        state.focus = self.probe(children, &parts, renderer);
        let before = self.field.key(state);
        let mut effects = Effects::default();
        let mut handled = false;
        let mut typed = false;

        if let Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(named),
            modifiers,
            ..
        }) = event
            && state.focus == Focus::Input
            && self.field.enabled()
        {
            let count = self.field.options(&state.text).len();
            let open = self.field.open(state);
            let tags = self.field.tags.len();

            match named {
                key::Named::ArrowDown | key::Named::ArrowUp if open => {
                    state.highlight = if *named == key::Named::ArrowDown {
                        (before.highlight + 1) % count
                    } else {
                        (before.highlight + count - 1) % count
                    };
                    handled = true;
                }
                key::Named::Enter if open => {
                    self.field.pick(state, before.highlight, shell);
                    handled = true;
                }
                key::Named::Escape if open => {
                    state.hidden = true;
                    handled = true;
                }
                key::Named::Tab if open => {
                    state.hidden = true;
                }
                key::Named::Backspace | key::Named::ArrowLeft
                    if state.text.is_empty() && tags > 0 && modifiers.is_empty() =>
                {
                    effects.focus = Some(Focus::Chip(tags - 1));
                    handled = true;
                }
                _ => {}
            }
        }

        if handled {
            shell.capture_event();
        } else {
            let mut messages = Vec::new();
            {
                let mut local = Shell::new(&mut messages);

                if let ([content_tree, trailing_tree], Some((_, content))) =
                    (children.as_mut_slice(), &mut self.content)
                {
                    content.as_widget_mut().update(
                        content_tree,
                        event,
                        parts.content,
                        cursor,
                        renderer,
                        clipboard,
                        &mut local,
                        viewport,
                    );
                    self.trailing.as_widget_mut().update(
                        trailing_tree,
                        event,
                        parts.trailing,
                        cursor,
                        renderer,
                        clipboard,
                        &mut local,
                        viewport,
                    );
                }

                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event
                    && !local.is_event_captured()
                    && self.field.enabled()
                    && cursor.is_over(layout.bounds())
                {
                    effects.focus = Some(Focus::Input);
                    local.capture_event();
                }

                absorb(shell, &local);
            }

            for message in messages {
                typed |= matches!(message, Inner::Input(_));
                self.field.handle(state, message, &mut effects, shell);
            }
        }

        if let Some(target) = effects.focus {
            self.focus(children, &parts, renderer, target);
            shell.request_redraw();
        }

        let focus = self.probe(children, &parts, renderer);

        if focus != state.focus {
            state.focus = focus;
            shell.request_redraw();
        }

        let after = self.field.key(state);

        if after != before
            || self
                .content
                .as_ref()
                .is_some_and(|(built, _)| *built != after)
        {
            if let Some(content_tree) = children.first_mut() {
                self.ensure(after, content_tree);
            }

            if state.focus == Focus::Input
                && let Some((_, content)) = &mut self.content
                && let Some(content_tree) = children.first_mut()
            {
                content.as_widget_mut().operate(
                    content_tree,
                    parts.content,
                    renderer,
                    &mut CaretToEnd,
                );
            }

            shell.invalidate_layout();
            shell.request_redraw();
        } else if typed && let Some(content_tree) = children.first_mut() {
            // The input shows the text of the field, not the text it took
            // when the field refused it.
            let content = view(&self.field, &after);
            content_tree.diff(&content);
            self.content = Some((after, content));
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
        let Some(parts) = parts(layout) else {
            return;
        };
        let state = tree.state.downcast_ref::<State>();

        let status = if !self.field.enabled() {
            Status::Disabled
        } else if state.focus == Focus::None {
            Status::Active
        } else {
            Status::Focused { is_hovered: false }
        };
        let frame = theme.style(&InputClass::Default, status);

        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border: frame.border,
                ..renderer::Quad::default()
            },
            frame.background,
        );

        if let ([content_tree, trailing_tree], Some((_, content))) =
            (tree.children.as_slice(), &self.content)
        {
            content.as_widget().draw(
                content_tree,
                renderer,
                theme,
                style,
                parts.content,
                cursor,
                viewport,
            );
            self.trailing.as_widget().draw(
                trailing_tree,
                renderer,
                theme,
                style,
                parts.trailing,
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
        let Some(parts) = parts(layout) else {
            return mouse::Interaction::None;
        };

        let inner = match (tree.children.as_slice(), &self.content) {
            ([content_tree, trailing_tree], Some((_, content))) => content
                .as_widget()
                .mouse_interaction(content_tree, parts.content, cursor, viewport, renderer)
                .max(self.trailing.as_widget().mouse_interaction(
                    trailing_tree,
                    parts.trailing,
                    cursor,
                    viewport,
                    renderer,
                )),
            _ => mouse::Interaction::None,
        };

        if inner == mouse::Interaction::None
            && self.field.enabled()
            && cursor.is_over(layout.bounds())
        {
            mouse::Interaction::Text
        } else {
            inner
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
        let parts = parts(layout)?;
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let [content_tree, trailing_tree] = children.as_mut_slice() else {
            return None;
        };
        let Self {
            field,
            content,
            trailing,
            ..
        } = self;
        let (_, content) = content.as_mut()?;
        let field = &*field;

        let mut overlays = Vec::new();

        if let Some(inner) = content.as_widget_mut().overlay(
            content_tree,
            parts.content,
            renderer,
            viewport,
            translation,
        ) {
            overlays.push(overlay::Element::new(Box::new(Overlay {
                inner,
                field,
                state: Some(state),
            })));
        }

        if let Some(inner) = trailing.as_widget_mut().overlay(
            trailing_tree,
            parts.trailing,
            renderer,
            viewport,
            translation,
        ) {
            overlays.push(overlay::Element::new(Box::new(Overlay {
                inner,
                field,
                state: None,
            })));
        }

        match overlays.len() {
            0 => None,
            1 => overlays.pop(),
            _ => Some(overlay::Group::with_children(overlays).overlay()),
        }
    }
}

/// An overlay of a field, such as the suggestion panel or a tooltip: adds
/// the picked suggestions and drops the other events of its content.
struct Overlay<'b, 'a, Message> {
    inner: overlay::Element<'b, Inner, Theme, iced::Renderer>,
    field: &'b TagsField<'a, Message>,
    state: Option<&'b mut State>,
}

impl<Message> overlay::Overlay<Message, Theme, iced::Renderer> for Overlay<'_, '_, Message> {
    fn layout(&mut self, renderer: &iced::Renderer, bounds: Size) -> layout::Node {
        self.inner.as_overlay_mut().layout(renderer, bounds)
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
            if let (Inner::Pick(index), Some(state)) = (message, self.state.as_deref_mut()) {
                self.field.pick(state, index, shell);
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

        Some(overlay::Element::new(Box::new(Overlay {
            inner,
            field: self.field,
            state: self.state.as_deref_mut(),
        })))
    }

    fn index(&self) -> f32 {
        self.inner.as_overlay().index()
    }
}

/// The chip of a tag: a label and, while the field is enabled, an "x" that
/// removes it. It takes the keyboard focus and handles the keys of a chip.
struct Chip<'a> {
    label: Element<'a, Inner>,
    remove: Option<Element<'a, Inner>>,
    index: usize,
    style: TagsFieldStyle,
    enabled: bool,
}

/// The state of a chip in the widget tree.
#[derive(Debug, Clone, Copy, Default)]
struct ChipState {
    focused: bool,
    /// Whether the left button was pressed on the "x".
    pressed: bool,
}

impl Focusable for ChipState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl Chip<'_> {
    /// The fill and the content color of the chip.
    fn colors(&self, theme: &Theme, focused: bool) -> (Color, Color) {
        let roles = theme.roles();

        match self.style {
            TagsFieldStyle::Filled => (roles.secondary_container, roles.on_secondary_container),
            TagsFieldStyle::Subtle if focused => (
                roles
                    .semantic
                    .primary_text
                    .scale_alpha(SUBTLE_FOCUS_OPACITY),
                roles.semantic.primary_text,
            ),
            TagsFieldStyle::Subtle => (roles.surface_container_highest, roles.on_surface),
        }
    }
}

impl Widget<Inner, Theme, iced::Renderer> for Chip<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<ChipState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(ChipState::default())
    }

    fn children(&self) -> Vec<Tree> {
        let mut children = vec![Tree::new(&self.label)];
        children.extend(self.remove.iter().map(Tree::new));
        children
    }

    fn diff(&self, tree: &mut Tree) {
        match &self.remove {
            Some(remove) => tree.diff_children(&[&self.label, remove]),
            None => tree.diff_children(std::slice::from_ref(&self.label)),
        }
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
        let loose = layout::Limits::new(Size::ZERO, limits.max());
        let label = self
            .label
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, &loose);
        let remove = match (&mut self.remove, tree.children.get_mut(1)) {
            (Some(remove), Some(remove_tree)) => {
                Some(remove.as_widget_mut().layout(remove_tree, renderer, &loose))
            }
            _ => None,
        };

        let label_size = label.size();
        let remove_size = remove.as_ref().map_or(Size::ZERO, layout::Node::size);
        let height = label_size.height.max(remove_size.height);
        let end = if remove.is_some() {
            spacing::XXS
        } else {
            spacing::SM
        };
        let width = spacing::SM + label_size.width + remove_size.width + end;

        let mut children =
            vec![label.move_to(Point::new(spacing::SM, (height - label_size.height) / 2.0))];

        if let Some(remove) = remove {
            children.push(remove.move_to(Point::new(
                spacing::SM + label_size.width,
                (height - remove_size.height) / 2.0,
            )));
        }

        layout::Node::with_children(Size::new(width, height), children)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<ChipState>();

        if self.enabled {
            operation.focusable(None, layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(None, layout.bounds());
        }

        operation.traverse(&mut |operation| {
            let mut layouts = layout.children();

            if let Some(label_layout) = layouts.next() {
                self.label.as_widget_mut().operate(
                    &mut tree.children[0],
                    label_layout,
                    renderer,
                    operation,
                );
            }

            if let (Some(remove), Some(remove_layout), Some(remove_tree)) =
                (&mut self.remove, layouts.next(), tree.children.get_mut(1))
            {
                remove
                    .as_widget_mut()
                    .operate(remove_tree, remove_layout, renderer, operation);
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
        shell: &mut Shell<'_, Inner>,
        viewport: &Rectangle,
    ) {
        let remove_layout = layout.children().nth(1);

        if let (Some(remove), Some(remove_layout), Some(remove_tree)) =
            (&mut self.remove, remove_layout, tree.children.get_mut(1))
        {
            remove.as_widget_mut().update(
                remove_tree,
                event,
                remove_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let state = tree.state.downcast_mut::<ChipState>();
        let over_remove = self.enabled
            && remove_layout.is_some_and(|remove_layout| cursor.is_over(remove_layout.bounds()));

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if over_remove => {
                state.pressed = true;
                shell.capture_event();
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) if state.pressed => {
                state.pressed = false;

                if over_remove {
                    shell.publish(Inner::Remove(self.index));
                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if state.focused && self.enabled && modifiers.is_empty() => {
                let inner = match named {
                    key::Named::Backspace | key::Named::Delete => Some(Inner::Remove(self.index)),
                    key::Named::ArrowLeft => Some(Inner::Back(self.index)),
                    key::Named::ArrowRight => Some(Inner::Forward(self.index)),
                    _ => None,
                };

                if let Some(inner) = inner {
                    shell.publish(inner);
                    shell.capture_event();
                }
            }
            _ => {}
        }

        if let Event::Mouse(mouse::Event::CursorMoved { .. }) = event
            && self.enabled
        {
            shell.request_redraw();
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
        let state = tree.state.downcast_ref::<ChipState>();
        let bounds = layout.bounds();
        let (fill, foreground) = self.colors(theme, state.focused && self.enabled);

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border::default().rounded(radius::SMALL),
                ..renderer::Quad::default()
            },
            fill,
        );

        let style = renderer::Style {
            text_color: foreground,
        };
        let mut layouts = layout.children();

        if let Some(label_layout) = layouts.next() {
            self.label.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                &style,
                label_layout,
                cursor,
                viewport,
            );
        }

        if let (Some(remove), Some(remove_layout), Some(remove_tree)) =
            (&self.remove, layouts.next(), tree.children.get(1))
        {
            let area = remove_layout.bounds();

            if self.enabled && cursor.is_over(area) {
                let side = 2.0 * REMOVE_HOVER_RADIUS;
                let center = area.center();

                renderer.fill_quad(
                    renderer::Quad {
                        bounds: Rectangle {
                            x: center.x - REMOVE_HOVER_RADIUS,
                            y: center.y - REMOVE_HOVER_RADIUS,
                            width: side,
                            height: side,
                        },
                        border: Border::default().rounded(REMOVE_HOVER_RADIUS),
                        ..renderer::Quad::default()
                    },
                    foreground.scale_alpha(if state.pressed {
                        PRESSED_OPACITY
                    } else {
                        HOVER_OPACITY
                    }),
                );
            }

            remove.as_widget().draw(
                remove_tree,
                renderer,
                theme,
                &style,
                remove_layout,
                cursor,
                viewport,
            );
        }

        if self.enabled && state.focused {
            crate::focus::draw_ring(renderer, theme, bounds, radius::SMALL);
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let over_remove = self.enabled
            && layout
                .children()
                .nth(1)
                .is_some_and(|remove| cursor.is_over(remove.bounds()));

        if over_remove {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::None
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Inner, Theme, iced::Renderer>> {
        let remove_layout = layout.children().nth(1)?;
        let remove_tree = tree.children.get_mut(1)?;

        self.remove.as_mut()?.as_widget_mut().overlay(
            remove_tree,
            remove_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delimiters(list: &[&str]) -> Vec<String> {
        list.iter()
            .map(|delimiter| (*delimiter).to_owned())
            .collect()
    }

    #[test]
    fn splits_on_every_delimiter_in_order() {
        assert_eq!(split("a,b;c", &delimiters(&[",", ";"])), ["a", "b", "c"]);
        assert_eq!(split("dart,", &delimiters(&[","])), ["dart", ""]);
        assert_eq!(split("go", &delimiters(&[","])), ["go"]);
        assert_eq!(split("a, b", &delimiters(&[", ", ","])), ["a", "b"]);
        assert_eq!(split("é,ü", &delimiters(&[","])), ["é", "ü"]);
    }

    #[test]
    fn accepts_follows_the_rules_of_the_field() {
        let field = tags_field::<(), &str>(&[])
            .max_tags(2)
            .validator(|tag| tag != "bad");
        let tags = vec!["dart".to_owned()];

        assert!(field.accepts("go", &tags));
        assert!(!field.accepts("", &tags));
        assert!(!field.accepts("dart", &tags), "repeats are refused");
        assert!(!field.accepts("bad", &tags), "the validator refuses");
        assert!(
            !field.accepts("go", &["a".to_owned(), "b".to_owned()]),
            "full"
        );
        assert!(
            tags_field::<(), &str>(&[])
                .allow_duplicates(true)
                .accepts("dart", &tags)
        );
    }
}
