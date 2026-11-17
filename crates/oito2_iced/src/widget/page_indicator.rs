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

//! Page indicators: dots showing the current page of a carousel or a paged
//! view, or a "page / length" counter when the dots don't fit.
//!
//! The current page is a pill three dots wide in the text-safe primary
//! tone; the other dots use the outline color. When the page changes, the
//! old pill shrinks and the new one grows; an indicator shows its final
//! state the first time it is drawn.
//!
//! With [`PageIndicator::on_select`] every dot is a target at least
//! [`MIN_TARGET_SIZE`] wide and tall (the spacing between the dots absorbs
//! as much of it as it can) that publishes its page on a click. The dots
//! are then one stop of keyboard navigation: the arrow keys (following the
//! [reading direction](PageIndicator::direction)), Home and End publish
//! the page they point at. Without it the indicator only shows the page
//! and is skipped by Tab.
//!
//! ```no_run
//! use oito2_iced::widget::page_indicator::{self, PageIndicatorLayout};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Page(usize),
//! }
//!
//! # fn view<'a>(page: usize) -> oito2_iced::Element<'a, Message> {
//! page_indicator::page_indicator(5, page)
//!     .layout(PageIndicatorLayout::Bounded)
//!     .on_select(Message::Page)
//!     .into()
//! # }
//! ```

use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Id, Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, renderer};
use iced::keyboard::{self, key};
use iced::time::Instant;
use iced::{
    Border, Color, Element as IcedElement, Event, Length, Point, Rectangle, Size, mouse, touch,
    window,
};

use super::helpers::ReadingDirection;
use super::styled_text;
use super::swap::ease_out;
use crate::tokens::{radius, sizes, spacing, typography};
use crate::{Element, Theme};

/// The default diameter of a dot.
pub const DOT_SIZE: f32 = 8.0;

/// The default smallest width and height of an interactive dot's target.
pub const MIN_TARGET_SIZE: f32 = sizes::MIN_POINTER_TARGET;

/// The length of the change of the current pill.
pub const CHANGE_DURATION: Duration = Duration::from_millis(200);

/// How a [`PageIndicator`] spaces its dots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum PageIndicatorLayout {
    /// Always [`PageIndicator::spacing`] apart.
    #[default]
    Fixed,
    /// Closer together as pages are added: the full spacing up to 5 pages,
    /// three quarters up to 10 and half beyond.
    Stepped,
    /// The full spacing when it fits, otherwise as close as needed to fit
    /// the width, down to half the dot size.
    Bounded,
}

/// Dots showing the current page. See [`page_indicator`].
pub struct PageIndicator<'a, Message> {
    length: usize,
    page: usize,
    on_select: Option<Box<dyn Fn(usize) -> Message + 'a>>,
    dot_size: f32,
    spacing: f32,
    layout: PageIndicatorLayout,
    min_target_size: f32,
    direction: ReadingDirection,
    animated: bool,
    id: Option<Id>,
    counter: Element<'a, Message>,
}

/// An indicator of `length` pages with `page` (from 0, clamped to the
/// last page) current. Without [`PageIndicator::on_select`] the dots are
/// not interactive.
pub fn page_indicator<'a, Message: 'a>(length: usize, page: usize) -> PageIndicator<'a, Message> {
    let page = page.min(length.saturating_sub(1));

    PageIndicator {
        length,
        page,
        on_select: None,
        dot_size: DOT_SIZE,
        spacing: spacing::SM,
        layout: PageIndicatorLayout::default(),
        min_target_size: MIN_TARGET_SIZE,
        direction: ReadingDirection::default(),
        animated: true,
        id: None,
        // The counter is the same literal text in every language.
        counter: styled_text(
            format!("{} / {}", page + 1, length),
            typography::LABEL_MEDIUM,
        )
        .into(),
    }
}

impl<'a, Message> PageIndicator<'a, Message> {
    /// Sets the function that turns the page of an activated dot into a
    /// message, which makes the dots interactive.
    #[must_use]
    pub fn on_select(mut self, on_select: impl Fn(usize) -> Message + 'a) -> Self {
        self.on_select = Some(Box::new(on_select));
        self
    }

