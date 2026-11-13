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

//! Splitters and paned views: initial layout, dragging with the pointer,
//! keyboard resizing, collapsing, the resize cursor and golden snapshots.
//! Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::mouse::{self, Interaction};
use iced::widget::{column, container, text};
use iced::{Event, Length, Point, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::paned_view::{self, Side};
use oito2_iced::widget::splitter::{self, Axis};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Split(splitter::State),
    Resized(f32),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// A filled box with `label` and the widget id `id`.
fn block(id: &'static str, label: &'static str) -> Element<'static, Message> {
    container(text(label))
        .id(id)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn bounds(ui: &mut Simulator<'_>, id: &'static str) -> Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

/// Presses at `from`, moves by `delta` and releases.
fn drag(ui: &mut Simulator<'_>, from: Point, delta: iced::Vector) {
    ui.point_at(from);
    let to = from + delta;
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::CursorMoved {
            position: from + delta * 0.5,
        }),
        Event::Mouse(mouse::Event::CursorMoved { position: to }),
    ]);
    ui.point_at(to);
    let _ = ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

/// The last splitter state among `messages`.
fn last_state(messages: &[Message]) -> splitter::State {
    messages
        .iter()
        .rev()
        .find_map(|message| match message {
            Message::Split(state) => Some(state.clone()),
            Message::Resized(_) => None,
        })
        .expect("a splitter state was published")
}

/// The last pane size among `messages`.
fn last_size(messages: &[Message]) -> f32 {
    messages
        .iter()
        .rev()
        .find_map(|message| match message {
            Message::Resized(size) => Some(*size),
            Message::Split(_) => None,
        })
        .expect("a pane size was published")
}

/// Three panes: a quarter of at least 100 pixels that collapses, a nested
/// vertical splitter, and a pane of at least 100 pixels.
fn three(state: &splitter::State) -> Element<'_, Message> {
    focus::navigation(
        splitter::splitter(
            state,
            vec![
                splitter::pane(block("a", "A"))
                    .fraction(0.25)
                    .min_size(100.0)
                    .collapsible(true),
                splitter::pane(block("b", "B")),
                splitter::pane(block("c", "C")).min_size(100.0),
            ],
        )
        .on_change(Message::Split),
    )
    .into()
}

fn three_ui(state: &splitter::State) -> Simulator<'_> {
    Simulator::with_size(support::settings(), (602.0, 300.0), three(state))
}

#[test]
fn panes_start_from_their_fractions() {
    let state = splitter::State::new();
    let mut ui = three_ui(&state);

    assert_eq!(bounds(&mut ui, "a").width, 150.0);
    assert!((bounds(&mut ui, "c").width - 225.0).abs() < 0.01);
    assert_eq!(bounds(&mut ui, "b").x, 151.0);
}

#[test]
fn dragging_a_divider_resizes_both_panes_within_their_limits() {
    let state = splitter::State::new();
    let mut ui = three_ui(&state);
    drag(
        &mut ui,
        Point::new(150.5, 150.0),
        iced::Vector::new(30.0, 0.0),
    );
    let state = last_state(&messages(ui));

    assert_eq!(state.sizes()[0], 180.0);
    assert_eq!(state.sizes()[1], 195.0);

    let mut ui = three_ui(&state);
    assert_eq!(bounds(&mut ui, "a").width, 180.0);

    drag(
        &mut ui,
        Point::new(180.5, 150.0),
        iced::Vector::new(-200.0, 0.0),
    );
    let state = last_state(&messages(ui));
    assert_eq!(state.sizes()[0], 100.0);
}

#[test]
fn the_drag_continues_outside_the_splitter_until_release() {
    let state = splitter::State::new();
    let mut ui = three_ui(&state);
    ui.point_at(Point::new(150.5, 150.0));
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(200.0, 900.0),
        }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(260.0, 150.0),
        }),
    ]);
    let messages = messages(ui);

    assert_eq!(messages.len(), 1);
    assert!((last_state(&messages).sizes()[0] - 199.5).abs() < 0.01);
}

