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

//! Image cropper: the user zooms and pans an image under a crop box,
//! turns it by quarter turns and mirrors it, and the application cuts the
//! result out.
//!
//! The crop box has the [aspect ratio](Crop::with_aspect_ratio) of the
//! crop, or is a [circle](Crop::with_circle), and fits the crop area, which
//! takes the available width and [`ImageCropper::height`]. The image
//! always covers the box: the zoom goes from 1, where it just covers it, to
//! the [largest zoom](Crop::with_max_zoom), and the pan stops at its edges.
//! The image moves by dragging and zooms with the mouse wheel or the
//! toolbar under the area, which also turns, mirrors and resets it. With
//! the focus on the crop area the arrow keys pan (more with Shift), `+`,
//! `=` and `-` zoom, `r` turns clockwise, `R` the other way and `h`
//! mirrors. [`preview`] shows the crop live.
//!
//! The picture is a [`Source`], made once from an [`image::DynamicImage`]
//! and scaled down to at most [`DEFAULT_MAX_DIMENSION`] pixels on its
//! longest side. The zoom, pan, turns and mirror live in the application,
//! in a [`Crop`]: the cropper shows it and publishes [`Event`]s through
//! [`ImageCropper::on_event`], which the application applies with
//! [`Crop::apply`]. [`crop`] (or [`Source::crop`]) cuts the result out as
//! an [`image::RgbaImage`].
//!
//! ```no_run
//! use oito2_iced_extras::image_cropper::{self, Crop, Source};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Cropper(image_cropper::Event),
//! }
//!
//! struct App {
//!     source: Source,
//!     crop: Crop,
//! }
//!
//! impl App {
//!     fn new(picture: image::DynamicImage) -> Self {
//!         let source = Source::new(picture);
//!         let crop = Crop::new(source.size()).with_aspect_ratio(4.0 / 3.0);
//!
//!         Self { source, crop }
//!     }
//!
//!     fn update(&mut self, message: Message) {
//!         match message {
//!             Message::Cropper(event) => self.crop.apply(event),
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         iced::widget::column![
//!             image_cropper::image_cropper(&self.source, &self.crop)
//!                 .on_event(Message::Cropper),
//!             image_cropper::preview(&self.source, &self.crop).width(120.0),
//!         ]
//!         .into()
//!     }
//!
//!     fn result(&self) -> image::RgbaImage {
//!         self.source.crop(&self.crop, Some(240))
//!     }
//! }
//! ```

use std::f32::consts::FRAC_PI_2;
use std::rc::Rc;
use std::sync::Arc;

use iced::advanced::graphics::geometry::{self, Fill, Frame, Path, Stroke, fill};
use iced::advanced::image::{self as image_core, Renderer as _};
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, renderer};
use iced::keyboard::{self, key};
use iced::widget::image::Handle;
use iced::widget::{column, container, row};
use iced::{
    Alignment, Color, Element as IcedElement, Event as IcedEvent, Length, Point, Radians,
    Rectangle, Size, Vector, mouse, touch,
};
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::tokens::{radius, spacing};
use oito2_iced::widget::button::icon_button;
use oito2_iced::widget::slider::slider;
use oito2_iced::{Element, Theme, focus};

/// The default height of the crop area.
pub const DEFAULT_HEIGHT: f32 = 320.0;

/// The default largest zoom, as a multiple of the zoom that just covers
/// the box.
pub const DEFAULT_MAX_ZOOM: f32 = 5.0;

/// The default longest side, in pixels, of a [`Source`].
pub const DEFAULT_MAX_DIMENSION: u32 = 4096;

/// The default width of a [`preview`].
pub const DEFAULT_PREVIEW_WIDTH: f32 = 96.0;

/// The pan of one arrow key press, in logical pixels.
const PAN_STEP: f32 = 8.0;

/// The pan of one arrow key press with Shift, in logical pixels.
const PAN_STEP_LARGE: f32 = 32.0;

/// The zoom factor of one key press.
const KEY_ZOOM: f32 = 1.1;

/// The zoom factor of the zoom buttons.
const BUTTON_ZOOM: f32 = 1.2;

/// The zoom factor of one logical pixel of mouse wheel scrolling.
const WHEEL_ZOOM: f32 = 1.0015;

/// The logical pixels of one line of mouse wheel scrolling.
const WHEEL_LINE: f32 = 60.0;

/// The opacity of the scrim outside the crop box.
const SCRIM_OPACITY: f32 = 0.55;

/// The opacity of the rule-of-thirds guides.
const GUIDE_OPACITY: f32 = 0.5;

/// The width of the crop box outline.
const BOX_BORDER_WIDTH: f32 = 2.0;

/// The width of the zoom slider of the toolbar.
const ZOOM_SLIDER_WIDTH: f32 = 140.0;

/// A picture to crop: its pixels, scaled down to a bounded size, and the
/// image handles the cropper draws it with. Cloning shares the pixels.
#[derive(Debug, Clone)]
pub struct Source {
    pixels: Arc<image::RgbaImage>,
    handle: Handle,
    mirrored: Handle,
}

impl Source {
    /// The picture `image`, scaled down to at most
    /// [`DEFAULT_MAX_DIMENSION`] pixels on its longest side.
    pub fn new(image: image::DynamicImage) -> Self {
        Self::with_max_dimension(image, Some(DEFAULT_MAX_DIMENSION))
    }

