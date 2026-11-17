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

//! Grouped controls: segment groups, toggle groups and choice chip bars.
//! Clicks, roving keyboard focus, disabled items, sizes per density and
//! golden snapshots of the catalog. Run with `ICED_TEST_BACKEND=tiny-skia`;
//! refresh the goldens with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::keyboard::key::Named;
use iced::widget::{column, container, row};
use iced::{Point, Rectangle};
use iced_test::{selector, simulator};
use oito2_iced::density::{self, Density};
use oito2_iced::icon::lucide;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::{button, chips, segment_group, toggle_group};
use oito2_iced::{Element, Theme, focus};

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Fruit(&'static str),
    Styles(Vec<&'static str>),
    Chip(usize),
    After,
}

type Simulator<'a> = simulator::Simulator<'a, Message, Theme>;

/// A selection, the item clicked and the selection published, if any.
type SingleCase = (
    &'static [&'static str],
    &'static str,
    Option<Vec<&'static str>>,
);

const FRUITS: [(&str, &str); 4] = [
    ("apple", "Apple"),
    ("banana", "Banana"),
    ("cherry", "Cherry"),
    ("date", "Date"),
];

fn fruits(selected: &'static str) -> Element<'static, Message> {
    segment_group::segment_group(
        FRUITS
            .map(|(value, label)| segment_group::segment(value, label).enabled(value != "cherry")),
        Some(selected),
    )
    .on_change(Message::Fruit)
    .into()
}

fn with_after(content: Element<'static, Message>) -> Element<'static, Message> {
    focus::navigation(
        column![
            content,
            button::filled("After").on_press(Message::After).id("after")
        ]
        .spacing(8)
        .padding(8),
    )
    .into()
}

fn styles(
    selected: &'static [&'static str],
    multiple: bool,
    deselectable: bool,
) -> Element<'static, Message> {
    toggle_group::toggle_group(
        [
            toggle_group::item("b", "B"),
            toggle_group::item("i", "I"),
            toggle_group::item("u", "U"),
        ],
        selected,
    )
    .multiple(multiple)
    .deselectable(deselectable)
    .on_change(Message::Styles)
    .into()
}

fn messages(ui: Simulator<'_>) -> Vec<Message> {
    ui.into_messages().collect()
}

fn focused(ui: &mut Simulator<'_>) -> Option<Rectangle> {
    ui.find(selector::is_focused())
        .ok()
        .map(|target| target.bounds())
}

fn text_center(ui: &mut Simulator<'_>, text: &'static str) -> Point {
    ui.find(text).expect("text exists").bounds().center()
}

/// Whether the focused element holds the text `label`.
fn focus_holds(ui: &mut Simulator<'_>, label: &'static str) -> bool {
    let center = text_center(ui, label);

    focused(ui).is_some_and(|bounds| bounds.contains(center))
}

#[test]
fn segments_select_by_click_except_disabled_and_selected_ones() {
    let mut ui = Simulator::with_settings(support::settings(), fruits("apple"));

    ui.click("Banana").expect("banana");
    ui.click("Cherry").expect("cherry");
    ui.click("Apple").expect("apple");

    assert_eq!(messages(ui), vec![Message::Fruit("banana")]);
}

#[test]
fn arrow_keys_select_the_next_enabled_segment_and_wrap() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(fruits("apple")));

    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "Apple"));

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::End);
    ui.tap_key(Named::Home);
    ui.tap_key(Named::ArrowDown);

    assert_eq!(
        messages(ui),
        vec![
            Message::Fruit("banana"),
            Message::Fruit("date"),
            Message::Fruit("date"),
        ]
    );

    let mut ui = Simulator::with_settings(support::settings(), with_after(fruits("banana")));
    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "Banana"));
    ui.tap_key(Named::ArrowRight);

    let mut wrapped = Simulator::with_settings(support::settings(), with_after(fruits("date")));
    wrapped.tap_key(Named::Tab);
    wrapped.tap_key(Named::ArrowRight);

    assert_eq!(messages(ui), vec![Message::Fruit("date")], "skips cherry");
    assert_eq!(messages(wrapped), vec![Message::Fruit("apple")], "wraps");
}

#[test]
fn a_segment_group_is_one_tab_stop() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(fruits("banana")));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "Banana"));

    ui.tap_key(Named::Tab);
    assert_eq!(focused(&mut ui), Some(after));
}

#[test]
fn a_disabled_segment_group_is_skipped() {
    let view = segment_group::segment_group(
        [
            segment_group::segment("a", "Alpha"),
            segment_group::segment("b", "Beta"),
        ],
        Some("a"),
    )
    .into();
    let mut ui = Simulator::with_settings(support::settings(), with_after(view));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.click("Beta").expect("beta");
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), Some(after));
    assert!(messages(ui).is_empty());
}

