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

//! Swaps: the shown child, the timed change and its size, reduced motion,
//! every effect mid-change, and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use std::time::Duration;

use iced::advanced::clipboard;
use iced::advanced::renderer::{Headless, Style};
use iced::advanced::widget::operation::{Operation, Outcome, black_box};
use iced::time::Instant;
use iced::widget::{Space, column, container, row, text};
use iced::{Event, Size, mouse, window};
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::selector::{self, Selector};
use iced_test::simulator;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::swap::{self, SwapEffect};
use oito2_iced::{Element, Theme};

type Ui<'a> = UserInterface<'a, (), Theme, iced::Renderer>;

/// A swap between a 20 × 20 square and a 40 × 10 bar, in a container
/// named "swap".
fn sized(swapped: bool, effect: SwapEffect, animated: bool) -> Element<'static, ()> {
    container(
        swap::swap(
            swapped,
            Space::new().width(20).height(20),
            Space::new().width(40).height(10),
        )
        .effect(effect)
        .animated(animated),
    )
    .id("swap")
    .into()
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

fn size_of(ui: &mut Ui<'_>, renderer: &iced::Renderer) -> Option<Size> {
    let mut find = selector::id("swap").find();
    ui.operate(renderer, &mut black_box(&mut find));

    match find.finish() {
        Outcome::Some(Some(target)) => Some(target.bounds().size()),
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

/// Builds the swap with `swapped` false, then rebuilds it with `swapped`
/// true, as an application does after a change.
fn changed<'a>(effect: SwapEffect, animated: bool, renderer: &mut iced::Renderer) -> Ui<'a> {
    let size = Size::new(200.0, 100.0);
    let before = UserInterface::build(
        sized(false, effect, animated),
        size,
        Cache::default(),
        renderer,
    );

    UserInterface::build(
        sized(true, effect, animated),
        size,
        before.into_cache(),
        renderer,
    )
}

#[test]
fn the_swapped_child_shows() {
    let view = |swapped: bool| -> Element<'static, ()> {
        swap::swap(swapped, text("Play"), text("Pause")).into()
    };

    let mut ui = simulator::Simulator::with_settings(support::settings(), view(false));
    assert!(ui.find("Play").is_ok());
    assert!(ui.find("Pause").is_err());

    let mut ui = simulator::Simulator::with_settings(support::settings(), view(true));
    assert!(ui.find("Pause").is_ok());
    assert!(ui.find("Play").is_err());
}

#[test]
fn a_change_holds_both_children_for_its_duration() {
    let _fonts = simulator::Simulator::with_settings(
        support::settings(),
        sized(false, SwapEffect::Fade, true),
    );
    let mut renderer = renderer();
    let mut ui = changed(SwapEffect::Fade, true, &mut renderer);

    assert_eq!(size_of(&mut ui, &renderer), Some(Size::new(40.0, 20.0)));

    let start = Instant::now();
    frame(&mut ui, &mut renderer, start);
    frame(&mut ui, &mut renderer, start + swap::DURATION / 2);
    assert_eq!(size_of(&mut ui, &renderer), Some(Size::new(40.0, 20.0)));

    frame(&mut ui, &mut renderer, start + swap::DURATION);
    assert_eq!(size_of(&mut ui, &renderer), Some(Size::new(40.0, 10.0)));
}

#[test]
fn without_animations_the_change_is_instant() {
    let mut renderer = renderer();
    let mut ui = changed(SwapEffect::Scale, false, &mut renderer);

    assert_eq!(size_of(&mut ui, &renderer), Some(Size::new(40.0, 10.0)));
}

#[test]
fn every_effect_draws_mid_change() {
    let mut renderer = renderer();

    for effect in [
        SwapEffect::Fade,
        SwapEffect::Scale,
        SwapEffect::Rotate,
        SwapEffect::Flip,
    ] {
        let mut ui = changed(effect, true, &mut renderer);
        let start = Instant::now();

        for step in 0..=4 {
            frame(
                &mut ui,
                &mut renderer,
                start + Duration::from_millis(50 * step),
            );
            ui.draw(
                &mut renderer,
                &Theme::light(),
                &Style::default(),
                mouse::Cursor::Unavailable,
            );
        }

        assert_eq!(size_of(&mut ui, &renderer), Some(Size::new(40.0, 10.0)));
    }
}

fn catalog() -> Element<'static, ()> {
    use oito2_iced::icon::{icon, lucide};

    column![
        row![
            swap::swap(false, icon(lucide::COPY), icon(lucide::CHECK)),
            swap::swap(true, icon(lucide::COPY), icon(lucide::CHECK)),
            swap::swap(false, icon(lucide::PLAY), icon(lucide::PAUSE)).effect(SwapEffect::Scale),
            swap::swap(true, icon(lucide::PLAY), icon(lucide::PAUSE)).effect(SwapEffect::Flip),
        ]
        .spacing(16),
        row![
            swap::swap(false, text("Off"), text("On")),
            swap::swap(true, text("Off"), text("On")),
        ]
        .spacing(16),
    ]
    .spacing(12)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (200.0, 100.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn swap_light() {
    snapshot(&Theme::light(), "swap_light");
}

#[test]
fn swap_dark() {
    snapshot(&Theme::dark(), "swap_dark");
}

#[test]
fn swap_dark_high_contrast() {
    let theme = Theme::base(Brightness::Dark, Accent::default().color(), Contrast::High);

    snapshot(&theme, "swap_dark_high_contrast");
}
