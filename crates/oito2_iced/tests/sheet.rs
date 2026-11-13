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

//! Sheets: placement on each edge, dismissal by Escape, the scrim, the close
//! button and drags, the non-modal sheet beside a usable page, the resting
//! heights of a bottom sheet, and golden snapshots of an end and a bottom
//! sheet. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use std::time::{Duration, Instant};

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, text};
use iced::{Event, Length, Point, mouse, window};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::{button, sheet};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Behind,
    Save,
    Close,
    Dismiss,
    Snap(usize),
}

const SIZE: (f32, f32) = (800.0, 600.0);

type Simulator = simulator::Simulator<'static, Message, Theme>;

fn page() -> Element<'static, Message> {
    column![
        text("Page"),
        button::filled("Behind")
            .on_press(Message::Behind)
            .id("behind"),
    ]
    .spacing(12)
    .padding(12)
    .width(Length::Fill)
    .height(Length::Fill)
    .into()
}

fn sheet() -> sheet::Sheet<'static, Message> {
    sheet::sheet(
        sheet::content(button::text("Save").on_press(Message::Save).id("save"))
            .title("Sheet")
            .on_close(Message::Close),
    )
    .on_dismiss(Message::Dismiss)
}

fn bottom(current: usize) -> sheet::Sheet<'static, Message> {
    sheet()
        .side(sheet::Side::Bottom)
        .snap_points([0.3, 0.8], current, Message::Snap)
}

fn view(sheet: Option<sheet::Sheet<'static, Message>>) -> Element<'static, Message> {
    focus::navigation(sheet::host(page(), sheet)).into()
}

fn simulator_sized(size: (f32, f32), sheet: Option<sheet::Sheet<'static, Message>>) -> Simulator {
    simulator::Simulator::with_size(support::settings(), size, view(sheet))
}

fn open(sheet: Option<sheet::Sheet<'static, Message>>) -> Simulator {
    simulator_sized(SIZE, sheet)
}

fn messages(ui: Simulator) -> Vec<Message> {
    ui.into_messages().collect()
}

fn title(ui: &mut Simulator) -> iced::Rectangle {
    ui.find("Sheet").expect("sheet title").bounds()
}

/// Presses at `from`, moves by `by` in steps of 10 pixels and releases.
fn drag(ui: &mut Simulator, from: Point, by: iced::Vector) {
    ui.point_at(from);
    let steps = (by.x.abs().max(by.y.abs()) / 10.0).ceil().max(1.0) as usize;
    let mut events = vec![Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))];

    for step in 1..=steps {
        let fraction = step as f32 / steps as f32;
        events.push(Event::Mouse(mouse::Event::CursorMoved {
            position: from + by * fraction,
        }));
    }

    ui.simulate(events);
    ui.point_at(from + by);
    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

/// Runs frames far enough apart for every animation to finish.
fn settle(ui: &mut Simulator) {
    let now = Instant::now();

    for second in 1..=3 {
        ui.simulate([Event::Window(window::Event::RedrawRequested(
            now + Duration::from_secs(second),
        ))]);
    }
}

#[test]
fn an_end_sheet_sits_at_the_right() {
    let mut ui = open(Some(sheet()));
    let title = title(&mut ui);

    assert!(title.x >= SIZE.0 - sheet::SIDE_WIDTH);
    assert!(title.x <= SIZE.0 - sheet::SIDE_WIDTH + 20.0);
}

#[test]
fn a_start_sheet_sits_at_the_left_and_mirrors_in_right_to_left() {
    let mut ui = open(Some(sheet().side(sheet::Side::Start)));
    assert!(title(&mut ui).x < 20.0);

    let mut ui = open(Some(
        sheet()
            .side(sheet::Side::Start)
            .direction(ReadingDirection::RightToLeft),
    ));
    assert!(title(&mut ui).x >= SIZE.0 - sheet::SIDE_WIDTH);
}

