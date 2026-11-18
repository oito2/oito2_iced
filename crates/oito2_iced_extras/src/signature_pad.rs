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

//! Signature pad: an area to sign with a pointer or a finger.
//!
//! Strokes get thinner as the pointer moves faster, between
//! [`SignaturePad::min_width`] and [`SignaturePad::max_width`], like ink.
//! Only the first pointer down draws, and a mouse draws with its left
//! button only. With [`SignaturePad::show_actions`] (on by default) undo
//! and clear buttons sit in the top end corner, disabled while the pad is
//! empty; the empty pad shows the localized "Sign here".
//!
//! The strokes live in the application, in a [`Signature`]: the pad shows
//! it and publishes [`Event`]s through [`SignaturePad::on_event`], which
//! the application applies with [`Signature::apply`]. Without
//! `on_event` the pad only shows the strokes. [`Signature::to_svg`],
//! [`Signature::to_image`] and [`Signature::to_png`] export the strokes.
//!
//! ```no_run
//! use oito2_iced_extras::signature_pad::{self, Signature};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Signed(signature_pad::Event),
//! }
//!
//! struct App {
//!     signature: Signature,
//! }
//!
//! impl App {
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::Signed(event) => self.signature.apply(event),
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         signature_pad::signature_pad(&self.signature)
//!             .on_event(Message::Signed)
//!             .into()
//!     }
//! }
//! ```

use std::fmt::Write as _;
use std::rc::Rc;

use iced::advanced::graphics::geometry::{self, Frame, LineCap, Path, Stroke};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::time::Instant;
use iced::widget::{Space, container, opaque, row, stack, text};
use iced::{
    Border, Color, Element as IcedElement, Event as IcedEvent, Length, Padding, Point, Rectangle,
    Size, mouse, touch,
};
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{radius, spacing, typography};
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::styled_text;
use oito2_iced::{Element, Theme};

/// The default height of the pad.
pub const DEFAULT_HEIGHT: f32 = 180.0;

/// The default thinnest stroke.
pub const DEFAULT_MIN_WIDTH: f32 = 1.2;

/// The default thickest stroke.
pub const DEFAULT_MAX_WIDTH: f32 = 3.6;

/// The least distance, in logical pixels, a pointer moves before the
/// stroke gets a new point.
const MIN_DISTANCE: f32 = 1.0;

/// How much the speed, in logical pixels per millisecond, thins the
/// stroke.
const SPEED_THINNING: f32 = 1.2;

/// The part of the previous width a new point keeps; the rest moves to
/// the width the speed asks for.
const WIDTH_EASING: f32 = 0.6;

/// A point of a stroke with the stroke width there.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrokePoint {
    /// The position in the pad, in logical pixels from its top left corner.
    pub position: Point,
    /// The stroke width at the point.
    pub width: f32,
}

impl StrokePoint {
    /// A point at `position` with the stroke `width`.
    pub fn new(position: Point, width: f32) -> Self {
        Self { position, width }
    }
}

/// A change to a [`Signature`], published by a [`SignaturePad`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A stroke starts at `point`, drawn in a pad of `size`.
    Begin {
        /// The first point of the stroke.
        point: StrokePoint,
        /// The size of the pad drawing it.
        size: Size,
    },
    /// The last stroke goes on to the point.
    Extend(StrokePoint),
    /// The last stroke is removed.
    Undo,
    /// Every stroke is removed.
    Clear,
}

/// The strokes of a signature, with undo, clear and export.
///
/// Positions are in the pad's logical pixels; [`Signature::size`] is the
/// size of the pad the last stroke was started in, which the exports use.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Signature {
    strokes: Vec<Vec<StrokePoint>>,
    size: Size,
}

impl Signature {
    /// An empty signature.
    pub fn new() -> Self {
        Self::default()
    }

    /// The strokes, in drawing order.
    pub fn strokes(&self) -> &[Vec<StrokePoint>] {
        &self.strokes
    }

    /// Whether nothing has been drawn.
    pub fn is_empty(&self) -> bool {
        self.strokes.is_empty()
    }

    /// The size of the pad, set when a stroke starts; the exports use it.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Sets the size the exports use.
    pub fn set_size(&mut self, size: Size) {
        self.size = size;
    }

    /// Removes the last stroke, if any.
    pub fn undo(&mut self) {
        self.strokes.pop();
    }

