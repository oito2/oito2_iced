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

//! Behaviour and golden tests of the tour widget: the card text and
//! buttons, the keys, the blocked and passed pointer input, the steps that
//! wait for a tap, the card placement, the focus, the spotlight move, the
//! scroll to the target and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation::{Operation, Outcome, black_box};
use iced::keyboard::key::Named;
use iced::time::Instant;
use iced::widget::{Space, column, container, scrollable, text};
use iced::{Event, Length, Point, Rectangle, Size, mouse, window};
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::selector::{self, Selector};
use iced_test::simulator;
use oito2_iced::l10n::Localizations;
use oito2_iced::overlay::Placement;
use oito2_iced::widget::button;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::tour::{self, Step, Tour};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    A,
    B,
    Page,
    Tour(tour::Event),
}

const SIZE: (f32, f32) = (640.0, 480.0);

/// Three steps: the first points at "A", the second at "B" (waiting for a
/// tap when `wait`), the last has no target.
fn three_steps(wait: bool) -> Tour {
    Tour::new(vec![
        Step::new("First").description("One").target("first"),
        Step::new("Second").target("second").wait_for_tap(wait),
        Step::new("Last"),
    ])
}

fn at(mut tour: Tour, index: usize) -> Tour {
    tour.go_to(index);
    tour
}

fn page() -> Element<'static, Message> {
    column![
        container(button::text("A").on_press(Message::A).id("a")).id("first"),
        Space::new().height(200),
        container(button::text("B").on_press(Message::B).id("b")).id("second"),
        button::text("Page").on_press(Message::Page).id("page"),
    ]
    .spacing(8)
    .padding(24)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn view(tour: &Tour) -> Element<'_, Message> {
    tour::host(page(), tour).on_event(Message::Tour).into()
}

fn open(tour: &Tour) -> simulator::Simulator<'_, Message, Theme> {
    simulator::Simulator::with_size(support::settings(), SIZE, view(tour))
}

fn messages(ui: simulator::Simulator<'_, Message, Theme>) -> Vec<Message> {
    ui.into_messages().collect()
}

fn bounds_of(ui: &mut simulator::Simulator<'_, Message, Theme>, id: &'static str) -> Rectangle {
    ui.find(selector::id(id)).expect("element").bounds()
}

/// A cursor move to a corner of the window, which lets the tour run the
/// work it keeps for the next event.
fn nudge(ui: &mut simulator::Simulator<'_, Message, Theme>) {
    ui.point_at(Point::new(1.0, 1.0));
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1.0, 1.0),
    })]);
}

#[test]
fn an_idle_tour_leaves_the_view_usable() {
    let tour = three_steps(false);
    let mut ui = open(&tour);

    assert!(ui.find("First").is_err());
    ui.click("B").expect("page button");
    assert_eq!(messages(ui), vec![Message::B]);
}

#[test]
fn the_card_shows_the_step_its_position_and_the_buttons() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);

    assert!(ui.find("First").is_ok());
    assert!(ui.find("One").is_ok());
    assert!(ui.find("Step 1 of 3").is_ok());
    assert!(ui.find("Back").is_err(), "no back button on the first step");
    ui.click("Next").expect("next button");
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Next)]);

    let tour = at(three_steps(false), 1);
    let mut ui = open(&tour);
    assert!(ui.find("Step 2 of 3").is_ok());
    ui.click("Back").expect("back button");
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Previous)]);

    let tour = at(three_steps(false), 2);
    let mut ui = open(&tour);
    assert!(ui.find("Next").is_err());
    ui.click("Done").expect("done button");
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Next)]);
}

#[test]
fn the_skip_button_skips() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);

    let title = ui.find("First").expect("title").bounds();
    // The skip button ends the title row, right after the title.
    ui.point_at(Point::new(title.x + title.width + 4.0, title.center_y()));
    let _ = ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Skip)]);
}

#[test]
fn the_strings_follow_the_localizations() {
    let tour = at(three_steps(false), 1);
    let strings = Localizations::pt_br();
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        tour::host(page(), &tour)
            .on_event(Message::Tour)
            .localizations(&strings),
    );

    assert!(ui.find("Passo 2 de 3").is_ok());
    assert!(ui.find("Voltar").is_ok());
    assert!(ui.find("Próximo").is_ok());
}

