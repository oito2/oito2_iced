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

//! Dialogs: a surface centered over a scrim that keeps the input, with an
//! optional title bar, a title, the content and a row of actions.
//!
//! [`host`] wraps the application view and shows the [`Dialog`] it is given
//! above it; the application owns whether the dialog is open and renders
//! `None` to close it. While open, the view below receives no input, Tab and
//! Shift+Tab cycle the focus inside the dialog, Escape publishes the dismiss
//! message, and so does a press on the scrim unless the dialog is not
//! [dismissible](Dialog::dismissible).
//!
//! The dialog is drawn on the dialog surface of the theme: the dialog chrome
//! tone with the window radius, a thin `outline_variant` border and a shadow.
//! The title uses the title large style; the actions sit at the end of the
//! last row.
//!
//! ```no_run
//! use oito2_iced::widget::{button, dialog};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Close,
//! }
//!
//! # fn view<'a>(page: oito2_iced::Element<'a, Message>, open: bool) -> oito2_iced::Element<'a, Message> {
//! dialog::host(
//!     page,
//!     open.then(|| {
//!         dialog::dialog(iced::widget::text("Changes are saved automatically."))
//!             .title("Settings")
//!             .action(button::filled("Close").on_press(Message::Close))
//!             .on_dismiss(Message::Close)
//!     }),
//! )
//! # }
//! ```

pub(crate) mod layer;

use std::time::Duration;

use iced::widget::text::{IntoFragment, Wrapping};
use iced::widget::{Column, Row, Space, container};
use iced::{Alignment, Length, Padding};

use self::layer::{Layer, Placement, Surface};
use super::styled_text;
use crate::Element;
use crate::foundation::{WindowButton, WindowButtonLayout, WindowControlStyle};
use crate::theme::container::Class as ContainerClass;
use crate::tokens::{sizes, spacing, typography};
use crate::window::{ControlType, Labels, window_control};

/// Opacity of the scrim color behind a dialog.
pub const SCRIM_OPACITY: f32 = 0.54;

/// Smallest width of a dialog, when the window allows it.
pub const MIN_WIDTH: f32 = 280.0;

/// Space kept free between a dialog and the window edges: vertical, then
/// horizontal.
pub const INSET: [f32; 2] = [24.0, 40.0];

/// The title bar of a dialog: a title, an optional leading element and a
/// window close button.
///
/// The close button follows the window control style, with the GNOME style
/// in place of [`WindowControlStyle::None`] and [`WindowControlStyle::Macos`],
/// and sits on the side the button layout puts the close button on. The bar
/// is [`sizes::TITLE_BAR_HEIGHT`] tall, transparent and without a border,
/// and does not move the window. The title is centered, except with the
/// Windows style. Built by [`title_bar`].
pub struct TitleBar<'a, Message> {
    title: Option<iced::widget::text::Fragment<'a>>,
    leading: Option<Element<'a, Message>>,
    on_close: Option<Message>,
    style: WindowControlStyle,
    close_leading: bool,
    labels: Labels,
}

/// An empty dialog title bar with the GNOME close button at the end.
pub fn title_bar<'a, Message>() -> TitleBar<'a, Message> {
    TitleBar {
        title: None,
        leading: None,
        on_close: None,
        style: WindowControlStyle::Gnome,
        close_leading: false,
        labels: Labels::default(),
    }
}

impl<'a, Message> TitleBar<'a, Message> {
    /// Shows `title` in the title medium style.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Places `element` before the title, such as an icon that tells what
    /// the dialog is about.
    #[must_use]
    pub fn leading(mut self, element: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(element.into());
        self
    }

    /// Sets the message published by the close button. Inside a
    /// [`Dialog`] it defaults to the dismiss message of the dialog.
    #[must_use]
    pub fn on_close(mut self, message: Message) -> Self {
        self.on_close = Some(message);
        self
    }

    /// Sets the window control style of the close button.
    #[must_use]
    pub fn control_style(mut self, style: WindowControlStyle) -> Self {
        self.style = style;
        self
    }

