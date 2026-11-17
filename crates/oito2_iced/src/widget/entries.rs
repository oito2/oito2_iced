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

//! Parsed entries: date, time and date-and-time text fields, and a
//! segmented entry for values like IP addresses or license keys.
//!
//! [`date_entry`] accepts the compact date format of the
//! [`Localizations`](crate::l10n::Localizations)
//! (`mm/dd/yyyy` in English) and offers a button that opens a
//! [calendar](super::calendar) in a popover; [`time_entry`] accepts `H:MM`
//! and `HH:MM`, with `:`, `.` or `h` between, and an optional AM/PM marker,
//! and offers a button that opens hour and minute
//! [number entries](super::number_entry) in a popover;
//! [`date_time_entry`] puts both side by side and reports one combined
//! value, where an empty time means midnight.
//!
//! The typed text lives in the entry and the value in the application: a
//! valid text publishes its value through `on_change`, invalid text
//! publishes nothing, so the last valid value stays, and `on_error_change`
//! tells when the text stops or starts parsing. The error shows under the
//! field only after the field loses the focus or is submitted with Enter,
//! so partial input is not flagged while typing. A new value from the
//! application replaces the text unless the text already reads as it.
//! The open state of the popovers lives in the application too.
//!
//! [`segmented_entry`] is a row of short fields joined by a delimiter. The
//! focus moves to the next segment when one fills up or when the first
//! visible character of the delimiter is typed, and back to the previous
//! one on Backspace in an empty segment or on the arrow toward the start
//! of the reading direction at its start.
//!
//! ```no_run
//! use oito2_iced::date::Date;
//! use oito2_iced::widget::entries;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Due(Option<Date>),
//!     CalendarOpen(bool),
//!     Address(Vec<String>),
//! }
//!
//! # fn view<'a>(today: Date, due: Option<Date>, open: bool, address: &'a [String]) -> oito2_iced::Element<'a, Message> {
//! iced::widget::column![
//!     entries::date_entry(today, due)
//!         .on_change(Message::Due)
//!         .open(open)
//!         .on_open_change(Message::CalendarOpen),
//!     entries::segmented_entry((0..4).map(|_| entries::segment(3).hint("000")), address)
//!         .on_change(Message::Address),
//! ]
//! .into()
//! # }
//! ```

use std::borrow::Cow;

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::Paragraph as _;
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Id, Tree, Widget};
use iced::advanced::{Clipboard, Shell, overlay, renderer, text};
use iced::keyboard::{self, key};
use iced::widget::text::{IntoFragment, LineHeight};
use iced::widget::text_input::{self, Value, cursor};
use iced::widget::{Row, container};
use iced::{Alignment, Event, Length, Padding, Pixels, Rectangle, Size, Vector, alignment, mouse};

use super::helpers::ReadingDirection;
use super::styled_text;
use crate::tokens::{spacing, typography};
use crate::{Element, Theme, density};

#[cfg(feature = "svg")]
pub use parsed::{
    DateEntry, DateTimeEntry, Part, TimeEntry, date_entry, date_time_entry, time_entry,
};

/// The paragraph of the text of the default renderer.
type Paragraph = <iced::Renderer as text::Renderer>::Paragraph;

/// The text style of the typed values: body large.
const VALUE_STYLE: typography::TextStyle = typography::BODY_LARGE;

/// The vertical padding that centers one line of the value in the control
/// height of the current density.
fn vertical_padding() -> f32 {
    ((density::control_height() - VALUE_STYLE.line_height) / 2.0).max(0.0)
}

/// Copies to `shell` everything but the messages of `local`.
fn absorb<A, B>(shell: &mut Shell<'_, A>, local: &Shell<'_, B>) {
    if local.is_event_captured() {
        shell.capture_event();
    }

    shell.request_redraw_at(local.redraw_request());
    shell.request_input_method(local.input_method());

    if local.is_layout_invalid() {
        shell.invalidate_layout();
    }

    if local.are_widgets_invalid() {
        shell.invalidate_widgets();
    }
}

/// The date, time and date-and-time entries, which need the icons.
#[cfg(feature = "svg")]
mod parsed {
    use std::borrow::Cow;
    use std::cell::Cell;
    use std::rc::Rc;

    use iced::advanced::layout::{self, Layout};
    use iced::advanced::widget::operation::Operation;
    use iced::advanced::widget::{Id, Tree, Widget, tree};
    use iced::advanced::{Clipboard, Shell, overlay, renderer};
    use iced::widget::text::{IntoFragment, LineHeight};
    use iced::widget::{Column, Row, Space, Stack, container, text_input};
    use iced::{
        Alignment, Element as IcedElement, Event, Length, Padding, Rectangle, Size, Vector, mouse,
    };

    use super::{VALUE_STYLE, vertical_padding};
    use crate::date::{Date, Time};
    use crate::icon::lucide;
    use crate::l10n::Localizations;
    use crate::theme::text::Class as TextClass;
    use crate::theme::text_input::Class as InputClass;
    use crate::tokens::{spacing, typography};
    use crate::widget::calendar::{Pick, calendar};
    use crate::widget::search::host::{Effects, Host, Msg, Reconcile, Signal};
    use crate::widget::{button, controls, number_entry, popover, segment_group, styled_text};
    use crate::{Element, Theme, density};

    /// The width of the hour and minute entries of the time popover.
    const NUMBER_WIDTH: f32 = 112.0;

