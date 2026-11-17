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

//! Golden snapshots of the title bar in every window control style, light
//! and dark. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::widget::{Column, text};
use iced_test::simulator::Simulator;
use oito2_iced::foundation::WindowControlStyle;
use oito2_iced::window::{self, WindowState};
use oito2_iced::{Element, Theme};

const WIDTH: f32 = 420.0;

fn view() -> Element<'static, ()> {
    let maximized = WindowState {
        maximized: true,
        ..WindowState::default()
    };
    let unfocused = WindowState {
        focused: false,
        ..WindowState::default()
    };

    let bars = [
        ("GNOME", WindowControlStyle::Gnome, WindowState::default()),
        ("Breeze", WindowControlStyle::Breeze, WindowState::default()),
        ("Xfwm", WindowControlStyle::Xfwm, WindowState::default()),
        ("COSMIC", WindowControlStyle::Cosmic, WindowState::default()),
        (
            "Windows",
            WindowControlStyle::Windows,
            WindowState::default(),
        ),
        ("macOS", WindowControlStyle::Macos, WindowState::default()),
        ("GNOME maximized", WindowControlStyle::Gnome, maximized),
        ("Windows maximized", WindowControlStyle::Windows, maximized),
        ("macOS unfocused", WindowControlStyle::Macos, unfocused),
        ("GNOME unfocused", WindowControlStyle::Gnome, unfocused),
    ];

    Column::with_children(bars.into_iter().map(|(title, style, state)| {
        window::title_bar(title, text(""), |_| ())
            .control_style(style)
            .state(state)
            .into()
    }))
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (WIDTH, 460.0), view());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn window_controls_light() {
    snapshot(&Theme::light(), "window_controls_light");
}

#[test]
fn window_controls_dark() {
    snapshot(&Theme::dark(), "window_controls_dark");
}
