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

//! Selectable containers: a surface that shows whether it is selected,
//! takes the keyboard focus as a whole and is activated by a click, Enter
//! or Space.
//!
//! ```no_run
//! use oito2_iced::widget::selectable;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     DarkMode(bool),
//! }
//!
//! # fn view<'a>(dark: bool) -> oito2_iced::Element<'a, Message> {
//! selectable::selectable_container("Dark", dark)
//!     .on_toggle(Message::DarkMode)
//!     .into()
//! # }
//! ```

use iced::advanced::widget::Id;
use iced::widget::{button, mouse_area};
use iced::{Border, Color, Length, Padding};

use super::focusable;
use crate::theme::TRANSPARENT;
use crate::tokens::{radius, sizes, spacing};
use crate::{Element, Theme};

/// Opacity of the default selection tint over the surface.
pub const SELECTION_TINT_OPACITY: f32 = 0.12;

/// Opacity of the content color painted over a hovered surface.
pub(crate) const HOVER_OVERLAY_OPACITY: f32 = 0.04;

/// A container that shows whether it is selected. See
/// [`selectable_container`].
pub struct SelectableContainer<'a, Message> {
    content: Element<'a, Message>,
    selected: bool,
    on_press: Option<Message>,
    id: Option<Id>,
    padding: Padding,
    radius: f32,
    selection_color: Option<Color>,
    width: Length,
    height: Length,
}

/// A transparent container around `content` that, while `selected`, takes
/// the selection tint (the primary color at 12 % opacity) and a 2-pixel
/// border in the indicator color. While enabled, hovering shows a thin
/// border in the divider color.
///
/// With [`on_press`](SelectableContainer::on_press) or
/// [`on_toggle`](SelectableContainer::on_toggle) the whole container is a
/// button: it takes the focus as a whole and publishes its message on a
/// click, Enter or Space. Without them it is static and skipped by focus
/// traversal.
pub fn selectable_container<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    selected: bool,
) -> SelectableContainer<'a, Message> {
    SelectableContainer {
        content: content.into(),
        selected,
        on_press: None,
        id: None,
        padding: Padding::new(spacing::SM),
        radius: radius::LARGE,
        selection_color: None,
        width: Length::Shrink,
        height: Length::Shrink,
    }
}

impl<'a, Message> SelectableContainer<'a, Message> {
    /// Sets the message published on a click, Enter or Space.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or makes the container static with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Publishes `on_toggle` with the opposite of the current selection on
    /// a click, Enter or Space.
    #[must_use]
    pub fn on_toggle(mut self, on_toggle: impl FnOnce(bool) -> Message) -> Self {
        self.on_press = Some(on_toggle(!self.selected));
        self
    }

    /// Sets the widget [`Id`], used to focus the container programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the space around the content (8 pixels by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the corner radius (the large radius by default).
    #[must_use]
    pub fn radius(mut self, radius: f32) -> Self {
        self.radius = radius;
        self
    }

    /// Sets the tint painted over the surface while selected, instead of
    /// the primary color at 12 % opacity.
    #[must_use]
    pub fn selection_color(mut self, color: Color) -> Self {
        self.selection_color = Some(color);
        self
    }

    /// Sets the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }
}

impl<'a, Message: Clone + 'a> From<SelectableContainer<'a, Message>> for Element<'a, Message> {
    fn from(container: SelectableContainer<'a, Message>) -> Self {
        Surface {
            content: container.content,
            selected: container.selected,
            on_press: container.on_press,
            on_hover: None,
            id: container.id,
            padding: container.padding,
            radius: container.radius,
            width: container.width,
            height: container.height,
            colors: SurfaceColors::Selectable {
                selection: container.selection_color,
            },
        }
        .into_element()
    }
}

/// The colors of an interactive [`Surface`].
pub(crate) enum SurfaceColors {
    /// Transparent with a transparent border that turns into the divider
    /// color on hover.
    Selectable {
        /// The tint painted while selected; the primary color at 12 %
        /// opacity when `None`.
        selection: Option<Color>,
    },
    /// A surface with a divider border that turns into the control border
    /// color on hover.
    Banner {
        /// The background; the theme surface when `None`.
        color: Option<Color>,
        /// A color blended at 8 % over the background.
        tint: Option<Color>,
    },
}

/// Opacity of a banner tint over its background.
pub(crate) const BANNER_TINT_OPACITY: f32 = 0.08;

/// A surface that shows its selection, its hover state and the focus ring,
/// shared by selectable containers and banners.
pub(crate) struct Surface<'a, Message> {
    pub(crate) content: Element<'a, Message>,
    pub(crate) selected: bool,
    pub(crate) on_press: Option<Message>,
    pub(crate) on_hover: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    pub(crate) id: Option<Id>,
    pub(crate) padding: Padding,
    pub(crate) radius: f32,
    pub(crate) width: Length,
    pub(crate) height: Length,
    pub(crate) colors: SurfaceColors,
}

/// The resolved appearance of a [`Surface`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct SurfaceStyle {
    pub(crate) background: Color,
    pub(crate) border: Color,
    pub(crate) border_width: f32,
}

