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

//! Segment groups: a row (or column) of mutually exclusive choices on a
//! recessed track, with a raised marker that slides to the selected one.
//!
//! The group is one stop in keyboard navigation: the arrow keys of its
//! orientation select the previous or next enabled segment, wrapping at the
//! ends, Home and End select the first and last enabled segment, and Space
//! or Enter selects the focused one. The selection lives in the
//! application: the group shows `selected` and publishes the value of a
//! chosen segment through [`SegmentGroup::on_change`].
//!
//! ```no_run
//! use oito2_iced::widget::segment_group;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     View(&'static str),
//! }
//!
//! # fn view<'a>(selected: &'static str) -> oito2_iced::Element<'a, Message> {
//! segment_group::segment_group(
//!     [
//!         segment_group::segment("day", "Day"),
//!         segment_group::segment("week", "Week"),
//!         segment_group::segment("month", "Month"),
//!     ],
//!     Some(selected),
//! )
//! .on_change(Message::View)
//! .into()
//! # }
//! ```

#[cfg(feature = "svg")]
use iced::widget::row;
use iced::widget::text::IntoFragment;
use iced::widget::{Button, button, container};
use iced::{Background, Border, Length, Padding};

use super::min_size::{Minimum, min_size};
use super::styled_text;
use super::toggle_group::{DISABLED_ITEM_OPACITY, ITEM_TEXT, Roving, track_style};
#[cfg(feature = "svg")]
use crate::icon::{Icon, icon};
use crate::theme::button::Class;
use crate::tokens::{radius, spacing};
use crate::{Element, Theme};

/// A choice of a [`SegmentGroup`]. Built by [`segment`].
pub struct Segment<'a, T, Message> {
    value: T,
    label: Element<'a, Message>,
    #[cfg(feature = "svg")]
    icon: Option<Icon>,
    enabled: bool,
}

/// A segment for `value` showing `label`.
pub fn segment<'a, T, Message: 'a>(
    value: T,
    label: impl IntoFragment<'a>,
) -> Segment<'a, T, Message> {
    Segment {
        value,
        label: styled_text(label, ITEM_TEXT).into(),
        #[cfg(feature = "svg")]
        icon: None,
        enabled: true,
    }
}

impl<T, Message> Segment<'_, T, Message> {
    /// Places `icon` before the label.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets whether the segment can be selected (true by default); a
    /// disabled segment is dimmed.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// A row of mutually exclusive choices. See [`segment_group`].
pub struct SegmentGroup<'a, T, Message> {
    segments: Vec<Segment<'a, T, Message>>,
    selected: Option<T>,
    on_change: Option<Box<dyn Fn(T) -> Message + 'a>>,
    vertical: bool,
}

/// A segment group of `segments` with the segment of `selected` selected.
/// Without [`SegmentGroup::on_change`] the group is disabled.
pub fn segment_group<'a, T, Message>(
    segments: impl IntoIterator<Item = Segment<'a, T, Message>>,
    selected: Option<T>,
) -> SegmentGroup<'a, T, Message> {
    SegmentGroup {
        segments: segments.into_iter().collect(),
        selected,
        on_change: None,
        vertical: false,
    }
}

impl<'a, T, Message> SegmentGroup<'a, T, Message> {
    /// Sets the function that turns the value of a chosen segment into a
    /// message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(T) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or disables the group with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(T) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(T) -> Message + 'a>);
        self
    }

    /// Runs the segments down instead of across; they share the width of
    /// the widest one and align their labels at the start.
    #[must_use]
    pub fn vertical(mut self, vertical: bool) -> Self {
        self.vertical = vertical;
        self
    }
}

/// The style of a segment: transparent, with the text-safe primary tone
/// when selected and dimmed when disabled. The marker behind the selected
/// segment is drawn by the group.
fn segment_style(theme: &Theme, selected: bool, enabled: bool) -> button::Style {
    let roles = theme.roles();
    let foreground = if selected {
        roles.semantic.primary_text
    } else {
        roles.on_surface
    };
    let opacity = if enabled { 1.0 } else { DISABLED_ITEM_OPACITY };

    button::Style {
        background: None::<Background>,
        text_color: foreground.scale_alpha(opacity),
        border: Border::default().rounded(radius::SMALL),
        ..button::Style::default()
    }
}

impl<'a, T, Message> From<SegmentGroup<'a, T, Message>> for Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
    Message: Clone + 'a,
{
    fn from(group: SegmentGroup<'a, T, Message>) -> Self {
        let vertical = group.vertical;
        let selected_index = group
            .segments
            .iter()
            .position(|segment| Some(&segment.value) == group.selected.as_ref());
        let interactive = group.on_change.is_some();

        let mut children = Vec::with_capacity(group.segments.len());
        let mut enabled = Vec::with_capacity(group.segments.len());
        let mut messages = Vec::with_capacity(group.segments.len());

        for (index, segment) in group.segments.into_iter().enumerate() {
            let selected = Some(index) == selected_index;
            let can_select = interactive && segment.enabled;
            let message = if can_select && !selected {
                group
                    .on_change
                    .as_ref()
                    .map(|on_change| on_change(segment.value.clone()))
            } else {
                None
            };

            #[cfg(feature = "svg")]
            let content: Element<'a, Message> = match segment.icon {
                Some(leading) => row![icon(leading), segment.label]
                    .spacing(spacing::SM)
                    .align_y(iced::Alignment::Center)
                    .into(),
                None => segment.label,
            };
            #[cfg(not(feature = "svg"))]
            let content: Element<'a, Message> = segment.label;

            let content = min_size(content, Minimum::None, Minimum::Control(0.0));
            let content: Element<'a, Message> = if vertical {
                container(content).align_left(Length::Fill).into()
            } else {
                content.into()
            };
            let enabled_look = segment.enabled;

            let button = Button::new(content)
                .padding(Padding::from([0.0, spacing::MD]))
                .width(if vertical {
                    Length::Fill
                } else {
                    Length::Shrink
                })
                .on_press_maybe(message.clone())
                .class(Class::Custom(Box::new(move |theme, _status| {
                    segment_style(theme, selected, enabled_look)
                })));

            children.push(button.into());
            enabled.push(can_select);
            messages.push(message);
        }

        let roving = Roving {
            items: children,
            enabled,
            messages,
            vertical,
            spacing: 0.0,
            stop: selected_index,
            select_on_move: true,
            marker: selected_index,
            ring_radius: radius::SMALL,
        };

        container(roving)
            .padding(spacing::XS + 1.0)
            .style(|theme| track_style(theme, radius::MEDIUM))
            .into()
    }
}