    /// A part of a date-and-time entry, and the popover it opens.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum Part {
        /// The date field and its calendar.
        Date,
        /// The time field and its hour and minute entries.
        Time,
    }

    /// Which fields an entry has.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Kind {
        Date,
        Time,
        DateTime,
    }

    impl Kind {
        fn has_date(self) -> bool {
            self != Kind::Time
        }

        fn has_time(self) -> bool {
            self != Kind::Date
        }
    }

    /// Parses and formats the values of a field.
    trait Rules {
        /// The value of the field.
        type Value: Copy + PartialEq;

        /// The value of `text`: `None` for an accepted empty text, or the
        /// error message.
        fn parse(&self, text: &str) -> Result<Option<Self::Value>, &'static str>;

        /// `value` as text.
        fn format(&self, value: Self::Value) -> String;
    }

    /// The rules of a date field.
    #[derive(Clone)]
    struct DateRules<'a> {
        localizations: Localizations,
        /// The day a one- or two-digit year is expanded around.
        reference: Date,
        first: Date,
        last: Date,
        selectable: Option<Rc<dyn Fn(Date) -> bool + 'a>>,
        accept_empty: bool,
    }

    impl DateRules<'_> {
        /// Whether `date` is within the limits and accepted by the
        /// predicate.
        fn accepts(&self, date: Date) -> bool {
            self.first <= date
                && date <= self.last
                && self.selectable.as_ref().is_none_or(|accepts| accepts(date))
        }
    }

    impl Rules for DateRules<'_> {
        type Value = Date;

        fn parse(&self, text: &str) -> Result<Option<Date>, &'static str> {
            let strings = self.localizations.material;
            let text = text.trim();

            if text.is_empty() {
                return if self.accept_empty {
                    Ok(None)
                } else {
                    Err(strings.invalid_date_format)
                };
            }

            let date = self
                .localizations
                .parse_date(text, self.reference)
                .ok_or(strings.invalid_date_format)?;

            if self.accepts(date) {
                Ok(Some(date))
            } else {
                Err(strings.date_out_of_range)
            }
        }

        fn format(&self, value: Date) -> String {
            self.localizations.compact_date(value)
        }
    }

    /// The rules of a time field.
    #[derive(Clone, Copy)]
    struct TimeRules {
        localizations: Localizations,
        use_24_hour: bool,
        accept_empty: bool,
    }

    impl Rules for TimeRules {
        type Value = Time;

        fn parse(&self, text: &str) -> Result<Option<Time>, &'static str> {
            let invalid = self.localizations.material.invalid_time;

            if text.trim().is_empty() {
                return if self.accept_empty {
                    Ok(None)
                } else {
                    Err(invalid)
                };
            }

            self.localizations.parse_time(text).map(Some).ok_or(invalid)
        }

        fn format(&self, value: Time) -> String {
            self.localizations.time_of_day(value, self.use_24_hour)
        }
    }

    /// The state of one field: the typed text, the last valid value and
    /// the error.
    #[derive(Debug, Clone)]
    struct Field<T: Copy> {
        ready: bool,
        draft: String,
        /// The value of the application the text was last checked against.
        shown: Option<T>,
        /// The value of the last valid text.
        valid: Option<T>,
        error: Option<&'static str>,
        /// Whether the error shows: after a blur or a submit.
        show_error: bool,
        /// A value picked in the popover, applied at the next layout.
        picked: Rc<Cell<Option<T>>>,
    }

    impl<T: Copy> Default for Field<T> {
        fn default() -> Self {
            Self {
                ready: false,
                draft: String::new(),
                shown: None,
                valid: None,
                error: None,
                show_error: false,
                picked: Rc::new(Cell::new(None)),
            }
        }
    }

    impl<T: Copy + PartialEq> Field<T> {
        /// Shows `value` and takes it as valid.
        fn set<R: Rules<Value = T>>(&mut self, rules: &R, value: T) {
            self.draft = rules.format(value);
            self.valid = Some(value);
            self.error = None;
            self.show_error = false;
        }

        /// Takes the typed `text`: a valid text sets the value and hides
        /// the error.
        fn input<R: Rules<Value = T>>(&mut self, rules: &R, text: String) {
            match rules.parse(&text) {
                Ok(value) => {
                    self.valid = value;
                    self.error = None;
                    self.show_error = false;
                }
                Err(error) => self.error = Some(error),
            }

            self.draft = text;
        }

        /// The error to show, once the field lost the focus or was
        /// submitted.
        fn visible_error(&self) -> Option<&'static str> {
            self.error.filter(|_| self.show_error)
        }

        /// Shows the value of the application on first sight, applies a
        /// picked value, and replaces the text with a new value of the
        /// application unless the text reads as it. `external` is `None`
        /// when the application value says nothing about this field; an
        /// empty text reads as `empty_reads_as`. Returns whether the field
        /// changed.
        fn sync<R: Rules<Value = T>>(
            &mut self,
            rules: &R,
            external: Option<Option<T>>,
            empty_reads_as: Option<T>,
        ) -> bool {
            if !self.ready {
                let value = external.flatten();

                self.ready = true;
                self.draft = value.map(|value| rules.format(value)).unwrap_or_default();
                self.valid = value;
                self.shown = value;
                return true;
            }

            if let Some(picked) = self.picked.take() {
                self.set(rules, picked);

                if let Some(external) = external {
                    self.shown = external;
                }

                return true;
            }

            let Some(external) = external else {
                return false;
            };

            if external == self.shown {
                return false;
            }

            self.shown = external;

            let current = rules.parse(&self.draft);
            let reads_as = current == Ok(external)
                || (current == Ok(None) && external.is_some() && external == empty_reads_as);

            if reads_as {
                return false;
            }

            self.draft = external
                .map(|value| rules.format(value))
                .unwrap_or_default();
            self.valid = external;
            self.error = None;
            self.show_error = false;
            true
        }
    }

    /// The state of an entry in the widget tree.
    #[derive(Debug, Clone, Default)]
    struct EntryState {
        date: Field<Date>,
        time: Field<Time>,
        /// The error last published.
        reported_error: Option<&'static str>,
        /// Whether the error changed outside an edit and is published on
        /// the next event.
        pending: bool,
    }

    /// The own events of an entry.
    #[derive(Debug, Clone)]
    enum EntryEvent {
        /// The text of a field was edited.
        Input(Part, String),
        /// Enter was pressed in a field.
        Submit(Part),
        /// A field lost the focus.
        Blur(Part),
        /// Nothing to do.
        Nothing,
    }

    /// The value of an entry: a date, a time, or both.
    type Value = (Option<Date>, Option<Time>);

    /// A closure that turns a value into a message.
    type Callback<'a, T, Message> = Box<dyn Fn(T) -> Message + 'a>;

    /// The properties of an entry, shared by its view, handler and
    /// reconciler.
    struct Spec<'a, Message> {
        kind: Kind,
        today: Date,
        date: DateRules<'a>,
        time: TimeRules,
        value: Value,
        on_change: Option<Callback<'a, Value, Message>>,
        on_error: Option<Callback<'a, Option<String>, Message>>,
        open: Option<Part>,
        on_open_change: Option<Callback<'a, Option<Part>, Message>>,
        date_label: Option<Cow<'a, str>>,
        time_label: Option<Cow<'a, str>>,
        date_id: Option<Id>,
        time_id: Option<Id>,
        width: Length,
    }

    impl<'a, Message: Clone + 'a> Spec<'a, Message> {
        fn new(kind: Kind, today: Date, value: Value) -> Self {
            let localizations = Localizations::default();

            Self {
                kind,
                today,
                date: DateRules {
                    localizations,
                    reference: today,
                    first: Date::MIN,
                    last: Date::MAX,
                    selectable: None,
                    accept_empty: true,
                },
                time: TimeRules {
                    localizations,
                    use_24_hour: false,
                    accept_empty: true,
                },
                value,
                on_change: None,
                on_error: None,
                open: None,
                on_open_change: None,
                date_label: None,
                time_label: None,
                date_id: None,
                time_id: None,
                width: Length::Fill,
            }
        }

        fn localizations(&mut self, localizations: &Localizations) {
            self.date.localizations = *localizations;
            self.time.localizations = *localizations;
        }

        /// Whether the entry takes edits.
        fn enabled(&self) -> bool {
            self.on_change.is_some()
        }

        /// The error of the entry: the date error, or else the time error.
        fn error_of(&self, state: &EntryState) -> Option<&'static str> {
            let date = state.date.error.filter(|_| self.kind.has_date());
            let time = state.time.error.filter(|_| self.kind.has_time());

            date.or(time)
        }

        /// The value the valid texts make, or `None` while a field holds
        /// invalid text.
        fn combined(&self, state: &EntryState) -> Option<Value> {
            if self.error_of(state).is_some() {
                return None;
            }

            Some(match self.kind {
                Kind::Date => (state.date.valid, None),
                Kind::Time => (None, state.time.valid),
                Kind::DateTime => match state.date.valid {
                    Some(date) => (Some(date), Some(state.time.valid.unwrap_or(Time::MIDNIGHT))),
                    None => (None, None),
                },
            })
        }

        /// The message of a new value, when the valid texts make one that
        /// differs from the value of the application.
        fn value_message(&self, state: &EntryState) -> Option<Message> {
            let next = self.combined(state)?;

            if next == self.value {
                return None;
            }

            self.on_change.as_ref().map(|on_change| on_change(next))
        }

        /// Publishes a change of the error and a new value.
        fn report(&self, state: &mut EntryState, effects: &mut Effects<Message>) {
            let error = self.error_of(state);

            if error != state.reported_error {
                state.reported_error = error;

                if let Some(on_error) = &self.on_error {
                    effects.publish(on_error(error.map(String::from)));
                }
            }

            if let Some(message) = self.value_message(state) {
                effects.publish(message);
            }
        }

        /// The message that requests the popover of `part` open, or every
        /// popover closed with `None`.
        fn open_message(&self, part: Option<Part>) -> Option<Message> {
            self.on_open_change
                .as_ref()
                .filter(|_| self.enabled())
                .map(|on_open_change| on_open_change(part))
        }

        /// Handles a signal of the host.
        fn handle(
            &self,
            state: &mut EntryState,
            signal: Signal<'_, EntryEvent>,
            effects: &mut Effects<Message>,
        ) {
            match signal {
                Signal::Own(EntryEvent::Input(part, text)) => {
                    if !self.enabled() {
                        return;
                    }

                    match part {
                        Part::Date => state.date.input(&self.date, text),
                        Part::Time => state.time.input(&self.time, text),
                    }

                    self.report(state, effects);
                    effects.rebuild();
                }
                Signal::Own(EntryEvent::Submit(part) | EntryEvent::Blur(part)) => {
                    let field_shows = match part {
                        Part::Date => &mut state.date.show_error,
                        Part::Time => &mut state.time.show_error,
                    };

                    if !*field_shows {
                        *field_shows = true;
                        effects.rebuild();
                    }
                }
                Signal::Before(..) if state.pending => {
                    state.pending = false;
                    self.report(state, effects);
                }
                _ => {}
            }
        }

        /// Syncs the fields with the value of the application and applies
        /// the picked values.
        fn reconcile(&self, state: &mut EntryState) -> Reconcile {
            let (date, time) = self.value;
            let mut rebuild = false;

            if self.kind.has_date() {
                rebuild |= state.date.sync(&self.date, Some(date), None);
            }

            if self.kind.has_time() {
                let (external, empty) = match self.kind {
                    Kind::DateTime => (date.map(|_| time), Some(Time::MIDNIGHT)),
                    _ => (Some(time), None),
                };

                rebuild |= state.time.sync(&self.time, external, empty);
            }

            if self.error_of(state) != state.reported_error {
                state.pending = true;
            }

            Reconcile {
                rebuild,
                focus: None,
            }
        }

        /// The text input of `part`, with the field box around it.
        fn control(&self, state: &EntryState, part: Part) -> Element<'a, Msg<Message, EntryEvent>> {
            let enabled = self.enabled();
            let (draft, error, id) = match part {
                Part::Date => (&state.date.draft, state.date.visible_error(), &self.date_id),
                Part::Time => (&state.time.draft, state.time.visible_error(), &self.time_id),
            };
            let placeholder = match part {
                Part::Date => self.date.localizations.material.date_help,
                Part::Time => "",
            };

            let mut input = text_input(placeholder, draft);

            if enabled {
                input = input
                    .on_input(move |text| Msg::Own(EntryEvent::Input(part, text)))
                    .on_submit(Msg::Own(EntryEvent::Submit(part)));
            }

            if let Some(id) = id {
                input = input.id(id.clone());
            }

            let height = density::control_height();
            let vertical = vertical_padding();
            let toggle = self.open_message(if self.open == Some(part) {
                None
            } else {
                Some(part)
            });

            let input = input
                .padding(Padding {
                    top: vertical,
                    bottom: vertical,
                    left: spacing::MD,
                    right: if toggle.is_some() {
                        height
                    } else {
                        spacing::MD
                    },
                })
                .size(VALUE_STYLE.size)
                .line_height(LineHeight::Absolute(VALUE_STYLE.line_height.into()))
                .width(Length::Fill)
                .class(if error.is_some() {
                    InputClass::Error
                } else {
                    InputClass::Default
                });

            let input: Element<'a, Msg<Message, EntryEvent>> = if enabled {
                IcedElement::new(FocusWatch {
                    content: input.into(),
                    on_blur: Msg::Own(EntryEvent::Blur(part)),
                })
            } else {
                controls::exclude_focus(input)
            };

            let mut decorations = Row::new()
                .height(Length::Fixed(height))
                .push(Space::new().width(Length::Fill));

            if let Some(toggle) = toggle {
                let strings = self.date.localizations.material;
                let trailing = match part {
                    Part::Date => {
                        button::icon_button(lucide::CALENDAR).tooltip(strings.select_date)
                    }
                    Part::Time => button::icon_button(lucide::CLOCK).tooltip(strings.select_time),
                };

                decorations = decorations.push(
                    container(trailing.on_press(Msg::App(toggle)))
                        .center_x(Length::Fixed(height))
                        .center_y(Length::Fixed(height)),
                );
            }

            Stack::new()
                .push(input)
                .push(decorations)
                .width(Length::Fill)
                .into()
        }

        /// The calendar of the date field.
        fn calendar_panel(
            self: &Rc<Self>,
            state: &EntryState,
        ) -> Element<'a, Msg<Message, EntryEvent>> {
            let selected = self.date.parse(&state.date.draft).ok().flatten();
            let spec = Rc::clone(self);
            let snapshot = state.clone();
            let mut panel = calendar(self.today)
                .limits(self.date.first, self.date.last)
                .selected(selected)
                .autofocus(true)
                .localizations(&self.date.localizations);

            if let Some(selectable) = self.date.selectable.clone() {
                panel = panel.selectable(move |date| selectable(date));
            }

            panel
                .on_pick(false, move |pick| {
                    let Pick::Day(date) = pick else {
                        return Vec::new();
                    };
                    let mut next = snapshot.clone();
                    next.date.set(&spec.date, date);
                    snapshot.date.picked.set(Some(date));

                    spec.value_message(&next)
                        .into_iter()
                        .chain(spec.open_message(None))
                        .map(Msg::App)
                        .collect()
                })
                .into()
        }

        /// The hour and minute entries of the time field, and the AM/PM
        /// choice without the 24-hour format.
        fn time_panel(
            self: &Rc<Self>,
            state: &EntryState,
        ) -> Element<'a, Msg<Message, EntryEvent>> {
            let rules = &self.time;
            let current = rules
                .parse(&state.time.draft)
                .ok()
                .flatten()
                .or(state.time.valid)
                .unwrap_or(Time::MIDNIGHT);
            // The 12-hour layout follows the format of the field: a time after
            // noon written differently than in the 24-hour format.
            let twelve = Time::new(13, 0).is_some_and(|afternoon| {
                rules.format(afternoon) != rules.localizations.time_of_day(afternoon, true)
            });
            let (hour, minute) = (current.hour(), current.minute());
            let pm = hour >= 12;

            // `remember` keeps the picked time for the next layout, so the
            // text shows it even when no new value is published; only the
            // number entries, which call their closure on a change alone,
            // remember it.
            let pick = {
                let spec = Rc::clone(self);
                let snapshot = state.clone();

                move |time: Option<Time>, remember: bool| -> Msg<Message, EntryEvent> {
                    let Some(time) = time else {
                        return Msg::Own(EntryEvent::Nothing);
                    };
                    let mut next = snapshot.clone();
                    next.time.set(&spec.time, time);

                    if remember {
                        snapshot.time.picked.set(Some(time));
                    }

                    spec.value_message(&next)
                        .map_or(Msg::Own(EntryEvent::Nothing), Msg::App)
                }
            };
            let pick = Rc::new(pick);

            let hour_entry = {
                let pick = Rc::clone(&pick);
                let (shown, min, max) = if twelve {
                    (
                        f64::from(match hour % 12 {
                            0 => 12,
                            h => h,
                        }),
                        1.0,
                        12.0,
                    )
                } else {
                    (f64::from(hour), 0.0, 23.0)
                };

                number_entry::number_entry(&rules.localizations, shown)
                    .range(min, max)
                    .width(NUMBER_WIDTH)
                    .on_change(move |value| {
                        let value = value as u32;
                        let hour = if twelve {
                            value % 12 + if pm { 12 } else { 0 }
                        } else {
                            value
                        };

                        pick(Time::new(hour, minute), true)
                    })
            };

            let minute_entry = {
                let pick = Rc::clone(&pick);

                number_entry::number_entry(&rules.localizations, f64::from(minute))
                    .range(0.0, 59.0)
                    .width(NUMBER_WIDTH)
                    .on_change(move |value| pick(Time::new(hour, value as u32), true))
            };

            let mut row = Row::new()
                .spacing(spacing::XS)
                .align_y(Alignment::Center)
                .push(hour_entry)
                .push(styled_text(":", VALUE_STYLE))
                .push(minute_entry);

            if twelve {
                let strings = rules.localizations.material;
                let pick = Rc::clone(&pick);

                row = row.push(
                    container(
                        segment_group::segment_group(
                            [
                                segment_group::segment(false, strings.am),
                                segment_group::segment(true, strings.pm),
                            ],
                            Some(pm),
                        )
                        .on_change(move |to_pm| {
                            let hour = hour % 12 + if to_pm { 12 } else { 0 };

                            pick(Time::new(hour, minute), false)
                        }),
                    )
                    .padding(Padding::ZERO.left(spacing::SM)),
                );
            }

            row.into()
        }

        /// The label, field, popover and error of `part`.
        fn part_view(
            self: &Rc<Self>,
            state: &EntryState,
            part: Part,
        ) -> Element<'a, Msg<Message, EntryEvent>> {
            let strings = self.date.localizations.material;
            let (label, error) = match part {
                Part::Date => (
                    self.date_label
                        .clone()
                        .unwrap_or(Cow::Borrowed(strings.date_input)),
                    state.date.visible_error(),
                ),
                Part::Time => (
                    self.time_label
                        .clone()
                        .unwrap_or(Cow::Borrowed(strings.time_input)),
                    state.time.visible_error(),
                ),
            };

            let control = self.control(state, part);
            let field: Element<'a, Msg<Message, EntryEvent>> = if self.open_message(None).is_some()
            {
                let panel = match part {
                    Part::Date => self.calendar_panel(state),
                    Part::Time => self.time_panel(state),
                };
                let mut anchor = popover(control, panel)
                    .open(self.open == Some(part))
                    .padding(spacing::SM);

                if let Some(close) = self.open_message(None) {
                    anchor = anchor.on_dismiss(Msg::App(close));
                }

                anchor.into()
            } else {
                control
            };

            let mut column = Column::new()
                .spacing(spacing::XS)
                .push(styled_text(label, typography::BODY_MEDIUM).class(TextClass::Variant))
                .push(field);

            if let Some(error) = error {
                column = column.push(
                    container(styled_text(error, typography::BODY_SMALL).class(TextClass::Error))
                        .padding([0.0, spacing::MD]),
                );
            }

            column.into()
        }

        /// The whole entry.
        fn view(self: &Rc<Self>, state: &EntryState) -> Element<'a, Msg<Message, EntryEvent>> {
            let content: Element<'a, Msg<Message, EntryEvent>> = match self.kind {
                Kind::Date => self.part_view(state, Part::Date),
                Kind::Time => self.part_view(state, Part::Time),
                Kind::DateTime => Row::new()
                    .spacing(spacing::SM)
                    .align_y(Alignment::Start)
                    .push(
                        container(self.part_view(state, Part::Date)).width(Length::FillPortion(3)),
                    )
                    .push(
                        container(self.part_view(state, Part::Time)).width(Length::FillPortion(2)),
                    )
                    .into(),
            };

            container(content).width(self.width).into()
        }

        /// The element of the entry.
        fn build(self) -> Element<'a, Message> {
            let spec = Rc::new(self);
            let view_spec = Rc::clone(&spec);
            let handler_spec = Rc::clone(&spec);

            Host::new(
                move |state: &EntryState| view_spec.view(state),
                move |state, signal, effects| handler_spec.handle(state, signal, effects),
            )
            .reconcile(move |state, _| spec.reconcile(state))
            .into()
        }
    }

    /// A date entry. See [`date_entry`].
    pub struct DateEntry<'a, Message> {
        spec: Spec<'a, Message>,
    }

    /// A date entry showing `value`, whose calendar marks `today` and opens
    /// on its month when the field is empty.
    ///
    /// The field takes the compact date format of the [`Localizations`]
    /// (`mm/dd/yyyy` in English, `dd/mm/aaaa` in Portuguese), shows it as a
    /// hint while empty and is labeled "Enter Date" unless
    /// [`DateEntry::label`] says otherwise. A year typed with one or two
    /// digits is expanded around `today`, as in
    /// [`Localizations::parse_date`]. Without [`DateEntry::on_change`] it is
    /// disabled.
    pub fn date_entry<'a, Message: Clone + 'a>(
        today: Date,
        value: Option<Date>,
    ) -> DateEntry<'a, Message> {
        DateEntry {
            spec: Spec::new(Kind::Date, today, (value, None)),
        }
    }

    impl<'a, Message: Clone + 'a> DateEntry<'a, Message> {
        /// Sets the function that turns each new valid date, or `None` for
        /// an emptied field, into a message.
        #[must_use]
        pub fn on_change(mut self, on_change: impl Fn(Option<Date>) -> Message + 'a) -> Self {
            self.spec.on_change = Some(Box::new(move |(date, _)| on_change(date)));
            self
        }

        /// Sets the function that turns the error message, when the text
        /// stops parsing, or `None`, when it parses again, into a message.
        #[must_use]
        pub fn on_error_change(
            mut self,
            on_error_change: impl Fn(Option<String>) -> Message + 'a,
        ) -> Self {
            self.spec.on_error = Some(Box::new(on_error_change));
            self
        }

        /// Sets the earliest and the latest accepted date; a date outside
        /// shows the "Out of range." error.
        #[must_use]
        pub fn limits(mut self, first: Date, last: Date) -> Self {
            self.spec.date.first = first.min(last);
            self.spec.date.last = first.max(last);
            self
        }

        /// Sets the function that decides which dates within the limits
        /// are accepted.
        #[must_use]
        pub fn selectable(mut self, selectable: impl Fn(Date) -> bool + 'a) -> Self {
            self.spec.date.selectable = Some(Rc::new(selectable));
            self
        }

        /// Sets whether an empty field is valid (true by default).
        #[must_use]
        pub fn accept_empty(mut self, accept: bool) -> Self {
            self.spec.date.accept_empty = accept;
            self
        }

        /// Sets the label above the field.
        #[must_use]
        pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
            self.spec.date_label = Some(label.into_fragment());
            self
        }

        /// Shows or hides the calendar.
        #[must_use]
        pub fn open(mut self, open: bool) -> Self {
            self.spec.open = open.then_some(Part::Date);
            self
        }

        /// Sets the function that turns the requested open state of the
        /// calendar into a message; without it the calendar button is
        /// hidden.
        #[must_use]
        pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
            self.spec.on_open_change = Some(Box::new(move |part| on_open_change(part.is_some())));
            self
        }

        /// Sets the strings and the date format (English by default).
        #[must_use]
        pub fn localizations(mut self, localizations: &Localizations) -> Self {
            self.spec.localizations(localizations);
            self
        }

        /// Sets the widget [`Id`] of the text input.
        #[must_use]
        pub fn id(mut self, id: impl Into<Id>) -> Self {
            self.spec.date_id = Some(id.into());
            self
        }

        /// Sets the width (the available width by default).
        #[must_use]
        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.spec.width = width.into();
            self
        }
    }

    impl<'a, Message: Clone + 'a> From<DateEntry<'a, Message>> for Element<'a, Message> {
        fn from(entry: DateEntry<'a, Message>) -> Self {
            entry.spec.build()
        }
    }

    /// A time entry. See [`time_entry`].
    pub struct TimeEntry<'a, Message> {
        spec: Spec<'a, Message>,
    }

    /// A time entry showing `value`.
    ///
    /// The field shows times as `h:mm AM` in English, or `HH:mm` with
    /// [`TimeEntry::use_24_hour`] and always in Portuguese, and is labeled
    /// "Enter time" unless [`TimeEntry::label`] says otherwise. Without
    /// [`TimeEntry::on_change`] it is disabled.
    pub fn time_entry<'a, Message: Clone + 'a>(value: Option<Time>) -> TimeEntry<'a, Message> {
        TimeEntry {
            spec: Spec::new(Kind::Time, Date::MIN, (None, value)),
        }
    }

    impl<'a, Message: Clone + 'a> TimeEntry<'a, Message> {
        /// Sets the function that turns each new valid time, or `None` for
        /// an emptied field, into a message.
        #[must_use]
        pub fn on_change(mut self, on_change: impl Fn(Option<Time>) -> Message + 'a) -> Self {
            self.spec.on_change = Some(Box::new(move |(_, time)| on_change(time)));
            self
        }

        /// Sets the function that turns the error message, when the text
        /// stops parsing, or `None`, when it parses again, into a message.
        #[must_use]
        pub fn on_error_change(
            mut self,
            on_error_change: impl Fn(Option<String>) -> Message + 'a,
        ) -> Self {
            self.spec.on_error = Some(Box::new(on_error_change));
            self
        }

        /// Sets whether an empty field is valid (true by default).
        #[must_use]
        pub fn accept_empty(mut self, accept: bool) -> Self {
            self.spec.time.accept_empty = accept;
            self
        }

        /// Shows times in the 24-hour format (false by default; Portuguese
        /// always uses it).
        #[must_use]
        pub fn use_24_hour(mut self, use_24_hour: bool) -> Self {
            self.spec.time.use_24_hour = use_24_hour;
            self
        }

        /// Sets the label above the field.
        #[must_use]
        pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
            self.spec.time_label = Some(label.into_fragment());
            self
        }

        /// Shows or hides the hour and minute popover.
        #[must_use]
        pub fn open(mut self, open: bool) -> Self {
            self.spec.open = open.then_some(Part::Time);
            self
        }

        /// Sets the function that turns the requested open state of the
        /// popover into a message; without it the clock button is hidden.
        #[must_use]
        pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
            self.spec.on_open_change = Some(Box::new(move |part| on_open_change(part.is_some())));
            self
        }

        /// Sets the strings and the time format (English by default).
        #[must_use]
        pub fn localizations(mut self, localizations: &Localizations) -> Self {
            self.spec.localizations(localizations);
            self
        }

        /// Sets the widget [`Id`] of the text input.
        #[must_use]
        pub fn id(mut self, id: impl Into<Id>) -> Self {
            self.spec.time_id = Some(id.into());
            self
        }

        /// Sets the width (the available width by default).
        #[must_use]
        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.spec.width = width.into();
            self
        }
    }

    impl<'a, Message: Clone + 'a> From<TimeEntry<'a, Message>> for Element<'a, Message> {
        fn from(entry: TimeEntry<'a, Message>) -> Self {
            entry.spec.build()
        }
    }

    /// A date entry and a time entry side by side. See [`date_time_entry`].
    pub struct DateTimeEntry<'a, Message> {
        spec: Spec<'a, Message>,
    }

    /// A date entry and a time entry side by side, showing `value` and
    /// reporting one combined value; an empty time means midnight. The
    /// calendar marks `today`.
    ///
    /// A new value is published only while both fields parse, and `None`
    /// without a date. Without [`DateTimeEntry::on_change`] both fields are
    /// disabled.
    pub fn date_time_entry<'a, Message: Clone + 'a>(
        today: Date,
        value: Option<(Date, Time)>,
    ) -> DateTimeEntry<'a, Message> {
        let value = (value.map(|(date, _)| date), value.map(|(_, time)| time));

        DateTimeEntry {
            spec: Spec::new(Kind::DateTime, today, value),
        }
    }

    impl<'a, Message: Clone + 'a> DateTimeEntry<'a, Message> {
        /// Sets the function that turns each new combined value, or `None`
        /// without a date, into a message.
        #[must_use]
        pub fn on_change(
            mut self,
            on_change: impl Fn(Option<(Date, Time)>) -> Message + 'a,
        ) -> Self {
            self.spec.on_change = Some(Box::new(move |(date, time)| {
                on_change(date.map(|date| (date, time.unwrap_or(Time::MIDNIGHT))))
            }));
            self
        }

        /// Sets the function that turns the error of the date field, or
        /// else of the time field, into a message when either stops
        /// parsing, and `None` when both parse.
        #[must_use]
        pub fn on_error_change(
            mut self,
            on_error_change: impl Fn(Option<String>) -> Message + 'a,
        ) -> Self {
            self.spec.on_error = Some(Box::new(on_error_change));
            self
        }

        /// Sets the earliest and the latest accepted date.
        #[must_use]
        pub fn limits(mut self, first: Date, last: Date) -> Self {
            self.spec.date.first = first.min(last);
            self.spec.date.last = first.max(last);
            self
        }

        /// Shows times in the 24-hour format (false by default; Portuguese
        /// always uses it).
        #[must_use]
        pub fn use_24_hour(mut self, use_24_hour: bool) -> Self {
            self.spec.time.use_24_hour = use_24_hour;
            self
        }

        /// Sets the label above the date field ("Enter Date" by default).
        #[must_use]
        pub fn date_label(mut self, label: impl IntoFragment<'a>) -> Self {
            self.spec.date_label = Some(label.into_fragment());
            self
        }

        /// Sets the label above the time field ("Enter time" by default).
        #[must_use]
        pub fn time_label(mut self, label: impl IntoFragment<'a>) -> Self {
            self.spec.time_label = Some(label.into_fragment());
            self
        }

        /// Shows the popover of a part, or none.
        #[must_use]
        pub fn open(mut self, open: Option<Part>) -> Self {
            self.spec.open = open;
            self
        }

        /// Sets the function that turns the requested popover, or `None`
        /// to close it, into a message; without it the picker buttons are
        /// hidden.
        #[must_use]
        pub fn on_open_change(
            mut self,
            on_open_change: impl Fn(Option<Part>) -> Message + 'a,
        ) -> Self {
            self.spec.on_open_change = Some(Box::new(on_open_change));
            self
        }

        /// Sets the strings and the formats (English by default).
        #[must_use]
        pub fn localizations(mut self, localizations: &Localizations) -> Self {
            self.spec.localizations(localizations);
            self
        }

        /// Sets the widget [`Id`] of the date text input.
        #[must_use]
        pub fn date_id(mut self, id: impl Into<Id>) -> Self {
            self.spec.date_id = Some(id.into());
            self
        }

        /// Sets the widget [`Id`] of the time text input.
        #[must_use]
        pub fn time_id(mut self, id: impl Into<Id>) -> Self {
            self.spec.time_id = Some(id.into());
            self
        }

        /// Sets the width (the available width by default).
        #[must_use]
        pub fn width(mut self, width: impl Into<Length>) -> Self {
            self.spec.width = width.into();
            self
        }
    }

    impl<'a, Message: Clone + 'a> From<DateTimeEntry<'a, Message>> for Element<'a, Message> {
        fn from(entry: DateTimeEntry<'a, Message>) -> Self {
            entry.spec.build()
        }
    }

    /// Publishes `on_blur` when the focus leaves `content`, observed on the
    /// next event after it moves.
    struct FocusWatch<'a, Message> {
        content: Element<'a, Message>,
        on_blur: Message,
    }

    impl<Message: Clone> Widget<Message, Theme, iced::Renderer> for FocusWatch<'_, Message> {
        fn tag(&self) -> tree::Tag {
            tree::Tag::of::<bool>()
        }

        fn state(&self) -> tree::State {
            tree::State::new(false)
        }

        fn children(&self) -> Vec<Tree> {
            vec![Tree::new(&self.content)]
        }

        fn diff(&self, tree: &mut Tree) {
            tree.diff_children(std::slice::from_ref(&self.content));
        }

        fn size(&self) -> Size<Length> {
            self.content.as_widget().size()
        }

        fn size_hint(&self) -> Size<Length> {
            self.content.as_widget().size_hint()
        }

        fn layout(
            &mut self,
            tree: &mut Tree,
            renderer: &iced::Renderer,
            limits: &layout::Limits,
        ) -> layout::Node {
            self.content
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, limits)
        }

        fn operate(
            &mut self,
            tree: &mut Tree,
            layout: Layout<'_>,
            renderer: &iced::Renderer,
            operation: &mut dyn Operation,
        ) {
            self.content.as_widget_mut().operate(
                &mut tree.children[0],
                layout,
                renderer,
                operation,
            );
        }

        fn update(
            &mut self,
            tree: &mut Tree,
            event: &Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &iced::Renderer,
            clipboard: &mut dyn Clipboard,
            shell: &mut Shell<'_, Message>,
            viewport: &Rectangle,
        ) {
            let Tree {
                state, children, ..
            } = tree;
            let Some(child) = children.first_mut() else {
                return;
            };

            self.content.as_widget_mut().update(
                child, event, layout, cursor, renderer, clipboard, shell, viewport,
            );

            let focused = crate::focus::run(
                &mut self.content,
                child,
                layout,
                renderer,
                crate::focus::operations::has_focus(),
            )
            .unwrap_or(false);
            let was_focused = state.downcast_mut::<bool>();

            if *was_focused && !focused {
                shell.publish(self.on_blur.clone());
            }

            *was_focused = focused;
        }

        fn draw(
            &self,
            tree: &Tree,
            renderer: &mut iced::Renderer,
            theme: &Theme,
            style: &renderer::Style,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
        ) {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }

        fn mouse_interaction(
            &self,
            tree: &Tree,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            viewport: &Rectangle,
            renderer: &iced::Renderer,
        ) -> mouse::Interaction {
            self.content.as_widget().mouse_interaction(
                &tree.children[0],
                layout,
                cursor,
                viewport,
                renderer,
            )
        }

        fn overlay<'b>(
            &'b mut self,
            tree: &'b mut Tree,
            layout: Layout<'b>,
            renderer: &iced::Renderer,
            viewport: &Rectangle,
            translation: Vector,
        ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
            self.content.as_widget_mut().overlay(
                &mut tree.children[0],
                layout,
                renderer,
                viewport,
                translation,
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        fn rules() -> DateRules<'static> {
            DateRules {
                localizations: Localizations::en(),
                reference: Date::new(2026, 11, 17).expect("date"),
                first: Date::new(2020, 1, 1).expect("date"),
                last: Date::new(2030, 12, 31).expect("date"),
                selectable: None,
                accept_empty: true,
            }
        }

        #[test]
        fn dates_parse_with_format_and_range_errors() {
            let rules = rules();
            let strings = rules.localizations.material;

            assert_eq!(rules.parse(" "), Ok(None));
            assert_eq!(rules.parse("03/15/2026"), Ok(Date::new(2026, 3, 15)));
            assert_eq!(rules.parse("03/15/26"), Ok(Date::new(2026, 3, 15)));
            assert_eq!(rules.parse("15/45/2026"), Err(strings.invalid_date_format));
            assert_eq!(rules.parse("01/01/2040"), Err(strings.date_out_of_range));
            assert_eq!(
                DateRules {
                    accept_empty: false,
                    ..rules
                }
                .parse(""),
                Err(strings.invalid_date_format)
            );
        }

        #[test]
        fn a_new_value_replaces_the_text_unless_it_reads_as_it() {
            let rules = rules();
            let mut field = Field::<Date>::default();
            let march = Date::new(2026, 3, 1);

            assert!(field.sync(&rules, Some(march), None));
            assert_eq!(field.draft, "03/01/2026");

            field.input(&rules, "3/2/2026".to_owned());
            assert!(!field.sync(&rules, Some(Date::new(2026, 3, 2)), None));
            assert_eq!(field.draft, "3/2/2026");

            assert!(field.sync(&rules, Some(Date::new(2026, 4, 1)), None));
            assert_eq!(field.draft, "04/01/2026");
        }
    }
}