#[test]
fn keys_resize_and_enter_collapses_and_restores() {
    let state = splitter::State::new();
    let mut ui = three_ui(&state);

    ui.tap_key(Named::Tab);
    let handle = focused(&mut ui).expect("the first handle has the focus");
    assert_eq!(handle.center_x(), 150.5);

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Enter);
    ui.tap_key(Named::Home);
    ui.tap_key(Named::End);
    let states: Vec<splitter::State> = messages(ui)
        .into_iter()
        .filter_map(|message| match message {
            Message::Split(state) => Some(state),
            Message::Resized(_) => None,
        })
        .collect();

    assert_eq!(states.len(), 5);
    assert_eq!(states[0].sizes(), &[166.0, 209.0, 225.0]);
    assert!(states[1].is_collapsed(0));
    assert_eq!(states[1].sizes()[0], 0.0);
    assert!(!states[2].is_collapsed(0));
    assert_eq!(states[2].sizes(), &[166.0, 209.0, 225.0]);
    assert_eq!(states[3].sizes()[0], 100.0);
    assert_eq!(states[4].sizes(), &[327.0, 48.0, 225.0]);

    let mut ui = three_ui(&states[1]);
    assert!(ui.find("A").is_err(), "a collapsed pane hides its content");
}

#[test]
fn a_double_click_collapses_the_pane() {
    let state = splitter::State::new();
    let mut ui = three_ui(&state);
    ui.point_at(Point::new(150.5, 150.0));
    let _ = ui.simulate(simulator::click());
    let _ = ui.simulate(simulator::click());

    assert!(last_state(&messages(ui)).is_collapsed(0));
}

#[test]
fn handles_show_the_resize_cursor() {
    let state = splitter::State::new();
    let vertical = || -> Element<'static, Message> {
        splitter::splitter(
            &splitter::State::new(),
            vec![splitter::pane(text("1")), splitter::pane(text("2"))],
        )
        .axis(Axis::Vertical)
        .on_change(Message::Split)
        .into()
    };
    let fixed = || -> Element<'static, Message> {
        splitter::splitter(
            &splitter::State::new(),
            vec![splitter::pane(text("1")), splitter::pane(text("2"))],
        )
        .into()
    };
    let _ = three_ui(&state);

    assert_eq!(
        support::mouse_interaction(three(&state), (602.0, 300.0), Point::new(152.0, 10.0)),
        Interaction::ResizingHorizontally
    );
    assert_ne!(
        support::mouse_interaction(three(&state), (602.0, 300.0), Point::new(100.0, 10.0)),
        Interaction::ResizingHorizontally
    );
    assert_eq!(
        support::mouse_interaction(vertical(), (300.0, 301.0), Point::new(20.0, 150.0)),
        Interaction::ResizingVertically
    );
    assert_ne!(
        support::mouse_interaction(fixed(), (301.0, 300.0), Point::new(150.0, 20.0)),
        Interaction::ResizingHorizontally
    );
}

#[test]
fn a_splitter_without_on_change_has_no_handle() {
    let state = splitter::State::new();
    let view = focus::navigation(splitter::splitter(
        &state,
        vec![splitter::pane(text("1")), splitter::pane(text("2"))],
    ));
    let mut ui = Simulator::with_size(support::settings(), (301.0, 100.0), view);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), None);
}

#[test]
fn right_to_left_splitters_start_on_the_right() {
    let state = splitter::State::new();
    let view: Element<'_, Message> = splitter::splitter(
        &state,
        vec![
            splitter::pane(block("first", "1")).fraction(0.25),
            splitter::pane(block("second", "2")),
        ],
    )
    .direction(ReadingDirection::RightToLeft)
    .on_change(Message::Split)
    .into();
    let mut ui = Simulator::with_size(support::settings(), (401.0, 100.0), view);

    assert_eq!(bounds(&mut ui, "first").x, 301.0);
    assert_eq!(bounds(&mut ui, "second").x, 0.0);

    drag(
        &mut ui,
        Point::new(300.5, 50.0),
        iced::Vector::new(-20.0, 0.0),
    );
    assert_eq!(last_state(&messages(ui)).sizes()[0], 120.0);
}

