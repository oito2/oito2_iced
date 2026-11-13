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

//! Editable labels: a text label that turns into a field to rename it in
//! place.
//!
//! A double click, Enter or F2 on the focused label starts editing with
//! the whole text selected. Enter or leaving the field keeps the change,
//! publishing the trimmed text when it changed and is not empty; Escape
//! restores the text. Enter and Escape return the focus to the label.
//! Without [`EditableLabel::on_submit`] the label is read-only and skipped
//! by focus traversal.
//!
//! The text belongs to the app; whether the label is being edited and the
//! text being typed live in the label, so the app only hears about the
//! final text.
//!
//! ```no_run
//! use oito2_iced::widget::editable_label;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Rename(String),
//! }
//!
//! # fn view<'a>(name: &'a str) -> oito2_iced::Element<'a, Message> {
//! editable_label::editable_label(name)
//!     .on_submit(Message::Rename)
//!     .into()
//! # }
//! ```

use std::rc::Rc;

use iced::advanced::widget::Id;
use iced::keyboard::{self, key};
use iced::widget::text::LineHeight;
use iced::widget::{container, mouse_area, text_input};
use iced::{Event, Length, mouse};

use super::search::host::{Effects, Host, Msg, Signal, Target};
use super::{focusable, styled_text};
use crate::Element;
use crate::theme::text::Class as TextClass;
use crate::theme::text_input::Class as InputClass;
use crate::tokens::typography::{self, TextStyle};
use crate::tokens::{radius, spacing};

/// A label showing `text`, read-only until [`EditableLabel::on_submit`]
/// is set.
pub fn editable_label<'a, Message: Clone + 'a>(text: &str) -> EditableLabel<'a, Message> {
    EditableLabel {
        text: text.to_owned(),
        placeholder: String::new(),
        style: typography::BODY_LARGE,
        on_submit: None,
        id: None,
        width: Length::Fill,
    }
}

/// An editable label. See [`editable_label`].
pub struct EditableLabel<'a, Message> {
    text: String,
    placeholder: String,
    style: TextStyle,
    on_submit: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    id: Option<Id>,
    width: Length,
}

