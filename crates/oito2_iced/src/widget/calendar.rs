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

//! Calendars and date pickers: a month grid that picks a date or a range
//! of dates, and a field that opens it in a popover.
//!
//! [`calendar`] shows a month with a heading (the month and year, and the
//! previous and next month buttons), the narrow weekday names from the
//! week start of the [`Localizations`], and optionally an ISO 8601 week
//! number column. Today, passed by the application since there is no time
//! zone support, is outlined; the selected day is filled; a range shows a
//! band between its ends.
//!
//! The grid is one stop in keyboard navigation: the arrow keys move the
//! focused day by a day or a week, Home and End to the start and the end of
//! its week, Page Up and Page Down by a month (a year with Shift), and
//! Enter or Space picks it. A click picks the day under the pointer. A
//! range takes two picks, in any order; until the second, the band
//! previews the range up to the hovered or focused day. Days outside
//! [`Calendar::limits`] or refused by [`Calendar::selectable`] can't be
//! picked.
//!
//! [`date_picker`] is a field showing the compact date (or the range) that
//! opens a calendar in a [popover](super::popover()); the calendar takes
//! the focus and a pick closes it. Enter, Space or Alt+Down opens it from
//! the keyboard.
//!
//! The selection and the open state live in the application; the shown
//! month, the focused day and a range in progress live in the widget.
//!
//! ```no_run
//! use oito2_iced::date::Date;
//! use oito2_iced::widget::calendar;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Pick(Date),
//!     PickerOpen(bool),
//! }
//!
//! # fn view<'a>(today: Date, day: Option<Date>, open: bool) -> oito2_iced::Element<'a, Message> {
//! iced::widget::column![
//!     calendar::calendar(today)
//!         .selected(day)
//!         .show_week_numbers(true)
//!         .on_select(Message::Pick),
//!     calendar::date_picker(today)
//!         .value(day)
//!         .label("Due date")
//!         .on_change(Message::Pick)
//!         .open(open)
//!         .on_open_change(Message::PickerOpen),
//! ]
//! .into()
//! # }
//! ```

#![cfg(feature = "svg")]

use std::borrow::Cow;
use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer, text};
use iced::font::Weight;
use iced::keyboard::{self, key};
use iced::widget::text::IntoFragment;
use iced::widget::{Column, Row, button as iced_button, container};
use iced::{
    Alignment, Background, Border, Color, Element as IcedElement, Event, Length, Padding, Pixels,
    Rectangle, Size, Vector, alignment, mouse, touch,
};

use super::helpers::{ReadingDirection, Side};
use super::search::host::{Effects, Host, Msg, Reconcile, Signal};
use super::{button, focusable, popover, styled_text};
use crate::date::Date;
use crate::icon::{icon, lucide};
use crate::l10n::Localizations;
use crate::theme::text::Class as TextClass;
use crate::theme::{
    DISABLED_CONTAINER_OPACITY, DISABLED_CONTENT_OPACITY, HOVER_OPACITY, PRESSED_OPACITY,
    TRANSPARENT, state_layer,
};
use crate::tokens::typography::{self, TextStyle};
use crate::tokens::{radius, sizes, spacing};
use crate::{Element, Theme, density};

/// The height of the weekday heading row, as a part of the cell side.
const HEADING_RATIO: f32 = 0.75;

/// A range of days, from `start` to `end` included, with `start` never
/// after `end`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DateRange {
    /// The first day.
    pub start: Date,
    /// The last day.
    pub end: Date,
}

impl DateRange {
    /// The range between `a` and `b`, in either order.
    #[must_use]
    pub fn new(a: Date, b: Date) -> Self {
        Self {
            start: a.min(b),
            end: a.max(b),
        }
    }

    /// Whether `day` is in the range.
    #[must_use]
    pub fn contains(self, day: Date) -> bool {
        self.start <= day && day <= self.end
    }
}

/// A pick made in a calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Pick {
    /// A day, in single mode.
    Day(Date),
    /// A completed range, in range mode.
    Range(DateRange),
}

/// Turns a pick into the messages to publish.
pub(crate) type OnPick<'a, Message> = Rc<dyn Fn(Pick) -> Vec<Message> + 'a>;

/// Decides whether a day can be picked.
type Predicate<'a> = Rc<dyn Fn(Date) -> bool + 'a>;

/// A month calendar. See [`calendar`].
pub struct Calendar<'a, Message> {
    today: Date,
    first: Date,
    last: Date,
    selected: Option<Date>,
    range: Option<DateRange>,
    range_mode: bool,
    on_pick: Option<OnPick<'a, Message>>,
    initial_month: Option<Date>,
    selectable: Option<Predicate<'a>>,
    week_numbers: bool,
    autofocus: bool,
    localizations: Localizations,
    direction: ReadingDirection,
    id: Option<Id>,
}

/// A calendar that marks `today` and opens on the month of the selection,
/// or of `today`. Without [`Calendar::on_select`] or
/// [`Calendar::on_range_select`] no day can be picked and the grid is
/// skipped by Tab; the month buttons still browse.
pub fn calendar<'a, Message>(today: Date) -> Calendar<'a, Message> {
    Calendar {
        today,
        first: Date::MIN,
        last: Date::MAX,
        selected: None,
        range: None,
        range_mode: false,
        on_pick: None,
        initial_month: None,
        selectable: None,
        week_numbers: false,
        autofocus: false,
        localizations: Localizations::default(),
        direction: ReadingDirection::default(),
        id: None,
    }
}

