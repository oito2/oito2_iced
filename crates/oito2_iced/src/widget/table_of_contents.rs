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

//! Tables of contents that follow the scroll position.
//!
//! A table of contents lists the sections of a scrollable document as
//! [`Entry`]s, highlights the current section and publishes the section
//! the user picks. Three pieces work together, with the current section
//! kept in the application:
//!
//! - [`table_of_contents`] shows the entries with the `active` one
//!   highlighted and publishes a picked entry through
//!   [`TableOfContents::on_select`];
//! - [`tracker`] wraps the [`scrollable`](iced::widget::scrollable()) that
//!   holds the sections and publishes the current section through
//!   [`Tracker::on_change`] as the content scrolls: the last section whose
//!   heading has reached [`Tracker::activation_offset`] pixels below the
//!   top of the viewport, or the last section when the view is scrolled to
//!   the end;
//! - [`scroll_to`] is the task that scrolls the document to the heading of
//!   a picked section.
//!
//! Each [`Entry::target`] is the [`Id`] of the heading of its section
//! inside the scrollable, such as a
//! [`container`](iced::widget::container()) with
//! [`id`](iced::widget::Container::id).
//!
//! ```no_run
//! use iced::Task;
//! use iced::widget::{column, container, row, scrollable, text};
//! use oito2_iced::widget::table_of_contents::{self as toc, Entry};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Section(usize),
//!     Pick(usize),
//! }
//!
//! struct Document {
//!     entries: Vec<Entry>,
//!     section: usize,
//! }
//!
//! impl Document {
//!     fn new() -> Self {
//!         Self {
//!             entries: vec![toc::entry("Intro", "intro"), toc::entry("Usage", "usage")],
//!             section: 0,
//!         }
//!     }
//!
//!     fn update(&mut self, message: Message) -> Task<Message> {
//!         match message {
//!             Message::Section(section) => {
//!                 self.section = section;
//!                 Task::none()
//!             }
//!             Message::Pick(section) => toc::scroll_to(
//!                 "document",
//!                 self.entries[section].target.clone(),
//!             ),
//!         }
//!     }
//!
//!     fn view(&self) -> oito2_iced::Element<'_, Message> {
//!         let body = scrollable(column![
//!             container(text("Intro")).id("intro"),
//!             text("…"),
//!             container(text("Usage")).id("usage"),
//!             text("…"),
//!         ])
//!         .id("document");
//!
//!         row![
//!             toc::table_of_contents(&self.entries, self.section).on_select(Message::Pick),
//!             toc::tracker(body, &self.entries, self.section).on_change(Message::Section),
//!         ]
//!         .into()
//!     }
//! }
//! ```

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::scrollable::{AbsoluteOffset, Scrollable};
use iced::advanced::widget::operation::{self, Focusable, Operation, Outcome};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::alignment::Horizontal;
use iced::font::Weight;
use iced::widget::button::{Status, Style};
use iced::widget::{Column, Space, container, stack};
use iced::{
    Background, Element as IcedElement, Event, Length, Padding, Rectangle, Size, Task, Vector,
    mouse,
};

use super::helpers::ReadingDirection;
use super::{focusable, styled_text};
use crate::theme::{HOVER_OPACITY, PRESSED_OPACITY, TRANSPARENT, state_layer};
use crate::tokens::spacing;
use crate::tokens::typography::{self, TextStyle};
use crate::{Element, Theme};

/// The default distance below the top of the viewport at which a heading
/// makes its section current (a literal in the OITO2 design).
pub const ACTIVATION_OFFSET: f32 = 32.0;

/// The extra indent of each nesting level (a literal in the OITO2 design).
const DEPTH_INDENT: f32 = 12.0;

/// The width of the line at the start of each entry (a literal in the
/// OITO2 design).
const LINE_WIDTH: f32 = 2.0;

/// A section listed by a table of contents.
#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// The section name.
    pub title: String,
    /// The [`Id`] of the section heading inside the scrollable, used to
    /// measure where the section starts and to scroll to it.
    pub target: Id,
    /// The nesting level, from 0; each level indents the entry further.
    pub depth: u16,
}

