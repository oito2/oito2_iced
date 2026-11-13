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

//! Toasts: the stack shows at most the visible toasts at the placement
//! edge, the action and dismiss buttons publish their events by click and
//! keyboard, Escape closes a focused toast, the time runs out by redraw
//! instants and stops under the pointer, and golden snapshots of a stack.
//! Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use std::time::Duration;

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{button, container, text};
use iced::{Event, Length, Point, window};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::info::InfoType;
use oito2_iced::widget::toast::{self, Placement, Toast, ToastId, Toasts};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Toast(toast::Event),
    Undo,
    Content,
}

const SIZE: (f32, f32) = (640.0, 400.0);

fn view(toasts: &Toasts<Message>) -> Element<'static, Message> {
    let content = container(button(text("Content")).on_press(Message::Content))
        .width(Length::Fill)
        .height(Length::Fill);

    oito2_iced::focus::navigation(toast::toaster(
        content,
        toasts,
        &Localizations::en(),
        Message::Toast,
    ))
    .into()
}

fn open(toasts: &Toasts<Message>) -> simulator::Simulator<'static, Message, Theme> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(toasts))
}

fn redraw(ui: &mut simulator::Simulator<'_, Message, Theme>, at: Instant) {
    ui.simulate([Event::Window(window::Event::RedrawRequested(at))]);
}

fn messages(ui: simulator::Simulator<'_, Message, Theme>) -> Vec<Message> {
    ui.into_messages().collect()
}

fn one(toast: Toast<Message>) -> (Toasts<Message>, ToastId) {
    let mut toasts = Toasts::new();
    let id = toasts.push(toast);
    (toasts, id)
}

#[test]
fn the_toasts_stack_from_the_placement_edge() {
    let mut toasts = Toasts::new();
    toasts.push(Toast::new("First"));
    toasts.push(Toast::new("Second"));

    let mut ui = open(&toasts);
    let first = ui.find("First").expect("first toast").bounds();
    let second = ui.find("Second").expect("second toast").bounds();
    assert!(first.y > second.y, "at the bottom the oldest is lowest");
    assert!(first.y + first.height <= SIZE.1 - 12.0);

    let mut top = Toasts::new().with_placement(Placement::TopEnd);
    top.push(Toast::new("First"));
    top.push(Toast::new("Second"));

    let mut ui = open(&top);
    let first = ui.find("First").expect("first toast").bounds();
    let second = ui.find("Second").expect("second toast").bounds();
    assert!(first.y < second.y, "at the top the oldest is highest");
    assert!(
        first.x > SIZE.0 / 2.0,
        "the end placement sits at the right"
    );
}

#[test]
fn only_the_visible_toasts_show() {
    let mut toasts = Toasts::new().with_max_visible(2);
    for title in ["One", "Two", "Three"] {
        toasts.push(Toast::new(title));
    }

    let mut ui = open(&toasts);
    assert!(ui.find("One").is_ok());
    assert!(ui.find("Two").is_ok());
    assert!(ui.find("Three").is_err(), "the third toast waits for room");
}

#[test]
fn the_action_publishes_its_event() {
    let (mut toasts, id) = one(Toast::new("Deleted").with_action("Undo", Message::Undo));

    let mut ui = open(&toasts);
    ui.click("Undo").expect("action button");

    let messages = messages(ui);
    assert_eq!(messages, vec![Message::Toast(toast::Event::Action(id))]);
    assert_eq!(toasts.update(toast::Event::Action(id)), Some(Message::Undo));
    assert!(!toasts.is_active(id));
}

#[test]
fn tab_reaches_the_toast_and_enter_dismisses() {
    let (toasts, id) = one(Toast::new("Saved"));

    let mut ui = open(&toasts);
    // The native content button takes no focus: Tab reaches the toast.
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);

    assert_eq!(
        messages(ui),
        vec![Message::Toast(toast::Event::Dismiss(id))]
    );
}

#[test]
fn escape_dismisses_the_focused_toast() {
    let (toasts, id) = one(Toast::new("Saved"));

    let mut ui = open(&toasts);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Escape);

    assert_eq!(
        messages(ui),
        vec![Message::Toast(toast::Event::Dismiss(id))]
    );
}

#[test]
fn escape_does_nothing_without_focus_or_when_not_dismissible() {
    let (toasts, _) = one(Toast::new("Saved"));
    let mut ui = open(&toasts);
    ui.tap_key(Named::Escape);
    assert!(messages(ui).is_empty());

    let (toasts, _) = one(Toast::new("Syncing").with_dismissible(false));
    let mut ui = open(&toasts);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Escape);
    assert!(messages(ui).is_empty());
}

