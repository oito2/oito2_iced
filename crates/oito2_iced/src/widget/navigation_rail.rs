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

//! Navigation rails and their items.
//!
//! A [`NavigationRail`] is a vertical bar of destinations at the start side
//! of a window, in one of three [`NavigationRailStyle`]s: icons only, icons
//! with a small label below, or a wider rail with the labels beside the
//! icons. The selected destination sits on a rounded pill in the secondary
//! container color.
//!
//! The rail is one stop in keyboard navigation: Tab focuses the selected
//! destination, the up and down arrows move the focus (stopping at the
//! ends), Home and End move it to the first and last destination, and Space
//! or Enter select the focused one. The destinations scroll with the mouse
//! wheel when they don't fit the height, and the rail scrolls to keep the
//! selected and the focused destination in view. The selection lives in
//! the application: the rail shows `selected` and publishes the index of a
//! chosen destination through [`NavigationRail::on_select`].
//!
//! ```no_run
//! use oito2_iced::icon::lucide;
//! use oito2_iced::widget::navigation_rail;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Go(usize),
//! }
//!
//! # fn view<'a>(selected: usize) -> oito2_iced::Element<'a, Message> {
//! navigation_rail::navigation_rail(
//!     [
//!         navigation_rail::item(lucide::HOUSE, "Home"),
//!         navigation_rail::item(lucide::STAR, "Starred"),
//!     ],
//!     Some(selected),
//! )
//! .on_select(Message::Go)
//! .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::widget::text::{self, IntoFragment, Wrapping};
use iced::widget::{Column, column, container, row};
use iced::{Alignment, Background, Color, Length, Padding};

use super::tab_bar::{Look, Strip, Target};
use super::{styled_text, tooltip};
use crate::icon::Icon;
use crate::theme::DISABLED_CONTENT_OPACITY;
use crate::tokens::typography::{self, TextStyle};
use crate::tokens::{radius, sizes, spacing};
use crate::{Element, Theme};

/// The opacity of the hover layer on a destination pill.
const RAIL_HOVER_OPACITY: f32 = 0.04;

/// A destination of a [`NavigationRail`] or a
/// [`NavigationPage`](super::navigation_page::NavigationPage). Built by
/// [`item`].
#[derive(Debug, Clone)]
pub struct NavigationItem<'a> {
    icon: Icon,
    selected_icon: Option<Icon>,
    label: Cow<'a, str>,
    tooltip: Option<Cow<'a, str>>,
}

/// A destination showing `icon` and named `label`.
pub fn item<'a>(icon: Icon, label: impl IntoFragment<'a>) -> NavigationItem<'a> {
    NavigationItem {
        icon,
        selected_icon: None,
        label: label.into_fragment(),
        tooltip: None,
    }
}

impl<'a> NavigationItem<'a> {
    /// Sets the icon shown while the destination is selected; the icon by
    /// default.
    #[must_use]
    pub fn selected_icon(mut self, icon: Icon) -> Self {
        self.selected_icon = Some(icon);
        self
    }

    /// Sets the tooltip shown where labels are hidden; the label by
    /// default.
    #[must_use]
    pub fn tooltip(mut self, tooltip: impl IntoFragment<'a>) -> Self {
        self.tooltip = Some(tooltip.into_fragment());
        self
    }

    /// The destination name.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// The icon to show in the `selected` state.
    pub(crate) fn glyph(&self, selected: bool) -> Icon {
        if selected {
            self.selected_icon.unwrap_or(self.icon)
        } else {
            self.icon
        }
    }
}

/// How a [`NavigationRail`] shows its destinations.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NavigationRailStyle {
    /// Icons only, with the label as a tooltip.
    Compact,
    /// Icons with a small label below.
    #[default]
    Labelled,
    /// Icons with the label beside them, in a wider rail.
    LabelledExtended,
}

impl NavigationRailStyle {
    /// The rail width in this style.
    pub const fn width(self) -> f32 {
        match self {
            Self::Compact => 64.0,
            Self::Labelled => 88.0,
            Self::LabelledExtended => 240.0,
        }
    }

    /// The vertical space around each destination in this style.
    const fn item_padding(self) -> f32 {
        match self {
            Self::Labelled => spacing::XS,
            Self::Compact | Self::LabelledExtended => spacing::XXS,
        }
    }
}

