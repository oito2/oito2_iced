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

//! Diagonal clips: the pointer inside and outside the triangle, two clips
//! splitting an area, the drawn triangle and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{Space, column, container, row, stack};
use iced::{Color, Length, Point};
use iced_test::simulator;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::diagonal_clip::{DiagonalCorner, diagonal_clip};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tapped(DiagonalCorner),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// A 100 × 100 button that reports `corner`, clipped to its triangle.
fn clipped(corner: DiagonalCorner) -> Element<'static, Message> {
    diagonal_clip(
        corner,
        iced::widget::button(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .on_press(Message::Tapped(corner)),
    )
    .into()
}

fn sized(content: Element<'static, Message>) -> Element<'static, Message> {
    container(content).width(100).height(100).into()
}

fn tap(ui: &mut Simulator<'_>, x: f32, y: f32) {
    ui.point_at(Point::new(x, y));
    ui.simulate(simulator::click());
}

#[test]
fn the_pointer_reaches_only_the_triangle_of_the_corner() {
    let mut ui =
        Simulator::with_settings(support::settings(), sized(clipped(DiagonalCorner::TopLeft)));

    tap(&mut ui, 10.0, 10.0);
    tap(&mut ui, 90.0, 90.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Tapped(DiagonalCorner::TopLeft)]
    );
}

#[test]
fn two_clips_split_an_area() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        sized(
            stack![
                clipped(DiagonalCorner::TopRight),
                clipped(DiagonalCorner::BottomLeft),
            ]
            .into(),
        ),
    );

    tap(&mut ui, 90.0, 10.0);
    tap(&mut ui, 10.0, 90.0);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![
            Message::Tapped(DiagonalCorner::TopRight),
            Message::Tapped(DiagonalCorner::BottomLeft),
        ]
    );
}

/// A 100 × 100 swatch in `color`.
fn swatch(color: fn(&Theme) -> Color) -> Element<'static, Message> {
    container(Space::new())
        .width(100)
        .height(100)
        .style(move |theme: &Theme| container::Style {
            background: Some(color(theme).into()),
            ..container::Style::default()
        })
        .into()
}

#[test]
fn only_the_triangle_is_drawn() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (100.0, 100.0),
        diagonal_clip(DiagonalCorner::BottomRight, swatch(|_| Color::BLACK)),
    );
    let theme = Theme::light();
    let image = support::pixels(
        &ui.snapshot(&theme).expect("snapshot"),
        "diagonal_clip_drawn",
    );

    let at = |x: f32, y: f32| support::pixel_at(&image, Point::new(x, y), 100.0);

    assert_eq!(at(90.0, 90.0), Color::BLACK);
    assert_ne!(at(10.0, 10.0), Color::BLACK);
    assert_eq!(at(55.0, 52.0), Color::BLACK);
    assert_ne!(at(45.0, 52.0), Color::BLACK);
}

fn catalog() -> Element<'static, Message> {
    let corner = |corner: DiagonalCorner| {
        container(diagonal_clip(corner, swatch(|theme| theme.roles().primary)))
            .width(48)
            .height(48)
    };
    let split = |light: fn(&Theme) -> Color, dark: fn(&Theme) -> Color| {
        container(stack![
            diagonal_clip(DiagonalCorner::TopLeft, swatch(light)),
            diagonal_clip(DiagonalCorner::BottomRight, swatch(dark)),
        ])
        .width(100)
        .height(100)
    };

    column![
        row![
            corner(DiagonalCorner::TopLeft),
            corner(DiagonalCorner::TopRight),
            corner(DiagonalCorner::BottomLeft),
            corner(DiagonalCorner::BottomRight),
        ]
        .spacing(8),
        split(
            |theme| theme.roles().primary_container,
            |theme| theme.roles().on_primary_container,
        ),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (260.0, 200.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn diagonal_clip_light() {
    snapshot(&Theme::light(), "diagonal_clip_light");
}

#[test]
fn diagonal_clip_dark() {
    snapshot(&Theme::dark(), "diagonal_clip_dark");
}

#[test]
fn diagonal_clip_dark_high_contrast() {
    let theme = Theme::base(Brightness::Dark, Accent::default().color(), Contrast::High);

    snapshot(&theme, "diagonal_clip_dark_high_contrast");
}
