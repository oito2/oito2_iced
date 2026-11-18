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

//! The ImageCropper page: zoom, pan, rotation, a circle crop, a live
//! preview and the cropped result.

use iced::widget::image::Handle;
use iced::widget::{column, image, row};
use iced::{Alignment, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::controls::check_button;
use oito2_iced::widget::toggles;
use oito2_iced_extras::image_cropper::{Crop, Event, Source, image_cropper, preview};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("image_cropper.rs");

/// Which cropper of the page an event is for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Which {
    /// The cropper of the preview.
    Preview,
    /// The cropper with a circle and a result.
    Main,
    /// The cropper without its toolbar.
    Plain,
}

/// The state of the page.
#[derive(Debug)]
pub struct State {
    source: Source,
    preview: Crop,
    crop: Crop,
    plain: Crop,
    result: Option<Handle>,
}

impl Default for State {
    fn default() -> Self {
        // #region preview-data
        let source = Source::new(landscape().into());
        let preview = Crop::new(source.size()).with_aspect_ratio(4.0 / 3.0);
        // #endregion

        Self {
            preview,
            crop: Crop::new(source.size()).with_aspect_ratio(4.0 / 3.0),
            plain: Crop::new(source.size()).with_max_zoom(3.0),
            source,
            result: None,
        }
    }
}

