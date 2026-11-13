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

//! Splitters: two or more panes side by side, or stacked, with a
//! draggable divider between each two.
//!
//! A divider keeps both of its panes within their limits. Its handle shows
//! the resize cursor, follows a drag until the button is released and takes
//! part in focus traversal; a focused handle moves by the keyboard step
//! with the arrow keys of its axis, Home and End take the pane before it to
//! its smallest and largest size, and Enter, Space or a double click
//! collapse or restore the collapsible pane next to it. Splitters nest.
//!
//! The sizes live in the application as a [`State`]: the splitter shows
//! the state it is given and publishes the next one through
//! [`Splitter::on_change`]. A new [`State`] lays the panes out from their
//! initial fractions on first layout; afterwards the sizes keep their
//! proportions when the space changes.
//!
//! ```no_run
//! use oito2_iced::widget::splitter;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Split(splitter::State),
//! }
//!
//! # fn view<'a>(state: &'a splitter::State) -> oito2_iced::Element<'a, Message> {
//! splitter::splitter(
//!     state,
//!     vec![
//!         splitter::pane(iced::widget::text("Files")).fraction(0.3).collapsible(true),
//!         splitter::pane(iced::widget::text("Editor")),
//!     ],
//! )
//! .on_change(Message::Split)
//! .into()
//! # }
//! ```

use super::helpers::ReadingDirection;
use super::resize_handle::{DIVIDER, Outcome, Request, Sizing, Split};
use crate::Element;

/// The smallest size of a pane by default, in pixels.
pub const DEFAULT_MIN_SIZE: f32 = 48.0;

/// The divider movement of one arrow key press by default, in pixels.
pub const DEFAULT_KEYBOARD_STEP: f32 = 16.0;

/// Differences in pixels below this are ignored when filling the space.
const EPSILON: f32 = 0.01;

/// The direction panes are laid out in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Axis {
    /// Side by side, from the start of the reading direction.
    #[default]
    Horizontal,
    /// Stacked from the top.
    Vertical,
}

/// One pane of a [`Splitter`]. Built by [`pane`].
pub struct Pane<'a, Message> {
    content: Element<'a, Message>,
    limits: Limits,
}

/// The sizing parameters of a pane.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Limits {
    fraction: Option<f32>,
    min_size: f32,
    max_size: f32,
    collapsible: bool,
    collapsed_size: f32,
    initially_collapsed: bool,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            fraction: None,
            min_size: DEFAULT_MIN_SIZE,
            max_size: f32::INFINITY,
            collapsible: false,
            collapsed_size: 0.0,
            initially_collapsed: false,
        }
    }
}

/// A pane showing `content`, at least [`DEFAULT_MIN_SIZE`] pixels and with
/// no largest size, sharing the space left by the panes with a fraction.
pub fn pane<'a, Message>(content: impl Into<Element<'a, Message>>) -> Pane<'a, Message> {
    Pane {
        content: content.into(),
        limits: Limits::default(),
    }
}

impl<Message> Pane<'_, Message> {
    /// The share of the space at start, from 0 to 1; the panes without one
    /// share what is left equally.
    #[must_use]
    pub fn fraction(mut self, fraction: f32) -> Self {
        self.limits.fraction = Some(fraction.clamp(0.0, 1.0));
        self
    }

    /// The smallest size in pixels; negative values count as zero.
    #[must_use]
    pub fn min_size(mut self, size: f32) -> Self {
        self.limits.min_size = size.max(0.0);
        self
    }

    /// The largest size in pixels; a value below the smallest size counts
    /// as the smallest size.
    #[must_use]
    pub fn max_size(mut self, size: f32) -> Self {
        self.limits.max_size = size;
        self
    }

    /// Whether the pane can collapse to its collapsed size.
    #[must_use]
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.limits.collapsible = collapsible;
        self
    }

    /// The size of the collapsed pane (0 by default, which hides its
    /// content).
    #[must_use]
    pub fn collapsed_size(mut self, size: f32) -> Self {
        self.limits.collapsed_size = size.max(0.0);
        self
    }

    /// Whether a collapsible pane starts collapsed; ignored on a pane that
    /// is not collapsible.
    #[must_use]
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.limits.initially_collapsed = collapsed;
        self
    }
}

