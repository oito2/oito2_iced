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

//! Info boxes and info badges tinted by the kind of their message, and
//! banners: bordered surfaces that can be selected and activated.
//!
//! ```no_run
//! use oito2_iced::widget::info::{self, InfoType};
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! # let _: oito2_iced::Element<'a, ()> =
//! info::info_box(InfoType::Warning)
//!     .title("Battery low")
//!     .subtitle("Connect the charger.")
//!     .into();
//! info::info_badge(InfoType::Success, "Beta").into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::widget::Id;
use iced::widget::text::IntoFragment;
use iced::widget::{Column, Row, container};
use iced::{Alignment, Border, Color, Length, Padding};

use super::selectable::{Surface, SurfaceColors};
use super::styled_text;
#[cfg(feature = "svg")]
use crate::icon::{Icon, icon, lucide};
use crate::l10n::Localizations;
use crate::theme::{ColorRoles, contrast, state_layer};
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme};

/// Opacity of the type color over the surface behind an [`InfoBox`].
pub const BOX_TINT_OPACITY: f32 = 0.08;

/// Opacity of the type color over the surface behind an info badge.
pub const BADGE_TINT_OPACITY: f32 = 0.14;

/// Opacity of the foreground color in the border of an [`InfoBox`].
const BOX_BORDER_OPACITY: f32 = 0.5;

/// The kind of message shown by an [`InfoBox`] or an info badge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfoType {
    /// Neutral information.
    Information,
    /// A successful or positive outcome.
    Success,
    /// Something that needs attention.
    Warning,
    /// An error or a destructive consequence.
    Danger,
    /// Something the user should notice, in the accent color.
    Important,
}

impl InfoType {
    /// Every kind, in declaration order.
    pub const ALL: [InfoType; 5] = [
        InfoType::Information,
        InfoType::Success,
        InfoType::Warning,
        InfoType::Danger,
        InfoType::Important,
    ];

    /// The icon that represents this kind.
    #[cfg(feature = "svg")]
    pub fn icon(self) -> Icon {
        match self {
            InfoType::Information => lucide::INFO,
            InfoType::Success => lucide::CIRCLE_CHECK,
            InfoType::Warning => lucide::TRIANGLE_EXCLAMATION_POINT,
            InfoType::Danger => lucide::CIRCLE_X,
            InfoType::Important => lucide::STAR,
        }
    }

    /// The localized name of this kind, such as "Warning".
    pub fn label(self, localizations: &Localizations) -> &'static str {
        match self {
            InfoType::Information => localizations.info_information,
            InfoType::Success => localizations.info_success,
            InfoType::Warning => localizations.info_warning,
            InfoType::Danger => localizations.info_danger,
            InfoType::Important => localizations.info_important,
        }
    }

    /// The color of this kind in `roles`: info, success, warning, error
    /// text or primary text.
    pub fn color(self, roles: &ColorRoles) -> Color {
        match self {
            InfoType::Information => roles.semantic.info,
            InfoType::Success => roles.semantic.success,
            InfoType::Warning => roles.semantic.warning,
            InfoType::Danger => roles.semantic.error_text,
            InfoType::Important => roles.semantic.primary_text,
        }
    }

    /// The tinted background and the readable foreground of this kind:
    /// the kind color at `tint` opacity over the surface, and the kind
    /// color adjusted to reach the 4.5:1 text contrast on that background.
    pub fn colors(self, roles: &ColorRoles, tint: f32) -> (Color, Color) {
        let color = self.color(roles);
        let background = state_layer(roles.surface, color, tint);

        (
            background,
            contrast::ensure(color, &[background], contrast::TEXT_AA),
        )
    }
}

/// A message box tinted by its kind. See [`info_box`].
pub struct InfoBox<'a, Message> {
    kind: InfoType,
    title: Option<Cow<'a, str>>,
    subtitle: Option<Cow<'a, str>>,
    content: Option<Element<'a, Message>>,
    trailing: Option<Element<'a, Message>>,
    icon: Option<Element<'a, Message>>,
}

/// A message box tinted by `kind`, filling the available width: the kind
/// color at 8 % over the surface, a border in the readable kind color at
/// half opacity and the medium radius. It shows the kind icon, a title in
/// the kind color, a subtitle, extra content and a trailing widget.
///
/// Without the `svg` feature, the box has no kind icon unless one is set
/// with [`InfoBox::icon`].
pub fn info_box<'a, Message>(kind: InfoType) -> InfoBox<'a, Message> {
    InfoBox {
        kind,
        title: None,
        subtitle: None,
        content: None,
        trailing: None,
        icon: None,
    }
}

impl<'a, Message> InfoBox<'a, Message> {
    /// Sets the headline, drawn in the kind color.
    #[must_use]
    pub fn title(mut self, title: impl IntoFragment<'a>) -> Self {
        self.title = Some(title.into_fragment());
        self
    }

