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

//! Autocomplete: the suggestions ignore accents and skip disabled items,
//! the arrows move the highlight while the field keeps the focus, Enter and
//! a click pick, Escape closes the panel, the panel takes the field width;
//! golden snapshots with the panel open. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::Id;
use iced::advanced::widget::operation::scrollable::Scrollable;
use iced::advanced::widget::operation::{self, Operation};
use iced::keyboard::key::Named;
use iced::widget::column;
use iced::{Event, Point, Rectangle, Size, Vector, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::collections::ListCollection;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::{Accent, spacing};
use oito2_iced::widget::autocomplete::autocomplete;
use oito2_iced::{Element, Theme};

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness<M: Model> {
    model: M,
    size: Size,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    messages: Vec<M::Message>,
}

trait Model {
    type Message: Clone;

    fn view(&self) -> Element<'_, Self::Message>;

    fn update(&mut self, message: Self::Message);
}

impl<M: Model> Harness<M> {
    fn new(model: M, size: Size) -> Self {
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
            size,
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            messages: Vec::new(),
        }
    }

    fn send(&mut self, events: impl IntoIterator<Item = Event>) {
        let events: Vec<Event> = events.into_iter().collect();
        let mut messages = Vec::new();
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let _ = ui.update(
            &events,
            self.cursor,
            &mut self.renderer,
            &mut clipboard::Null,
            &mut messages,
        );
        self.cache = Some(ui.into_cache());

        for message in messages {
            self.messages.push(message.clone());
            self.model.update(message);
        }
    }

    fn operate(&mut self, operation: &mut dyn operation::Operation) {
        let mut ui = UserInterface::build(
            self.model.view(),
            self.size,
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

    fn bounds(&mut self, id: &'static str) -> Rectangle {
        self.find(selector::id(id)).expect("widget exists").bounds()
    }

    fn focused(&mut self) -> Option<Rectangle> {
        self.find(selector::is_focused())
            .map(|target| target.bounds())
    }

    fn click_at(&mut self, point: Point) {
        self.cursor = mouse::Cursor::Available(point);
        self.send(simulator::click());
    }

    fn key(&mut self, key: Named) {
        self.send(simulator::tap_key(key, None));
    }
}

/// Finds the bounds of the first scrollable and of its content: the
/// option list of an open panel.
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

/// The height of an option row: one line of body medium text between
/// small paddings.
const ROW: f32 = 20.02 + 2.0 * spacing::SM;

impl<M: Model> Harness<M> {
    /// The bounds of the open option list and of its content.
    fn panel(&mut self) -> Option<(Rectangle, Rectangle)> {
        let mut panel = Panel::default();
        self.operate(&mut panel);
        panel.found
    }

    /// The number of options showing.
    fn option_count(&mut self) -> usize {
        self.panel().map_or(0, |(_, content)| {
            ((content.height - 2.0 * spacing::XS) / ROW).round() as usize
        })
    }

    /// The center of the option at `index`.
    fn option(&mut self, index: usize) -> Point {
        let (bounds, content) = self.panel().expect("the panel is open");

        Point::new(
            bounds.center_x(),
            content.y + spacing::XS + ROW * (index as f32 + 0.5),
        )
    }

    fn type_text(&mut self, text: &str) {
        self.send(simulator::typewrite(text));
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    City(String),
    Picked(&'static str),
}

struct App {
    cities: ListCollection<&'static str>,
    city: String,
    show_all: bool,
    own: bool,
}

impl Model for App {
    type Message = Message;

    fn view(&self) -> Element<'_, Message> {
        let mut field = autocomplete("City", &self.city, &self.cities)
            .id("city")
            .show_all_when_empty(self.show_all)
            .on_input(Message::City)
            .on_select(Message::Picked);

        if self.own {
            field = field.suggestions(self.cities.items().iter().take(1));
        }

        column![field].padding(16).width(332).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::City(city) => self.city = city,
            Message::Picked(_) => {}
        }
    }
}

fn cities() -> ListCollection<&'static str> {
    ListCollection::new([
        "Belo Horizonte",
        "Brasília",
        "São Paulo",
        "Salvador",
        "Recife",
    ])
    .expect("unique cities")
    .with_disabled(|city| *city == "Recife")
}

fn app(show_all: bool) -> Harness<App> {
    Harness::new(
        App {
            cities: cities(),
            city: String::new(),
            show_all,
            own: false,
        },
        Size::new(500.0, 500.0),
    )
}

/// Clicks the field to focus it.
fn focus(ui: &mut Harness<App>) -> Rectangle {
    let field = ui.bounds("city");
    ui.click_at(field.center());
    field
}

#[test]
fn suggests_matches_ignoring_accents_and_skips_disabled_items() {
    let mut ui = app(false);

    focus(&mut ui);
    assert_eq!(ui.option_count(), 0, "nothing shows while empty");

    ui.type_text("sa");
    assert_eq!(ui.model.city, "sa");
    assert_eq!(ui.option_count(), 2, "São Paulo and Salvador");

    ui.model.city = "re".to_owned();
    ui.send([]);
    assert_eq!(ui.option_count(), 0, "Recife is disabled");
}

#[test]
fn the_panel_opens_below_the_field_and_takes_its_width() {
    let mut ui = app(false);

    let field = focus(&mut ui);
    ui.type_text("b");
    let (bounds, _) = ui.panel().expect("the panel is open");

    // The list sits inside the one-pixel outline of the panel.
    assert_eq!(bounds.width, field.width - 2.0);
    assert_eq!(bounds.x, field.x + 1.0);
    assert!(bounds.y >= field.y + field.height);
}

#[test]
fn picks_with_the_arrows_and_enter_keeping_the_focus() {
    let mut ui = app(false);

    let field = focus(&mut ui);
    ui.type_text("b");
    assert_eq!(ui.option_count(), 2);

    ui.key(Named::ArrowDown);
    assert_eq!(ui.focused(), Some(field), "the field keeps the focus");

    ui.key(Named::Enter);
    assert_eq!(
        ui.messages[ui.messages.len() - 2..],
        [
            Message::City("Brasília".to_owned()),
            Message::Picked("Brasília")
        ]
    );
    assert_eq!(ui.model.city, "Brasília");
    assert_eq!(ui.option_count(), 0, "the panel closes after a pick");
    assert_eq!(ui.focused(), Some(field));
}

#[test]
fn the_arrows_wrap_around() {
    let mut ui = app(false);

    focus(&mut ui);
    ui.type_text("b");
    ui.key(Named::ArrowUp);
    ui.key(Named::Enter);

    assert_eq!(ui.messages.last(), Some(&Message::Picked("Brasília")));
}

#[test]
fn a_click_picks_an_option() {
    let mut ui = app(true);

    focus(&mut ui);
    assert_eq!(ui.option_count(), 4, "every enabled city shows while empty");

    let salvador = ui.option(3);
    ui.click_at(salvador);

    assert_eq!(ui.messages.last(), Some(&Message::Picked("Salvador")));
    assert_eq!(ui.model.city, "Salvador");
    assert_eq!(ui.option_count(), 0);
}

#[test]
fn escape_closes_the_panel_until_the_text_changes() {
    let mut ui = app(false);

    let field = focus(&mut ui);
    ui.type_text("sa");
    ui.key(Named::Escape);
    assert_eq!(ui.option_count(), 0);
    assert_eq!(ui.focused(), Some(field), "the field keeps the focus");

    ui.type_text("l");
    assert_eq!(ui.option_count(), 1);

    ui.key(Named::Escape);
    ui.key(Named::ArrowDown);
    assert_eq!(ui.option_count(), 1, "the arrows reopen it");
}

#[test]
fn a_click_outside_closes_the_panel() {
    let mut ui = app(false);

    focus(&mut ui);
    ui.type_text("sa");
    ui.click_at(Point::new(480.0, 480.0));

    assert_eq!(ui.option_count(), 0);
    assert_eq!(ui.focused(), None);
}

#[test]
fn the_app_can_pass_its_own_suggestions() {
    let mut ui = app(false);
    ui.model.own = true;

    focus(&mut ui);
    ui.type_text("x");
    assert_eq!(ui.option_count(), 1, "the suggestions show as given");
}

fn catalog<'a>(cities: &'a ListCollection<&'static str>) -> Element<'a, ()> {
    column![
        autocomplete("City", "sa", cities)
            .id("city")
            .on_input(|_| ()),
    ]
    .padding(spacing::LG)
    .width(280.0 + 2.0 * spacing::LG)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let cities = ListCollection::new(["Belo Horizonte", "Salvador", "São Paulo", "Santos"])
        .expect("unique cities");
    let mut ui =
        simulator::Simulator::with_size(support::settings(), (320.0, 240.0), catalog(&cities));
    let field = ui.find(selector::id("city")).expect("field").bounds();

    ui.point_at(field.center());
    let _ = ui.simulate(simulator::click());
    ui.point_at(Point::new(2.0, 2.0));

    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn autocomplete_light() {
    snapshot(&Theme::light(), "autocomplete_light");
}

#[test]
fn autocomplete_dark() {
    snapshot(&Theme::dark(), "autocomplete_dark");
}

#[test]
fn autocomplete_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "autocomplete_light_high_contrast");
}
