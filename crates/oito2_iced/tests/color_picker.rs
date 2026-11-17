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

//! Color pickers: the color conversions, the fields, the swatches, the
//! area and the sliders by pointer and keyboard, the hue kept for grays,
//! the disabled state, the picker button and golden snapshots. Run with
//! `ICED_TEST_BACKEND=tiny-skia`; refresh the goldens with
//! `OITO2_UPDATE_GOLDENS=1`.

use oito2_iced_test_utils as support;

use iced::advanced::clipboard;
use iced::advanced::renderer::Headless;
use iced::advanced::widget::operation;
use iced::keyboard::{self, key::Named};
use iced::widget::{column, container};
use iced::{Color, Event, Point, Rectangle, Size, mouse};
use iced_test::Selector;
use iced_test::runtime::user_interface::{Cache, UserInterface};
use iced_test::{selector, simulator};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;
use oito2_iced::widget::color_picker::{
    self, AREA_HEIGHT, ColorFormat, Hsl, Hsv, SLIDER_HEIGHT, from_hex, to_hex,
};
use oito2_iced::{Element, Theme};

const BLUE: Color = Color::from_rgb8(0x35, 0x84, 0xE4);
const RED: Color = Color::from_rgb8(0xE0, 0x1B, 0x24);
const GREEN: Color = Color::from_rgb8(0x2E, 0xC2, 0x7E);
const GRAY: Color = Color::from_rgb8(0x80, 0x80, 0x80);

/// The padding around the picker, so its parts sit at known positions.
const PAD: f32 = 8.0;

/// The width of the picker in the behaviour tests.
const WIDTH: f32 = 264.0;

/// The space between the parts of the picker.
const GAP: f32 = 12.0;

const SIZE: Size = Size::new(400.0, 600.0);

fn argb(color: Color) -> u32 {
    let [r, g, b, a] = color.into_rgba8();
    u32::from_be_bytes([a, r, g, b])
}

#[test]
fn hex_conversion() {
    assert_eq!(to_hex(BLUE, true), "#3584E4");
    let translucent = Color::from_rgba8(0x35, 0x84, 0xE4, 128.0 / 255.0);
    assert_eq!(to_hex(translucent, true), "#3584E480");
    assert_eq!(to_hex(translucent, false), "#3584E4");
    assert_eq!(from_hex("#3584e4"), Some(BLUE));
    assert_eq!(from_hex("f00"), Some(Color::from_rgb8(0xFF, 0, 0)));
    assert_eq!(from_hex(" #3584E480 "), Some(translucent));
    assert_eq!(from_hex("#12345"), None);
    assert_eq!(from_hex("zzz"), None);
    assert_eq!(from_hex("12#3456"), None);
    assert_eq!(from_hex("##123456"), None);
    assert_eq!(from_hex("#"), None);
}

#[test]
fn hsv_and_hsl_convert_like_flutter() {
    let hsv = Hsv::from_color(BLUE);
    assert!((hsv.hue - 212.91).abs() < 0.01, "{hsv:?}");
    assert!((hsv.saturation - 0.7675).abs() < 0.001, "{hsv:?}");
    assert!((hsv.value - 228.0 / 255.0).abs() < 1e-6, "{hsv:?}");
    assert_eq!(hsv.to_color(), BLUE);

    let hsl = Hsl::from_color(BLUE);
    assert!((hsl.saturation - 0.7642).abs() < 0.001, "{hsl:?}");
    assert!((hsl.lightness - 0.5510).abs() < 0.001, "{hsl:?}");
    assert_eq!(hsl.to_color(), BLUE);

    // Grays have no hue; black gets a full HSL saturation.
    assert_eq!(Hsv::from_color(GRAY).hue, 0.0);
    assert!(Hsv::from_color(GRAY).is_gray());
    assert_eq!(Hsl::from_color(Color::BLACK).saturation, 1.0);
    assert_eq!(Hsl::from_color(Color::WHITE).saturation, 0.0);

    // Every 8-bit color survives a round trip.
    for rgb in (0..0x0100_0000u32).step_by(0x010307) {
        let [_, r, g, b] = rgb.to_be_bytes();
        let color = Color::from_rgb8(r, g, b);
        assert_eq!(Hsv::from_color(color).to_color(), color);
        assert_eq!(Hsl::from_color(color).to_color(), color);
    }

    let half = Hsv {
        hue: 120.0,
        saturation: 0.5,
        value: 1.0,
        alpha: 0.99,
    }
    .to_color();
    assert_eq!(half.into_rgba8(), [0x80, 0xFF, 0x80, 252]);
}

