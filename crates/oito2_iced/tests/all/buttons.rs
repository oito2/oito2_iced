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

//! Buttons: clicks, keyboard activation, focus traversal, sizes per density
//! and golden snapshots of the catalog. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, row};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Save,
    Cancel,
    Add,
}

fn form() -> Element<'static, Message> {
    oito2_iced::focus::navigation(
        column![
            button::filled("Save").on_press(Message::Save).id("save"),
            button::outlined("Disabled").id("disabled"),
            button::text("Cancel")
                .on_press(Message::Cancel)
                .id("cancel"),
            button::icon_button(lucide::PLUS)
                .on_press(Message::Add)
                .id("add"),
        ]
        .spacing(8),
    )
    .into()
}

fn focused(ui: &mut simulator::Simulator<'_, Message, Theme>) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn bounds(ui: &mut simulator::Simulator<'_, Message, Theme>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

#[test]
fn a_click_publishes_the_press_message() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), form());

    ui.click("Save").expect("save button");
    ui.click("Disabled").expect("disabled button");

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Save]);
}

#[test]
fn tab_skips_disabled_buttons_and_enter_activates() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), form());
    let save = bounds(&mut ui, "save");
    let cancel = bounds(&mut ui, "cancel");
    let add = bounds(&mut ui, "add");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(save));

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(cancel));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(add));
    ui.tap_key(Named::Space);

    let messages: Vec<Message> = ui.into_messages().collect();
    assert_eq!(messages, vec![Message::Cancel, Message::Add]);
}

#[test]
fn buttons_follow_the_control_height_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = simulator::Simulator::with_settings(support::settings(), form());
        let save = bounds(&mut ui, "save");
        let cancel = bounds(&mut ui, "cancel");
        let add = bounds(&mut ui, "add");

        assert_eq!(save.height, height, "{density:?}");
        assert!(save.width >= button::MIN_WIDTH, "{density:?}");
        assert!(cancel.width >= button::MIN_WIDTH, "{density:?}");
        assert_eq!((add.width, add.height), (height, height), "{density:?}");
    }

    density::set(Density::Pointer);
}

#[test]
fn a_floating_action_button_is_56_pixels_square() {
    let view: Element<'static, Message> = column![
        button::fab(lucide::PENCIL).on_press(Message::Add).id("fab"),
        button::fab(lucide::PENCIL)
            .label("Compose")
            .on_press(Message::Add)
            .id("extended"),
    ]
    .into();

    let mut ui = simulator::Simulator::with_settings(support::settings(), view);
    let fab = bounds(&mut ui, "fab");
    let extended = bounds(&mut ui, "extended");

    assert_eq!((fab.width, fab.height), (56.0, 56.0));
    assert_eq!(extended.height, 56.0);
    assert!(extended.width > 56.0);
}

fn catalog() -> Element<'static, Message> {
    let l10n = Localizations::en();

    column![
        row![
            button::filled("Filled").on_press(Message::Save),
            button::tonal("Tonal").on_press(Message::Save),
            button::elevated("Elevated").on_press(Message::Save),
            button::outlined("Outlined").on_press(Message::Save),
            button::text("Text").on_press(Message::Save),
            button::danger("Danger").on_press(Message::Save),
        ]
        .spacing(8),
        row![
            button::filled("Disabled"),
            button::outlined("Disabled"),
            button::filled("Icon")
                .icon(lucide::DOWNLOAD)
                .on_press(Message::Save),
            button::outlined("Icon")
                .icon(lucide::UPLOAD)
                .on_press(Message::Save),
        ]
        .spacing(8),
        row![
            button::icon_button(lucide::HEART).on_press(Message::Add),
            button::icon_button(lucide::HEART)
                .selected(true)
                .on_press(Message::Add),
            button::icon_button(lucide::HEART),
            button::back_button(&l10n, ReadingDirection::LeftToRight).on_press(Message::Cancel),
            button::close_button(&l10n).on_press(Message::Cancel),
            button::fab(lucide::PENCIL).on_press(Message::Add),
            button::fab(lucide::PENCIL)
                .label("Compose")
                .on_press(Message::Add),
        ]
        .spacing(8)
        .align_y(iced::Alignment::Center),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (640.0, 200.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn buttons_light() {
    snapshot(&Theme::light(), "buttons_light");
}

#[test]
fn buttons_dark() {
    snapshot(&Theme::dark(), "buttons_dark");
}

#[test]
fn buttons_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "buttons_light_high_contrast");
}

#[test]
fn enabled_buttons_show_the_hand_cursor() {
    let mut ui = simulator::Simulator::with_settings(support::settings(), form());
    let targets = ["save", "cancel", "add", "disabled"].map(|id| (id, bounds(&mut ui, id)));

    for (id, bounds) in targets {
        let interaction = support::mouse_interaction(form(), (1024.0, 768.0), bounds.center());

        if id == "disabled" {
            assert_ne!(interaction, iced::mouse::Interaction::Pointer, "{id}");
        } else {
            assert_eq!(interaction, iced::mouse::Interaction::Pointer, "{id}");
        }
    }
}
