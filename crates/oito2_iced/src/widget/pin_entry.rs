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

//! Pin entries: a code, such as a PIN or a one-time code, drawn as one
//! cell per character.
//!
//! One hidden text input sits under the cells and holds the whole code, so
//! the entry is a single focus stop: typing fills the next empty cell,
//! Backspace empties the last filled one and a pasted code fills the cells
//! in order. The caret always stays at the end of the code, so the active
//! cell is the next empty one (the last cell once all are filled). A
//! numeric entry keeps only digits; a text entry keeps everything but
//! white space. Characters past [`PinEntry::length`] are dropped.
//!
//! Each cell is as tall as the control height of the current
//! [`density`] plus 6 and 4 narrower than tall, filled like
//! a text field and outlined; the active cell of a focused entry takes the
//! thick outline in the text-safe primary tone, and with
//! [`PinEntry::error`] every cell takes it in the text-safe error tone.
//!
//! The code lives in the application: the entry shows `value` and
//! publishes each new code through [`PinEntry::on_change`]; without it the
//! entry is disabled. Once the last cell is filled it publishes
//! [`PinEntry::on_complete`] and gives up the focus.
//!
//! Iced has no accessibility tree: the localized description of the entry
//! ("Code of 6 characters") and the shown character of each cell are
//! reported as text to widget operations, where selectors find them.
//!
//! ```no_run
//! use oito2_iced::widget::pin_entry;
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Code(String),
//!     Verify(String),
//! }
//!
//! # fn view<'a>(code: &str) -> oito2_iced::Element<'a, Message> {
//! pin_entry::pin_entry(code)
//!     .length(4)
//!     .on_change(Message::Code)
//!     .on_complete(Message::Verify)
//!     .into()
//! # }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::text::{self, Renderer as _};
use iced::advanced::widget::operation::{Focusable, Operation, TextInput};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, renderer};
use iced::widget::text::LineHeight;
use iced::widget::text_input;
use iced::{Border, Event, Length, Padding, Pixels, Point, Rectangle, Size, mouse};

use crate::l10n::Localizations;
use crate::theme::text_input::Class as InputClass;
use crate::theme::{DISABLED_CONTENT_OPACITY, TRANSPARENT};
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme, density};

/// The number of cells of an entry, unless set with [`PinEntry::length`].
pub const DEFAULT_LENGTH: usize = 6;

/// How much taller a cell is than the control height of the density.
const CELL_EXTRA_HEIGHT: f32 = 6.0;

/// How much narrower a cell is than tall.
const CELL_NARROWING: f32 = 4.0;

/// The character a cell of an obscured entry shows.
const BULLET: &str = "\u{2022}";

/// An entry showing the code `value`, with [`DEFAULT_LENGTH`] cells. Without
/// [`PinEntry::on_change`] it is disabled.
pub fn pin_entry<'a, Message>(value: &str) -> PinEntry<'a, Message> {
    PinEntry {
        value: value.to_owned(),
        length: DEFAULT_LENGTH,
        numeric: true,
        obscure: false,
        error: false,
        on_change: None,
        on_complete: None,
        id: None,
        localizations: Localizations::en(),
        label: None,
    }
}

/// A pin entry. See [`pin_entry`].
pub struct PinEntry<'a, Message> {
    value: String,
    length: usize,
    numeric: bool,
    obscure: bool,
    error: bool,
    on_change: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_complete: Option<Box<dyn Fn(String) -> Message + 'a>>,
    id: Option<Id>,
    localizations: Localizations,
    label: Option<String>,
}

impl<'a, Message> PinEntry<'a, Message> {
    /// Sets the number of cells ([`DEFAULT_LENGTH`] by default); it must be
    /// greater than zero.
    #[must_use]
    pub fn length(mut self, length: usize) -> Self {
        debug_assert!(length > 0, "the length is not greater than zero");
        self.length = length.max(1);
        self
    }