    /// Removes every stroke.
    pub fn clear(&mut self) {
        self.strokes.clear();
    }

    /// Adds a whole stroke, such as one of a saved signature; an empty
    /// stroke is ignored.
    pub fn add_stroke(&mut self, stroke: impl IntoIterator<Item = StrokePoint>) {
        let stroke: Vec<StrokePoint> = stroke.into_iter().collect();

        if !stroke.is_empty() {
            self.strokes.push(stroke);
        }
    }

    /// Applies an event of a [`SignaturePad`].
    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Begin { point, size } => {
                self.size = size;
                self.strokes.push(vec![point]);
            }
            Event::Extend(point) => {
                if let Some(stroke) = self.strokes.last_mut() {
                    stroke.push(point);
                }
            }
            Event::Undo => self.undo(),
            Event::Clear => self.clear(),
        }
    }

    /// The strokes as an SVG document of `size` (the pad size with
    /// `None`), one path per stroke segment so the width can vary.
    ///
    /// Strokes use the opaque RGB value of `color`; its alpha is ignored.
    /// A single-point stroke becomes a filled circle.
    pub fn to_svg(&self, size: Option<Size>, color: Color) -> String {
        let size = size.unwrap_or(self.size);
        let [r, g, b, _] = color.into_rgba8();
        let hex = format!("#{r:02x}{g:02x}{b:02x}");
        let mut svg = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.1}\" height=\"{h:.1}\" \
             viewBox=\"0 0 {w:.1} {h:.1}\"><g fill=\"none\" stroke=\"{hex}\" \
             stroke-linecap=\"round\" stroke-linejoin=\"round\">",
            w = size.width,
            h = size.height,
        );

        for stroke in &self.strokes {
            if let [point] = stroke.as_slice() {
                let _ = write!(
                    svg,
                    "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"{:.1}\" fill=\"{hex}\" stroke=\"none\"/>",
                    point.position.x,
                    point.position.y,
                    point.width / 2.0,
                );
                continue;
            }

            for pair in stroke.windows(2) {
                let (a, b) = (pair[0], pair[1]);
                let _ = write!(
                    svg,
                    "<path d=\"M{:.1} {:.1}L{:.1} {:.1}\" stroke-width=\"{:.1}\"/>",
                    a.position.x,
                    a.position.y,
                    b.position.x,
                    b.position.y,
                    (a.width + b.width) / 2.0,
                );
            }
        }

        svg.push_str("</g></svg>");
        svg
    }

    /// Renders the strokes in `color` on a transparent background of the
    /// pad size, at `pixel_ratio` physical pixels per logical pixel, with
    /// antialiased edges. The image is at least one pixel wide and tall.
    ///
    /// Segments are round-capped lines whose width is the mean of the
    /// widths of their ends, and a single-point stroke is a dot; where
    /// they overlap the ink does not get darker.
    pub fn to_image(&self, color: Color, pixel_ratio: f32) -> image::RgbaImage {
        let ratio = if pixel_ratio > 0.0 { pixel_ratio } else { 1.0 };
        let width = pixels(self.size.width * ratio);
        let height = pixels(self.size.height * ratio);
        let mut coverage = vec![0.0_f32; width as usize * height as usize];

        for stroke in &self.strokes {
            for (a, b) in segments(stroke) {
                let radius = (a.width + b.width) / 4.0 * ratio;
                let from = Point::new(a.position.x * ratio, a.position.y * ratio);
                let to = Point::new(b.position.x * ratio, b.position.y * ratio);

                cover_capsule(&mut coverage, width, height, from, to, radius);
            }
        }

        let [r, g, b, a] = color.into_rgba8();

        image::RgbaImage::from_fn(width, height, |x, y| {
            let amount = coverage[(y * width + x) as usize];

            if amount <= 0.0 {
                image::Rgba([0, 0, 0, 0])
            } else {
                image::Rgba([r, g, b, (f32::from(a) * amount).round() as u8])
            }
        })
    }

    /// The strokes of [`Signature::to_image`] encoded as PNG bytes, or
    /// `None` when encoding fails.
    pub fn to_png(&self, color: Color, pixel_ratio: f32) -> Option<Vec<u8>> {
        let image = self.to_image(color, pixel_ratio);
        let mut bytes = std::io::Cursor::new(Vec::new());

        image
            .write_to(&mut bytes, image::ImageFormat::Png)
            .ok()
            .map(|()| bytes.into_inner())
    }
}