    /// Puts the close button on the side `layout` puts it on: at the start
    /// when the leading buttons include it, at the end otherwise.
    #[must_use]
    pub fn button_layout(mut self, layout: &WindowButtonLayout) -> Self {
        self.close_leading = layout.leading.contains(&WindowButton::Close);
        self
    }

    /// Sets the labels of the window buttons; defaults to English.
    #[must_use]
    pub fn labels(mut self, labels: Labels) -> Self {
        self.labels = labels;
        self
    }
}

/// The control style of a dialog close button for the system `style`.
fn close_style(style: WindowControlStyle) -> WindowControlStyle {
    match style {
        WindowControlStyle::None | WindowControlStyle::Macos => WindowControlStyle::Gnome,
        style => style,
    }
}

impl<'a, Message> From<TitleBar<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(bar: TitleBar<'a, Message>) -> Self {
        let style = close_style(bar.style);
        let edge = if style == WindowControlStyle::Windows {
            0.0
        } else {
            spacing::SM
        };

        let mut close = Some(
            window_control(ControlType::Close, style, bar.labels.close.clone())
                .height(sizes::TITLE_BAR_HEIGHT),
        )
        .map(|control| match bar.on_close {
            Some(message) => control.on_press(message),
            None => control,
        });

        let mut leading: Vec<Element<'a, Message>> = vec![Space::new().width(edge).into()];
        if bar.close_leading
            && let Some(control) = close.take()
        {
            leading.push(control.into());
        }
        if let Some(element) = bar.leading {
            leading.push(Space::new().width(spacing::XS).into());
            leading.push(element);
        }

        let mut trailing: Vec<Element<'a, Message>> = Vec::new();
        if let Some(control) = close {
            trailing.push(control.into());
        }
        trailing.push(Space::new().width(edge).into());

        let leading = Row::with_children(leading).align_y(Alignment::Center);
        let trailing = Row::with_children(trailing).align_y(Alignment::Center);
        let middle = bar
            .title
            .map(|title| styled_text(title, typography::TITLE_MEDIUM).wrapping(Wrapping::None));

        let content: Element<'a, Message> = match middle {
            Some(middle) if style != WindowControlStyle::Windows => Row::new()
                .push(container(leading).width(Length::Fill))
                .push(container(middle).padding([0.0, spacing::LG]))
                .push(
                    container(trailing)
                        .width(Length::Fill)
                        .align_x(Alignment::End),
                )
                .align_y(Alignment::Center)
                .into(),
            Some(middle) => Row::new()
                .push(leading)
                .push(
                    container(middle)
                        .padding([0.0, spacing::LG])
                        .width(Length::Fill),
                )
                .push(trailing)
                .align_y(Alignment::Center)
                .into(),
            None => Row::new()
                .push(leading)
                .push(Space::new().width(Length::Fill))
                .push(trailing)
                .align_y(Alignment::Center)
                .into(),
        };

        container(content)
            .width(Length::Fill)
            .height(sizes::TITLE_BAR_HEIGHT)
            .align_y(Alignment::Center)
            .clip(true)
            .into()
    }
}

/// A dialog: the content with an optional title bar, title and actions.
/// Built by [`dialog`]; shown by [`host`], or drawn in place as an
/// [`Element`].
pub struct Dialog<'a, Message> {
    content: Element<'a, Message>,
    title: Option<iced::widget::text::Fragment<'a>>,
    title_bar: Option<TitleBar<'a, Message>>,
    actions: Vec<Element<'a, Message>>,
    on_dismiss: Option<Message>,
    dismissible: bool,
    width: Length,
    max_width: f32,
}

/// A dialog showing `content`.
pub fn dialog<'a, Message>(content: impl Into<Element<'a, Message>>) -> Dialog<'a, Message> {
    Dialog {
        content: content.into(),
        title: None,
        title_bar: None,
        actions: Vec::new(),
        on_dismiss: None,
        dismissible: true,
        width: Length::Fill,
        max_width: f32::INFINITY,
    }
}