/// `icon` at `size`, drawn in the text color of the parent; an empty
/// square of that size without the `svg` feature.
pub(crate) fn glyph<'a, Message: 'a>(icon: Icon, size: f32) -> Element<'a, Message> {
    #[cfg(feature = "svg")]
    {
        crate::icon::icon(icon).size(size).into()
    }
    #[cfg(not(feature = "svg"))]
    {
        let _ = icon;
        iced::widget::space().width(size).height(size).into()
    }
}

/// `label` in `style` and `color`, on one line; dimmed unless `enabled`.
pub(crate) fn label_text<'a>(
    label: Cow<'a, str>,
    style: TextStyle,
    color: fn(&Theme) -> Color,
    enabled: bool,
) -> iced::widget::Text<'a, Theme> {
    let opacity = if enabled {
        1.0
    } else {
        DISABLED_CONTENT_OPACITY
    };

    styled_text(label, style)
        .wrapping(Wrapping::None)
        .style(move |theme: &Theme| text::Style {
            color: Some(color(theme).scale_alpha(opacity)),
        })
}

/// The content of a rail destination in `style`: the pill with the icon,
/// the label below or beside it, dimmed unless `enabled`.
fn item_content<'a, Message: 'a>(
    item: &NavigationItem<'a>,
    selected: bool,
    style: NavigationRailStyle,
    enabled: bool,
) -> Element<'a, Message> {
    let icon = glyph(item.glyph(selected), sizes::ICON_SIZE);

    match style {
        NavigationRailStyle::Compact => {
            let pill = container(icon).center_x(48.0).center_y(40.0);
            let tip = item.tooltip.clone().unwrap_or_else(|| item.label.clone());

            tooltip(pill, tip).into()
        }
        NavigationRailStyle::Labelled => {
            let (text_style, color): (TextStyle, fn(&Theme) -> Color) = if selected {
                (
                    TextStyle {
                        weight: typography::EMPHASIS_WEIGHT,
                        ..typography::LABEL_SMALL
                    },
                    |theme| theme.roles().on_surface,
                )
            } else {
                (typography::LABEL_SMALL, |theme| {
                    theme.roles().on_surface_variant
                })
            };

            column![
                container(icon).center_x(56.0).center_y(32.0),
                label_text(item.label.clone(), text_style, color, enabled),
            ]
            .spacing(spacing::XXS)
            .align_x(Alignment::Center)
            .into()
        }
        NavigationRailStyle::LabelledExtended => container(
            row![
                icon,
                styled_text(item.label.clone(), typography::LABEL_LARGE).wrapping(Wrapping::None)
            ]
            .spacing(spacing::MD)
            .align_y(Alignment::Center),
        )
        .padding(Padding::from([0.0, spacing::MD]))
        .width(Length::Fill)
        .center_y(40.0)
        .into(),
    }
}

/// The content color of a destination: the icon, and the label beside it.
fn item_color(theme: &Theme, selected: bool) -> Color {
    let roles = theme.roles();

    if selected {
        roles.on_secondary_container
    } else {
        roles.on_surface_variant
    }
}

/// The selected destination pill.
fn pill_color(theme: &Theme) -> Color {
    theme.roles().secondary_container
}

/// The hover layer of a destination pill.
fn hover_color(theme: &Theme) -> Color {
    theme.roles().on_surface.scale_alpha(RAIL_HOVER_OPACITY)
}

/// The look of the destinations of a rail in `style`.
fn look(style: NavigationRailStyle) -> Look {
    Look {
        marker: Some(pill_color),
        marker_inset: 0.0,
        slide: false,
        radius: radius::LARGE,
        hover: hover_color,
        text: item_color,
        target: if style == NavigationRailStyle::Labelled {
            Target::FirstChild
        } else {
            Target::Item
        },
    }
}

/// The scrolling column of rail destinations in `style`, `selected`
/// selected, publishing through `on_select`.
pub(crate) fn rail_strip<'a, Message: 'a>(
    items: &[NavigationItem<'a>],
    selected: Option<usize>,
    style: NavigationRailStyle,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
) -> Strip<'a, Message> {
    let padding = style.item_padding();
    let enabled = on_select.is_some();
    let mut strip = Strip::new(
        items
            .iter()
            .enumerate()
            .map(|(index, item)| item_content(item, Some(index) == selected, style, enabled))
            .collect(),
        look(style),
    );
    strip.selected = selected;
    strip.on_select = on_select;
    strip.vertical = true;
    strip.spacing = 2.0 * padding;
    strip.padding = Padding::from([padding, spacing::SM]);
    strip.main = Length::Fill;
    strip
}

