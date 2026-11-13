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

//! The density of the controls: pointer or touch sizes.
//!
//! Iced lays widgets out without the theme, so the OITO2 widgets read the
//! density from this module while they measure themselves. The setting
//! belongs to the current thread, the one that builds and lays out the
//! view; an application calls [`set`] with the density of its
//! [`Theme`](crate::Theme) at the start of its `view`, so both always agree.
//!
//! ```
//! use oito2_iced::density::{self, Density};
//!
//! density::set(Density::Touch);
//! assert_eq!(density::control_height(), 40.0);
//! density::set(Density::Pointer);
//! assert_eq!(density::control_height(), 34.0);
//! ```

use std::cell::Cell;

pub use oito2_foundation::Density;

use crate::tokens::sizes;

/// Size of the icons inside controls on a touch screen.
const TOUCH_ICON_SIZE: f32 = 24.0;

thread_local! {
    static DENSITY: Cell<Density> = const { Cell::new(Density::Pointer) };
}

/// Sets the density the widgets built and laid out on the current thread
/// use. [`Density::Pointer`] until set.
pub fn set(density: Density) {
    DENSITY.with(|cell| cell.set(density));
}

/// The density of the current thread.
pub fn get() -> Density {
    DENSITY.with(Cell::get)
}

/// Minimum height of buttons and single-line fields at the current density:
/// 34 pixels for a pointer, 40 for touch.
pub fn control_height() -> f32 {
    control_height_of(get())
}

/// Size of the icons inside controls at the current density: 20 pixels for
/// a pointer, 24 for touch.
pub fn icon_size() -> f32 {
    icon_size_of(get())
}

/// Minimum height of buttons and single-line fields at `density`.
pub const fn control_height_of(density: Density) -> f32 {
    match density {
        Density::Pointer => sizes::BUTTON_HEIGHT,
        Density::Touch => sizes::TOUCH_BUTTON_HEIGHT,
    }
}

/// Size of the icons inside controls at `density`.
pub const fn icon_size_of(density: Density) -> f32 {
    match density {
        Density::Pointer => sizes::ICON_SIZE,
        Density::Touch => TOUCH_ICON_SIZE,
    }
}