    /// Sets the select function, or makes the dots not interactive with
    /// `None`.
    #[must_use]
    pub fn on_select_maybe(mut self, on_select: Option<impl Fn(usize) -> Message + 'a>) -> Self {
        self.on_select = on_select.map(|f| Box::new(f) as Box<dyn Fn(usize) -> Message + 'a>);
        self
    }

    /// Sets the diameter of the dots ([`DOT_SIZE`] by default).
    #[must_use]
    pub fn dot_size(mut self, dot_size: f32) -> Self {
        self.dot_size = dot_size.max(0.0);
        self
    }

    /// Sets the space between the dots (small spacing by default);
    /// [`PageIndicator::layout`] may reduce it.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing.max(0.0);
        self
    }

    /// Sets how the dots are spaced ([`PageIndicatorLayout::Fixed`] by
    /// default).
    #[must_use]
    pub fn layout(mut self, layout: PageIndicatorLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Sets the smallest width and height of an interactive dot's target
    /// ([`MIN_TARGET_SIZE`] by default). A smaller value, down to 0 for
    /// targets the size of the dots, suits an indicator whose pages another
    /// control also reaches.
    #[must_use]
    pub fn min_target_size(mut self, size: f32) -> Self {
        self.min_target_size = size.max(0.0);
        self
    }

    /// Sets the reading direction: right to left puts the first page at
    /// the right and mirrors the left and right arrow keys.
    #[must_use]
    pub fn direction(mut self, direction: ReadingDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Sets whether the current pill animates when the page changes (true
    /// by default); false shows the final state at once, for reduced
    /// motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Sets the widget [`Id`], used to focus the dots programmatically and
    /// to find them in tests.
    #[must_use]
    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id = Some(id.into());
        self
    }
}

/// The geometry of the dots for a given width.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Metrics {
    /// The width of the current pill.
    selected: f32,
    /// The width of the target of the current pill.
    selected_box: f32,
    /// The width of the target of the other dots.
    dot_box: f32,
    /// The space between targets.
    gap: f32,
    /// The height of the targets.
    height: f32,
    /// Whether the dots fit the width.
    fits: bool,
}

impl<Message> PageIndicator<'_, Message> {
    fn interactive(&self) -> bool {
        self.on_select.is_some()
    }

    /// The geometry of the dots in at most `max_width`.
    fn metrics(&self, max_width: f32) -> Metrics {
        let dot = self.dot_size;
        let selected = dot * 3.0;
        let interactive = self.interactive();
        let box_width = |width: f32| {
            if interactive {
                width.max(self.min_target_size)
            } else {
                width
            }
        };
        let others = self.length.saturating_sub(1) as f32;
        let boxes = box_width(selected) + others * box_width(dot);
        let absorbed = box_width(dot) - dot;
        let gaps = others.max(1.0);

        let spacing = match self.layout {
            PageIndicatorLayout::Fixed => self.spacing,
            PageIndicatorLayout::Stepped if self.length <= 5 => self.spacing,
            PageIndicatorLayout::Stepped if self.length <= 10 => self.spacing * 0.75,
            PageIndicatorLayout::Stepped => self.spacing * 0.5,
            PageIndicatorLayout::Bounded => {
                (dot / 2.0).max(self.spacing.min((max_width - boxes) / gaps + absorbed))
            }
        };
        let gap = (spacing - absorbed).max(0.0);

        Metrics {
            selected,
            selected_box: box_width(selected),
            dot_box: box_width(dot),
            gap,
            height: self.min_target_size.max(dot),
            fits: max_width >= boxes + others * gap,
        }
    }

    /// The target rectangle of each dot, from the first page, inside
    /// `bounds`.
    fn targets(&self, metrics: &Metrics, bounds: Rectangle) -> Vec<Rectangle> {
        let mut x = 0.0;
        let mut targets = Vec::with_capacity(self.length);

        for index in 0..self.length {
            let width = if index == self.page {
                metrics.selected_box
            } else {
                metrics.dot_box
            };
            let left = match self.direction {
                ReadingDirection::LeftToRight => bounds.x + x,
                ReadingDirection::RightToLeft => bounds.x + bounds.width - x - width,
            };

            targets.push(Rectangle::new(
                Point::new(left, bounds.y),
                Size::new(width, metrics.height),
            ));
            x += width + metrics.gap;
        }

        targets
    }