impl SurfaceColors {
    /// The appearance in `theme` for the given selection, hover state and
    /// interactivity.
    pub(crate) fn resolve(
        &self,
        theme: &Theme,
        selected: bool,
        hovered: bool,
        interactive: bool,
    ) -> SurfaceStyle {
        let roles = theme.roles();

        let (base, selection, border, hover_border) = match *self {
            SurfaceColors::Selectable { selection } => {
                (None, selection, TRANSPARENT, roles.outline_variant)
            }
            SurfaceColors::Banner { color, tint } => {
                let base = color.unwrap_or(roles.surface);
                let base = match tint {
                    Some(tint) => blend(base, tint, BANNER_TINT_OPACITY),
                    None => base,
                };

                (Some(base), None, roles.outline_variant, roles.outline)
            }
        };

        let background = if selected {
            let selection = selection.unwrap_or(roles.primary.scale_alpha(SELECTION_TINT_OPACITY));

            blend(base.unwrap_or(roles.surface), selection, 1.0)
        } else {
            base.unwrap_or(TRANSPARENT)
        };

        let hovered = hovered && interactive;

        let background = if hovered {
            blend(background, roles.on_surface, HOVER_OVERLAY_OPACITY)
        } else {
            background
        };

        let (border, border_width) = if selected {
            (roles.indicator(), sizes::FOCUS_BORDER_WIDTH)
        } else if hovered {
            (hover_border, 1.0)
        } else {
            (border, 1.0)
        };

        SurfaceStyle {
            background,
            border,
            border_width,
        }
    }
}

/// Paints `layer` at `opacity` (times its own alpha) over `base`. A fully
/// transparent `base` gives `layer` with the combined alpha.
pub(crate) fn blend(base: Color, layer: Color, opacity: f32) -> Color {
    if base.a <= 0.0 {
        layer.scale_alpha(opacity)
    } else {
        let alpha = opacity * layer.a;
        let a = alpha + base.a * (1.0 - alpha);

        Color {
            r: (layer.r * alpha + base.r * base.a * (1.0 - alpha)) / a,
            g: (layer.g * alpha + base.g * base.a * (1.0 - alpha)) / a,
            b: (layer.b * alpha + base.b * base.a * (1.0 - alpha)) / a,
            a,
        }
    }
}

impl<'a, Message: Clone + 'a> Surface<'a, Message> {
    /// Builds the element: a button styled by the colors, wrapped for focus
    /// and, with a hover callback, in a mouse area.
    pub(crate) fn into_element(self) -> Element<'a, Message> {
        let Surface {
            content,
            selected,
            on_press,
            on_hover,
            id,
            padding,
            radius,
            width,
            height,
            colors,
        } = self;

        let interactive = on_press.is_some();

        let native = button(content)
            .padding(padding)
            .width(width)
            .height(height)
            .on_press_maybe(on_press.clone())
            .style(move |theme: &Theme, status| {
                let hovered = matches!(status, button::Status::Hovered | button::Status::Pressed);
                let style = colors.resolve(theme, selected, hovered, interactive);

                button::Style {
                    background: Some(style.background.into()),
                    text_color: theme.roles().on_surface,
                    border: Border::default()
                        .color(style.border)
                        .width(style.border_width)
                        .rounded(radius),
                    ..button::Style::default()
                }
            });

        let mut wrapper = focusable(native).radius(radius).enabled(interactive);

        if let Some(message) = on_press {
            wrapper = wrapper.on_activate(message);
        }

        if let Some(id) = id {
            wrapper = wrapper.id(id);
        }

        match on_hover {
            Some(on_hover) => mouse_area(wrapper)
                .on_enter(on_hover(true))
                .on_exit(on_hover(false))
                .into(),
            None => wrapper.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blending_over_a_transparent_base_keeps_the_layer_color() {
        let layer = Color::from_rgb(0.2, 0.4, 0.6);

        assert_eq!(blend(TRANSPARENT, layer, 0.12), layer.scale_alpha(0.12));
    }

    #[test]
    fn blending_over_an_opaque_base_matches_the_state_layer() {
        let base = Color::from_rgb(1.0, 1.0, 1.0);
        let layer = Color::from_rgb(0.0, 0.0, 0.0);
        let blended = blend(base, layer, 0.25);
        let expected = crate::theme::state_layer(base, layer, 0.25);

        assert!((blended.r - expected.r).abs() < 1e-5);
        assert!((blended.a - 1.0).abs() < 1e-5);
    }

    #[test]
    fn a_selected_surface_has_the_indicator_border() {
        let theme = Theme::light();
        let colors = SurfaceColors::Selectable { selection: None };

        let selected = colors.resolve(&theme, true, false, true);
        assert_eq!(selected.border, theme.roles().indicator());
        assert_eq!(selected.border_width, sizes::FOCUS_BORDER_WIDTH);

        let idle = colors.resolve(&theme, false, false, true);
        assert_eq!(idle.border, TRANSPARENT);
        assert_eq!(idle.background, TRANSPARENT);

        let hovered = colors.resolve(&theme, false, true, true);
        assert_eq!(hovered.border, theme.roles().outline_variant);

        let static_hovered = colors.resolve(&theme, false, true, false);
        assert_eq!(static_hovered.border, TRANSPARENT);
    }

    #[test]
    fn a_banner_strengthens_its_border_on_hover() {
        let theme = Theme::light();
        let colors = SurfaceColors::Banner {
            color: None,
            tint: None,
        };

        let idle = colors.resolve(&theme, false, false, true);
        assert_eq!(idle.border, theme.roles().outline_variant);
        assert_eq!(idle.background, theme.roles().surface);

        let hovered = colors.resolve(&theme, false, true, true);
        assert_eq!(hovered.border, theme.roles().outline);
    }
}