/// An app driven like the runtime does: events update a user interface
/// built from the model, and the messages update the model before the next
/// events.
struct Harness {
    model: Model,
    cache: Option<Cache>,
    renderer: iced::Renderer,
    cursor: mouse::Cursor,
    messages: Vec<Message>,
}

#[derive(Debug, Clone, PartialEq)]
enum Message {
    Color(Color),
    Open(bool),
}

struct Model {
    color: Color,
    swatches: Vec<Color>,
    enabled: bool,
    button: bool,
    open: bool,
    localizations: Localizations,
}

impl Model {
    fn new(swatches: Vec<Color>) -> Self {
        Self {
            color: BLUE,
            swatches,
            enabled: true,
            button: false,
            open: false,
            localizations: Localizations::en(),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let content: Element<'_, Message> = if self.button {
            color_picker::color_picker_button(self.color)
                .open(self.open)
                .on_open_change(Message::Open)
                .on_change(Message::Color)
                .into()
        } else {
            color_picker::color_picker(self.color)
                .width(WIDTH)
                .swatches(self.swatches.clone())
                .localizations(&self.localizations)
                .on_change_maybe(self.enabled.then_some(Message::Color))
                .into()
        };

        oito2_iced::focus::navigation(container(content).padding(PAD)).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Color(color) => self.color = color,
            Message::Open(open) => self.open = open,
        }
    }
}

impl Harness {
    fn new(model: Model) -> Self {
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

        let mut harness = Self {
            model,
            cache: None,
            renderer,
            cursor: mouse::Cursor::Unavailable,
            messages: Vec::new(),
        };
        harness.send([]);
        harness
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
            &mut clipboard::Null,
            &mut messages,
        );
        self.cache = Some(ui.into_cache());

        for message in messages {
            self.messages.push(message.clone());
            self.model.update(message);
        }
    }

    fn find<S>(&mut self, selector: S) -> Option<S::Output>
    where
        S: Selector + Send,
        S::Output: Clone + Send,
    {
        let mut ui = UserInterface::build(
            self.model.view(),
            SIZE,
            self.cache.take().unwrap_or_default(),
            &mut self.renderer,
        );
        let mut find = selector.find();
        ui.operate(&self.renderer, &mut operation::black_box(&mut find));
        self.cache = Some(ui.into_cache());

        match operation::Operation::finish(&find) {
            operation::Outcome::Some(output) => output,
            _ => None,
        }
    }

    fn text(&mut self, text: &'static str) -> Option<Rectangle> {
        self.find(text).map(|target| target.bounds())
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

    fn shift_key(&mut self, key: Named) {
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::SHIFT,
            )),
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(key),
                modified_key: keyboard::Key::Named(key),
                physical_key: keyboard::key::Physical::Unidentified(
                    keyboard::key::NativeCode::Unidentified,
                ),
                location: keyboard::Location::Standard,
                modifiers: keyboard::Modifiers::SHIFT,
                repeat: false,
                text: None,
            }),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
    }

    /// Clicks the field at `bounds`, selects its text and types `text`.
    fn retype(&mut self, bounds: Rectangle, text: &str) {
        self.click_at(bounds.center());
        self.send([
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::COMMAND,
            )),
            simulator::press_key(keyboard::Key::Character("a".into()), None),
            Event::Keyboard(keyboard::Event::ModifiersChanged(
                keyboard::Modifiers::empty(),
            )),
        ]);
        self.send(simulator::typewrite(text));
    }

    fn hsv(&self) -> Hsv {
        Hsv::from_color(self.model.color)
    }
}

/// The bounds of the saturation and brightness area.
fn area() -> Rectangle {
    Rectangle::new(Point::new(PAD, PAD), Size::new(WIDTH, AREA_HEIGHT))
}

/// The bounds of the hue slider.
fn hue_slider() -> Rectangle {
    Rectangle::new(
        Point::new(PAD, PAD + AREA_HEIGHT + GAP),
        Size::new(WIDTH, SLIDER_HEIGHT),
    )
}

/// The bounds of the opacity slider.
fn alpha_slider() -> Rectangle {
    Rectangle::new(
        Point::new(PAD, PAD + AREA_HEIGHT + 2.0 * GAP + SLIDER_HEIGHT),
        Size::new(WIDTH, SLIDER_HEIGHT),
    )
}