    /// The picture `image`, scaled down to at most `max_dimension` pixels
    /// on its longest side, keeping its aspect ratio; `None` keeps the
    /// full size. Smaller pictures are never scaled up.
    pub fn with_max_dimension(image: image::DynamicImage, max_dimension: Option<u32>) -> Self {
        let image = match max_dimension {
            Some(max) if max > 0 && (image.width() > max || image.height() > max) => {
                image.resize(max, max, image::imageops::FilterType::Triangle)
            }
            _ => image,
        };
        let pixels = image.into_rgba8();
        let mirrored = image::imageops::flip_horizontal(&pixels);
        let handle = Handle::from_rgba(pixels.width(), pixels.height(), pixels.as_raw().clone());
        let mirrored = Handle::from_rgba(mirrored.width(), mirrored.height(), mirrored.into_raw());

        Self {
            pixels: Arc::new(pixels),
            handle,
            mirrored,
        }
    }

    /// Decodes a PNG or JPEG picture from `bytes`, scaled down as in
    /// [`Source::new`].
    ///
    /// # Errors
    ///
    /// Fails when the bytes are not a picture in a supported format.
    pub fn decode(bytes: &[u8]) -> Result<Self, image::ImageError> {
        image::load_from_memory(bytes).map(Self::new)
    }

    /// The size of the picture, in pixels.
    pub fn size(&self) -> Size<u32> {
        Size::new(self.pixels.width(), self.pixels.height())
    }

    /// The pixels of the picture.
    pub fn pixels(&self) -> &image::RgbaImage {
        &self.pixels
    }

    /// The image handle of the picture.
    pub fn handle(&self) -> &Handle {
        &self.handle
    }

    /// The picture cut as `crop` shows it; see [`crop`].
    pub fn crop(&self, crop: &Crop, width: Option<u32>) -> image::RgbaImage {
        self::crop(&self.pixels, crop, width)
    }
}

/// A change to a [`Crop`], published by an [`ImageCropper`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Moves the image by the vector, in parts of the crop box width and
    /// height.
    Pan(Vector),
    /// Sets the zoom.
    Zoom(f32),
    /// Multiplies the zoom by the factor.
    ZoomBy(f32),
    /// Turns the image a quarter counterclockwise.
    RotateLeft,
    /// Turns the image a quarter clockwise.
    RotateRight,
    /// Mirrors the image horizontally.
    FlipHorizontally,
    /// Restores the zoom, pan, turns and mirror.
    Reset,
}

/// The crop of a picture: the shape of the crop box and the zoom, pan,
/// quarter turns and mirror of the picture under it.
///
/// The pan is the offset of the picture center from the box center, in
/// parts of the box width and height, and is kept so the picture always
/// covers the box.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crop {
    image_size: Size<u32>,
    aspect_ratio: f32,
    circle: bool,
    max_zoom: f32,
    zoom: f32,
    offset: Vector,
    quarter_turns: u8,
    flipped: bool,
}

impl Crop {
    /// The crop of a picture of `image_size` pixels, with a square box,
    /// no zoom, pan, turn or mirror, zooming up to [`DEFAULT_MAX_ZOOM`].
    pub fn new(image_size: Size<u32>) -> Self {
        Self {
            image_size,
            aspect_ratio: 1.0,
            circle: false,
            max_zoom: DEFAULT_MAX_ZOOM,
            zoom: 1.0,
            offset: Vector::ZERO,
            quarter_turns: 0,
            flipped: false,
        }
    }