/// One part of a [`segmented_entry`]. Built by [`segment`].
#[derive(Debug, Clone)]
pub struct Segment<'a> {
    length: usize,
    hint: Option<Cow<'a, str>>,
    numeric: bool,
    id: Option<Id>,
}

/// A segment that holds up to `length` digits.
pub fn segment<'a>(length: usize) -> Segment<'a> {
    Segment {
        length: length.max(1),
        hint: None,
        numeric: true,
        id: None,
    }
}

impl<'a> Segment<'a> {
    /// Sets the placeholder, such as `000`.
    #[must_use]
    pub fn hint(mut self, hint: impl IntoFragment<'a>) -> Self {
        self.hint = Some(hint.into_fragment());
        self
    }

    /// Sets whether only digits are accepted (true by default).
    #[must_use]
    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }

    /// Sets the widget [`Id`] of the text input of the segment.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// A segmented entry. See [`segmented_entry`].
pub struct SegmentedEntry<'a, Message> {
    segments: Vec<Segment<'a>>,
    values: Vec<String>,
    delimiter: String,
    on_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    direction: ReadingDirection,
}

/// A row of short fields, one per segment, showing `values` (missing
/// values are empty), joined by `.` unless [`SegmentedEntry::delimiter`]
/// says otherwise. Each field is as wide as its digits and as tall as the
/// text fields. Without [`SegmentedEntry::on_change`] it is disabled.
pub fn segmented_entry<'a, Message>(
    segments: impl IntoIterator<Item = Segment<'a>>,
    values: &[String],
) -> SegmentedEntry<'a, Message> {
    let segments: Vec<Segment<'a>> = segments.into_iter().collect();
    let values = (0..segments.len())
        .map(|index| values.get(index).cloned().unwrap_or_default())
        .collect();

    SegmentedEntry {
        segments,
        values,
        delimiter: ".".to_owned(),
        on_change: None,
        direction: ReadingDirection::default(),
    }
}

