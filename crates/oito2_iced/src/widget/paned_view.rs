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

//! Paned views: a side pane and a page separated by a one-pixel divider,
//! with a handle that resizes the pane.
//!
//! The pane never shrinks below its smallest size and never leaves less
//! than the smallest page size for the page; when even the smallest pane
//! does not fit, the end is clipped. The handle follows a drag until the
//! button is released and, once focused, moves by the keyboard step with
//! the arrow keys of its axis.
//!
//! The pane size lives in the application: the view shows the size it is
//! given and publishes the next one through [`PanedView::on_resize`].
//!
//! ```no_run
//! use oito2_iced::widget::paned_view;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Resized(f32),
//! }
//!
//! # fn view<'a>(pane_size: f32) -> oito2_iced::Element<'a, Message> {
//! paned_view::paned_view(
//!     iced::widget::text("Sidebar"),
//!     iced::widget::text("Content"),
//!     pane_size,
//! )
//! .on_resize(Message::Resized)
//! .into()
//! # }
//! ```

use super::helpers::ReadingDirection;
use super::resize_handle::{DIVIDER, Outcome, Request, Sizing, Split};
use super::splitter::{Axis, DEFAULT_KEYBOARD_STEP};
use crate::Element;

/// The initial pane size an application usually starts from, in pixels.
pub const DEFAULT_PANE_SIZE: f32 = 280.0;

/// The smallest pane size by default, in pixels.
pub const DEFAULT_MIN_PANE_SIZE: f32 = 180.0;

/// The smallest size left for the page by default, in pixels.
pub const DEFAULT_MIN_PAGE_SIZE: f32 = 320.0;

/// The side of a [`PanedView`] its pane is placed on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Side {
    /// The start side: left in left-to-right text, right otherwise.
    #[default]
    Start,
    /// The end side: right in left-to-right text, left otherwise.
    End,
    /// The top.
    Top,
    /// The bottom.
    Bottom,
}

impl Side {
    /// Whether the pane and the page are stacked vertically.
    pub fn is_vertical(self) -> bool {
        matches!(self, Side::Top | Side::Bottom)
    }

    /// Whether the pane comes before the page in reading order.
    fn is_first(self) -> bool {
        matches!(self, Side::Start | Side::Top)
    }
}

/// Clamps `size` between `min_pane` and the space of `available` pixels
/// left after `min_page`; the smallest pane size wins when both do not fit.
pub(crate) fn clamp_pane_size(size: f32, min_pane: f32, min_page: f32, available: f32) -> f32 {
    let max = min_pane.max(available - min_page);
    size.clamp(min_pane, max)
}

/// A side pane next to a page. Built by [`paned_view`].
pub struct PanedView<'a, Message> {
    pane: Element<'a, Message>,
    page: Element<'a, Message>,
    pane_size: f32,
    side: Side,
    min_pane_size: f32,
    min_page_size: f32,
    resizable: bool,
    direction: ReadingDirection,
    keyboard_step: f32,
    on_resize: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

/// A view splitting its area between `pane`, `pane_size` pixels wide on the
/// start side, and `page`. Without [`PanedView::on_resize`] the divider
/// has no handle.
pub fn paned_view<'a, Message>(
    pane: impl Into<Element<'a, Message>>,
    page: impl Into<Element<'a, Message>>,
    pane_size: f32,
) -> PanedView<'a, Message> {
    PanedView {
        pane: pane.into(),
        page: page.into(),
        pane_size,
        side: Side::Start,
        min_pane_size: DEFAULT_MIN_PANE_SIZE,
        min_page_size: DEFAULT_MIN_PAGE_SIZE,
        resizable: true,
        direction: ReadingDirection::default(),
        keyboard_step: DEFAULT_KEYBOARD_STEP,
        on_resize: None,
    }
}

