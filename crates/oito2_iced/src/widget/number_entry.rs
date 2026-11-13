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

//! Number entries: a number field with decrease and increase buttons, like
//! a GNOME spin button.
//!
//! The entry shows its value in the number format of the locale with a
//! fixed number of decimal places, aligned to the end. Typed text is read
//! when the field loses the focus or on Enter, rounded, kept between the
//! bounds and published; text that is not a number is replaced by the
//! current value. The text being typed lives in the entry, the value in the
//! app.
//!
//! While the entry is enabled, the Up and Down arrows change the value by
//! the step, Page Up and Page Down by the page step, and Home and End jump
//! to the bounds when they are finite. With the focus in the field the
//! mouse wheel steps too, and dragging sideways on the buttons scrubs the
//! value one step per 4 pixels.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced::widget::number_entry;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Copies(f64),
//! }
//!
//! # fn view<'a>(copies: f64) -> oito2_iced::Element<'a, Message> {
//! number_entry::number_entry(&Localizations::en(), copies)
//!     .range(1.0, 99.0)
//!     .label("Copies")
//!     .on_change(Message::Copies)
//!     .into()
//! # }
//! ```

#[cfg(feature = "svg")]
use std::borrow::Cow;
#[cfg(feature = "svg")]
use std::rc::Rc;

#[cfg(feature = "svg")]
use iced::advanced::widget::Id;
#[cfg(feature = "svg")]
use iced::keyboard::{self, key};
#[cfg(feature = "svg")]
use iced::widget::text::IntoFragment;
#[cfg(feature = "svg")]
use iced::widget::{Column, text_input};
#[cfg(feature = "svg")]
use iced::{Event, Length, Point, Rectangle, alignment, mouse};

#[cfg(feature = "svg")]
use super::search::field_control;
#[cfg(feature = "svg")]
use super::search::host::{Context, Effects, Host, Msg, Reconcile, Signal};
#[cfg(feature = "svg")]
use super::{button, styled_text};
#[cfg(feature = "svg")]
use crate::Element;
#[cfg(feature = "svg")]
use crate::icon::lucide;
#[cfg(feature = "svg")]
use crate::l10n::Localizations;
#[cfg(feature = "svg")]
use crate::theme::text::Class as TextClass;
#[cfg(feature = "svg")]
use crate::tokens::{spacing, typography};

/// The most decimal places an entry shows.
pub const MAX_DECIMALS: usize = 20;

/// The horizontal drag on the buttons that changes the value by one step.
pub const SCRUB_DISTANCE: f32 = 4.0;

#[cfg(feature = "svg")]
/// The separators of a number format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Separators {
    group: char,
    decimal: char,
}

#[cfg(feature = "svg")]
impl Separators {
    /// The separators of the locale named `locale_name`: a dot between
    /// thousands and a comma before decimals in Portuguese, the reverse in
    /// English.
    fn of(locale_name: &str) -> Self {
        if locale_name.starts_with("pt") {
            Self {
                group: '.',
                decimal: ',',
            }
        } else {
            Self {
                group: ',',
                decimal: '.',
            }
        }
    }
}

#[cfg(feature = "svg")]
/// Rounds `value` half away from zero to `decimals` places, leaving values
/// whose scaled form is not finite as they are.
fn round(value: f64, decimals: usize) -> f64 {
    let factor = 10f64.powi(i32::try_from(decimals.min(MAX_DECIMALS)).unwrap_or(0));
    let scaled = value * factor;

    if scaled.is_finite() {
        scaled.round() / factor
    } else {
        value
    }
}

#[cfg(feature = "svg")]
/// Rounds `value` to `decimals` places and keeps it between `min` and
/// `max`.
fn normalize(value: f64, decimals: usize, min: f64, max: f64) -> f64 {
    round(value, decimals).max(min).min(max)
}

#[cfg(feature = "svg")]
/// Formats `value` with `decimals` places, the thousands grouped and the
/// separators of `separators`.
fn format(value: f64, decimals: usize, separators: Separators) -> String {
    let decimals = decimals.min(MAX_DECIMALS);
    let rounded = round(value, decimals);
    let text = format!("{:.*}", decimals, rounded.abs());
    let (integer, fraction) = text.split_once('.').unwrap_or((&text, ""));

    let mut out = String::new();

    if rounded < 0.0 {
        out.push('-');
    }

    let digits = integer.len();

    for (index, digit) in integer.chars().enumerate() {
        if index > 0 && (digits - index) % 3 == 0 {
            out.push(separators.group);
        }

        out.push(digit);
    }

    if !fraction.is_empty() {
        out.push(separators.decimal);
        out.push_str(fraction);
    }

    out
}