impl<'a, Message> SegmentedEntry<'a, Message> {
    /// Sets the text drawn between segments; typing its first visible
    /// character, such as `-` for `" - "`, moves to the next segment.
    #[must_use]
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.delimiter = delimiter.into();
        self
    }

    /// Sets the function that turns the text of every segment into a
    /// message, called whenever one changes.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Vec<String>) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the reading direction, which orders the segments and the arrow
    /// keys that move between them.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

/// The character that moves to the next segment: the first non-blank
/// character of `delimiter`, or its first character when it is blank.
fn advance_character(delimiter: &str) -> Option<char> {
    delimiter
        .trim()
        .chars()
        .next()
        .or_else(|| delimiter.chars().next())
}

/// `text` with the characters a segment refuses removed and cut to its
/// length.
fn filter_segment(text: &str, segment: &Segment<'_>, advance: Option<char>) -> String {
    text.chars()
        .filter(|c| {
            if segment.numeric {
                c.is_ascii_digit()
            } else {
                Some(*c) != advance
            }
        })
        .take(segment.length)
        .collect()
}

/// The width of a segment field that fits `length` digits, plus the
/// padding.
fn segment_width<R: text::Renderer<Font = iced::Font>>(length: usize) -> f32 {
    let digits = "0".repeat(length);
    let width = R::Paragraph::with_text(text::Text {
        content: digits.as_str(),
        bounds: Size::INFINITE,
        size: Pixels(VALUE_STYLE.size),
        line_height: text::LineHeight::Absolute(Pixels(VALUE_STYLE.line_height)),
        font: crate::font::inter(VALUE_STYLE.weight),
        align_x: text::Alignment::Left,
        align_y: alignment::Vertical::Top,
        shaping: text::Shaping::Advanced,
        wrapping: text::Wrapping::None,
    })
    .min_bounds()
    .width;

    width.ceil() + spacing::MD * 2.0 + 2.0
}

