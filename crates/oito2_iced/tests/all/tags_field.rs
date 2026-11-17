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

//! Tags fields: Enter, delimiters and paste, refused tags, the maximum,
//! the keys of the chips, the "x" and the clear button, suggestions, the
//! Tab order, the disabled field, the localized tooltips, and golden
//! snapshots. Run with `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens
//! with `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard::{Clipboard, Kind};
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation::scrollable::Scrollable;
use iced::advanced::widget::{Id, Operation, operation};
use iced::keyboard::{self, key::Named};
use iced::widget::{column, text_input};
use iced::{Event, Point, Rectangle, Size, Vector, mouse, window};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::collections::ListCollection;
use oito2_iced::density::{self, Density};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::tags_field::{self, TagsFieldStyle};
use oito2_iced::{Element, Theme};

/// A clipboard holding a fixed text.
struct Holding(Option<String>);

impl Clipboard for Holding {
    fn read(&self, _kind: Kind) -> Option<String> {
        self.0.clone()
    }

    fn write(&mut self, _kind: Kind, contents: String) {
        self.0 = Some(contents);
    }
}

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness {
    model: App,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    clipboard: Holding,
    messages: Vec<Message>,
}

const SIZE: Size = Size::new(480.0, 400.0);

/// The width of the field.
const WIDTH: f32 = 400.0;

