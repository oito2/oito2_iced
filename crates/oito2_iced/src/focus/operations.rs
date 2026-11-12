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

//! Focus operations beyond the ones Iced provides: focus the first or the
//! last focusable widget, move the focus without wrapping at the ends, and
//! ask whether any widget has the focus.
//!
//! Run them with [`iced::advanced::widget::operate`] or from a widget's
//! `operate`; they visit every focusable widget below the element they run
//! on, in layout order.

use iced::Rectangle;
use iced::advanced::widget::Id;
use iced::advanced::widget::operation::focusable::{Count, count};
use iced::advanced::widget::operation::{self, Focusable, Operation, Outcome};

/// Focuses the focusable widget at `target` (in layout order) and
/// unfocuses every other one.
fn focus_index<T>(target: usize) -> impl Operation<T> {
    struct FocusIndex {
        target: usize,
        current: usize,
    }

    impl<T> Operation<T> for FocusIndex {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            if self.current == self.target {
                state.focus();
            } else {
                state.unfocus();
            }

            self.current += 1;
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }
    }

    FocusIndex { target, current: 0 }
}

/// Runs `focus_index` with the index chosen from the focus count, or does
/// nothing when `choose` returns `None`.
fn focus_chosen<T>(choose: fn(Count) -> Option<usize>) -> impl Operation<T>
where
    T: Send + 'static,
{
    struct Chosen<T> {
        count: Option<Box<dyn Operation<Count>>>,
        choose: fn(Count) -> Option<usize>,
        _marker: std::marker::PhantomData<fn() -> T>,
    }

    impl<T> Operation<T> for Chosen<T>
    where
        T: Send + 'static,
    {
        fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, state: &mut dyn Focusable) {
            if let Some(count) = self.count.as_mut() {
                count.focusable(id, bounds, state);
            }
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
            operate(self);
        }

        fn finish(&self) -> Outcome<T> {
            let Some(count) = self.count.as_ref() else {
                return Outcome::None;
            };

            match count.finish() {
                Outcome::Some(count) => match (self.choose)(count) {
                    Some(target) => Outcome::Chain(Box::new(focus_index::<T>(target))),
                    None => Outcome::None,
                },
                Outcome::None | Outcome::Chain(_) => Outcome::None,
            }
        }
    }

    Chosen {
        count: Some(Box::new(count())),
        choose,
        _marker: std::marker::PhantomData,
    }
}

/// Focuses the first focusable widget.
pub fn focus_first<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    focus_chosen(|count| (count.total > 0).then_some(0))
}

/// Focuses the last focusable widget.
pub fn focus_last<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    focus_chosen(|count| count.total.checked_sub(1))
}

/// Moves the focus to the next focusable widget, staying on the last one
/// at the end. Focuses the first widget when none has the focus.
pub fn focus_next_clamped<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    focus_chosen(|count| match count.focused {
        Some(focused) if focused + 1 < count.total => Some(focused + 1),
        Some(_) => None,
        None => (count.total > 0).then_some(0),
    })
}

/// Moves the focus to the previous focusable widget, staying on the first
/// one at the start. Focuses the last widget when none has the focus.
pub fn focus_previous_clamped<T>() -> impl Operation<T>
where
    T: Send + 'static,
{
    focus_chosen(|count| match count.focused {
        Some(focused) => focused.checked_sub(1),
        None => count.total.checked_sub(1),
    })
}

/// Whether any focusable widget has the focus.
pub fn has_focus() -> impl Operation<bool> {
    struct HasFocus {
        found: bool,
    }

    impl Operation<bool> for HasFocus {
        fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
            self.found |= state.is_focused();
        }

        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<bool>)) {
            operate(self);
        }

        fn finish(&self) -> Outcome<bool> {
            Outcome::Some(self.found)
        }
    }

    HasFocus { found: false }
}

pub use operation::focusable::{find_focused, focus, focus_next, focus_previous, unfocus};