impl<'a, Message: Clone + 'a> EditableLabel<'a, Message> {
    /// Sets the message produced from the new, trimmed text after a change;
    /// without it the label is read-only.
    #[must_use]
    pub fn on_submit(mut self, on_submit: impl Fn(String) -> Message + 'a) -> Self {
        self.on_submit = Some(Rc::new(on_submit));
        self
    }

    /// Shows `placeholder` in the variant color while the text is empty.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        placeholder.clone_into(&mut self.placeholder);
        self
    }

    /// Sets the text style (body large by default).
    #[must_use]
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the widget [`Id`] of the label and of its field while editing.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width of the label and of the field (the available width
    /// by default). The label text shows at the start of that width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

/// The own events of an editable label.
#[derive(Debug, Clone)]
enum Label {
    /// The label was double clicked.
    Start,
    /// The text was edited.
    Input(String),
    /// Enter was pressed in the field.
    Submit,
}

/// The state of an editable label.
#[derive(Debug, Default)]
struct LabelState {
    /// Whether the field shows.
    editing: bool,
    /// The text in the field.
    draft: String,
}

/// Returns the trimmed `draft` when it is not empty and differs from
/// `text`: the text to publish when editing ends with the change kept.
fn submitted(draft: &str, text: &str) -> Option<String> {
    let trimmed = draft.trim();

    (!trimmed.is_empty() && trimmed != text).then(|| trimmed.to_owned())
}

/// Returns whether `event` is a press of `named` without modifiers.
fn is_key(event: &Event, named: key::Named) -> bool {
    matches!(
        event,
        Event::Keyboard(keyboard::Event::KeyPressed {
            key: keyboard::Key::Named(pressed),
            modifiers,
            ..
        }) if *pressed == named && modifiers.is_empty()
    )
}

impl<'a, Message: Clone + 'a> From<EditableLabel<'a, Message>> for Element<'a, Message> {
    fn from(label: EditableLabel<'a, Message>) -> Self {
        let EditableLabel {
            text,
            placeholder,
            style,
            on_submit,
            id,
            width,
        } = label;

        let editable = on_submit.is_some();
        let shown_text = text.clone();

        let view = move |state: &LabelState| -> Element<'a, Msg<Message, Label>> {
            if state.editing {
                let mut input = text_input("", &state.draft)
                    .on_input(|text| Msg::Own(Label::Input(text)))
                    .on_submit(Msg::Own(Label::Submit))
                    .padding(spacing::XS)
                    .size(style.size)
                    .line_height(LineHeight::Absolute(style.line_height.into()))
                    .font(crate::font::inter(style.weight))
                    .width(width)
                    .class(InputClass::Default);

                if let Some(id) = &id {
                    input = input.id(id.clone());
                }

                return input.into();
            }

            let empty = shown_text.is_empty();
            let content = styled_text(
                if empty {
                    placeholder.clone()
                } else {
                    shown_text.clone()
                },
                style,
            )
            .class(if empty {
                TextClass::Variant
            } else {
                TextClass::Default
            });

            let mut wrapper = focusable(container(content).padding(spacing::XS))
                .radius(radius::SMALL)
                .enabled(editable);

            if let Some(id) = &id {
                wrapper = wrapper.id(id.clone());
            }

            let label: Element<'a, Msg<Message, Label>> = if editable {
                mouse_area(wrapper)
                    .on_double_click(Msg::Own(Label::Start))
                    .interaction(mouse::Interaction::Text)
                    .into()
            } else {
                wrapper.into()
            };

            container(label).width(width).into()
        };

        let finish_text = text.clone();

        let start = move |state: &mut LabelState, effects: &mut Effects<Message>| {
            if editable && !state.editing {
                state.editing = true;
                state.draft.clone_from(&text);
                effects.rebuild();
                effects.focus(Target::Input);
                effects.select_all();
            }
        };

        let finish = move |state: &mut LabelState,
                           keep: bool,
                           refocus: bool,
                           effects: &mut Effects<Message>| {
            if !state.editing {
                return;
            }

            state.editing = false;
            effects.rebuild();

            if keep
                && let Some(changed) = submitted(&state.draft, &finish_text)
                && let Some(on_submit) = &on_submit
            {
                effects.publish(on_submit(changed));
            }

            if refocus {
                effects.focus(Target::Button);
            }
        };

        Host::new(
            view,
            move |state: &mut LabelState,
                  signal: Signal<'_, Label>,
                  effects: &mut Effects<Message>| {
                match signal {
                    Signal::Own(Label::Start) => start(state, effects),
                    Signal::Own(Label::Input(draft)) => {
                        state.draft = draft;
                        effects.rebuild();
                    }
                    Signal::Own(Label::Submit) => finish(state, true, true, effects),
                    Signal::Focus(false) => finish(state, true, false, effects),
                    Signal::After(event, context)
                        if state.editing
                            && context.input_focused
                            && is_key(event, key::Named::Escape) =>
                    {
                        finish(state, false, true, effects);
                        effects.capture();
                    }
                    Signal::After(event, context)
                        if !state.editing
                            && context.other_focused
                            && (is_key(event, key::Named::Enter)
                                || is_key(event, key::Named::F2)) =>
                    {
                        start(state, effects);
                        effects.capture();
                    }
                    Signal::Focus(true) | Signal::Before(..) | Signal::After(..) => {}
                }
            },
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::submitted;

    #[test]
    fn only_trimmed_non_empty_changes_are_submitted() {
        assert_eq!(submitted("  Ideas ", "Notes"), Some("Ideas".to_owned()));
        assert_eq!(submitted("Notes ", "Notes"), None);
        assert_eq!(submitted("   ", "Notes"), None);
    }
}
