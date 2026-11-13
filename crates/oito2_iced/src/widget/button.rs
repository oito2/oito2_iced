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

//! Buttons: the Material text buttons, the floating action button and the
//! icon buttons, each with keyboard focus.
//!
//! Every button is at least as tall as the control height of the current
//! [density](crate::density) and grows when its label needs more room. It
//! takes part in focus traversal while it has a press message, shows the
//! focus ring while focused and publishes the press message on Enter and
//! Space as well as on a click.
//!
//! ```no_run
//! use oito2_iced::widget::button;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//!     Cancel,
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! iced::widget::row![
//!     button::text("Cancel").on_press(Message::Cancel),
//!     button::filled("Save").on_press(Message::Save),
//! ]
//! .spacing(8)
//! .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::widget::Id;
use iced::widget::text::IntoFragment;
use iced::widget::{container, row};
use iced::{Alignment, Length, Padding};

use super::min_size::{Minimum, min_size};
use super::{focusable, styled_text, tooltip};
use crate::Element;
#[cfg(feature = "svg")]
use crate::icon::{Icon, icon, lucide};
#[cfg(feature = "svg")]
use crate::l10n::Localizations;
use crate::tokens::{radius, spacing, typography};
#[cfg(feature = "svg")]
use crate::widget::helpers::{ReadingDirection, Side};

pub use crate::theme::button::Class;

/// Minimum width of a text button, padding included.
pub const MIN_WIDTH: f32 = 64.0;

/// Side of a floating action button.
pub const FAB_SIZE: f32 = 56.0;

/// A button of the OITO2 catalog. Built by [`button`], [`filled`],
/// [`tonal`], [`elevated`], [`outlined`], [`text`], [`danger`] and [`fab`].
pub struct Button<'a, Message> {
    content: Element<'a, Message>,
    #[cfg(feature = "svg")]
    icon: Option<Icon>,
    class: Class<'a>,
    on_press: Option<Message>,
    id: Option<Id>,
    width: Length,
    padding: Padding,
    min_width: Minimum,
    min_height: Minimum,
    ring_radius: f32,
    tooltip: Option<Cow<'a, str>>,
}

/// A filled button showing any `content`, laid out like a text button.
pub fn button<'a, Message>(content: impl Into<Element<'a, Message>>) -> Button<'a, Message> {
    Button {
        content: content.into(),
        #[cfg(feature = "svg")]
        icon: None,
        class: Class::Filled,
        on_press: None,
        id: None,
        width: Length::Shrink,
        padding: Padding::from([0.0, spacing::LG]),
        min_width: Minimum::Fixed(MIN_WIDTH - 2.0 * spacing::LG),
        min_height: Minimum::Control(0.0),
        ring_radius: radius::MEDIUM,
        tooltip: None,
    }
}

/// A button with `label` in the label style and `class` colors.
fn labeled<'a, Message: 'a>(label: impl IntoFragment<'a>, class: Class<'a>) -> Button<'a, Message> {
    button(styled_text(label, typography::LABEL_LARGE)).class(class)
}

/// A filled button: the main action of a view.
pub fn filled<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    labeled(label, Class::Filled)
}

/// A tonal button, on the secondary container: a secondary action that
/// still stands out.
pub fn tonal<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    labeled(label, Class::Tonal)
}

/// An elevated button: the neutral desktop button, a raised surface that
/// gains a shadow on hover.
pub fn elevated<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    labeled(label, Class::Elevated)
}

/// An outlined button: a medium-emphasis action.
pub fn outlined<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    labeled(label, Class::Outlined)
}

/// A text button: a low-emphasis action, with narrower padding.
pub fn text<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    let button = labeled(label, Class::Text);

    Button {
        padding: Padding::from([0.0, spacing::MD]),
        min_width: Minimum::Fixed(MIN_WIDTH - 2.0 * spacing::MD),
        ..button
    }
}

/// A danger button, on the error color: a destructive action.
pub fn danger<'a, Message: 'a>(label: impl IntoFragment<'a>) -> Button<'a, Message> {
    labeled(label, Class::Danger)
}

/// A floating action button: `icon` on the primary container, 56 pixels
/// square with the large radius and a low shadow. [`Button::label`] turns
/// it into an extended one.
#[cfg(feature = "svg")]
pub fn fab<'a, Message: 'a>(icon: Icon) -> Button<'a, Message> {
    Button {
        content: self::icon(icon).into(),
        #[cfg(feature = "svg")]
        icon: None,
        class: Class::Fab,
        on_press: None,
        id: None,
        width: Length::Shrink,
        padding: Padding::ZERO,
        min_width: Minimum::Fixed(FAB_SIZE),
        min_height: Minimum::Fixed(FAB_SIZE),
        ring_radius: radius::LARGE,
        tooltip: None,
    }
}

impl<'a, Message> Button<'a, Message> {
    /// Sets the style class.
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the message published on a click, Enter or Space. Without it
    /// the button is disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or disables the button with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets the widget [`Id`], used to focus the button programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width; the content is centered in it.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the padding around the content.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Shows `label` below the button while the pointer rests on it.
    #[must_use]
    pub fn tooltip(mut self, label: impl IntoFragment<'a>) -> Self {
        self.tooltip = Some(label.into_fragment());
        self
    }

