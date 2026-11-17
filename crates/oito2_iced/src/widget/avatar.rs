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

//! Avatars and avatar groups.
//!
//! An [`avatar`] is a picture of a person or thing: with the crate
//! feature `image`, a raster picture drawn over the fallback; otherwise the
//! [`initials`] of its name on an accent color picked from the
//! name, or a person icon when there is no name. A [`Status`] adds a
//! presence dot at the bottom end, and [`Shape`] makes it a circle or a
//! rounded square.
//!
//! A [`group`] lays avatars out in a row that overlaps, each with a ring
//! of the page background, and shows a "+N" bubble for the avatars past
//! [`AvatarGroup::max`].
//!
//! [`description`] and [`more_description`] give the localized text that
//! describes an avatar and the bubble, for tooltips or assistive text.
//!
//! ```no_run
//! use oito2_iced::widget::avatar::{self, Status};
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! avatar::group(["Ana Souza", "Bruno Lima", "Caio", "Dani", "Eva"].map(|name| {
//!     avatar::avatar().name(name).size(32.0)
//! }))
//! .max(3)
//! .size(32.0)
//! .into()
//! # }
//! # fn single<'a>() -> oito2_iced::Element<'a, ()> {
//! avatar::avatar().name("Ana Souza").status(Status::Online).into()
//! # }
//! ```

use iced::advanced::Renderer as _;
use iced::advanced::graphics::geometry::{self, Frame, LineCap, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::renderer::{self, Quad};
use iced::advanced::text::{self, Renderer as _};
use iced::advanced::widget::{Operation, Tree, Widget};
use iced::font::Weight;
use iced::widget::{Stack, container};
use iced::{
    Background, Border, Color, Element as IcedElement, Length, Padding, Pixels, Point, Radians,
    Rectangle, Size, mouse,
};

use super::styled_text;
use crate::l10n::Localizations;
use crate::theme::contrast;
use crate::tokens::{Accent, typography};
use crate::{Element, Theme};

/// The default side of an [`Avatar`].
pub const DEFAULT_SIZE: f32 = 40.0;

/// The default side of the avatars of an [`AvatarGroup`].
pub const DEFAULT_GROUP_SIZE: f32 = 32.0;

/// The default number of avatars an [`AvatarGroup`] shows.
pub const DEFAULT_MAX: usize = 4;

/// The default fraction of each avatar of a group covered by the next one.
pub const DEFAULT_OVERLAP: f32 = 0.3;

/// The fraction of the side a square avatar rounds its corners by (a
/// literal in the OITO2 design).
const SQUARE_RADIUS: f32 = 0.22;

/// The width of the ring around each avatar of a group (a literal in the
/// OITO2 design).
const RING_WIDTH: f32 = 2.0;

/// The fraction of the side the initials take (a literal in the OITO2
/// design).
const INITIALS_SCALE: f32 = 0.38;

/// The fraction of the side the fallback icon takes (a literal in the OITO2
/// design).
const ICON_SCALE: f32 = 0.55;

/// The fraction of the side the status dot takes, and its smallest and
/// largest sides (literals in the OITO2 design).
const DOT_SCALE: (f32, f32, f32) = (0.28, 8.0, 16.0);

/// The fraction of the dot side its ring takes (a literal in the OITO2
/// design).
const DOT_RING: f32 = 0.18;

/// A presence status shown as a dot on an [`Avatar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Status {
    /// Available.
    Online,
    /// Away from the device.
    Away,
    /// Not to be disturbed.
    Busy,
    /// Not connected.
    Offline,
}

impl Status {
    /// The localized name of the status, such as "Busy".
    #[must_use]
    pub fn label(self, localizations: &Localizations) -> &'static str {
        match self {
            Status::Online => localizations.status_online,
            Status::Away => localizations.status_away,
            Status::Busy => localizations.status_busy,
            Status::Offline => localizations.status_offline,
        }
    }

    /// The color of the dot in `theme`.
    fn color(self, theme: &Theme) -> Color {
        let roles = theme.roles();

        match self {
            Status::Online => roles.semantic.success,
            Status::Away => roles.semantic.warning,
            Status::Busy => roles.semantic.error_text,
            Status::Offline => roles.outline,
        }
    }
}

/// The outline of an [`Avatar`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Shape {
    /// A circle.
    #[default]
    Circle,
    /// A rounded square.
    Square,
}

impl Shape {
    /// The corner radius of a box of `side` with this shape.
    fn radius(self, side: f32) -> f32 {
        match self {
            Shape::Circle => side / 2.0,
            Shape::Square => side * SQUARE_RADIUS,
        }
    }
}

/// Returns the initials of `name`: the first letters of its first and last
/// words, in upper case, or an empty string when `name` is blank.
///
/// A letter keeps the combining marks, variation selectors, skin tone
/// modifiers and zero-width-joined characters that follow it, so
/// decomposed accents and emoji sequences stay whole.
#[must_use]
pub fn initials(name: &str) -> String {
    let mut words = name.split_whitespace();
    let Some(first) = words.next() else {
        return String::new();
    };
    let mut letters = first_letter(first);

    if let Some(last) = words.last() {
        letters.push_str(&first_letter(last));
    }

    letters.to_uppercase()
}

/// The first user-perceived character of `word`.
fn first_letter(word: &str) -> String {
    let mut letter = String::new();
    let mut chars = word.chars().peekable();
    let mut regional = 0;

    while let Some(character) = chars.next() {
        if !letter.is_empty() && !extends(character) {
            let pair = is_regional(character) && regional == 1;

            if !pair {
                break;
            }
        }

        if is_regional(character) {
            regional += 1;
        }

        letter.push(character);

        if character == '\u{200D}'
            && let Some(joined) = chars.next()
        {
            letter.push(joined);
        }
    }

    letter
}

/// Whether `character` belongs to the character before it: a combining
/// mark, a variation selector, a skin tone modifier, a joiner or a tag.
fn extends(character: char) -> bool {
    matches!(
        character,
        '\u{0300}'..='\u{036F}'
            | '\u{1AB0}'..='\u{1AFF}'
            | '\u{1DC0}'..='\u{1DFF}'
            | '\u{20D0}'..='\u{20FF}'
            | '\u{FE00}'..='\u{FE0F}'
            | '\u{FE20}'..='\u{FE2F}'
            | '\u{200D}'
            | '\u{1F3FB}'..='\u{1F3FF}'
            | '\u{E0020}'..='\u{E007F}'
            | '\u{E0100}'..='\u{E01EF}'
    )
}

/// Whether `character` is a regional indicator, two of which make a flag.
fn is_regional(character: char) -> bool {
    matches!(character, '\u{1F1E6}'..='\u{1F1FF}')
}

/// The accent color picked for `name` by a stable hash of its UTF-16 code
/// units.
#[must_use]
pub fn color_for(name: &str) -> Color {
    let hash = name.encode_utf16().fold(0_u64, |hash, unit| {
        (hash * 31 + u64::from(unit)) & 0x7fff_ffff
    });
    let index = usize::try_from(hash % Accent::ALL.len() as u64).unwrap_or(0);

    Accent::ALL[index].color()
}

/// The localized description of an avatar: its name and status joined by
/// commas, such as "Ana Souza, Busy".
#[must_use]
pub fn description(
    name: Option<&str>,
    status: Option<Status>,
    localizations: &Localizations,
) -> String {
    name.into_iter()
        .chain(status.map(|status| status.label(localizations)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The localized description of the "+N" bubble of a group, such as
/// "2 more".
#[must_use]
pub fn more_description(hidden: usize, localizations: &Localizations) -> String {
    localizations.more_count(i64::try_from(hidden).unwrap_or(i64::MAX))
}

/// A picture of a person or thing. See [`avatar`].
#[derive(Debug, Clone)]
pub struct Avatar {
    name: Option<String>,
    size: f32,
    shape: Shape,
    status: Option<Status>,
    background: Option<Color>,
    #[cfg(feature = "svg")]
    icon: Option<crate::icon::Icon>,
    #[cfg(feature = "image")]
    image: Option<iced::widget::image::Handle>,
}

/// An avatar of [`DEFAULT_SIZE`] showing a person icon until it gets a
/// [name](Avatar::name).
#[must_use]
pub fn avatar() -> Avatar {
    Avatar {
        name: None,
        size: DEFAULT_SIZE,
        shape: Shape::Circle,
        status: None,
        background: None,
        #[cfg(feature = "svg")]
        icon: None,
        #[cfg(feature = "image")]
        image: None,
    }
}

impl Avatar {
    /// Sets the name the initials and the fallback color come from.
    #[must_use]
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Sets the width and height ([`DEFAULT_SIZE`] by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    /// Sets the outline (a circle by default).
    #[must_use]
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    /// Sets the presence status shown as a dot.
    #[must_use]
    pub fn status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the presence status, or removes the dot with `None`.
    #[must_use]
    pub fn status_maybe(mut self, status: Option<Status>) -> Self {
        self.status = status;
        self
    }

    /// Sets the fallback background, instead of the color picked from the
    /// name.
    #[must_use]
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the icon shown without a name, instead of the person icon.
    #[cfg(feature = "svg")]
    #[must_use]
    pub fn icon(mut self, icon: crate::icon::Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the picture, drawn over the fallback and cropped to cover the
    /// avatar; the fallback shows while the picture loads.
    #[cfg(feature = "image")]
    #[must_use]
    pub fn image(mut self, handle: impl Into<iced::widget::image::Handle>) -> Self {
        self.image = Some(handle.into());
        self
    }

    /// The localized description of the avatar, such as "Ana Souza, Busy".
    #[must_use]
    pub fn description(&self, localizations: &Localizations) -> String {
        description(self.name.as_deref(), self.status, localizations)
    }

    /// The name, when it is not blank.
    fn visible_name(&self) -> Option<&str> {
        self.name.as_deref().filter(|name| !name.trim().is_empty())
    }

    /// The fallback background and the color of its content in `theme`.
    fn colors(&self, theme: &Theme) -> (Color, Color) {
        let roles = theme.roles();

        match (self.background, self.visible_name()) {
            (Some(background), Some(_)) => (background, contrast::foreground_for(background)),
            (Some(background), None) => (background, roles.on_surface_variant),
            (None, Some(name)) => {
                let background = color_for(name);

                (background, contrast::foreground_for(background))
            }
            (None, None) => (roles.surface_container_highest, roles.on_surface_variant),
        }
    }

    /// Draws the fallback icon in `bounds`.
    fn draw_icon(&self, renderer: &mut iced::Renderer, bounds: Rectangle, color: Color) {
        let side = self.size * ICON_SCALE;
        let area = Rectangle {
            x: bounds.center_x() - side / 2.0,
            y: bounds.center_y() - side / 2.0,
            width: side,
            height: side,
        };

        #[cfg(feature = "svg")]
        if let Some(icon) = self.icon {
            use iced::advanced::svg::{Renderer as _, Svg};

            renderer.draw_svg(
                Svg::new(icon.handle())
                    .color(Color { a: 1.0, ..color })
                    .opacity(color.a),
                area,
                bounds,
            );
            return;
        }

        draw_person(renderer, area, color);
    }

    /// Draws the picture over the fallback, cropped to cover `bounds`.
    #[cfg(feature = "image")]
    fn draw_image(&self, renderer: &mut iced::Renderer, bounds: Rectangle, viewport: &Rectangle) {
        use iced::advanced::image::{self as image, Renderer as _};

        let Some(handle) = &self.image else {
            return;
        };
        let Some(size) = renderer.measure_image(handle) else {
            return;
        };
        if size.width == 0 || size.height == 0 {
            return;
        }

        let (width, height) = (size.width as f32, size.height as f32);
        let scale = (bounds.width / width).max(bounds.height / height);
        let drawn = Size::new(width * scale, height * scale);
        let target = Rectangle::new(
            Point::new(
                bounds.center_x() - drawn.width / 2.0,
                bounds.center_y() - drawn.height / 2.0,
            ),
            drawn,
        );
        let Some(clip) = bounds.intersection(viewport) else {
            return;
        };

        renderer.draw_image(
            image::Image {
                handle: handle.clone(),
                filter_method: image::FilterMethod::Linear,
                rotation: Radians(0.0),
                border_radius: self.shape.radius(self.size).into(),
                opacity: 1.0,
                snap: false,
            },
            target,
            clip,
        );
    }
}

/// Draws the person icon in `area`: a head and the shoulders, on a 24-unit
/// grid.
fn draw_person<R: geometry::Renderer>(renderer: &mut R, area: Rectangle, color: Color) {
    let unit = area.width / 24.0;
    let mut frame = Frame::new(renderer, area.size());
    let at = |x: f32, y: f32| Point::new(x * unit, y * unit);
    let path = Path::new(|builder| {
        builder.circle(at(12.0, 8.0), 5.0 * unit);
        builder.move_to(at(20.0, 21.0));
        builder.arc(iced::widget::canvas::path::Arc {
            center: at(12.0, 21.0),
            radius: 8.0 * unit,
            start_angle: Radians(0.0),
            end_angle: Radians(-std::f32::consts::PI),
        });
    });

    frame.stroke(
        &path,
        Stroke::default()
            .with_color(color)
            .with_width(2.0 * unit)
            .with_line_cap(LineCap::Round),
    );

    let geometry = frame.into_geometry();

    renderer.with_translation(area.position() - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Avatar {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.size, self.size)
    }

    fn operate(
        &mut self,
        _tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        if let Some(name) = self.visible_name() {
            operation.text(None, layout.bounds(), &initials(name));
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let (background, foreground) = self.colors(theme);
        let radius = self.shape.radius(self.size);

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border::default().rounded(radius),
                ..Quad::default()
            },
            background,
        );

        let letters = self.visible_name().map(initials).unwrap_or_default();

        if letters.is_empty() {
            self.draw_icon(renderer, bounds, foreground);
        } else {
            let size = self.size * INITIALS_SCALE;

            renderer.fill_text(
                text::Text {
                    content: letters,
                    bounds: Size::new(bounds.width + 1.0, bounds.height),
                    size: Pixels(size),
                    line_height: text::LineHeight::Absolute(Pixels(size)),
                    font: crate::font::inter(Weight::Semibold),
                    align_x: text::Alignment::Center,
                    align_y: iced::alignment::Vertical::Center,
                    shaping: text::Shaping::Advanced,
                    wrapping: text::Wrapping::None,
                },
                bounds.center(),
                foreground,
                bounds,
            );
        }

        #[cfg(feature = "image")]
        self.draw_image(renderer, bounds, viewport);

        if let Some(status) = self.status {
            let (scale, min, max) = DOT_SCALE;
            let dot = (self.size * scale).clamp(min, max);
            let dot_bounds = Rectangle {
                x: bounds.x + bounds.width - dot,
                y: bounds.y + bounds.height - dot,
                width: dot,
                height: dot,
            };

            renderer.with_layer(
                dot_bounds.intersection(viewport).unwrap_or(dot_bounds),
                |renderer| {
                    renderer.fill_quad(
                        Quad {
                            bounds: dot_bounds,
                            border: Border::default()
                                .rounded(dot / 2.0)
                                .width(dot * DOT_RING)
                                .color(theme.roles().background),
                            ..Quad::default()
                        },
                        status.color(theme),
                    );
                },
            );
        }
    }
}

impl<'a, Message: 'a> From<Avatar> for Element<'a, Message> {
    fn from(avatar: Avatar) -> Self {
        IcedElement::new(avatar)
    }
}

/// A row of overlapping avatars. See [`group`].
pub struct AvatarGroup<'a, Message> {
    avatars: Vec<Element<'a, Message>>,
    max: usize,
    size: f32,
    overlap: f32,
    shape: Shape,
}

/// A row of overlapping `avatars`, usually [`Avatar`]s of the group
/// [size](AvatarGroup::size), showing at most [`AvatarGroup::max`] of them
/// and a "+N" bubble for the rest.
pub fn group<'a, Message, E>(avatars: impl IntoIterator<Item = E>) -> AvatarGroup<'a, Message>
where
    E: Into<Element<'a, Message>>,
{
    AvatarGroup {
        avatars: avatars.into_iter().map(Into::into).collect(),
        max: DEFAULT_MAX,
        size: DEFAULT_GROUP_SIZE,
        overlap: DEFAULT_OVERLAP,
        shape: Shape::Circle,
    }
}

impl<Message> AvatarGroup<'_, Message> {
    /// Sets the number of avatars shown before the "+N" bubble, at least 1
    /// ([`DEFAULT_MAX`] by default).
    #[must_use]
    pub fn max(mut self, max: usize) -> Self {
        self.max = max.max(1);
        self
    }

    /// Sets the side of the avatars, used for the spacing and the bubble
    /// ([`DEFAULT_GROUP_SIZE`] by default).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size.max(0.0);
        self
    }

    /// Sets the fraction of each avatar covered by the next one, from 0 up
    /// to, but not including, 1 ([`DEFAULT_OVERLAP`] by default).
    #[must_use]
    pub fn overlap(mut self, overlap: f32) -> Self {
        self.overlap = overlap.clamp(0.0, 0.99);
        self
    }

    /// Sets the shape of the rings and the bubble, which should match the
    /// avatars' (a circle by default).
    #[must_use]
    pub fn shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self
    }

    /// The number of avatars past [`AvatarGroup::max`], counted in the
    /// "+N" bubble.
    #[must_use]
    pub fn hidden(&self) -> usize {
        self.avatars.len().saturating_sub(self.max)
    }
}

