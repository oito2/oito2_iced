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

//! Collections and controllers without user interface: lists, selection,
//! trees, asynchronous lists and text folding.
//!
//! Controllers are plain structs: the application keeps them in its model and
//! calls their methods from `update`. Methods that change state return
//! whether something changed, so the caller knows when to refresh.

mod async_list;
mod list;
mod selection;
mod text_folding;
mod tree;

pub use async_list::{
    AsyncListController, AsyncListPage, AsyncListRequest, LoadId, SortDescriptor, SortDirection,
};
pub use list::{ListCollection, ListGroup};
pub use selection::{SelectionController, SelectionMode, SelectionOptions};
pub use text_folding::{FoldedText, fold_text, fold_text_with_offsets};
pub use tree::{TreeCollection, TreeEntry, TreeNode};

use std::error::Error;
use std::fmt;

/// The error of a collection operation that its input does not allow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectionError {
    /// Two items or nodes share the same value.
    DuplicateValue(String),
    /// An index is outside the valid range; `len` is the exclusive upper
    /// bound for `index`, or the largest accepted index plus one.
    IndexOutOfRange {
        /// The rejected index.
        index: usize,
        /// The number of positions the operation accepts.
        len: usize,
    },
    /// A value that must name an existing node does not.
    UnknownNode(String),
    /// A node would be moved into itself or into one of its descendants.
    InvalidMove(String),
}

impl fmt::Display for CollectionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DuplicateValue(value) => write!(f, "duplicate value \"{value}\""),
            Self::IndexOutOfRange { index, len } => {
                write!(f, "index {index} is out of range for {len} positions")
            }
            Self::UnknownNode(value) => write!(f, "unknown node \"{value}\""),
            Self::InvalidMove(value) => write!(
                f,
                "a node cannot move into itself or its descendants (\"{value}\")"
            ),
        }
    }
}

impl Error for CollectionError {}