/// An entry named `title` for the section whose heading has the `target`
/// [`Id`], at depth 0.
pub fn entry(title: impl Into<String>, target: impl Into<Id>) -> Entry {
    Entry {
        title: title.into(),
        target: target.into(),
        depth: 0,
    }
}

impl Entry {
    /// Sets the nesting level, from 0.
    #[must_use]
    pub fn depth(mut self, depth: u16) -> Self {
        self.depth = depth;
        self
    }
}

/// The list of the sections of a document. See [`table_of_contents`].
pub struct TableOfContents<'a, Message> {
    entries: &'a [Entry],
    active: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    direction: ReadingDirection,
}

/// A table of contents listing `entries`, in document order, with the
/// entry at index `active` highlighted as the current section.
pub fn table_of_contents<Message>(
    entries: &[Entry],
    active: usize,
) -> TableOfContents<'_, Message> {
    TableOfContents {
        entries,
        active,
        on_select: None,
        direction: ReadingDirection::LeftToRight,
    }
}

impl<'a, Message> TableOfContents<'a, Message> {
    /// Sets the function that turns the index of a picked entry into a
    /// message, usually answered with [`scroll_to`]. Without it the
    /// entries can't be picked and are skipped by Tab.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the reading direction, which puts the line and the indent at
    /// the start of the entries.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }
}

impl<'a, Message: Clone + 'a> From<TableOfContents<'a, Message>> for Element<'a, Message> {
    fn from(toc: TableOfContents<'a, Message>) -> Self {
        let entries = toc.entries.iter().enumerate().map(|(index, entry)| {
            let message = toc.on_select.as_ref().map(|on_select| on_select(index));

            entry_view(entry, index == toc.active, message, toc.direction)
        });

        Column::with_children(entries).width(Length::Fill).into()
    }
}

/// The view of `entry`: its title after the line at its start, indented
/// by its depth, highlighted while `active`.
fn entry_view<'a, Message: Clone + 'a>(
    entry: &Entry,
    active: bool,
    on_press: Option<Message>,
    direction: ReadingDirection,
) -> Element<'a, Message> {
    let indent = spacing::MD + f32::from(entry.depth) * DEPTH_INDENT;
    let (left, right) = match direction {
        ReadingDirection::LeftToRight => (indent, spacing::SM),
        ReadingDirection::RightToLeft => (spacing::SM, indent),
    };
    let style = TextStyle {
        weight: if active {
            Weight::Semibold
        } else {
            typography::BODY_MEDIUM.weight
        },
        ..typography::BODY_MEDIUM
    };
    let mut label = container(styled_text(entry.title.clone(), style))
        .padding(Padding {
            top: spacing::XS + 2.0,
            right,
            bottom: spacing::XS + 2.0,
            left,
        })
        .width(Length::Fill);

    if direction == ReadingDirection::RightToLeft {
        label = label.align_x(Horizontal::Right);
    }

    let native = iced::widget::Button::new(label)
        .padding(0)
        .width(Length::Fill)
        .on_press_maybe(on_press.clone())
        .style(move |theme: &Theme, status| entry_style(theme, status, active));

    let mut wrapper = focusable(native).radius(0.0).enabled(on_press.is_some());

    if let Some(message) = on_press {
        wrapper = wrapper.on_activate(message);
    }

    let line = container(Space::new().width(LINE_WIDTH).height(Length::Fill))
        .height(Length::Fill)
        .style(move |theme: &Theme| {
            let roles = theme.roles();

            container::Style {
                background: Some(Background::Color(if active {
                    roles.indicator()
                } else {
                    roles.outline_variant
                })),
                ..container::Style::default()
            }
        });
    let line = container(line)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(match direction {
            ReadingDirection::LeftToRight => Horizontal::Left,
            ReadingDirection::RightToLeft => Horizontal::Right,
        });

    stack![wrapper, line].width(Length::Fill).into()
}