/// An edit of a segment.
#[derive(Debug, Clone)]
struct SegmentInput(usize, String);

/// Where the caret goes in a segment that takes the focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Caret {
    Start,
    End,
}

/// The fields of a [`SegmentedEntry`] and the focus moves between them.
struct Segments<'a, Message> {
    row: Element<'a, SegmentInput>,
    segments: Vec<Segment<'a>>,
    values: Vec<String>,
    advance: Option<char>,
    on_change: Option<Box<dyn Fn(Vec<String>) -> Message + 'a>>,
    direction: ReadingDirection,
}

impl<Message> Segments<'_, Message> {
    /// The state of the text input of segment `index`, whose child is
    /// counted from the other end of the row in right-to-left.
    fn input_state<'t>(
        &self,
        row: &'t mut Tree,
        index: usize,
    ) -> Option<&'t mut text_input::State<Paragraph>> {
        let child = if self.direction == ReadingDirection::RightToLeft {
            self.segments.len().checked_sub(index + 1)?
        } else {
            index
        };

        row.children
            .get_mut(child * 2)
            .map(|tree| tree.state.downcast_mut::<text_input::State<Paragraph>>())
    }

    /// The segment whose input has the focus.
    fn focused(&self, row: &mut Tree) -> Option<usize> {
        (0..self.segments.len()).find(|index| {
            self.input_state(row, *index)
                .is_some_and(|state| state.is_focused())
        })
    }

    /// Where the caret of segment `index` is: at the start, at the end,
    /// both for an empty segment, or neither.
    fn caret(&self, row: &mut Tree, index: usize) -> (bool, bool) {
        let value = Value::new(&self.values[index]);

        match self
            .input_state(row, index)
            .map(|state| state.cursor().state(&value))
        {
            Some(cursor::State::Index(at)) => (at == 0, at >= value.len()),
            _ => (false, false),
        }
    }

    /// Focuses segment `index`, if any, with the caret at `caret`, and
    /// unfocuses the others.
    fn focus(&self, row: &mut Tree, index: usize, caret: Caret) {
        if index >= self.segments.len() {
            return;
        }

        for other in 0..self.segments.len() {
            if let Some(state) = self.input_state(row, other) {
                if other == index {
                    state.focus();

                    if caret == Caret::Start {
                        state.move_cursor_to_front();
                    }
                } else {
                    state.unfocus();
                }
            }
        }
    }

    /// Handles a key press in the focused segment `index`; returns whether
    /// the key moved the focus.
    fn key(&self, row: &mut Tree, index: usize, key: &keyboard::Key, typed: Option<&str>) -> bool {
        let last = self.segments.len() - 1;
        let (back, forward) = self.direction.arrows();
        let (at_start, at_end) = self.caret(row, index);

        let target = match key {
            keyboard::Key::Named(key::Named::Backspace)
                if self.values[index].is_empty() && index > 0 =>
            {
                Some((index - 1, Caret::End))
            }
            keyboard::Key::Named(named) if *named == back && at_start && index > 0 => {
                Some((index - 1, Caret::End))
            }
            keyboard::Key::Named(named) if *named == forward && at_end && index < last => {
                Some((index + 1, Caret::Start))
            }
            _ if index < last
                && self.advance.is_some()
                && typed.and_then(|text| text.chars().next()) == self.advance =>
            {
                Some((index + 1, Caret::End))
            }
            _ => None,
        };

        match target {
            Some((target, caret)) => {
                self.focus(row, target, caret);
                true
            }
            None => false,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Segments<'_, Message> {
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.row)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.row));
    }

    fn size(&self) -> Size<Length> {
        self.row.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.row.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.row
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        self.row
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        let enabled = self.on_change.is_some();
        let row = &mut tree.children[0];

        if enabled
            && let Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                modifiers,
                text,
                ..
            }) = event
            && !modifiers.control()
            && !modifiers.alt()
            && !modifiers.logo()
            && let Some(index) = self.focused(row)
            && self.key(row, index, key, text.as_deref())
        {
            shell.capture_event();
            shell.request_redraw();
            return;
        }

        let mut inputs = Vec::new();
        {
            let mut local = Shell::new(&mut inputs);

            self.row.as_widget_mut().update(
                row, event, layout, cursor, renderer, clipboard, &mut local, viewport,
            );

            absorb(shell, &local);
        }

        let Some(on_change) = &self.on_change else {
            return;
        };

        for SegmentInput(index, text) in inputs {
            let Some(segment) = self.segments.get(index) else {
                continue;
            };
            let filtered = filter_segment(&text, segment, self.advance);

            if filtered == self.values[index] {
                continue;
            }

            let full = filtered.chars().count() >= segment.length;
            self.values[index] = filtered;
            shell.publish(on_change(self.values.clone()));

            if full {
                self.focus(row, index + 1, Caret::End);
                shell.request_redraw();
            }
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.row.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        self.row.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        _tree: &'b mut Tree,
        _layout: Layout<'b>,
        _renderer: &iced::Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        None
    }
}

