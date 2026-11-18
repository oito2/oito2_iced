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

//! The motion page: animated icons, marquees, a floating panel and a
//! guided tour over the page.

use iced::widget::{Space, column, container, row, stack, text};
use iced::{Alignment, Element, Length, Rectangle};

use oito2_iced::Theme;
use oito2_iced::icon::animated::{self, Mode};
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::overlay::Placement;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::button;
use oito2_iced::widget::floating_panel::{self, PanelState};
use oito2_iced::widget::marquee::{self, Axis};
use oito2_iced::widget::segment_group;
use oito2_iced::widget::tour::{Step, Tour};
use oito2_iced::window::Labels;

use super::{Gallery, Message, card, page};

/// The id of the animated icons card, a tour target.
pub const ICONS: &str = "showcase-motion-icons";
/// The id of the marquee card, a tour target.
pub const MARQUEES: &str = "showcase-motion-marquees";
/// The id of the floating panel card, a tour target.
pub const PANEL: &str = "showcase-motion-panel";
/// The id of the button that starts the tour, a tour target that waits
/// for a tap.
pub const START: &str = "showcase-motion-start";

/// The id of the title bar of the floating panel.
pub const PANEL_BAR: &str = "showcase-motion-panel-bar";

/// The height of the area the floating panel moves in.
const PANEL_AREA: f32 = 320.0;

/// The state of the motion page.
#[derive(Debug)]
pub struct Motion {
    pub(super) mode: Mode,
    pub(super) panel: PanelState,
    pub(super) panel_open: bool,
    pub tour: Tour,
    pub(super) tour_result: Option<bool>,
}

impl Default for Motion {
    fn default() -> Self {
        Self {
            mode: Mode::Once,
            panel: PanelState::new(Rectangle::new(
                iced::Point::new(24.0, 24.0),
                iced::Size::new(300.0, 200.0),
            )),
            panel_open: true,
            tour: Tour::new(vec![
                Step::new("Welcome").description("This short tour shows the motion widgets."),
                Step::new("Animated icons")
                    .description("Pick how the icons play: once, over and over, or still.")
                    .target(ICONS),
                Step::new("Marquees")
                    .description("Long content scrolls; hover over it to pause.")
                    .target(MARQUEES)
                    .placement(Placement::Top),
                Step::new("Floating panel")
                    .description("Drag the title bar, resize from the edges, or use the arrows.")
                    .target(PANEL)
                    .placement(Placement::Top),
                Step::new("Start again")
                    .description("Press the button to finish the tour.")
                    .target(START)
                    .wait_for_tap(true),
            ]),
            tour_result: None,
        }
    }
}

/// The animated icons with their names.
const ANIMATED: [(&str, animated::Data); 10] = [
    ("ok", animated::OK),
    ("ok filled", animated::OK_FILLED),
    ("heart", animated::HEART),
    ("heart filled", animated::HEART_FILLED),
    ("star", animated::STAR),
    ("star filled", animated::STAR_FILLED),
    ("star half", animated::STAR_HALF_FILLED),
    ("compass", animated::COMPASS),
    ("compass filled", animated::COMPASS_FILLED),
    ("no network", animated::NO_NETWORK),
];

