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

//! Master-detail pages: the wide and narrow layouts, selection, the back
//! button, the empty content, pane resizing and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::mouse;
use iced::widget::text;
use iced::{Event, Point};
use iced_test::{selector, simulator};
use oito2_iced::icon::{Icon, icon, lucide};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::button;
use oito2_iced::widget::master_detail::{self, detail_page, master_tile};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Select(usize),
    Back,
    Resized(f32),
    Add,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

const WIDE: (f32, f32) = (800.0, 480.0);
const NARROW: (f32, f32) = (400.0, 480.0);

const ICONS: [Icon; 3] = [lucide::WIFI, lucide::BLUETOOTH, lucide::MONITOR];
const NAMES: [&str; 3] = ["Network", "Bluetooth", "Display"];

fn page(selected: Option<usize>) -> Element<'static, Message> {
    focus::navigation(
        master_detail::master_detail(
            NAMES.len(),
            selected,
            |index, selected, _width| {
                master_tile(selected, Message::Select(index))
                    .leading(icon(ICONS[index]))
                    .title(NAMES[index])
                    .into()
            },
            |index, back| {
                detail_page(text(format!("Body {index}")))
                    .title(NAMES[index])
                    .back(back)
                    .into()
            },
        )
        .on_back(Message::Back)
        .on_resize(Message::Resized),
    )
    .into()
}

fn ui(selected: Option<usize>, size: (f32, f32)) -> Simulator<'static> {
    Simulator::with_size(support::settings(), size, page(selected))
}

fn focused(ui: &mut Simulator<'_>) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

#[test]
fn wide_layouts_show_the_list_and_the_first_page() {
    let mut ui = ui(None, WIDE);

    assert!(ui.find("Network").is_ok());
    assert!(ui.find("Display").is_ok());
    assert!(ui.find("Body 0").is_ok());

    ui.click("Display").expect("the display tile");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Select(2)]
    );

    let mut ui = self::ui(Some(2), WIDE);
    assert!(ui.find("Body 2").is_ok());
    assert!(ui.find("Body 0").is_err());
}

#[test]
fn the_wide_detail_has_no_back_button() {
    let mut ui = ui(Some(1), WIDE);

    for _ in 0..4 {
        ui.tap_key(Named::Tab);
    }
    // Three tiles and the pane handle; the detail has nothing focusable.
    let handle = focused(&mut ui).expect("the pane handle has the focus");
    assert_eq!(handle.center_x(), 280.5);

    ui.tap_key(Named::Tab);
    let next = focused(&mut ui);
    assert!(
        next.is_none_or(|bounds| bounds.x < 280.0),
        "nothing in the detail takes the focus: {next:?}"
    );
}

#[test]
fn narrow_layouts_show_the_detail_with_a_back_button() {
    let mut ui = ui(None, NARROW);
    assert!(ui.find("Network").is_ok());
    assert!(ui.find("Body 0").is_err());

    ui.click("Bluetooth").expect("the bluetooth tile");
    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Select(1)]
    );

    let mut ui = self::ui(Some(1), NARROW);
    assert!(ui.find("Body 1").is_ok());
    assert!(ui.find("Network").is_err());

    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);
    assert_eq!(ui.into_messages().collect::<Vec<_>>(), vec![Message::Back]);
}

#[test]
fn the_selection_survives_a_layout_change() {
    assert!(ui(Some(2), WIDE).find("Body 2").is_ok());
    assert!(ui(Some(2), NARROW).find("Body 2").is_ok());
}

#[test]
fn the_breakpoint_is_620_pixels() {
    assert!(ui(None, (620.0, 400.0)).find("Body 0").is_ok());
    assert!(ui(None, (619.0, 400.0)).find("Body 0").is_err());
}

#[test]
fn dragging_the_divider_resizes_the_master() {
    let mut ui = ui(None, WIDE);
    ui.point_at(Point::new(280.5, 200.0));
    let _ = ui.simulate([
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
        Event::Mouse(mouse::Event::CursorMoved {
            position: Point::new(320.5, 200.0),
        }),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)),
    ]);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Resized(320.0)]
    );
}

