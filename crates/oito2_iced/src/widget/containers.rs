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

//! Bordered containers, sections, tiles, tile lists and watermarks.
//!
//! ```no_run
//! use oito2_iced::widget::containers;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     OpenWifi,
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! containers::section(containers::tile_list([
//!     containers::tile().title("Wi-Fi").subtitle("Connected").on_press(Message::OpenWifi),
//!     containers::tile().title("Bluetooth").enabled(false),
//! ]))
//! .headline("Network")
//! .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::widget::Id;
use iced::alignment::{Horizontal, Vertical};
use iced::border::Radius;
use iced::widget::text::IntoFragment;
use iced::widget::{Column, Row, Stack, button, container, rule};
use iced::{Alignment, Border, Color, Length, Padding};

use super::selectable::{HOVER_OVERLAY_OPACITY, blend};
use super::{focusable, styled_text};
use crate::theme::{DISABLED_CONTENT_OPACITY, TRANSPARENT};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// Default opacity of the fill of a tinted [`BorderContainer`].
pub const TINT_OPACITY: f32 = 0.1;

/// Default opacity of a [`Watermark`].
pub const WATERMARK_OPACITY: f32 = 0.1;

/// A container with a thin border. See [`border_container`].
pub struct BorderContainer<'a, Message> {
    content: Element<'a, Message>,
    padding: Padding,
    margin: Padding,
    width: Length,
    height: Length,
    color: Option<Color>,
    border_color: Option<Color>,
    tint: Option<(Color, f32)>,
    radius: f32,
    clip: bool,
}

/// A container around `content` with a 1-pixel border in the divider color
/// (`outline_variant`), the large radius and the theme surface as
/// background. It sizes to its content unless given a width or height.
pub fn border_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
) -> BorderContainer<'a, Message> {
    BorderContainer {
        content: content.into(),
        padding: Padding::ZERO,
        margin: Padding::ZERO,
        width: Length::Shrink,
        height: Length::Shrink,
        color: None,
        border_color: None,
        tint: None,
        radius: radius::LARGE,
        clip: false,
    }
}

impl<'a, Message> BorderContainer<'a, Message> {
    /// Sets the space inside the border.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the space outside the border.
    #[must_use]
    pub fn margin(mut self, margin: impl Into<Padding>) -> Self {
        self.margin = margin.into();
        self
    }

    /// Sets the width of the bordered area.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the bordered area.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the background, instead of the theme surface.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the border color, instead of the divider color.
    #[must_use]
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }

    /// Tints the container: a `tint` border over a fill of `tint` at 10 %
    /// opacity, such as a status color for a highlighted note. Replaces the
    /// background and the border color.
    #[must_use]
    pub fn tint(mut self, tint: Color) -> Self {
        self.tint = Some((tint, TINT_OPACITY));
        self
    }

    /// Sets the opacity of the tinted fill, clamped to 0 to 1. Has effect
    /// only on a [tinted](Self::tint) container.
    #[must_use]
    pub fn tint_opacity(mut self, opacity: f32) -> Self {
        if let Some((_, current)) = &mut self.tint {
            *current = opacity.clamp(0.0, 1.0);
        }
        self
    }

    /// Sets the corner radius (the large radius by default).
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Clips the content to the bounds of the container (off by default,
    /// so focus rings of the children stay visible).
    #[must_use]
    pub fn clip(mut self, clip: bool) -> Self {
        self.clip = clip;
        self
    }
}

impl<'a, Message: 'a> From<BorderContainer<'a, Message>> for Element<'a, Message> {
    fn from(bordered: BorderContainer<'a, Message>) -> Self {
        let BorderContainer {
            content,
            padding,
            margin,
            width,
            height,
            color,
            border_color,
            tint,
            radius,
            clip,
        } = bordered;

        let (color, border_color) = match tint {
            Some((tint, opacity)) => (Some(tint.scale_alpha(opacity)), Some(tint)),
            None => (color, border_color),
        };

        let inner = container(content)
            .padding(padding)
            .width(width)
            .height(height)
            .clip(clip)
            .style(move |theme: &Theme| {
                let roles = theme.roles();

                container::Style {
                    text_color: Some(roles.on_surface),
                    background: Some(color.unwrap_or(roles.surface).into()),
                    border: Border::default()
                        .color(border_color.unwrap_or(roles.outline_variant))
                        .width(1.0)
                        .rounded(radius),
                    ..container::Style::default()
                }
            });

        if margin == Padding::ZERO {
            inner.into()
        } else {
            container(inner).padding(margin).into()
        }
    }
}

/// A titled group of content. See [`section`].
pub struct Section<'a, Message> {
    content: Element<'a, Message>,
    headline: Option<Cow<'a, str>>,
    actions: Vec<Element<'a, Message>>,
    padding: Padding,
    margin: Padding,
}