#[test]
fn a_single_toggle_group_keeps_one_item_on_and_can_turn_it_off() {
    let cases: [SingleCase; 3] = [
        (&[], "B", Some(vec!["b"])),
        (&["b"], "I", Some(vec!["i"])),
        (&["i"], "I", Some(vec![])),
    ];

    for (selected, click, expected) in cases {
        let mut ui = Simulator::with_settings(support::settings(), styles(selected, false, true));
        ui.click(click).expect("item");

        assert_eq!(
            messages(ui),
            expected
                .into_iter()
                .map(Message::Styles)
                .collect::<Vec<_>>(),
            "{selected:?} + {click}"
        );
    }
}

#[test]
fn a_non_deselectable_toggle_group_keeps_its_item_on() {
    let mut ui = Simulator::with_settings(support::settings(), styles(&["b"], false, false));

    ui.click("B").expect("item");

    assert!(messages(ui).is_empty());
}

#[test]
fn a_multiple_toggle_group_toggles_items_on_their_own() {
    let cases: [(&'static [&'static str], &str, Vec<&str>); 3] = [
        (&[], "B", vec!["b"]),
        (&["b"], "U", vec!["b", "u"]),
        (&["b", "u"], "B", vec!["u"]),
    ];

    for (selected, click, expected) in cases {
        let mut ui = Simulator::with_settings(support::settings(), styles(selected, true, true));
        ui.click(click).expect("item");

        assert_eq!(messages(ui), vec![Message::Styles(expected)]);
    }
}

#[test]
fn arrows_move_the_toggle_focus_and_space_toggles() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(styles(&[], true, true)));

    ui.tap_key(Named::Tab);
    assert!(focus_holds(&mut ui, "B"));

    ui.tap_key(Named::ArrowRight);
    assert!(focus_holds(&mut ui, "I"));
    ui.tap_key(Named::Space);

    ui.tap_key(Named::ArrowRight);
    ui.tap_key(Named::ArrowRight);
    assert!(focus_holds(&mut ui, "B"), "wraps to the first item");

    ui.tap_key(Named::End);
    assert!(focus_holds(&mut ui, "U"));
    ui.tap_key(Named::Enter);

    ui.tap_key(Named::ArrowDown);
    assert!(
        focus_holds(&mut ui, "U"),
        "vertical arrows do nothing across"
    );

    assert_eq!(
        messages(ui),
        vec![Message::Styles(vec!["i"]), Message::Styles(vec!["u"])]
    );
}

#[test]
fn a_click_moves_the_toggle_focus_to_the_item() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(styles(&[], true, true)));

    ui.click("U").expect("item");
    ui.tap_key(Named::ArrowLeft);
    ui.tap_key(Named::Space);

    assert_eq!(
        messages(ui),
        vec![Message::Styles(vec!["u"]), Message::Styles(vec!["i"])]
    );
}

#[test]
fn disabled_toggle_items_are_skipped() {
    let view = toggle_group::toggle_group(
        [
            toggle_group::item("b", "B"),
            toggle_group::item("i", "I").enabled(false),
            toggle_group::item("u", "U"),
        ],
        &[],
    )
    .vertical(true)
    .on_change(Message::Styles)
    .into();
    let mut ui = Simulator::with_settings(support::settings(), with_after(view));

    ui.click("I").expect("item");
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::ArrowDown);
    assert!(focus_holds(&mut ui, "U"));
    ui.tap_key(Named::ArrowDown);
    assert!(focus_holds(&mut ui, "B"));

    assert!(messages(ui).is_empty());
}

#[test]
fn group_items_follow_the_control_height_of_the_density() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let view: Element<'static, Message> = column![
            container(fruits("apple")).id("segments"),
            container(styles(&["b"], false, true)).id("toggles"),
        ]
        .into();
        let mut ui = Simulator::with_settings(support::settings(), view);
        let segments = ui
            .find(selector::id("segments"))
            .expect("segments")
            .bounds();
        let toggles = ui.find(selector::id("toggles")).expect("toggles").bounds();

        assert_eq!(segments.height, height + 2.0 * 5.0, "{density:?}");
        assert_eq!(toggles.height, height + 2.0 * 3.0, "{density:?}");
        assert!(toggles.width >= 3.0 * height + 2.0, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn chip_bar(width: f32, enabled: bool) -> Element<'static, Message> {
    let labels: Vec<String> = (0..10).map(|index| format!("Chip {index}")).collect();
    let selected: Vec<bool> = (0..10).map(|index| index == 1).collect();

    container(
        chips::choice_chip_bar(labels, &selected).on_select_maybe(enabled.then_some(Message::Chip)),
    )
    .width(width)
    .into()
}

