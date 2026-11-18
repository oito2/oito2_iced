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

//! The QrCode page: content, error levels, module shapes, colors, a center
//! image and data that doesn't fit.

use iced::widget::image::Handle;
use iced::widget::{column, container, text};
use iced::{Color, Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::theme::qr_code::Class as QrClass;
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::controls::check_button;
use oito2_iced::widget::segment_group::{segment, segment_group};
use oito2_iced::widget::text_field::text_field;
use oito2_iced::widget::toggles;
use oito2_iced_extras::qr_code::{self, ErrorLevel, qr_code};

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("qr_code.rs");

/// The state of the page.
#[derive(Debug)]
pub struct State {
    data: String,
    level: ErrorLevel,
    rounded: bool,
    logo: Handle,
}

impl Default for State {
    fn default() -> Self {
        Self {
            data: "https://oito2.github.io/oito2_ui/".to_owned(),
            level: ErrorLevel::Medium,
            rounded: false,
            logo: logo(),
        }
    }
}

/// A small round mark to place in the middle of a code: a blue disk with
/// a white head and body.
fn logo() -> Handle {
    const SIDE: u32 = 96;

    let inside = |x: u32, y: u32, cx: f32, cy: f32, r: f32| {
        (x as f32 + 0.5 - cx).hypot(y as f32 + 0.5 - cy) <= r
    };
    let mark = image::RgbaImage::from_fn(SIDE, SIDE, |x, y| {
        if inside(x, y, 48.0, 32.0, 14.0) || inside(x, y, 48.0, 62.0, 18.0) {
            image::Rgba([0xFF, 0xFF, 0xFF, 0xFF])
        } else if inside(x, y, 48.0, 48.0, 48.0) {
            {
                let [r, g, b, a] = Accent::Blue.color().into_rgba8();
                image::Rgba([r, g, b, a])
            }
        } else {
            image::Rgba([0, 0, 0, 0])
        }
    });

    Handle::from_rgba(SIDE, SIDE, mark.into_raw())
}

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// The text changed.
    Data(String),
    /// The error level changed.
    Level(ErrorLevel),
    /// The rounded modules option changed.
    Rounded(bool),
}

/// Applies a message.
pub fn update(state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Data(data) => state.data = data,
        Message::Level(level) => state.level = level,
        Message::Rounded(rounded) => state.rounded = rounded,
    }

    Task::none()
}

/// The page.
pub fn view<'a>(state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let l10n = &ctx.l10n;

    let preview = doc::example(
        ctx,
        SOURCE,
        &["preview"],
        // #region preview
        qr_code("https://oito2.github.io/oito2_ui/").localizations(l10n),
        // #endregion
    );

    let options = doc::example(
        ctx,
        SOURCE,
        &["options"],
        column![
            text_field("https://…", &state.data)
                .label("Text or link")
                .on_input(Message::Data),
            segment_group(
                ErrorLevel::ALL.map(|level| segment(level, level.name())),
                Some(state.level),
            )
            .on_change(Message::Level),
            check_button(
                toggles::checkbox(state.rounded)
                    .on_change(|value| Message::Rounded(value.unwrap_or_default())),
                "Rounded modules",
            ),
            container(
                // #region options
                qr_code(state.data.clone())
                    .error_level(state.level)
                    .rounded_modules(state.rounded)
                    .localizations(l10n),
                // #endregion
            )
            .center_x(Length::Fill),
        ]
        .spacing(spacing::MD)
        .width(Length::Fixed(360.0)),
    );

    let colors = doc::example(
        ctx,
        SOURCE,
        &["colors"],
        // #region colors
        qr_code(state.data.clone())
            .error_level(state.level)
            .rounded_modules(state.rounded)
            .color(Color::from_rgb8(0x1A, 0x3A, 0x6B))
            .background_color(Color::from_rgb8(0xF1, 0xF5, 0xFB))
            .localizations(l10n),
        // #endregion
    );

    let themed = doc::example(
        ctx,
        SOURCE,
        &["themed"],
        // #region themed
        qr_code(state.data.clone())
            .class(QrClass::Themed)
            .localizations(l10n),
        // #endregion
    );

    let image = doc::example(
        ctx,
        SOURCE,
        &["image"],
        // #region image
        qr_code(state.data.clone())
            .rounded_modules(state.rounded)
            .center_image(state.logo.clone())
            .localizations(l10n),
        // #endregion
    );

    let error = doc::example(
        ctx,
        SOURCE,
        &["error"],
        // #region error
        qr_code("oito2 ".repeat(600))
            .error(container(text("Too long to encode")).center(Length::Fixed(qr_code::SIZE))),
        // #endregion
    );

    doc::page(
        "QrCode",
        "Draws a QR code of a text or link, with a choice of error correction, module shape, \
         colors and a center logo.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "Give `qr_code` the data to encode. It paints a square of `size` pixels, 160 by \
                 default, with the standard four-module quiet zone included. Encoded grids are \
                 kept for reuse, so a view that is built again doesn't encode the text again.",
            ),
            doc::code(
                ctx,
                "use oito2_iced_extras::qr_code::{ErrorLevel, qr_code};",
            ),
            doc::note(
                "The default black on white is what every scanner reads, in light and dark \
                 themes. Custom colors need strong contrast, with `color` darker than \
                 `background_color`.",
            ),
            doc::heading("Examples"),
            doc::subheading("Content, level and modules"),
            doc::prose(
                "`error_level` sets how much of the code can be damaged or covered and still \
                 read: `Low` about 7 %, `Medium` (the default) about 15 %, `Quartile` about \
                 25 % and `High` about 30 %. Higher levels take more modules. \
                 `rounded_modules` draws the dark modules as rounded squares.",
            ),
            options,
            doc::subheading("Colors"),
            doc::prose(
                "`color` paints the dark modules and `background_color` the light modules and \
                 the quiet zone.",
            ),
            colors,
            doc::prose(
                "The `Themed` class draws the modules in the text color on the surface, \
                 following the theme.",
            ),
            themed,
            doc::subheading("Center image"),
            doc::prose(
                "A `center_image`, such as a logo, covers the middle of the modules, a \
                 `center_image_fraction` of their width (0.22 by default, at most 0.3). The \
                 error level is then always `High`, which recovers the most covered modules, \
                 so the code still reads.",
            ),
            image,
            doc::subheading("Data that doesn't fit"),
            doc::prose(
                "When the data is too long for a QR code at the chosen level, a localized \
                 message shows in its place, or the element given to `error`.",
            ),
            error,
            doc::subheading("Modules only"),
            doc::prose(
                "`qr_code::modules` returns the module grid of a text without painting it, for \
                 exporting a code or drawing it another way. It returns `DataTooLong` when the \
                 data doesn't fit.",
            ),
            doc::code(
                ctx,
                "let grid = qr_code::modules(\"oito2\", ErrorLevel::High)?;\n\
                 for row in grid.rows() {\n    \
                     let line: String = row.iter().map(|&dark| if dark { '#' } else { ' ' }).collect();\n    \
                     println!(\"{line}\");\n\
                 }",
            ),
            doc::api(
                ctx,
                &[
                    "oito2_iced_extras::qr_code::qr_code",
                    "oito2_iced_extras::qr_code::QrCode",
                    "oito2_iced_extras::qr_code::ErrorLevel",
                    "oito2_iced_extras::qr_code::modules",
                    "oito2_iced_extras::qr_code::Modules",
                    "oito2_iced_extras::qr_code::DataTooLong",
                    "oito2_iced_extras::qr_code::SIZE",
                    "oito2_iced::theme::qr_code::Class",
                ],
            ),
        ],
    )
}