/// The pixel count of a side of `length` physical pixels: at least one.
fn pixels(length: f32) -> u32 {
    if length.is_finite() && length > 1.0 {
        length.ceil() as u32
    } else {
        1
    }
}

/// The segments of `stroke`: each pair of consecutive points, or the
/// point paired with itself for a single-point stroke.
fn segments(stroke: &[StrokePoint]) -> Vec<(StrokePoint, StrokePoint)> {
    match stroke {
        [] => Vec::new(),
        [point] => vec![(*point, *point)],
        _ => stroke.windows(2).map(|pair| (pair[0], pair[1])).collect(),
    }
}

/// Raises `coverage`, a `width` × `height` grid of pixel coverages, to the
/// coverage of a round-capped line from `from` to `to` with `radius`.
fn cover_capsule(
    coverage: &mut [f32],
    width: u32,
    height: u32,
    from: Point,
    to: Point,
    radius: f32,
) {
    let reach = radius + 1.0;
    let left = (from.x.min(to.x) - reach).floor().max(0.0) as u32;
    let top = (from.y.min(to.y) - reach).floor().max(0.0) as u32;
    let right = ((from.x.max(to.x) + reach).ceil().max(0.0) as u32).min(width);
    let bottom = ((from.y.max(to.y) + reach).ceil().max(0.0) as u32).min(height);

    for y in top..bottom {
        for x in left..right {
            let center = Point::new(x as f32 + 0.5, y as f32 + 0.5);
            let amount = (radius - distance_to_segment(center, from, to) + 0.5).clamp(0.0, 1.0);
            let cell = &mut coverage[(y * width + x) as usize];

            *cell = cell.max(amount);
        }
    }
}

/// The distance from `point` to the segment from `a` to `b`.
fn distance_to_segment(point: Point, a: Point, b: Point) -> f32 {
    let (dx, dy) = (b.x - a.x, b.y - a.y);
    let length = dx * dx + dy * dy;
    let t = if length > 0.0 {
        (((point.x - a.x) * dx + (point.y - a.y) * dy) / length).clamp(0.0, 1.0)
    } else {
        0.0
    };

    point.distance(Point::new(a.x + t * dx, a.y + t * dy))
}

/// The stroke width after a move of `distance` logical pixels in
/// `elapsed_ms` milliseconds (at least one): faster moves aim at thinner
/// strokes between `min` and `max`, and the width eases toward that aim
/// so it does not jump between points.
fn next_width(width: f32, distance: f32, elapsed_ms: f32, min: f32, max: f32) -> f32 {
    let speed = distance / elapsed_ms.max(1.0);
    let target = (max - speed * SPEED_THINNING).clamp(min, max);

    width * WIDTH_EASING + target * (1.0 - WIDTH_EASING)
}

/// An area to sign. See [`signature_pad`].
pub struct SignaturePad<'a, Message> {
    signature: &'a Signature,
    on_event: Option<Rc<dyn Fn(Event) -> Message + 'a>>,
    height: f32,
    color: Option<Color>,
    min_width: f32,
    max_width: f32,
    show_actions: bool,
    placeholder: Option<String>,
    localizations: Localizations,
    direction: ReadingDirection,
}