    /// The page a key points at, if `key` is one the dots take.
    fn page_for(&self, key: &keyboard::Key) -> Option<Option<usize>> {
        let (back, forward) = self.direction.arrows();
        let last = self.length.saturating_sub(1);

        match key {
            keyboard::Key::Named(named) if *named == back => Some(self.page.checked_sub(1)),
            keyboard::Key::Named(named) if *named == forward => {
                Some((self.page < last).then_some(self.page + 1))
            }
            keyboard::Key::Named(key::Named::Home) => Some(Some(0)),
            keyboard::Key::Named(key::Named::End) => Some(Some(last)),
            _ => None,
        }
    }
}

/// A change of the current page.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Change {
    from: usize,
    /// The first frame of the change; `None` until it is drawn.
    started: Option<Instant>,
}

/// The tree state of a [`PageIndicator`].
#[derive(Debug, Clone, Default)]
struct State {
    focused: bool,
    /// Whether the dots fit at the last layout.
    fits: bool,
    metrics: Option<Metrics>,
    page: usize,
    change: Option<Change>,
    now: Option<Instant>,
    /// The target the left button was pressed on.
    pressed: Option<usize>,
}

impl operation::Focusable for State {
    fn is_focused(&self) -> bool {
        self.focused
    }

    fn focus(&mut self) {
        self.focused = true;
    }

    fn unfocus(&mut self) {
        self.focused = false;
    }
}

impl State {
    /// The progress of the page change, from 0 to 1, eased.
    fn progress(&self) -> f32 {
        match self.change {
            Some(Change {
                started: Some(started),
                ..
            }) => self.now.map_or(1.0, |now| {
                ease_out(
                    now.saturating_duration_since(started).as_secs_f32()
                        / CHANGE_DURATION.as_secs_f32(),
                )
            }),
            Some(_) => 0.0,
            None => 1.0,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for PageIndicator<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            page: self.page,
            fits: true,
            ..State::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.counter)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.counter));

        let state = tree.state.downcast_mut::<State>();

        if state.page != self.page {
            state.change = self.animated.then_some(Change {
                from: state.page,
                started: None,
            });
            state.page = self.page;
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let metrics = self.metrics(limits.max().width);
        let counter =
            self.counter
                .as_widget_mut()
                .layout(&mut tree.children[0], renderer, &limits.loose());
        let state = tree.state.downcast_mut::<State>();

        state.fits = metrics.fits;
        state.metrics = Some(metrics);

        if self.length == 0 {
            return layout::Node::with_children(Size::ZERO, vec![counter]);
        }

        let size = if metrics.fits {
            let others = self.length.saturating_sub(1) as f32;

            Size::new(
                metrics.selected_box + others * (metrics.dot_box + metrics.gap),
                metrics.height,
            )
        } else {
            counter.size()
        };

        layout::Node::with_children(
            limits.resolve(Length::Shrink, Length::Shrink, size),
            vec![counter],
        )
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let state = tree.state.downcast_mut::<State>();
        let fits = state.fits;

        if self.interactive() && fits && self.length > 0 {
            operation.focusable(self.id.as_ref(), layout.bounds(), state);
        } else {
            state.focused = false;
            operation.container(self.id.as_ref(), layout.bounds());
        }

        if !fits && let Some(counter_layout) = layout.children().next() {
            operation.traverse(&mut |operation| {
                self.counter.as_widget_mut().operate(
                    &mut tree.children[0],
                    counter_layout,
                    renderer,
                    operation,
                );
            });
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if let Some(change) = &mut state.change {
            if let Event::Window(window::Event::RedrawRequested(now)) = event {
                let started = *change.started.get_or_insert(*now);

                state.now = Some(*now);

                if now.saturating_duration_since(started) >= CHANGE_DURATION {
                    state.change = None;
                }
            }
            shell.request_redraw();
        }

        let Some(on_select) = &self.on_select else {
            state.focused = false;
            state.pressed = None;
            return;
        };

        if !state.fits {
            state.pressed = None;
            return;
        }

        let Some(metrics) = state.metrics else {
            return;
        };

        let bounds = layout.bounds();
        let hit = |cursor: mouse::Cursor| {
            cursor.position_over(bounds).and_then(|position| {
                self.targets(&metrics, bounds)
                    .iter()
                    .position(|target| target.contains(position))
            })
        };

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                state.pressed = hit(cursor);

                if state.pressed.is_some() {
                    shell.capture_event();
                } else if state.focused && !cursor.is_over(bounds) {
                    state.focused = false;
                    shell.request_redraw();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. }) => {
                if let Some(pressed) = state.pressed.take()
                    && hit(cursor) == Some(pressed)
                {
                    shell.publish(on_select(pressed));
                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::CursorLeft)
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.pressed = None;
            }
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. })
                if state.focused && !shell.is_event_captured() && modifiers.is_empty() =>
            {
                if let Some(target) = self.page_for(key) {
                    if let Some(target) = target.filter(|target| *target != self.page) {
                        shell.publish(on_select(target));
                    }
                    shell.capture_event();
                }
            }
            _ => {}
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
        use iced::advanced::Renderer as _;

        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        if !state.fits {
            if let Some(counter_layout) = layout.children().next() {
                self.counter.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    style,
                    counter_layout,
                    cursor,
                    viewport,
                );
            }
            return;
        }

        let Some(metrics) = state.metrics else {
            return;
        };
        let roles = theme.roles();
        let active = roles.semantic.primary_text;
        let progress = state.progress();
        let from = state.change.map(|change| change.from);
        let dot = self.dot_size;

        for (index, target) in self.targets(&metrics, bounds).iter().enumerate() {
            // The share of the current pill the dot has: 1 for the current
            // page, 0 for the others, in between while the page changes.
            let now = if index == self.page { 1.0 } else { 0.0 };
            let before = match from {
                Some(from) if from == index => 1.0,
                Some(_) => 0.0,
                None => now,
            };
            let share = before + (now - before) * progress;
            let width = dot + (metrics.selected - dot) * share;

            renderer.fill_quad(
                renderer::Quad {
                    bounds: Rectangle {
                        x: target.center_x() - width / 2.0,
                        y: target.center_y() - dot / 2.0,
                        width,
                        height: dot,
                    },
                    border: Border::default().rounded(dot / 2.0),
                    ..renderer::Quad::default()
                },
                mix(roles.outline, active, share),
            );
        }

        if state.focused && self.interactive() {
            crate::focus::draw_ring(renderer, theme, bounds, radius::SMALL);
        }
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();
        let bounds = layout.bounds();

        match (state.metrics, cursor.position_over(bounds)) {
            (Some(metrics), Some(position)) if self.interactive() && state.fits => {
                if self
                    .targets(&metrics, bounds)
                    .iter()
                    .any(|target| target.contains(position))
                {
                    mouse::Interaction::Pointer
                } else {
                    mouse::Interaction::default()
                }
            }
            _ => mouse::Interaction::default(),
        }
    }
}

