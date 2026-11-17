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

//! Window controls, title bar and header bar: the action of each button in
//! every style, the button placement of each layout, the maximize and
//! restore buttons, hover feedback, the inactive dimming and the window
//! state.

use oito2_iced_test_utils as support;

use iced::widget::text;
use iced::window::Event as WindowEvent;
use iced::{Color, Event, Point, Rectangle, Size, mouse};
use iced_test::simulator::{self, Simulator};
use oito2_iced::foundation::{WindowButton, WindowButtonLayout, WindowControlStyle};
use oito2_iced::l10n::Localizations;
use oito2_iced::window::{self, Action, ControlType, Labels, Placement, WindowState};
use oito2_iced::{Element, Theme};

const SIZE: (f32, f32) = (480.0, 120.0);

const DRAWN_STYLES: [WindowControlStyle; 6] = [
    WindowControlStyle::Gnome,
    WindowControlStyle::Breeze,
    WindowControlStyle::Xfwm,
    WindowControlStyle::Cosmic,
    WindowControlStyle::Windows,
    WindowControlStyle::Macos,
];

const ALL_BUTTONS: [WindowButton; 3] = [
    WindowButton::Minimize,
    WindowButton::Maximize,
    WindowButton::Close,
];

fn controls(style: WindowControlStyle, state: WindowState) -> Element<'static, Action> {
    window::controls(style, &ALL_BUTTONS, state, &Labels::default(), |a| a)
}

fn simulator(view: impl Into<Element<'static, Action>>) -> Simulator<'static, Action, Theme> {
    Simulator::with_size(support::settings(), SIZE, view.into())
}

fn actions(ui: Simulator<'static, Action, Theme>) -> Vec<String> {
    ui.into_messages().map(|a| format!("{a:?}")).collect()
}

fn bounds(ui: &mut Simulator<'static, Action, Theme>, label: &str) -> Rectangle {
    ui.find(label).expect("labeled element").bounds()
}

fn hover(ui: &mut Simulator<'static, Action, Theme>, point: Point) {
    ui.point_at(point);
    ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: point })]);
}

fn pixel(ui: &mut Simulator<'static, Action, Theme>, theme: &Theme, point: Point) -> Color {
    let snapshot = ui.snapshot(theme).expect("snapshot");
    let image = support::pixels(&snapshot, "window_controls_probe");

    support::pixel_at(&image, point, SIZE.0)
}

fn close_to(actual: Color, expected: Color) -> bool {
    let [r, g, b, _] = actual.into_rgba8();
    let [er, eg, eb, _] = expected.into_rgba8();

    r.abs_diff(er) <= 3 && g.abs_diff(eg) <= 3 && b.abs_diff(eb) <= 3
}

#[test]
fn every_style_publishes_the_action_of_each_button() {
    let cases = [
        ("Minimize", Action::Minimize),
        ("Maximize", Action::ToggleMaximize),
        ("Close", Action::Close),
    ];

    for style in DRAWN_STYLES {
        for (label, action) in cases {
            let mut ui = simulator(controls(style, WindowState::default()));
            ui.click(label).expect("control found");

            assert_eq!(
                actions(ui),
                vec![format!("{action:?}")],
                "{style:?} {label}"
            );
        }
    }
}

#[test]
fn the_none_style_shows_no_buttons() {
    let mut ui = simulator(controls(WindowControlStyle::None, WindowState::default()));

    assert!(ui.find("Close").is_err());
}

#[test]
fn a_maximized_window_shows_the_restore_button() {
    let maximized = WindowState {
        maximized: true,
        ..WindowState::default()
    };
    let mut ui = simulator(controls(WindowControlStyle::Gnome, maximized));

    assert!(ui.find("Maximize").is_err());
    ui.click("Restore").expect("restore button");
    assert_eq!(actions(ui), vec![format!("{:?}", Action::ToggleMaximize)]);
    assert_eq!(
        ControlType::of(WindowButton::Maximize, maximized),
        ControlType::Restore
    );
    assert_eq!(
        ControlType::of(WindowButton::Maximize, WindowState::default()),
        ControlType::Maximize
    );
}

#[test]
fn the_labels_follow_the_localizations() {
    let labels = Labels::from(Localizations::pt_br());
    let mut ui = simulator(window::controls(
        WindowControlStyle::Gnome,
        &ALL_BUTTONS,
        WindowState::default(),
        &labels,
        |a| a,
    ));

    ui.click("Fechar").expect("close button");
    assert_eq!(actions(ui), vec![format!("{:?}", Action::Close)]);
}

#[test]
fn every_drawn_style_is_at_least_a_24_pixel_target() {
    for style in DRAWN_STYLES {
        let mut ui = simulator(controls(style, WindowState::default()));
        let close = bounds(&mut ui, "Close");

        assert!(
            close.width >= 24.0 && close.height >= 24.0,
            "{style:?} {close:?}"
        );
    }
}

#[test]
fn the_gnome_layout_puts_the_buttons_after_the_centered_title() {
    let mut ui = simulator(window::title_bar("Files", text(""), |a| a));
    let title = bounds(&mut ui, "Files");
    let minimize = bounds(&mut ui, "Minimize");
    let maximize = bounds(&mut ui, "Maximize");
    let close = bounds(&mut ui, "Close");

    assert!((title.center_x() - SIZE.0 / 2.0).abs() < 1.0, "{title:?}");
    assert!(title.x + title.width < minimize.x);
    assert!(minimize.x < maximize.x && maximize.x < close.x);
    assert!(
        (close.x + close.width - (SIZE.0 - 8.0)).abs() < 0.5,
        "{close:?}"
    );
}