#[test]
fn panes_larger_than_the_space_are_clipped() {
    let state = splitter::State::new();
    let view: Element<'_, Message> = splitter::splitter(
        &state,
        vec![
            splitter::pane(block("first", "1")).min_size(250.0),
            splitter::pane(block("second", "2")).min_size(250.0),
        ],
    )
    .on_change(Message::Split)
    .into();
    let mut ui = Simulator::with_size(support::settings(), (300.0, 100.0), view);

    assert_eq!(bounds(&mut ui, "first").width, 250.0);
    assert_eq!(bounds(&mut ui, "second").x, 251.0);
}

/// A paned view of a 1000 by 600 window with the pane of `size` on `side`.
fn paned(size: f32, side: Side, direction: ReadingDirection) -> Element<'static, Message> {
    focus::navigation(
        paned_view::paned_view(block("pane", "Pane"), block("page", "Page"), size)
            .side(side)
            .min_pane_size(200.0)
            .min_page_size(400.0)
            .direction(direction)
            .on_resize(Message::Resized),
    )
    .into()
}

fn paned_ui(size: f32, side: Side, direction: ReadingDirection) -> Simulator<'static> {
    Simulator::with_size(
        support::settings(),
        (1000.0, 600.0),
        paned(size, side, direction),
    )
}

#[test]
fn a_paned_view_starts_at_its_size_on_the_start_side() {
    let mut ui = paned_ui(300.0, Side::Start, ReadingDirection::LeftToRight);
    let pane = bounds(&mut ui, "pane");

    assert_eq!((pane.x, pane.width), (0.0, 300.0));
    assert_eq!(bounds(&mut ui, "page").x, 301.0);
}

#[test]
fn dragging_the_paned_view_handle_stays_within_the_limits() {
    let ltr = ReadingDirection::LeftToRight;
    let mut ui = paned_ui(300.0, Side::Start, ltr);
    drag(
        &mut ui,
        Point::new(300.5, 300.0),
        iced::Vector::new(100.0, 0.0),
    );
    let size = last_size(&messages(ui));
    assert_eq!(size, 400.0);

    let mut ui = paned_ui(size, Side::Start, ltr);
    drag(
        &mut ui,
        Point::new(400.5, 300.0),
        iced::Vector::new(-1000.0, 0.0),
    );
    let size = last_size(&messages(ui));
    assert_eq!(size, 200.0);

    let mut ui = paned_ui(size, Side::Start, ltr);
    drag(
        &mut ui,
        Point::new(200.5, 300.0),
        iced::Vector::new(2000.0, 0.0),
    );
    assert_eq!(last_size(&messages(ui)), 600.0);
}

#[test]
fn arrow_keys_resize_the_focused_paned_view_handle() {
    let mut ui = paned_ui(300.0, Side::Start, ReadingDirection::LeftToRight);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::ArrowLeft);

    assert_eq!(
        messages(ui),
        vec![
            Message::Resized(316.0),
            Message::Resized(300.0),
            Message::Resized(284.0)
        ]
    );
}

#[test]
fn a_pane_larger_than_the_limit_moves_on_the_first_key() {
    let mut ui = paned_ui(900.0, Side::Start, ReadingDirection::LeftToRight);
    assert_eq!(bounds(&mut ui, "pane").width, 600.0);

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowLeft);
    assert_eq!(messages(ui), vec![Message::Resized(584.0)]);
}

#[test]
fn the_end_side_places_the_pane_after_the_page() {
    let mut ui = paned_ui(300.0, Side::End, ReadingDirection::LeftToRight);
    assert_eq!(bounds(&mut ui, "pane").x, 700.0);

    drag(
        &mut ui,
        Point::new(699.5, 300.0),
        iced::Vector::new(-50.0, 0.0),
    );
    assert_eq!(last_size(&messages(ui)), 350.0);
}

