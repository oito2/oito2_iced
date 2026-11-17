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

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{
    button, checkbox, column, container, pick_list, progress_bar, radio, row, rule, slider, text,
    text_input, toggler,
};
use iced_test::simulator;
use oito2_iced::theme::button::Class as ButtonClass;
use oito2_iced::theme::container::Class as ContainerClass;
use oito2_iced::theme::progress_bar::Class as ProgressClass;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::focusable;
use oito2_iced::{Element, Theme, focus};

fn view() -> Element<'static, ()> {
    let buttons = row![
        focusable(button("Filled").class(ButtonClass::Filled).on_press(())),
        focusable(button("Tonal").class(ButtonClass::Tonal).on_press(())),
        focusable(button("Elevated").class(ButtonClass::Elevated).on_press(())),
        focusable(button("Outlined").class(ButtonClass::Outlined).on_press(())),
        focusable(button("Text").class(ButtonClass::Text).on_press(())),
        focusable(button("Danger").class(ButtonClass::Danger).on_press(())),
        focusable(button("Disabled")),
    ]
    .spacing(8);

    let inputs = row![
        checkbox(true).label("Checked").on_toggle(|_| ()),
        checkbox(false).label("Unchecked").on_toggle(|_| ()),
        text_input("Placeholder", "").width(180),
        pick_list(["One", "Two"], Some("One"), |_| ()).width(120),
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);

    let selection = row![
        radio("On", 1, Some(1), |_| ()),
        radio("Off", 2, Some(1), |_| ()),
        toggler(true).label("On").on_toggle(|_| ()),
        toggler(false).label("Off").on_toggle(|_| ()),
        slider(0.0..=1.0, 0.4, |_| ()).width(160),
    ]
    .spacing(16)
    .align_y(iced::Alignment::Center);

    let progress = row![
        progress_bar(0.0..=1.0, 0.6).length(140).girth(6),
        progress_bar(0.0..=1.0, 0.6)
            .length(140)
            .girth(6)
            .class(ProgressClass::Success),
        progress_bar(0.0..=1.0, 0.6)
            .length(140)
            .girth(6)
            .class(ProgressClass::Error),
    ]
    .spacing(16);

    focus::navigation(
        container(
            column![
                text("Snapshot"),
                buttons,
                inputs,
                rule::horizontal(1),
                selection,
                progress
            ]
            .spacing(16),
        )
        .class(ContainerClass::Card)
        .padding(16),
    )
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (760.0, 300.0), view());

    // Focus the first button so the focus ring is part of the golden.
    ui.tap_key(Named::Tab);

    let snapshot = ui.snapshot(theme).expect("snapshot");
    oito2_iced_test_utils::golden!(&snapshot, name);
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
fn widgets_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::Blue.color(), Contrast::High);

    snapshot(&theme, "widgets_light_high_contrast");
}

#[test]
fn widgets_dark_high_contrast() {
    let theme = Theme::base(Brightness::Dark, Accent::Blue.color(), Contrast::High);

    snapshot(&theme, "widgets_dark_high_contrast");
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
