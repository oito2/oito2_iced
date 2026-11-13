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

//! Text fields: a single-line input with an optional label above it,
//! leading icon, trailing element, and helper or error text below it.
//!
//! A field is as tall as the control height of the current
//! density (see [`density`]), the height of the buttons, with or without
//! icons; the leading icon and the trailing element sit in square boxes as
//! wide as the field is tall. The input is filled with the high surface
//! container and outlined; the outline thickens in the text-safe primary
//! tone while focused and turns to the text-safe error tone with an error.
//!
//! ```no_run
//! use oito2_iced::widget::text_field;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Name(String),
//! }
//!
//! # fn view<'a>(name: &'a str) -> oito2_iced::Element<'a, Message> {
//! text_field::text_field("Your name", name)
//!     .label("Name")
//!     .helper("As shown to other people")
//!     .on_input(Message::Name)
//!     .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::widget::Id;
use iced::widget::text::{IntoFragment, LineHeight};
use iced::widget::{Column, Row, Space, Stack, container, text_input};
use iced::{Length, Padding};

use super::styled_text;
use crate::theme::text::Class as TextClass;
use crate::theme::text_input::Class;
use crate::tokens::{spacing, typography};
use crate::{Element, density};

#[cfg(feature = "svg")]
use crate::icon::{Icon, icon};

/// The text style of the value: body large.
const VALUE_STYLE: typography::TextStyle = typography::BODY_LARGE;

/// The vertical padding that centers one line of the value in the control
/// height of the current density, net of the outline.
fn vertical_padding() -> f32 {
    ((density::control_height() - VALUE_STYLE.line_height) / 2.0).max(0.0)
}

/// A single-line text field showing `value`, with `placeholder` while it
/// is empty. Without [`TextField::on_input`] it is disabled.
pub fn text_field<'a, Message: Clone + 'a>(
    placeholder: &str,
    value: &str,
) -> TextField<'a, Message> {
    TextField {
        input: text_input(placeholder, value),
        enabled: false,
        label: None,
        helper: None,
        error: None,
        #[cfg(feature = "svg")]
        leading: None,
        trailing: None,
        width: Length::Fill,
    }
}

/// A text field. See [`text_field`].
pub struct TextField<'a, Message> {
    input: text_input::TextInput<'a, Message, crate::Theme>,
    enabled: bool,
    label: Option<Cow<'a, str>>,
    helper: Option<Cow<'a, str>>,
    error: Option<Cow<'a, str>>,
    #[cfg(feature = "svg")]
    leading: Option<Icon>,
    trailing: Option<Element<'a, Message>>,
    width: Length,
}

impl<'a, Message: Clone + 'a> TextField<'a, Message> {
    /// Sets the message produced from the new text on every edit; without
    /// it the field is disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.input = self.input.on_input(on_input);
        self.enabled = true;
        self
    }

    /// Sets the edit message, or disables the field with `None`.
    #[must_use]
    pub fn on_input_maybe(mut self, on_input: Option<impl Fn(String) -> Message + 'a>) -> Self {
        self.enabled = on_input.is_some();
        self.input = self.input.on_input_maybe(on_input);
        self
    }

    /// Sets the message published on Enter.
    #[must_use]
    pub fn on_submit(mut self, message: Message) -> Self {
        self.input = self.input.on_submit(message);
        self
    }

    /// Sets the message produced from pasted text, instead of
    /// [`TextField::on_input`].
    #[must_use]
    pub fn on_paste(mut self, on_paste: impl Fn(String) -> Message + 'a) -> Self {
        self.input = self.input.on_paste(on_paste);
        self
    }

    /// Hides the text behind bullets, for passwords.
    #[must_use]
    pub fn secure(mut self, secure: bool) -> Self {
        self.input = self.input.secure(secure);
        self
    }

    /// Sets the widget [`Id`] of the input, used to focus it with
    /// [`iced::widget::operation::focus`] and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.input = self.input.id(id);
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Shows `helper` below the field while it has no error.
    #[must_use]
    pub fn helper(mut self, helper: impl IntoFragment<'a>) -> Self {
        self.helper = Some(helper.into_fragment());
        self
    }

    /// Shows `error` below the field, in place of the helper, and outlines
    /// the field in the error tone. `None` clears it.
    #[must_use]
    pub fn error(mut self, error: Option<impl IntoFragment<'a>>) -> Self {
        self.error = error.map(IntoFragment::into_fragment);
        self
    }

    /// Places `icon` in a box at the start of the field.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn leading(mut self, icon: Icon) -> Self {
        self.leading = Some(icon);
        self
    }

    /// Places `element`, such as an icon button, in a box at the end of the
    /// field.
    #[must_use]
    pub fn trailing(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(element.into());
        self
    }

    /// Sets the width (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<TextField<'a, Message>> for Element<'a, Message> {
    fn from(field: TextField<'a, Message>) -> Self {
        let height = density::control_height();
        let vertical = vertical_padding();

        #[cfg(feature = "svg")]
        let has_leading = field.leading.is_some();
        #[cfg(not(feature = "svg"))]
        let has_leading = false;

        let padding = Padding {
            top: vertical,
            bottom: vertical,
            left: if has_leading { height } else { spacing::MD },
            right: if field.trailing.is_some() {
                height
            } else {
                spacing::MD
            },
        };

        let input = field
            .input
            .padding(padding)
            .size(VALUE_STYLE.size)
            .line_height(LineHeight::Absolute(VALUE_STYLE.line_height.into()))
            .width(Length::Fill)
            .class(if field.error.is_some() {
                Class::Error
            } else {
                Class::Default
            });

        let boxed = |content: Element<'a, Message>| -> Element<'a, Message> {
            container(content)
                .center_x(Length::Fixed(height))
                .center_y(Length::Fixed(height))
                .into()
        };

        let mut decorations = Row::new().height(Length::Fixed(height));

        #[cfg(feature = "svg")]
        if let Some(leading) = field.leading {
            decorations = decorations.push(boxed(
                container(icon(leading))
                    .class(crate::theme::container::Class::Custom(Box::new(
                        |theme: &crate::Theme| {
                            container::Style::default().color(theme.roles().on_surface_variant)
                        },
                    )))
                    .into(),
            ));
        }

        decorations = decorations.push(Space::new().width(Length::Fill));

        if let Some(trailing) = field.trailing {
            decorations = decorations.push(boxed(trailing));
        }

        // The native input stays focusable while disabled; a disabled field
        // is kept out of focus traversal.
        let input: Element<'a, Message> = if field.enabled {
            input.into()
        } else {
            super::controls::exclude_focus(input)
        };

        let control = Stack::new()
            .push(input)
            .push(decorations)
            .width(Length::Fill);

        let mut column = Column::new().spacing(spacing::XS).width(field.width);

        if let Some(label) = field.label {
            column =
                column.push(styled_text(label, typography::BODY_MEDIUM).class(TextClass::Variant));
        }

        column = column.push(control);

        let below = match (field.error, field.helper) {
            (Some(error), _) => {
                Some(styled_text(error, typography::BODY_SMALL).class(TextClass::Error))
            }
            (None, Some(helper)) => {
                Some(styled_text(helper, typography::BODY_SMALL).class(TextClass::Variant))
            }
            (None, None) => None,
        };

        if let Some(below) = below {
            column = column.push(container(below).padding([0.0, spacing::MD]));
        }

        column.into()
    }
}