#[test]
fn chips_publish_their_index_on_click_and_on_enter() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(chip_bar(2000.0, true)));

    ui.click("Chip 3").expect("chip");
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Tab);
    ui.tap_key(Named::Enter);

    assert_eq!(messages(ui), vec![Message::Chip(3), Message::Chip(1)]);
}

#[test]
fn a_disabled_chip_bar_publishes_nothing_and_is_skipped() {
    let mut ui = Simulator::with_settings(support::settings(), with_after(chip_bar(2000.0, false)));
    let after = ui.find(selector::id("after")).expect("after").bounds();

    ui.click("Chip 3").expect("chip");
    ui.tap_key(Named::Tab);

    assert_eq!(focused(&mut ui), Some(after));
    assert!(messages(ui).is_empty());
}

#[test]
fn the_forward_arrow_scrolls_the_chips() {
    let mut ui = Simulator::with_size(support::settings(), (300.0, 100.0), chip_bar(300.0, true));
    let height = density::control_height();
    let middle = (height + 4.0) / 2.0;

    ui.point_at(Point::new(40.0, middle));
    ui.simulate(simulator::click());

    ui.point_at(Point::new(300.0 - height / 2.0, middle));
    ui.simulate(simulator::click());

    ui.point_at(Point::new(40.0 + height, middle));
    ui.simulate(simulator::click());

    let messages = messages(ui);
    assert_eq!(messages.first(), Some(&Message::Chip(0)));
    assert_eq!(messages.len(), 2, "{messages:?}");
    assert_ne!(messages[1], Message::Chip(0), "the row scrolled");
}

#[test]
fn the_chip_bar_is_as_tall_as_a_control() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let view: Element<'static, Message> = container(chip_bar(2000.0, true)).id("bar").into();
        let mut ui = Simulator::with_settings(support::settings(), view);
        let bar = ui.find(selector::id("bar")).expect("bar").bounds();

        assert_eq!(bar.height, height + 4.0, "{density:?}");
    }

    density::set(Density::Pointer);
}

fn catalog() -> Element<'static, Message> {
    let labels = [
        "All",
        "Images",
        "Videos",
        "Documents",
        "Audio",
        "Archives",
        "Code",
    ];

    column![
        row![
            fruits("banana"),
            toggle_group::toggle_group(
                [
                    toggle_group::icon_item("b", lucide::LETTER_B).tooltip("Bold"),
                    toggle_group::icon_item("i", lucide::ITALIC).tooltip("Italic"),
                    toggle_group::icon_item("u", lucide::UNDERLINE).tooltip("Underline"),
                ],
                &["b", "u"],
            )
            .multiple(true)
            .on_change(Message::Styles),
        ]
        .spacing(12)
        .align_y(iced::Alignment::Center),
        row![
            segment_group::segment_group(
                [
                    segment_group::segment("list", "List").icon(lucide::LIST),
                    segment_group::segment("grid", "Grid").icon(lucide::LAYOUT_GRID),
                    segment_group::segment("off", "Off").enabled(false),
                ],
                Some("grid"),
            )
            .vertical(true)
            .on_change(Message::Fruit),
            toggle_group::toggle_group(
                [
                    toggle_group::item("left", "Left"),
                    toggle_group::item("center", "Center"),
                    toggle_group::item("right", "Right").enabled(false),
                ],
                &["center"],
            )
            .vertical(true)
            .on_change(Message::Styles),
            column![
                container(
                    chips::choice_chip_bar(
                        labels,
                        &[false, true, false, true, false, false, false]
                    )
                    .on_select(Message::Chip)
                )
                .width(320),
                chips::choice_chip_bar(labels, &[true, false, false, false, false, false, false])
                    .pill(true)
                    .show_checkmark(false)
                    .style(chips::BarStyle::Wrap)
                    .on_select(Message::Chip),
                chips::choice_chip_bar(["Disabled", "Selected"], &[false, true]),
            ]
            .spacing(12)
            .width(360),
        ]
        .spacing(12),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = Simulator::with_size(support::settings(), (640.0, 320.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn groups_light() {
    snapshot(&Theme::light(), "groups_light");
}

#[test]
fn groups_dark() {
    snapshot(&Theme::dark(), "groups_dark");
}

#[test]
fn groups_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "groups_light_high_contrast");
}
