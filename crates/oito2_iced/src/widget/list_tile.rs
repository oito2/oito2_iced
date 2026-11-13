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

//! List tiles: interactive rows with a title, an optional subtitle and
//! widgets on both sides, and the list tiles of a check box, radio or
//! switch, which a click anywhere on the row toggles.
//!
//! A tile with a press message takes part in focus traversal, shows the
//! focus ring around itself while focused and publishes the message on a
//! click, Enter or Space. It is at least 40 pixels tall (52 with a
//! subtitle) for a pointer and 56 (72) on a touch screen, following the
//! [density] when the tile is built.
//!
//! ```no_run
//! use oito2_iced::widget::{list_tile, toggles};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Open,
//!     Wifi(bool),
//! }
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, Message> {
//! iced::widget::column![
//!     list_tile::list_tile()
//!         .title("Network")
//!         .subtitle("Connected")
//!         .on_press(Message::Open),
//!     list_tile::switch_list_tile(toggles::switch(true).on_toggle(Message::Wifi))
//!         .title("Wi-Fi"),
//! ]
//! .into()
//! # }
//! ```

use iced::advanced::widget::Id;
use iced::widget::button::{Status, Style};
use iced::widget::text::{self, Fragment, IntoFragment};
use iced::widget::{Column, Row};
use iced::{Alignment, Background, Border, Color, Length, Padding, mouse};

use super::min_size::{Minimum, min_size};
use super::toggles::{Checkbox, Radio, Switch, Toggle};
use super::{focusable, styled_text};
use crate::density::{self, Density};
use crate::theme::{DISABLED_CONTENT_OPACITY, button, state_layer};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// Opacity of the tone laid over a row while the pointer is on it.
pub const ROW_HOVER_OPACITY: f32 = 0.04;

/// The style of a transparent clickable row with `radius` corners: the
/// `base` background, if any, with the hover tone over it while the pointer
/// is on the row and `enabled`, and `content` as the default text color.
pub(crate) fn row_style(
    theme: &Theme,
    status: Status,
    base: Option<Color>,
    content: Color,
    enabled: bool,
    radius: f32,
) -> Style {
    let hover = theme.roles().on_surface;
    let hovered = enabled && matches!(status, Status::Hovered | Status::Pressed);

    let background = match (base, hovered) {
        (Some(base), true) => Some(state_layer(base, hover, ROW_HOVER_OPACITY)),
        (None, true) => Some(hover.scale_alpha(ROW_HOVER_OPACITY)),
        (base, false) => base,
    };

    Style {
        background: background.map(Background::Color),
        text_color: content,
        border: Border::default().rounded(radius),
        ..Style::default()
    }
}

/// An interactive row. Built by [`list_tile`].
pub struct ListTile<'a, Message> {
    leading: Option<Element<'a, Message>>,
    title: Option<Fragment<'a>>,
    subtitle: Option<Fragment<'a>>,
    trailing: Option<Element<'a, Message>>,
    on_press: Option<Message>,
    activatable: bool,
    selected: bool,
    enabled: bool,
    center_title: bool,
    radius: f32,
    padding: Option<Padding>,
    spacing: f32,
    id: Option<Id>,
}

/// An empty list tile with rounded corners, to fill with
/// [`ListTile::title`], [`ListTile::subtitle`], [`ListTile::leading`] and
/// [`ListTile::trailing`].
///
/// The title is in the large body style and the subtitle in the medium one,
/// in the variant color; icons at the sides take the variant color. A
/// selected tile has the secondary container background.
pub fn list_tile<'a, Message>() -> ListTile<'a, Message> {
    ListTile {
        leading: None,
        title: None,
        subtitle: None,
        trailing: None,
        on_press: None,
        activatable: false,
        selected: false,
        enabled: true,
        center_title: false,
        radius: radius::MEDIUM,
        padding: None,
        spacing: spacing::MD,
        id: None,
    }
}

impl<'a, Message> ListTile<'a, Message> {
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

    /// Sets the widget before the title, usually an icon.
    #[must_use]
    pub fn leading(mut self, leading: impl Into<Element<'a, Message>>) -> Self {
        self.leading = Some(leading.into());
        self
    }

    /// Sets the widget after the title, such as a chevron.
    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Sets the message published on a click, Enter or Space. Without it
    /// the row is static: no hover, no focus.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or makes the row static with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Highlights the row as selected.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets whether the row responds to input (true by default); a disabled
    /// row is dimmed and skipped by focus traversal.
    #[must_use]
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Centers the title and the subtitle.
    #[must_use]
    pub fn center_title(mut self, center: bool) -> Self {
        self.center_title = center;
        self
    }