#[test]
fn tiles_receive_the_pane_width() {
    let view = |width: f32| -> Simulator<'static> {
        let page = master_detail::master_detail(
            1,
            None,
            |index, selected, width| {
                master_tile(selected, Message::Select(index))
                    .title(format!("Width {width}"))
                    .into()
            },
            |_, _| text("Body").into(),
        )
        .pane_size(250.0);

        Simulator::with_size(support::settings(), (width, 400.0), page)
    };

    assert!(view(800.0).find("Width 250").is_ok());
    assert!(view(500.0).find("Width 500").is_ok());
}

#[test]
fn an_empty_list_keeps_the_bar_and_the_footer() {
    let view = master_detail::master_detail(
        0,
        None,
        |index, selected, _| master_tile(selected, Message::Select(index)).into(),
        |_, _| text("Body").into(),
    )
    .master_bar(|| text("Search").into())
    .master_footer(|| text("Footer").into())
    .empty(|| text("Nothing here").into());
    let mut ui = Simulator::with_size(support::settings(), WIDE, view);

    assert!(ui.find("Nothing here").is_ok());
    assert!(ui.find("Search").is_ok());
    assert!(ui.find("Footer").is_ok());
}

#[test]
fn an_empty_list_without_bar_shows_only_the_empty_content() {
    let view = master_detail::master_detail(
        0,
        None,
        |index, selected, _| master_tile(selected, Message::Select(index)).into(),
        |_, _| text("Body").into(),
    )
    .empty(|| text("Nothing here").into());
    let mut ui = Simulator::with_size(support::settings(), WIDE, view);

    assert!(ui.find("Nothing here").is_ok());
    assert!(ui.find("Body").is_err());
}

#[test]
fn a_detail_page_without_title_actions_or_back_has_no_bar() {
    let view: Element<'static, Message> = detail_page(text("Body")).into();
    let mut ui = Simulator::with_size(support::settings(), NARROW, view);
    let body = ui.find("Body").expect("body").bounds();

    assert_eq!(body.y, 0.0);

    let view: Element<'static, Message> = detail_page(text("Body")).title("Title").into();
    let mut ui = Simulator::with_size(support::settings(), NARROW, view);
    let body = ui.find("Body").expect("body").bounds();

    assert_eq!(body.y, oito2_iced::tokens::sizes::TITLE_BAR_HEIGHT);
}

fn snapshot(selected: Option<usize>, size: (f32, f32), theme: &Theme, name: &str) {
    let view: Element<'static, Message> = focus::navigation(
        master_detail::master_detail(
            NAMES.len(),
            selected,
            |index, selected, _width| {
                master_tile(selected, Message::Select(index))
                    .leading(icon(ICONS[index]))
                    .title(NAMES[index])
                    .subtitle("Settings")
                    .into()
            },
            |index, back| {
                detail_page(text(format!("Body {index}")))
                    .title(NAMES[index])
                    .action(button::icon_button(lucide::ELLIPSIS_VERTICAL).on_press(Message::Add))
                    .back(back)
                    .floating_action_button(button::fab(lucide::PLUS).on_press(Message::Add))
                    .into()
            },
        )
        .on_back(Message::Back)
        .on_resize(Message::Resized),
    )
    .into();
    let mut ui = Simulator::with_size(support::settings(), size, view);
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

fn high_contrast() -> Theme {
    Theme::base(Brightness::Light, Accent::default().color(), Contrast::High)
}

#[test]
fn master_detail_light() {
    snapshot(Some(1), WIDE, &Theme::light(), "master_detail_light");
}

#[test]
fn master_detail_dark() {
    snapshot(Some(1), WIDE, &Theme::dark(), "master_detail_dark");
}

#[test]
fn master_detail_light_high_contrast() {
    snapshot(
        Some(1),
        WIDE,
        &high_contrast(),
        "master_detail_light_high_contrast",
    );
}

#[test]
fn master_detail_narrow_light() {
    snapshot(
        Some(1),
        NARROW,
        &Theme::light(),
        "master_detail_narrow_light",
    );
}

#[test]
fn master_detail_narrow_dark() {
    snapshot(Some(1), NARROW, &Theme::dark(), "master_detail_narrow_dark");
}
