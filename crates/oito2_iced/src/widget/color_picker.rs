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

//! Color pickers: a saturation and brightness area, a hue slider, an
//! optional opacity slider, text fields in a chosen [`ColorFormat`] and
//! optional swatches, and a button that opens one in a popover.
//!
//! Every part works by pointer and by keyboard: while focused, the arrow
//! keys move the area and the sliders by 1 %, or 10 % with Shift (on the
//! sliders, only the arrows of the reading direction take Shift; Up and
//! Down move them by 1 %). The fields apply their text on Enter or when the
//! keyboard focus leaves them, and show the current color again when the
//! text is not valid.
//!
//! The color lives in the application: the picker shows `value` and
//! publishes each new color through [`ColorPicker::on_change`]; without it
//! the picker is disabled. The picker keeps the color as HSV, so the hue
//! survives grays, which have none of their own. The field format and the
//! text being typed live in the picker.
//!
//! [`to_hex`] and [`from_hex`] convert colors to and from `#RRGGBB` text;
//! [`Hsv`] and [`Hsl`] convert them to and from hue-based models.
//!
//! ```no_run
//! use iced::Color;
//! use oito2_iced::widget::color_picker;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Accent(Color),
//!     PickerOpen(bool),
//! }
//!
//! # fn view<'a>(accent: Color, open: bool) -> oito2_iced::Element<'a, Message> {
//! iced::widget::column![
//!     color_picker::color_picker(accent)
//!         .swatches([Color::from_rgb8(0xE0, 0x1B, 0x24), Color::from_rgb8(0x2E, 0xC2, 0x7E)])
//!         .on_change(Message::Accent),
//!     color_picker::color_picker_button(accent)
//!         .open(open)
//!         .on_open_change(Message::PickerOpen)
//!         .on_change(Message::Accent),
//! ]
//! .into()
//! # }
//! ```

use iced::advanced::graphics::geometry::{self, Frame, Path};
use iced::advanced::graphics::gradient::Linear;
use iced::advanced::layout::{self, Layout};
use iced::advanced::text::Paragraph as _;
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::widget::text::LineHeight;
use iced::widget::{Column, Row, Stack, container, text_input};
use iced::{
    Alignment, Border, Color, Element as IcedElement, Event, Length, Padding, Pixels, Point,
    Rectangle, Size, Vector, mouse, touch,
};

use super::controls::{color_disk, exclude_focus};
use super::helpers::ReadingDirection;
use super::search::host::{Effects, Host, Msg, Reconcile, Signal};
use super::segment_group::{segment, segment_group};
use super::{button, popover, styled_text};
use crate::l10n::Localizations;
use crate::theme::text::Class as TextClass;
use crate::tokens::{radius, spacing, typography};
use crate::{Element, Theme, density};

/// The default width of a [`ColorPicker`].
pub const DEFAULT_WIDTH: f32 = 264.0;

/// The height of the saturation and brightness area.
pub const AREA_HEIGHT: f32 = 150.0;

/// The height of a channel slider, thumb included.
pub const SLIDER_HEIGHT: f32 = 18.0;

/// The radius of the thumb ring of a channel slider, which is also the
/// inset of its track ends.
const THUMB_RADIUS: f32 = 9.0;

/// The radius of the white ring of a thumb.
const THUMB_RING_RADIUS: f32 = 8.0;

/// The radius of the color dot of a thumb.
const THUMB_DOT_RADIUS: f32 = 6.0;

/// The dark outline of a thumb, so it shows on any background.
const THUMB_SHADOW: Color = Color::from_rgba8(0, 0, 0, 0.4);

/// The space above and below the track of a channel slider.
const TRACK_INSET: f32 = 3.0;

/// The corner radius of the track of a channel slider.
const TRACK_RADIUS: f32 = 6.0;

/// The side of the color preview next to the fields.
const PREVIEW_SIZE: f32 = 32.0;

/// The side of the swatch of a [`ColorPickerButton`].
const BUTTON_SWATCH_SIZE: f32 = 18.0;

/// The diameter of a preset swatch.
const SWATCH_SIZE: f32 = 20.0;

/// The side of a checkerboard square.
const CHECKER_CELL: f32 = 6.0;

/// The gray squares of the checkerboard.
const CHECKER_GRAY: Color = Color::from_rgb8(0xCC, 0xCC, 0xCC);

/// The change of an arrow key.
const KEY_STEP: f64 = 0.01;

/// The change of an arrow key with Shift.
const SHIFT_STEP: f64 = 0.1;

/// The text format of the fields of a [`ColorPicker`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ColorFormat {
    /// One hexadecimal field, such as `#3584E4`.
    #[default]
    Hex,
    /// Red, green and blue fields from 0 to 255.
    Rgb,
    /// Hue, saturation and lightness fields.
    Hsl,
}

impl ColorFormat {
    /// Every format, in the order of the format selector.
    pub const ALL: [ColorFormat; 3] = [ColorFormat::Hex, ColorFormat::Rgb, ColorFormat::Hsl];

    /// The name of the format in the format selector.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            ColorFormat::Hex => "HEX",
            ColorFormat::Rgb => "RGB",
            ColorFormat::Hsl => "HSL",
        }
    }
}