/// The colors of an entry: the indicator color while `active`, the variant
/// text color otherwise, over a state layer while hovered or pressed.
fn entry_style(theme: &Theme, status: Status, active: bool) -> Style {
    let roles = theme.roles();
    let text_color = if active {
        roles.indicator()
    } else {
        roles.on_surface_variant
    };
    let background = match status {
        Status::Hovered => state_layer(TRANSPARENT, roles.on_surface, HOVER_OPACITY),
        Status::Pressed => state_layer(TRANSPARENT, roles.on_surface, PRESSED_OPACITY),
        Status::Active | Status::Disabled => TRANSPARENT,
    };

    Style {
        background: (background != TRANSPARENT).then_some(Background::Color(background)),
        text_color,
        ..Style::default()
    }
}

/// A scrollable document that reports its current section. See
/// [`tracker`].
pub struct Tracker<'a, Message> {
    content: Element<'a, Message>,
    targets: Vec<Id>,
    active: usize,
    activation_offset: f32,
    on_change: Option<Box<dyn Fn(usize) -> Message + 'a>>,
}

/// Wraps `content`, which holds the scrollable with the headings of
/// `entries`, to report the current section; `active` is the section the
/// application shows as current, so only a different one is published.
pub fn tracker<'a, Message>(
    content: impl Into<Element<'a, Message>>,
    entries: &[Entry],
    active: usize,
) -> Tracker<'a, Message> {
    Tracker {
        content: content.into(),
        targets: entries.iter().map(|entry| entry.target.clone()).collect(),
        active,
        activation_offset: ACTIVATION_OFFSET,
        on_change: None,
    }
}

impl<'a, Message> Tracker<'a, Message> {
    /// Sets how far below the top of the viewport a heading makes its
    /// section current ([`ACTIVATION_OFFSET`] by default).
    #[must_use]
    pub fn activation_offset(mut self, offset: f32) -> Self {
        self.activation_offset = offset;
        self
    }

    /// Sets the function that turns the index of the new current section
    /// into a message.
    #[must_use]
    pub fn on_change(mut self, on_change: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_change = Some(Box::new(on_change));
        self
    }
}

/// The tree state of a [`Tracker`]: the section last measured.
#[derive(Debug, Clone, Copy, Default)]
struct TrackerState {
    measured: Option<usize>,
}

/// An operation that measures the first scrollable and the widgets with
/// the target ids inside it.
struct Measure<'a> {
    targets: &'a [Id],
    /// The bounds, content bounds and translation of the scrollable.
    scroll: Option<(Rectangle, Rectangle, Vector)>,
    headings: Vec<Option<Rectangle>>,
}

impl Measure<'_> {
    fn found(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if self.scroll.is_none() {
            return;
        }
        if let Some(id) = id
            && let Some(index) = self.targets.iter().position(|target| target == id)
            && let Some(slot) = self.headings.get_mut(index)
        {
            slot.get_or_insert(bounds);
        }
    }

    /// The current section, or `None` when no heading can tell it.
    fn active(&self, activation_offset: f32) -> Option<usize> {
        let (bounds, content, translation) = self.scroll?;
        let max_scroll = content.height - bounds.height;

        if self.targets.is_empty() {
            return None;
        }
        if max_scroll > 0.0 && translation.y >= max_scroll - 1.0 {
            return Some(self.targets.len() - 1);
        }

        let mut active = None;
        let mut first_below = None;

        for (index, heading) in self.headings.iter().enumerate() {
            let Some(heading) = heading else {
                continue;
            };

            if heading.y - content.y <= translation.y + activation_offset {
                active = Some(index);
            } else if first_below.is_none() {
                first_below = Some(index);
            }
        }

        active.or_else(|| first_below.map(|below: usize| below.saturating_sub(1)))
    }
}