    /// Sets the message text.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl IntoFragment<'a>) -> Self {
        self.subtitle = Some(subtitle.into_fragment());
        self
    }

    /// Sets extra content below the subtitle.
    #[must_use]
    pub fn content(mut self, content: impl Into<Element<'a, Message>>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// Sets a widget at the end, such as a close or action button.
    #[must_use]
    pub fn trailing(mut self, trailing: impl Into<Element<'a, Message>>) -> Self {
        self.trailing = Some(trailing.into());
        self
    }

    /// Replaces the kind icon; text and icons in it are drawn in the kind
    /// color.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<'a, Message: 'a> From<InfoBox<'a, Message>> for Element<'a, Message> {
    fn from(info: InfoBox<'a, Message>) -> Self {
        let kind = info.kind;
        let foreground = move |theme: &Theme| kind.colors(theme.roles(), BOX_TINT_OPACITY).1;

        #[cfg(feature = "svg")]
        let leading: Option<Element<'a, Message>> = Some(info.icon.unwrap_or_else(|| {
            icon(kind.icon())
                .size(crate::tokens::sizes::ICON_SIZE)
                .into()
        }));
        #[cfg(not(feature = "svg"))]
        let leading = info.icon;

        let mut row = Row::new().align_y(Alignment::Start).width(Length::Fill);

        if let Some(leading) = leading {
            row = row.push(
                container(leading)
                    .padding(Padding {
                        right: spacing::MD,
                        ..Padding::ZERO
                    })
                    .style(move |theme: &Theme| container::Style {
                        text_color: Some(foreground(theme)),
                        ..container::Style::default()
                    }),
            );
        }

        let mut text = Column::new().spacing(spacing::XS).width(Length::Fill);

        if let Some(title) = info.title {
            text = text.push(styled_text(title, typography::TITLE_SMALL).style(
                move |theme: &Theme| iced::widget::text::Style {
                    color: Some(foreground(theme)),
                },
            ));
        }

        if let Some(subtitle) = info.subtitle {
            text = text.push(styled_text(subtitle, typography::BODY_MEDIUM));
        }

        if let Some(content) = info.content {
            text = text.push(content);
        }

        row = row.push(text);

        if let Some(trailing) = info.trailing {
            row = row.push(container(trailing).padding(Padding {
                left: spacing::SM,
                ..Padding::ZERO
            }));
        }

        container(row)
            .padding(spacing::MD)
            .width(Length::Fill)
            .style(move |theme: &Theme| {
                let roles = theme.roles();
                let (background, foreground) = kind.colors(roles, BOX_TINT_OPACITY);

                container::Style {
                    text_color: Some(roles.on_surface),
                    background: Some(background.into()),
                    border: Border::default()
                        .color(foreground.scale_alpha(BOX_BORDER_OPACITY))
                        .width(1.0)
                        .rounded(radius::MEDIUM),
                    ..container::Style::default()
                }
            })
            .into()
    }
}

/// A small pill-shaped label tinted by `kind`, for statuses and counters:
/// the kind color at 14 % over the surface behind `label` in the label
/// medium style, semibold, in the readable kind color.
pub fn info_badge<'a, Message: 'a>(
    kind: InfoType,
    label: impl IntoFragment<'a>,
) -> Element<'a, Message> {
    let style = typography::LABEL_MEDIUM;
    let pill = (style.line_height + 2.0 * spacing::XXS) / 2.0;

    let label = styled_text(label, style).font(crate::font::inter(iced::font::Weight::Semibold));

    container(label)
        .padding([spacing::XXS, spacing::SM])
        .style(move |theme: &Theme| {
            let (background, foreground) = kind.colors(theme.roles(), BADGE_TINT_OPACITY);

            container::Style {
                text_color: Some(foreground),
                background: Some(background.into()),
                border: Border::default().rounded(pill),
                ..container::Style::default()
            }
        })
        .into()
}

/// The content of a [`Banner`].
enum BannerContent<'a, Message> {
    /// Any element.
    Custom(Element<'a, Message>),
    /// An icon before a title and an optional subtitle.
    Tile {
        title: Cow<'a, str>,
        subtitle: Option<Cow<'a, str>>,
        icon: Option<Element<'a, Message>>,
    },
}

/// A bordered surface that can be selected and activated. See [`banner`].
pub struct Banner<'a, Message> {
    content: BannerContent<'a, Message>,
    on_press: Option<Message>,
    on_hover: Option<Box<dyn Fn(bool) -> Message + 'a>>,
    selected: bool,
    color: Option<Color>,
    tint: Option<Color>,
    id: Option<Id>,
    padding: Padding,
    width: Length,
}