    /// Sets the crop box width divided by its height (1 by default); a
    /// ratio that is not positive keeps the current one.
    #[must_use]
    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.set_aspect_ratio(aspect_ratio);
        self
    }

    /// Makes the crop box a circle, which ignores the aspect ratio.
    #[must_use]
    pub fn with_circle(mut self, circle: bool) -> Self {
        self.set_circle(circle);
        self
    }

    /// Sets the largest zoom ([`DEFAULT_MAX_ZOOM`] by default); it is at
    /// least 1.
    #[must_use]
    pub fn with_max_zoom(mut self, max_zoom: f32) -> Self {
        self.max_zoom = if max_zoom >= 1.0 { max_zoom } else { 1.0 };
        self.set_zoom(self.zoom);
        self
    }

    /// The size of the picture, in pixels.
    pub fn image_size(&self) -> Size<u32> {
        self.image_size
    }

    /// The crop box width divided by its height as set, which a circle
    /// ignores.
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    /// Whether the crop box is a circle.
    pub fn circle(&self) -> bool {
        self.circle
    }

    /// The largest zoom.
    pub fn max_zoom(&self) -> f32 {
        self.max_zoom
    }

    /// The zoom, from 1 (the picture just covers the box) to the largest.
    pub fn zoom(&self) -> f32 {
        self.zoom
    }

    /// The pan of the picture center from the box center, in parts of the
    /// box width and height.
    pub fn offset(&self) -> Vector {
        self.offset
    }

    /// The clockwise quarter turns applied to the picture before the
    /// mirror, from 0 to 3.
    pub fn quarter_turns(&self) -> u8 {
        self.quarter_turns
    }

    /// Whether the picture is mirrored horizontally.
    pub fn flipped(&self) -> bool {
        self.flipped
    }

    /// The width divided by the height of the crop box: 1 for a circle.
    pub fn box_aspect(&self) -> f32 {
        if self.circle { 1.0 } else { self.aspect_ratio }
    }

    /// Sets the size of a new picture and restores the zoom, pan, turns
    /// and mirror.
    pub fn set_image_size(&mut self, image_size: Size<u32>) {
        self.image_size = image_size;
        self.reset();
    }

    /// Sets the crop box aspect ratio, keeping the picture over the box; a
    /// ratio that is not positive is ignored.
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        if aspect_ratio > 0.0 && aspect_ratio.is_finite() {
            self.aspect_ratio = aspect_ratio;
            self.clamp_offset();
        }
    }

    /// Makes the crop box a circle or not, keeping the picture over the
    /// box.
    pub fn set_circle(&mut self, circle: bool) {
        self.circle = circle;
        self.clamp_offset();
    }

    /// Sets the zoom, clamped from 1 to the largest, and keeps the picture
    /// over the box.
    pub fn set_zoom(&mut self, zoom: f32) {
        if zoom.is_nan() {
            return;
        }

        self.zoom = zoom.clamp(1.0, self.max_zoom);
        self.clamp_offset();
    }

    /// Multiplies the zoom by `factor`.
    pub fn zoom_by(&mut self, factor: f32) {
        self.set_zoom(self.zoom * factor);
    }

    /// Moves the picture by `delta`, in parts of the box width and height,
    /// as far as it still covers the box.
    pub fn pan_by(&mut self, delta: Vector) {
        self.offset += delta;
        self.clamp_offset();
    }

    /// Turns the picture a quarter clockwise on screen.
    pub fn rotate_right(&mut self) {
        self.turn(true);
    }

    /// Turns the picture a quarter counterclockwise on screen.
    pub fn rotate_left(&mut self) {
        self.turn(false);
    }

    /// Mirrors the picture horizontally.
    pub fn flip_horizontally(&mut self) {
        self.flipped = !self.flipped;
        self.offset.x = -self.offset.x;
    }

    /// Restores the zoom, pan, turns and mirror.
    pub fn reset(&mut self) {
        self.zoom = 1.0;
        self.offset = Vector::ZERO;
        self.quarter_turns = 0;
        self.flipped = false;
    }

    /// Applies an event of an [`ImageCropper`].
    pub fn apply(&mut self, event: Event) {
        match event {
            Event::Pan(delta) => self.pan_by(delta),
            Event::Zoom(zoom) => self.set_zoom(zoom),
            Event::ZoomBy(factor) => self.zoom_by(factor),
            Event::RotateLeft => self.rotate_left(),
            Event::RotateRight => self.rotate_right(),
            Event::FlipHorizontally => self.flip_horizontally(),
            Event::Reset => self.reset(),
        }
    }

    /// The size of the image [`crop`] returns: `width` pixels wide when
    /// given, otherwise at the picture's resolution under the box; at
    /// least one pixel each way.
    pub fn output_size(&self, width: Option<u32>) -> Size<u32> {
        let aspect = self.box_aspect();
        let (box_width, box_height) = self.box_in_image();
        let (width, height) = match width {
            Some(width) => (width as f32, width as f32 / aspect),
            None => (box_width, box_height),
        };

        Size::new(at_least_one(width), at_least_one(height))
    }

    /// Turns the picture a quarter `clockwise` or counterclockwise as seen
    /// on screen, and the pan with it.
    ///
    /// The turns apply before the mirror, so a mirrored picture takes the
    /// opposite turn to look turned the requested way.
    fn turn(&mut self, clockwise: bool) {
        let step = if clockwise != self.flipped { 1 } else { 3 };
        let aspect = self.box_aspect();
        let Vector { x, y } = self.offset;

        self.quarter_turns = (self.quarter_turns + step) % 4;
        self.offset = if clockwise {
            Vector::new(-y / aspect, x * aspect)
        } else {
            Vector::new(y / aspect, -x * aspect)
        };
        self.clamp_offset();
    }

    /// The picture size after the quarter turns, in pixels.
    fn turned(&self) -> (f32, f32) {
        let (width, height) = (self.image_size.width as f32, self.image_size.height as f32);

        if self.quarter_turns % 2 == 1 {
            (height, width)
        } else {
            (width, height)
        }
    }

    /// The width and height of the crop box in turned picture pixels.
    fn box_in_image(&self) -> (f32, f32) {
        let aspect = self.box_aspect();
        let (width, height) = self.turned();

        (
            width.min(aspect * height) / self.zoom,
            height.min(width / aspect) / self.zoom,
        )
    }

    /// Limits the pan so the picture still covers the box.
    fn clamp_offset(&mut self) {
        let (width, height) = self.turned();

        if width <= 0.0 || height <= 0.0 {
            self.offset = Vector::ZERO;
            return;
        }

        let aspect = self.box_aspect();
        let extent_x = (width / height / aspect).max(1.0) * self.zoom;
        let extent_y = (aspect * height / width).max(1.0) * self.zoom;
        let max_x = ((extent_x - 1.0) / 2.0).max(0.0);
        let max_y = ((extent_y - 1.0) / 2.0).max(0.0);

        self.offset = Vector::new(
            self.offset.x.clamp(-max_x, max_x),
            self.offset.y.clamp(-max_y, max_y),
        );
    }

    /// The bounds the unturned, unmirrored picture is drawn in, and its
    /// turn, for a crop box at `bounds` on screen.
    fn placement(&self, bounds: Rectangle) -> (Rectangle, f32) {
        let (box_width, _) = self.box_in_image();
        let scale = if box_width > 0.0 {
            bounds.width / box_width
        } else {
            0.0
        };
        let center = Point::new(
            bounds.center_x() + self.offset.x * bounds.width,
            bounds.center_y() + self.offset.y * bounds.height,
        );
        let size = Size::new(
            self.image_size.width as f32 * scale,
            self.image_size.height as f32 * scale,
        );
        let turns = f32::from(self.quarter_turns) * FRAC_PI_2;

        (
            Rectangle::new(
                Point::new(center.x - size.width / 2.0, center.y - size.height / 2.0),
                size,
            ),
            if self.flipped { -turns } else { turns },
        )
    }
}