impl Limits {
    /// The limits with the largest size at least the smallest one and the
    /// initial collapse only on a collapsible pane.
    fn sanitized(self) -> Self {
        Self {
            max_size: self.max_size.max(self.min_size),
            initially_collapsed: self.initially_collapsed && self.collapsible,
            ..self
        }
    }
}

/// The pane sizes of a [`Splitter`], kept by the application.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct State {
    sizes: Vec<f32>,
    available: f32,
    collapsed: Vec<bool>,
    restore: Vec<Option<f32>>,
}

impl State {
    /// A state that lays the panes out from their initial fractions.
    pub fn new() -> Self {
        Self::default()
    }

    /// The pane sizes in pixels, from the start; empty before the first
    /// change.
    pub fn sizes(&self) -> &[f32] {
        &self.sizes
    }

    /// Whether the pane at `index` is collapsed.
    pub fn is_collapsed(&self, index: usize) -> bool {
        self.collapsed.get(index).copied().unwrap_or(false)
    }

    /// The smallest size of pane `i`: its collapsed size while collapsed.
    fn min(&self, limits: &[Limits], i: usize) -> f32 {
        if self.is_collapsed(i) {
            limits[i].collapsed_size
        } else {
            limits[i].min_size
        }
    }

    /// The largest size of pane `i`: its collapsed size while collapsed.
    fn max(&self, limits: &[Limits], i: usize) -> f32 {
        if self.is_collapsed(i) {
            limits[i].collapsed_size
        } else {
            limits[i].max_size
        }
    }

    /// Makes the state fit `limits` and `available` pixels: starts over when
    /// the number of panes changed, then lays the panes out again when the
    /// space changed.
    fn prepare(&mut self, limits: &[Limits], available: f32) {
        if self.collapsed.len() != limits.len() {
            *self = Self {
                sizes: Vec::new(),
                available: 0.0,
                collapsed: limits.iter().map(|l| l.initially_collapsed).collect(),
                restore: vec![None; limits.len()],
            };
        }

        if self.sizes.len() != limits.len() || self.available != available {
            self.fit(limits, available);
        }
    }

    /// Lays the panes out in `available` pixels, keeping the current
    /// proportions or, at first, the initial fractions.
    fn fit(&mut self, limits: &[Limits], available: f32) {
        if self.sizes.len() == limits.len() && self.available > 0.0 {
            let scale = available / self.available;
            for size in &mut self.sizes {
                *size *= scale;
            }
        } else {
            let used: f32 = limits.iter().filter_map(|l| l.fraction).sum();
            let open = limits.iter().filter(|l| l.fraction.is_none()).count();
            let share = if open == 0 {
                0.0
            } else {
                (1.0 - used).clamp(0.0, 1.0) / open as f32
            };
            self.sizes = limits
                .iter()
                .map(|l| l.fraction.unwrap_or(share) * available)
                .collect();
        }

        self.available = available;

        for (i, limit) in limits.iter().enumerate() {
            if self.is_collapsed(i) {
                self.sizes[i] = limit.collapsed_size;
            }
        }

        self.balance(limits);
    }

    /// Clamps every pane to its limits and gives the difference to the
    /// panes that can take it, from the end, so the sizes fill the space.
    fn balance(&mut self, limits: &[Limits]) {
        for i in 0..self.sizes.len() {
            self.sizes[i] = self.sizes[i].clamp(self.min(limits, i), self.max(limits, i));
        }

        let mut excess = self.available - self.sizes.iter().sum::<f32>();

        for _ in 0..2 {
            for i in (0..self.sizes.len()).rev() {
                if excess.abs() <= EPSILON {
                    return;
                }
                let next = (self.sizes[i] + excess).clamp(self.min(limits, i), self.max(limits, i));
                excess -= next - self.sizes[i];
                self.sizes[i] = next;
            }
        }
    }

