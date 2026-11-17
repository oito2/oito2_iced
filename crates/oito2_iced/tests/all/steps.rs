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

//! Steps: linear and free selection with the pointer and the keyboard, the
//! content of the current step in a row and in a column, the read-only
//! state, the localized descriptions and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, text};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::button;
use oito2_iced::widget::steps::{self, Axis, StepState};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Step(usize),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

fn three(current: usize) -> steps::Steps<'static, Message> {
    steps::steps(
        [
            steps::step("Account").content(text("Account form")),
            steps::step("Profile").content(text("Profile form")),
            steps::step("Done"),
        ],
        current,
    )
}

fn view(steps: steps::Steps<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            container(steps).width(480),
            button::filled("After").on_press(Message::After).id("after"),
        ]
        .spacing(8)
        .padding(16),
    )
    .into()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

#[test]
fn linear_steps_reach_only_done_and_current_ones() {
    let mut ui =
        Simulator::with_settings(support::settings(), view(three(1).on_select(Message::Step)));

    assert!(ui.find("Profile form").is_ok());
    assert!(ui.find("Account form").is_err());

    ui.click("Done").expect("done");
    ui.click("Profile").expect("profile");
    ui.click("Account").expect("account");

    assert_eq!(messages(ui), vec![Message::Step(1), Message::Step(0)]);
}

#[test]
fn free_steps_reach_any_step_also_vertically() {
    let mut ui = Simulator::with_settings(
        support::settings(),
        view(
            three(1)
                .linear(false)
                .axis(Axis::Vertical)
                .on_select(Message::Step),
        ),
    );

    assert!(ui.find("Profile form").is_ok());
    ui.click("Done").expect("done");

    assert_eq!(messages(ui), vec![Message::Step(2)]);
}

#[test]
fn tab_reaches_the_steps_that_can_be_reached_and_enter_selects() {
    let mut ui =
        Simulator::with_settings(support::settings(), view(three(1).on_select(Message::Step)));

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Space);
    // "Done" can't be reached, so Tab moves on to the button.
    ui.tap_key(Named::Tab);

    let after = ui.find(selector::id("after")).expect("after").bounds();
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(after)
    );
    assert_eq!(messages(ui), vec![Message::Step(0), Message::Step(1)]);
}

#[test]
fn read_only_steps_ignore_clicks_and_are_skipped() {
    let mut ui = Simulator::with_settings(support::settings(), view(three(1)));

    ui.click("Account").expect("account");
    ui.tap_key(Named::Tab);

    let after = ui.find(selector::id("after")).expect("after").bounds();
    assert_eq!(
        ui.find(selector::is_focused())
            .ok()
            .map(|target| target.bounds()),
        Some(after)
    );
    assert!(messages(ui).is_empty());
}

#[test]
fn all_steps_done_show_no_content() {
    let mut ui = Simulator::with_settings(support::settings(), view(three(3)));

    assert!(ui.find("Account form").is_err());
    assert!(ui.find("Profile form").is_err());
    assert!(ui.find("Done").is_ok());
}

#[test]
fn headers_share_the_width_up_to_the_largest() {
    let wide = steps::steps([steps::step::<Message>("A"), steps::step("B")], 0);
    let mut ui = Simulator::with_settings(
        support::settings(),
        Element::from(container(wide).width(600).padding(0)),
    );
    let first = ui.find("A").expect("a").bounds();
    let second = ui.find("B").expect("b").bounds();

    // The first header sits at the start and the last at the end, joined by
    // the line between them.
    assert!(first.x < 100.0);
    assert!(second.x > 500.0);
}

#[test]
fn steps_describe_their_position_and_state() {
    let en = Localizations::en();
    let pt = Localizations::pt_br();

    assert_eq!(
        steps::description("Account", 0, 3, steps::state_of(0, 1, false), &en),
        "Account, Step 1 of 3, Completed"
    );
    assert_eq!(
        steps::description("Profile", 1, 3, StepState::Current, &en),
        "Profile, Step 2 of 3, Current"
    );
    assert_eq!(
        steps::description("Done", 2, 3, StepState::Upcoming, &en),
        "Done, Step 3 of 3"
    );
    assert_eq!(
        steps::description("Pagamento", 2, 4, StepState::Error, &pt),
        "Pagamento, Passo 3 de 4, Requer atenção"
    );
}

fn catalog(axis: Axis) -> Element<'static, Message> {
    let steps = steps::steps(
        [
            steps::step("Account").description("Your e-mail"),
            steps::step("Profile").content(text("Profile form")),
            steps::step("Payment").error(true),
            steps::step("Done"),
        ],
        1,
    )
    .axis(axis)
    .on_select(Message::Step);

    container(steps).padding(16).into()
}

fn snapshot(theme: &Theme, axis: Axis, size: (f32, f32), name: &str) {
    let mut ui = Simulator::with_size(support::settings(), size, catalog(axis));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn steps_light() {
    snapshot(
        &Theme::light(),
        Axis::Horizontal,
        (640.0, 160.0),
        "steps_light",
    );
}

#[test]
fn steps_dark() {
    snapshot(
        &Theme::dark(),
        Axis::Horizontal,
        (640.0, 160.0),
        "steps_dark",
    );
}

#[test]
fn steps_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(
        &theme,
        Axis::Horizontal,
        (640.0, 160.0),
        "steps_light_high_contrast",
    );
}

#[test]
fn steps_vertical_light() {
    snapshot(
        &Theme::light(),
        Axis::Vertical,
        (320.0, 320.0),
        "steps_vertical_light",
    );
}

#[test]
fn steps_focused() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (640.0, 160.0),
        Element::from(focus::navigation(catalog(Axis::Horizontal))),
    );

    ui.tap_key(Named::Tab);
    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "steps_focused");
}