/// Rounds a channel from 0 to 1 to 8 bits.
fn byte(channel: f64) -> u8 {
    (channel * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Returns `color` formatted as `#RRGGBB`, or `#RRGGBBAA` when it is
/// translucent and `with_alpha` is set.
#[must_use]
pub fn to_hex(color: Color, with_alpha: bool) -> String {
    let part = |channel: f32| byte(f64::from(channel));
    let hex = format!(
        "#{:02X}{:02X}{:02X}",
        part(color.r),
        part(color.g),
        part(color.b)
    );

    if with_alpha && color.a < 1.0 {
        format!("{hex}{:02X}", part(color.a))
    } else {
        hex
    }
}

/// Parses `text` as `#RGB`, `#RRGGBB` or `#RRGGBBAA`, with or without `#`
/// and surrounding white space.
///
/// Returns `None` when `text` is not one of these forms.
#[must_use]
pub fn from_hex(text: &str) -> Option<Color> {
    let text = text.trim();
    let hex = text.strip_prefix('#').unwrap_or(text);

    if hex.is_empty() || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
        return None;
    }

    let hex = match hex.len() {
        3 => hex
            .chars()
            .flat_map(|c| [c, c])
            .chain("FF".chars())
            .collect(),
        6 => format!("{hex}FF"),
        8 => hex.to_owned(),
        _ => return None,
    };

    let rgba = u32::from_str_radix(&hex, 16).ok()?;
    let [r, g, b, a] = rgba.to_be_bytes();

    Some(Color::from_rgba8(r, g, b, f32::from(a) / 255.0))
}

/// A color as hue (degrees, 0 to 360), saturation, value (brightness) and
/// alpha (0 to 1).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hsv {
    /// The hue, in degrees from 0 to 360.
    pub hue: f64,
    /// The saturation, from 0 to 1.
    pub saturation: f64,
    /// The value (brightness), from 0 to 1.
    pub value: f64,
    /// The opacity, from 0 to 1.
    pub alpha: f64,
}

/// A color as hue (degrees, 0 to 360), saturation, lightness and alpha
/// (0 to 1).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hsl {
    /// The hue, in degrees from 0 to 360.
    pub hue: f64,
    /// The saturation, from 0 to 1.
    pub saturation: f64,
    /// The lightness, from 0 to 1.
    pub lightness: f64,
    /// The opacity, from 0 to 1.
    pub alpha: f64,
}

/// The red, green, blue and alpha channels of `color`, and the largest and
/// the smallest of the color channels.
fn channels(color: Color) -> (f64, f64, f64, f64, f64, f64) {
    let (r, g, b) = (f64::from(color.r), f64::from(color.g), f64::from(color.b));

    (
        r,
        g,
        b,
        f64::from(color.a),
        r.max(g).max(b),
        r.min(g).min(b),
    )
}

/// The hue of the channels `r`, `g` and `b`, whose largest is `max` and
/// whose spread is `delta`; 0 for grays.
fn hue_of(r: f64, g: f64, b: f64, max: f64, delta: f64) -> f64 {
    let hue = if max == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    if hue.is_nan() { 0.0 } else { hue }
}

/// The 8-bit color of `hue` with the given chroma, secondary component and
/// lightness offset.
fn color_from_hue(alpha: f64, hue: f64, chroma: f64, secondary: f64, offset: f64) -> Color {
    let (r, g, b) = if hue < 60.0 {
        (chroma, secondary, 0.0)
    } else if hue < 120.0 {
        (secondary, chroma, 0.0)
    } else if hue < 180.0 {
        (0.0, chroma, secondary)
    } else if hue < 240.0 {
        (0.0, secondary, chroma)
    } else if hue < 300.0 {
        (secondary, 0.0, chroma)
    } else {
        (chroma, 0.0, secondary)
    };

    Color::from_rgba8(
        byte(r + offset),
        byte(g + offset),
        byte(b + offset),
        f32::from(byte(alpha)) / 255.0,
    )
}

/// The secondary component of `chroma` at `hue`.
fn secondary(hue: f64, chroma: f64) -> f64 {
    chroma * (1.0 - ((hue / 60.0).rem_euclid(2.0) - 1.0).abs())
}

impl Hsv {
    /// The HSV form of `color`; grays get the hue 0.
    #[must_use]
    pub fn from_color(color: Color) -> Self {
        let (r, g, b, alpha, max, min) = channels(color);
        let delta = max - min;

        Self {
            hue: hue_of(r, g, b, max, delta),
            saturation: if max == 0.0 { 0.0 } else { delta / max },
            value: max,
            alpha,
        }
    }

    /// The color, rounded to 8 bits per channel.
    #[must_use]
    pub fn to_color(self) -> Color {
        let chroma = self.saturation * self.value;

        color_from_hue(
            self.alpha,
            self.hue,
            chroma,
            secondary(self.hue, chroma),
            self.value - chroma,
        )
    }

    /// Whether the color has no hue of its own: no saturation or no
    /// brightness.
    #[must_use]
    pub fn is_gray(self) -> bool {
        self.saturation == 0.0 || self.value == 0.0
    }
}

impl Hsl {
    /// The HSL form of `color`; grays get the hue 0, and black the
    /// saturation 1.
    #[must_use]
    pub fn from_color(color: Color) -> Self {
        let (r, g, b, alpha, max, min) = channels(color);
        let delta = max - min;
        let lightness = (max + min) / 2.0;
        let saturation = if lightness == 1.0 {
            0.0
        } else {
            let saturation = delta / (1.0 - (2.0 * lightness - 1.0).abs());

            if saturation.is_nan() {
                1.0
            } else {
                saturation.clamp(0.0, 1.0)
            }
        };

        Self {
            hue: hue_of(r, g, b, max, delta),
            saturation,
            lightness,
            alpha,
        }
    }

    /// The color, rounded to 8 bits per channel.
    #[must_use]
    pub fn to_color(self) -> Color {
        let chroma = (1.0 - (2.0 * self.lightness - 1.0).abs()) * self.saturation;

        color_from_hue(
            self.alpha,
            self.hue,
            chroma,
            secondary(self.hue, chroma),
            self.lightness - chroma / 2.0,
        )
    }
}