/// A titled group: an optional headline in the title medium style, with
/// optional actions at the end of its row, above a [`border_container`]
/// holding `content` with 16 pixels of padding. The section fills the
/// available width.
pub fn section<'a, Message>(content: impl Into<Element<'a, Message>>) -> Section<'a, Message> {
    Section {
        content: content.into(),
        headline: None,
        actions: Vec::new(),
        padding: Padding::new(spacing::LG),
        margin: Padding::ZERO,
    }
}

impl<'a, Message> Section<'a, Message> {
    /// Sets the section title.
    #[must_use]
    pub fn headline(mut self, headline: impl IntoFragment<'a>) -> Self {
        self.headline = Some(headline.into_fragment());
        self
    }

    /// Adds `action`, such as a button, at the end of the headline row.
    #[must_use]
    pub fn action(mut self, action: impl Into<Element<'a, Message>>) -> Self {
        self.actions.push(action.into());
        self
    }

    /// Sets the space inside the bordered container (16 pixels by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the space around the whole section.
    #[must_use]
    pub fn margin(mut self, margin: impl Into<Padding>) -> Self {
        self.margin = margin.into();
        self
    }
}

impl<'a, Message: 'a> From<Section<'a, Message>> for Element<'a, Message> {
    fn from(section: Section<'a, Message>) -> Self {
        let body = border_container(section.content)
            .padding(section.padding)
            .width(Length::Fill);

        let mut column = Column::new().width(Length::Fill);

        if section.headline.is_some() || !section.actions.is_empty() {
            let title: Element<'a, Message> = match section.headline {
                Some(headline) => container(styled_text(headline, typography::TITLE_MEDIUM))
                    .width(Length::Fill)
                    .into(),
                None => iced::widget::space::horizontal().into(),
            };

            let header = Row::with_children(std::iter::once(title).chain(section.actions))
                .align_y(Alignment::Center)
                .width(Length::Fill);

            column = column.push(container(header).padding(Padding {
                left: spacing::XS,
                bottom: spacing::SM,
                ..Padding::ZERO
            }));
        }

        container(column.push(body))
            .padding(section.margin)
            .width(Length::Fill)
            .into()
    }
}

/// A row with a leading widget, a title, a subtitle and a trailing widget.
/// See [`tile`].
pub struct Tile<'a, Message> {
    leading: Option<Element<'a, Message>>,
    title: Option<Cow<'a, str>>,
    subtitle: Option<Cow<'a, str>>,
    trailing: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    enabled: bool,
    id: Option<Id>,
    padding: Padding,
    corners: Radius,
}

/// A row, as used in settings and property lists, with an optional
/// leading widget (usually an icon, drawn in the secondary text color), a
/// title in the body large style, a subtitle in the body medium style and
/// the secondary color, and a trailing widget such as a switch or a value.
/// The tile fills the available width.
///
/// With [`on_press`](Tile::on_press) the whole row is a button: it shows a
/// hover overlay, takes the focus and publishes its message on a click,
/// Enter or Space. A [disabled](Tile::enabled) tile is dimmed and static.
pub fn tile<'a, Message>() -> Tile<'a, Message> {
    Tile {
        leading: None,
        title: None,
        subtitle: None,
        trailing: None,
        on_press: None,
        enabled: true,
        id: None,
        padding: Padding::from([spacing::MD, spacing::LG]),
        corners: Radius::default(),
    }
}