#[test]
fn the_hex_field_the_rgb_fields_and_swatches_set_it() {
    let mut ui = Harness::new(Model::new(vec![RED, GREEN]));

    let hex = ui.text("#3584E4").expect("hex field");
    ui.retype(hex, "#ff7800");
    ui.key(Named::Enter);
    assert_eq!(argb(ui.model.color), 0xFFFF7800);
    assert!(ui.text("#FF7800").is_some());

    let rgb = ui.text("RGB").expect("format segment");
    ui.click_at(rgb.center());
    let blue = ui.text("0").expect("blue field");
    ui.retype(blue, "255");
    ui.key(Named::Enter);
    assert_eq!(argb(ui.model.color), 0xFFFF78FF);

    let swatch_y = ui.text("RGB").expect("segment").y + 48.0;
    // The second swatch: a 28-pixel disk after the first and a 2-pixel gap.
    ui.click_at(Point::new(PAD + 28.0 + 2.0 + 14.0, swatch_y));
    assert_eq!(argb(ui.model.color), 0xFF2EC27E);
}

#[test]
fn invalid_text_shows_the_color_again() {
    let mut ui = Harness::new(Model::new(Vec::new()));

    let hex = ui.text("#3584E4").expect("hex field");
    ui.retype(hex, "#12345");
    ui.key(Named::Enter);

    assert!(ui.messages.is_empty());
    assert!(ui.text("#3584E4").is_some());
}

#[test]
fn letters_do_not_reach_the_numeric_fields() {
    let mut ui = Harness::new(Model::new(Vec::new()));

    let rgb = ui.text("RGB").expect("format segment");
    ui.click_at(rgb.center());
    let red = ui.text("53").expect("red field");
    ui.retype(red, "x");

    assert!(ui.text("53").is_some(), "the letter is refused");
}

#[test]
fn grays_keep_the_hue_also_typed_in_hsl() {
    let mut ui = Harness::new(Model::new(vec![GRAY]));
    let before = ui.hsv().hue;

    let swatch_y = ui.text("HEX").expect("segment").y + 48.0;
    ui.click_at(Point::new(PAD + 14.0, swatch_y));
    assert_eq!(argb(ui.model.color), 0xFF808080);

    let hsl = ui.text("HSL").expect("segment");
    ui.click_at(hsl.center());
    // The hue field still shows the hue of the blue.
    let hue = ui.text("213").expect("hue field keeps the hue");
    assert!((before - 213.0).abs() < 1.0);

    ui.retype(hue, "120");
    ui.key(Named::Enter);
    let saturation = ui.text("0").expect("saturation field");
    ui.retype(saturation, "50");
    ui.key(Named::Enter);

    assert!((ui.hsv().hue - 120.0).abs() < 1.0, "{:?}", ui.hsv());
}

#[test]
fn typed_text_applies_when_the_format_changes() {
    let mut ui = Harness::new(Model::new(Vec::new()));

    let rgb = ui.text("RGB").expect("segment");
    ui.click_at(rgb.center());
    let red = ui.text("53").expect("red field");
    ui.retype(red, "0");
    let hsl = ui.text("HSL").expect("segment");
    ui.click_at(hsl.center());

    assert_eq!(argb(ui.model.color), 0xFF0084E4);
}

#[test]
fn typed_text_applies_when_the_focus_leaves_the_field() {
    let mut ui = Harness::new(Model::new(Vec::new()));

    let hex = ui.text("#3584E4").expect("hex field");
    ui.retype(hex, "#2ec27e");
    assert_eq!(ui.model.color, BLUE, "nothing applies while typing");

    ui.key(Named::Tab);
    assert_eq!(argb(ui.model.color), 0xFF2EC27E);
}

#[test]
fn the_area_and_the_sliders_move_by_keyboard() {
    let mut ui = Harness::new(Model::new(Vec::new()));
    let start = ui.hsv();

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(area()));

    ui.key(Named::ArrowRight);
    let hsv = ui.hsv();
    assert!(hsv.saturation > start.saturation);
    assert!((hsv.value - start.value).abs() < 0.005);

    let before = ui.hsv().value;
    ui.shift_key(Named::ArrowDown);
    assert!(
        (ui.hsv().value - (before - 0.1)).abs() < 0.005,
        "{:?}",
        ui.hsv()
    );

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(hue_slider()));
    let hue = ui.hsv().hue;
    ui.key(Named::ArrowRight);
    assert!((ui.hsv().hue - (hue + 3.6)).abs() < 1.0, "{:?}", ui.hsv());

    ui.key(Named::Tab);
    assert_eq!(ui.focused(), Some(alpha_slider()));
    ui.key(Named::ArrowLeft);
    assert!((ui.model.color.a - 0.99).abs() < 0.005);
    ui.shift_key(Named::ArrowLeft);
    assert!((ui.model.color.a - 0.89).abs() < 0.005);
    ui.key(Named::ArrowUp);
    assert!((ui.model.color.a - 0.90).abs() < 0.005);
}

