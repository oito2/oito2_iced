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

//! Password fields: a [text field](super::text_field) that hides its text
//! behind bullets, with a button that shows or hides it and a warning
//! below the focused field while Caps Lock is on.
//!
//! Whether the text shows lives in the field. Iced reports no lock keys,
//! so the field follows Caps Lock from the key presses it sees: a press of
//! Caps Lock toggles it, starting from off, and every typed letter tells
//! it again, as a capital typed without Shift (or a small letter typed
//! with it) means Caps Lock is on.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::widget::password;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Password(String),
//!     LogIn,
//! }
//!
//! # fn view<'a>(password: &'a str) -> oito2_iced::Element<'a, Message> {
//! password::password_field(&Localizations::en(), password)
//!     .label("Password")
//!     .on_input(Message::Password)
//!     .on_submit(Message::LogIn)
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
use iced::widget::text::IntoFragment;
#[cfg(feature = "svg")]
use iced::widget::{Column, container, row};
#[cfg(feature = "svg")]
use iced::{Alignment, Event, Length};

#[cfg(feature = "svg")]
use super::search::host::{Effects, Host, Msg, Reconcile, Signal};
#[cfg(feature = "svg")]
use super::{button, styled_text, text_field};
#[cfg(feature = "svg")]
use crate::icon::{icon, lucide};
#[cfg(feature = "svg")]
use crate::l10n::Localizations;
#[cfg(feature = "svg")]
use crate::tokens::{spacing, typography};
#[cfg(feature = "svg")]
use crate::{Element, Theme};

/// The size of the warning icon next to the Caps Lock warning.
pub const WARNING_ICON_SIZE: f32 = 14.0;

/// A password field showing `value` hidden, with the localized tooltips
/// and Caps Lock warning of `localizations`. Without
/// [`PasswordField::on_input`] it is disabled.
#[cfg(feature = "svg")]
pub fn password_field<'a, Message: Clone + 'a>(
    localizations: &Localizations,
    value: &str,
) -> PasswordField<'a, Message> {
    PasswordField {
        value: value.to_owned(),
        placeholder: String::new(),
        label: None,
        error: None,
        show_label: localizations.show_password,
        hide_label: localizations.hide_password,
        caps_lock_label: localizations.caps_lock_on,
        caps_lock_warning: true,
        on_input: None,
        on_submit: None,
        id: None,
        width: Length::Fill,
    }
}