impl<'a, Message: 'a> From<SegmentedEntry<'a, Message>> for Element<'a, Message> {
    fn from(entry: SegmentedEntry<'a, Message>) -> Self {
        let SegmentedEntry {
            segments,
            values,
            delimiter,
            on_change,
            direction,
        } = entry;

        let enabled = on_change.is_some();
        let vertical = vertical_padding();
        let mut children: Vec<Element<'a, SegmentInput>> = Vec::new();

        for (index, (segment, value)) in segments.iter().zip(&values).enumerate() {
            if index > 0 {
                children.push(
                    container(styled_text(delimiter.clone(), VALUE_STYLE))
                        .padding([0.0, spacing::XS])
                        .into(),
                );
            }

            let mut input =
                iced::widget::text_input(segment.hint.as_deref().unwrap_or_default(), value)
                    .padding(Padding {
                        top: vertical,
                        bottom: vertical,
                        left: spacing::MD,
                        right: spacing::MD,
                    })
                    .size(VALUE_STYLE.size)
                    .line_height(LineHeight::Absolute(VALUE_STYLE.line_height.into()))
                    .align_x(alignment::Horizontal::Center)
                    .width(Length::Fixed(segment_width::<iced::Renderer>(
                        segment.length,
                    )));

            if enabled {
                input = input.on_input(move |text| SegmentInput(index, text));
            }

            if let Some(id) = &segment.id {
                input = input.id(id.clone());
            }

            children.push(if enabled {
                input.into()
            } else {
                super::controls::exclude_focus(input)
            });
        }

        if direction == ReadingDirection::RightToLeft {
            children.reverse();
        }

        let row = Row::from_vec(children).align_y(Alignment::Center).into();
        let advance = advance_character(&delimiter);

        Element::new(Segments {
            row,
            segments,
            values,
            advance,
            on_change,
            direction,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_advance_character_is_the_first_visible_one() {
        assert_eq!(advance_character(" - "), Some('-'));
        assert_eq!(advance_character("."), Some('.'));
        assert_eq!(advance_character(" "), Some(' '));
        assert_eq!(advance_character(""), None);
    }

    #[test]
    fn segments_keep_digits_up_to_their_length() {
        let digits = segment(3);
        let text = segment(4).numeric(false);

        assert_eq!(filter_segment("1a2b3c4", &digits, Some('.')), "123");
        assert_eq!(filter_segment("ab-cd-e", &text, Some('-')), "abcd");
    }
}
