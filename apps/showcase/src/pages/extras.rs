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

//! The extras page: QR codes, a signature pad, an image cropper and a
//! navigation menu.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::segment_group;
use oito2_iced::widget::toggles;
use oito2_iced_extras::image_cropper::{self, Crop, Source};
use oito2_iced_extras::navigation_menu::{self, MenuState};
use oito2_iced_extras::qr_code::{self, ErrorLevel};
use oito2_iced_extras::signature_pad::{self, Signature};

use super::{Gallery, Message, card, page};

/// The data of the QR code examples.
const QR_DATA: [(&str, &str); 3] = [
    ("Link", "https://github.com/oito2/oito2_iced"),
    ("Text", "Olá, coração! Ação não é só intenção."),
    ("Too long", ""),
];

/// The sections the navigation menu links to directly.
pub const SECTIONS: [&str; 4] = ["Home", "Components", "Guides", "Blog"];

/// The shape of the crop box.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CropShape {
    /// A 4:3 rectangle.
    Landscape,
    /// A square.
    Square,
    /// A circle.
    Circle,
}

/// The state of the extras page.
#[derive(Debug)]
pub struct Extras {
    pub(super) qr_data: usize,
    pub(super) qr_level: ErrorLevel,
    pub(super) qr_rounded: bool,
    pub(super) signature: Signature,
    pub(super) source: Source,
    pub(super) crop: Crop,
    pub(super) shape: CropShape,
    pub(super) menu: MenuState,
    pub(super) section: usize,
    pub(super) showing: &'static str,
}

impl Default for Extras {
    fn default() -> Self {
        let source = Source::new(landscape().into());
        let crop = Crop::new(source.size()).with_aspect_ratio(4.0 / 3.0);

        Self {
            qr_data: 0,
            qr_level: ErrorLevel::Medium,
            qr_rounded: false,
            signature: Signature::new(),
            source,
            crop,
            shape: CropShape::Landscape,
            menu: MenuState::default(),
            section: 0,
            showing: SECTIONS[0],
        }
    }
}

impl Extras {
    /// Changes the shape of the crop box.
    pub(super) fn set_shape(&mut self, shape: CropShape) {
        self.shape = shape;
        match shape {
            CropShape::Landscape => {
                self.crop.set_circle(false);
                self.crop.set_aspect_ratio(4.0 / 3.0);
            }
            CropShape::Square => {
                self.crop.set_circle(false);
                self.crop.set_aspect_ratio(1.0);
            }
            CropShape::Circle => self.crop.set_circle(true),
        }
    }
}

/// A generated landscape picture: a sky gradient, a sun, hills and grass.
fn landscape() -> image::RgbaImage {
    const WIDTH: u32 = 960;
    const HEIGHT: u32 = 640;

    image::RgbaImage::from_fn(WIDTH, HEIGHT, |x, y| {
        let (fx, fy) = (x as f32 / WIDTH as f32, y as f32 / HEIGHT as f32);
        let hill = 0.62 + 0.06 * (fx * std::f32::consts::TAU * 1.5).sin();
        let grass = 0.74 + 0.05 * (fx * std::f32::consts::PI).sin();
        let sun = (fx - 0.72).hypot((fy - 0.3) * HEIGHT as f32 / WIDTH as f32) < 0.07;

        let mix = |a: [f32; 3], b: [f32; 3], t: f32| {
            let channel = |i: usize| (a[i] + (b[i] - a[i]) * t).round() as u8;
            image::Rgba([channel(0), channel(1), channel(2), 255])
        };

        if fy > grass {
            image::Rgba([38, 160, 101, 255])
        } else if fy > hill {
            image::Rgba([28, 92, 180, 255])
        } else if sun {
            image::Rgba([255, 244, 196, 255])
        } else {
            mix([34.0, 116.0, 220.0], [214.0, 190.0, 90.0], fy / hill)
        }
    })
}