/// A color picker. See [`color_picker`].
pub struct ColorPicker<'a, Message> {
    value: Color,
    on_change: Option<Box<dyn Fn(Color) -> Message + 'a>>,
    show_alpha: bool,
    swatches: Vec<Color>,
    initial_format: ColorFormat,
    width: f32,
    localizations: Localizations,
    direction: ReadingDirection,
}

/// A color picker showing `value`. Without [`ColorPicker::on_change`] it
/// is disabled.
pub fn color_picker<'a, Message>(value: Color) -> ColorPicker<'a, Message> {
    ColorPicker {
        value,
        on_change: None,
        show_alpha: true,
        swatches: Vec::new(),
        initial_format: ColorFormat::Hex,
        width: DEFAULT_WIDTH,
        localizations: Localizations::en(),
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> ColorPicker<'a, Message> {
    /// Sets the function that turns each new color into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the change function, or disables the picker with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(Color) -> Message + 'a>) -> Self {
        self.on_change = on_change.map(|f| Box::new(f) as Box<dyn Fn(Color) -> Message + 'a>);
        self
    }

    /// Sets whether the opacity slider shows and the hex field includes
    /// the alpha channel (true by default); without it, edits keep the
    /// current opacity.
    #[must_use]
    pub fn show_alpha(mut self, show_alpha: bool) -> Self {
        self.show_alpha = show_alpha;
        self
    }

    /// Sets the preset colors offered below the fields.
    #[must_use]
    pub fn swatches(mut self, swatches: impl IntoIterator<Item = Color>) -> Self {
        self.swatches = swatches.into_iter().collect();
        self
    }

    /// Sets the field format shown first ([`ColorFormat::Hex`] by default).
    #[must_use]
    pub fn initial_format(mut self, format: ColorFormat) -> Self {
        self.initial_format = format;
        self
    }

    /// Sets the picker width ([`DEFAULT_WIDTH`] by default).
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Takes the prefix of the hex field from `localizations` (English by
    /// default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the reading direction, which runs the sliders from the right
    /// and swaps their Left and Right arrows when right to left.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

/// A text field of the picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Hex,
    Red,
    Green,
    Blue,
    Hue,
    Saturation,
    Lightness,
}

impl Field {
    /// Every field, in the order of [`PickerState::pending`].
    const ALL: [Field; 7] = [
        Field::Hex,
        Field::Red,
        Field::Green,
        Field::Blue,
        Field::Hue,
        Field::Saturation,
        Field::Lightness,
    ];

    fn index(self) -> usize {
        self as usize
    }

    /// Whether the field takes only digits.
    fn numeric(self) -> bool {
        self != Field::Hex
    }
}

/// An event of the composition of a picker.
#[derive(Debug, Clone)]
enum Own {
    /// The area picked a saturation and a brightness.
    Area(f64, f64),
    /// The hue slider moved to a position from 0 to 1.
    Hue(f64),
    /// The opacity slider moved to a value from 0 to 1.
    Alpha(f64),
    /// A field was edited.
    Input(Field, String),
    /// Enter was pressed in a field.
    Submit(Field),
    /// Another field format was chosen.
    Format(ColorFormat),
    /// A swatch was chosen.
    Swatch(Color),
}

/// The state of a picker in the widget tree.
#[derive(Debug, Default)]
struct PickerState {
    /// The shown color.
    hsv: Hsv,
    /// The chosen field format, once known.
    format: Option<ColorFormat>,
    /// The last color the application showed.
    value: Option<Color>,
    /// The text typed in each field and not applied yet.
    pending: [Option<String>; 7],
}

impl PickerState {
    /// Shows `color`, keeping the current hue, or `hue` when given, for
    /// grays.
    fn set_color(&mut self, color: Color, hue: Option<f64>) {
        let next = Hsv::from_color(color);

        self.hsv = if next.is_gray() {
            Hsv {
                hue: hue.unwrap_or(self.hsv.hue),
                ..next
            }
        } else {
            next
        };
    }

    /// The text `field` shows for the current color.
    fn text(&self, field: Field, show_alpha: bool) -> String {
        let color = self.hsv.to_color();
        let hsl = || Hsl::from_color(color);
        let percent = |value: f64| format!("{}", (value * 100.0).round());

        match field {
            Field::Hex => to_hex(color, show_alpha),
            Field::Red => byte(f64::from(color.r)).to_string(),
            Field::Green => byte(f64::from(color.g)).to_string(),
            Field::Blue => byte(f64::from(color.b)).to_string(),
            Field::Hue => format!("{}", self.hsv.hue.round()),
            Field::Saturation => percent(hsl().saturation),
            Field::Lightness => percent(hsl().lightness),
        }
    }

    /// Applies the text typed in `field`, if it differs from the shown
    /// text, and returns the new color. Text that is not valid is dropped,
    /// so the field shows the current color again.
    fn commit(&mut self, field: Field, show_alpha: bool) -> Option<Color> {
        let text = self.pending[field.index()].take()?;

        if text == self.text(field, show_alpha) {
            return None;
        }

        let color = self.hsv.to_color();
        let number = || text.parse::<i64>().ok();

        let (next, hue) = match field {
            Field::Hex => {
                let parsed = from_hex(&text)?;
                let next = if show_alpha {
                    parsed
                } else {
                    Color {
                        a: color.a,
                        ..parsed
                    }
                };

                (next, None)
            }
            Field::Red | Field::Green | Field::Blue => {
                let channel = number()?.clamp(0, 255) as f32 / 255.0;
                let mut next = color;

                match field {
                    Field::Red => next.r = channel,
                    Field::Green => next.g = channel,
                    _ => next.b = channel,
                }

                (next, None)
            }
            Field::Hue | Field::Saturation | Field::Lightness => {
                let max = if field == Field::Hue { 360 } else { 100 };
                let value = number()?.clamp(0, max) as f64;
                let hsl = Hsl::from_color(color);
                let hue = self.hsv.hue;

                match field {
                    Field::Hue => (Hsl { hue: value, ..hsl }.to_color(), Some(value)),
                    Field::Saturation => (
                        Hsl {
                            hue,
                            saturation: value / 100.0,
                            ..hsl
                        }
                        .to_color(),
                        Some(hue),
                    ),
                    _ => (
                        Hsl {
                            hue,
                            lightness: value / 100.0,
                            ..hsl
                        }
                        .to_color(),
                        Some(hue),
                    ),
                }
            }
        };

        self.set_color(next, hue);

        Some(self.hsv.to_color())
    }

    /// Applies the text typed in every field but `except`, and returns
    /// the colors each one gave.
    fn commit_all(&mut self, except: Option<Field>, show_alpha: bool) -> Vec<Color> {
        Field::ALL
            .into_iter()
            .filter(|field| Some(*field) != except)
            .filter_map(|field| self.commit(field, show_alpha))
            .collect()
    }
}

/// What the composition of a picker shows besides its state.
#[derive(Debug, Clone)]
struct Props {
    show_alpha: bool,
    swatches: Vec<Color>,
    initial_format: ColorFormat,
    width: f32,
    localizations: Localizations,
    direction: ReadingDirection,
    enabled: bool,
}

type Composed<'a, Message> = Element<'a, Msg<Message, Own>>;

impl<'a, Message: Clone + 'a> From<ColorPicker<'a, Message>> for Element<'a, Message> {
    fn from(picker: ColorPicker<'a, Message>) -> Self {
        let props = Props {
            show_alpha: picker.show_alpha,
            swatches: picker.swatches,
            initial_format: picker.initial_format,
            width: picker.width,
            localizations: picker.localizations,
            direction: picker.direction,
            enabled: picker.on_change.is_some(),
        };
        let value = picker.value;
        let initial_format = props.initial_format;
        let show_alpha = props.show_alpha;
        let on_change = picker.on_change;

        let handler = move |state: &mut PickerState,
                            signal: Signal<'_, Own>,
                            effects: &mut Effects<Message>| {
            let colors = match signal {
                Signal::Own(own) => handle(state, own, show_alpha),
                Signal::Focus(false) => state.commit_all(None, show_alpha),
                Signal::Focus(true) | Signal::Before(..) | Signal::After(..) => return,
            };

            if let Some(on_change) = &on_change {
                for color in colors {
                    effects.publish(on_change(color));
                }
            }

            effects.rebuild();
        };

        let reconcile = move |state: &mut PickerState, _input_focused: bool| {
            let mut rebuild = false;

            if state.format.is_none() {
                state.format = Some(initial_format);
                rebuild = true;
            }

            if state.value != Some(value) {
                if state.value.is_none() {
                    state.hsv = Hsv::from_color(value);
                } else if value != state.hsv.to_color() {
                    state.set_color(value, None);
                }

                state.value = Some(value);
                rebuild = true;
            }

            Reconcile {
                rebuild,
                focus: None,
            }
        };

        Host::new(move |state: &PickerState| compose(state, &props), handler)
            .reconcile(reconcile)
            .into()
    }
}

/// Applies the event `own` to the state and returns the colors to publish.
fn handle(state: &mut PickerState, own: Own, show_alpha: bool) -> Vec<Color> {
    match own {
        Own::Area(saturation, value) => {
            state.hsv.saturation = saturation;
            state.hsv.value = value;
        }
        Own::Hue(position) => state.hsv.hue = (position * 360.0).clamp(0.0, 360.0),
        Own::Alpha(alpha) => state.hsv.alpha = alpha,
        Own::Input(field, text) => {
            if field.numeric() && !text.chars().all(|c| c.is_ascii_digit()) {
                return Vec::new();
            }

            let colors = state.commit_all(Some(field), show_alpha);
            state.pending[field.index()] = Some(text);

            return colors;
        }
        Own::Submit(field) => return state.commit(field, show_alpha).into_iter().collect(),
        Own::Format(format) => {
            let colors = state.commit_all(None, show_alpha);
            state.format = Some(format);

            return colors;
        }
        Own::Swatch(color) => state.set_color(color, None),
    }

    vec![state.hsv.to_color()]
}

/// Builds the parts of a picker from its state.
fn compose<'a, Message: Clone + 'a>(state: &PickerState, props: &Props) -> Composed<'a, Message> {
    let hsv = state.hsv;
    let color = hsv.to_color();
    let opaque = Color { a: 1.0, ..color };
    let enabled = props.enabled;
    let format = state.format.unwrap_or(props.initial_format);
    let pure = |hue: f64| {
        Hsv {
            hue,
            saturation: 1.0,
            value: 1.0,
            alpha: 1.0,
        }
        .to_color()
    };

    let mut column = Column::new()
        .spacing(spacing::MD)
        .width(Length::Fixed(props.width));

    column = column.push(IcedElement::new(Area {
        hsv,
        on_change: enabled.then(|| {
            Box::new(|saturation, value| Msg::Own(Own::Area(saturation, value)))
                as Box<dyn Fn(f64, f64) -> Msg<Message, Own> + 'a>
        }),
    }));

    column = column.push(IcedElement::new(ChannelSlider {
        value: hsv.hue / 360.0,
        colors: (0..=6).map(|step| pure(f64::from(step) * 60.0)).collect(),
        thumb: pure(hsv.hue),
        checkerboard: false,
        direction: props.direction,
        on_change: enabled.then(|| {
            Box::new(|position| Msg::Own(Own::Hue(position)))
                as Box<dyn Fn(f64) -> Msg<Message, Own> + 'a>
        }),
    }));

    if props.show_alpha {
        column = column.push(IcedElement::new(ChannelSlider {
            value: hsv.alpha,
            colors: vec![Color { a: 0.0, ..opaque }, opaque],
            thumb: color,
            checkerboard: true,
            direction: props.direction,
            on_change: enabled.then(|| {
                Box::new(|alpha| Msg::Own(Own::Alpha(alpha)))
                    as Box<dyn Fn(f64) -> Msg<Message, Own> + 'a>
            }),
        }));
    }

    column = column.push(
        Row::new()
            .push(swatch(color, PREVIEW_SIZE, radius::SMALL))
            .push(fields(state, format, props))
            .spacing(spacing::SM)
            .align_y(Alignment::Center),
    );

    column = column.push(
        segment_group(
            ColorFormat::ALL.map(|format| segment(format, format.label())),
            Some(format),
        )
        .on_change_maybe(enabled.then_some(|format| Msg::Own(Own::Format(format)))),
    );

    if !props.swatches.is_empty() {
        let disks = props.swatches.iter().map(|&swatch| {
            color_disk(swatch, to_hex(swatch, true))
                .size(SWATCH_SIZE)
                .selected(swatch.into_rgba8() == color.into_rgba8())
                .on_press_maybe(enabled.then_some(Msg::Own(Own::Swatch(swatch))))
                .into()
        });

        column = column.push(
            Row::with_children(disks)
                .spacing(spacing::XXS)
                .wrap()
                .vertical_spacing(spacing::XXS),
        );
    }

    column.into()
}

/// The fields of the picker in `format`.
fn fields<'a, Message: Clone + 'a>(
    state: &PickerState,
    format: ColorFormat,
    props: &Props,
) -> Composed<'a, Message> {
    let field = |field: Field, label: &str| text_field(state, field, label, props);

    match format {
        ColorFormat::Hex => field(Field::Hex, props.localizations.hex_color),
        ColorFormat::Rgb => Row::new()
            .push(field(Field::Red, "R"))
            .push(field(Field::Green, "G"))
            .push(field(Field::Blue, "B"))
            .spacing(spacing::XS)
            .into(),
        ColorFormat::Hsl => Row::new()
            .push(field(Field::Hue, "H"))
            .push(field(Field::Saturation, "S"))
            .push(field(Field::Lightness, "L"))
            .spacing(spacing::XS)
            .into(),
    }
}