impl<'a, Message> Dialog<'a, Message> {
    /// Shows `title` above the content, in the title large style.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Shows `bar` at the top of the dialog.
    #[must_use]
    pub fn title_bar(mut self, bar: TitleBar<'a, Message>) -> Self {
        self.title_bar = Some(bar);
        self
    }

    /// Adds `action`, usually a button, after the previous ones in the row
    /// at the bottom.
    #[must_use]
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Sets the message published on Escape, on a press on the scrim and by
    /// the close button of the title bar.
    #[must_use]
    pub fn on_dismiss(mut self, message: Message) -> Self {
        self.on_dismiss = Some(message);
        self
    }

    /// Sets whether a press on the scrim dismisses the dialog (`true` by
    /// default). Escape dismisses it either way.
    #[must_use]
    pub fn dismissible(mut self, dismissible: bool) -> Self {
        self.dismissible = dismissible;
        self
    }

    /// Sets the width. By default a dialog shown by [`host`] is as wide as
    /// its content, at least [`MIN_WIDTH`], and one drawn in place fills the
    /// width it is given.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the largest width.
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        self.max_width = width;
        self
    }
}

impl<'a, Message> From<Dialog<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(dialog: Dialog<'a, Message>) -> Self {
        let Dialog {
            content,
            title,
            title_bar,
            actions,
            on_dismiss,
            width,
            max_width,
            ..
        } = dialog;

        let mut children: Vec<Element<'a, Message>> = Vec::new();

        if let Some(mut bar) = title_bar {
            if bar.on_close.is_none() {
                bar.on_close = on_dismiss;
            }
            children.push(bar.into());
        }

        if let Some(title) = title {
            children.push(
                container(styled_text(title, typography::TITLE_LARGE))
                    .padding(Padding {
                        top: spacing::XL,
                        right: spacing::XL,
                        bottom: 0.0,
                        left: spacing::XL,
                    })
                    .into(),
            );
        }

        children.push(
            container(content)
                .padding(Padding {
                    top: spacing::LG,
                    right: spacing::XL,
                    bottom: spacing::XL,
                    left: spacing::XL,
                })
                .into(),
        );

        if !actions.is_empty() {
            children.push(
                container(
                    Row::with_children(actions)
                        .spacing(spacing::SM)
                        .align_y(Alignment::Center),
                )
                .padding(Padding {
                    top: 0.0,
                    right: spacing::XL,
                    bottom: spacing::XL,
                    left: spacing::XL,
                })
                .width(Length::Fill)
                .align_x(Alignment::End)
                .into(),
            );
        }

        container(Column::with_children(children))
            .class(ContainerClass::Dialog)
            .width(width)
            .max_width(max_width)
            .clip(true)
            .into()
    }
}

/// `base` with `dialog`, when given, centered above it over a scrim.
///
/// Wrap the whole application view, inside [`focus::navigation`]:
/// `focus::navigation(dialog::host(view, dialog))`.
///
/// [`focus::navigation`]: crate::focus::navigation
pub fn host<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    dialog: Option<Dialog<'a, Message>>,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    let surface = dialog.map(|dialog| {
        let on_dismiss = dialog.on_dismiss.clone();
        let dismissible = dialog.dismissible;

        Surface {
            content: dialog.into(),
            placement: Placement::Center {
                inset: Padding::from(INSET),
                min_width: MIN_WIDTH,
            },
            modal: true,
            scrim: SCRIM_OPACITY,
            on_dismiss,
            dismissible,
            draggable: false,
            snaps: None,
            entrance: Duration::ZERO,
            settle: Duration::ZERO,
        }
    });

    Layer::new(base, surface).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_close_button_falls_back_to_gnome_without_system_buttons() {
        assert_eq!(
            close_style(WindowControlStyle::None),
            WindowControlStyle::Gnome
        );
        assert_eq!(
            close_style(WindowControlStyle::Macos),
            WindowControlStyle::Gnome
        );
        assert_eq!(
            close_style(WindowControlStyle::Windows),
            WindowControlStyle::Windows
        );
    }
}
