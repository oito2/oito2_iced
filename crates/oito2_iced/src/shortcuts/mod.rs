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

//! Keyboard shortcuts: key chords, platform labels and a command registry.
//!
//! - [`KeyChord`] is one key press with modifiers and [`Shortcut`] a chord
//!   or a sequence of chords, parsed from text such as `Mod+K` or `G H`.
//!   The `Mod` (primary) modifier stands for Command on Apple systems and
//!   Control elsewhere; [`OperatingSystem`] selects the reading and the
//!   labels.
//! - [`KeyChord::from_event`] and [`KeyChord::matches_event`] connect the
//!   chords to Iced keyboard events.
//! - [`Command`] and [`CommandRegistry`] register actions with optional
//!   shortcuts, search them, and detect clashes. [`SequenceMatcher`] turns
//!   a stream of key presses into the command to run.

mod commands;
mod key;

pub use commands::{Command, CommandRegistry, KeyMatch, SequenceMatcher, available_in, conflicts};
pub use key::{KeyChord, OperatingSystem, ParseShortcutError, Shortcut, uses_command_key};