impl Harness {
    fn new(model: App) -> Self {
        let settings = support::settings();
        let _fonts = simulator::Simulator::<()>::with_settings(
            support::settings(),
            iced::widget::Space::new(),
        );
        let backend = std::env::var("ICED_TEST_BACKEND").ok();
        let renderer =
            iced_test::futures::futures::executor::block_on(<iced::Renderer as Headless>::new(
                settings.default_font,
                settings.default_text_size,
                backend.as_deref(),
            ))
            .expect("headless renderer");

        Self {
            model,
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            clipboard: Holding(None),
            messages: Vec::new(),
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.model.view(),
            SIZE,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let _ = ui.update(
            &events,
            self.cursor,
            &mut self.renderer,
            &mut self.clipboard,
            &mut messages,
        );
        self.cache = Some(ui.into_cache());

        for message in messages {
            self.messages.push(message.clone());
            self.model.update(message);
        }
    }

    fn operate(&mut self, operation: &mut dyn Operation) {
        let mut ui = UserInterface::build(
            self.model.view(),
            SIZE,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        ui.operate(&self.renderer, operation);
        self.cache = Some(ui.into_cache());
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut find = selector.find();
        self.operate(&mut operation::black_box(&mut find));

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn bounds_of(&mut self, text: &str) -> Rectangle {
        self.find(text).expect("text shows").bounds()
    }

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    /// Whether the chip of `tag` has the focus.
    fn chip_focused(&mut self, tag: &str) -> bool {
        let label = self.bounds_of(tag);

        self.focused()
            .is_some_and(|chip| chip.contains(label.center()) && chip.height < 30.0)
    }

    /// The text in the input.
    fn typed(&mut self) -> String {
        let mut text = Typed(None);
        self.operate(&mut text);
        text.0.unwrap_or_default()
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn click_input(&mut self) {
        let input = self.find(selector::id("tags")).expect("input").bounds();
        self.click_at(input.center());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }

    fn command(&mut self, letter: &str) {
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::COMMAND,
            )),
            simulator::press_key(keyboard::Key::Character(letter.into()), None),
            simulator::release_key(keyboard::Key::Character(letter.into())),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }

    fn paste(&mut self, text: &str) {
        self.clipboard = Holding(Some(text.to_owned()));
        self.command("v");
    }

    /// The bounds of the open suggestion list and of its content.
    fn panel(&mut self) -> Option<(Rectangle, Rectangle)> {
        let mut panel = Panel::default();
        self.operate(&mut panel);
        panel.found
    }

    /// The center of the suggestion at `index`.
    fn option(&mut self, index: usize) -> Point {
        let (bounds, content) = self.panel().expect("the panel is open");

        Point::new(
            bounds.center_x(),
            content.y + spacing::XS + ROW * (index as f32 + 0.5),
        )
    }

    fn option_count(&mut self) -> usize {
        self.panel().map_or(0, |(_, content)| {
            ((content.height - 2.0 * spacing::XS) / ROW).round() as usize
        })
    }
}

/// The height of a suggestion row: one line of body medium text between
/// small paddings.
const ROW: f32 = 20.02 + 2.0 * spacing::SM;

/// Reads the text of the first text input.
struct Typed(Option<String>);

impl Operation for Typed {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(
        &mut self,
        id: Option<&Id>,
        _bounds: Rectangle,
        state: &mut dyn operation::TextInput,
    ) {
        if id == Some(&Id::new("tags")) && self.0.is_none() {
            self.0 = Some(state.text().to_owned());
        }
    }
}

/// Finds the bounds of the first scrollable and of its content.
#[derive(Default)]
struct Panel {
    found: Option<(Rectangle, Rectangle)>,
}

impl Operation for Panel {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        _translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.found.get_or_insert((bounds, content_bounds));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Tags(Vec<String>),
    Other(String),
}

struct App {
    tags: Vec<String>,
    max_tags: Option<usize>,
    suggestions: Option<ListCollection<&'static str>>,
    style: TagsFieldStyle,
    clearable: bool,
    disabled: bool,
    localizations: Localizations,
    other: String,
}

impl App {
    fn new(tags: &[&str]) -> Self {
        Self {
            tags: tags.iter().map(|tag| (*tag).to_owned()).collect(),
            max_tags: None,
            suggestions: None,
            style: TagsFieldStyle::Filled,
            clearable: false,
            disabled: false,
            localizations: Localizations::en(),
            other: String::new(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let mut field = tags_field::tags_field(&self.tags)
            .placeholder("Add tag")
            .style(self.style)
            .clearable(self.clearable)
            .localizations(&self.localizations)
            .id("tags")
            .width(WIDTH);

        if let Some(max) = self.max_tags {
            field = field.max_tags(max);
        }

        if let Some(suggestions) = &self.suggestions {
            field = field.suggestions(suggestions);
        }

        if !self.disabled {
            field = field.on_change(Message::Tags);
        }

        let other = text_input("Other", &self.other)
            .id("other")
            .on_input(Message::Other);

        oito2_iced::focus::navigation(column![field, other].spacing(12).padding(16)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Tags(tags) => self.tags = tags,
            Message::Other(text) => self.other = text,
        }
    }
}

fn tags(ui: &Harness) -> Vec<&str> {
    ui.model.tags.iter().map(String::as_str).collect()
}

#[test]
fn enter_and_delimiters_add_tags_and_repeats_are_refused() {
    let mut ui = Harness::new(App::new(&[]));

    ui.click_input();
    ui.type_text("flutter");
    ui.key(Named::Enter);
    assert_eq!(tags(&ui), ["flutter"]);
    assert_eq!(ui.typed(), "");

    ui.paste("dart, linux,flutter,gt");
    assert_eq!(tags(&ui), ["flutter", "dart", "linux"]);
    assert_eq!(ui.typed(), "gt");

    ui.type_text("k,");
    assert_eq!(tags(&ui), ["flutter", "dart", "linux", "gtk"]);
    assert_eq!(ui.typed(), "");
    assert_eq!(
        ui.focused(),
        ui.find(selector::id("tags")).map(|input| input.bounds()),
        "the input keeps the focus as chips are added"
    );
}

#[test]
fn a_pasted_pair_keeps_its_delimiter_when_the_first_is_refused() {
    let mut ui = Harness::new(App::new(&["dart"]));

    ui.click_input();
    ui.paste("dart,go");
    assert_eq!(tags(&ui), ["dart"]);
    assert_eq!(ui.typed(), "go");

    ui.command("a");
    ui.paste("dart,");
    assert_eq!(tags(&ui), ["dart"]);
    assert_eq!(ui.typed(), "dart");
}

#[test]
fn a_refused_tag_stays_in_the_field_with_the_caret_at_its_end() {
    let mut ui = Harness::new(App::new(&["dart"]));

    ui.click_input();
    ui.type_text("dart,");
    assert_eq!(ui.typed(), "dart");
    assert_eq!(tags(&ui), ["dart"]);

    ui.type_text("s");
    assert_eq!(ui.typed(), "darts");
}

#[test]
fn max_tags_stops_adding() {
    let mut ui = Harness::new(App {
        max_tags: Some(2),
        ..App::new(&[])
    });

    ui.click_input();
    ui.paste("a,b,c,");
    assert_eq!(tags(&ui), ["a", "b"]);

    ui.type_text("d");
    assert_eq!(ui.typed(), "", "a full field takes no text");
}

#[test]
fn backspace_reaches_and_removes_tags_and_the_x_removes_too() {
    let mut ui = Harness::new(App::new(&["one", "two", "three"]));

    ui.click_input();
    ui.key(Named::Backspace);
    assert!(ui.chip_focused("three"));

    ui.key(Named::ArrowLeft);
    assert!(ui.chip_focused("two"));

    ui.key(Named::Backspace);
    assert_eq!(tags(&ui), ["one", "three"]);
    assert!(ui.chip_focused("one"), "the previous chip takes the focus");

    ui.key(Named::ArrowRight);
    assert!(ui.chip_focused("three"));
    ui.key(Named::ArrowRight);
    assert_eq!(
        ui.focused(),
        ui.find(selector::id("tags")).map(|input| input.bounds()),
        "Right from the last chip returns to the input"
    );

    let label = ui.bounds_of("three");
    ui.click_at(Point::new(label.x + label.width + 11.0, label.center_y()));
    assert_eq!(tags(&ui), ["one"]);

    ui.key(Named::Tab);
    ui.key(Named::Delete);
    assert_eq!(tags(&ui), ["one"], "Tab left the field");
}

#[test]
fn removing_the_first_chip_focuses_the_input() {
    let mut ui = Harness::new(App::new(&["one", "two"]));

    ui.key(Named::Tab);
    assert!(
        ui.chip_focused("one"),
        "the chips come first in the Tab order"
    );

    ui.key(Named::Delete);
    assert_eq!(tags(&ui), ["two"]);
    assert_eq!(
        ui.focused(),
        ui.find(selector::id("tags")).map(|input| input.bounds())
    );
}

#[test]
fn the_tab_order_is_the_chips_then_the_input_then_the_clear_button() {
    let mut ui = Harness::new(App {
        clearable: true,
        ..App::new(&["dart", "linux"])
    });
    let input = ui.find(selector::id("tags")).map(|input| input.bounds());
    let other = ui.find(selector::id("other")).map(|other| other.bounds());

    ui.key(Named::Tab);
    assert!(ui.chip_focused("dart"));
    ui.key(Named::Tab);
    assert!(ui.chip_focused("linux"));
    ui.key(Named::Tab);
    assert_eq!(ui.focused(), input);
    ui.key(Named::Tab);
    let clear = ui.focused().expect("the clear button");
    assert!(clear.x > WIDTH - 40.0, "the clear button ends the field");
    ui.key(Named::Enter);
    assert!(
        ui.model.tags.is_empty(),
        "the clear button removes every tag"
    );

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), other);
}

#[test]
fn the_clear_button_removes_every_tag() {
    let mut ui = Harness::new(App {
        clearable: true,
        ..App::new(&["dart", "linux"])
    });
    let field = ui.find(selector::id("tags")).expect("input").bounds();

    ui.click_at(Point::new(16.0 + WIDTH - 17.0, field.center_y()));
    assert!(ui.model.tags.is_empty());

    let input = ui.find(selector::id("tags")).map(|input| input.bounds());
    assert_eq!(ui.focused(), input, "the input takes the focus");
}

#[test]
fn a_click_in_the_field_focuses_the_input() {
    let mut ui = Harness::new(App::new(&["dart"]));
    let label = ui.bounds_of("dart");

    ui.click_at(label.center());
    assert_eq!(
        ui.focused(),
        ui.find(selector::id("tags")).map(|input| input.bounds())
    );
}

#[test]
fn suggestions_add_a_tag() {
    let mut ui = Harness::new(App {
        suggestions: Some(
            ListCollection::new(["Dart", "Flutter", "Fuchsia"]).expect("unique suggestions"),
        ),
        ..App::new(&["Dart"])
    });

    ui.click_input();
    ui.type_text("f");
    assert_eq!(ui.option_count(), 2, "Flutter and Fuchsia");

    let flutter = ui.option(0);
    ui.click_at(flutter);
    assert_eq!(tags(&ui), ["Dart", "Flutter"]);
    assert_eq!(ui.typed(), "");
    assert_eq!(ui.option_count(), 0);

    ui.type_text("u");
    assert_eq!(ui.option_count(), 1, "the added tags are not offered");
    ui.key(Named::Escape);
    assert_eq!(ui.option_count(), 0);

    ui.type_text("c");
    ui.key(Named::ArrowDown);
    ui.key(Named::Enter);
    assert_eq!(tags(&ui), ["Dart", "Flutter", "Fuchsia"]);
}

#[test]
fn a_disabled_field_takes_no_focus_and_no_input() {
    let mut ui = Harness::new(App {
        disabled: true,
        ..App::new(&["dart"])
    });
    let other = ui.find(selector::id("other")).map(|other| other.bounds());
    let label = ui.bounds_of("dart");

    ui.click_at(Point::new(label.x + label.width + 60.0, label.center_y()));
    ui.type_text("go,");
    ui.key(Named::Backspace);
    assert!(ui.messages.is_empty());

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), other, "Tab skips the chips and the input");
}