impl<'a, Message: 'a> From<AvatarGroup<'a, Message>> for Element<'a, Message> {
    fn from(group: AvatarGroup<'a, Message>) -> Self {
        let hidden = group.hidden();
        let size = group.size;
        let shape = group.shape;
        let extent = size + 2.0 * RING_WIDTH;
        let ring_radius = shape.radius(extent);

        let mut items: Vec<Element<'a, Message>> =
            group.avatars.into_iter().take(group.max).collect();

        if hidden > 0 {
            let bubble = container(
                styled_text(format!("+{hidden}"), typography::LABEL_MEDIUM)
                    .font(crate::font::inter(Weight::Semibold)),
            )
            .center(Length::Fixed(size))
            .style(move |theme: &Theme| {
                let roles = theme.roles();

                container::Style {
                    background: Some(Background::Color(roles.surface_container_highest)),
                    text_color: Some(roles.on_surface_variant),
                    border: Border::default().rounded(shape.radius(size)),
                    ..container::Style::default()
                }
            });

            items.push(bubble.into());
        }

        let count = items.len();
        let step = extent * (1.0 - group.overlap);
        let width = step * count.saturating_sub(1) as f32 + extent;

        let layers = items.into_iter().enumerate().map(|(index, item)| {
            let ringed = container(item)
                .padding(RING_WIDTH)
                .style(move |theme: &Theme| container::Style {
                    background: Some(Background::Color(theme.roles().background)),
                    border: Border::default().rounded(ring_radius),
                    ..container::Style::default()
                });

            container(ringed)
                .padding(Padding {
                    left: step * index as f32,
                    ..Padding::ZERO
                })
                .into()
        });

        Stack::with_children(layers)
            .width(Length::Fixed(width))
            .height(Length::Fixed(extent))
            .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initials_take_the_first_and_last_words() {
        assert_eq!(initials("Ana Maria Souza"), "AS");
        assert_eq!(initials("  joão "), "J");
        assert_eq!(initials(""), "");
        assert_eq!(initials("e\u{301}lise"), "E\u{301}");
        assert_eq!(
            initials("\u{1F469}\u{200D}\u{1F4BB} dev"),
            "\u{1F469}\u{200D}\u{1F4BB}D"
        );
    }
}