#[test]
fn the_start_side_follows_right_to_left_text() {
    let mut ui = paned_ui(300.0, Side::Start, ReadingDirection::RightToLeft);
    let pane = bounds(&mut ui, "pane");
    assert_eq!(pane.x + pane.width, 1000.0);

    drag(
        &mut ui,
        Point::new(699.5, 300.0),
        iced::Vector::new(-50.0, 0.0),
    );
    assert_eq!(last_size(&messages(ui)), 350.0);
}

#[test]
fn top_and_bottom_stack_vertically() {
    let ltr = ReadingDirection::LeftToRight;
    let square = |size, side| {
        Simulator::with_size(
            support::settings(),
            (1000.0, 1000.0),
            paned(size, side, ltr),
        )
    };
    let mut ui = square(300.0, Side::Top);
    assert_eq!(bounds(&mut ui, "pane").height, 300.0);

    drag(
        &mut ui,
        Point::new(500.0, 300.5),
        iced::Vector::new(0.0, 40.0),
    );
    assert_eq!(last_size(&messages(ui)), 340.0);

    let mut ui = square(250.0, Side::Bottom);
    assert_eq!(bounds(&mut ui, "pane").y, 750.0);
}

#[test]
fn a_fixed_pane_has_no_handle() {
    let view = focus::navigation(
        paned_view::paned_view(block("pane", "Pane"), block("page", "Page"), 300.0)
            .resizable(false)
            .on_resize(Message::Resized),
    );
    let mut ui = Simulator::with_size(support::settings(), (1000.0, 600.0), view);
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), None);

    drag(
        &mut ui,
        Point::new(300.5, 300.0),
        iced::Vector::new(100.0, 0.0),
    );
    assert!(messages(ui).is_empty());
}

#[test]
fn a_pane_wider_than_the_space_is_clipped() {
    let mut ui = Simulator::with_size(
        support::settings(),
        (150.0, 100.0),
        paned(300.0, Side::Start, ReadingDirection::LeftToRight),
    );

    assert_eq!(bounds(&mut ui, "pane").width, 200.0);
}

/// A tinted box with `label`.
fn tinted(label: &'static str) -> Element<'static, Message> {
    container(text(label))
        .padding(8)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.roles().surface_container_low.into()),
            text_color: Some(theme.roles().on_surface),
            ..container::Style::default()
        })
        .into()
}

fn catalog<'a>(outer: &'a splitter::State, inner: &'a splitter::State) -> Element<'a, Message> {
    focus::navigation(
        column![
            splitter::splitter(
                outer,
                vec![
                    splitter::pane(tinted("Files")).fraction(0.3),
                    splitter::pane(
                        splitter::splitter(
                            inner,
                            vec![
                                splitter::pane(tinted("Editor")),
                                splitter::pane(tinted("Terminal")),
                            ],
                        )
                        .axis(Axis::Vertical)
                        .on_change(Message::Split),
                    ),
                    splitter::pane(tinted("Outline")),
                ],
            )
            .on_change(Message::Split),
            container(
                paned_view::paned_view(tinted("Pane"), tinted("Page"), 200.0)
                    .side(Side::End)
                    .min_page_size(200.0)
                    .on_resize(Message::Resized),
            )
            .height(120),
        ]
        .spacing(12)
        .padding(12),
    )
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let (outer, inner) = (splitter::State::new(), splitter::State::new());
    let mut ui = Simulator::with_size(support::settings(), (640.0, 360.0), catalog(&outer, &inner));
    ui.tap_key(Named::Tab);
    ui.point_at(Point::new(427.5, 300.0));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn splitter_light() {
    snapshot(&Theme::light(), "splitter_light");
}

#[test]
fn splitter_dark() {
    snapshot(&Theme::dark(), "splitter_dark");
}

#[test]
fn splitter_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "splitter_light_high_contrast");
}