#[test]
fn escape_skips_and_the_arrows_move() {
    let tour = at(three_steps(false), 1);

    let mut ui = open(&tour);
    ui.tap_key(Named::Escape);
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Skip)]);

    let mut ui = open(&tour);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);
    assert_eq!(
        messages(ui),
        vec![
            Message::Tour(tour::Event::Next),
            Message::Tour(tour::Event::Previous),
        ]
    );

    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        tour::host(page(), &tour)
            .on_event(Message::Tour)
            .direction(ReadingDirection::RightToLeft),
    );
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);
    assert_eq!(
        messages(ui),
        vec![
            Message::Tour(tour::Event::Previous),
            Message::Tour(tour::Event::Next),
        ]
    );
}

#[test]
fn the_left_arrow_does_nothing_on_the_first_step() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);

    ui.tap_key(Named::ArrowLeft);
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn without_a_message_the_card_is_inert() {
    let tour = at(three_steps(false), 1);
    let mut ui =
        simulator::Simulator::with_size(support::settings(), SIZE, tour::host(page(), &tour));

    ui.click("Next").expect("next button");
    ui.tap_key(Named::Escape);
    ui.tap_key(Named::ArrowRight);
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn input_outside_the_spotlight_is_blocked_and_inside_passes() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);

    ui.click("B").expect("blocked button is found");
    ui.click("Page").expect("blocked button is found");
    ui.click("A").expect("target button");
    assert_eq!(messages(ui), vec![Message::A]);
}

#[test]
fn a_step_without_a_target_blocks_the_whole_view() {
    let tour = at(three_steps(false), 2);
    let mut ui = open(&tour);

    ui.click("A").expect("button is found");
    ui.click("B").expect("button is found");
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn a_waiting_step_moves_on_after_the_target_tap() {
    let tour = at(three_steps(true), 1);
    let mut ui = open(&tour);

    assert!(ui.find("Next").is_err(), "the step waits for the tap");
    ui.click("B").expect("target button");
    assert_eq!(
        messages(ui),
        vec![Message::B, Message::Tour(tour::Event::Next)]
    );
}

#[test]
fn a_waiting_step_without_its_target_offers_next() {
    let tour = at(
        Tour::new(vec![
            Step::new("Missing").target("nowhere").wait_for_tap(true),
        ]),
        0,
    );
    let mut ui = open(&tour);

    ui.click("Done").expect("done button");
    assert_eq!(messages(ui), vec![Message::Tour(tour::Event::Next)]);
}

/// The left and right edges of the card around its `title`, which fills
/// the title row between the start padding and the skip button.
fn card_edges(title: Rectangle) -> (f32, f32) {
    let skip = oito2_iced::density::control_height();

    (title.x - 16.0, title.x + title.width + skip + 8.0)
}

#[test]
fn the_card_opens_below_the_target_centered_on_it() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);
    let target = bounds_of(&mut ui, "first");
    let title = ui.find("First").expect("title").bounds();

    assert!(title.y > target.y + target.height, "the card is below");
    let (card_left, card_right) = card_edges(title);
    assert!(
        (card_right - card_left - 320.0).abs() < 1.0,
        "the card is 320 wide"
    );
    // Centered on the target, then kept a margin inside the window.
    let expected_left = (target.center_x() - 160.0).max(8.0);
    assert!(
        (card_left - expected_left).abs() < 1.0,
        "left at {card_left}"
    );
}

#[test]
fn the_card_follows_the_placement_and_the_reading_direction() {
    let tour = Tour::new(vec![
        Step::new("Side").target("second").placement(Placement::End),
    ]);
    let tour = at(tour, 0);
    let mut ui = open(&tour);
    let target = bounds_of(&mut ui, "second");
    let title = ui.find("Side").expect("title").bounds();
    assert!(title.x > target.x + target.width, "after the target in LTR");
    assert!(title.y < target.y + target.height && title.y + 200.0 > target.y);

    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        tour::host(page(), &tour)
            .on_event(Message::Tour)
            .direction(ReadingDirection::RightToLeft),
    );
    let title = ui.find("Side").expect("title").bounds();
    let done = ui.find("Done").expect("done").bounds();
    assert!(
        done.x < title.x,
        "the buttons lead and the title trails in RTL"
    );
}

#[test]
fn a_step_without_a_target_centers_the_card() {
    let tour = at(three_steps(false), 2);
    let mut ui = open(&tour);
    let title = ui.find("Last").expect("title").bounds();
    let done = ui.find("Done").expect("done").bounds();

    let (left, right) = card_edges(title);
    assert!(((left + right) / 2.0 - SIZE.0 / 2.0).abs() < 1.0);
    let middle = (title.y + done.y + done.height) / 2.0;
    assert!((middle - SIZE.1 / 2.0).abs() < 20.0, "centered at {middle}");
}