impl Operation for Measure<'_> {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation)) {
        operate(self);
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        self.found(id, bounds);
    }

    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn Focusable) {
        self.found(id, bounds);
    }

    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, _text: &str) {
        self.found(id, bounds);
    }

    fn scrollable(
        &mut self,
        _id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        self.scroll
            .get_or_insert((bounds, content_bounds, translation));
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Tracker<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<TrackerState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(TrackerState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self
            .content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);

        layout::Node::with_children(content.size(), vec![content])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            operation,
        );
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
        let Some(content_layout) = layout.children().next() else {
            return;
        };

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            content_layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );

        let Some(on_change) = &self.on_change else {
            return;
        };

        let mut measure = Measure {
            targets: &self.targets,
            scroll: None,
            headings: vec![None; self.targets.len()],
        };

        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            content_layout,
            renderer,
            &mut measure,
        );

        let state = tree.state.downcast_mut::<TrackerState>();

        if let Some(active) = measure.active(self.activation_offset) {
            if active != self.active && state.measured != Some(active) {
                shell.publish(on_change(active));
            }
            state.measured = Some(active);
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
        if let Some(content_layout) = layout.children().next() {
            self.content.as_widget().draw(
                &tree.children[0],
                renderer,
                theme,
                style,
                content_layout,
                cursor,
                viewport,
            );
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
        layout
            .children()
            .next()
            .map(|content_layout| {
                self.content.as_widget().mouse_interaction(
                    &tree.children[0],
                    content_layout,
                    cursor,
                    viewport,
                    renderer,
                )
            })
            .unwrap_or_default()
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let content_layout = layout.children().next()?;

        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            content_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Tracker<'a, Message>> for Element<'a, Message> {
    fn from(tracker: Tracker<'a, Message>) -> Self {
        IcedElement::new(tracker)
    }
}

/// An operation that finds the scrollable with the `scrollable` id and the
/// widget with the `target` id inside it, then scrolls the scrollable so
/// the target reaches the top of its viewport, as far as the content
/// allows.
struct Locate {
    scrollable: Id,
    target: Id,
    /// The bounds and the content bounds of the scrollable.
    scroll: Option<(Rectangle, Rectangle)>,
    heading: Option<Rectangle>,
}

impl Locate {
    fn found(&mut self, id: Option<&Id>, bounds: Rectangle) {
        if self.scroll.is_some() && id == Some(&self.target) {
            self.heading.get_or_insert(bounds);
        }
    }
}

impl<T: 'static> Operation<T> for Locate {
    fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<T>)) {
        if self.heading.is_none() {
            operate(self);
        }
    }

    fn container(&mut self, id: Option<&Id>, bounds: Rectangle) {
        self.found(id, bounds);
    }

    fn focusable(&mut self, id: Option<&Id>, bounds: Rectangle, _state: &mut dyn Focusable) {
        self.found(id, bounds);
    }

    fn text(&mut self, id: Option<&Id>, bounds: Rectangle, _text: &str) {
        self.found(id, bounds);
    }

    fn scrollable(
        &mut self,
        id: Option<&Id>,
        bounds: Rectangle,
        content_bounds: Rectangle,
        _translation: Vector,
        _state: &mut dyn Scrollable,
    ) {
        if self.scroll.is_none() && id == Some(&self.scrollable) {
            self.scroll = Some((bounds, content_bounds));
        }
    }

    fn finish(&self) -> Outcome<T> {
        let (Some((bounds, content)), Some(heading)) = (self.scroll, self.heading) else {
            return Outcome::None;
        };
        let max_scroll = (content.height - bounds.height).max(0.0);
        let offset = (heading.y - content.y).clamp(0.0, max_scroll);

        Outcome::Chain(Box::new(operation::scrollable::scroll_to(
            self.scrollable.clone(),
            AbsoluteOffset {
                x: None,
                y: Some(offset),
            },
        )))
    }
}

/// The operation behind [`scroll_to`]: it scrolls the scrollable with
/// the `scrollable` id so the heading with the `target` id inside it
/// reaches the top of the viewport, as far as the content allows, through
/// a chained [`Outcome`].
pub fn scroll_operation<T: 'static>(scrollable: Id, target: Id) -> impl Operation<T> {
    Locate {
        scrollable,
        target,
        scroll: None,
        heading: None,
    }
}

/// The task that scrolls the scrollable with the `scrollable` id so the
/// heading with the `target` id inside it reaches the top of the viewport,
/// as far as the content allows; it does nothing when either isn't found.
pub fn scroll_to<T>(scrollable: impl Into<Id>, target: impl Into<Id>) -> Task<T>
where
    T: Send + 'static,
{
    iced::advanced::widget::operate(scroll_operation(scrollable.into(), target.into()))
}