impl<'a, Message> Calendar<'a, Message> {
    /// Sets the earliest and the latest day that can be picked
    /// ([`Date::MIN`] and [`Date::MAX`] by default). The month buttons stop
    /// at their months.
    #[must_use]
    pub fn limits(mut self, first: Date, last: Date) -> Self {
        self.first = first.min(last);
        self.last = first.max(last);
        self
    }

    /// Sets the selected day, in single mode.
    #[must_use]
    pub fn selected(mut self, day: Option<Date>) -> Self {
        self.selected = day;
        self
    }

    /// Sets the function that turns a picked day into a message, in single
    /// mode.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(Date) -> Message + 'a) -> Self {
        self.range_mode = false;
        self.on_pick = Some(Rc::new(move |pick| match pick {
            Pick::Day(day) => vec![on_select(day)],
            Pick::Range(_) => Vec::new(),
        }));
        self
    }

    /// Sets the selected range, shown as a band, in range mode.
    #[must_use]
    pub fn selected_range(mut self, range: Option<DateRange>) -> Self {
        self.range = range;
        self
    }

    /// Sets the function that turns a completed range into a message and
    /// turns on range mode: the first pick starts a range, the second
    /// completes it.
    #[must_use]
    pub fn on_range_select(mut self, on_range_select: impl Fn(DateRange) -> Message + 'a) -> Self {
        self.range_mode = true;
        self.on_pick = Some(Rc::new(move |pick| match pick {
            Pick::Range(range) => vec![on_range_select(range)],
            Pick::Day(_) => Vec::new(),
        }));
        self
    }

    /// Sets the messages of a pick, in range mode when `range_mode`.
    pub(crate) fn on_pick(
        mut self,
        range_mode: bool,
        on_pick: impl Fn(Pick) -> Vec<Message> + 'a,
    ) -> Self {
        self.range_mode = range_mode;
        self.on_pick = Some(Rc::new(on_pick));
        self
    }

    /// Sets the month shown first, instead of the month of the selection
    /// or of today.
    #[must_use]
    pub fn initial_month(mut self, month: Date) -> Self {
        self.initial_month = Some(month);
        self
    }

    /// Sets the function that decides which days within the limits can be
    /// picked; every day can by default.
    #[must_use]
    pub fn selectable(mut self, selectable: impl Fn(Date) -> bool + 'a) -> Self {
        self.selectable = Some(Rc::new(selectable));
        self
    }

    /// Sets whether a column of ISO 8601 week numbers shows before the days
    /// (false by default).
    #[must_use]
    pub fn show_week_numbers(mut self, show: bool) -> Self {
        self.week_numbers = show;
        self
    }

    /// Sets whether the grid comes first in the focus order, so a popover
    /// that focuses its first element on open focuses the grid (false by
    /// default: the month buttons come first).
    #[must_use]
    pub fn autofocus(mut self, autofocus: bool) -> Self {
        self.autofocus = autofocus;
        self
    }

    /// Sets the strings, the weekday names and the week start (English by
    /// default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the reading direction, which mirrors the grid, the month
    /// buttons and the left and right arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the widget [`Id`] of the grid, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// The properties of a calendar, shared by its view, handler and
/// reconciler.
struct Config<'a, Message> {
    today: Date,
    first: Date,
    last: Date,
    selected: Option<Date>,
    range: Option<DateRange>,
    range_mode: bool,
    on_pick: Option<OnPick<'a, Message>>,
    initial_month: Option<Date>,
    selectable: Option<Predicate<'a>>,
    week_numbers: bool,
    autofocus: bool,
    localizations: Localizations,
    direction: ReadingDirection,
    id: Option<Id>,
}

impl<Message> Config<'_, Message> {
    /// Whether days can be picked.
    fn enabled(&self) -> bool {
        self.on_pick.is_some()
    }

    /// Whether the reading direction is right to left.
    fn rtl(&self) -> bool {
        self.direction == ReadingDirection::RightToLeft
    }

    /// `day` moved within the limits.
    fn clamp(&self, day: Date) -> Date {
        day.max(self.first).min(self.last)
    }

    /// Whether `day` is within the limits and accepted by the predicate.
    fn can_pick(&self, day: Date) -> bool {
        self.first <= day
            && day <= self.last
            && self.selectable.as_ref().is_none_or(|accepts| accepts(day))
    }

    /// Whether a month before `month` holds pickable days.
    fn can_go_back(&self, month: Date) -> bool {
        month.first_of_month() > self.first.first_of_month()
    }

    /// Whether a month after `month` holds pickable days.
    fn can_go_forward(&self, month: Date) -> bool {
        month.first_of_month() < self.last.first_of_month()
    }

    /// The day the selection starts on, if any.
    fn anchor(&self) -> Option<Date> {
        self.selected.or(self.range.map(|range| range.start))
    }

    /// The column of the first day of `month`, from the week start.
    fn offset(&self, month: Date) -> u32 {
        (month.first_of_month().weekday() + 7 - self.localizations.first_day_of_week() % 7) % 7
    }
}

/// The own events of a calendar.
#[derive(Debug, Clone, Copy)]
enum CalendarEvent {
    /// A month button: the month before (-1) or after (1).
    Month(i32),
    /// A key moved the focused day.
    Focus(Date),
    /// A day was picked.
    Pick(Date),
    /// The pointer rests on a day, or left the grid.
    Hover(Option<Date>),
}