#[cfg(feature = "svg")]
/// Returns whether `text` holds only digits, the separators, signs and
/// white space, the characters a number can be typed with.
fn accepts(text: &str, separators: Separators) -> bool {
    text.chars().all(|c| {
        c.is_ascii_digit()
            || c == separators.group
            || c == separators.decimal
            || matches!(c, '-' | '+' | '\u{2212}')
            || c.is_whitespace()
    })
}

#[cfg(feature = "svg")]
/// Reads the number in `text`, written with `separators`, or `None` when
/// it is not a number.
fn parse(text: &str, separators: Separators) -> Option<f64> {
    let mut plain = String::new();

    for c in text.trim().chars() {
        match c {
            c if c == separators.group || c.is_whitespace() => {}
            c if c == separators.decimal => plain.push('.'),
            '\u{2212}' => plain.push('-'),
            c => plain.push(c),
        }
    }

    let digits = plain.trim_start_matches(['-', '+']);
    let well_formed = plain.len() - digits.len() <= 1
        && digits.chars().any(|c| c.is_ascii_digit())
        && digits.chars().all(|c| c.is_ascii_digit() || c == '.');

    if well_formed {
        plain.parse().ok()
    } else {
        None
    }
}

/// A number entry showing `value`, with the localized "Decrease" and
/// "Increase" tooltips of `localizations` and its number format. Without
/// [`NumberEntry::on_change`] it is read-only.
#[cfg(feature = "svg")]
pub fn number_entry<'a, Message: Clone + 'a>(
    localizations: &Localizations,
    value: f64,
) -> NumberEntry<'a, Message> {
    NumberEntry {
        config: Config {
            value,
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
            step: 1.0,
            page_step: None,
            decimals: 0,
            separators: Separators::of(localizations.locale_name),
        },
        on_change: None,
        decrease_label: localizations.decrease,
        increase_label: localizations.increase,
        label: None,
        placeholder: String::new(),
        id: None,
        width: Length::Fill,
    }
}

#[cfg(feature = "svg")]
/// The numbers of a number entry.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Config {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
    page_step: Option<f64>,
    decimals: usize,
    separators: Separators,
}

#[cfg(feature = "svg")]
impl Config {
    /// The change of Page Up and Page Down: ten steps unless set.
    fn page_step(&self) -> f64 {
        self.page_step.unwrap_or(self.step * 10.0)
    }

    /// Formats `value` in the format of the entry.
    fn format(&self, value: f64) -> String {
        format(value, self.decimals, self.separators)
    }

    /// Reads a number typed in the format of the entry.
    fn parse(&self, text: &str) -> Option<f64> {
        parse(text, self.separators)
    }
}

/// A number entry. See [`number_entry`].
#[cfg(feature = "svg")]
pub struct NumberEntry<'a, Message> {
    config: Config,
    on_change: Option<Rc<dyn Fn(f64) -> Message + 'a>>,
    decrease_label: &'static str,
    increase_label: &'static str,
    label: Option<Cow<'a, str>>,
    placeholder: String,
    id: Option<Id>,
    width: Length,
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> NumberEntry<'a, Message> {
    /// Sets the message produced from each new value; without it the entry
    /// is read-only.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(f64) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }

    /// Keeps the value between `min` and `max` (unbounded by default).
    #[must_use]
    pub fn range(mut self, min: f64, max: f64) -> Self {
        debug_assert!(min <= max, "the minimum is greater than the maximum");
        self.config.min = min;
        self.config.max = max;
        self
    }

    /// Sets the change of the arrow keys, the buttons, the wheel and each
    /// scrub distance (1 by default); it must be greater than zero.
    #[must_use]
    pub fn step(mut self, step: f64) -> Self {
        debug_assert!(step > 0.0, "the step is not greater than zero");
        self.config.step = step;
        self
    }

    /// Sets the change of Page Up and Page Down (ten steps by default).
    #[must_use]
    pub fn page_step(mut self, page_step: f64) -> Self {
        self.config.page_step = Some(page_step);
        self
    }

    /// Sets the number of decimal places shown and kept, from 0 (the
    /// default) to [`MAX_DECIMALS`].
    #[must_use]
    pub fn decimals(mut self, decimals: usize) -> Self {
        debug_assert!(decimals <= MAX_DECIMALS, "too many decimal places");
        self.config.decimals = decimals.min(MAX_DECIMALS);
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Shows `placeholder` while the field is empty.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        placeholder.clone_into(&mut self.placeholder);
        self
    }

    /// Sets the widget [`Id`] of the input.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