    /// Sets the message produced from each new code; without it the entry
    /// is disabled and skipped by focus traversal.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(String) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }

    /// Sets the message produced from the code when its last cell is
    /// filled.
    #[must_use]
    pub fn on_complete(mut self, on_complete: impl Fn(String) -> Message + 'a) -> Self {
        self.on_complete = Some(Box::new(on_complete));
        self
    }

    /// Sets whether only digits are accepted (true by default); otherwise
    /// every character but white space is.
    #[must_use]
    pub fn numeric(mut self, numeric: bool) -> Self {
        self.numeric = numeric;
        self
    }

    /// Shows the characters as bullets.
    #[must_use]
    pub fn obscure(mut self, obscure: bool) -> Self {
        self.obscure = obscure;
        self
    }

    /// Outlines every cell in the error tone.
    #[must_use]
    pub fn error(mut self, error: bool) -> Self {
        self.error = error;
        self
    }

    /// Sets the widget [`Id`] of the hidden text input, used to focus the
    /// entry with [`iced::widget::operation::focus`] and to find it in
    /// tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Takes the localized description of the entry from `localizations`
    /// (English by default).
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Replaces the localized description of the entry with `label`.
    #[must_use]
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// Keeps the characters of `text` a code takes: digits when `numeric`,
/// everything but white space otherwise, at most `length` of them.
fn filter(text: &str, numeric: bool, length: usize) -> String {
    text.chars()
        .filter(|c| {
            if numeric {
                c.is_ascii_digit()
            } else {
                !c.is_whitespace()
            }
        })
        .take(length)
        .collect()
}

/// The size of one cell for the current density.
fn cell_size() -> Size {
    let height = density::control_height() + CELL_EXTRA_HEIGHT;

    Size::new(height - CELL_NARROWING, height)
}

/// The width of `length` cells with their gaps.
fn total_width(length: usize, cell: Size) -> f32 {
    let count = length as f32;

    cell.width * count + spacing::SM * (count - 1.0).max(0.0)
}

impl<'a, Message: 'a> From<PinEntry<'a, Message>> for Element<'a, Message> {
    fn from(entry: PinEntry<'a, Message>) -> Self {
        let cell = cell_size();
        let width = total_width(entry.length, cell);
        let enabled = entry.on_change.is_some();

        let mut input = text_input("", &entry.value)
            .secure(entry.obscure)
            .size(1.0)
            .line_height(LineHeight::Absolute(Pixels(1.0)))
            .padding(Padding::from([(cell.height - 1.0) / 2.0, 0.0]))
            .width(Length::Fixed(width))
            .class(InputClass::Custom(Box::new(|_theme, _status| {
                text_input::Style {
                    background: TRANSPARENT.into(),
                    border: Border::default(),
                    icon: TRANSPARENT,
                    placeholder: TRANSPARENT,
                    value: TRANSPARENT,
                    selection: TRANSPARENT,
                }
            })));

        if enabled {
            input = input.on_input(|text| text);
        }

        if let Some(id) = entry.id.clone() {
            input = input.id(id);
        }

        let label = entry.label.clone().unwrap_or_else(|| {
            entry
                .localizations
                .code_entry(i64::try_from(entry.length).unwrap_or(i64::MAX))
        });

        Element::new(Cells {
            input: input.into(),
            label,
            entry,
            cell,
            width,
        })
    }
}

/// The widget of a pin entry: draws the cells and hands the input to the
/// hidden text input under them.
struct Cells<'a, Message> {
    input: Element<'a, String>,
    label: String,
    entry: PinEntry<'a, Message>,
    cell: Size,
    width: f32,
}

/// The state of a pin entry in the widget tree.
#[derive(Debug, Default)]
struct State {
    /// Whether the hidden text input has the focus.
    focused: bool,
    /// The value the entry was last shown with.
    shown: Option<String>,
    /// The code last published while the value has not changed since.
    published: Option<String>,
}