impl<'a, Message> PanedView<'a, Message> {
    /// Which side the pane is on; [`Side::Start`] by default.
    #[must_use]
    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// The smallest pane size; defaults to [`DEFAULT_MIN_PANE_SIZE`].
    #[must_use]
    pub fn min_pane_size(mut self, size: f32) -> Self {
        self.min_pane_size = size.max(0.0);
        self
    }

    /// The smallest size left for the page; defaults to
    /// [`DEFAULT_MIN_PAGE_SIZE`].
    #[must_use]
    pub fn min_page_size(mut self, size: f32) -> Self {
        self.min_page_size = size.max(0.0);
        self
    }

    /// Whether the divider can be moved (true by default); a fixed divider
    /// has no handle.
    #[must_use]
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    /// The reading direction: the start side is on the right in
    /// right-to-left text.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// How far one arrow key press moves the divider; defaults to
    /// [`DEFAULT_KEYBOARD_STEP`].
    #[must_use]
    pub fn keyboard_step(mut self, step: f32) -> Self {
        self.keyboard_step = step;
        self
    }

    /// Publishes the new pane size while the divider is moved.
    #[must_use]
    pub fn on_resize(mut self, on_resize: impl Fn(f32) -> Message + 'a) -> Self {
        self.on_resize = Some(Box::new(on_resize));
        self
    }
}

/// The sizing of a [`PanedView`].
struct PaneSizing<'a, Message> {
    size: f32,
    first: bool,
    min_pane: f32,
    min_page: f32,
    available: f32,
    on_resize: Option<Box<dyn Fn(f32) -> Message + 'a>>,
}

impl<Message> PaneSizing<'_, Message> {
    /// The pane size within the limits of the current space.
    fn clamped(&self, size: f32) -> f32 {
        clamp_pane_size(size, self.min_pane, self.min_page, self.available)
    }
}

impl<Message> Sizing<Message> for PaneSizing<'_, Message> {
    fn sizes(&mut self, extent: f32) -> Vec<f32> {
        self.available = extent;
        let pane = self.clamped(self.size);
        let page = (extent - pane - DIVIDER).max(0.0);

        if self.first {
            vec![pane, page]
        } else {
            vec![page, pane]
        }
    }

    fn apply(&mut self, _divider: usize, request: Request) -> Outcome<Message> {
        let (Some(on_resize), Request::Move(delta)) = (&self.on_resize, request) else {
            return Outcome::Ignored;
        };
        let current = self.clamped(self.size);
        let next = self.clamped(current + if self.first { delta } else { -delta });

        if next == current {
            self.size = current;
            return Outcome::Unchanged;
        }

        self.size = next;
        Outcome::Changed(on_resize(next))
    }
}

impl<'a, Message: 'a> From<PanedView<'a, Message>> for Element<'a, Message> {
    fn from(view: PanedView<'a, Message>) -> Self {
        let first = view.side.is_first();
        let axis = if view.side.is_vertical() {
            Axis::Vertical
        } else {
            Axis::Horizontal
        };
        let children = if first {
            vec![view.pane, view.page]
        } else {
            vec![view.page, view.pane]
        };
        let resizable = view.resizable && view.on_resize.is_some();

        Split::new(
            children,
            axis,
            view.direction == ReadingDirection::RightToLeft,
            resizable,
            view.keyboard_step,
            PaneSizing {
                size: view.pane_size,
                first,
                min_pane: view.min_pane_size,
                min_page: view.min_page_size,
                available: 0.0,
                on_resize: view.on_resize,
            },
        )
        .into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pane_leaves_the_smallest_page_size() {
        assert_eq!(clamp_pane_size(300.0, 200.0, 400.0, 1000.0), 300.0);
        assert_eq!(clamp_pane_size(100.0, 200.0, 400.0, 1000.0), 200.0);
        assert_eq!(clamp_pane_size(900.0, 200.0, 400.0, 1000.0), 600.0);
        assert_eq!(clamp_pane_size(900.0, 200.0, 400.0, 300.0), 200.0);
    }
}