#[test]
fn the_time_runs_out_by_the_redraw_instants() {
    let (toasts, id) = one(Toast::new("Saved"));
    let start = Instant::now();

    let mut ui = open(&toasts);
    redraw(&mut ui, start);
    redraw(&mut ui, start + Duration::from_millis(3900));
    assert!(ui.into_messages().next().is_none());

    let mut ui = open(&toasts);
    redraw(&mut ui, start);
    redraw(&mut ui, start + toast::DEFAULT_DURATION);
    redraw(&mut ui, start + toast::DEFAULT_DURATION * 2);

    assert_eq!(
        messages(ui),
        vec![Message::Toast(toast::Event::Dismiss(id))],
        "the dismissal is published once"
    );
}

#[test]
fn loading_and_endless_toasts_stay() {
    let mut toasts = Toasts::new();
    toasts.push(Toast::new("Uploading").with_loading(true));
    toasts.push(Toast::new("Pinned").with_duration(None));
    let start = Instant::now();

    let mut ui = open(&toasts);
    redraw(&mut ui, start);
    redraw(&mut ui, start + Duration::from_secs(60));

    assert!(messages(ui).is_empty());
}

#[test]
fn the_pointer_over_the_stack_stops_the_time() {
    let (toasts, id) = one(Toast::new("Saved"));
    let start = Instant::now();

    let mut ui = open(&toasts);
    let bounds = ui.find("Saved").expect("toast").bounds();
    redraw(&mut ui, start);
    ui.point_at(bounds.center());
    redraw(&mut ui, start + Duration::from_secs(1));
    redraw(&mut ui, start + Duration::from_secs(30));

    ui.point_at(Point::new(5.0, 5.0));
    redraw(&mut ui, start + Duration::from_secs(31));
    redraw(&mut ui, start + Duration::from_millis(33_900));
    assert!(
        ui.into_messages().next().is_none(),
        "one second ran before the pause, three after it"
    );

    let mut ui = open(&toasts);
    redraw(&mut ui, start);
    ui.point_at(bounds.center());
    redraw(&mut ui, start + Duration::from_secs(1));
    ui.point_at(Point::new(5.0, 5.0));
    redraw(&mut ui, start + Duration::from_secs(30));
    redraw(&mut ui, start + Duration::from_secs(33));

    assert_eq!(
        messages(ui),
        vec![Message::Toast(toast::Event::Dismiss(id))]
    );
}

#[test]
fn a_toast_closing_at_first_sight_is_removed() {
    let (mut toasts, id) = one(Toast::new("Saved"));
    toasts.dismiss(id);

    let mut ui = open(&toasts);
    redraw(&mut ui, Instant::now());

    assert_eq!(
        messages(ui),
        vec![Message::Toast(toast::Event::Removed(id))]
    );
}

#[test]
fn clicks_on_a_toast_do_not_reach_the_content_below() {
    let (toasts, _) = one(Toast::new("Saved").with_duration(None));

    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        oito2_iced::focus::navigation(toast::toaster(
            button(text("Below").width(Length::Fill).height(Length::Fill))
                .on_press(Message::Content),
            &toasts,
            &Localizations::en(),
            Message::Toast,
        )),
    );

    let bounds = ui.find("Saved").expect("toast").bounds();
    ui.point_at(bounds.center());
    ui.simulate(simulator::click());
    assert!(ui.into_messages().next().is_none());
}

fn catalog() -> Toasts<Message> {
    let mut toasts = Toasts::new().with_max_visible(3);
    toasts.push(
        Toast::new("File deleted")
            .with_description("report.pdf was moved to the trash.")
            .with_kind(InfoType::Information)
            .with_action("Undo", Message::Undo),
    );
    toasts.push(Toast::new("Changes saved").with_kind(InfoType::Success));
    toasts.push(
        Toast::new("Connection lost")
            .with_description("Trying again in 30 seconds.")
            .with_kind(InfoType::Danger)
            .with_dismissible(false),
    );
    toasts
}

fn snapshot(theme: &Theme, name: &str) {
    let toasts = catalog();
    let content = container(text("Content"))
        .padding(12)
        .width(Length::Fill)
        .height(Length::Fill)
        .class(oito2_iced::theme::container::Class::Background);
    let view: Element<'static, Message> =
        toast::toaster(content, &toasts, &Localizations::en(), Message::Toast).into();

    let mut ui = simulator::Simulator::with_size(support::settings(), SIZE, view);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn toast_light() {
    snapshot(&Theme::light(), "toast_light");
}

#[test]
fn toast_dark() {
    snapshot(&Theme::dark(), "toast_dark");
}

#[test]
fn toast_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "toast_light_high_contrast");
}

#[test]
fn the_dismiss_button_follows_the_density() {
    let (toasts, _) = one(Toast::new("Saved"));

    for (density, side) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);
        let mut ui = open(&toasts);
        ui.tap_key(Named::Tab);
        let button = ui
            .find(selector::is_focused())
            .expect("dismiss button")
            .bounds();

        assert_eq!((button.width, button.height), (side, side), "{density:?}");
    }

    density::set(Density::Pointer);
}
