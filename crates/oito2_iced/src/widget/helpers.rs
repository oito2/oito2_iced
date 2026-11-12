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

//! Small helpers shared by the widgets: double-click detection, reading
//! direction, check box and radio state transitions, and motion durations.

use std::time::{Duration, Instant};

use iced::keyboard::key::Named;

/// The longest time between two clicks on the same target that still makes
/// a double click.
pub const DOUBLE_CLICK_TIMEOUT: Duration = Duration::from_millis(300);

/// Tells a double click from two single clicks on the same target, without
/// delaying the single clicks.
///
/// A click completes a double click when it lands on the same target as the
/// previous one less than [`DOUBLE_CLICK_TIMEOUT`] after it. The caller
/// passes the time of each click, so the detector never reads the clock.
#[derive(Debug, Clone, Default)]
pub struct DoubleClick<T> {
    pending: Option<(T, Instant)>,
}

impl<T: PartialEq> DoubleClick<T> {
    /// A detector without a pending click.
    pub fn new() -> Self {
        Self { pending: None }
    }

    /// Records a click on `target` at `now` and returns whether it completes
    /// a double click. A completed double click leaves no pending click.
    pub fn click(&mut self, target: T, now: Instant) -> bool {
        if let Some((pending, at)) = &self.pending
            && *pending == target
            && now.saturating_duration_since(*at) < DOUBLE_CLICK_TIMEOUT
        {
            self.pending = None;
            return true;
        }

        self.pending = Some((target, now));
        false
    }

    /// Forgets the pending click.
    pub fn cancel(&mut self) {
        self.pending = None;
    }
}

/// The reading direction of the text and of directional layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ReadingDirection {
    /// Left to right.
    #[default]
    LeftToRight,
    /// Right to left.
    RightToLeft,
}

/// The side a chevron or an arrow points to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    /// Points left.
    Left,
    /// Points right.
    Right,
}

impl ReadingDirection {
    /// The side a chevron points to: toward the end of the reading direction
    /// when `forward`, toward its start otherwise.
    pub fn chevron(self, forward: bool) -> Side {
        if forward == (self == ReadingDirection::LeftToRight) {
            Side::Right
        } else {
            Side::Left
        }
    }

    /// The arrow keys that move toward the start (`back`) and the end
    /// (`forward`) of the reading direction, as `(back, forward)`.
    pub fn arrows(self) -> (Named, Named) {
        match self {
            ReadingDirection::LeftToRight => (Named::ArrowLeft, Named::ArrowRight),
            ReadingDirection::RightToLeft => (Named::ArrowRight, Named::ArrowLeft),
        }
    }
}

/// The value of a check box after a click: unchecked (`Some(false)`) becomes
/// checked; checked becomes mixed (`None`) when `tristate`, unchecked
/// otherwise; mixed becomes unchecked.
pub fn next_check_value(value: Option<bool>, tristate: bool) -> Option<bool> {
    match value {
        Some(false) => Some(true),
        Some(true) if tristate => None,
        Some(true) | None => Some(false),
    }
}

/// The new value of a radio group after the radio of `value` is activated
/// while the group shows `selected`.
///
/// Returns `Some(Some(value))` to select it, `Some(None)` to clear the group
/// when it is already selected and `toggleable`, and `None` when nothing
/// changes.
pub fn activate_radio<T: PartialEq>(
    value: T,
    selected: Option<&T>,
    toggleable: bool,
) -> Option<Option<T>> {
    if selected != Some(&value) {
        Some(Some(value))
    } else if toggleable {
        Some(None)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_clicks_on_the_same_target_in_time_make_a_double_click() {
        let mut detector = DoubleClick::new();
        let start = Instant::now();

        assert!(!detector.click("a", start));
        assert!(detector.click("a", start + Duration::from_millis(200)));
        assert!(!detector.click("a", start + Duration::from_millis(250)));
    }

    #[test]
    fn late_clicks_and_other_targets_start_over() {
        let mut detector = DoubleClick::new();
        let start = Instant::now();

        assert!(!detector.click("a", start));
        assert!(!detector.click("a", start + DOUBLE_CLICK_TIMEOUT));
        assert!(!detector.click("b", start + Duration::from_millis(350)));
        assert!(detector.click("b", start + Duration::from_millis(400)));
    }

    #[test]
    fn chevrons_and_arrows_follow_the_reading_direction() {
        assert_eq!(ReadingDirection::LeftToRight.chevron(true), Side::Right);
        assert_eq!(ReadingDirection::RightToLeft.chevron(true), Side::Left);
        assert_eq!(ReadingDirection::RightToLeft.chevron(false), Side::Right);
        assert_eq!(
            ReadingDirection::RightToLeft.arrows(),
            (Named::ArrowRight, Named::ArrowLeft)
        );
    }

    #[test]
    fn check_values_cycle() {
        assert_eq!(next_check_value(Some(false), false), Some(true));
        assert_eq!(next_check_value(Some(true), false), Some(false));
        assert_eq!(next_check_value(Some(true), true), None);
        assert_eq!(next_check_value(None, true), Some(false));
    }

    #[test]
    fn radios_select_and_toggle() {
        assert_eq!(activate_radio(1, Some(&2), false), Some(Some(1)));
        assert_eq!(activate_radio(1, None, false), Some(Some(1)));
        assert_eq!(activate_radio(1, Some(&1), false), None);
        assert_eq!(activate_radio(1, Some(&1), true), Some(None));
    }
}