/// The width of `content` in the text style `style`.
fn text_width<Renderer>(content: &str, style: typography::TextStyle) -> f32
where
    Renderer: iced::advanced::text::Renderer<Font = iced::Font>,
{
    use iced::advanced::text;

    Renderer::Paragraph::with_text(text::Text {
        content,
        bounds: Size::INFINITE,
        size: Pixels(style.size),
        line_height: LineHeight::Absolute(Pixels(style.line_height)),
        font: crate::font::inter(style.weight),
        align_x: text::Alignment::Default,
        align_y: iced::alignment::Vertical::Top,
        shaping: text::Shaping::Advanced,
        wrapping: text::Wrapping::None,
    })
    .min_bounds()
    .width
    .ceil()
}

/// A field of the picker with `label` before its text, which applies on
/// Enter and when the focus leaves it.
fn text_field<'a, Message: Clone + 'a>(
    state: &PickerState,
    field: Field,
    label: &str,
    props: &Props,
) -> Composed<'a, Message> {
    let style = typography::BODY_LARGE;
    let height = density::control_height();
    let vertical = ((height - style.line_height) / 2.0).max(0.0);
    let text = state.pending[field.index()]
        .clone()
        .unwrap_or_else(|| state.text(field, props.show_alpha));

    let mut input = text_input("", &text)
        .padding(Padding {
            top: vertical,
            bottom: vertical,
            left: spacing::SM + text_width::<iced::Renderer>(label, style) + spacing::XS,
            right: spacing::SM,
        })
        .size(style.size)
        .line_height(LineHeight::Absolute(style.line_height.into()))
        .width(Length::Fill);

    let input: Composed<'a, Message> = if props.enabled {
        input = input
            .on_input(move |text| Msg::Own(Own::Input(field, text)))
            .on_submit(Msg::Own(Own::Submit(field)));
        input.into()
    } else {
        exclude_focus(input)
    };

    let prefix = container(styled_text(label.to_owned(), style).class(TextClass::Variant))
        .padding(Padding::from([0.0, spacing::SM]))
        .height(Length::Fixed(height))
        .align_y(Alignment::Center);

    Stack::new()
        .push(input)
        .push(prefix)
        .width(Length::Fill)
        .into()
}