/// The extras page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let extras = &gallery.extras;

    let data = segment_group::segment_group(
        QR_DATA
            .iter()
            .enumerate()
            .map(|(index, (label, _))| segment_group::segment(index, *label)),
        Some(extras.qr_data),
    )
    .on_change(Message::QrData);
    let levels = segment_group::segment_group(
        ErrorLevel::ALL.map(|level| segment_group::segment(level, level.name())),
        Some(extras.qr_level),
    )
    .on_change(Message::QrLevel);
    let qr_data = match QR_DATA.get(extras.qr_data) {
        Some((_, "")) | None => "A long text. ".repeat(300),
        Some((_, data)) => (*data).to_owned(),
    };

    let qr = card(
        "QR code",
        column![
            row![data, levels].spacing(spacing::LG).wrap(),
            row![
                toggles::switch(extras.qr_rounded).on_toggle(Message::QrRounded),
                text("Rounded modules"),
            ]
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
            row![
                qr_code::qr_code(qr_data.clone())
                    .error_level(extras.qr_level)
                    .rounded_modules(extras.qr_rounded)
                    .localizations(l10n),
                qr_code::qr_code(qr_data)
                    .error_level(extras.qr_level)
                    .rounded_modules(extras.qr_rounded)
                    .class(oito2_iced::theme::qr_code::Class::Themed)
                    .localizations(l10n),
            ]
            .spacing(spacing::LG)
            .wrap(),
        ]
        .spacing(spacing::MD),
    );

    let signature_state = if extras.signature.is_empty() {
        "Empty".to_owned()
    } else {
        let strokes = extras.signature.strokes().len();
        format!("{strokes} stroke{}", if strokes == 1 { "" } else { "s" })
    };
    let signature = card(
        "Signature pad",
        column![
            signature_pad::signature_pad(&extras.signature)
                .on_event(Message::Signature)
                .localizations(l10n),
            text(signature_state).class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    );

    let shapes = segment_group::segment_group(
        [
            segment_group::segment(CropShape::Landscape, "4:3"),
            segment_group::segment(CropShape::Square, "1:1"),
            segment_group::segment(CropShape::Circle, "Circle"),
        ],
        Some(extras.shape),
    )
    .on_change(Message::CropShape);
    let output = extras.crop.output_size(Some(240));
    let cropper = card(
        "Image cropper",
        column![
            shapes,
            image_cropper::image_cropper(&extras.source, &extras.crop)
                .on_event(Message::Cropper)
                .localizations(l10n),
            row![
                image_cropper::preview(&extras.source, &extras.crop).width(120.0),
                text(format!(
                    "Output at 240 px: {} × {}",
                    output.width, output.height
                ))
                .class(TextClass::Variant),
            ]
            .spacing(spacing::LG)
            .align_y(Alignment::Center),
        ]
        .spacing(spacing::MD),
    );

    let links = |labels: [&'static str; 3]| {
        column(labels.map(|label| {
            button::text(label)
                .on_press(Message::MenuLink(label))
                .into()
        }))
        .spacing(spacing::XS)
    };
    let menu = card(
        "Navigation menu",
        column![
            navigation_menu::navigation_menu(
                &extras.menu,
                [
                    navigation_menu::item("Home").on_press(Message::MenuSection(0)),
                    navigation_menu::item("Components")
                        .panel(links(["Buttons", "Entries", "Overlays"])),
                    navigation_menu::item("Guides").panel(links([
                        "Getting started",
                        "Styling",
                        "Animations"
                    ])),
                    navigation_menu::item("Blog").on_press(Message::MenuSection(3)),
                ],
            )
            .selected(Some(extras.section))
            .on_event(Message::Menu),
            text(format!("Showing: {}", extras.showing)).class(TextClass::Variant),
        ]
        .spacing(spacing::MD)
        .width(Length::Fill),
    );

    page(
        "Extras",
        "QR codes, signatures, image cropping and a navigation menu.",
        [qr, signature, cropper, menu],
    )
}