#[test]
fn the_macos_layout_puts_the_buttons_before_the_title() {
    let mut ui = simulator(
        window::title_bar("Files", text(""), |a| a).control_style(WindowControlStyle::Macos),
    );
    let title = bounds(&mut ui, "Files");
    let close = bounds(&mut ui, "Close");
    let minimize = bounds(&mut ui, "Minimize");
    let maximize = bounds(&mut ui, "Maximize");

    assert!(close.x < minimize.x && minimize.x < maximize.x);
    assert!(maximize.x + maximize.width < title.x);
    assert!((title.center_x() - SIZE.0 / 2.0).abs() < 1.0, "{title:?}");
}

#[test]
fn a_layout_splits_the_buttons_between_both_sides() {
    let layout = WindowButtonLayout::new(
        vec![WindowButton::Close],
        vec![WindowButton::Minimize, WindowButton::Maximize],
    );
    let mut ui = simulator(window::title_bar("Files", text(""), |a| a).button_layout(layout));
    let title = bounds(&mut ui, "Files");
    let close = bounds(&mut ui, "Close");
    let minimize = bounds(&mut ui, "Minimize");

    assert!(close.x < title.x && title.x < minimize.x);
}

#[test]
fn the_windows_style_starts_the_title_and_fills_the_bar() {
    let mut ui = simulator(
        window::title_bar("Files", text(""), |a| a).control_style(WindowControlStyle::Windows),
    );
    let title = bounds(&mut ui, "Files");
    let close = bounds(&mut ui, "Close");

    assert!(title.x < 40.0, "{title:?}");
    assert_eq!(close.size(), Size::new(46.0, 45.0));
    assert_eq!(close.x + close.width, SIZE.0);
}

#[test]
fn a_button_in_the_title_bar_neither_drags_nor_opens_the_menu() {
    let mut ui = simulator(window::title_bar("Files", text(""), |a| a));
    let close = bounds(&mut ui, "Close");
    ui.point_at(close.center());
    ui.simulate([
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Right)),
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Right)),
    ]);
    ui.simulate(simulator::click());

    assert_eq!(actions(ui), vec![format!("{:?}", Action::Close)]);
}

#[test]
fn the_breeze_and_windows_close_buttons_turn_red_on_hover() {
    let cases = [
        (
            WindowControlStyle::Breeze,
            Color::from_rgb8(0xDA, 0x44, 0x53),
        ),
        (
            WindowControlStyle::Windows,
            Color::from_rgb8(0xC4, 0x2B, 0x1C),
        ),
    ];

    for (style, red) in cases {
        let mut ui = simulator(controls(style, WindowState::default()));
        let close = bounds(&mut ui, "Close");
        let probe = Point::new(close.center_x() + 8.0, close.center_y());
        let theme = Theme::light();

        let idle = pixel(&mut ui, &theme, probe);
        assert!(!close_to(idle, red), "{style:?} idle {idle:?}");

        hover(&mut ui, close.center());
        let hovered = pixel(&mut ui, &theme, probe);
        assert!(close_to(hovered, red), "{style:?} hovered {hovered:?}");
    }
}

#[test]
fn the_title_bar_dims_while_the_window_is_unfocused() {
    let theme = Theme::light();
    let probe = |ui: &mut Simulator<'static, Action, Theme>| {
        let close = bounds(ui, "Close");
        pixel(
            ui,
            &theme,
            Point::new(close.center_x() + 8.0, close.center_y()),
        )
    };

    let mut focused = simulator(window::title_bar("Files", text(""), |a| a));
    let mut unfocused = simulator(
        window::title_bar("Files", text(""), |a| a).state(WindowState {
            focused: false,
            ..WindowState::default()
        }),
    );

    let background = theme.roles().chrome.title_bar;
    let distance = |color: Color| {
        let [r, g, b, _] = color.into_rgba8();
        let [br, bg, bb, _] = background.into_rgba8();
        u32::from(r.abs_diff(br)) + u32::from(g.abs_diff(bg)) + u32::from(b.abs_diff(bb))
    };

    let focused = probe(&mut focused);
    let unfocused = probe(&mut unfocused);
    assert!(
        distance(unfocused) < distance(focused),
        "focused {focused:?}, unfocused {unfocused:?}"
    );
}

#[test]
fn the_header_bar_search_replaces_the_title() {
    let bar = |search: bool| {
        let bar = window::header_bar(|a| a)
            .title("Files")
            .start(text("Back"))
            .end(text("Menu"));

        if search {
            bar.search(text("Query"))
        } else {
            bar
        }
    };

    let mut ui = simulator(bar(false));
    let back = bounds(&mut ui, "Back");
    let title = bounds(&mut ui, "Files");
    let menu = bounds(&mut ui, "Menu");
    let minimize = bounds(&mut ui, "Minimize");
    assert!(back.x < title.x && title.x < menu.x && menu.x < minimize.x);
    assert!(ui.find("Query").is_err());

    let mut ui = simulator(bar(true));
    assert!(ui.find("Files").is_err());
    let query = bounds(&mut ui, "Query");
    assert!((query.center_x() - SIZE.0 / 2.0).abs() < 1.0, "{query:?}");
}

#[test]
fn the_window_state_follows_the_window_events() {
    let mut state = WindowState::default();
    assert!(state.focused && !state.maximized && !state.fullscreen);

    assert!(!state.update(&WindowEvent::Unfocused));
    assert!(!state.focused);
    assert!(!state.update(&WindowEvent::Focused));
    assert!(state.focused);

    assert!(state.update(&WindowEvent::Resized(Size::new(800.0, 600.0))));
    assert!(state.update(&WindowEvent::Moved(Point::ORIGIN)));
    assert!(!state.update(&WindowEvent::CloseRequested));

    state.apply(Placement {
        maximized: true,
        fullscreen: true,
    });
    assert!(state.maximized && state.fullscreen && state.focused);
}