/// The own events of a number entry.
#[cfg(feature = "svg")]
#[derive(Debug, Clone)]
enum Entry {
    /// The text was edited.
    Input(String),
    /// Enter was pressed in the field.
    Submit,
    /// A button was pressed: one step up (true) or down (false).
    Step(bool),
}

/// A horizontal drag on the buttons.
#[cfg(feature = "svg")]
#[derive(Debug, Clone, Copy)]
struct Scrub {
    x: f32,
    distance: f32,
    stepped: bool,
}

/// The state of a number entry.
#[cfg(feature = "svg")]
#[derive(Debug, Default)]
struct EntryState {
    /// The text in the field while it differs from the formatted value.
    draft: Option<String>,
    /// The value, decimals and separators the field was shown for.
    shown: Option<(u64, usize, Separators)>,
    /// The drag on the buttons, while the left button is held on them.
    scrub: Option<Scrub>,
}

/// The behaviour of a number entry, shared by its view and its handler.
#[cfg(feature = "svg")]
struct Logic<'a, Message> {
    config: Config,
    on_change: Option<Rc<dyn Fn(f64) -> Message + 'a>>,
}

#[cfg(feature = "svg")]
impl<Message> Logic<'_, Message> {
    /// Whether the entry takes changes.
    fn enabled(&self) -> bool {
        self.on_change.is_some()
    }

    /// The text the field shows.
    fn text(&self, state: &EntryState) -> String {
        state
            .draft
            .clone()
            .unwrap_or_else(|| self.config.format(self.config.value))
    }

    /// The number typed in the field, or the value when the text is not a
    /// number.
    fn current(&self, state: &EntryState) -> f64 {
        state
            .draft
            .as_deref()
            .and_then(|text| self.config.parse(text))
            .unwrap_or(self.config.value)
    }

    /// Normalizes and shows `value`, publishing it when it differs from the
    /// value of the app.
    fn set(&self, state: &mut EntryState, value: f64, effects: &mut Effects<Message>) {
        let config = &self.config;
        let next = normalize(value, config.decimals, config.min, config.max);

        state.draft = Some(config.format(next));

        if next != config.value
            && let Some(on_change) = &self.on_change
        {
            effects.publish(on_change(next));
        }

        effects.rebuild();
    }

    /// Applies the typed text, or shows the value again when the text is
    /// not a number or the entry is read-only.
    fn commit(&self, state: &mut EntryState, effects: &mut Effects<Message>) {
        let Some(draft) = &state.draft else {
            return;
        };

        match self.config.parse(draft) {
            Some(parsed) if self.enabled() => self.set(state, parsed, effects),
            _ => {
                state.draft = None;
                effects.rebuild();
            }
        }
    }

    /// Adds `delta` to the typed number, or to the value when the text is
    /// not a number.
    fn step_by(&self, state: &mut EntryState, delta: f64, effects: &mut Effects<Message>) {
        if self.enabled() {
            let current = self.current(state);
            self.set(state, current + delta, effects);
        }
    }

    /// Handles a key press while the entry has the focus; returns whether
    /// the key is one of the entry.
    fn key(&self, state: &mut EntryState, key: key::Named, effects: &mut Effects<Message>) -> bool {
        let config = &self.config;

        match key {
            key::Named::ArrowUp => self.step_by(state, config.step, effects),
            key::Named::ArrowDown => self.step_by(state, -config.step, effects),
            key::Named::PageUp => self.step_by(state, config.page_step(), effects),
            key::Named::PageDown => self.step_by(state, -config.page_step(), effects),
            key::Named::Home if self.enabled() && config.min.is_finite() => {
                self.set(state, config.min, effects);
            }
            key::Named::End if self.enabled() && config.max.is_finite() => {
                self.set(state, config.max, effects);
            }
            _ => return false,
        }

        true
    }

    /// The area of the buttons: the end of the input, two control heights
    /// wide.
    fn buttons(context: &Context) -> Option<Rectangle> {
        let input = context.input_bounds?;
        let width = (input.height * 2.0).min(input.width);

        Some(Rectangle {
            x: input.x + input.width - width,
            width,
            ..input
        })
    }

    /// Starts, follows and ends a horizontal drag on the buttons.
    fn scrub(
        &self,
        state: &mut EntryState,
        event: &Event,
        context: &Context,
        effects: &mut Effects<Message>,
    ) {
        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) if self.enabled() => {
                if let Some(position) =
                    Self::buttons(context).and_then(|buttons| context.cursor.position_over(buttons))
                {
                    state.scrub = Some(Scrub {
                        x: position.x,
                        distance: 0.0,
                        stepped: false,
                    });
                }
            }
            Event::Mouse(mouse::Event::CursorMoved {
                position: Point { x, .. },
            }) => {
                let Some(mut scrub) = state.scrub else {
                    return;
                };

                scrub.distance += x - scrub.x;
                scrub.x = *x;

                while scrub.distance.abs() >= SCRUB_DISTANCE {
                    let up = scrub.distance > 0.0;
                    scrub.distance -= SCRUB_DISTANCE.copysign(scrub.distance);
                    scrub.stepped = true;
                    let step = if up {
                        self.config.step
                    } else {
                        -self.config.step
                    };
                    self.step_by(state, step, effects);
                }

                state.scrub = Some(scrub);
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
                if state.scrub.take().is_some_and(|scrub| scrub.stepped) =>
            {
                effects.hide_cursor();
            }
            _ => {}
        }
    }

    /// Handles a signal of the host.
    fn handle(
        &self,
        state: &mut EntryState,
        signal: Signal<'_, Entry>,
        effects: &mut Effects<Message>,
    ) {
        match signal {
            Signal::Own(Entry::Input(text)) => {
                if self.enabled() && accepts(&text, self.config.separators) {
                    state.draft = Some(text);
                }

                effects.rebuild();
            }
            Signal::Own(Entry::Submit) => self.commit(state, effects),
            Signal::Own(Entry::Step(up)) => {
                let step = if up {
                    self.config.step
                } else {
                    -self.config.step
                };
                self.step_by(state, step, effects);
            }
            Signal::Focus(false) => self.commit(state, effects),
            Signal::Focus(true) => {}
            Signal::Before(event, context) => self.scrub(state, event, context, effects),
            Signal::After(
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key),
                    modifiers,
                    ..
                }),
                context,
            ) if (context.input_focused || context.other_focused) && modifiers.is_empty() => {
                if self.key(state, *key, effects) {
                    effects.capture();
                }
            }
            Signal::After(Event::Mouse(mouse::Event::WheelScrolled { delta }), context)
                if context.input_focused && context.cursor.is_over(context.bounds) =>
            {
                let y = match delta {
                    mouse::ScrollDelta::Lines { y, .. } | mouse::ScrollDelta::Pixels { y, .. } => {
                        *y
                    }
                };

                if y != 0.0 {
                    let step = if y > 0.0 {
                        self.config.step
                    } else {
                        -self.config.step
                    };
                    self.step_by(state, step, effects);
                    effects.capture();
                }
            }
            Signal::After(..) => {}
        }
    }

    /// Reconciles the state with the value, the decimals and the format:
    /// a new value or number of decimals replaces the typed text, and so
    /// does a new format while the field has no focus.
    fn reconcile(&self, state: &mut EntryState, focused: bool) -> Reconcile {
        let config = &self.config;
        let shown = (config.value.to_bits(), config.decimals, config.separators);
        let previous = state.shown.replace(shown);

        let replace = match previous {
            Some((value, decimals, separators)) => {
                value != shown.0 || decimals != shown.1 || (separators != shown.2 && !focused)
            }
            None => false,
        };

        if replace && state.draft.is_some() {
            state.draft = None;

            Reconcile {
                rebuild: true,
                focus: None,
            }
        } else {
            Reconcile {
                rebuild: false,
                focus: None,
            }
        }
    }
}

