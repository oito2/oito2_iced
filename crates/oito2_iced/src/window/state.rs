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

//! The state of a window as the chrome shows it.

use iced::Task;
use iced::window::{self, Event, Id, Mode};

/// The state of a window as the chrome shows it: the maximize button turns
/// into a restore button while the window is maximized, and the title bar
/// dims while the window is not focused.
///
/// The application keeps one in its model and feeds it the events of
/// [`iced::window::events`]: [`WindowState::update`] applies the focus
/// changes and tells when the maximized and full-screen flags must be
/// queried again with [`WindowState::query`], whose result goes to
/// [`WindowState::apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WindowState {
    /// Whether the window fills the work area.
    pub maximized: bool,
    /// Whether the window has the input focus.
    pub focused: bool,
    /// Whether the window is in full-screen mode.
    pub fullscreen: bool,
}

impl Default for WindowState {
    /// A focused window that is neither maximized nor full screen.
    fn default() -> Self {
        Self {
            maximized: false,
            focused: true,
            fullscreen: false,
        }
    }
}

/// The maximized and full-screen flags of a window, read by
/// [`WindowState::query`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Placement {
    /// Whether the window fills the work area.
    pub maximized: bool,
    /// Whether the window is in full-screen mode.
    pub fullscreen: bool,
}

impl WindowState {
    /// Applies a window `event`: [`Event::Focused`] and
    /// [`Event::Unfocused`] set [`WindowState::focused`].
    ///
    /// Returns `true` for the events after which the maximized and
    /// full-screen flags may have changed ([`Event::Opened`],
    /// [`Event::Resized`] and [`Event::Moved`]); the application then runs
    /// [`WindowState::query`] for the window.
    pub fn update(&mut self, event: &Event) -> bool {
        match event {
            Event::Focused => {
                self.focused = true;
                false
            }
            Event::Unfocused => {
                self.focused = false;
                false
            }
            Event::Opened { .. } | Event::Resized(_) | Event::Moved(_) => true,
            _ => false,
        }
    }

    /// Reads the maximized and full-screen flags of the window `id`.
    pub fn query(id: Id) -> Task<Placement> {
        window::is_maximized(id).then(move |maximized| {
            window::mode(id).map(move |mode| Placement {
                maximized,
                fullscreen: mode == Mode::Fullscreen,
            })
        })
    }

    /// Stores the flags read by [`WindowState::query`].
    pub fn apply(&mut self, placement: Placement) {
        self.maximized = placement.maximized;
        self.fullscreen = placement.fullscreen;
    }
}