/// Iced reports no text of a tooltip to widget operations, so the
/// localized tooltip of the "x" is checked in a snapshot.
#[test]
fn tags_field_remove_tooltip_pt_br() {
    let pt = Localizations::pt_br();
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (384.0, 110.0),
        column![
            tags_field::tags_field(&["dart", "linux"])
                .localizations(&pt)
                .on_change(Message::Tags)
                .width(360)
        ]
        .padding(12),
    );
    let label = ui.find("dart").expect("chip").bounds();
    let x = Point::new(label.x + label.width + 11.0, label.center_y());

    ui.point_at(x);
    let _ = ui.simulate([Event::Mouse(mouse::Event::CursorMoved { position: x })]);
    std::thread::sleep(oito2_iced::widget::TOOLTIP_DELAY);
    let _ = ui.simulate([Event::Window(window::Event::RedrawRequested(
        std::time::Instant::now(),
    ))]);

    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "tags_field_remove_tooltip_pt_br_light");
}

#[test]
fn the_field_is_as_tall_as_a_control_and_grows_with_rows() {
    for (density, height) in [(Density::Pointer, 34.0), (Density::Touch, 40.0)] {
        density::set(density);

        let mut ui = Harness::new(App::new(&["dart"]));
        let label = ui.bounds_of("dart");
        let input = ui.find(selector::id("tags")).expect("input").bounds();
        assert_eq!(input.height, 24.0);
        assert!(
            (label.center_y() - (16.0 + height / 2.0)).abs() < 1.0,
            "{density:?}"
        );
    }

    density::set(Density::Pointer);

    let many: Vec<String> = (0..12).map(|index| format!("tag{index}")).collect();
    let many: Vec<&str> = many.iter().map(String::as_str).collect();
    let mut ui = Harness::new(App::new(&many));
    let first = ui.bounds_of("tag0");
    let last = ui.bounds_of("tag11");
    assert!(last.y > first.y + 20.0, "the chips wrap onto more rows");
}