/// The state of a calendar in the widget tree.
#[derive(Debug)]
struct CalendarState {
    ready: bool,
    month: Date,
    focused: Date,
    range_start: Option<Date>,
    hovered: Option<Date>,
    anchor: Option<Date>,
    limits: (Date, Date),
}

impl Default for CalendarState {
    fn default() -> Self {
        Self {
            ready: false,
            month: Date::MIN,
            focused: Date::MIN,
            range_start: None,
            hovered: None,
            anchor: None,
            limits: (Date::MIN, Date::MAX),
        }
    }
}

impl<Message> Config<'_, Message> {
    /// Handles an own event of the calendar.
    fn handle(
        &self,
        state: &mut CalendarState,
        signal: Signal<'_, CalendarEvent>,
        effects: &mut Effects<Message>,
    ) {
        let Signal::Own(event) = signal else {
            return;
        };

        match event {
            CalendarEvent::Month(delta) => {
                let allowed = if delta < 0 {
                    self.can_go_back(state.month)
                } else {
                    self.can_go_forward(state.month)
                };

                if let Some(month) = state.month.add_months(delta).filter(|_| allowed) {
                    let day = state.focused.day().min(month.days_in_month());
                    let focused = Date::new(month.year(), month.month(), day).unwrap_or(month);

                    state.month = month.first_of_month();
                    state.focused = self.clamp(focused);
                    effects.rebuild();
                }
            }
            CalendarEvent::Focus(day) => {
                let target = self.clamp(day);

                state.focused = target;
                state.month = target.first_of_month();
                effects.rebuild();
            }
            CalendarEvent::Pick(day) => {
                let Some(on_pick) = &self.on_pick else {
                    return;
                };

                if !self.can_pick(day) {
                    return;
                }

                state.focused = day;

                let pick = if !self.range_mode {
                    Some(Pick::Day(day))
                } else if let Some(start) = state.range_start.take() {
                    state.hovered = None;
                    Some(Pick::Range(DateRange::new(start, day)))
                } else {
                    state.range_start = Some(day);
                    None
                };

                if let Some(pick) = pick {
                    for message in on_pick(pick) {
                        effects.publish(message);
                    }
                }

                effects.rebuild();
            }
            CalendarEvent::Hover(day) => {
                if state.range_start.is_some() && state.hovered != day {
                    state.hovered = day;
                    effects.rebuild();
                }
            }
        }
    }

    /// Sets the shown month and the focused day on first sight, and follows
    /// a selection or limits changed from outside: the focused day moves to
    /// the new selection, within the limits, and its month shows.
    fn reconcile(&self, state: &mut CalendarState) -> Reconcile {
        let anchor = self.anchor();
        let limits = (self.first, self.last);

        if !state.ready {
            let today = self.clamp(self.today);

            state.ready = true;
            state.focused = anchor.unwrap_or(today);
            state.month = self
                .initial_month
                .or(anchor)
                .unwrap_or(today)
                .first_of_month();
            state.anchor = anchor;
            state.limits = limits;

            return Reconcile {
                rebuild: true,
                focus: None,
            };
        }

        let mut focused = state.focused;

        if let Some(day) = anchor
            && state.anchor != anchor
        {
            focused = day;
        }
        state.anchor = anchor;

        if state.limits != limits {
            state.limits = limits;
            focused = self.clamp(focused);
        }

        let rebuild = focused != state.focused;

        if rebuild {
            state.focused = focused;
            state.month = focused.first_of_month();
        }

        Reconcile {
            rebuild,
            focus: None,
        }
    }
}

/// The icon of a month button pointing to the end of the reading
/// direction when `forward`, to its start otherwise.
fn chevron(direction: ReadingDirection, forward: bool) -> crate::icon::Icon {
    match direction.chevron(forward) {
        Side::Left => lucide::CHEVRON_LEFT,
        Side::Right => lucide::CHEVRON_RIGHT,
    }
}

/// The heading of a calendar: the month and year, and the month buttons.
fn heading<'a, Message: Clone + 'a>(
    config: &Config<'a, Message>,
    month: Date,
) -> Element<'a, Msg<Message, CalendarEvent>> {
    let strings = &config.localizations;
    let rtl = config.rtl();

    let title = container(styled_text(
        strings.month_year(month),
        typography::TITLE_SMALL,
    ))
    .padding(if rtl {
        Padding::ZERO.right(spacing::SM)
    } else {
        Padding::ZERO.left(spacing::SM)
    })
    .width(Length::Fill)
    .align_x(if rtl {
        alignment::Horizontal::Right
    } else {
        alignment::Horizontal::Left
    });

    let previous = button::icon_button(chevron(config.direction, false))
        .tooltip(strings.material.previous_month)
        .on_press_maybe(
            config
                .can_go_back(month)
                .then_some(Msg::Own(CalendarEvent::Month(-1))),
        );
    let next = button::icon_button(chevron(config.direction, true))
        .tooltip(strings.material.next_month)
        .on_press_maybe(
            config
                .can_go_forward(month)
                .then_some(Msg::Own(CalendarEvent::Month(1))),
        );

    let mut parts: Vec<Element<'a, Msg<Message, CalendarEvent>>> =
        vec![title.into(), previous.into(), next.into()];

    if rtl {
        parts.reverse();
    }

    Row::from_vec(parts)
        .align_y(Alignment::Center)
        .width(Length::Fill)
        .into()
}

