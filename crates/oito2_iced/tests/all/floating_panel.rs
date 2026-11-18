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

//! Behaviour and golden tests of the floating panel widget.
//! Covers moving by the title bar, resizing by the corner, the title bar
//! buttons, the arrow keys, the pointer through to the layers below, the
//! kept content state and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::Id;
use iced::advanced::widget::operation::{Operation as _, Outcome, black_box, focusable};
use iced::keyboard::{self, key::Named};
use iced::widget::{container, stack, text, text_input};
use iced::{Event, Length, Point, Rectangle, Size, mouse};
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::foundation::WindowControlStyle;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::button;
use oito2_iced::widget::floating_panel::{self, FloatingPanel, PanelState};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Changed(PanelState),
    Closed,
    Below,
    Typed(String),
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

const AREA: (f32, f32) = (480.0, 360.0);

fn rect(x: f32, y: f32, width: f32, height: f32) -> Rectangle {
    Rectangle::new(Point::new(x, y), Size::new(width, height))
}

fn inspector(state: PanelState) -> FloatingPanel<'static, Message> {
    floating_panel::floating_panel(state, "Inspector", text("Panel body"))
        .on_change(Message::Changed)
        .on_close(Message::Closed)
        .id("panel")
}

/// The panel over a page with a button in its bottom-right corner.
fn view(panel: FloatingPanel<'static, Message>) -> Element<'static, Message> {
    focus::navigation(stack![
        container(button::filled("Below").on_press(Message::Below))
            .align_right(Length::Fill)
            .align_bottom(Length::Fill)
            .padding(8),
        panel,
    ])
    .into()
}

fn sim(panel: FloatingPanel<'static, Message>) -> Simulator<'static> {
    Simulator::with_size(support::settings(), AREA, view(panel))
}

fn at_50() -> PanelState {
    PanelState::new(rect(50.0, 50.0, 300.0, 200.0))
}

fn drag(ui: &mut Simulator<'_>, from: Point, by: (f32, f32)) {
    ui.point_at(from);
    ui.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    for step in 1..=4 {
        let t = step as f32 / 4.0;
        let at = Point::new(from.x + by.0 * t, from.y + by.1 * t);

        ui.point_at(at);
        ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: at })]);
    }

    ui.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
}

fn last_state(ui: Simulator<'_>) -> Option<PanelState> {
    ui.into_messages()
        .filter_map(|message| match message {
            Message::Changed(state) => Some(state),
            _ => None,
        })
        .last()
}

fn shift(key: Named) -> Event {
    let key = keyboard::Key::Named(key);

    Event::Keyboard(keyboard::Event::KeyPressed {
        key: key.clone(),
        modified_key: key,
        physical_key: keyboard::key::Physical::Unidentified(
            keyboard::key::NativeCode::Unidentified,
        ),
        location: keyboard::Location::Standard,
        modifiers: keyboard::Modifiers::SHIFT,
        repeat: false,
        text: None,
    })
}

#[test]
fn dragging_the_title_bar_moves_the_panel() {
    let mut ui = sim(inspector(at_50()));

    drag(&mut ui, Point::new(70.0, 70.0), (100.0, 40.0));

    let state = last_state(ui).expect("a change");

    assert_eq!(state.rect, rect(150.0, 90.0, 300.0, 200.0));
}

#[test]
fn the_title_bar_keeps_inside_the_area() {
    let mut ui = sim(inspector(at_50()));

    drag(&mut ui, Point::new(70.0, 70.0), (-200.0, 400.0));

    let state = last_state(ui).expect("a change");

    assert_eq!(state.rect.x, 0.0);
    assert_eq!(
        state.rect.y,
        AREA.1 - oito2_iced::tokens::sizes::TITLE_BAR_HEIGHT
    );
}

#[test]
fn dragging_the_corner_resizes_the_panel() {
    let mut ui = sim(inspector(at_50()));

    drag(&mut ui, Point::new(347.0, 247.0), (50.0, 30.0));

    let state = last_state(ui).expect("a change");

    assert_eq!(state.rect, rect(50.0, 50.0, 350.0, 230.0));
}

#[test]
fn the_edges_resize_along_one_axis() {
    let mut ui = sim(inspector(at_50()));

    drag(&mut ui, Point::new(348.0, 150.0), (30.0, 30.0));

    assert_eq!(
        last_state(ui).expect("a change").rect,
        rect(50.0, 50.0, 330.0, 200.0)
    );

    let mut ui = sim(inspector(at_50()));

    drag(&mut ui, Point::new(150.0, 248.0), (30.0, -30.0));

    assert_eq!(
        last_state(ui).expect("a change").rect,
        rect(50.0, 50.0, 300.0, 170.0)
    );
}