/// The pixel count of a side `length` pixels long, rounded: at least one.
fn at_least_one(length: f32) -> u32 {
    if length.is_finite() && length > 1.0 {
        length.round() as u32
    } else {
        1
    }
}

/// Cuts `image` as `crop` shows it under the crop box: at the picture's
/// resolution, or `width` pixels wide when given (see
/// [`Crop::output_size`]), with smooth sampling, and cut to a circle with
/// antialiased edges when the box is one.
///
/// The result is the same for the same input.
pub fn crop(image: &image::RgbaImage, crop: &Crop, width: Option<u32>) -> image::RgbaImage {
    let size = crop.output_size(width);
    let (out_width, out_height) = (size.width as f32, size.height as f32);
    let (box_width, box_height) = crop.box_in_image();
    let (image_width, image_height) = (image.width() as f32, image.height() as f32);
    let turns = crop.quarter_turns;

    image::RgbaImage::from_fn(size.width, size.height, |u, v| {
        let fx = (u as f32 + 0.5) / out_width - 0.5;
        let fy = (v as f32 + 0.5) / out_height - 0.5;
        let mut x = (fx - crop.offset.x) * box_width;
        let mut y = (fy - crop.offset.y) * box_height;

        if crop.flipped {
            x = -x;
        }

        for _ in 0..turns {
            (x, y) = (y, -x);
        }

        let mut pixel = sample(image, image_width / 2.0 + x, image_height / 2.0 + y);

        if crop.circle {
            let (rx, ry) = (out_width / 2.0, out_height / 2.0);
            let (dx, dy) = ((u as f32 + 0.5 - rx) / rx, (v as f32 + 0.5 - ry) / ry);
            let inside = ((1.0 - dx.hypot(dy)) * rx.min(ry) + 0.5).clamp(0.0, 1.0);

            pixel[3] *= inside;
        }

        let alpha = pixel[3];

        if alpha <= 0.0 {
            image::Rgba([0, 0, 0, 0])
        } else {
            image::Rgba([
                channel(pixel[0]),
                channel(pixel[1]),
                channel(pixel[2]),
                (alpha * 255.0).round().clamp(0.0, 255.0) as u8,
            ])
        }
    })
}

/// A color channel from 0 to 255 out of a value from 0 to 255.
fn channel(value: f32) -> u8 {
    value.round().clamp(0.0, 255.0) as u8
}

/// The color of `image` at the point (`x`, `y`) in pixels from its top
/// left corner, interpolated between the four nearest pixels and clamped
/// to the edges: straight color channels from 0 to 255 and the alpha from
/// 0 to 1.
fn sample(image: &image::RgbaImage, x: f32, y: f32) -> [f32; 4] {
    let (width, height) = (image.width(), image.height());

    if width == 0 || height == 0 {
        return [0.0; 4];
    }

    let (x, y) = (x - 0.5, y - 0.5);
    let (x0, y0) = (x.floor(), y.floor());
    let (tx, ty) = (x - x0, y - y0);
    let at = |dx: f32, dy: f32| {
        let px = (x0 + dx).clamp(0.0, (width - 1) as f32) as u32;
        let py = (y0 + dy).clamp(0.0, (height - 1) as f32) as u32;
        let [r, g, b, a] = image.get_pixel(px, py).0;
        let alpha = f32::from(a) / 255.0;

        [
            f32::from(r) * alpha,
            f32::from(g) * alpha,
            f32::from(b) * alpha,
            alpha,
        ]
    };
    let corners = [
        (at(0.0, 0.0), (1.0 - tx) * (1.0 - ty)),
        (at(1.0, 0.0), tx * (1.0 - ty)),
        (at(0.0, 1.0), (1.0 - tx) * ty),
        (at(1.0, 1.0), tx * ty),
    ];
    let mut sum = [0.0_f32; 4];

    for (color, weight) in corners {
        for (total, part) in sum.iter_mut().zip(color) {
            *total += part * weight;
        }
    }

    if sum[3] > 0.0 {
        [sum[0] / sum[3], sum[1] / sum[3], sum[2] / sum[3], sum[3]]
    } else {
        [0.0; 4]
    }
}

/// The crop box of an area of `bounds` with the box `aspect`: as large as
/// fits inside the area less its padding, centered.
fn crop_box(bounds: Rectangle, aspect: f32) -> Rectangle {
    let room = Size::new(
        (bounds.width - spacing::XL * 2.0).max(1.0),
        (bounds.height - spacing::XL * 2.0).max(1.0),
    );
    let size = if room.width / room.height > aspect {
        Size::new(room.height * aspect, room.height)
    } else {
        Size::new(room.width, room.width / aspect)
    };

    Rectangle::new(
        Point::new(
            bounds.center_x() - size.width / 2.0,
            bounds.center_y() - size.height / 2.0,
        ),
        size,
    )
}

/// Draws `source` turned and placed as `crop` shows it under a crop box at
/// `bounds`, clipped to `clip`.
fn draw_picture(
    renderer: &mut iced::Renderer,
    source: &Source,
    crop: &Crop,
    bounds: Rectangle,
    clip: Rectangle,
) {
    let (placement, turn) = crop.placement(bounds);
    let handle = if crop.flipped {
        source.mirrored.clone()
    } else {
        source.handle.clone()
    };

    renderer.draw_image(
        image_core::Image {
            handle,
            filter_method: image_core::FilterMethod::Linear,
            rotation: Radians(turn),
            border_radius: 0.0.into(),
            opacity: 1.0,
            snap: false,
        },
        placement,
        clip,
    );
}