/// A pad showing the strokes of `signature`. Without
/// [`SignaturePad::on_event`] it only shows them.
pub fn signature_pad<'a, Message>(signature: &'a Signature) -> SignaturePad<'a, Message> {
    SignaturePad {
        signature,
        on_event: None,
        height: DEFAULT_HEIGHT,
        color: None,
        min_width: DEFAULT_MIN_WIDTH,
        max_width: DEFAULT_MAX_WIDTH,
        show_actions: true,
        placeholder: None,
        localizations: Localizations::en(),
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> SignaturePad<'a, Message> {
    /// Sets the function that turns a drawing, undo or clear [`Event`]
    /// into a message.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(Event) -> Message + 'a) -> Self {
        self.on_event = Some(Rc::new(on_event));
        self
    }

    /// Sets the pad height ([`DEFAULT_HEIGHT`] by default); the pad takes
    /// the available width.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// Sets the ink color; the theme's text color by default.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the thinnest stroke ([`DEFAULT_MIN_WIDTH`] by default); a
    /// width that is not positive keeps the default.
    #[must_use]
    pub fn min_width(mut self, width: f32) -> Self {
        if width > 0.0 {
            self.min_width = width;
        }
        self
    }

    /// Sets the thickest stroke ([`DEFAULT_MAX_WIDTH`] by default); it is
    /// never thinner than the thinnest.
    #[must_use]
    pub fn max_width(mut self, width: f32) -> Self {
        if width > 0.0 {
            self.max_width = width;
        }
        self
    }

    /// Sets whether the undo and clear buttons show (true by default).
    #[must_use]
    pub fn show_actions(mut self, show: bool) -> Self {
        self.show_actions = show;
        self
    }

    /// Sets the text of the empty pad; the localized "Sign here" by
    /// default.
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = Some(placeholder.into());
        self
    }

    /// Sets the strings of the placeholder and of the button tooltips
    /// (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the reading direction, which puts the buttons in the top right
    /// (left to right, the default) or top left corner.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<SignaturePad<'a, Message>> for Element<'a, Message> {
    fn from(pad: SignaturePad<'a, Message>) -> Self {
        let empty = pad.signature.is_empty();
        let max_width = pad.max_width.max(pad.min_width);

        let frame = container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let roles = theme.roles();

                container::Style {
                    background: Some(roles.surface_container_high.into()),
                    border: Border {
                        color: roles.outline,
                        width: 1.0,
                        radius: radius::MEDIUM.into(),
                    },
                    ..container::Style::default()
                }
            });

        let baseline = container(
            container(Space::new())
                .width(Length::Fill)
                .height(1.0)
                .style(|theme: &Theme| {
                    container::Style::default().background(theme.roles().outline_variant)
                }),
        )
        .padding(Padding {
            top: 0.0,
            right: spacing::LG,
            bottom: spacing::XL,
            left: spacing::LG,
        })
        .align_bottom(Length::Fill)
        .width(Length::Fill);

        let ink = Ink {
            strokes: pad.signature.strokes(),
            on_event: pad.on_event.clone(),
            color: pad.color,
            min_width: pad.min_width,
            max_width,
        };

        let mut layers: Vec<Element<'a, Message>> = vec![frame.into()];

        if empty {
            let placeholder = pad
                .placeholder
                .unwrap_or_else(|| pad.localizations.sign_here.to_owned());

            layers.push(
                container(styled_text(placeholder, typography::BODY_LARGE).style(
                    |theme: &Theme| text::Style {
                        color: Some(theme.roles().on_surface_variant),
                    },
                ))
                .center(Length::Fill)
                .into(),
            );
        }

        layers.push(baseline.into());
        layers.push(IcedElement::new(ink));

        if pad.show_actions {
            let press = |event: Event| {
                pad.on_event
                    .as_ref()
                    .filter(|_| !empty)
                    .map(|on_event| on_event(event))
            };
            let actions = row![
                icon_button(lucide::UNDO_2)
                    .tooltip(pad.localizations.undo)
                    .on_press_maybe(press(Event::Undo)),
                icon_button(lucide::ERASER)
                    .tooltip(pad.localizations.clear)
                    .on_press_maybe(press(Event::Clear)),
            ];
            let corner = container(opaque(actions))
                .padding(spacing::XS)
                .width(Length::Fill)
                .align_top(Length::Fill);
            let corner = match pad.direction {
                ReadingDirection::LeftToRight => corner.align_right(Length::Fill),
                ReadingDirection::RightToLeft => corner.align_left(Length::Fill),
            };

            layers.push(corner.into());
        }

        stack(layers)
            .width(Length::Fill)
            .height(Length::Fixed(pad.height))
            .into()
    }
}

/// The pointer drawing the current stroke.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Finger(touch::Finger),
}

/// The tree state of the drawing layer: the stroke being drawn.
#[derive(Debug, Clone, Copy, Default)]
struct State {
    pointer: Option<Pointer>,
    last: Option<(Point, Instant)>,
    width: f32,
}

/// The drawing layer of a [`SignaturePad`]: paints the strokes and turns
/// pointer moves into [`Event`]s.
struct Ink<'a, Message> {
    strokes: &'a [Vec<StrokePoint>],
    on_event: Option<Rc<dyn Fn(Event) -> Message + 'a>>,
    color: Option<Color>,
    min_width: f32,
    max_width: f32,
}