    /// Moves the divider after pane `i` by `delta` pixels toward the end,
    /// within the limits of both panes. Returns whether a size changed.
    fn move_divider(&mut self, limits: &[Limits], i: usize, delta: f32) -> bool {
        let (a, b) = (self.sizes[i], self.sizes[i + 1]);
        let total = a + b;
        let (min, max) = (self.min(limits, i), self.max(limits, i));
        let mut next = (a + delta).clamp(min, max);
        let low = total - self.max(limits, i + 1);
        let high = total - self.min(limits, i + 1);
        if low <= high {
            next = next.clamp(low, high);
        }

        if next < min || next > max || next == a {
            return false;
        }

        self.sizes[i] = next;
        self.sizes[i + 1] = total - next;
        true
    }

    /// Collapses or restores the collapsible pane next to divider `i`,
    /// preferring the one before it. Returns whether a pane changed.
    fn toggle(&mut self, limits: &[Limits], i: usize) -> bool {
        let target = if limits[i].collapsible {
            i
        } else if limits[i + 1].collapsible {
            i + 1
        } else {
            return false;
        };
        let neighbor = if target == i { i + 1 } else { i };

        if self.collapsed[target] {
            self.collapsed[target] = false;
            let restore = self.restore[target].unwrap_or(limits[target].min_size);
            let gain = restore - self.sizes[target];
            self.sizes[target] = restore;
            self.sizes[neighbor] -= gain;
        } else {
            self.restore[target] = Some(self.sizes[target]);
            let freed = self.sizes[target] - limits[target].collapsed_size;
            self.collapsed[target] = true;
            self.sizes[target] = limits[target].collapsed_size;
            self.sizes[neighbor] += freed;
        }

        self.balance(limits);
        true
    }
}

/// Panes side by side, or stacked, with a draggable divider between each
/// two. Built by [`splitter`].
pub struct Splitter<'a, Message> {
    state: State,
    panes: Vec<Pane<'a, Message>>,
    axis: Axis,
    direction: ReadingDirection,
    keyboard_step: f32,
    on_change: Option<Box<dyn Fn(State) -> Message + 'a>>,
}

/// A horizontal splitter showing `panes` (two or more) with the sizes of
/// `state`. Without [`Splitter::on_change`] the dividers stay where they
/// are and have no handle.
pub fn splitter<'a, Message>(
    state: &State,
    panes: Vec<Pane<'a, Message>>,
) -> Splitter<'a, Message> {
    Splitter {
        state: state.clone(),
        panes,
        axis: Axis::Horizontal,
        direction: ReadingDirection::default(),
        keyboard_step: DEFAULT_KEYBOARD_STEP,
        on_change: None,
    }
}

impl<'a, Message> Splitter<'a, Message> {
    /// Whether the panes sit side by side or stacked.
    #[must_use]
    pub fn axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// The reading direction: side-by-side panes start on the right in
    /// right-to-left text.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// The divider movement of one arrow key press; defaults to
    /// [`DEFAULT_KEYBOARD_STEP`].
    #[must_use]
    pub fn keyboard_step(mut self, step: f32) -> Self {
        self.keyboard_step = step;
        self
    }

    /// Publishes the next state after a divider moved or a pane collapsed
    /// or was restored.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(State) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

/// The sizing of a [`Splitter`].
struct SplitterSizing<'a, Message> {
    state: State,
    limits: Vec<Limits>,
    on_change: Option<Box<dyn Fn(State) -> Message + 'a>>,
}

impl<Message> Sizing<Message> for SplitterSizing<'_, Message> {
    fn sizes(&mut self, extent: f32) -> Vec<f32> {
        let dividers = self.limits.len().saturating_sub(1) as f32 * DIVIDER;
        self.state
            .prepare(&self.limits, (extent - dividers).max(0.0));
        self.state.sizes.clone()
    }

    fn hidden(&self, index: usize) -> bool {
        self.state.is_collapsed(index)
            && self
                .limits
                .get(index)
                .is_some_and(|limits| limits.collapsed_size == 0.0)
    }

    fn apply(&mut self, divider: usize, request: Request) -> Outcome<Message> {
        let Some(on_change) = &self.on_change else {
            return Outcome::Ignored;
        };
        if divider + 1 >= self.state.sizes.len() {
            return Outcome::Ignored;
        }

        let limits = &self.limits;
        let state = &mut self.state;
        let changed = match request {
            Request::Move(delta) => state.move_divider(limits, divider, delta),
            Request::Toggle => state.toggle(limits, divider),
            Request::Home => {
                let delta = state.min(limits, divider) - state.sizes[divider];
                state.move_divider(limits, divider, delta)
            }
            Request::End => {
                let delta = state.sizes[divider + 1] - state.min(limits, divider + 1);
                state.move_divider(limits, divider, delta)
            }
        };

        if changed {
            Outcome::Changed(on_change(state.clone()))
        } else {
            Outcome::Unchanged
        }
    }
}