/// An image cropper. See [`image_cropper`].
pub struct ImageCropper<'a, Message> {
    source: &'a Source,
    crop: Crop,
    on_event: Option<Rc<dyn Fn(Event) -> Message + 'a>>,
    height: f32,
    show_toolbar: bool,
    localizations: Localizations,
    id: Option<Id>,
}

/// A cropper showing `source` as `crop` places it. Without
/// [`ImageCropper::on_event`] it only shows it.
pub fn image_cropper<'a, Message>(source: &'a Source, crop: &Crop) -> ImageCropper<'a, Message> {
    ImageCropper {
        source,
        crop: *crop,
        on_event: None,
        height: DEFAULT_HEIGHT,
        show_toolbar: true,
        localizations: Localizations::en(),
        id: None,
    }
}

impl<'a, Message> ImageCropper<'a, Message> {
    /// Sets the function that turns a pan, zoom, turn, mirror or reset
    /// [`Event`] into a message.
    #[must_use]
    pub fn on_event(mut self, on_event: impl Fn(Event) -> Message + 'a) -> Self {
        self.on_event = Some(Rc::new(on_event));
        self
    }

    /// Sets the height of the crop area ([`DEFAULT_HEIGHT`] by default);
    /// it takes the available width.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height.max(0.0);
        self
    }

    /// Sets whether the zoom, turn, mirror and reset controls show under
    /// the crop area (true by default).
    #[must_use]
    pub fn show_toolbar(mut self, show: bool) -> Self {
        self.show_toolbar = show;
        self
    }

    /// Sets the strings of the toolbar tooltips (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the widget [`Id`] of the crop area, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl<'a, Message: Clone + 'a> From<ImageCropper<'a, Message>> for Element<'a, Message> {
    fn from(cropper: ImageCropper<'a, Message>) -> Self {
        let area = focus::ring(IcedElement::new(Area {
            source: cropper.source,
            crop: cropper.crop,
            on_event: cropper.on_event.clone(),
            height: cropper.height,
            id: cropper.id,
        }))
        .radius(0.0);

        if !cropper.show_toolbar {
            return area.into();
        }

        let crop = cropper.crop;
        let strings = cropper.localizations;
        let on_event = cropper.on_event;
        let press = |event: Event| on_event.as_ref().map(|on_event| on_event(event));
        let zoom = slider(crop.zoom())
            .range(1.0, crop.max_zoom())
            .on_change_maybe(
                on_event
                    .clone()
                    .map(|on_event| move |zoom| on_event(Event::Zoom(zoom))),
            );

        let toolbar = row![
            icon_button(lucide::ZOOM_OUT)
                .tooltip(strings.zoom_out)
                .on_press_maybe(press(Event::ZoomBy(1.0 / BUTTON_ZOOM))),
            container(zoom).width(ZOOM_SLIDER_WIDTH),
            icon_button(lucide::ZOOM_IN)
                .tooltip(strings.zoom_in)
                .on_press_maybe(press(Event::ZoomBy(BUTTON_ZOOM))),
            icon_button(lucide::ROTATE_CCW)
                .tooltip(strings.rotate_left)
                .on_press_maybe(press(Event::RotateLeft)),
            icon_button(lucide::ROTATE_CW)
                .tooltip(strings.rotate_right)
                .on_press_maybe(press(Event::RotateRight)),
            icon_button(lucide::ARROW_LEFT_RIGHT)
                .tooltip(strings.flip_horizontally)
                .selected(crop.flipped())
                .on_press_maybe(press(Event::FlipHorizontally)),
            icon_button(lucide::REFRESH_CCW)
                .tooltip(strings.reset)
                .on_press_maybe(press(Event::Reset)),
        ]
        .spacing(spacing::XS)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .wrap()
        .align_x(Alignment::Center);

        column![area, toolbar]
            .spacing(spacing::SM)
            .width(Length::Fill)
            .into()
    }
}

/// The tree state of the crop area.
#[derive(Debug, Clone, Copy, Default)]
struct AreaState {
    focused: bool,
    /// The pointer moving the image and its last position.
    drag: Option<(Pointer, Point)>,
}

/// The pointer moving the image.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Pointer {
    Mouse,
    Finger(touch::Finger),
}

impl operation::Focusable for AreaState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// The crop area of an [`ImageCropper`]: the picture under the crop box,
/// dimmed outside it, moved by the pointer and the keyboard.
struct Area<'a, Message> {
    source: &'a Source,
    crop: Crop,
    on_event: Option<Rc<dyn Fn(Event) -> Message + 'a>>,
    height: f32,
    id: Option<Id>,
}