/// Reads whether the hidden text input has the focus.
#[derive(Default)]
struct Probe {
    focused: bool,
}

impl Operation for Probe {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        self.focused |= state.is_focused();
    }
}

/// Moves the caret of the hidden text input to the end, and removes its
/// focus when `release` is true.
struct Caret {
    release: bool,
}

impl Operation for Caret {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn text_input(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn TextInput) {
        state.move_cursor_to_end();
    }

    fn focusable(&mut self, _id: Option<&Id>, _bounds: Rectangle, state: &mut dyn Focusable) {
        if self.release {
            state.unfocus();
        }
    }
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

impl<Message> Cells<'_, Message> {
    /// The characters the cells show, one per cell.
    fn characters(&self) -> Vec<String> {
        self.entry
            .value
            .chars()
            .take(self.entry.length)
            .map(|c| {
                if self.entry.obscure {
                    BULLET.to_owned()
                } else {
                    c.to_string()
                }
            })
            .collect()
    }

    /// The bounds of the cell at `index` in an entry at `origin`.
    fn cell_bounds(&self, origin: Point, index: usize) -> Rectangle {
        Rectangle {
            x: origin.x + (self.cell.width + spacing::SM) * index as f32,
            y: origin.y,
            width: self.cell.width,
            height: self.cell.height,
        }
    }

    /// Forgets the published code once the value of the app changes.
    fn reconcile(&self, state: &mut State) {
        if state.shown.as_deref() != Some(self.entry.value.as_str()) {
            state.shown = Some(self.entry.value.clone());
            state.published = None;
        }
    }