/// A generated landscape picture to crop, so the page needs no asset: a
/// sky gradient, a sun, hills and grass.
fn landscape() -> ::image::RgbaImage {
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 640;

    ::image::RgbaImage::from_fn(WIDTH, HEIGHT, |x, y| {
        let (fx, fy) = (x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
        let hill = 0.62 + 0.06 * (fx * std::f32::consts::TAU * 1.5).sin();
        let grass = 0.74 + 0.05 * (fx * std::f32::consts::PI).sin();
        let sun = (fx - 0.72).hypot((fy - 0.3) * HEIGHT as f32 / WIDTH as f32) < 0.07;

        let mix = |a: [f32; 3], b: [f32; 3], t: f32| {
            let channel = |i: usize| (a[i] + (b[i] - a[i]) * t).round() as u8;
            ::image::Rgba([channel(0), channel(1), channel(2), 255])
        };

        if fy > grass {
            ::image::Rgba([38, 160, 101, 255])
        } else if fy > hill {
            ::image::Rgba([28, 92, 180, 255])
        } else if sun {
            ::image::Rgba([255, 244, 196, 255])
        } else {
            mix([34.0, 116.0, 220.0], [214.0, 190.0, 90.0], fy / hill)
        }
    })
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// A cropper changed its crop.
    Cropper(Which, Event),
    /// The circle option changed.
    Circle(bool),
    /// Cuts the result out.
    Crop,
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Cropper(which, event) => match which {
            Which::Preview => state.preview.apply(event),
            Which::Main => state.crop.apply(event),
            Which::Plain => state.plain.apply(event),
        },
        // #region crop-update
        Message::Crop => {
            let result = state.source.crop(&state.crop, Some(240));
            let (width, height) = result.dimensions();
            state.result = Some(Handle::from_rgba(width, height, result.into_raw()));
        }
        // #endregion
        Message::Circle(circle) => state.crop.set_circle(circle),
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;
    let width = Length::Fixed(400.0);

    let preview_example = doc::example(
        ctx,
        SOURCE,
        &["preview-data", "preview"],
        iced::widget::container(
            // #region preview
            image_cropper(&state.source, &state.preview)
                .on_event(|event| Message::Cropper(Which::Preview, event))
                .localizations(l10n),
            // #endregion
        )
        .width(width),
    );

    let circle = doc::example(
        ctx,
        SOURCE,
        &["circle", "crop-update"],
        // #region circle
        column![
            check_button(
                toggles::checkbox(state.crop.circle())
                    .on_change(|value| Message::Circle(value.unwrap_or_default())),
                "Circle",
            ),
            image_cropper(&state.source, &state.crop)
                .on_event(|event| Message::Cropper(Which::Main, event))
                .localizations(l10n),
            row![
                preview(&state.source, &state.crop).width(120.0),
                button::filled("Crop").on_press(Message::Crop),
            ]
            .push(
                state
                    .result
                    .as_ref()
                    .map(|result| image(result.clone()).width(120)),
            )
            .spacing(spacing::MD)
            .align_y(Alignment::Center),
        ]
        .spacing(spacing::SM)
        .width(width),
        // #endregion
    );

    let plain = doc::example(
        ctx,
        SOURCE,
        &["plain"],
        // #region plain
        column![
            image_cropper(&state.source, &state.plain)
                .height(240.0)
                .show_toolbar(false)
                .on_event(|event| Message::Cropper(Which::Plain, event))
                .localizations(l10n),
            row([
                ("Zoom in", Event::ZoomBy(1.5)),
                ("Zoom out", Event::ZoomBy(1.0 / 1.5)),
                ("Rotate", Event::RotateRight),
                ("Flip", Event::FlipHorizontally),
                ("Reset", Event::Reset),
            ]
            .map(|(label, event)| {
                button::outlined(label)
                    .on_press(Message::Cropper(Which::Plain, event))
                    .into()
            }),)
            .spacing(spacing::SM)
            .wrap(),
        ]
        .spacing(spacing::SM)
        .align_x(Alignment::Center)
        .width(width),
        // #endregion
    );

    doc::page(
        "ImageCropper",
        "Zooms, pans, rotates and flips an image under a crop box, and cuts the cropped part \
         out as an image.",
        [
            preview_example,
            doc::heading("Usage"),
            doc::prose(
                "The picture is a `Source`, made once from an `image::DynamicImage` (or \
                 `Source::decode` of encoded bytes) and scaled down to at most 4096 pixels on \
                 its longest side. The zoom, pan, turns and mirror live in the application, in \
                 a `Crop`: `image_cropper` shows it and publishes `image_cropper::Event`s \
                 through `on_event`, which `Crop::apply` applies. The zoom goes from 1, where \
                 the image just covers the box, to the largest zoom, 5 by default; the image \
                 always covers the box.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_extras::image_cropper::{Crop, Source, image_cropper};",
            ),
            doc::prose(
                "The crop box has the aspect ratio of the `Crop` (`with_aspect_ratio`, width \
                 over height, 1 by default) and fits the crop area, which takes the available \
                 width and `height` pixels, 320 by default. Drag to move the image and scroll \
                 the mouse wheel to zoom.",
            ),
            doc::heading("Examples"),
            doc::subheading("Circle, live preview and crop"),
            doc::prose(
                "With a circle (`with_circle` or `set_circle`) the crop box is a circle and the \
                 aspect ratio is ignored. `preview` shows the crop live at its `width`. \
                 `Source::crop` cuts the result out as an `image::RgbaImage`, at the source \
                 resolution or the width given, cut to a circle for a circle crop; \
                 `Crop::output_size` tells its size beforehand.",
            ),
            circle,
            doc::subheading("Without the toolbar"),
            doc::prose(
                "The toolbar under the crop area zooms, rotates, flips and resets. With \
                 `show_toolbar(false)` only the crop area shows, and the application applies \
                 the events itself, or calls `zoom_by`, `pan_by`, `rotate_left`, \
                 `rotate_right`, `flip_horizontally` and `reset` on the `Crop`. Here the \
                 largest zoom is 3.",
            ),
            plain,
            doc::heading("Keyboard"),
            doc::table(
                &["Key", "Action"],
                &[
                    vec!["Tab", "Moves the focus to the crop area and the toolbar."],
                    vec!["Arrow keys", "Move the image; more with Shift."],
                    vec!["+, =", "Zoom in."],
                    vec!["-", "Zooms out."],
                    vec!["r", "Turns the image a quarter clockwise."],
                    vec!["R", "Turns the image a quarter counterclockwise."],
                    vec!["h", "Mirrors the image horizontally."],
                ],
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_extras::image_cropper::image_cropper",
                    "oito2_iced_extras::image_cropper::ImageCropper",
                    "oito2_iced_extras::image_cropper::Source",
                    "oito2_iced_extras::image_cropper::Crop",
                    "oito2_iced_extras::image_cropper::Event",
                    "oito2_iced_extras::image_cropper::preview",
                    "oito2_iced_extras::image_cropper::Preview",
                    "oito2_iced_extras::image_cropper::crop",
                ],
            ),
        ],
    )
}