#[test]
fn a_side_sheet_leaves_a_margin_uncovered() {
    let mut ui = open(Some(sheet().size(2000.0)));
    assert!(title(&mut ui).x >= sheet::SIDE_MARGIN);

    let mut narrow = simulator_sized((40.0, 400.0), Some(sheet()));
    let _ = narrow
        .snapshot(&Theme::light())
        .expect("a narrow window draws");
}

#[test]
fn escape_the_close_button_and_the_scrim_dismiss() {
    let mut ui = open(Some(sheet()));
    ui.tap_key(Named::Escape);
    assert_eq!(messages(ui), vec![Message::Dismiss]);

    let mut ui = open(Some(sheet()));
    let title = title(&mut ui);
    ui.point_at(Point::new(SIZE.0 - 16.0 - 12.0, title.center_y()));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Close]);

    let mut ui = open(Some(sheet()));
    ui.point_at(Point::new(10.0, 400.0));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Dismiss]);

    let mut ui = open(Some(sheet().dismissible(false)));
    ui.point_at(Point::new(10.0, 400.0));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn a_modal_sheet_keeps_the_input() {
    let mut ui = open(Some(sheet().dismissible(false)));

    ui.click("Behind").expect("page button is found");
    ui.click(selector::id("save")).expect("sheet button");
    assert_eq!(messages(ui), vec![Message::Save]);
}

#[test]
fn tab_cycles_inside_a_modal_sheet() {
    let mut ui = open(Some(sheet()));
    let save = ui.find(selector::id("save")).expect("save").bounds();
    let focused = |ui: &mut Simulator| ui.find(selector::is_focused()).ok().map(|t| t.bounds());

    ui.tap_key(Named::Tab);
    let close = focused(&mut ui).expect("the close button takes the focus");
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(save));
    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(close), "wraps inside the sheet");
}

#[test]
fn dragging_toward_the_edge_dismisses() {
    let mut ui = open(Some(sheet()));
    let from = title(&mut ui).center();

    drag(&mut ui, from, iced::Vector::new(300.0, 0.0));
    assert_eq!(messages(ui), vec![Message::Dismiss]);
}

#[test]
fn a_short_drag_settles_back() {
    let mut ui = open(Some(sheet()));
    let before = title(&mut ui);

    drag(&mut ui, before.center(), iced::Vector::new(40.0, 0.0));
    assert!(title(&mut ui).x > before.x, "follows the drag");

    settle(&mut ui);
    assert_eq!(title(&mut ui), before);
    assert_eq!(messages(ui), Vec::<Message>::new());
}

#[test]
fn a_drag_that_does_not_pass_the_slop_still_clicks() {
    let mut ui = open(Some(sheet()));
    let save = ui.find(selector::id("save")).expect("save").bounds();

    drag(&mut ui, save.center(), iced::Vector::new(4.0, 0.0));
    assert_eq!(messages(ui), vec![Message::Save]);
}

#[test]
fn a_non_modal_sheet_leaves_the_page_usable() {
    let mut ui = open(Some(sheet().modal(false)));

    ui.click("Behind").expect("page button");
    ui.tap_key(Named::Escape);
    assert!(ui.find("Sheet").is_ok());

    // Behind, then the close button of the sheet.
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Escape);
    assert_eq!(messages(ui), vec![Message::Behind, Message::Dismiss]);
}

#[test]
fn a_bottom_sheet_rests_at_its_heights() {
    let mut low = open(Some(bottom(0)));
    let mut high = open(Some(bottom(1)));
    let low_top = title(&mut low).y;
    let high_top = title(&mut high).y;

    assert!((low_top - high_top - 0.5 * SIZE.1).abs() < 1.0);
    assert!(low_top > SIZE.1 * 0.7 && low_top < SIZE.1 * 0.7 + 60.0);
}

#[test]
fn the_handle_moves_between_heights_by_keyboard_and_click() {
    let mut ui = open(Some(bottom(0)));
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowUp);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Enter);
    assert_eq!(messages(ui), vec![Message::Snap(1), Message::Snap(1)]);

    let mut ui = open(Some(bottom(1)));
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowDown);
    ui.tap_key(Named::Space);
    assert_eq!(messages(ui), vec![Message::Snap(0), Message::Snap(0)]);

    let mut ui = open(Some(bottom(0)));
    let title = title(&mut ui);
    ui.point_at(Point::new(SIZE.0 / 2.0, title.y - 16.0 - 12.0));
    ui.simulate(simulator::click());
    assert_eq!(messages(ui), vec![Message::Snap(1)]);
}

