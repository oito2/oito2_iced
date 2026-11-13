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

//! Toggle buttons with a clickable label, and option buttons.
//!
//! A toggle button is a compact row with a control and a title (and an
//! optional subtitle), as desktop check and switch buttons: a click
//! anywhere on the row, or Enter or Space while it has the focus, toggles
//! it. The row is a single focus stop with the focus ring around itself;
//! the control inside never takes the focus.
//!
//! ```no_run
//! use oito2_iced::widget::{controls, toggles};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Hidden(Option<bool>),
//!     Dark(bool),
//!     Add,
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! iced::widget::column![
//!     controls::check_button(
//!         toggles::checkbox(false).on_change(Message::Hidden),
//!         "Show hidden files",
//!     ),
//!     controls::switch_button(toggles::switch(true).on_toggle(Message::Dark), "Dark style"),
//!     controls::option_button("+").label("Add").on_press(Message::Add),
//! ]
//! .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Id, Operation, Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::widget::text::{self, Fragment, IntoFragment};
use iced::widget::{Column, Row, container};
use iced::{Alignment, Border, Color, Event, Length, Padding, Rectangle, Size, Vector, mouse};

use super::list_tile::row_style;
use super::min_size::{Minimum, min_size};
use super::toggles::{Checkbox, Radio, Switch, Toggle};
use super::{focusable, styled_text, tooltip};
use crate::theme::{DISABLED_CONTENT_OPACITY, button};
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme};

/// Space around the content of a toggle button.
pub const TOGGLE_BUTTON_PADDING: Padding = Padding {
    top: spacing::XXS,
    right: spacing::XS,
    bottom: spacing::XXS,
    left: spacing::XS,
};

/// A row with a control and a clickable label. Built by [`toggle_button`],
/// [`check_button`], [`radio_button`] and [`switch_button`].
pub struct ToggleButton<'a, Message> {
    control: Element<'a, Message>,
    title: Fragment<'a>,
    subtitle: Option<Fragment<'a>>,
    on_toggle: Option<Message>,
    enabled: bool,
    control_at_end: bool,
    padding: Padding,
    id: Option<Id>,
}

/// A toggle button with any `control` before `title`. The control keeps its
/// own input but never takes the focus; the row publishes the message of
/// [`ToggleButton::on_toggle`] when it is clicked outside the control or
/// activated with the keyboard.
pub fn toggle_button<'a, Message: 'a>(
    control: impl Into<Element<'a, Message>>,
    title: impl IntoFragment<'a>,
) -> ToggleButton<'a, Message> {
    ToggleButton {
        control: Element::new(ExcludeFocus {
            content: control.into(),
        }),
        title: title.into_fragment(),
        subtitle: None,
        on_toggle: None,
        enabled: false,
        control_at_end: false,
        padding: TOGGLE_BUTTON_PADDING,
        id: None,
    }
}

/// A toggle button around a painted toggle: the control only draws itself
/// and the row publishes the toggle's activation message.
fn from_toggle<'a, Message: Clone + 'a>(
    toggle: Toggle<Message>,
    title: impl IntoFragment<'a>,
) -> ToggleButton<'a, Message> {
    let control_at_end = !toggle.leads();
    let enabled = toggle.enabled;
    let on_toggle = toggle.on_activate.clone();

    ToggleButton {
        control: toggle.passive().into(),
        title: title.into_fragment(),
        subtitle: None,
        on_toggle,
        enabled,
        control_at_end,
        padding: TOGGLE_BUTTON_PADDING,
        id: None,
    }
}

/// A check box with a clickable `title` after it; the row publishes the
/// message of the check box's [`on_change`](Checkbox::on_change).
pub fn check_button<'a, Message: Clone + 'a>(
    checkbox: Checkbox<'a, Message>,
    title: impl IntoFragment<'a>,
) -> ToggleButton<'a, Message> {
    from_toggle(checkbox.into_toggle(), title)
}

/// A radio with a clickable `title` after it; the row publishes the message
/// of the radio's [`on_change`](Radio::on_change).
pub fn radio_button<'a, T: PartialEq, Message: Clone + 'a>(
    radio: Radio<'a, T, Message>,
    title: impl IntoFragment<'a>,
) -> ToggleButton<'a, Message> {
    from_toggle(radio.into_toggle(), title)
}

/// A switch with a clickable `title` before it; the row publishes the
/// message of the switch's [`on_toggle`](Switch::on_toggle).
pub fn switch_button<'a, Message: Clone + 'a>(
    switch: Switch<'a, Message>,
    title: impl IntoFragment<'a>,
) -> ToggleButton<'a, Message> {
    from_toggle(switch.into_toggle(), title)
}