/// A square swatch of `color` with the corner `radius`, over a
/// checkerboard that shows through translucent colors, and outlined.
fn swatch<'a, Message: 'a>(color: Color, side: f32, radius: f32) -> Element<'a, Message> {
    IcedElement::new(Swatch {
        color,
        side,
        radius,
    })
}

/// Draws the checkerboard inside the rounded rectangle `clip` of a frame,
/// with its squares starting at `origin`.
fn draw_checkerboard<Renderer: geometry::Renderer>(
    frame: &mut Frame<Renderer>,
    clip: Rectangle,
    radius: f32,
    origin: Point,
) {
    frame.fill(&rounded(clip, radius), Color::WHITE);

    let outline = rounded_polygon(clip, radius);
    let columns = ((clip.x + clip.width - origin.x) / CHECKER_CELL)
        .ceil()
        .max(0.0) as i32;
    let rows = ((clip.y + clip.height - origin.y) / CHECKER_CELL)
        .ceil()
        .max(0.0) as i32;

    for y in 0..rows {
        for x in 0..columns {
            if (x + y) % 2 != 0 {
                continue;
            }

            let cell = Rectangle::new(
                Point::new(
                    origin.x + x as f32 * CHECKER_CELL,
                    origin.y + y as f32 * CHECKER_CELL,
                ),
                Size::new(CHECKER_CELL, CHECKER_CELL),
            );
            let piece = clip_polygon(&outline, cell);

            if piece.len() > 2 {
                frame.fill(&polygon(&piece), CHECKER_GRAY);
            }
        }
    }
}