impl<'a, Message: 'a> From<PageIndicator<'a, Message>> for Element<'a, Message> {
    fn from(indicator: PageIndicator<'a, Message>) -> Self {
        IcedElement::new(indicator)
    }
}

/// The color `share` of the way from `from` to `to`.
fn mix(from: Color, to: Color, share: f32) -> Color {
    let share = share.clamp(0.0, 1.0);

    Color {
        r: from.r + (to.r - from.r) * share,
        g: from.g + (to.g - from.g) * share,
        b: from.b + (to.b - from.b) * share,
        a: from.a + (to.a - from.a) * share,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gap(layout: PageIndicatorLayout, length: usize, width: f32) -> f32 {
        page_indicator::<()>(length, 0)
            .layout(layout)
            .metrics(width)
            .gap
    }

    #[test]
    fn stepped_spacing_shrinks_with_more_pages() {
        assert_eq!(gap(PageIndicatorLayout::Stepped, 4, 600.0), 8.0);
        assert_eq!(gap(PageIndicatorLayout::Stepped, 8, 600.0), 6.0);
        assert_eq!(gap(PageIndicatorLayout::Stepped, 12, 600.0), 4.0);
    }

    #[test]
    fn bounded_spacing_shrinks_to_fit_the_width() {
        assert_eq!(gap(PageIndicatorLayout::Bounded, 10, 600.0), 8.0);

        let fitted = gap(PageIndicatorLayout::Bounded, 10, 150.0);
        assert!((4.0..8.0).contains(&fitted), "{fitted}");
    }

    #[test]
    fn colors_mix_by_share() {
        assert_eq!(mix(Color::BLACK, Color::WHITE, 0.0), Color::BLACK);
        assert_eq!(mix(Color::BLACK, Color::WHITE, 1.0), Color::WHITE);
    }
}