#[cfg(feature = "svg")]
impl<'a, Message: Clone + 'a> From<NumberEntry<'a, Message>> for Element<'a, Message> {
    fn from(entry: NumberEntry<'a, Message>) -> Self {
        let logic = Rc::new(Logic {
            config: entry.config,
            on_change: entry.on_change,
        });
        let view_logic = Rc::clone(&logic);

        let decrease_label = entry.decrease_label;
        let increase_label = entry.increase_label;
        let label = entry.label;
        let placeholder = entry.placeholder;
        let id = entry.id;
        let width = entry.width;

        let view = move |state: &EntryState| -> Element<'a, Msg<Message, Entry>> {
            let logic = &view_logic;
            let config = &logic.config;
            let text = logic.text(state);
            let current = logic.current(state);

            let mut input = text_input(&placeholder, &text)
                .on_input(|text| Msg::Own(Entry::Input(text)))
                .on_submit(Msg::Own(Entry::Submit))
                .align_x(alignment::Horizontal::Right);

            if let Some(id) = &id {
                input = input.id(id.clone());
            }

            let decrease = button::icon_button(lucide::MINUS)
                .tooltip(decrease_label)
                .on_press_maybe(
                    (logic.enabled() && current > config.min)
                        .then_some(Msg::Own(Entry::Step(false))),
                );
            let increase = button::icon_button(lucide::PLUS)
                .tooltip(increase_label)
                .on_press_maybe(
                    (logic.enabled() && current < config.max)
                        .then_some(Msg::Own(Entry::Step(true))),
                );

            let mut column = Column::new().spacing(spacing::XS).width(width);

            if let Some(label) = &label {
                column = column.push(
                    styled_text(label.clone(), typography::BODY_MEDIUM).class(TextClass::Variant),
                );
            }

            column
                .push(field_control(
                    input,
                    0.0,
                    vec![decrease.into(), increase.into()],
                    false,
                ))
                .into()
        };