impl<'a, Message: Clone + 'a> From<Calendar<'a, Message>> for Element<'a, Message> {
    fn from(calendar: Calendar<'a, Message>) -> Self {
        let range_mode = calendar.range_mode
            || (calendar.on_pick.is_none()
                && calendar.range.is_some()
                && calendar.selected.is_none());

        let config = Rc::new(Config {
            today: calendar.today,
            first: calendar.first,
            last: calendar.last,
            selected: calendar.selected,
            range: calendar.range,
            range_mode,
            on_pick: calendar.on_pick,
            initial_month: calendar.initial_month,
            selectable: calendar.selectable,
            week_numbers: calendar.week_numbers,
            autofocus: calendar.autofocus,
            localizations: calendar.localizations,
            direction: calendar.direction,
            id: calendar.id,
        });

        let view_config = Rc::clone(&config);
        let handler_config = Rc::clone(&config);

        let view = move |state: &CalendarState| -> Element<'a, Msg<Message, CalendarEvent>> {
            IcedElement::new(Grid {
                heading: heading(&view_config, state.month),
                config: Rc::clone(&view_config),
                month: state.month,
                focused: state.focused,
                range_start: state.range_start,
                hovered: state.hovered,
            })
        };

        Host::new(view, move |state, signal, effects| {
            handler_config.handle(state, signal, effects);
        })
        .reconcile(move |state, _| config.reconcile(state))
        .into()
    }
}

/// The heading and the day grid of a calendar, which take the pointer and
/// the keyboard.
struct Grid<'a, Message> {
    heading: Element<'a, Msg<Message, CalendarEvent>>,
    config: Rc<Config<'a, Message>>,
    month: Date,
    focused: Date,
    range_start: Option<Date>,
    hovered: Option<Date>,
}

/// The interaction state of a [`Grid`].
#[derive(Debug, Clone, Copy, Default)]
struct GridState {
    focused: bool,
    /// Whether the focus came from the keyboard, which shows the focus ring
    /// on the focused day.
    keyboard: bool,
    /// The day under a press that has not been released yet.
    pressed: Option<Date>,
}

impl operation::Focusable for GridState {
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

/// Where a day sits in the shown selection.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct DayMarks {
    selected: bool,
    in_range: bool,
    start: bool,
    end: bool,
}

/// The side of a day cell: the control height of the density plus two.
fn cell_side() -> f32 {
    density::control_height() + 2.0
}

impl<Message> Grid<'_, Message> {
    /// The number of columns: the days and the week numbers.
    fn columns(&self) -> usize {
        7 + usize::from(self.config.week_numbers)
    }

    /// The number of week rows of the month.
    fn rows(&self) -> u32 {
        (self.config.offset(self.month) + self.month.days_in_month()).div_ceil(7)
    }

    /// The area below the heading: the weekday names and the days.
    fn body(&self, bounds: Rectangle, heading: f32) -> Rectangle {
        Rectangle {
            y: bounds.y + heading,
            height: bounds.height - heading,
            ..bounds
        }
    }

    /// The bounds of the cell at `row` (0 is the weekday names) and the
    /// logical `column` (0 is the week numbers when shown), mirrored in
    /// right-to-left.
    fn cell(&self, body: Rectangle, row: u32, column: usize) -> Rectangle {
        let side = cell_side();
        let visual = if self.config.rtl() {
            self.columns() - 1 - column
        } else {
            column
        };
        let heading = side * HEADING_RATIO;
        let (y, height) = if row == 0 {
            (body.y, heading)
        } else {
            (body.y + heading + side * (row - 1) as f32, side)
        };

        Rectangle {
            x: body.x + side * visual as f32,
            y,
            width: side,
            height,
        }
    }

    /// The day of the cell at the week `row` (from 0) and `day_column`
    /// (from 0, at the week start), if the cell holds a day of the month.
    fn day_of(&self, row: u32, day_column: u32) -> Option<Date> {
        let number = i64::from(row * 7 + day_column) - i64::from(self.config.offset(self.month));

        if number < 0 || number >= i64::from(self.month.days_in_month()) {
            return None;
        }

        self.month.first_of_month().add_days(number)
    }

    /// The day under `point`, if any.
    fn day_at(&self, body: Rectangle, point: iced::Point) -> Option<Date> {
        let side = cell_side();
        let top = body.y + side * HEADING_RATIO;

        if !body.contains(point) || point.y < top {
            return None;
        }

        let row = ((point.y - top) / side).floor() as u32;
        let visual = ((point.x - body.x) / side).floor() as usize;

        if visual >= self.columns() {
            return None;
        }

        let column = if self.config.rtl() {
            self.columns() - 1 - visual
        } else {
            visual
        };
        let day_column = column.checked_sub(usize::from(self.config.week_numbers))?;

        self.day_of(row, u32::try_from(day_column).ok()?)
    }

    /// Where `day` sits in the selection: a range in progress previews up
    /// to the hovered day, or to the focused day while the grid has the
    /// focus.
    fn marks(&self, day: Date, grid_focused: bool) -> DayMarks {
        let config = &self.config;

        if !config.range_mode {
            return DayMarks {
                selected: config.selected == Some(day),
                ..DayMarks::default()
            };
        }

        let range = match self.range_start {
            Some(pending) => {
                let other = self
                    .hovered
                    .or(grid_focused.then_some(self.focused))
                    .unwrap_or(pending);
                Some(DateRange::new(pending, other))
            }
            None => config.range,
        };

        let Some(range) = range else {
            return DayMarks::default();
        };

        let start = day == range.start;
        let end = day == range.end;

        DayMarks {
            selected: start || end,
            in_range: range.contains(day),
            start,
            end,
        }
    }

    /// The day the keyboard `key` moves the focus to, with Shift held when
    /// `shift`.
    fn target(&self, key: key::Named, shift: bool) -> Option<Date> {
        let focused = self.focused;
        let first_day = self.config.localizations.first_day_of_week() % 7;
        let week_start = i64::from((focused.weekday() + 7 - first_day) % 7);
        let back = if self.config.rtl() { 1 } else { -1 };

        match key {
            key::Named::ArrowLeft => focused.add_days(back),
            key::Named::ArrowRight => focused.add_days(-back),
            key::Named::ArrowUp => focused.add_days(-7),
            key::Named::ArrowDown => focused.add_days(7),
            key::Named::Home => focused.add_days(-week_start),
            key::Named::End => focused.add_days(6 - week_start),
            key::Named::PageUp => focused.add_months(if shift { -12 } else { -1 }),
            key::Named::PageDown => focused.add_months(if shift { 12 } else { 1 }),
            _ => None,
        }
    }
}