/// The path of `rect` with the corner `radius`.
fn rounded(rect: Rectangle, radius: f32) -> Path {
    Path::rounded_rectangle(rect.position(), rect.size(), radius.into())
}

/// The closed path through `points`.
fn polygon(points: &[Point]) -> Path {
    Path::new(|builder| {
        if let Some((first, rest)) = points.split_first() {
            builder.move_to(*first);

            for point in rest {
                builder.line_to(*point);
            }

            builder.close();
        }
    })
}

/// The outline of `rect` with the corner `radius`, as a convex polygon
/// with eight segments per corner.
fn rounded_polygon(rect: Rectangle, radius: f32) -> Vec<Point> {
    const SEGMENTS: usize = 8;

    let radius = radius.min(rect.width / 2.0).min(rect.height / 2.0).max(0.0);
    let corners = [
        (
            Point::new(rect.x + rect.width - radius, rect.y + radius),
            -90.0_f32,
        ),
        (
            Point::new(rect.x + rect.width - radius, rect.y + rect.height - radius),
            0.0,
        ),
        (
            Point::new(rect.x + radius, rect.y + rect.height - radius),
            90.0,
        ),
        (Point::new(rect.x + radius, rect.y + radius), 180.0),
    ];

    corners
        .into_iter()
        .flat_map(|(center, start)| {
            (0..=SEGMENTS).map(move |step| {
                let angle = (start + 90.0 * step as f32 / SEGMENTS as f32).to_radians();

                Point::new(
                    center.x + radius * angle.cos(),
                    center.y + radius * angle.sin(),
                )
            })
        })
        .collect()
}

/// The part of the convex polygon `points` inside `rect`.
fn clip_polygon(points: &[Point], rect: Rectangle) -> Vec<Point> {
    let edges: [&dyn Fn(Point) -> f32; 4] = [
        &|p| p.x - rect.x,
        &|p| rect.x + rect.width - p.x,
        &|p| p.y - rect.y,
        &|p| rect.y + rect.height - p.y,
    ];

    edges.into_iter().fold(points.to_vec(), |points, inside| {
        let mut clipped = Vec::with_capacity(points.len() + 4);

        for (index, &a) in points.iter().enumerate() {
            let b = points[(index + 1) % points.len()];
            let (da, db) = (inside(a), inside(b));

            if da >= 0.0 {
                clipped.push(a);
            }

            if (da >= 0.0) != (db >= 0.0) {
                let t = da / (da - db);
                clipped.push(Point::new(a.x + (b.x - a.x) * t, a.y + (b.y - a.y) * t));
            }
        }

        clipped
    })
}

/// Draws a round thumb filled with `color` at `center`, with a white ring
/// and a dark outline.
fn draw_thumb<Renderer: geometry::Renderer>(
    frame: &mut Frame<Renderer>,
    center: Point,
    color: Color,
) {
    frame.fill(&Path::circle(center, THUMB_RADIUS), THUMB_SHADOW);
    frame.fill(&Path::circle(center, THUMB_RING_RADIUS), Color::WHITE);
    frame.fill(&Path::circle(center, THUMB_DOT_RADIUS), color);
}

/// Draws the frame `draw` fills over `bounds`.
fn draw_frame<Renderer: geometry::Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    draw: impl FnOnce(&mut Frame<Renderer>),
) {
    let mut frame = Frame::new(renderer, bounds.size());
    draw(&mut frame);
    renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
        renderer.draw_geometry(frame.into_geometry());
    });
}

/// The square color swatch of the preview and of the button.
struct Swatch {
    color: Color,
    side: f32,
    radius: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Swatch {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.side), Length::Fixed(self.side))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.side, self.side)
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        use iced::advanced::Renderer as _;

        let bounds = layout.bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: Border::default()
                    .color(theme.roles().outline)
                    .width(1.0)
                    .rounded(self.radius),
                ..renderer::Quad::default()
            },
            Color::TRANSPARENT,
        );

        let color = self.color;
        let radius = self.radius;

        draw_frame(renderer, bounds, |frame| {
            let inner = Rectangle::new(Point::new(1.0, 1.0), bounds.size()).shrink(Padding {
                top: 0.0,
                left: 0.0,
                right: 2.0,
                bottom: 2.0,
            });

            draw_checkerboard(frame, inner, radius, inner.position());
            frame.fill(&rounded(inner, radius), color);
        });
    }
}

/// The interaction state of the area and of the channel sliders.
#[derive(Debug, Clone, Copy, Default)]
struct DragState {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the ring.
    keyboard: bool,
    dragging: bool,
}