impl<Message> Ink<'_, Message> {
    /// Starts a stroke at `position` of the layer with `bounds`.
    fn begin(
        &self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        bounds: Rectangle,
        position: Point,
        pointer: Pointer,
    ) {
        let Some(on_event) = &self.on_event else {
            return;
        };
        let local = Point::new(position.x - bounds.x, position.y - bounds.y);

        state.pointer = Some(pointer);
        state.last = Some((local, Instant::now()));
        state.width = (self.min_width + self.max_width) / 2.0;

        shell.publish(on_event(Event::Begin {
            point: StrokePoint::new(local, state.width),
            size: bounds.size(),
        }));
        shell.capture_event();
    }

    /// Goes on with the stroke to `position` when it is at least one
    /// logical pixel away from the last point.
    fn extend(
        &self,
        state: &mut State,
        shell: &mut Shell<'_, Message>,
        bounds: Rectangle,
        position: Point,
    ) {
        let (Some(on_event), Some((last, time))) = (&self.on_event, state.last) else {
            return;
        };
        let local = Point::new(position.x - bounds.x, position.y - bounds.y);
        let distance = local.distance(last);

        if distance < MIN_DISTANCE {
            return;
        }

        let now = Instant::now();
        let elapsed = now.saturating_duration_since(time).as_millis() as f32;

        state.width = next_width(
            state.width,
            distance,
            elapsed,
            self.min_width,
            self.max_width,
        );
        state.last = Some((local, now));

        shell.publish(on_event(Event::Extend(StrokePoint::new(
            local,
            state.width,
        ))));
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Ink<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fill)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &IcedEvent,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let bounds = layout.bounds();

        if self.on_event.is_none() {
            *state = State::default();
            return;
        }

        match event {
            IcedEvent::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if state.pointer.is_none()
                    && let Some(position) = cursor.position_over(bounds)
                {
                    self.begin(state, shell, bounds, position, Pointer::Mouse);
                }
            }
            IcedEvent::Touch(touch::Event::FingerPressed { id, position })
                if state.pointer.is_none() && bounds.contains(*position) =>
            {
                self.begin(state, shell, bounds, *position, Pointer::Finger(*id));
            }
            IcedEvent::Mouse(mouse::Event::CursorMoved { .. })
                if state.pointer == Some(Pointer::Mouse) =>
            {
                if let Some(position) = cursor.land().position() {
                    self.extend(state, shell, bounds, position);
                }

                shell.capture_event();
            }
            IcedEvent::Touch(touch::Event::FingerMoved { id, position })
                if state.pointer == Some(Pointer::Finger(*id)) =>
            {
                self.extend(state, shell, bounds, *position);
                shell.capture_event();
            }
            IcedEvent::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.pointer == Some(Pointer::Mouse) =>
            {
                *state = State::default();
            }
            IcedEvent::Touch(
                touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
            ) if state.pointer == Some(Pointer::Finger(*id)) => {
                *state = State::default();
            }
            _ => {}
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
        _viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        if self.strokes.is_empty() {
            return;
        }

        let bounds = layout.bounds();
        let ink = self.color.unwrap_or(theme.roles().on_surface);
        let mut frame = Frame::new(renderer, bounds.size());

        for stroke in self.strokes {
            if let [point] = stroke.as_slice() {
                frame.fill(&Path::circle(point.position, point.width / 2.0), ink);
                continue;
            }

            for pair in stroke.windows(2) {
                let (a, b) = (pair[0], pair[1]);

                frame.stroke(
                    &Path::line(a.position, b.position),
                    Stroke::default()
                        .with_color(ink)
                        .with_width((a.width + b.width) / 2.0)
                        .with_line_cap(LineCap::Round),
                );
            }
        }

        renderer.with_layer(bounds, |renderer| {
            draw_frame(renderer, frame, bounds.position());
        });
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.on_event.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Crosshair
        } else {
            mouse::Interaction::default()
        }
    }
}