impl<Message> Area<'_, Message> {
    /// The event a key press with `modifiers` asks for, moving the image
    /// in a crop box of `size`.
    fn key_event(
        key: &keyboard::Key,
        modified_key: &keyboard::Key,
        modifiers: keyboard::Modifiers,
        size: Size,
    ) -> Option<Event> {
        if modifiers.control() || modifiers.alt() || modifiers.logo() {
            return None;
        }

        if let keyboard::Key::Named(named) = key {
            let step = if modifiers.shift() {
                PAN_STEP_LARGE
            } else {
                PAN_STEP
            };
            let (dx, dy) = match named {
                key::Named::ArrowLeft => (step, 0.0),
                key::Named::ArrowRight => (-step, 0.0),
                key::Named::ArrowUp => (0.0, step),
                key::Named::ArrowDown => (0.0, -step),
                _ => return None,
            };

            return Some(Event::Pan(Vector::new(
                dx / size.width.max(1.0),
                dy / size.height.max(1.0),
            )));
        }

        let keyboard::Key::Character(character) = modified_key else {
            return None;
        };

        match character.as_str() {
            "+" | "=" => Some(Event::ZoomBy(KEY_ZOOM)),
            "-" => Some(Event::ZoomBy(1.0 / KEY_ZOOM)),
            "r" => Some(Event::RotateRight),
            "R" => Some(Event::RotateLeft),
            "h" => Some(Event::FlipHorizontally),
            _ => None,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Area<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<AreaState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(AreaState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(self.height))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, Length::Fixed(self.height))
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<AreaState>();

        if self.on_event.is_some() {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), layout.bounds());
        }
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
        let state = tree.state.downcast_mut::<AreaState>();
        let Some(on_event) = self.on_event.clone() else {
            *state = AreaState::default();
            return;
        };
        let bounds = layout.bounds();
        let crop_box = crop_box(bounds, self.crop.box_aspect());
        let pan = |from: Point, to: Point| {
            Event::Pan(Vector::new(
                (to.x - from.x) / crop_box.width.max(1.0),
                (to.y - from.y) / crop_box.height.max(1.0),
            ))
        };

        match event {
            IcedEvent::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(position) = cursor.position_over(bounds) {
                    state.focused = true;
                    state.drag = Some((Pointer::Mouse, position));
                    shell.capture_event();
                    shell.request_redraw();
                } else if state.focused {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            IcedEvent::Touch(touch::Event::FingerPressed { id, position })
                if state.drag.is_none() && bounds.contains(*position) =>
            {
                state.focused = true;
                state.drag = Some((Pointer::Finger(*id), *position));
                shell.capture_event();
            }
            IcedEvent::Mouse(mouse::Event::CursorMoved { position }) => {
                if let Some((Pointer::Mouse, last)) = state.drag {
                    if *position != last {
                        shell.publish(on_event(pan(last, *position)));
                    }

                    state.drag = Some((Pointer::Mouse, *position));
                    shell.capture_event();
                }
            }
            IcedEvent::Touch(touch::Event::FingerMoved { id, position }) => {
                if let Some((Pointer::Finger(finger), last)) = state.drag
                    && finger == *id
                {
                    if *position != last {
                        shell.publish(on_event(pan(last, *position)));
                    }

                    state.drag = Some((Pointer::Finger(finger), *position));
                    shell.capture_event();
                }
            }
            IcedEvent::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if matches!(state.drag, Some((Pointer::Mouse, _))) =>
            {
                state.drag = None;
            }
            IcedEvent::Touch(
                touch::Event::FingerLifted { id, .. } | touch::Event::FingerLost { id, .. },
            ) if matches!(state.drag, Some((Pointer::Finger(finger), _)) if finger == *id) => {
                state.drag = None;
            }
            IcedEvent::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.position_over(bounds).is_some() {
                    let lift = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => y * WHEEL_LINE,
                        mouse::ScrollDelta::Pixels { y, .. } => *y,
                    };

                    if lift != 0.0 {
                        shell.publish(on_event(Event::ZoomBy(WHEEL_ZOOM.powf(lift))));
                    }

                    shell.capture_event();
                }
            }
            IcedEvent::Keyboard(keyboard::Event::KeyPressed {
                key,
                modified_key,
                modifiers,
                ..
            }) if state.focused && !shell.is_event_captured() => {
                if let Some(event) = Self::key_event(key, modified_key, *modifiers, crop_box.size())
                {
                    shell.publish(on_event(event));
                    shell.capture_event();
                }
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
        let roles = theme.roles();
        let bounds = layout.bounds();
        let crop_box = crop_box(bounds, self.crop.box_aspect());
        let circle = self.crop.circle();

        renderer.with_layer(bounds, |renderer| {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    ..renderer::Quad::default()
                },
                roles.surface_container_highest,
            );

            draw_picture(renderer, self.source, &self.crop, crop_box, bounds);

            let mut frame = Frame::new(renderer, bounds.size());
            let local = Rectangle::new(
                Point::new(crop_box.x - bounds.x, crop_box.y - bounds.y),
                crop_box.size(),
            );
            let hole = if circle {
                Path::circle(local.center(), local.width / 2.0)
            } else {
                Path::rectangle(local.position(), local.size())
            };
            let scrim = Path::new(|builder| {
                builder.rectangle(Point::ORIGIN, bounds.size());

                if circle {
                    builder.circle(local.center(), local.width / 2.0);
                } else {
                    builder.rectangle(local.position(), local.size());
                }
            });

            frame.fill(
                &scrim,
                Fill {
                    style: geometry::Style::Solid(Color {
                        a: SCRIM_OPACITY,
                        ..roles.scrim
                    }),
                    rule: fill::Rule::EvenOdd,
                },
            );

            if !circle {
                let guide = Stroke::default()
                    .with_color(Color::WHITE.scale_alpha(GUIDE_OPACITY))
                    .with_width(1.0);

                for third in [1.0, 2.0] {
                    let x = local.x + local.width * third / 3.0;
                    let y = local.y + local.height * third / 3.0;

                    frame.stroke(
                        &Path::line(
                            Point::new(x, local.y),
                            Point::new(x, local.y + local.height),
                        ),
                        guide,
                    );
                    frame.stroke(
                        &Path::line(Point::new(local.x, y), Point::new(local.x + local.width, y)),
                        guide,
                    );
                }
            }

            frame.stroke(
                &hole,
                Stroke::default()
                    .with_color(Color::WHITE)
                    .with_width(BOX_BORDER_WIDTH),
            );

            // Images draw above the geometry of their layer, so the scrim
            // and the outline go in a layer of their own.
            renderer.with_layer(bounds, |renderer| {
                draw_frame(renderer, frame, bounds.position());
            });
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
            mouse::Interaction::Move
        } else {
            mouse::Interaction::default()
        }
    }
}

