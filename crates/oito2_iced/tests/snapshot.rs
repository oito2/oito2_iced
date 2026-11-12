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

//! Golden snapshots of the styled widgets, compared with a tolerance of
//! 0.5 % of the pixels. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the
//! goldens with `OITO2_UPDATE_GOLDENS=1`.

mod support;

use iced::keyboard::key::Named;
use iced::widget::{button, checkbox, column, container, row, text, text_input};
use iced_test::simulator;
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::widget::focusable;
use oito2_iced::{Element, Theme, focus};

fn view() -> Element<'static, ()> {
    let buttons = row![
        focusable(button("Filled").class(ButtonClass::Filled).on_press(())),
        focusable(button("Tonal").class(ButtonClass::Tonal).on_press(())),
        focusable(button("Outlined").class(ButtonClass::Outlined).on_press(())),
        focusable(button("Text").class(ButtonClass::Text).on_press(())),
        focusable(button("Danger").class(ButtonClass::Danger).on_press(())),
        focusable(button("Disabled")),
    ]
    .spacing(8);

    let inputs = row![
        checkbox(true).label("Checked").on_toggle(|_| ()),
        checkbox(false).label("Unchecked").on_toggle(|_| ()),
        text_input("Placeholder", "").width(200),
    ]
    .spacing(16);

    focus::navigation(
        container(column![text("Snapshot"), buttons, inputs].spacing(16))
            .class(ContainerClass::Card)
            .padding(16),
    )
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (640.0, 200.0), view());

    // Focus the first button so the focus ring is part of the golden.
    ui.tap_key(Named::Tab);

    let snapshot = ui.snapshot(theme).expect("snapshot");
    support::assert_golden(&snapshot, name);
}

#[test]
fn widgets_light() {
    snapshot(&Theme::light(), "widgets_light");
}

#[test]
fn widgets_dark() {
    snapshot(&Theme::dark(), "widgets_dark");
}

#[test]
fn the_comparator_tolerates_small_differences_only() {
    let base = support::Image {
        width: 100,
        height: 100,
        rgba: vec![255; 100 * 100 * 4],
        file_name: String::new(),
    };

    let mut few = base.rgba.clone();
    few[..4 * 40].fill(0);
    let few = support::Image {
        rgba: few,
        file_name: String::new(),
        ..base
    };
    assert!(support::difference(&base_clone(&base), &few) <= support::TOLERANCE);

    let mut many = base_clone(&base).rgba;
    many[..4 * 60].fill(0);
    let many = support::Image {
        width: 100,
        height: 100,
        rgba: many,
        file_name: String::new(),
    };
    assert!(support::difference(&base_clone(&base), &many) > support::TOLERANCE);
}

fn base_clone(image: &support::Image) -> support::Image {
    support::Image {
        width: image.width,
        height: image.height,
        rgba: image.rgba.clone(),
        file_name: String::new(),
    }
}