/// A single rail destination. See [`rail_item`].
pub struct NavigationRailItem<'a, Message> {
    item: NavigationItem<'a>,
    selected: bool,
    on_press: Option<Message>,
    style: NavigationRailStyle,
}

/// The destination `item` alone, laid out as in a rail of `style`: a pill
/// in the secondary container color while `selected`, with the focus ring
/// and the hand cursor. Without [`NavigationRailItem::on_press`] it is
/// disabled.
pub fn rail_item<'a, Message>(
    item: NavigationItem<'a>,
    selected: bool,
    style: NavigationRailStyle,
) -> NavigationRailItem<'a, Message> {
    NavigationRailItem {
        item,
        selected,
        on_press: None,
        style,
    }
}

impl<Message> NavigationRailItem<'_, Message> {
    /// Sets the message published on a click, Enter or Space.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or disables the item with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }
}

impl<'a, Message> From<NavigationRailItem<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(item: NavigationRailItem<'a, Message>) -> Self {
        let on_select = item.on_press.map(|message| {
            Box::new(move |_index: usize| message.clone()) as Box<dyn Fn(usize) -> Message + 'a>
        });
        let mut strip = rail_strip(
            std::slice::from_ref(&item.item),
            item.selected.then_some(0),
            item.style,
            on_select,
        );
        strip.main = Length::Shrink;

        container(strip).width(item.style.width()).into()
    }
}

/// A vertical bar of destinations. See [`navigation_rail`].
pub struct NavigationRail<'a, Message> {
    items: Vec<NavigationItem<'a>>,
    selected: Option<usize>,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    style: NavigationRailStyle,
    leading: Option<Element<'a, Message>>,
    trailing: Option<Element<'a, Message>>,
    background: Option<Color>,
    padding: Padding,
}

/// A rail of `items` with the destination at `selected` selected, or none.
/// Without [`NavigationRail::on_select`] the rail is disabled.
pub fn navigation_rail<'a, Message>(
    items: impl IntoIterator<Item = NavigationItem<'a>>,
    selected: Option<usize>,
) -> NavigationRail<'a, Message> {
    NavigationRail {
        items: items.into_iter().collect(),
        selected,
        on_select: None,
        style: NavigationRailStyle::Labelled,
        leading: None,
        trailing: None,
        background: None,
        padding: Padding::from([spacing::SM, 0.0]),
    }
}

impl<'a, Message> NavigationRail<'a, Message> {
    /// Sets the function that turns the index of a chosen destination into
    /// a message.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or disables the rail with `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets how the destinations are shown (labelled by default).
    #[must_use]
    pub fn style(mut self, style: NavigationRailStyle) -> Self {
        self.style = style;
        self
    }

    /// Places `leading` above the destinations.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Places `trailing` below the destinations.
    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Sets the rail background, instead of the chrome sidebar color.
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the space around the destinations (the small spacing above and
    /// below by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

/// The background of a rail: `color`, or the chrome sidebar color.
pub(crate) fn rail_background(theme: &Theme, color: Option<Color>) -> container::Style {
    container::Style {
        background: Some(Background::Color(
            color.unwrap_or(theme.roles().chrome.sidebar),
        )),
        ..container::Style::default()
    }
}

impl<'a, Message> From<NavigationRail<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(rail: NavigationRail<'a, Message>) -> Self {
        let style = rail.style;
        let align = if style == NavigationRailStyle::LabelledExtended {
            Alignment::Start
        } else {
            Alignment::Center
        };
        let strip = rail_strip(&rail.items, rail.selected, style, rail.on_select);

        let content = Column::new()
            .push(rail.leading)
            .push(strip)
            .push(rail.trailing)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(align);
        let background = rail.background;

        container(content)
            .width(style.width())
            .height(Length::Fill)
            .padding(rail.padding)
            .style(move |theme| rail_background(theme, background))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_rail_widths_follow_the_style() {
        assert_eq!(NavigationRailStyle::Compact.width(), 64.0);
        assert_eq!(NavigationRailStyle::Labelled.width(), 88.0);
        assert_eq!(NavigationRailStyle::LabelledExtended.width(), 240.0);
    }

    #[test]
    fn the_selected_icon_replaces_the_icon_while_selected() {
        let home = Icon::new("home", "");
        let filled = Icon::new("home-filled", "");
        let item = item(home, "Home").selected_icon(filled);

        assert_eq!(item.glyph(false), home);
        assert_eq!(item.glyph(true), filled);
        assert_eq!(super::item(home, "Home").glyph(true), home);
    }
}
