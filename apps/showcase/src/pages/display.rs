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

//! The display page: avatars, timers, swaps, carousels, page indicators,
//! diagonal clips and draggable content.

use std::time::{Duration, Instant};

use iced::widget::{column, container, row, stack, text};
use iced::{Alignment, Element, Length, Point};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::avatar::{self, Shape, Status};
use oito2_iced::widget::button;
use oito2_iced::widget::carousel::{self, Controller};
use oito2_iced::widget::diagonal_clip::{DiagonalCorner, diagonal_clip};
use oito2_iced::widget::draggable::{self, draggable};
use oito2_iced::widget::page_indicator::{PageIndicatorLayout, page_indicator};
use oito2_iced::widget::swap::{SwapEffect, swap};
use oito2_iced::widget::timer::{Timer, timer_text};

use super::{Gallery, Message, card, page};

/// The id of the carousel.
pub const CAROUSEL: &str = "showcase-carousel";

/// The state of the display page.
#[derive(Debug)]
pub struct Display {
    pub(super) timer: Timer,
    pub(super) stopwatch: Timer,
    pub(super) playing: bool,
    pub(super) carousel: Controller,
    pub(super) indicator: usize,
    pub(super) note: Point,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            timer: Timer::countdown(Duration::from_secs(90)),
            stopwatch: Timer::stopwatch(),
            playing: false,
            carousel: Controller::new(),
            indicator: 2,
            note: Point::new(120.0, 64.0),
        }
    }
}

/// What the timer buttons do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerAction {
    /// Starts or resumes both timers.
    Start,
    /// Pauses both timers.
    Pause,
    /// Resets both timers.
    Reset,
}

impl Display {
    /// Applies a timer button at `now`.
    pub fn timer(&mut self, action: TimerAction, now: Instant) {
        for timer in [&mut self.timer, &mut self.stopwatch] {
            match action {
                TimerAction::Start => timer.start(now),
                TimerAction::Pause => timer.pause(now),
                TimerAction::Reset => timer.reset(),
            }
        }
    }
}

/// The names of the people the avatars show.
const PEOPLE: [&str; 6] = [
    "Ana Souza",
    "Bruno Lima",
    "Carla Dias",
    "Davi Rocha",
    "Eva Nunes",
    "Fábio Reis",
];

/// The display page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let display = &gallery.display;

    let avatars = row![
        avatar::avatar().name(PEOPLE[0]).status(Status::Online),
        avatar::avatar()
            .name(PEOPLE[1])
            .shape(Shape::Square)
            .status(Status::Busy),
        avatar::avatar().status(Status::Away),
        avatar::avatar().name(PEOPLE[2]).size(56.0),
        avatar::group(PEOPLE.map(|name| avatar::avatar().name(name).size(32.0)))
            .max(4)
            .size(32.0),
    ]
    .spacing(spacing::LG)
    .align_y(Alignment::Center)
    .wrap();

    let running = display.timer.is_running();
    let timers = column![
        row![
            column![
                text("Countdown").class(TextClass::Variant),
                timer_text(&display.timer)
                    .style(typography::HEADLINE_SMALL)
                    .on_complete(Message::TimerTick),
            ]
            .spacing(spacing::XS),
            column![
                text("Stopwatch").class(TextClass::Variant),
                timer_text(&display.stopwatch)
                    .style(typography::HEADLINE_SMALL)
                    .tenths(true),
            ]
            .spacing(spacing::XS),
        ]
        .spacing(spacing::XL),
        row![
            if running {
                button::tonal("Pause").on_press(Message::Timer(TimerAction::Pause))
            } else {
                button::filled("Start").on_press(Message::Timer(TimerAction::Start))
            },
            button::outlined("Reset").on_press(Message::Timer(TimerAction::Reset)),
        ]
        .spacing(spacing::SM),
    ]
    .spacing(spacing::MD);

    let swaps = row![
        button::button(swap(display.playing, text("Play"), text("Pause")).effect(SwapEffect::Fade))
            .on_press(Message::Playing(!display.playing)),
        button::button(
            swap(
                display.playing,
                oito2_iced::icon::icon(lucide::PLAY),
                oito2_iced::icon::icon(lucide::PAUSE)
            )
            .effect(SwapEffect::Scale)
        )
        .on_press(Message::Playing(!display.playing)),
        button::button(
            swap(
                display.playing,
                oito2_iced::icon::icon(lucide::SUN),
                oito2_iced::icon::icon(lucide::MOON)
            )
            .effect(SwapEffect::Rotate)
        )
        .on_press(Message::Playing(!display.playing)),
    ]
    .spacing(spacing::SM);

    let slides = carousel::carousel(
        &display.carousel,
        ["Mountains", "Forest", "Sea", "Desert"].map(|title| {
            container(oito2_iced::widget::styled_text(
                title,
                typography::HEADLINE_SMALL,
            ))
            .class(ContainerClass::Sidebar)
            .center(Length::Fill)
            .into()
        }),
    )
    .height(200.0)
    .localizations(l10n)
    .id(CAROUSEL)
    .on_page_change(Message::Slide);

    let indicators = row![
        page_indicator(5, display.indicator).on_select(Message::Indicator),
        page_indicator(30, display.indicator)
            .layout(PageIndicatorLayout::Bounded)
            .on_select(Message::Indicator),
    ]
    .spacing(spacing::XL)
    .align_y(Alignment::Center)
    .wrap();

    let clips = row![
        container(stack![
            diagonal_clip(
                DiagonalCorner::TopLeft,
                container(text(""))
                    .class(ContainerClass::Inverse)
                    .center(Length::Fill)
            ),
            diagonal_clip(
                DiagonalCorner::BottomRight,
                container(text(""))
                    .class(ContainerClass::Sidebar)
                    .center(Length::Fill)
            ),
        ])
        .width(96)
        .height(96),
        container(stack![
            container(text("Board").class(TextClass::Variant))
                .class(ContainerClass::Card)
                .padding(spacing::MD)
                .width(Length::Fill)
                .height(Length::Fill),
            draggable(
                display.note,
                container(text(draggable::hint(l10n)))
                    .class(ContainerClass::Inverse)
                    .padding(spacing::MD)
                    .max_width(200),
            )
            .on_move(Message::Note),
        ])
        .width(Length::Fixed(360.0))
        .height(Length::Fixed(200.0)),
    ]
    .spacing(spacing::XL)
    .align_y(Alignment::Center)
    .wrap();

    page(
        "Display",
        "People, time and content that moves; animations end in their final state when turned off.",
        [
            card("Avatars", avatars),
            card("Timers", timers),
            card("Swap", swaps),
            card("Carousel", slides),
            card("Page indicators", indicators),
            card("Diagonal clip and draggable", clips),
        ],
    )
}