#[test]
fn a_fixed_panel_does_not_resize() {
    let mut ui = sim(inspector(at_50()).resizable(false));

    drag(&mut ui, Point::new(347.0, 247.0), (50.0, 30.0));

    assert_eq!(last_state(ui), None);
}

#[test]
fn the_cursor_shows_the_resize_direction() {
    let _fonts = sim(inspector(at_50()));
    let cursor_at = |panel: FloatingPanel<'static, Message>, x: f32, y: f32| {
        support::mouse_interaction(view(panel), AREA, Point::new(x, y))
    };

    assert_eq!(
        cursor_at(inspector(at_50()), 347.0, 247.0),
        mouse::Interaction::ResizingDiagonallyDown
    );
    assert_eq!(
        cursor_at(inspector(at_50()), 348.0, 150.0),
        mouse::Interaction::ResizingHorizontally
    );
    assert_eq!(
        cursor_at(inspector(at_50()), 150.0, 248.0),
        mouse::Interaction::ResizingVertically
    );
    assert_eq!(
        cursor_at(inspector(at_50()).resizable(false), 347.0, 247.0),
        mouse::Interaction::Idle
    );
    assert_eq!(
        cursor_at(inspector(at_50()), 420.0, 300.0),
        mouse::Interaction::None
    );
}

#[test]
fn a_double_click_on_the_title_bar_maximizes() {
    let mut ui = sim(inspector(at_50()));

    ui.point_at(Point::new(120.0, 70.0));
    ui.simulate(simulator::click().chain(simulator::click()));

    let state = last_state(ui).expect("a change");

    assert!(state.maximized);
    assert_eq!(state.rect, at_50().rect);
}

#[test]
fn the_buttons_maximize_minimize_and_close() {
    let mut ui = sim(inspector(at_50()));

    ui.click("Maximize").expect("maximize button");
    ui.click("Minimize").expect("minimize button");
    ui.click("Close").expect("close button");

    let messages: Vec<Message> = ui.into_messages().collect();

    assert_eq!(
        messages,
        vec![
            Message::Changed(at_50().toggle_maximized()),
            Message::Changed(at_50().toggle_minimized()),
            Message::Closed,
        ]
    );
}

#[test]
fn a_maximized_panel_covers_the_area_and_shows_restore() {
    let mut ui = sim(inspector(at_50().toggle_maximized()));

    let title = ui.find("Inspector").expect("title").bounds();

    assert!(title.y < 46.0);
    ui.click("Restore").expect("restore button");
    ui.click("Minimize").expect("minimize button");

    let minimized = last_state(ui).expect("a change");

    assert!(minimized.minimized && !minimized.maximized);
}

#[test]
fn a_minimized_panel_hides_its_content() {
    let mut ui = sim(inspector(at_50().toggle_minimized()));

    assert!(ui.find("Inspector").is_ok());
    assert!(ui.find("Panel body").is_err());
}

#[test]
fn without_on_close_there_is_no_close_button() {
    let mut ui = sim(floating_panel::floating_panel(
        at_50(),
        "Notes",
        text("Panel body"),
    ));

    assert!(ui.find("Minimize").is_ok());
    assert!(ui.find("Close").is_err());
}

#[test]
fn the_arrow_keys_move_and_shift_resizes_from_the_title_bar() {
    let mut ui = sim(inspector(at_50()));

    // The button of the layer below comes first.
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);

    let focused = ui
        .find(selector::is_focused())
        .ok()
        .map(|target| target.bounds());

    assert_eq!(focused, Some(rect(50.0, 50.0, 300.0, 46.0)));

    ui.tap_key(Named::ArrowRight);
    ui.simulate([shift(Named::ArrowDown)]);

    let changes: Vec<Rectangle> = ui
        .into_messages()
        .filter_map(|message| match message {
            Message::Changed(state) => Some(state.rect),
            _ => None,
        })
        .collect();

    assert_eq!(
        changes,
        vec![
            rect(66.0, 50.0, 300.0, 200.0),
            rect(50.0, 50.0, 300.0, 216.0)
        ]
    );
}

#[test]
fn without_on_change_the_panel_is_still_and_skipped() {
    let mut ui = sim(floating_panel::floating_panel(
        at_50(),
        "Notes",
        text("Panel body"),
    ));

    drag(&mut ui, Point::new(70.0, 70.0), (100.0, 40.0));
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowRight);

    assert!(ui.into_messages().next().is_none());
}