impl<'a, Message> Tile<'a, Message> {
    /// Sets the widget before the text, usually an icon.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Sets the primary text.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Sets the secondary text, below the title.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl IntoFragment<'a>) -> Self {
        self.subtitle = Some(subtitle.into_fragment());
        self
    }

    /// Sets the widget after the text, such as a switch or a value.
    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Sets the message published on a click, Enter or Space, which makes
    /// the tile a button.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or makes the tile static with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Sets whether the tile can be interacted with (true by default). A
    /// disabled tile is dimmed, ignores clicks and is skipped by focus
    /// traversal.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Sets the widget [`Id`], used to focus the tile programmatically and
    /// to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the space around the content (12 pixels vertically and 16
    /// horizontally by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<Tile<'a, Message>> for Element<'a, Message> {
    fn from(tile: Tile<'a, Message>) -> Self {
        let enabled = tile.enabled;
        let secondary = move |theme: &Theme| {
            let roles = theme.roles();

            if enabled {
                roles.on_surface_variant
            } else {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            }
        };

        let mut row = Row::new()
            .spacing(spacing::MD)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        if let Some(leading) = tile.leading {
            row = row.push(
                container(leading).style(move |theme: &Theme| container::Style {
                    text_color: Some(secondary(theme)),
                    ..container::Style::default()
                }),
            );
        }

        let mut text = Column::new().width(Length::Fill);

        if let Some(title) = tile.title {
            text = text.push(styled_text(title, typography::BODY_LARGE).style(
                move |theme: &Theme| iced::widget::text::Style {
                    color: Some(if enabled {
                        theme.roles().on_surface
                    } else {
                        theme
                            .roles()
                            .on_surface
                            .scale_alpha(DISABLED_CONTENT_OPACITY)
                    }),
                },
            ));
        }

        if let Some(subtitle) = tile.subtitle {
            text = text.push(styled_text(subtitle, typography::BODY_MEDIUM).style(
                move |theme: &Theme| iced::widget::text::Style {
                    color: Some(secondary(theme)),
                },
            ));
        }

        row = row.push(text);

        if let Some(trailing) = tile.trailing {
            row = row.push(trailing);
        }

        let on_press = tile.on_press.filter(|_| enabled);

        let Some(message) = on_press else {
            return container(row)
                .padding(tile.padding)
                .width(Length::Fill)
                .into();
        };

        let corners = tile.corners;

        let native = button(row)
            .padding(tile.padding)
            .width(Length::Fill)
            .on_press(message.clone())
            .style(move |theme: &Theme, status| {
                let roles = theme.roles();
                let background = match status {
                    button::Status::Hovered | button::Status::Pressed => {
                        blend(TRANSPARENT, roles.on_surface, HOVER_OVERLAY_OPACITY)
                    }
                    button::Status::Active | button::Status::Disabled => TRANSPARENT,
                };

                button::Style {
                    background: Some(background.into()),
                    text_color: roles.on_surface,
                    border: Border {
                        radius: corners,
                        ..Border::default()
                    },
                    ..button::Style::default()
                }
            });

        let mut wrapper = focusable(native)
            .radius(corners.top_left.max(corners.bottom_left))
            .on_activate(message);

        if let Some(id) = tile.id {
            wrapper = wrapper.id(id);
        }

        wrapper.into()
    }
}

/// A [`border_container`] filling the available width with `tiles` in a
/// column, separated by 1-pixel dividers. The tiles sit inside the border
/// and the first and last ones follow its rounded corners.
pub fn tile_list<'a, Message: Clone + 'a>(
    tiles: impl IntoIterator<Item = Tile<'a, Message>>,
) -> BorderContainer<'a, Message> {
    let tiles: Vec<Tile<'a, Message>> = tiles.into_iter().collect();
    let last = tiles.len().saturating_sub(1);
    let inner = radius::LARGE - 1.0;

    let mut column = Column::new().width(Length::Fill);

    for (index, mut tile) in tiles.into_iter().enumerate() {
        if index > 0 {
            column = column.push(rule::horizontal(1));
        }

        if index == 0 {
            tile.corners.top_left = inner;
            tile.corners.top_right = inner;
        }

        if index == last {
            tile.corners.bottom_left = inner;
            tile.corners.bottom_right = inner;
        }

        column = column.push(tile);
    }

    // The tiles sit inside the 1-pixel border, so their hover overlay never
    // covers it.
    border_container(column).padding(1).width(Length::Fill)
}

/// Content with a faint decoration behind it. See [`watermark`].
pub struct Watermark<'a, Message> {
    mark: Element<'a, Message>,
    content: Element<'a, Message>,
    align_x: Horizontal,
    align_y: Vertical,
    padding: Padding,
    opacity: f32,
}

/// Places a faint `mark`, such as a large icon in an empty page, behind
/// `content`, aligned to the end and centered vertically, 20 pixels from the
/// edges.
///
/// The mark covers the area `content` takes; to watermark a whole page,
/// give `content` the full size. Text and icons in the mark are drawn in
/// the theme content color at 10 % opacity. The mark is drawn below the
/// content, which receives the pointer first.
pub fn watermark<'a, Message>(
    mark: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
) -> Watermark<'a, Message> {
    Watermark {
        mark: mark.into(),
        content: content.into(),
        align_x: Horizontal::Right,
        align_y: Vertical::Center,
        padding: Padding::new(spacing::PAGE),
        opacity: WATERMARK_OPACITY,
    }
}

impl<'a, Message> Watermark<'a, Message> {
    /// Sets where the mark sits (end and center by default).
    #[must_use]
    pub fn align(mut self, horizontal: Horizontal, vertical: Vertical) -> Self {
        self.align_x = horizontal;
        self.align_y = vertical;
        self
    }

    /// Sets the space between the mark and the edges.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the opacity of the mark, clamped to 0 to 1.
    #[must_use]
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

impl<'a, Message: 'a> From<Watermark<'a, Message>> for Element<'a, Message> {
    fn from(watermark: Watermark<'a, Message>) -> Self {
        let opacity = watermark.opacity;

        let mark = container(watermark.mark)
            .padding(watermark.padding)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(watermark.align_x)
            .align_y(watermark.align_y)
            .style(move |theme: &Theme| container::Style {
                text_color: Some(theme.roles().on_surface.scale_alpha(opacity)),
                ..container::Style::default()
            });

        let size = watermark.content.as_widget().size();

        Stack::new()
            .width(size.width)
            .height(size.height)
            .push(watermark.content)
            .push_under(mark)
            .into()
    }
}