/// A password field. See [`password_field`].
#[cfg(feature = "svg")]
pub struct PasswordField<'a, Message> {
    value: String,
    placeholder: String,
    label: Option<Cow<'a, str>>,
    error: Option<Cow<'a, str>>,
    show_label: &'static str,
    hide_label: &'static str,
    caps_lock_label: &'static str,
    caps_lock_warning: bool,
    on_input: Option<Rc<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    id: Option<Id>,
    width: Length,
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> PasswordField<'a, Message> {
    /// Sets the message produced from the new text on every edit; without
    /// it the field and its button are disabled.
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

    /// Shows `placeholder` while the field is empty.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        placeholder.clone_into(&mut self.placeholder);
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Shows `error` below the field and outlines it in the error tone.
    /// `None` clears it.
    #[must_use]
    pub fn error(mut self, error: Option<impl IntoFragment<'a>>) -> Self {
        self.error = error.map(IntoFragment::into_fragment);
        self
    }

    /// Sets whether the warning shows below the focused field while Caps
    /// Lock is on (true by default).
    #[must_use]
    pub fn caps_lock_warning(mut self, warning: bool) -> Self {
        self.caps_lock_warning = warning;
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

/// The own events of a password field.
#[cfg(feature = "svg")]
#[derive(Debug, Clone)]
enum Password {
    /// The text was edited.
    Input(String),
    /// The show or hide button was pressed.
    Reveal,
}

/// The state of a password field.
#[cfg(feature = "svg")]
#[derive(Debug, Default)]
struct PasswordState {
    /// Whether the text shows.
    visible: bool,
    /// Whether Caps Lock is on, as far as the key presses tell.
    caps_lock: bool,
    /// Whether the input has the focus.
    focused: bool,
    /// The text last typed, until the app shows a new value.
    typed: Option<String>,
    /// The value of the app the field last showed.
    shown: Option<String>,
}

/// Returns whether Caps Lock is on after a key press, or `None` when the
/// press does not tell: Caps Lock itself toggles `current`; a letter with
/// distinct cases tells it from its case and Shift.
#[cfg(feature = "svg")]
fn caps_lock_after(
    current: bool,
    key: &keyboard::Key,
    text: Option<&str>,
    modifiers: keyboard::Modifiers,
) -> Option<bool> {
    if matches!(key, keyboard::Key::Named(key::Named::CapsLock)) {
        return Some(!current);
    }

    let mut chars = text?.chars();
    let letter = chars.next()?;

    if chars.next().is_some() || letter.is_uppercase() == letter.is_lowercase() {
        return None;
    }

    Some(letter.is_uppercase() != modifiers.shift())
}

/// The Caps Lock warning: a warning icon and `label` in the warning tone.
#[cfg(feature = "svg")]
fn warning<'a, M: 'a>(label: &'static str) -> Element<'a, M> {
    container(
        row![
            icon(lucide::TRIANGLE_EXCLAMATION_POINT).size(WARNING_ICON_SIZE),
            styled_text(label, typography::BODY_SMALL),
        ]
        .spacing(spacing::XS)
        .align_y(Alignment::Center),
    )
    .padding([0.0, spacing::MD])
    .class(crate::theme::container::Class::Custom(Box::new(
        |theme: &Theme| container::Style::default().color(theme.roles().semantic.warning),
    )))
    .into()
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> From<PasswordField<'a, Message>> for Element<'a, Message> {
    fn from(field: PasswordField<'a, Message>) -> Self {
        let PasswordField {
            value,
            placeholder,
            label,
            error,
            show_label,
            hide_label,
            caps_lock_label,
            caps_lock_warning,
            on_input,
            on_submit,
            id,
            width,
        } = field;

        let enabled = on_input.is_some();
        let has_error = error.is_some();
        let app_value = value.clone();

        let view = move |state: &PasswordState| -> Element<'a, Msg<Message, Password>> {
            let text = state.typed.as_deref().unwrap_or(&value);

            let reveal = button::icon_button(if state.visible {
                lucide::EYE_OFF
            } else {
                lucide::EYE
            })
            .tooltip(if state.visible {
                hide_label
            } else {
                show_label
            })
            .on_press_maybe(enabled.then_some(Msg::Own(Password::Reveal)));

            let mut input = text_field::text_field(&placeholder, text)
                .secure(!state.visible)
                .error(error.clone())
                .trailing(reveal);

            if enabled {
                input = input.on_input(|text| Msg::Own(Password::Input(text)));
            }

            if let Some(message) = &on_submit {
                input = input.on_submit(Msg::App(message.clone()));
            }

            if let Some(label) = &label {
                input = input.label(label.clone());
            }

            if let Some(id) = &id {
                input = input.id(id.clone());
            }

            let mut column = Column::new()
                .push(input)
                .spacing(if has_error { spacing::XXS } else { spacing::XS })
                .width(width);

            if caps_lock_warning && state.focused && state.caps_lock {
                column = column.push(warning(caps_lock_label));
            }

            column.into()
        };

        Host::new(
            view,
            move |state: &mut PasswordState,
                  signal: Signal<'_, Password>,
                  effects: &mut Effects<Message>| {
                match signal {
                    Signal::Own(Password::Input(text)) => {
                        if let Some(on_input) = &on_input {
                            effects.publish(on_input(text.clone()));
                        }

                        state.typed = Some(text);
                        effects.rebuild();
                    }
                    Signal::Own(Password::Reveal) => {
                        state.visible = !state.visible;
                        effects.rebuild();
                    }
                    Signal::Focus(focused) => {
                        state.focused = focused;
                        effects.rebuild();
                    }
                    Signal::After(
                        Event::Keyboard(keyboard::Event::KeyPressed {
                            key,
                            text,
                            modifiers,
                            ..
                        }),
                        _,
                    ) => {
                        if let Some(on) =
                            caps_lock_after(state.caps_lock, key, text.as_deref(), *modifiers)
                            && on != state.caps_lock
                        {
                            state.caps_lock = on;
                            effects.rebuild();
                        }
                    }
                    Signal::Before(..) | Signal::After(..) => {}
                }
            },
        )
        .reconcile(move |state: &mut PasswordState, _| {
            let changed = state.shown.as_deref() != Some(app_value.as_str());

            if changed {
                state.shown = Some(app_value.clone());
            }

            Reconcile {
                rebuild: changed && state.typed.take().is_some(),
                focus: None,
            }
        })
        .into()
    }
}

#[cfg(all(test, feature = "svg"))]
mod tests {
    use super::*;

    fn letter(c: &str, modifiers: keyboard::Modifiers, current: bool) -> Option<bool> {
        caps_lock_after(
            current,
            &keyboard::Key::Character(c.into()),
            Some(c),
            modifiers,
        )
    }

    #[test]
    fn caps_lock_toggles_and_letters_tell_it() {
        let caps = keyboard::Key::Named(key::Named::CapsLock);
        let none = keyboard::Modifiers::empty();

        assert_eq!(caps_lock_after(false, &caps, None, none), Some(true));
        assert_eq!(caps_lock_after(true, &caps, None, none), Some(false));
        assert_eq!(letter("A", none, false), Some(true));
        assert_eq!(letter("a", none, true), Some(false));
        assert_eq!(letter("a", keyboard::Modifiers::SHIFT, false), Some(true));
        assert_eq!(letter("A", keyboard::Modifiers::SHIFT, true), Some(false));
        assert_eq!(letter("1", none, true), None);
    }
}