impl<'a, Message> ToggleButton<'a, Message> {
    /// Sets the secondary text below the title.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl IntoFragment<'a>) -> Self {
        self.subtitle = Some(subtitle.into_fragment());
        self
    }

    /// Sets the message published when the row is clicked or activated with
    /// the keyboard; a [`toggle_button`] without it is disabled and dimmed.
    #[must_use]
    pub fn on_toggle(mut self, message: Message) -> Self {
        self.on_toggle = Some(message);
        self.enabled = true;
        self
    }

    /// Places the control after the text instead of before it.
    #[must_use]
    pub fn control_at_end(mut self, at_end: bool) -> Self {
        self.control_at_end = at_end;
        self
    }

    /// Sets the space around the row content.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the widget [`Id`] of the row.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<ToggleButton<'a, Message>> for Element<'a, Message> {
    fn from(button: ToggleButton<'a, Message>) -> Self {
        let enabled = button.enabled;
        let foreground = move |theme: &Theme| {
            let on_surface = theme.roles().on_surface;
            if enabled {
                on_surface
            } else {
                on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            }
        };

        let mut texts =
            Column::new().push(styled_text(button.title, typography::BODY_MEDIUM).style(
                move |theme: &Theme| text::Style {
                    color: Some(foreground(theme)),
                },
            ));

        if let Some(subtitle) = button.subtitle {
            texts = texts.push(styled_text(subtitle, typography::BODY_SMALL).style(
                move |theme: &Theme| text::Style {
                    color: Some(if enabled {
                        theme.roles().on_surface_variant
                    } else {
                        foreground(theme)
                    }),
                },
            ));
        }

        let content = if button.control_at_end {
            Row::new().push(texts).push(button.control)
        } else {
            Row::new().push(button.control).push(texts)
        }
        .spacing(spacing::SM)
        .align_y(Alignment::Center);

        let native = iced::widget::Button::new(content)
            .padding(button.padding)
            .on_press_maybe(button.on_toggle.clone())
            .class(button::Class::Custom(Box::new(move |theme, status| {
                row_style(
                    theme,
                    status,
                    None,
                    foreground(theme),
                    enabled,
                    radius::SMALL,
                )
            })));

        // An enabled row with nothing to publish, such as a selected radio,
        // still reads as clickable.
        let native: Element<'a, Message> = if enabled && button.on_toggle.is_none() {
            iced::widget::mouse_area(native)
                .interaction(mouse::Interaction::Pointer)
                .into()
        } else {
            native.into()
        };

        let mut wrapper = focusable(native).radius(radius::SMALL).enabled(enabled);

        if let Some(message) = button.on_toggle {
            wrapper = wrapper.on_activate(message);
        }

        if let Some(id) = button.id {
            wrapper = wrapper.id(id);
        }

        wrapper.into()
    }
}

/// Lays out, draws and forwards input to its content, but hides it from
/// widget operations, so nothing inside takes the keyboard focus.
pub(crate) struct ExcludeFocus<'a, Message> {
    content: Element<'a, Message>,
}

/// Hides `content` from widget operations, so nothing inside takes the
/// keyboard focus, while it still lays out, draws and takes input.
pub(crate) fn exclude_focus<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message> {
    Element::new(ExcludeFocus {
        content: content.into(),
    })
}

impl<Message> Widget<Message, Theme, iced::Renderer> for ExcludeFocus<'_, Message> {
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
        _tree: &mut Tree,
        _layout: Layout<'_>,
        _renderer: &iced::Renderer,
        _operation: &mut dyn Operation,
    ) {
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

/// A square outlined button for secondary options. Built by
/// [`option_button`] and [`option_color`].
pub struct OptionButton<'a, Message> {
    content: Element<'a, Message>,
    label: Option<Cow<'a, str>>,
    on_press: Option<Message>,
    id: Option<Id>,
}

/// A small square outlined button showing `content`, usually an icon, such
/// as the buttons next to a list. It is as tall and wide as a pointer
/// button. Give an icon-only button a [`OptionButton::label`].
pub fn option_button<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> OptionButton<'a, Message> {
    OptionButton {
        content: content.into(),
        label: None,
        on_press: None,
        id: None,
    }
}

/// An option button showing a `color` swatch named `label`: an icon-sized
/// square with the check radius and an outline.
pub fn option_color<'a, Message: 'a>(
    color: Color,
    label: impl IntoFragment<'a>,
) -> OptionButton<'a, Message> {
    let swatch = container(iced::widget::Space::new())
        .width(sizes::ICON_SIZE)
        .height(sizes::ICON_SIZE)
        .style(move |theme: &Theme| container::Style {
            background: Some(color.into()),
            border: Border::default()
                .color(theme.roles().outline)
                .width(1.0)
                .rounded(radius::CHECK),
            ..container::Style::default()
        });

    OptionButton {
        content: swatch.into(),
        label: Some(label.into_fragment()),
        on_press: None,
        id: None,
    }
}

impl<'a, Message> OptionButton<'a, Message> {
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

    /// Sets the label shown as a tooltip.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Sets the widget [`Id`].
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<OptionButton<'a, Message>> for Element<'a, Message> {
    fn from(button: OptionButton<'a, Message>) -> Self {
        let side = Minimum::Fixed(sizes::BUTTON_HEIGHT);
        let native = iced::widget::Button::new(min_size(button.content, side, side))
            .padding(0)
            .class(button::Class::Outlined)
            .on_press_maybe(button.on_press.clone());

        let mut wrapper = focusable(native)
            .radius(radius::MEDIUM)
            .enabled(button.on_press.is_some());

        if let Some(message) = button.on_press {
            wrapper = wrapper.on_activate(message);
        }

        if let Some(id) = button.id {
            wrapper = wrapper.id(id);
        }

        match button.label {
            Some(label) => tooltip(wrapper, label).into(),
            None => wrapper.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::widget::toggles;

    #[derive(Debug, Clone, PartialEq)]
    enum Message {
        Check(Option<bool>),
        Switch(bool),
    }

    #[test]
    fn switch_buttons_put_the_control_after_the_title() {
        let check = check_button(toggles::checkbox(false).on_change(Message::Check), "a");
        assert!(!check.control_at_end);
        assert!(check.enabled);
        assert_eq!(check.on_toggle, Some(Message::Check(Some(true))));

        let switch = switch_button(toggles::switch(false).on_toggle(Message::Switch), "b");
        assert!(switch.control_at_end);
        assert_eq!(switch.on_toggle, Some(Message::Switch(true)));
    }

    #[test]
    fn a_generic_toggle_button_is_disabled_until_it_has_a_message() {
        let button = toggle_button::<Message>("x", "a");
        assert!(!button.enabled);
        assert!(button.on_toggle(Message::Switch(true)).enabled);
    }
}
