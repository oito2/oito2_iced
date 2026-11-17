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

//! Expandable sections and expansion panels: toggling by click and
//! keyboard, the collapsed content, the chevron position, the panel
//! sections and golden snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, text};
use iced_test::{selector, simulator};
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, sizes, spacing};
use oito2_iced::widget::expandable::{self, ButtonPosition};
use oito2_iced::{Element, Theme};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Advanced(bool),
    Other(bool),
    Section(usize, bool),
}

type Ui<'a> = simulator::Simulator<'a, Message, Theme>;

fn ui(view: Element<'static, Message>) -> Ui<'static> {
    simulator::Simulator::with_settings(support::settings(), view)
}

fn bounds(ui: &mut Ui<'_>, id: &'static str) -> iced::Rectangle {
    ui.find(selector::id(id)).expect("widget exists").bounds()
}

fn focused(ui: &mut Ui<'_>) -> Option<iced::Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn section(expanded: bool) -> Element<'static, Message> {
    expandable::expandable("Advanced", text("details"), expanded)
        .collapsed(text("summary"))
        .on_toggle(Message::Advanced)
        .id("advanced")
        .into()
}

#[test]
fn a_click_on_the_header_requests_the_opposite_state() {
    let mut collapsed = ui(section(false));
    collapsed.click("Advanced").expect("header");
    assert_eq!(
        collapsed.into_messages().collect::<Vec<_>>(),
        vec![Message::Advanced(true)]
    );

    let mut expanded = ui(section(true));
    expanded.click("Advanced").expect("header");
    assert_eq!(
        expanded.into_messages().collect::<Vec<_>>(),
        vec![Message::Advanced(false)]
    );
}

#[test]
fn the_content_follows_the_expanded_state() {
    let mut collapsed = ui(section(false));
    assert!(collapsed.find("details").is_err());
    assert!(collapsed.find("summary").is_ok());

    let mut expanded = ui(section(true));
    assert!(expanded.find("details").is_ok());
    assert!(expanded.find("summary").is_err());
}

#[test]
fn an_expanded_section_shows_its_full_content_at_once() {
    let view: Element<'static, Message> = column![
        expandable::expandable("Advanced", text("details").height(100), true)
            .on_toggle(Message::Advanced)
            .id("advanced"),
        text("after"),
    ]
    .into();
    let mut ui = ui(view);
    let header = bounds(&mut ui, "advanced");
    let after = ui.find("after").expect("text after").bounds();

    assert_eq!(after.y, header.y + header.height + spacing::XS + 100.0);
}

#[test]
fn the_header_is_one_tab_stop_activated_by_enter_and_space() {
    let view: Element<'static, Message> = oito2_iced::focus::navigation(column![
        expandable::expandable("Advanced", text("details"), false)
            .on_toggle(Message::Advanced)
            .id("advanced"),
        expandable::expandable("Disabled", text("hidden"), false).id("disabled"),
        expandable::expandable("Other", text("more"), true)
            .on_toggle(Message::Other)
            .id("other"),
    ])
    .into();
    let mut ui = ui(view);
    let advanced = bounds(&mut ui, "advanced");
    let other = bounds(&mut ui, "other");

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(advanced));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(other));
    ui.tap_key(Named::Space);

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Advanced(true), Message::Other(false)]
    );
}

#[test]
fn a_section_without_a_toggle_is_disabled() {
    let mut ui = ui(expandable::expandable("Advanced", text("details"), false)
        .id("advanced")
        .into());

    ui.click("Advanced").expect("header");
    assert_eq!(ui.into_messages().count(), 0);
}

#[test]
fn the_chevron_can_sit_before_the_header() {
    for (position, start) in [(ButtonPosition::End, true), (ButtonPosition::Start, false)] {
        let mut ui = ui(expandable::expandable("Advanced", text("details"), false)
            .button_position(position)
            .on_toggle(Message::Advanced)
            .id("advanced")
            .into());
        let header = bounds(&mut ui, "advanced");
        let label = ui.find("Advanced").expect("label").bounds();

        if start {
            assert_eq!(label.x, header.x, "{position:?}");
        } else {
            assert_eq!(
                label.x,
                header.x + sizes::ICON_SIZE + spacing::SM,
                "{position:?}"
            );
        }
    }
}

#[test]
fn the_header_shows_the_hand_cursor_while_enabled() {
    let enabled = || -> Element<'static, Message> { section(false) };
    let mut ui = ui(enabled());
    let header = bounds(&mut ui, "advanced");

    assert_eq!(
        support::mouse_interaction(enabled(), (1024.0, 768.0), header.center()),
        iced::mouse::Interaction::Pointer
    );

    let disabled: Element<'static, Message> =
        expandable::expandable("Advanced", text("details"), false).into();
    assert_ne!(
        support::mouse_interaction(disabled, (1024.0, 768.0), header.center()),
        iced::mouse::Interaction::Pointer
    );
}

fn panel(expanded: &[bool]) -> Element<'static, Message> {
    expandable::expansion_panel(expanded)
        .section("General", text("General settings"))
        .section("Network", text("Network settings"))
        .section("Privacy", text("Privacy settings"))
        .on_toggle(Message::Section)
        .into()
}

#[test]
fn a_panel_shows_the_open_sections_and_reports_activations() {
    let mut ui = ui(panel(&[false, true]));

    assert!(ui.find("General settings").is_err());
    assert!(ui.find("Network settings").is_ok());
    assert!(ui.find("Privacy settings").is_err());

    ui.click("Privacy").expect("privacy header");
    ui.click("Network").expect("network header");

    assert_eq!(
        ui.into_messages().collect::<Vec<_>>(),
        vec![Message::Section(2, true), Message::Section(1, false)]
    );
}

#[test]
fn set_expanded_keeps_one_section_open() {
    let mut open = vec![false, true, false];

    let changes = expandable::set_expanded(&mut open, 2, true, true);

    assert_eq!(changes, vec![(1, false), (2, true)]);
    assert_eq!(open, vec![false, false, true]);
}

fn catalog() -> Element<'static, Message> {
    column![
        expandable::expandable("Collapsed", text("details"), false).on_toggle(Message::Advanced),
        expandable::expandable("Expanded", text("The content of an open section."), true)
            .on_toggle(Message::Advanced),
        expandable::expandable("With a summary", text("details"), false)
            .collapsed(text("A summary while collapsed"))
            .button_position(ButtonPosition::Start)
            .on_toggle(Message::Advanced),
        expandable::expandable("Disabled", text("details"), false),
        panel(&[false, true, false]),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (420.0, 400.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn expandable_light() {
    snapshot(&Theme::light(), "expandable_light");
}

#[test]
fn expandable_dark() {
    snapshot(&Theme::dark(), "expandable_dark");
}

#[test]
fn expandable_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "expandable_light_high_contrast");
}