        let handler_logic = Rc::clone(&logic);

        Host::new(view, move |state, signal, effects| {
            handler_logic.handle(state, signal, effects);
        })
        .reconcile(move |state, focused| logic.reconcile(state, focused))
        .into()
    }
}

#[cfg(all(test, feature = "svg"))]
mod tests {
    use super::*;

    const EN: Separators = Separators {
        group: ',',
        decimal: '.',
    };
    const PT: Separators = Separators {
        group: '.',
        decimal: ',',
    };

    #[test]
    fn formats_with_the_decimals_and_the_locale_separators() {
        assert_eq!(format(12.0, 0, EN), "12");
        assert_eq!(format(7.26, 1, EN), "7.3");
        assert_eq!(format(10.0, 1, EN), "10.0");
        assert_eq!(format(1234.5, 2, EN), "1,234.50");
        assert_eq!(format(1234.5, 2, PT), "1.234,50");
        assert_eq!(format(-1_234_567.0, 0, EN), "-1,234,567");
        assert_eq!(format(-0.01, 0, EN), "0");
    }

    #[test]
    fn parses_the_locale_format_and_rejects_the_rest() {
        assert_eq!(parse("7.26", EN), Some(7.26));
        assert_eq!(parse(" 1,234 ", EN), Some(1234.0));
        assert_eq!(parse("1.234,5", PT), Some(1234.5));
        assert_eq!(parse("\u{2212}3", EN), Some(-3.0));
        assert_eq!(parse("+4", EN), Some(4.0));
        assert_eq!(parse("..", EN), None);
        assert_eq!(parse("", EN), None);
        assert_eq!(parse("-", EN), None);
        assert_eq!(parse("--3", EN), None);
        assert_eq!(parse("1e5", EN), None);
        assert_eq!(parse("inf", EN), None);
    }

    #[test]
    fn accepts_only_number_characters() {
        assert!(accepts("-1,234.5", EN));
        assert!(accepts("1\u{a0}234", EN));
        assert!(!accepts("12a", EN));
    }

    #[test]
    fn normalizes_by_rounding_half_away_from_zero_and_clamping() {
        assert_eq!(normalize(7.26, 1, 0.0, 10.0), 7.3);
        assert_eq!(normalize(7.25, 1, 0.0, 10.0), 7.3);
        assert_eq!(normalize(-7.25, 1, -10.0, 10.0), -7.3);
        assert_eq!(normalize(42.0, 1, 0.0, 10.0), 10.0);
        assert_eq!(normalize(-3.0, 0, 0.0, 10.0), 0.0);
        assert_eq!(normalize(5.5, 0, f64::NEG_INFINITY, f64::INFINITY), 6.0);
    }

    #[test]
    fn the_locale_picks_the_separators() {
        assert_eq!(Separators::of("en"), EN);
        assert_eq!(Separators::of("pt_BR"), PT);
    }
}