fn catalog() -> Element<'static, Message> {
    let tags = ["flutter", "dart", "linux"];

    column![
        tags_field::tags_field(&tags)
            .label("Tags")
            .placeholder("Add tag")
            .on_change(Message::Tags)
            .width(360),
        tags_field::tags_field(&tags)
            .style(TagsFieldStyle::Subtle)
            .clearable(true)
            .id("subtle")
            .on_change(Message::Tags)
            .width(360),
        tags_field::tags_field::<Message, &str>(&[])
            .placeholder("Add tag")
            .on_change(Message::Tags)
            .width(360),
        tags_field::tags_field::<Message, _>(&tags).width(360),
    ]
    .spacing(12)
    .padding(12)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (384.0, 210.0), catalog());

    // Focus the last chip of the subtle field.
    let _ = ui.click(selector::id("subtle")).expect("subtle field");
    let _ = ui.tap_key(Named::Backspace);
    ui.point_at(Point::new(2.0, 2.0));

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn tags_field_light() {
    snapshot(&Theme::light(), "tags_field_light");
}

#[test]
fn tags_field_dark() {
    snapshot(&Theme::dark(), "tags_field_dark");
}

#[test]
fn tags_field_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "tags_field_light_high_contrast");
}

#[test]
fn tags_field_suggestions_light() {
    let suggestions =
        ListCollection::new(["Dart", "Flutter", "Fuchsia"]).expect("unique suggestions");
    let mut ui = simulator::Simulator::with_size(
        support::settings(),
        (384.0, 220.0),
        column![
            tags_field::tags_field(&["Dart"])
                .suggestions(&suggestions)
                .id("tags")
                .on_change(Message::Tags)
                .width(360)
        ]
        .padding(12),
    );

    let _ = ui.click(selector::id("tags")).expect("input");
    let _ = ui.typewrite("f");
    ui.point_at(Point::new(2.0, 2.0));

    let snapshot = ui.snapshot(&Theme::light()).expect("snapshot");

    support::golden!(&snapshot, "tags_field_suggestions_light");
}
