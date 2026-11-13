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

//! Check boxes, radios and switches: clicks, keyboard activation, focus
//! traversal, sizes per density and golden snapshots of the catalog. Run
//! with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::toggles;
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Check(Option<bool>),
    Pick(Option<u8>),
    Switch(bool),
}

type Ui = simulator::Simulator<'static, Message, Theme>;

fn ui(view: impl Into<Element<'static, Message>>) -> Ui {
    simulator::Simulator::with_settings(support::settings(), oito2_iced::focus::navigation(view))
}

fn bounds(ui: &mut Ui, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn focused(ui: &mut Ui) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn click(ui: &mut Ui, id: &'static str) {
    let center = bounds(ui, id).center();
    ui.point_at(center);
    let _ = ui.simulate(simulator::click());
}

#[test]
fn a_click_publishes_the_next_value() {
    let mut ui = ui(column![
        toggles::checkbox(false).on_change(Message::Check).id("off"),
        toggles::checkbox(true)
            .tristate(true)
            .on_change(Message::Check)
            .id("tristate"),
        toggles::checkbox(None)
            .tristate(true)
            .on_change(Message::Check)
            .id("mixed"),
        toggles::radio(2, Some(1))
            .on_change(Message::Pick)
            .id("two"),
        toggles::switch(false)
            .on_toggle(Message::Switch)
            .id("switch"),
    ]);

    for id in ["off", "tristate", "mixed", "two", "switch"] {
        click(&mut ui, id);
    }

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![
            Message::Check(Some(true)),
            Message::Check(None),
            Message::Check(Some(false)),
            Message::Pick(Some(2)),
            Message::Switch(true),
        ]
    );
}

#[test]
fn a_selected_radio_ignores_clicks_unless_toggleable() {
    let mut ui = ui(column![
        toggles::radio(1, Some(1))
            .on_change(Message::Pick)
            .id("fixed"),
        toggles::radio(1, Some(1))
            .toggleable(true)
            .on_change(Message::Pick)
            .id("toggleable"),
    ]);

    click(&mut ui, "fixed");
    click(&mut ui, "toggleable");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Pick(None)]);
}

#[test]
fn disabled_controls_ignore_clicks_and_tab() {
    let mut ui = ui(column![
        toggles::checkbox::<Message>(false).id("no_callback"),
        toggles::switch(false)
            .on_toggle(Message::Switch)
            .enabled(false)
            .id("disabled"),
        toggles::radio(1, None).on_change(Message::Pick).id("radio"),
    ]);
    let radio = bounds(&mut ui, "radio");

    click(&mut ui, "no_callback");
    click(&mut ui, "disabled");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(radio));

    let messages: Vec<Message> = ui.into_messages().collect();
    assert!(messages.is_empty(), "{messages:?}");
}

#[test]
fn tab_focuses_and_space_or_enter_toggles() {
    let mut ui = ui(row![
        toggles::checkbox(false)
            .on_change(Message::Check)
            .id("check"),
        toggles::switch(true)
            .on_toggle(Message::Switch)
            .id("switch"),
    ]);
    let check = bounds(&mut ui, "check");
    let switch = bounds(&mut ui, "switch");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(check));
    ui.tap_key(Named::Space);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(switch));
    ui.tap_key(Named::Enter);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(
        messages,
        vec![Message::Check(Some(true)), Message::Switch(false)]
    );
}

#[test]
fn controls_follow_the_target_of_the_density() {
    for (density, target, switch_width) in
        [(Density::Pointer, 34.0, 50.0), (Density::Touch, 48.0, 60.0)]
    {
        density::set(density);

        let mut ui = ui(row![
            toggles::checkbox(false)
                .on_change(Message::Check)
                .id("check"),
            toggles::radio(1, None).on_change(Message::Pick).id("radio"),
            toggles::switch(false)
                .on_toggle(Message::Switch)
                .id("switch"),
        ]);
        let check = bounds(&mut ui, "check");
        let radio = bounds(&mut ui, "radio");
        let switch = bounds(&mut ui, "switch");

        assert_eq!((check.width, check.height), (target, target), "{density:?}");
        assert_eq!((radio.width, radio.height), (target, target), "{density:?}");
        assert_eq!(
            (switch.width, switch.height),
            (switch_width, target),
            "{density:?}"
        );
    }

    density::set(Density::Pointer);
}

#[test]
fn controls_ask_for_the_hand_cursor_only_while_enabled() {
    let _ = ui(column![]);

    let enabled: Element<'static, Message> =
        toggles::checkbox(false).on_change(Message::Check).into();
    let disabled: Element<'static, Message> = toggles::checkbox(false).into();
    let point = iced::Point::new(17.0, 17.0);

    assert_eq!(
        support::mouse_interaction(enabled, (100.0, 100.0), point),
        iced::mouse::Interaction::Pointer
    );
    assert_ne!(
        support::mouse_interaction(disabled, (100.0, 100.0), point),
        iced::mouse::Interaction::Pointer
    );
}

fn catalog() -> Element<'static, Message> {
    let checkboxes = row![
        toggles::checkbox(false).on_change(Message::Check),
        toggles::checkbox(true).on_change(Message::Check),
        toggles::checkbox(None)
            .tristate(true)
            .on_change(Message::Check),
        toggles::checkbox::<Message>(false),
        toggles::checkbox::<Message>(true),
        toggles::checkbox::<Message>(None),
    ];
    let radios = row![
        toggles::radio(1, Some(2)).on_change(Message::Pick),
        toggles::radio(2, Some(2)).on_change(Message::Pick),
        toggles::radio::<u8, Message>(1, Some(2)),
        toggles::radio::<u8, Message>(2, Some(2)),
    ];
    let switches = row![
        toggles::switch(false).on_toggle(Message::Switch),
        toggles::switch(true).on_toggle(Message::Switch),
        toggles::switch(false)
            .on_off_shapes(true)
            .on_toggle(Message::Switch),
        toggles::switch(true)
            .on_off_shapes(true)
            .on_toggle(Message::Switch),
        toggles::switch(true)
            .direction(ReadingDirection::RightToLeft)
            .on_toggle(Message::Switch),
        toggles::switch::<Message>(false),
        toggles::switch::<Message>(true),
    ];

    column![checkboxes, radios, switches].padding(8).into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (380.0, 120.0),
        oito2_iced::focus::navigation(catalog()),
    );
    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn toggles_light() {
    snapshot(&Theme::light(), "toggles_light");
}

#[test]
fn toggles_dark() {
    snapshot(&Theme::dark(), "toggles_dark");
}

#[test]
fn toggles_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "toggles_light_high_contrast");
}