impl operation::Focusable for DragState {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
        self.keyboard = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

/// The steps of the arrow key `key` with `modifiers`, as a change of the
/// horizontal and the vertical value, or `None` for other keys. Shift
/// takes bigger steps when `shift_vertical` allows it for Up and Down.
fn arrow_step(
    key: &keyboard::Key,
    modifiers: keyboard::Modifiers,
    (back, forward): (key::Named, key::Named),
    shift_vertical: bool,
) -> Option<(f64, f64)> {
    let shift = if modifiers.is_empty() {
        false
    } else if modifiers == keyboard::Modifiers::SHIFT {
        true
    } else {
        return None;
    };
    let step = if shift { SHIFT_STEP } else { KEY_STEP };

    let keyboard::Key::Named(named) = key else {
        return None;
    };

    match *named {
        named if named == forward => Some((step, 0.0)),
        named if named == back => Some((-step, 0.0)),
        key::Named::ArrowUp if shift_vertical || !shift => Some((0.0, step)),
        key::Named::ArrowDown if shift_vertical || !shift => Some((0.0, -step)),
        _ => None,
    }
}

/// Handles the pointer and keyboard input shared by the area and the
/// sliders: `pick` turns a pointer position into a message, and `step`
/// turns an arrow step into one.
#[allow(clippy::too_many_arguments)]
fn drag_update<Message>(
    state: &mut DragState,
    event: &Event,
    bounds: Rectangle,
    cursor: mouse::Cursor,
    shell: &mut Shell<'_, Message>,
    arrows: (key::Named, key::Named),
    shift_vertical: bool,
    pick: impl Fn(Point) -> Message,
    step: impl Fn(f64, f64) -> Message,
) {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerPressed { .. }) => {
            if let Some(position) = cursor.position_over(bounds) {
                shell.publish(pick(position));
                state.dragging = true;
                state.focused = true;
                state.keyboard = false;
                shell.capture_event();
                shell.request_redraw();
            } else if state.focused {
                state.focused = false;
                shell.request_redraw();
            }
        }
        Event::Mouse(mouse::Event::CursorMoved { .. })
        | Event::Touch(touch::Event::FingerMoved { .. })
            if state.dragging =>
        {
            if let Some(position) = cursor.land().position() {
                shell.publish(pick(position));
            }

            shell.capture_event();
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. })
            if state.dragging =>
        {
            state.dragging = false;
        }
        Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
            if state.focused && !shell.is_event_captured() =>
        {
            if let Some((dx, dy)) = arrow_step(key, *modifiers, arrows, shift_vertical) {
                shell.publish(step(dx, dy));
                state.keyboard = true;
                shell.capture_event();
                shell.request_redraw();
            }
        }
        _ => {}
    }
}