    /// Reads the focus of the hidden text input into `state`; returns
    /// whether it changed.
    fn track_focus(
        &mut self,
        state: &mut State,
        child: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
    ) -> bool {
        let mut probe = Probe::default();
        self.input
            .as_widget_mut()
            .operate(child, layout, renderer, &mut probe);
        let changed = probe.focused != state.focused;
        state.focused = probe.focused;

        changed
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Cells<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.input)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.input));
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.width), Length::Fixed(self.cell.height))
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        let Tree {
            state, children, ..
        } = tree;
        self.reconcile(state.downcast_mut::<State>());

        let size = Size::new(self.width, self.cell.height);
        let Some(child) = children.first_mut() else {
            return layout::Node::new(size);
        };
        let input = self.input.as_widget_mut().layout(
            child,
            renderer,
            &layout::Limits::new(Size::ZERO, size),
        );

        layout::Node::with_children(size, vec![input])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let bounds = layout.bounds();
        let enabled = self.entry.on_change.is_some();

        // A disabled entry hands no operation to its hidden input, so the
        // entry itself carries the id.
        operation.container(
            if enabled {
                None
            } else {
                self.entry.id.as_ref()
            },
            bounds,
        );
        operation.text(None, bounds, &self.label);

        for (index, character) in self.characters().iter().enumerate() {
            operation.text(None, self.cell_bounds(bounds.position(), index), character);
        }

        if !enabled {
            return;
        }

        let (Some(input_layout), Some(child)) =
            (layout.children().next(), tree.children.first_mut())
        else {
            return;
        };

        operation.traverse(&mut |operation| {
            self.input
                .as_widget_mut()
                .operate(child, input_layout, renderer, operation);
        });

        let state = tree.state.downcast_mut::<State>();
        let _ = self.track_focus(state, child, input_layout, renderer);
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
        if self.entry.on_change.is_none() {
            return;
        }

        let Tree {
            state, children, ..
        } = tree;
        let state = state.downcast_mut::<State>();
        let (Some(input_layout), Some(child)) = (layout.children().next(), children.first_mut())
        else {
            return;
        };

        let mut texts = Vec::new();
        {
            let mut local = Shell::new(&mut texts);

            self.input.as_widget_mut().update(
                child,
                event,
                input_layout,
                cursor,
                renderer,
                clipboard,
                &mut local,
                viewport,
            );

            absorb(shell, &local);
        }

        let mut complete = false;

        for text in texts {
            let code = filter(&text, self.entry.numeric, self.entry.length);
            let current = state.published.as_deref().unwrap_or(&self.entry.value);

            if code == current {
                continue;
            }

            if let Some(on_change) = &self.entry.on_change {
                shell.publish(on_change(code.clone()));
            }

            if code.chars().count() == self.entry.length {
                complete = true;

                if let Some(on_complete) = &self.entry.on_complete {
                    shell.publish(on_complete(code.clone()));
                }
            }

            state.published = Some(code);
        }

        if state.focused || complete {
            self.input.as_widget_mut().operate(
                child,
                input_layout,
                renderer,
                &mut Caret { release: complete },
            );
        }

        if self.track_focus(state, child, input_layout, renderer) || complete {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_ref::<State>();
        let roles = theme.roles();
        let enabled = self.entry.on_change.is_some();
        let origin = layout.bounds().position();
        let characters = self.characters();
        let active = characters.len().min(self.entry.length - 1);
        let style = typography::TITLE_LARGE;
        let font = crate::font::inter(style.weight);

        let text_color = if enabled {
            roles.on_surface
        } else {
            roles.on_surface.scale_alpha(DISABLED_CONTENT_OPACITY)
        };

        for index in 0..self.entry.length {
            let bounds = self.cell_bounds(origin, index);
            let is_active = state.focused && enabled && index == active;

            let color = if self.entry.error {
                roles.semantic.error_text
            } else if is_active {
                roles.semantic.primary_text
            } else {
                roles.outline
            };
            let color = if enabled {
                color
            } else {
                color.scale_alpha(DISABLED_CONTENT_OPACITY)
            };
            let width = if is_active || self.entry.error {
                sizes::FOCUS_BORDER_WIDTH
            } else {
                1.0
            };

            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border: Border::default()
                        .color(color)
                        .width(width)
                        .rounded(radius::MEDIUM),
                    ..renderer::Quad::default()
                },
                roles.surface_container_high,
            );

            if let Some(character) = characters.get(index) {
                renderer.fill_text(
                    text::Text {
                        content: character.clone(),
                        bounds: bounds.size(),
                        size: Pixels(style.size),
                        line_height: LineHeight::Absolute(Pixels(style.line_height)),
                        font,
                        align_x: text::Alignment::Center,
                        align_y: iced::alignment::Vertical::Center,
                        shaping: text::Shaping::Advanced,
                        wrapping: text::Wrapping::None,
                    },
                    bounds.center(),
                    text_color,
                    bounds,
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
        if self.entry.on_change.is_none() {
            return mouse::Interaction::None;
        }

        match (layout.children().next(), tree.children.first()) {
            (Some(input_layout), Some(child)) => self.input.as_widget().mouse_interaction(
                child,
                input_layout,
                cursor,
                viewport,
                renderer,
            ),
            _ => mouse::Interaction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_numeric_code_keeps_only_digits_up_to_the_length() {
        assert_eq!(filter("12a34", true, 4), "1234");
        assert_eq!(filter("987654", true, 4), "9876");
        assert_eq!(filter(" 1 2", true, 6), "12");
    }

    #[test]
    fn a_text_code_drops_only_white_space() {
        assert_eq!(filter("a\u{1F600} b", false, 3), "a\u{1F600}b");
        assert_eq!(filter("ab\tcd", false, 3), "abc");
    }

    #[test]
    fn cells_are_six_taller_than_the_control_and_four_narrower() {
        let cell = cell_size();

        assert_eq!(cell.height, density::control_height() + 6.0);
        assert_eq!(cell.width, cell.height - 4.0);
        assert_eq!(total_width(4, cell), cell.width * 4.0 + spacing::SM * 3.0);
    }
}