#[test]
fn dragging_a_bottom_sheet_moves_between_heights_or_dismisses() {
    let mut ui = open(Some(bottom(0)));
    let from = title(&mut ui).center();
    drag(&mut ui, from, iced::Vector::new(0.0, -300.0));
    assert_eq!(messages(ui), vec![Message::Snap(1)]);

    let mut ui = open(Some(bottom(0)));
    let from = title(&mut ui).center();
    drag(&mut ui, from, iced::Vector::new(0.0, 150.0));
    assert_eq!(messages(ui), vec![Message::Dismiss]);
}

#[test]
fn a_sheet_opened_later_slides_in_from_its_edge() {
    use iced::advanced::clipboard;
    use iced::advanced::renderer::Headless;
    use iced::advanced::widget::operation::{Operation, Outcome, black_box};
    use iced_test::runtime::user_interface::{Cache, UserInterface};
    use iced_test::selector::Selector;

    let _fonts = open(None);
    let backend = std::env::var("ICED_TEST_BACKEND").ok();
    let mut renderer =
        iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
            oito2_iced::font::INTER,
            iced::Pixels(16.0),
            backend.as_deref(),
        ))
        .expect("headless renderer");
    let size = iced::Size::new(SIZE.0, SIZE.1);

    let closed = UserInterface::build(view(None), size, Cache::default(), &mut renderer);
    let mut ui = UserInterface::build(
        view(Some(sheet())),
        size,
        closed.into_cache(),
        &mut renderer,
    );

    type Ui<'a> = UserInterface<'a, Message, Theme, iced::Renderer>;

    fn title_x(ui: &mut Ui<'_>, renderer: &iced::Renderer) -> Option<f32> {
        let mut find = Selector::find("Sheet");
        ui.operate(renderer, &mut black_box(&mut find));

        match find.finish() {
            Outcome::Some(Some(target)) => Some(target.bounds().x),
            _ => None,
        }
    }

    fn frame(ui: &mut Ui<'_>, renderer: &mut iced::Renderer, at: Instant) {
        let mut messages = Vec::new();
        let _ = ui.update(
            &[Event::Window(window::Event::RedrawRequested(at))],
            mouse::Cursor::Unavailable,
            renderer,
            &mut clipboard::Null,
            &mut messages,
        );
    }

    let start = Instant::now();
    let hidden = title_x(&mut ui, &renderer).expect("title");
    assert!(hidden >= SIZE.0, "the first frame starts off screen");

    frame(&mut ui, &mut renderer, start);
    frame(&mut ui, &mut renderer, start + Duration::from_millis(100));
    let midway = title_x(&mut ui, &renderer).expect("title");
    assert!(midway < hidden && midway > SIZE.0 - sheet::SIDE_WIDTH + 16.0);

    frame(&mut ui, &mut renderer, start + Duration::from_secs(1));
    let shown = title_x(&mut ui, &renderer).expect("title");
    assert!((shown - (SIZE.0 - sheet::SIDE_WIDTH + 16.0)).abs() < 1.0);
}

fn snapshot(sheet: sheet::Sheet<'static, Message>, theme: &Theme, name: &str) {
    let mut ui = simulator_sized((640.0, 400.0), Some(sheet));
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn sheet_light() {
    snapshot(sheet(), &Theme::light(), "sheet_light");
}

#[test]
fn sheet_dark() {
    snapshot(sheet(), &Theme::dark(), "sheet_dark");
}

#[test]
fn sheet_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(sheet(), &theme, "sheet_light_high_contrast");
}

#[test]
fn sheet_bottom_light() {
    snapshot(bottom(0), &Theme::light(), "sheet_bottom_light");
}