    /// Gives the row square corners, for rows inside a bordered list that
    /// already rounds them.
    #[must_use]
    pub fn square(mut self) -> Self {
        self.radius = 0.0;
        self
    }

    /// Sets the corner radius of the hover, selection and focus shapes
    /// (medium radius by default).
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the space around the content; medium horizontal and extra-small
    /// vertical spacing by default.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the space between the leading widget, the text and the trailing
    /// widget (medium spacing by default).
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the widget [`Id`], used to focus the row programmatically and to
    /// find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Keeps the row focusable while enabled even without a press message.
    fn activatable(mut self, activatable: bool) -> Self {
        self.activatable = activatable;
        self
    }
}

/// The minimum height of a list tile, padding included.
fn min_height(density: Density, has_subtitle: bool) -> f32 {
    match (density, has_subtitle) {
        (Density::Pointer, false) => 40.0,
        (Density::Pointer, true) => 52.0,
        (Density::Touch, false) => 56.0,
        (Density::Touch, true) => 72.0,
    }
}

impl<'a, Message: Clone + 'a> From<ListTile<'a, Message>> for Element<'a, Message> {
    fn from(tile: ListTile<'a, Message>) -> Self {
        let enabled = tile.enabled;
        let selected = tile.selected;
        let interactive = enabled && (tile.on_press.is_some() || tile.activatable);
        let on_press = tile.on_press.filter(|_| enabled);
        let padding = tile
            .padding
            .unwrap_or(Padding::from([spacing::XS, spacing::MD]));
        let min_height = min_height(density::get(), tile.subtitle.is_some());

        let foreground = move |theme: &Theme| {
            let roles = theme.roles();
            if !enabled {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            } else if selected {
                roles.on_secondary_container
            } else {
                roles.on_surface
            }
        };
        let secondary = move |theme: &Theme| {
            let roles = theme.roles();
            if !enabled {
                roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
            } else if selected {
                roles.on_secondary_container
            } else {
                roles.on_surface_variant
            }
        };

        let (align, text_align) = if tile.center_title {
            (Alignment::Center, text::Alignment::Center)
        } else {
            (Alignment::Start, text::Alignment::Default)
        };

        let mut texts = Column::new().width(Length::Fill).align_x(align);

        if let Some(title) = tile.title {
            texts = texts.push(
                styled_text(title, typography::BODY_LARGE)
                    .align_x(text_align)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(foreground(theme)),
                    }),
            );
        }

        if let Some(subtitle) = tile.subtitle {
            texts = texts.push(
                styled_text(subtitle, typography::BODY_MEDIUM)
                    .align_x(text_align)
                    .style(move |theme: &Theme| text::Style {
                        color: Some(secondary(theme)),
                    }),
            );
        }

        let content = Row::new()
            .push(tile.leading)
            .push(texts)
            .push(tile.trailing)
            .spacing(tile.spacing)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        let content = min_size(
            content,
            Minimum::None,
            Minimum::Fixed((min_height - padding.top - padding.bottom).max(0.0)),
        );

        let radius = tile.radius;
        let native = iced::widget::Button::new(content)
            .padding(padding)
            .width(Length::Fill)
            .on_press_maybe(on_press.clone())
            .class(button::Class::Custom(Box::new(move |theme, status| {
                let roles = theme.roles();
                let base = selected.then_some(roles.secondary_container);
                row_style(theme, status, base, secondary(theme), interactive, radius)
            })));

        // An interactive tile with nothing to publish, such as a selected
        // radio tile, still reads as clickable.
        let native: Element<'a, Message> = if interactive && on_press.is_none() {
            iced::widget::mouse_area(native)
                .interaction(mouse::Interaction::Pointer)
                .into()
        } else {
            native.into()
        };

        let mut wrapper = focusable(native).radius(radius).enabled(interactive);

        if let Some(message) = on_press {
            wrapper = wrapper.on_activate(message);
        }

        if let Some(id) = tile.id {
            wrapper = wrapper.id(id);
        }

        wrapper.into()
    }
}

/// The side of the control of a toggle list tile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ControlAffinity {
    /// Before the title.
    Leading,
    /// After the title.
    Trailing,
    /// The usual side of the control: leading for check boxes and radios,
    /// trailing for switches.
    #[default]
    Platform,
}