#[test]
fn the_pointer_reaches_the_layers_below_outside_the_panel() {
    let mut ui = sim(inspector(at_50()));
    let below = ui.find("Below").expect("button below").bounds();

    ui.point_at(below.center());
    ui.simulate(simulator::click());

    assert_eq!(ui.into_messages().collect::<Vec<_>>(), vec![Message::Below]);
}

#[test]
fn the_panel_blocks_the_layers_below() {
    let mut ui = sim(inspector(PanelState::new(rect(300.0, 250.0, 300.0, 200.0))));
    let below = ui.find("Below").expect("button below").bounds();

    ui.point_at(below.center());
    ui.simulate(simulator::click());

    assert!(ui.into_messages().all(|message| message != Message::Below));
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

fn with_field(state: PanelState) -> Element<'static, Message> {
    floating_panel::floating_panel(
        state,
        "Notes",
        text_input("Note", "").id("field").on_input(Message::Typed),
    )
    .on_change(Message::Changed)
    .into()
}

fn is_focused(ui: &mut Ui<'_>, renderer: &iced::Renderer) -> bool {
    let mut check = focusable::is_focused(Id::new("field"));

    ui.operate(renderer, &mut black_box(&mut check));

    matches!(check.finish(), Outcome::Some(true))
}

#[test]
fn minimizing_keeps_the_content_state() {
    let _fonts = sim(inspector(at_50()));
    let mut renderer = renderer();
    let size = Size::new(AREA.0, AREA.1);

    let mut ui: Ui<'_> =
        UserInterface::build(with_field(at_50()), size, Cache::default(), &mut renderer);
    let mut focus = focusable::focus::<()>(Id::new("field"));

    ui.operate(&renderer, &mut black_box(&mut focus));
    assert!(is_focused(&mut ui, &renderer));

    let mut messages = Vec::new();
    let _ = ui.update(
        &[],
        mouse::Cursor::Unavailable,
        &mut renderer,
        &mut clipboard::Null,
        &mut messages,
    );

    let minimized = UserInterface::build(
        with_field(at_50().toggle_minimized()),
        size,
        ui.into_cache(),
        &mut renderer,
    );
    let mut restored: Ui<'_> = UserInterface::build(
        with_field(at_50()),
        size,
        minimized.into_cache(),
        &mut renderer,
    );

    assert!(is_focused(&mut restored, &renderer));
}

/// The golden scene: a page with a panel at (40, 30), 300 × 200.
fn scene(panel: FloatingPanel<'static, Message>) -> Element<'static, Message> {
    focus::navigation(stack![
        container(text("Page content")).center(Length::Fill),
        panel,
    ])
    .into()
}

fn scene_panel(state: PanelState) -> FloatingPanel<'static, Message> {
    floating_panel::floating_panel(
        state,
        "Inspector",
        container(text("Width: 320\nHeight: 240\nOpacity: 100%")).padding(spacing::MD),
    )
    .on_change(Message::Changed)
    .on_close(Message::Closed)
}

fn scene_state() -> PanelState {
    PanelState::new(rect(40.0, 30.0, 300.0, 200.0))
}

fn snapshot(theme: &Theme, name: &str, panel: FloatingPanel<'static, Message>, focused: bool) {
    let mut ui = Simulator::with_size(support::settings(), (480.0, 300.0), scene(panel));

    if focused {
        ui.tap_key(Named::Tab);
    }

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn floating_panel_light() {
    snapshot(
        &Theme::light(),
        "floating_panel_light",
        scene_panel(scene_state()),
        false,
    );
}

#[test]
fn floating_panel_dark() {
    snapshot(
        &Theme::dark(),
        "floating_panel_dark",
        scene_panel(scene_state()),
        false,
    );
}

#[test]
fn floating_panel_minimized_breeze_light() {
    snapshot(
        &Theme::light(),
        "floating_panel_minimized_breeze_light",
        scene_panel(scene_state().toggle_minimized()).control_style(WindowControlStyle::Breeze),
        false,
    );
}

#[test]
fn floating_panel_dark_high_contrast_focused() {
    let theme = Theme::base(Brightness::Dark, Accent::default().color(), Contrast::High);

    snapshot(
        &theme,
        "floating_panel_dark_high_contrast_focused",
        scene_panel(scene_state()),
        true,
    );
}

#[test]
fn floating_panel_maximized_light() {
    snapshot(
        &Theme::light(),
        "floating_panel_maximized_light",
        scene_panel(scene_state().toggle_maximized()),
        false,
    );
}