/// The motion page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let motion = &gallery.motion;

    let modes = segment_group::segment_group(
        [
            segment_group::segment(Mode::Once, "Once"),
            segment_group::segment(Mode::Repeat, "Repeat"),
            segment_group::segment(Mode::Still, "Still"),
        ],
        Some(motion.mode),
    )
    .on_change(Message::IconMode);

    let icons = row(ANIMATED.map(|(name, data)| {
        column![
            animated::icon(data).size(40.0).mode(motion.mode),
            text(name).class(TextClass::Variant).size(12),
        ]
        .spacing(spacing::XS)
        .align_x(Alignment::Center)
        .width(88)
        .into()
    }))
    .spacing(spacing::SM)
    .wrap();

    let icons = container(card(
        "Animated icons",
        column![modes, icons].spacing(spacing::MD),
    ))
    .id(ICONS);

    let ticker = marquee::marquee(text(
        "oito2_iced 0.1.0 · Linux, Windows and macOS · GPL-3.0-or-later · \
         OITO2 Inter and Lucide icons · Material 3 tokens from oito2_ui",
    ));
    let symbols = marquee::marquee(
        row([
            lucide::SUN,
            lucide::CLOUD,
            lucide::CLOUD_RAIN,
            lucide::SNOWFLAKE,
            lucide::WIND,
            lucide::MOON,
            lucide::STAR,
            lucide::ZAP,
            lucide::UMBRELLA,
            lucide::THERMOMETER,
            lucide::DROPLETS,
            lucide::SUNRISE,
            lucide::SUNSET,
            lucide::RAINBOW,
            lucide::CLOUD_SUN,
            lucide::CLOUD_LIGHTNING,
            lucide::CLOUD_SNOW,
            lucide::CLOUD_FOG,
            lucide::TORNADO,
            lucide::LEAF,
        ]
        .map(|icon| oito2_iced::icon::icon(icon).size(24.0).into()))
        .spacing(spacing::LG),
    )
    .reverse(true)
    .speed(60.0);
    let vertical = marquee::marquee(
        column(
            [
                "Build 1042 passed",
                "Release 0.1.0 tagged",
                "Docs published",
                "Parity check green",
                "Goldens updated",
                "Showcase rebuilt",
            ]
            .map(|line| text(line).into()),
        )
        .spacing(spacing::SM),
    )
    .axis(Axis::Vertical)
    .speed(20.0)
    .height(72)
    .width(240);

    let marquees = container(card(
        "Marquee",
        column![
            container(ticker).max_width(480),
            container(symbols).max_width(360),
            vertical,
            text("Hover over a marquee to pause it.").class(TextClass::Variant),
        ]
        .spacing(spacing::MD),
    ))
    .id(MARQUEES);

    let rect = motion.panel.rect;
    let area_content = container(
        text(format!(
            "x {:.0}, y {:.0}, {:.0} × {:.0}",
            rect.x, rect.y, rect.width, rect.height
        ))
        .class(TextClass::Variant),
    )
    .center(Length::Fill);
    let area: Element<'a, Message, Theme> = if motion.panel_open {
        stack![
            area_content,
            floating_panel::floating_panel(
                motion.panel,
                "Inspector",
                container(
                    column![
                        text("Width: 320"),
                        text("Height: 240"),
                        text("Opacity: 100%"),
                    ]
                    .spacing(spacing::XS),
                )
                .padding(spacing::MD),
            )
            .on_change(Message::FloatingPanel)
            .on_close(Message::PanelOpen(false))
            .id(PANEL_BAR)
            .labels(Labels::new(l10n)),
        ]
        .into()
    } else {
        stack![
            area_content,
            container(button::outlined("Open the panel").on_press(Message::PanelOpen(true)))
                .padding(spacing::MD),
        ]
        .into()
    };
    let area = container(area)
        .class(ContainerClass::Background)
        .width(Length::Fill)
        .height(PANEL_AREA)
        .clip(true);

    let panel = container(card("Floating panel", area)).id(PANEL);

    let result = match motion.tour_result {
        None => "Not run yet",
        Some(true) => "Completed",
        Some(false) => "Skipped",
    };
    let tour = card(
        "Tour",
        row![
            container(button::filled("Start the tour").on_press(Message::StartTour)).id(START),
            text(result).class(TextClass::Variant),
            Space::new().width(Length::Fill),
        ]
        .spacing(spacing::LG)
        .align_y(Alignment::Center),
    );

    page(
        "Motion",
        "Animated icons, marquees, floating panels and guided tours.",
        [tour, icons.into(), marquees.into(), panel.into()],
    )
}