    /// Places `icon` before the content, at the icon size of the density.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Places `label` after the content: on a floating action button, makes
    /// it an extended one with the label after its icon.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self
    where
        Message: 'a,
    {
        self.content = row![self.content, styled_text(label, typography::LABEL_LARGE)]
            .spacing(spacing::SM)
            .align_y(Alignment::Center)
            .into();

        if matches!(self.class, Class::Fab) {
            self.padding = Padding::from([0.0, spacing::LG]);
        }

        self
    }
}

/// Wraps a button content in the focus, tooltip and minimum size every
/// button of the catalog shares.
#[allow(clippy::too_many_arguments)]
fn finish<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    class: Class<'a>,
    on_press: Option<Message>,
    id: Option<Id>,
    width: Length,
    padding: Padding,
    (min_width, min_height): (Minimum, Minimum),
    ring_radius: f32,
    tooltip: Option<Cow<'a, str>>,
) -> Element<'a, Message> {
    let content = min_size(content, min_width, min_height);

    let content: Element<'a, Message> = if width == Length::Shrink {
        content.into()
    } else {
        container(content).center_x(Length::Fill).into()
    };

    let native = iced::widget::Button::new(content)
        .class(class)
        .padding(padding)
        .width(width)
        .on_press_maybe(on_press.clone());

    let mut wrapper = focusable(native)
        .radius(ring_radius)
        .enabled(on_press.is_some());

    if let Some(message) = on_press {
        wrapper = wrapper.on_activate(message);
    }

    if let Some(id) = id {
        wrapper = wrapper.id(id);
    }

    match tooltip {
        Some(label) => self::tooltip(wrapper, label).into(),
        None => wrapper.into(),
    }
}

impl<'a, Message: Clone + 'a> From<Button<'a, Message>> for Element<'a, Message> {
    fn from(button: Button<'a, Message>) -> Self {
        #[cfg(feature = "svg")]
        let content = match button.icon {
            Some(leading) => row![icon(leading), button.content]
                .spacing(spacing::SM)
                .align_y(Alignment::Center)
                .into(),
            None => button.content,
        };
        #[cfg(not(feature = "svg"))]
        let content = button.content;

        finish(
            content,
            button.class,
            button.on_press,
            button.id,
            button.width,
            button.padding,
            (button.min_width, button.min_height),
            button.ring_radius,
            button.tooltip,
        )
    }
}

/// A square icon button as tall as the control height of the density. With
/// [`IconButton::selected`] it is a toggle: the selected state fills it with
/// the secondary container and shows [`IconButton::selected_icon`] when
/// given.
#[cfg(feature = "svg")]
pub fn icon_button<'a, Message>(icon: Icon) -> IconButton<'a, Message> {
    IconButton {
        icon,
        selected_icon: None,
        selected: false,
        on_press: None,
        id: None,
        tooltip: None,
    }
}

/// An icon button that goes back: a chevron pointing to the start of the
/// reading `direction`, with the localized "Back" tooltip.
#[cfg(feature = "svg")]
pub fn back_button<'a, Message>(
    localizations: &Localizations,
    direction: ReadingDirection,
) -> IconButton<'a, Message> {
    let chevron = match direction.chevron(false) {
        Side::Left => lucide::CHEVRON_LEFT,
        Side::Right => lucide::CHEVRON_RIGHT,
    };

    icon_button(chevron).tooltip(localizations.material.back)
}

/// An icon button that closes: an "x" with the localized "Close" tooltip.
#[cfg(feature = "svg")]
pub fn close_button<'a, Message>(localizations: &Localizations) -> IconButton<'a, Message> {
    icon_button(lucide::X).tooltip(localizations.close)
}

/// A square icon button. See [`icon_button`].
#[cfg(feature = "svg")]
pub struct IconButton<'a, Message> {
    icon: Icon,
    selected_icon: Option<Icon>,
    selected: bool,
    on_press: Option<Message>,
    id: Option<Id>,
    tooltip: Option<Cow<'a, str>>,
}

#[cfg(feature = "svg")]
impl<'a, Message> IconButton<'a, Message> {
    /// Sets the message published on a click, Enter or Space. Without it
    /// the button is disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or disables the button with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Makes the button a toggle in the given state.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the icon shown while the button is selected.
    #[must_use]
    pub fn selected_icon(mut self, icon: Icon) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    /// Sets the widget [`Id`].
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Shows `label` below the button while the pointer rests on it.
    #[must_use]
    pub fn tooltip(mut self, label: impl IntoFragment<'a>) -> Self {
        self.tooltip = Some(label.into_fragment());
        self
    }
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> From<IconButton<'a, Message>> for Element<'a, Message> {
    fn from(button: IconButton<'a, Message>) -> Self {
        let selected = button.selected && button.on_press.is_some();

        let (glyph, class) = if selected {
            (
                button.selected_icon.unwrap_or(button.icon),
                Class::IconSelected,
            )
        } else {
            (button.icon, Class::Icon)
        };

        finish(
            icon(glyph).into(),
            class,
            button.on_press,
            button.id,
            Length::Shrink,
            Padding::ZERO,
            (Minimum::Control(0.0), Minimum::Control(0.0)),
            radius::MEDIUM,
            button.tooltip,
        )
    }
}