#[test]
fn the_pointer_picks_on_the_area_and_the_sliders() {
    let mut ui = Harness::new(Model::new(Vec::new()));

    let area = area();
    ui.click_at(Point::new(
        area.x + area.width * 0.25,
        area.y + area.height * 0.75,
    ));
    let hsv = ui.hsv();
    assert!((hsv.saturation - 0.25).abs() < 0.01, "{hsv:?}");
    assert!((hsv.value - 0.25).abs() < 0.01, "{hsv:?}");

    let hue = hue_slider();
    ui.click_at(Point::new(
        hue.x + 9.0 + (hue.width - 18.0) / 3.0,
        hue.center_y(),
    ));
    assert!((ui.hsv().hue - 120.0).abs() < 1.5, "{:?}", ui.hsv());

    let alpha = alpha_slider();
    ui.click_at(Point::new(alpha.x + 9.0, alpha.center_y()));
    assert_eq!(ui.model.color.a, 0.0);
}

#[test]
fn a_picker_without_a_message_is_disabled() {
    let mut model = Model::new(vec![RED]);
    model.enabled = false;
    let mut ui = Harness::new(model);

    ui.click_at(area().center());
    ui.click_at(hue_slider().center());
    ui.key(Named::Tab);

    assert!(ui.messages.is_empty());
    assert_eq!(ui.focused(), None);
}

#[test]
fn the_hex_prefix_is_localized() {
    let mut model = Model::new(Vec::new());
    model.localizations = Localizations::pt_br();
    let mut ui = Harness::new(model);

    assert!(ui.text(Localizations::pt_br().hex_color).is_some());
    assert!(ui.text("HEX").is_some());
}

#[test]
fn the_button_opens_the_picker_in_a_popover() {
    let mut model = Model::new(Vec::new());
    model.button = true;
    let mut ui = Harness::new(model);

    assert!(ui.text("HEX").is_none());
    let button = ui.text("#3584E4").expect("button label");
    ui.click_at(button.center());
    assert_eq!(ui.messages, vec![Message::Open(true)]);
    assert!(ui.text("HEX").is_some(), "the picker shows");

    ui.key(Named::Escape);
    assert_eq!(ui.messages.last(), Some(&Message::Open(false)));
    assert!(ui.text("HEX").is_none());
}

fn catalog() -> Element<'static, Message> {
    column![
        color_picker::color_picker(Color::from_rgba8(0x35, 0x84, 0xE4, 0.8))
            .width(220.0)
            .swatches([RED, Color::from_rgb8(0xFF, 0x78, 0x00), GREEN, BLUE])
            .on_change(Message::Color),
        iced::widget::row![
            color_picker::color_picker_button(BLUE)
                .on_open_change(Message::Open)
                .on_change(Message::Color),
            color_picker::color_picker_button(Color::from_rgba8(0xE0, 0x1B, 0x24, 0.5))
                .on_open_change(Message::Open)
                .on_change(Message::Color),
        ]
        .spacing(8),
        color_picker::color_picker(GRAY)
            .width(220.0)
            .show_alpha(false)
            .initial_format(ColorFormat::Rgb),
    ]
    .spacing(16)
    .padding(16)
    .into()
}

fn snapshot(theme: &Theme, name: &str) {
    let mut ui = simulator::Simulator::with_size(support::settings(), (300.0, 760.0), catalog());
    let snapshot = ui.snapshot(theme).expect("snapshot");

    support::golden!(&snapshot, name);
}

#[test]
fn color_picker_light() {
    snapshot(&Theme::light(), "color_picker_light");
}

#[test]
fn color_picker_dark() {
    snapshot(&Theme::dark(), "color_picker_dark");
}

#[test]
fn color_picker_light_high_contrast() {
    let theme = Theme::base(Brightness::Light, Accent::default().color(), Contrast::High);

    snapshot(&theme, "color_picker_light_high_contrast");
}