/// The scale the preview pixels are cut at, in pixels per logical pixel.
const PREVIEW_SCALE: f32 = 2.0;

/// The live crop of a picture. See [`preview`].
pub struct Preview<'a> {
    source: &'a Source,
    crop: Crop,
    width: f32,
}

/// The crop of `source` as `crop` shows it, [`DEFAULT_PREVIEW_WIDTH`]
/// wide and as tall as the crop box shape asks, with rounded corners or
/// cut to a circle.
///
/// The preview is cut with [`crop`] at twice its size whenever the crop
/// changes.
pub fn preview<'a>(source: &'a Source, crop: &Crop) -> Preview<'a> {
    Preview {
        source,
        crop: *crop,
        width: DEFAULT_PREVIEW_WIDTH,
    }
}

impl Preview<'_> {
    /// Sets the width ([`DEFAULT_PREVIEW_WIDTH`] by default); the height
    /// follows the crop box.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width.max(0.0);
        self
    }

    fn extent(&self) -> Size {
        Size::new(self.width, self.width / self.crop.box_aspect())
    }

    /// What the cut pixels depend on: the crop, the pixel width and the
    /// picture.
    fn key(&self) -> PreviewKey {
        (
            self.crop,
            at_least_one(self.width * PREVIEW_SCALE),
            Arc::as_ptr(&self.source.pixels) as usize,
        )
    }
}

/// The crop, pixel width and picture address a preview was cut for.
type PreviewKey = (Crop, u32, usize);

/// The tree state of a [`Preview`]: the last cut and what it was cut for.
#[derive(Debug, Clone, Default)]
struct PreviewState {
    cut: Option<(PreviewKey, Handle)>,
}