#[test]
fn the_card_takes_the_focus_on_each_step() {
    let tour = at(three_steps(false), 0);
    let mut ui = open(&tour);

    nudge(&mut ui);
    let focused = ui.find(selector::is_focused()).expect("a focused widget");
    let next = ui.find("Next").expect("next").bounds();
    assert!(
        focused.bounds().contains(next.center()),
        "next has the focus"
    );

    let tour = at(three_steps(true), 1);
    let mut ui = open(&tour);
    nudge(&mut ui);
    let focused = ui.find(selector::is_focused()).expect("a focused widget");
    let title = ui.find("Second").expect("title").bounds();
    assert!(
        focused.bounds().x > title.x + title.width - 1.0
            && (focused.bounds().center_y() - title.center_y()).abs() < 10.0,
        "skip has the focus while the step waits"
    );
}

#[test]
fn tab_cycles_inside_the_card() {
    let tour = at(three_steps(false), 1);
    let mut ui = open(&tour);

    for _ in 0..4 {
        ui.tap_key(Named::Tab);
        let focused = ui.find(selector::is_focused()).expect("a focused widget");
        let page = bounds_of(&mut ui, "page");
        assert!(!focused.bounds().intersects(&page));
    }
}

fn renderer() -> iced::Renderer {
    let backend = std::env::var("ICED_TEST_BACKEND").ok();

    iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
        oito2_iced::font::INTER,
        iced::Pixels(16.0),
        backend.as_deref(),
    ))
    .expect("headless renderer")
}

type Ui<'a> = UserInterface<'a, Message, Theme, iced::Renderer>;

fn send(
    ui: &mut Ui<'_>,
    renderer: &mut iced::Renderer,
    events: &[Event],
    cursor: mouse::Cursor,
) -> Vec<Message> {
    let mut messages = Vec::new();
    let _ = ui.update(
        events,
        cursor,
        renderer,
        &mut clipboard::Null,
        &mut messages,
    );
    messages
}

fn is_focused(ui: &mut Ui<'_>, renderer: &iced::Renderer, id: &'static str) -> bool {
    let mut operation = iced::advanced::widget::operation::focusable::is_focused(id.into());
    ui.operate(renderer, &mut black_box(&mut operation));

    matches!(operation.finish(), Outcome::Some(true))
}

fn center_of(ui: &mut Ui<'_>, renderer: &iced::Renderer, id: &'static str) -> Option<Point> {
    let mut find = selector::id(id).find();
    ui.operate(renderer, &mut black_box(&mut find));

    match find.finish() {
        Outcome::Some(Some(target)) => Some(target.bounds().center()),
        _ => None,
    }
}

fn click_at(point: Point) -> Vec<Event> {
    vec![
        Event::Mouse(mouse::Event::CursorMoved { position: point }),
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]
}

#[test]
fn the_focus_returns_after_the_tour() {
    let _fonts = simulator::Simulator::with_size(support::settings(), SIZE, page());
    let mut renderer = renderer();
    let size = Size::new(SIZE.0, SIZE.1);
    let idle = Tour::new(vec![Step::new("Welcome")]);
    let running = at(idle.clone(), 0);

    let mut ui = UserInterface::build(view(&idle), size, Cache::default(), &mut renderer);
    let mut focus = iced::advanced::widget::operation::focusable::focus::<()>("page".into());
    ui.operate(&renderer, &mut black_box(&mut focus));
    assert!(is_focused(&mut ui, &renderer, "page"));

    let mut ui = UserInterface::build(view(&running), size, ui.into_cache(), &mut renderer);
    let _ = send(
        &mut ui,
        &mut renderer,
        &[Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )],
        mouse::Cursor::Unavailable,
    );
    assert!(
        !is_focused(&mut ui, &renderer, "page"),
        "the card took the focus"
    );

    let mut ui = UserInterface::build(view(&idle), size, ui.into_cache(), &mut renderer);
    let _ = send(
        &mut ui,
        &mut renderer,
        &[Event::Window(
            window::Event::RedrawRequested(Instant::now()),
        )],
        mouse::Cursor::Unavailable,
    );
    assert!(
        is_focused(&mut ui, &renderer, "page"),
        "the focus came back"
    );
}