/// The 2D saturation (across) and brightness (up) area.
struct Area<'a, Message> {
    hsv: Hsv,
    on_change: Option<Box<dyn Fn(f64, f64) -> Message + 'a>>,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Area<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<DragState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(DragState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(AREA_HEIGHT))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, AREA_HEIGHT)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<DragState>();

        if self.on_change.is_some() {
            operation.focusable(None, layout.bounds(), state);
        } else {
            state.focused = false;
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<DragState>();
        let Some(on_change) = &self.on_change else {
            *state = DragState::default();
            return;
        };

        let bounds = layout.bounds();
        let (saturation, value) = (self.hsv.saturation, self.hsv.value);

        drag_update(
            state,
            event,
            bounds,
            cursor,
            shell,
            ReadingDirection::LeftToRight.arrows(),
            true,
            |position| {
                on_change(
                    f64::from((position.x - bounds.x) / bounds.width).clamp(0.0, 1.0),
                    f64::from(1.0 - (position.y - bounds.y) / bounds.height).clamp(0.0, 1.0),
                )
            },
            |ds, dv| {
                on_change(
                    (saturation + ds).clamp(0.0, 1.0),
                    (value + dv).clamp(0.0, 1.0),
                )
            },
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<DragState>();
        let bounds = layout.bounds();
        let hsv = self.hsv;
        let pure = Hsv {
            hue: hsv.hue,
            saturation: 1.0,
            value: 1.0,
            alpha: 1.0,
        }
        .to_color();

        draw_frame(renderer, bounds, |frame| {
            let size = bounds.size();
            let rect = rounded(Rectangle::new(Point::ORIGIN, size), radius::MEDIUM);

            frame.fill(&rect, pure);
            frame.fill(
                &rect,
                Linear::new(Point::ORIGIN, Point::new(size.width, 0.0))
                    .add_stop(0.0, Color::WHITE)
                    .add_stop(1.0, Color::from_rgba(1.0, 1.0, 1.0, 0.0)),
            );
            frame.fill(
                &rect,
                Linear::new(Point::ORIGIN, Point::new(0.0, size.height))
                    .add_stop(0.0, Color::from_rgba(0.0, 0.0, 0.0, 0.0))
                    .add_stop(1.0, Color::BLACK),
            );

            draw_thumb(
                frame,
                Point::new(
                    hsv.saturation as f32 * size.width,
                    (1.0 - hsv.value as f32) * size.height,
                ),
                Color {
                    a: 1.0,
                    ..hsv.to_color()
                },
            );
        });

        if state.focused && state.keyboard {
            crate::focus::draw_ring(renderer, theme, bounds, radius::MEDIUM);
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.on_change.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

/// A horizontal slider over a gradient of `colors`, for one channel from
/// 0 to 1.
struct ChannelSlider<'a, Message> {
    value: f64,
    colors: Vec<Color>,
    thumb: Color,
    checkerboard: bool,
    direction: ReadingDirection,
    on_change: Option<Box<dyn Fn(f64) -> Message + 'a>>,
}

impl<Message> ChannelSlider<'_, Message> {
    fn rtl(&self) -> bool {
        self.direction == ReadingDirection::RightToLeft
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for ChannelSlider<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<DragState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(DragState::default())
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fixed(SLIDER_HEIGHT))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, Length::Fill, SLIDER_HEIGHT)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        _renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<DragState>();

        if self.on_change.is_some() {
            operation.focusable(None, layout.bounds(), state);
        } else {
            state.focused = false;
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<DragState>();
        let rtl = self.rtl();
        let Some(on_change) = &self.on_change else {
            *state = DragState::default();
            return;
        };

        let bounds = layout.bounds();
        let value = self.value;

        drag_update(
            state,
            event,
            bounds,
            cursor,
            shell,
            self.direction.arrows(),
            false,
            |position| {
                let extent = (bounds.width - 2.0 * THUMB_RADIUS).max(1.0);
                let x = f64::from((position.x - bounds.x - THUMB_RADIUS) / extent).clamp(0.0, 1.0);

                on_change(if rtl { 1.0 - x } else { x })
            },
            |dx, dy| on_change((value + dx + dy).clamp(0.0, 1.0)),
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<DragState>();
        let bounds = layout.bounds();
        let rtl = self.rtl();
        let value = if rtl { 1.0 - self.value } else { self.value } as f32;

        draw_frame(renderer, bounds, |frame| {
            let size = bounds.size();
            let track = Rectangle::new(
                Point::new(0.0, TRACK_INSET),
                Size::new(size.width, size.height - 2.0 * TRACK_INSET),
            );

            if self.checkerboard {
                draw_checkerboard(frame, track, TRACK_RADIUS, Point::ORIGIN);
            }

            let last = self.colors.len().saturating_sub(1).max(1) as f32;
            let gradient = self.colors.iter().enumerate().fold(
                Linear::new(Point::ORIGIN, Point::new(size.width, 0.0)),
                |gradient, (index, color)| {
                    let offset = index as f32 / last;
                    gradient.add_stop(if rtl { 1.0 - offset } else { offset }, *color)
                },
            );

            frame.fill(&rounded(track, TRACK_RADIUS), gradient);

            draw_thumb(
                frame,
                Point::new(
                    THUMB_RADIUS + value * (size.width - 2.0 * THUMB_RADIUS),
                    size.height / 2.0,
                ),
                self.thumb,
            );
        });

        if state.focused && state.keyboard {
            crate::focus::draw_ring(renderer, theme, bounds, THUMB_RADIUS);
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        if self.on_change.is_some() && cursor.is_over(layout.bounds()) {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        }
    }
}

/// A button showing a color swatch and its hex value that opens a
/// [`ColorPicker`] in a popover. See [`color_picker_button`].
pub struct ColorPickerButton<'a, Message> {
    picker: ColorPicker<'a, Message>,
    open: bool,
    on_open_change: Option<Box<dyn Fn(bool) -> Message + 'a>>,
}

/// A button showing a swatch of `value` and its hex value, which opens a
/// [`ColorPicker`] in a [popover](super::popover()).
///
/// Whether the popover is open lives in the application: the button shows
/// it while [`ColorPickerButton::open`] is set and publishes the message of
/// [`ColorPickerButton::on_open_change`] when it is pressed or the popover
/// is dismissed. Without [`ColorPickerButton::on_change`] the button is
/// disabled.
pub fn color_picker_button<'a, Message>(value: Color) -> ColorPickerButton<'a, Message> {
    ColorPickerButton {
        picker: color_picker(value),
        open: false,
        on_open_change: None,
    }
}

impl<'a, Message> ColorPickerButton<'a, Message> {
    /// Sets the function that turns each new color into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Color) -> Message + 'a) -> Self {
        self.picker = self.picker.on_change(on_change);
        self
    }

    /// Sets the change function, or disables the button with `None`.
    #[must_use]
    pub fn on_change_maybe(mut self, on_change: Option<impl Fn(Color) -> Message + 'a>) -> Self {
        self.picker = self.picker.on_change_maybe(on_change);
        self
    }

    /// Shows the popover while `open` is true.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the function that turns the requested open state into a
    /// message: true when the button is pressed while closed, false when it
    /// is pressed while open or the popover is dismissed.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Box::new(on_open_change));
        self
    }

    /// Sets whether the picker offers opacity and the label includes the
    /// alpha channel (true by default).
    #[must_use]
    pub fn show_alpha(mut self, show_alpha: bool) -> Self {
        self.picker = self.picker.show_alpha(show_alpha);
        self
    }

    /// Sets the preset colors offered in the picker.
    #[must_use]
    pub fn swatches(mut self, swatches: impl IntoIterator<Item = Color>) -> Self {
        self.picker = self.picker.swatches(swatches);
        self
    }

    /// Takes the strings of the picker from `localizations` (English by
    /// default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.picker = self.picker.localizations(localizations);
        self
    }

    /// Sets the reading direction of the picker and of the popover.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.picker = self.picker.direction(direction);
        self
    }
}

impl<'a, Message: Clone + 'a> From<ColorPickerButton<'a, Message>> for Element<'a, Message> {
    fn from(picker_button: ColorPickerButton<'a, Message>) -> Self {
        let ColorPickerButton {
            picker,
            open,
            on_open_change,
        } = picker_button;
        let value = picker.value;
        let direction = picker.direction;
        let hex = to_hex(value, picker.show_alpha);
        let enabled = picker.on_change.is_some();
        let on_open_change = on_open_change.filter(|_| enabled);

        let content = Row::new()
            .push(swatch(value, BUTTON_SWATCH_SIZE, radius::CHECK))
            .push(styled_text(hex, typography::LABEL_LARGE))
            .spacing(spacing::SM)
            .align_y(Alignment::Center);

        let anchor = button::button(content)
            .class(button::Class::Outlined)
            .on_press_maybe(on_open_change.as_ref().map(|change| change(!open)));

        let mut popover = popover(anchor, picker)
            .open(open && on_open_change.is_some())
            .direction(direction);

        if let Some(change) = &on_open_change {
            popover = popover.on_dismiss(change(false));
        }

        popover.into()
    }
}