/// A surface around `content` with a border in the divider color, the
/// large radius and 16 pixels of padding. While enabled, hovering turns the
/// border into the control border color; while selected, the surface takes
/// the selection tint and a 2-pixel border in the indicator color.
///
/// With [`on_press`](Banner::on_press) the banner is a button: it takes the
/// focus as a whole and publishes its message on a click, Enter or Space.
pub fn banner<'a, Message>(content: impl Into<Element<'a, Message>>) -> Banner<'a, Message> {
    banner_with(BannerContent::Custom(content.into()))
}

/// A [`banner`] holding `title` in the title medium style, with an
/// optional [icon](Banner::icon) before it and an optional
/// [subtitle](Banner::subtitle) below it.
pub fn banner_tile<'a, Message>(title: impl IntoFragment<'a>) -> Banner<'a, Message> {
    banner_with(BannerContent::Tile {
        title: title.into_fragment(),
        subtitle: None,
        icon: None,
    })
}

fn banner_with<Message>(content: BannerContent<'_, Message>) -> Banner<'_, Message> {
    Banner {
        content,
        on_press: None,
        on_hover: None,
        selected: false,
        color: None,
        tint: None,
        id: None,
        padding: Padding::new(spacing::LG),
        width: Length::Shrink,
    }
}

impl<'a, Message> Banner<'a, Message> {
    /// Sets the message published on a click, Enter or Space.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Sets the press message, or makes the banner static with `None`.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Publishes `on_hover(true)` when the pointer enters the banner and
    /// `on_hover(false)` when it leaves, even on a static banner.
    #[must_use]
    pub fn on_hover(mut self, on_hover: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_hover = Some(Box::new(on_hover));
        self
    }

    /// Sets whether the banner is selected.
    #[must_use]
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the background, instead of the theme surface.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Blends `tint` at 8 % over the background.
    #[must_use]
    pub fn tint(mut self, tint: Color) -> Self {
        self.tint = Some(tint);
        self
    }

    /// Sets the widget [`Id`], used to focus the banner programmatically
    /// and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the space around the content (16 pixels by default).
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the subtitle of a [`banner_tile`]; has no effect on a
    /// [`banner`] with custom content.
    #[must_use]
    pub fn subtitle(mut self, subtitle: impl IntoFragment<'a>) -> Self {
        if let BannerContent::Tile { subtitle: slot, .. } = &mut self.content {
            *slot = Some(subtitle.into_fragment());
        }
        self
    }

    /// Sets the icon of a [`banner_tile`], placed before the text; has no
    /// effect on a [`banner`] with custom content.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        if let BannerContent::Tile { icon: slot, .. } = &mut self.content {
            *slot = Some(icon.into());
        }
        self
    }
}

impl<'a, Message: Clone + 'a> From<Banner<'a, Message>> for Element<'a, Message> {
    fn from(banner: Banner<'a, Message>) -> Self {
        let content = match banner.content {
            BannerContent::Custom(content) => content,
            BannerContent::Tile {
                title,
                subtitle,
                icon,
            } => {
                let mut text = Column::new()
                    .spacing(spacing::XS)
                    .width(Length::Fill)
                    .push(styled_text(title, typography::TITLE_MEDIUM));

                if let Some(subtitle) = subtitle {
                    text = text.push(
                        styled_text(subtitle, typography::BODY_MEDIUM)
                            .class(crate::theme::text::Class::Variant),
                    );
                }

                let mut row = Row::new().spacing(spacing::LG).align_y(Alignment::Start);

                if let Some(icon) = icon {
                    row = row.push(icon);
                }

                row.push(text).into()
            }
        };

        Surface {
            content,
            selected: banner.selected,
            on_press: banner.on_press,
            on_hover: banner.on_hover,
            id: banner.id,
            padding: banner.padding,
            radius: radius::LARGE,
            width: banner.width,
            height: Length::Shrink,
            colors: SurfaceColors::Banner {
                color: banner.color,
                tint: banner.tint,
            },
        }
        .into_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_kind_is_readable_on_its_tint() {
        for theme in [Theme::light(), Theme::dark()] {
            for kind in InfoType::ALL {
                for tint in [BOX_TINT_OPACITY, BADGE_TINT_OPACITY] {
                    let (background, foreground) = kind.colors(theme.roles(), tint);

                    assert!(
                        contrast::ratio(foreground, background) >= contrast::TEXT_AA - 0.01,
                        "{kind:?} at {tint}"
                    );
                    assert!(
                        contrast::ratio(theme.roles().on_surface, background) >= contrast::TEXT_AA,
                        "{kind:?} body at {tint}"
                    );
                }
            }
        }
    }

    #[test]
    fn every_kind_has_a_localized_label() {
        let en = Localizations::en();

        assert_eq!(InfoType::Warning.label(&en), "Warning");
        assert_eq!(InfoType::Danger.label(&en), "Error");
    }
}