impl<'a, Message: 'a> From<Splitter<'a, Message>> for Element<'a, Message> {
    fn from(splitter: Splitter<'a, Message>) -> Self {
        let (children, limits): (Vec<_>, Vec<_>) = splitter
            .panes
            .into_iter()
            .map(|pane| (pane.content, pane.limits.sanitized()))
            .unzip();
        let resizable = splitter.on_change.is_some();

        Split::new(
            children,
            splitter.axis,
            splitter.direction == ReadingDirection::RightToLeft,
            resizable,
            splitter.keyboard_step,
            SplitterSizing {
                state: splitter.state,
                limits,
                on_change: splitter.on_change,
            },
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The limits of three panes: a quarter that collapses with a 100
    /// pixel minimum, a pane sharing the rest, and a pane of 100 or more.
    fn three() -> Vec<Limits> {
        vec![
            Limits {
                fraction: Some(0.25),
                min_size: 100.0,
                collapsible: true,
                ..Limits::default()
            },
            Limits::default(),
            Limits {
                min_size: 100.0,
                ..Limits::default()
            },
        ]
    }

    #[test]
    fn panes_start_from_their_fractions() {
        let mut state = State::new();
        state.prepare(&three(), 600.0);

        assert_eq!(state.sizes(), &[150.0, 225.0, 225.0]);
    }

    #[test]
    fn a_divider_stays_within_the_limits() {
        let limits = three();
        let mut state = State::new();
        state.prepare(&limits, 600.0);

        assert!(state.move_divider(&limits, 0, 30.0));
        assert_eq!(state.sizes()[0], 180.0);
        assert!(state.move_divider(&limits, 0, -200.0));
        assert_eq!(state.sizes()[0], 100.0);
        assert!(!state.move_divider(&limits, 0, -10.0));
        assert!(state.move_divider(&limits, 1, 1000.0));
        assert_eq!(state.sizes()[2], 100.0);
    }

    #[test]
    fn collapsing_gives_the_space_to_the_neighbor_and_restores_it() {
        let limits = three();
        let mut state = State::new();
        state.prepare(&limits, 600.0);
        state.move_divider(&limits, 0, 16.0);

        assert!(state.toggle(&limits, 0));
        assert!(state.is_collapsed(0));
        assert_eq!(state.sizes(), &[0.0, 375.0, 225.0]);

        assert!(state.toggle(&limits, 0));
        assert_eq!(state.sizes(), &[166.0, 209.0, 225.0]);
    }

    #[test]
    fn sizes_keep_their_proportions_when_the_space_changes() {
        let limits = three();
        let mut state = State::new();
        state.prepare(&limits, 600.0);
        state.prepare(&limits, 1200.0);

        assert_eq!(state.sizes(), &[300.0, 450.0, 450.0]);
    }

    #[test]
    fn a_new_pane_count_starts_over() {
        let limits = three();
        let mut state = State::new();
        state.prepare(&limits, 600.0);
        state.toggle(&limits, 0);

        let two = vec![Limits::default(), Limits::default()];
        state.prepare(&two, 600.0);

        assert_eq!(state.sizes(), &[300.0, 300.0]);
        assert!(!state.is_collapsed(0));
    }

    #[test]
    fn panes_larger_than_the_space_keep_their_minimum() {
        let limits = vec![
            Limits {
                min_size: 250.0,
                ..Limits::default()
            };
            2
        ];
        let mut state = State::new();
        state.prepare(&limits, 299.0);

        assert_eq!(state.sizes(), &[250.0, 250.0]);
    }
}