/// Builds the tour on its first step, then moves it to the second, as an
/// application does after a [`tour::Event::Next`].
fn moved<'a>(
    first: &'a Tour,
    second: &'a Tour,
    animated: bool,
    renderer: &mut iced::Renderer,
) -> Ui<'a> {
    let size = Size::new(SIZE.0, SIZE.1);
    let view = |tour: &'a Tour| -> Element<'a, Message> {
        tour::host(page(), tour)
            .on_event(Message::Tour)
            .animated(animated)
            .into()
    };
    let before = UserInterface::build(view(first), size, Cache::default(), renderer);

    UserInterface::build(view(second), size, before.into_cache(), renderer)
}

#[test]
fn the_spotlight_moves_to_the_next_target() {
    let _fonts = simulator::Simulator::with_size(support::settings(), SIZE, page());
    let mut renderer = renderer();
    let first = at(three_steps(false), 0);
    let second = at(three_steps(false), 1);
    let start = Instant::now();

    let mut ui = moved(&first, &second, true, &mut renderer);
    let b = center_of(&mut ui, &renderer, "b").expect("b");
    let frame = |at| [Event::Window(window::Event::RedrawRequested(at))];

    let _ = send(
        &mut ui,
        &mut renderer,
        &frame(start),
        mouse::Cursor::Unavailable,
    );
    let blocked = send(
        &mut ui,
        &mut renderer,
        &click_at(b),
        mouse::Cursor::Available(b),
    );
    assert_eq!(
        blocked,
        Vec::<Message>::new(),
        "the spotlight is still on A"
    );

    let _ = send(
        &mut ui,
        &mut renderer,
        &frame(start + tour::MOVE_DURATION),
        mouse::Cursor::Unavailable,
    );
    let passed = send(
        &mut ui,
        &mut renderer,
        &click_at(b),
        mouse::Cursor::Available(b),
    );
    assert_eq!(passed, vec![Message::B], "the spotlight reached B");

    let mut ui = moved(&first, &second, false, &mut renderer);
    let _ = send(
        &mut ui,
        &mut renderer,
        &frame(start),
        mouse::Cursor::Unavailable,
    );
    let passed = send(
        &mut ui,
        &mut renderer,
        &click_at(b),
        mouse::Cursor::Available(b),
    );
    assert_eq!(
        passed,
        vec![Message::B],
        "without animation the move is instant"
    );
}

#[test]
fn the_target_scrolls_into_view() {
    let tour = at(
        Tour::new(vec![Step::new("Deep").target("deep").wait_for_tap(true)]),
        0,
    );
    let content = scrollable(
        column![
            text("Top"),
            Space::new().height(1200),
            container(button::text("Deep").on_press(Message::B).id("deep-button")).id("deep"),
            Space::new().height(1200),
        ]
        .width(Length::Fill),
    )
    .height(300);
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        tour::host(column![content].padding(20), &tour).on_event(Message::Tour),
    );

    nudge(&mut ui);
    // The target is in the middle of the 300 pixel viewport that starts
    // at 20.
    ui.point_at(Point::new(60.0, 170.0));
    let _ = ui.simulate(simulator::click());
    assert_eq!(
        messages(ui),
        vec![Message::B, Message::Tour(tour::Event::Next)]
    );
}

fn snapshot(tour: &Tour, theme: &Theme, direction: ReadingDirection, name: &str) {
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        SIZE,
        tour::host(page(), tour)
            .on_event(Message::Tour)
            .direction(direction),
    );
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn golden_light() {
    snapshot(
        &at(three_steps(false), 0),
        &Theme::light(),
        ReadingDirection::LeftToRight,
        "tour_light",
    );
}

#[test]
fn golden_dark() {
    snapshot(
        &at(three_steps(false), 1),
        &Theme::dark(),
        ReadingDirection::LeftToRight,
        "tour_dark",
    );
}

#[test]
fn golden_centered() {
    snapshot(
        &at(three_steps(false), 2),
        &Theme::light(),
        ReadingDirection::LeftToRight,
        "tour_centered_light",
    );
}

#[test]
fn golden_right_to_left_waiting() {
    let tour = Tour::new(vec![
        Step::new("First").target("first"),
        Step::new("Settings")
            .description("Change the theme and the language here.")
            .target("second")
            .placement(Placement::End)
            .wait_for_tap(true),
    ]);

    snapshot(
        &at(tour, 1),
        &Theme::dark(),
        ReadingDirection::RightToLeft,
        "tour_rtl_dark",
    );
}