/// Draws `content` centered in `bounds` in the text `style` at `weight`.
fn draw_label(
    renderer: &mut iced::Renderer,
    content: &str,
    style: TextStyle,
    weight: Weight,
    bounds: Rectangle,
    color: Color,
) {
    use iced::advanced::text::Renderer as _;

    renderer.fill_text(
        text::Text {
            content: content.to_owned(),
            bounds: Size::new(bounds.width + 1.0, bounds.height),
            size: Pixels(style.size),
            line_height: text::LineHeight::Absolute(Pixels(style.line_height)),
            font: crate::font::inter(weight),
            align_x: text::Alignment::Center,
            align_y: alignment::Vertical::Center,
            shaping: text::Shaping::Advanced,
            wrapping: text::Wrapping::None,
        },
        bounds.center(),
        color,
        bounds,
    );
}

impl<'a, Message: Clone + 'a> Widget<Msg<Message, CalendarEvent>, Theme, iced::Renderer>
    for Grid<'a, Message>
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<GridState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(GridState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.heading)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.heading));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = cell_side();
        let width = side * self.columns() as f32;
        let heading_limits =
            layout::Limits::new(Size::new(width, 0.0), Size::new(width, f32::INFINITY));
        let heading =
            self.heading
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &heading_limits);
        let height = heading.size().height + side * HEADING_RATIO + side * self.rows() as f32;
        let size = limits.resolve(Length::Shrink, Length::Shrink, Size::new(width, height));

        layout::Node::with_children(size, vec![heading])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<GridState>();
        let Some(heading_layout) = layout.children().next() else {
            return;
        };
        let body = self.body(layout.bounds(), heading_layout.bounds().height);
        let enabled = self.config.enabled();

        if !enabled {
            state.focused = false;
            operation.container(self.config.id.as_ref(), body);
        }

        if enabled && self.config.autofocus {
            operation.focusable(self.config.id.as_ref(), body, state);
        }

        operation.traverse(&mut |operation| {
            self.heading.as_widget_mut().operate(
                &mut children[0],
                heading_layout,
                renderer,
                operation,
            );
        });

        if enabled && !self.config.autofocus {
            operation.focusable(self.config.id.as_ref(), body, state);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &iced::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Msg<Message, CalendarEvent>>,
        viewport: &Rectangle,
    ) {
        let Some(heading_layout) = layout.children().next() else {
            return;
        };

        self.heading.as_widget_mut().update(
            &mut tree.children[0],
            event,
            heading_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        if shell.is_event_captured() {
            return;
        }

        let state = tree.state.downcast_mut::<GridState>();
        let bounds = layout.bounds();
        let body = self.body(bounds, heading_layout.bounds().height);
        let enabled = self.config.enabled();

        if !enabled {
            *state = GridState::default();
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let day = cursor
                    .position_over(body)
                    .and_then(|point| self.day_at(body, point))
                    .filter(|day| enabled && self.config.can_pick(*day));

                if let Some(day) = day {
                    state.pressed = Some(day);
                    state.focused = true;
                    state.keyboard = false;
                    shell.capture_event();
                    shell.request_redraw();
                } else if state.focused && !cursor.is_over(bounds) {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(pressed) = state.pressed.take() {
                    let day = cursor
                        .position_over(body)
                        .and_then(|point| self.day_at(body, point));

                    if day == Some(pressed) {
                        shell.publish(Msg::Own(CalendarEvent::Pick(pressed)));
                    }

                    shell.capture_event();
                }
            }
            Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = None;
            }
            Event::Mouse(mouse::Event::CursorMoved { .. }) if self.range_start.is_some() => {
                match cursor.position_over(body) {
                    Some(point) => {
                        if let Some(day) = self.day_at(body, point)
                            && self.hovered != Some(day)
                        {
                            shell.publish(Msg::Own(CalendarEvent::Hover(Some(day))));
                        }
                    }
                    None => {
                        if self.hovered.is_some() {
                            shell.publish(Msg::Own(CalendarEvent::Hover(None)));
                        }
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key: keyboard::Key::Named(named),
                modifiers,
                ..
            }) if state.focused
                && !modifiers.control()
                && !modifiers.alt()
                && !modifiers.logo() =>
            {
                if matches!(named, key::Named::Enter | key::Named::Space) {
                    shell.publish(Msg::Own(CalendarEvent::Pick(self.focused)));
                } else if let Some(target) = self.target(*named, modifiers.shift()) {
                    shell.publish(Msg::Own(CalendarEvent::Focus(target)));
                } else {
                    return;
                }

                state.keyboard = true;
                shell.capture_event();
                shell.request_redraw();
            }
            _ => {}
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
        use iced::advanced::Renderer as _;

        let Some(heading_layout) = layout.children().next() else {
            return;
        };

        self.heading.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            heading_layout,
            cursor,
            viewport,
        );

        let state = tree.state.downcast_ref::<GridState>();
        let config = &self.config;
        let roles = theme.roles();
        let body = self.body(layout.bounds(), heading_layout.bounds().height);
        let week = usize::from(config.week_numbers);
        let muted = typography::LABEL_SMALL;
        let grid_focused = state.focused && config.enabled();

        let fill = roles.selected_fill();
        let on_fill = if fill == roles.primary {
            roles.on_primary
        } else {
            roles.on_indicator()
        };
        let ring = roles.indicator();

        if config.week_numbers {
            draw_label(
                renderer,
                config.localizations.week_number_heading,
                muted,
                muted.weight,
                self.cell(body, 0, 0),
                roles.on_surface_variant,
            );
        }

        let names = config.localizations.narrow_weekdays();
        let first_day = config.localizations.first_day_of_week() % 7;

        for column in 0..7 {
            let name = names[(first_day as usize + column) % 7];
            draw_label(
                renderer,
                name,
                muted,
                muted.weight,
                self.cell(body, 0, column + week),
                roles.on_surface_variant,
            );
        }

        let rtl = config.rtl();

        for row in 0..self.rows() {
            if config.week_numbers {
                let wednesday = self
                    .month
                    .first_of_month()
                    .add_days(i64::from(row * 7 + 3) - i64::from(config.offset(self.month)));

                if let Some(day) = wednesday {
                    draw_label(
                        renderer,
                        &day.iso_week().to_string(),
                        muted,
                        muted.weight,
                        self.cell(body, row + 1, 0),
                        roles.on_surface_variant,
                    );
                }
            }

            for column in 0..7 {
                let Some(day) = self.day_of(row, column) else {
                    continue;
                };

                let bounds = self.cell(body, row + 1, column as usize + week);
                let marks = self.marks(day, grid_focused);
                let pickable = config.can_pick(day);
                let today = day == config.today;
                let focus_ring = grid_focused && state.keyboard && day == self.focused;

                if marks.in_range && !(marks.start && marks.end) {
                    let half = bounds.width / 2.0;
                    let (skip_left, skip_right) = if rtl {
                        (marks.end, marks.start)
                    } else {
                        (marks.start, marks.end)
                    };

                    for (skip, x) in [(skip_left, bounds.x), (skip_right, bounds.x + half)] {
                        if !skip {
                            renderer.fill_quad(
                                renderer::Quad {
                                    bounds: Rectangle {
                                        x,
                                        width: half,
                                        ..bounds
                                    },
                                    ..renderer::Quad::default()
                                },
                                roles.secondary_container,
                            );
                        }
                    }
                }

                let border = if focus_ring {
                    Border::default()
                        .color(ring)
                        .width(sizes::FOCUS_BORDER_WIDTH)
                } else if today && !marks.selected {
                    Border::default().color(ring).width(1.0)
                } else {
                    Border::default()
                };

                if marks.selected || border.width > 0.0 {
                    renderer.fill_quad(
                        renderer::Quad {
                            bounds,
                            border: border.rounded(radius::MEDIUM),
                            ..renderer::Quad::default()
                        },
                        if marks.selected { fill } else { TRANSPARENT },
                    );
                }

                let color = if !pickable {
                    roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
                } else if marks.selected {
                    on_fill
                } else if marks.in_range {
                    roles.on_secondary_container
                } else {
                    roles.on_surface
                };
                let body_style = typography::BODY_MEDIUM;
                let weight = if today {
                    typography::EMPHASIS_WEIGHT
                } else {
                    body_style.weight
                };

                draw_label(
                    renderer,
                    &day.day().to_string(),
                    body_style,
                    weight,
                    bounds,
                    color,
                );
            }
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let Some(heading_layout) = layout.children().next() else {
            return mouse::Interaction::default();
        };

        let body = self.body(layout.bounds(), heading_layout.bounds().height);

        if self.config.enabled()
            && cursor
                .position_over(body)
                .and_then(|point| self.day_at(body, point))
                .is_some_and(|day| self.config.can_pick(day))
        {
            return mouse::Interaction::Pointer;
        }

        self.heading.as_widget().mouse_interaction(
            &tree.children[0],
            heading_layout,
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
    ) -> Option<overlay::Element<'b, Msg<Message, CalendarEvent>, Theme, iced::Renderer>> {
        let heading_layout = layout.children().next()?;

        self.heading.as_widget_mut().overlay(
            &mut tree.children[0],
            heading_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

/// A closure that turns a value into a message.
type Callback<'a, T, Message> = Rc<dyn Fn(T) -> Message + 'a>;

/// A field that shows a date, or a range, and opens a calendar in a
/// popover. See [`date_picker`].
pub struct DatePicker<'a, Message> {
    today: Date,
    first: Date,
    last: Date,
    value: Option<Date>,
    range: Option<DateRange>,
    on_change: Option<Callback<'a, Date, Message>>,
    on_range_change: Option<Callback<'a, DateRange, Message>>,
    open: bool,
    on_open_change: Option<Callback<'a, bool, Message>>,
    label: Option<Cow<'a, str>>,
    placeholder: Option<Cow<'a, str>>,
    selectable: Option<Predicate<'a>>,
    week_numbers: bool,
    localizations: Localizations,
    direction: ReadingDirection,
    id: Option<Id>,
    width: Length,
}

/// A date picker whose calendar marks `today`.
///
/// The field is as tall as the text fields and shows the value in the
/// compact date format of the [`Localizations`], or the placeholder (the
/// localized date format hint by default). A click, Enter, Space or
/// Alt+Down opens the calendar; a pick publishes the change and closes it.
/// The field is disabled without [`DatePicker::on_change`] or
/// [`DatePicker::on_range_change`], and without
/// [`DatePicker::on_open_change`].
pub fn date_picker<'a, Message>(today: Date) -> DatePicker<'a, Message> {
    DatePicker {
        today,
        first: Date::MIN,
        last: Date::MAX,
        value: None,
        range: None,
        on_change: None,
        on_range_change: None,
        open: false,
        on_open_change: None,
        label: None,
        placeholder: None,
        selectable: None,
        week_numbers: false,
        localizations: Localizations::default(),
        direction: ReadingDirection::default(),
        id: None,
        width: Length::Fill,
    }
}

impl<'a, Message> DatePicker<'a, Message> {
    /// Sets the earliest and the latest day that can be picked.
    #[must_use]
    pub fn limits(mut self, first: Date, last: Date) -> Self {
        self.first = first.min(last);
        self.last = first.max(last);
        self
    }

    /// Sets the picked day, in single mode.
    #[must_use]
    pub fn value(mut self, value: Option<Date>) -> Self {
        self.value = value;
        self
    }

    /// Sets the function that turns a picked day into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(Date) -> Message + 'a) -> Self {
        self.on_change = Some(Rc::new(on_change));
        self
    }

    /// Sets the picked range, in range mode.
    #[must_use]
    pub fn range(mut self, range: Option<DateRange>) -> Self {
        self.range = range;
        self
    }

    /// Sets the function that turns a picked range into a message and turns
    /// on range mode.
    #[must_use]
    pub fn on_range_change(mut self, on_range_change: impl Fn(DateRange) -> Message + 'a) -> Self {
        self.on_range_change = Some(Rc::new(on_range_change));
        self
    }

    /// Shows or hides the calendar.
    #[must_use]
    pub fn open(mut self, open: bool) -> Self {
        self.open = open;
        self
    }

    /// Sets the function that turns the requested open state into a
    /// message: `true` on a click, Enter, Space or Alt+Down while closed;
    /// `false` on a click while open, Escape, a click outside or a pick.
    #[must_use]
    pub fn on_open_change(mut self, on_open_change: impl Fn(bool) -> Message + 'a) -> Self {
        self.on_open_change = Some(Rc::new(on_open_change));
        self
    }

    /// Shows `label` above the field.
    #[must_use]
    pub fn label(mut self, label: impl IntoFragment<'a>) -> Self {
        self.label = Some(label.into_fragment());
        self
    }

    /// Sets the text shown while nothing is picked (the localized date
    /// format hint by default).
    #[must_use]
    pub fn placeholder(mut self, placeholder: impl IntoFragment<'a>) -> Self {
        self.placeholder = Some(placeholder.into_fragment());
        self
    }

    /// Sets the function that decides which days within the limits can be
    /// picked.
    #[must_use]
    pub fn selectable(mut self, selectable: impl Fn(Date) -> bool + 'a) -> Self {
        self.selectable = Some(Rc::new(selectable));
        self
    }

    /// Sets whether the calendar shows week numbers.
    #[must_use]
    pub fn show_week_numbers(mut self, show: bool) -> Self {
        self.week_numbers = show;
        self
    }

    /// Sets the strings and the date format (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets the reading direction of the field and the calendar.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets the widget [`Id`] of the field, used to focus it
    /// programmatically and to find it in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the width of the field (the available width by default).
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

/// The style of the date picker field: the fill and outline of the text
/// fields, with the hover and press overlays of a button.
fn field_style(theme: &Theme, status: iced_button::Status) -> iced_button::Style {
    let roles = theme.roles();
    let background = roles.surface_container_high;

    let (background, text_color, outline) = match status {
        iced_button::Status::Active => (background, roles.on_surface, roles.outline),
        iced_button::Status::Hovered => (
            state_layer(background, roles.on_surface, HOVER_OPACITY),
            roles.on_surface,
            roles.outline,
        ),
        iced_button::Status::Pressed => (
            state_layer(background, roles.on_surface, PRESSED_OPACITY),
            roles.on_surface,
            roles.outline,
        ),
        iced_button::Status::Disabled => (
            background,
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY),
            roles.on_surface.scale_alpha(DISABLED_CONTAINER_OPACITY),
        ),
    };

    iced_button::Style {
        background: Some(Background::Color(background)),
        text_color,
        border: Border::default()
            .color(outline)
            .width(1.0)
            .rounded(radius::MEDIUM),
        ..iced_button::Style::default()
    }
}

impl<'a, Message: Clone + 'a> From<DatePicker<'a, Message>> for Element<'a, Message> {
    fn from(picker: DatePicker<'a, Message>) -> Self {
        let DatePicker {
            today,
            first,
            last,
            value,
            range,
            on_change,
            on_range_change,
            open,
            on_open_change,
            label,
            placeholder,
            selectable,
            week_numbers,
            localizations,
            direction,
            id,
            width,
        } = picker;

        let range_mode = on_range_change.is_some() || (on_change.is_none() && range.is_some());
        let enabled =
            (on_change.is_some() || on_range_change.is_some()) && on_open_change.is_some();
        let open = open && enabled;
        let rtl = direction == ReadingDirection::RightToLeft;

        let display = if range_mode {
            range.map(|range| {
                localizations.date_range(
                    &localizations.compact_date(range.start),
                    &localizations.compact_date(range.end),
                )
            })
        } else {
            value.map(|day| localizations.compact_date(day))
        };

        let request = |value: bool| {
            on_open_change
                .as_ref()
                .filter(|_| enabled)
                .map(|on_open_change| on_open_change(value))
        };
        let toggle = request(!open).map(Msg::App);
        let open_request = if open { None } else { request(true) };
        let close = request(false);

        let height = density::control_height();
        let content: Element<'a, Msg<Message, ()>> = match display {
            Some(text) => styled_text(text, typography::BODY_LARGE).into(),
            None => styled_text(
                placeholder.unwrap_or(Cow::Borrowed(localizations.material.date_help)),
                typography::BODY_LARGE,
            )
            .class(TextClass::Variant)
            .into(),
        };
        let content = container(content)
            .padding(if rtl {
                Padding::ZERO.right(spacing::MD)
            } else {
                Padding::ZERO.left(spacing::MD)
            })
            .width(Length::Fill)
            .align_x(if rtl {
                alignment::Horizontal::Right
            } else {
                alignment::Horizontal::Left
            })
            .center_y(Length::Fixed(height));
        let glyph = container(icon(lucide::CALENDAR))
            .class(crate::theme::container::Class::Custom(Box::new(
                |theme: &Theme| container::Style::default().color(theme.roles().on_surface_variant),
            )))
            .center_x(Length::Fixed(height))
            .center_y(Length::Fixed(height));

        let mut parts: Vec<Element<'a, Msg<Message, ()>>> = vec![content.into(), glyph.into()];
        if rtl {
            parts.reverse();
        }

        let field = iced_button(
            Row::from_vec(parts)
                .width(Length::Fill)
                .height(Length::Fixed(height)),
        )
        .style(field_style)
        .padding(0)
        .width(Length::Fill)
        .on_press_maybe(toggle);

        let mut wrapper = focusable(field).radius(radius::MEDIUM).enabled(enabled);
        if let Some(message) = open_request.clone() {
            wrapper = wrapper.on_activate(Msg::App(message));
        }
        if let Some(id) = id {
            wrapper = wrapper.id(id);
        }

        let on_close = on_open_change.clone();
        let mut panel = calendar(today)
            .limits(first, last)
            .selected(value)
            .selected_range(range)
            .show_week_numbers(week_numbers)
            .autofocus(true)
            .localizations(&localizations)
            .direction(direction);

        if let Some(selectable) = selectable {
            panel.selectable = Some(selectable);
        }

        if enabled {
            panel = panel.on_pick(range_mode, move |pick| {
                let mut messages = Vec::new();

                match pick {
                    Pick::Day(day) => {
                        if let Some(on_change) = &on_change {
                            messages.push(Msg::App(on_change(day)));
                        }
                    }
                    Pick::Range(range) => {
                        if let Some(on_range_change) = &on_range_change {
                            messages.push(Msg::App(on_range_change(range)));
                        }
                    }
                }

                if let Some(on_close) = &on_close {
                    messages.push(Msg::App(on_close(false)));
                }

                messages
            });
        }

        let mut anchor = popover(wrapper, panel)
            .open(open)
            .padding(spacing::SM)
            .direction(direction);
        if let Some(close) = close {
            anchor = anchor.on_dismiss(Msg::App(close));
        }

        let mut column = Column::new().spacing(spacing::XS).width(width);

        if let Some(label) = label {
            column =
                column.push(styled_text(label, typography::BODY_MEDIUM).class(TextClass::Variant));
        }

        column = column.push(anchor);

        Host::fixed(
            column.into(),
            move |_: &mut (), signal, effects: &mut Effects<Message>| {
                if let Signal::After(
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key: keyboard::Key::Named(key::Named::ArrowDown),
                        modifiers,
                        ..
                    }),
                    context,
                ) = signal
                    && *modifiers == keyboard::Modifiers::ALT
                    && context.other_focused
                    && !context.captured
                    && let Some(message) = &open_request
                {
                    effects.publish(message.clone());
                    effects.capture();
                }
            },
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn day(year: i32, month: u32, day: u32) -> Date {
        Date::new(year, month, day).expect("valid date")
    }

    #[test]
    fn ranges_order_their_ends() {
        let range = DateRange::new(day(2026, 3, 20), day(2026, 3, 5));

        assert_eq!(range.start, day(2026, 3, 5));
        assert_eq!(range.end, day(2026, 3, 20));
        assert!(range.contains(day(2026, 3, 13)));
        assert!(!range.contains(day(2026, 3, 21)));
    }
}