/// Lowers the alpha of the pixels of `image` outside rounded corners of
/// `radius` pixels.
fn round_corners(image: &mut image::RgbaImage, radius: f32) {
    let (width, height) = (image.width() as f32, image.height() as f32);
    let radius = radius.min(width / 2.0).min(height / 2.0);

    if radius <= 0.0 {
        return;
    }

    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let (cx, cy) = (x as f32 + 0.5, y as f32 + 0.5);
        let dx = (radius - cx).max(cx - (width - radius)).max(0.0);
        let dy = (radius - cy).max(cy - (height - radius)).max(0.0);

        if dx > 0.0 && dy > 0.0 {
            let inside = (radius - dx.hypot(dy) + 0.5).clamp(0.0, 1.0);

            pixel[3] = (f32::from(pixel[3]) * inside).round() as u8;
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Preview<'_> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<PreviewState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(PreviewState::default())
    }

    fn size(&self) -> Size<Length> {
        let extent = self.extent();

        Size::new(Length::Fixed(extent.width), Length::Fixed(extent.height))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let state = tree.state.downcast_mut::<PreviewState>();
        let key = self.key();

        if state.cut.as_ref().is_none_or(|(cut, _)| *cut != key) {
            let mut pixels = crop(&self.source.pixels, &self.crop, Some(key.1));

            if !self.crop.circle() {
                round_corners(&mut pixels, radius::MEDIUM * PREVIEW_SCALE);
            }

            let handle = Handle::from_rgba(pixels.width(), pixels.height(), pixels.into_raw());

            state.cut = Some((key, handle));
        }

        let extent = self.extent();

        layout::atomic(
            limits,
            Length::Fixed(extent.width),
            Length::Fixed(extent.height),
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let Some((_, handle)) = &tree.state.downcast_ref::<PreviewState>().cut else {
            return;
        };
        let bounds = layout.bounds();

        renderer.draw_image(
            image_core::Image::new(handle.clone()).filter_method(image_core::FilterMethod::Linear),
            bounds,
            bounds,
        );
    }
}

impl<'a, Message: 'a> From<Preview<'a>> for Element<'a, Message> {
    fn from(preview: Preview<'a>) -> Self {
        IcedElement::new(preview)
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

    const RED: [u8; 4] = [255, 0, 0, 255];
    const BLUE: [u8; 4] = [0, 0, 255, 255];

    /// A `width` × `height` picture: left half red, right half blue.
    fn halves(width: u32, height: u32) -> image::RgbaImage {
        image::RgbaImage::from_fn(width, height, |x, _| {
            image::Rgba(if x < width / 2 { RED } else { BLUE })
        })
    }

    fn landscape() -> Crop {
        Crop::new(Size::new(400, 200))
    }

    #[test]
    fn zoom_stays_between_one_and_the_largest() {
        let mut crop = landscape().with_max_zoom(3.0);

        crop.zoom_by(1.2);
        assert!((crop.zoom() - 1.2).abs() < 1e-6);
        crop.set_zoom(10.0);
        assert_eq!(crop.zoom(), 3.0);
        crop.set_zoom(0.2);
        assert_eq!(crop.zoom(), 1.0);
    }

    #[test]
    fn the_pan_keeps_the_box_covered() {
        let mut crop = landscape();

        // At zoom 1 the 2:1 picture is twice the square box wide.
        crop.pan_by(Vector::new(5.0, 5.0));
        assert_eq!(crop.offset(), Vector::new(0.5, 0.0));

        crop.set_zoom(2.0);
        crop.pan_by(Vector::new(-5.0, -5.0));
        assert_eq!(crop.offset(), Vector::new(-1.5, -0.5));

        crop.set_zoom(1.0);
        assert_eq!(crop.offset(), Vector::new(-0.5, 0.0));
    }

    #[test]
    fn turns_rotate_the_pan_and_a_flipped_picture_turns_the_other_way() {
        let mut crop = landscape().with_max_zoom(5.0);

        crop.set_zoom(2.0);
        crop.pan_by(Vector::new(0.5, 0.25));
        crop.rotate_right();
        assert_eq!(crop.quarter_turns(), 1);
        assert_eq!(crop.offset(), Vector::new(-0.25, 0.5));

        crop.rotate_left();
        assert_eq!(crop.quarter_turns(), 0);

        crop.flip_horizontally();
        assert!(crop.flipped());
        assert_eq!(crop.offset(), Vector::new(-0.5, 0.25));

        crop.rotate_right();
        assert_eq!(crop.quarter_turns(), 3);

        crop.reset();
        assert_eq!(crop, landscape());
    }

    #[test]
    fn crops_at_the_picture_resolution_or_a_width() {
        let crop = landscape();
        let image = halves(400, 200);

        // The square box covers the 200 pixel height at zoom 1.
        assert_eq!(super::crop(&image, &crop, None).dimensions(), (200, 200));
        assert_eq!(super::crop(&image, &crop, Some(64)).dimensions(), (64, 64));

        let wide = crop.with_aspect_ratio(4.0 / 3.0);

        assert_eq!(wide.output_size(None), Size::new(267, 200));
        assert_eq!(wide.output_size(Some(240)), Size::new(240, 180));
    }

    #[test]
    fn a_flipped_picture_still_turns_the_way_asked() {
        let mut crop = landscape();

        // Flipped, blue is on the left, and a clockwise turn brings the
        // left to the top.
        crop.flip_horizontally();
        crop.rotate_right();

        let result = super::crop(&halves(400, 200), &crop, Some(40));

        assert_eq!(result.get_pixel(20, 5).0, BLUE);
        assert_eq!(result.get_pixel(20, 34).0, RED);
    }

    #[test]
    fn the_pan_moves_the_cut() {
        let mut crop = landscape();

        // Moving the picture right shows its red left half.
        crop.pan_by(Vector::new(0.5, 0.0));
        assert_eq!(
            super::crop(&halves(400, 200), &crop, Some(10))
                .get_pixel(5, 5)
                .0,
            RED
        );

        crop.pan_by(Vector::new(-1.0, 0.0));
        assert_eq!(
            super::crop(&halves(400, 200), &crop, Some(10))
                .get_pixel(5, 5)
                .0,
            BLUE
        );
    }

    #[test]
    fn a_circle_crop_is_transparent_at_the_corners() {
        let crop = landscape().with_circle(true);
        let result = super::crop(&halves(400, 200), &crop, Some(40));

        assert_eq!(result.get_pixel(0, 0).0[3], 0);
        assert_eq!(result.get_pixel(39, 39).0[3], 0);
        assert_eq!(result.get_pixel(5, 20).0[3], 255);
    }

    #[test]
    fn crops_are_deterministic() {
        let mut crop = landscape().with_aspect_ratio(1.5);

        crop.set_zoom(1.7);
        crop.pan_by(Vector::new(0.13, -0.07));
        crop.rotate_left();

        let image = image::RgbaImage::from_fn(64, 48, |x, y| {
            image::Rgba([(x * 4) as u8, (y * 5) as u8, ((x + y) * 2) as u8, 255])
        });
        let crop = Crop {
            image_size: Size::new(64, 48),
            ..crop
        };

        assert_eq!(
            super::crop(&image, &crop, Some(33)),
            super::crop(&image, &crop, Some(33))
        );
    }

    #[test]
    fn sources_are_scaled_down_to_the_bound() {
        let source = Source::with_max_dimension(halves(400, 200).into(), Some(100));

        assert_eq!(source.size(), Size::new(100, 50));

        let small = Source::new(halves(40, 20).into());

        assert_eq!(small.size(), Size::new(40, 20));
        assert!(Source::decode(&[1, 2, 3, 4]).is_err());
    }

    #[test]
    fn rounded_corners_clear_only_the_corners() {
        let mut image = halves(40, 20);

        round_corners(&mut image, 8.0);

        assert_eq!(image.get_pixel(0, 0).0[3], 0);
        assert_eq!(image.get_pixel(39, 19).0[3], 0);
        assert_eq!(image.get_pixel(8, 0).0[3], 255);
        assert_eq!(image.get_pixel(20, 10).0, BLUE);
    }

    #[test]
    fn the_box_fits_the_padded_area() {
        let area = Rectangle::new(Point::new(0.0, 0.0), Size::new(400.0, 320.0));

        assert_eq!(
            crop_box(area, 1.0),
            Rectangle::new(Point::new(64.0, 24.0), Size::new(272.0, 272.0))
        );
        assert_eq!(crop_box(area, 4.0).size(), Size::new(352.0, 88.0));
    }
}
