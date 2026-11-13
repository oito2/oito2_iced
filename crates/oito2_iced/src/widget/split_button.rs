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

//! A split button: a main action with an attached arrow that opens a menu
//! of related actions.
//!
//! The two parts share one rounded shape, split by a one-pixel gap: the
//! main part has the rounded corners at the start of the reading
//! direction, the arrow part those at the end. Each part is a focus stop
//! of its own. The arrow opens a [`menu`](super::menu) below itself.
//!
//! ```no_run
//! use oito2_iced::widget::{menu, split_button};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Save,
//!     SaveAs,
//!     Menu(bool),
//! }
//!
//! # fn view<'a>(open: bool) -> oito2_iced::Element<'a, Message> {
//! split_button::split_button("Save", [menu::item("Save as…").on_press(Message::SaveAs)])
//!     .variant(split_button::Variant::Filled)
//!     .on_press(Message::Save)
//!     .open(open)
//!     .on_open_change(Message::Menu)
//!     .into()
//! # }
//! ```

use iced::Padding;
use iced::advanced::widget::Id;
use iced::border::Radius;
use iced::widget::text::{Fragment, IntoFragment};
use iced::widget::{Row, button};

use super::menu::{Item, List, Mark, Shape, anchored_menu};
use super::min_size::{Minimum, min_size};
use super::{focusable, styled_text};
use crate::theme::button::Class;
use crate::tokens::{radius, spacing, typography};
use crate::widget::helpers::ReadingDirection;
use crate::{Element, Theme};

/// The look of a [`SplitButton`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// A neutral elevated button.
    #[default]
    Elevated,
    /// A filled primary button.
    Filled,
    /// An outlined button.
    Outlined,
}

impl Variant {
    fn class(self) -> Class<'static> {
        match self {
            Variant::Elevated => Class::Elevated,
            Variant::Filled => Class::Filled,
            Variant::Outlined => Class::Outlined,
        }
    }
}

/// A closure that turns the requested open state into a message.
type OnOpenChange<'a, Message> = Box<dyn Fn(bool) -> Message + 'a>;

/// A main action with an arrow that opens a menu. Built by
/// [`split_button`].
pub struct SplitButton<'a, Message> {
    label: Fragment<'a>,
    items: Vec<Item<'a, Message>>,
    variant: Variant,
    on_press: Option<Message>,
    open: bool,
    on_open_change: Option<OnOpenChange<'a, Message>>,
    direction: ReadingDirection,
    id: Option<Id>,
    menu_id: Option<Id>,
}

/// A split button with `label` on its main part and a menu of `items`
/// behind its arrow.
pub fn split_button<'a, Message>(
    label: impl IntoFragment<'a>,
    items: impl IntoIterator<Item = Item<'a, Message>>,
) -> SplitButton<'a, Message> {
    SplitButton {
        label: label.into_fragment(),
        items: items.into_iter().collect(),
        variant: Variant::default(),
        on_press: None,
        open: false,
        on_open_change: None,
        direction: ReadingDirection::LeftToRight,
        id: None,
        menu_id: None,
    }
}

impl<'a, Message> SplitButton<'a, Message> {
    /// Sets the look.
    #[must_use]
    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    /// Sets the message of the main part. Without it both parts are
    /// disabled.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the main message, or disables both parts with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Shows or hides the menu.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the closure that turns the requested open state into a message:
    /// `true` on a click, Enter or Space on the arrow while closed; `false`
    /// on the same while open, Escape, a click outside, Tab, or after an
    /// item that closes the menu. Without it the arrow is disabled.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Sets the reading direction, which places the rounded corners, the
    /// menu and its items.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the widget [`Id`] of the main part.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the widget [`Id`] of the arrow part.
    #[must_use]
    pub fn menu_id(mut self, id: impl Into<Id>) -> Self {
        self.menu_id = Some(id.into());
        self
    }
}

/// A part of a split button: a native button with the colors of `variant`
/// and the corners of `corners`, with focus and the control height.
fn part<'a, Message: Clone + 'a>(
    content: Element<'a, Message>,
    variant: Variant,
    corners: Radius,
    padding: Padding,
    on_press: Option<Message>,
    id: Option<Id>,
) -> Element<'a, Message> {
    let content = min_size(content, Minimum::None, Minimum::Control(0.0));
    let class = variant.class();

    let native = button(content)
        .padding(padding)
        .style(move |theme: &Theme, status| {
            let style = <Theme as button::Catalog>::style(theme, &class, status);

            button::Style {
                border: style.border.rounded(corners),
                ..style
            }
        })
        .on_press_maybe(on_press.clone());

    let mut wrapper = focusable(native)
        .radius(radius::MEDIUM)
        .enabled(on_press.is_some());
    if let Some(message) = on_press {
        wrapper = wrapper.on_activate(message);
    }
    if let Some(id) = id {
        wrapper = wrapper.id(id);
    }

    wrapper.into()
}

impl<'a, Message: Clone + 'a> From<SplitButton<'a, Message>> for Element<'a, Message> {
    fn from(split: SplitButton<'a, Message>) -> Self {
        let SplitButton {
            label,
            items,
            variant,
            on_press,
            open,
            on_open_change,
            direction,
            id,
            menu_id,
        } = split;

        let enabled = on_press.is_some();
        let on_open_change = on_open_change.filter(|_| enabled);
        let open = open && on_open_change.is_some();
        let request = |value: bool| on_open_change.as_ref().map(|f| f(value));
        let close = request(false);

        let ltr = direction == ReadingDirection::LeftToRight;
        let rounded = Radius::new(0.0);
        let (start, end) = if ltr {
            (rounded.left(radius::MEDIUM), rounded.right(radius::MEDIUM))
        } else {
            (rounded.right(radius::MEDIUM), rounded.left(radius::MEDIUM))
        };

        let main = part(
            styled_text(label, typography::LABEL_LARGE).into(),
            variant,
            start,
            Padding::from([0.0, spacing::LG]),
            on_press,
            id,
        );
        let arrow = part(
            Mark::new(Shape::ChevronDown).into(),
            variant,
            end,
            Padding::from([0.0, spacing::SM]),
            request(!open),
            menu_id,
        );

        let panel = List::new(items, open, close.clone(), direction).into_panel();
        let arrow: Element<'a, Message> =
            anchored_menu(arrow, panel, open, close, direction).into();

        let parts = if ltr {
            vec![main, arrow]
        } else {
            vec![arrow, main]
        };

        Row::from_vec(parts).spacing(1.0).into()
    }
}