/// Draws the geometry of `frame` with its origin at `origin`.
fn draw_frame<Renderer>(renderer: &mut Renderer, frame: Frame<Renderer>, origin: Point)
where
    Renderer: geometry::Renderer,
{
    let geometry = frame.into_geometry();

    renderer.with_translation(origin - Point::ORIGIN, |renderer| {
        renderer.draw_geometry(geometry);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn point(x: f32, y: f32, width: f32) -> StrokePoint {
        StrokePoint::new(Point::new(x, y), width)
    }

    #[test]
    fn faster_moves_thin_the_stroke_gradually() {
        let width = 2.4;
        let slow = next_width(width, 2.0, 20.0, 1.2, 3.6);
        let fast = next_width(width, 40.0, 4.0, 1.2, 3.6);

        // Slow: aims at 3.48 and eases 40 % of the way.
        assert!((slow - (2.4 * 0.6 + 3.48 * 0.4)).abs() < 1e-5);
        // Fast: aims at the thinnest width.
        assert!((fast - (2.4 * 0.6 + 1.2 * 0.4)).abs() < 1e-5);
        // An instant move counts as one millisecond.
        assert_eq!(
            next_width(width, 1.0, 0.0, 1.2, 3.6),
            next_width(width, 1.0, 1.0, 1.2, 3.6)
        );
    }

    #[test]
    fn events_build_strokes() {
        let mut signature = Signature::new();

        signature.apply(Event::Extend(point(1.0, 1.0, 2.0)));
        assert!(signature.is_empty());

        signature.apply(Event::Begin {
            point: point(1.0, 1.0, 2.0),
            size: Size::new(100.0, 50.0),
        });
        signature.apply(Event::Extend(point(5.0, 1.0, 2.0)));
        signature.add_stroke([point(3.0, 3.0, 2.0)]);
        signature.add_stroke([]);

        assert_eq!(signature.strokes().len(), 2);
        assert_eq!(signature.strokes()[0].len(), 2);
        assert_eq!(signature.size(), Size::new(100.0, 50.0));

        signature.apply(Event::Undo);
        assert_eq!(signature.strokes().len(), 1);
        signature.apply(Event::Clear);
        assert!(signature.is_empty());
    }

    #[test]
    fn svg_has_a_path_per_segment_and_a_circle_per_dot() {
        let mut signature = Signature::new();

        signature.set_size(Size::new(120.0, 60.0));
        signature.add_stroke([point(10.0, 20.0, 2.0), point(30.0, 20.0, 3.0)]);
        signature.add_stroke([point(50.0, 40.0, 4.0)]);

        let svg = signature.to_svg(None, Color::from_rgba8(0x12, 0x34, 0x56, 0.5));

        assert_eq!(
            svg,
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"120.0\" height=\"60.0\" \
             viewBox=\"0 0 120.0 60.0\"><g fill=\"none\" stroke=\"#123456\" \
             stroke-linecap=\"round\" stroke-linejoin=\"round\">\
             <path d=\"M10.0 20.0L30.0 20.0\" stroke-width=\"2.5\"/>\
             <circle cx=\"50.0\" cy=\"40.0\" r=\"2.0\" fill=\"#123456\" stroke=\"none\"/>\
             </g></svg>"
        );
        assert!(
            signature
                .to_svg(Some(Size::new(10.0, 5.0)), Color::BLACK)
                .contains("width=\"10.0\" height=\"5.0\"")
        );
    }

    #[test]
    fn the_image_inks_only_the_strokes() {
        let mut signature = Signature::new();

        signature.set_size(Size::new(40.0, 20.0));
        signature.add_stroke([point(5.0, 10.0, 4.0), point(35.0, 10.0, 4.0)]);

        let image = signature.to_image(Color::from_rgb8(0, 0, 255), 2.0);

        assert_eq!(image.dimensions(), (80, 40));
        assert_eq!(image.get_pixel(40, 20).0, [0, 0, 255, 255]);
        assert_eq!(image.get_pixel(40, 2).0, [0, 0, 0, 0]);
        // The round cap reaches 2 logical pixels before the first point.
        assert_eq!(image.get_pixel(7, 20).0[3], 255);
        assert_eq!(image.get_pixel(4, 20).0[3], 0);
    }

    #[test]
    fn an_empty_pad_renders_one_pixel() {
        let image = Signature::new().to_image(Color::BLACK, 2.0);

        assert_eq!(image.dimensions(), (1, 1));
    }

    #[test]
    fn png_bytes_carry_the_signature() {
        let mut signature = Signature::new();

        signature.set_size(Size::new(30.0, 10.0));
        signature.add_stroke([point(5.0, 5.0, 3.0)]);

        let bytes = signature.to_png(Color::BLACK, 1.0).expect("png");

        assert_eq!(&bytes[1..4], b"PNG");

        let decoded = image::load_from_memory(&bytes).expect("decodes").to_rgba8();

        assert_eq!(decoded.dimensions(), (30, 10));
        assert_eq!(decoded.get_pixel(5, 5).0, [0, 0, 0, 255]);
    }
}