/// A list tile with a check box, radio or switch. Built by
/// [`checkbox_list_tile`], [`radio_list_tile`] and [`switch_list_tile`].
pub struct ToggleListTile<'a, Message> {
    toggle: Toggle<Message>,
    title: Option<Fragment<'a>>,
    subtitle: Option<Fragment<'a>>,
    secondary: Option<Element<'a, Message>>,
    affinity: ControlAffinity,
    padding: Option<Padding>,
    id: Option<Id>,
}

fn toggle_list_tile<'a, Message>(toggle: Toggle<Message>) -> ToggleListTile<'a, Message> {
    ToggleListTile {
        toggle,
        title: None,
        subtitle: None,
        secondary: None,
        affinity: ControlAffinity::Platform,
        padding: None,
        id: None,
    }
}

/// A list tile with `checkbox`; a click anywhere on the row, Enter or Space
/// toggles it, publishing the message of its
/// [`on_change`](Checkbox::on_change). The check box leads by default.
pub fn checkbox_list_tile<'a, Message>(
    checkbox: Checkbox<'a, Message>,
) -> ToggleListTile<'a, Message> {
    toggle_list_tile(checkbox.into_toggle())
}

/// A list tile with `radio`; a click anywhere on the row, Enter or Space
/// selects it, publishing the message of its
/// [`on_change`](Radio::on_change). The radio leads by default.
pub fn radio_list_tile<'a, T: PartialEq, Message>(
    radio: Radio<'a, T, Message>,
) -> ToggleListTile<'a, Message> {
    toggle_list_tile(radio.into_toggle())
}

/// A list tile with `switch`; a click anywhere on the row, Enter or Space
/// toggles it, publishing the message of its
/// [`on_toggle`](Switch::on_toggle). The switch trails by default.
pub fn switch_list_tile<'a, Message>(switch: Switch<'a, Message>) -> ToggleListTile<'a, Message> {
    toggle_list_tile(switch.into_toggle())
}

impl<'a, Message> ToggleListTile<'a, Message> {
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

    /// Sets the widget on the side opposite the control, such as an icon.
    #[must_use]
    pub fn secondary(mut self, secondary: impl Into<Element<'a, Message>>) -> Self {
        self.secondary = Some(secondary.into());
        self
    }

    /// Sets the side of the control.
    #[must_use]
    pub fn control_affinity(mut self, affinity: ControlAffinity) -> Self {
        self.affinity = affinity;
        self
    }

    /// Sets the space around the content; medium horizontal (large on a
    /// touch screen) and extra-small vertical spacing by default.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets the widget [`Id`] of the row.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<ToggleListTile<'a, Message>> for Element<'a, Message> {
    fn from(tile: ToggleListTile<'a, Message>) -> Self {
        let leading = match tile.affinity {
            ControlAffinity::Leading => true,
            ControlAffinity::Trailing => false,
            ControlAffinity::Platform => tile.toggle.leads(),
        };
        let enabled = tile.toggle.enabled;
        let message = tile.toggle.on_activate.clone();
        let control: Element<'a, Message> = tile.toggle.passive().into();
        let horizontal = match density::get() {
            Density::Pointer => spacing::MD,
            Density::Touch => spacing::LG,
        };

        let (start, end) = if leading {
            (Some(control), tile.secondary)
        } else {
            (tile.secondary, Some(control))
        };

        let mut row = list_tile()
            .on_press_maybe(message)
            .activatable(true)
            .enabled(enabled)
            .padding(
                tile.padding
                    .unwrap_or(Padding::from([spacing::XS, horizontal])),
            );

        row.leading = start;
        row.trailing = end;
        row.title = tile.title;
        row.subtitle = tile.subtitle;
        row.id = tile.id;

        row.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tiles_grow_with_a_subtitle_and_on_touch_screens() {
        assert_eq!(min_height(Density::Pointer, false), 40.0);
        assert_eq!(min_height(Density::Pointer, true), 52.0);
        assert_eq!(min_height(Density::Touch, false), 56.0);
        assert_eq!(min_height(Density::Touch, true), 72.0);
    }

    #[test]
    fn rows_show_the_hover_tone_only_while_enabled() {
        let theme = Theme::light();
        let hovered = row_style(&theme, Status::Hovered, None, Color::BLACK, true, 0.0);
        let static_row = row_style(&theme, Status::Hovered, None, Color::BLACK, false, 0.0);

        assert!(hovered.background.is_some());
        assert!(static_row.background.is_none());
    }
}
