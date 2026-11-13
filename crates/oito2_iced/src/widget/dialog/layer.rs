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

//! A root layer that shows one surface above the application view:
//! centered, against a side edge or against the bottom edge, over a scrim
//! while modal.
//!
//! A modal surface keeps the input: the view below receives only window
//! events, Tab and Shift+Tab cycle the focus inside the surface, a press on
//! the scrim publishes the dismiss message when allowed and the focus of the
//! view below is cleared when the surface opens. A non-modal surface leaves
//! the view below usable beside it. Escape publishes the dismiss message,
//! for a non-modal surface only while the focus is inside it.
//!
//! A surface against an edge slides in from it when it opens after the
//! layer was created, follows a drag toward its edge, and settles back,
//! moves to another resting height or is dismissed when the drag ends.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::{self, Operation};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Shell, overlay, renderer};
use iced::keyboard::{self, key};
use iced::{Event, Length, Padding, Point, Rectangle, Size, Vector, mouse, touch, window};

use crate::animation::ease_out_cubic;
use crate::focus::operations::{focus_first, focus_last, has_focus};
use crate::{Element, Theme};

/// Distance a press travels along the drag axis before it drags the
/// surface instead of reaching the content.
pub(crate) const DRAG_SLOP: f32 = 18.0;

/// Drag speed, in pixels per second, past which a released drag counts as
/// a fling.
pub(crate) const FLING_VELOCITY: f32 = 700.0;

/// Fraction of its extent a surface without resting heights is dragged
/// before the release dismisses it.
pub(crate) const DISMISS_FRACTION: f32 = 0.35;

/// Time window of the pointer samples the release speed is measured over.
const VELOCITY_WINDOW: Duration = Duration::from_millis(100);

/// The window edge a surface rests against.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Edge {
    /// The left edge.
    Left,
    /// The right edge.
    Right,
    /// The bottom edge.
    Bottom,
}

/// Where the surface sits in the window.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Placement {
    /// Centered, `inset` away from the window edges, as wide as the
    /// content needs and at least `min_width` wide when the window allows
    /// it; content that fills its width takes that width.
    Center {
        /// Space kept free at the window edges.
        inset: Padding,
        /// Smallest width.
        min_width: f32,
    },
    /// Against the left or right edge, as tall as the window and `width`
    /// wide, leaving at least `margin` of the window uncovered.
    Side {
        /// The edge, left or right.
        edge: Edge,
        /// Preferred width.
        width: f32,
        /// Smallest uncovered width of the window.
        margin: f32,
    },
    /// Against the bottom edge and centered, as wide as the window up to
    /// `max_width`; as tall as its content up to `max_height` of the window
    /// height, or as tall as the highest resting height when given.
    Bottom {
        /// Largest width.
        max_width: f32,
        /// Largest height, as a fraction of the window height.
        max_height: f32,
    },
}

impl Placement {
    /// The edge the surface rests against, if any.
    fn edge(&self) -> Option<Edge> {
        match self {
            Placement::Center { .. } => None,
            Placement::Side { edge, .. } => Some(*edge),
            Placement::Bottom { .. } => Some(Edge::Bottom),
        }
    }
}

/// Resting heights of a surface against the bottom edge, as fractions of
/// the window height in ascending order, and the current one.
pub(crate) struct Snaps<'a, Message> {
    /// The heights, ascending.
    pub points: Vec<f32>,
    /// Index of the current height.
    pub current: usize,
    /// Produces the message that moves the surface to another height.
    pub on_snap: Box<dyn Fn(usize) -> Message + 'a>,
}

/// The surface shown by a [`Layer`].
pub(crate) struct Surface<'a, Message> {
    /// The surface, already decorated.
    pub content: Element<'a, Message>,
    /// Where it sits.
    pub placement: Placement,
    /// Whether it keeps the input and dims the view below.
    pub modal: bool,
    /// Opacity of the scrim color while modal.
    pub scrim: f32,
    /// Published on Escape, on a press on the scrim when
    /// `dismiss_on_scrim`, and on a drag that dismisses.
    pub on_dismiss: Option<Message>,
    /// Whether a press on the scrim and a drag toward the edge dismiss.
    pub dismissible: bool,
    /// Whether a surface against an edge follows drags toward it.
    pub draggable: bool,
    /// Resting heights of a surface against the bottom edge.
    pub snaps: Option<Snaps<'a, Message>>,
    /// Duration of the slide in from the edge.
    pub entrance: Duration,
    /// Duration of the settling after a drag or a height change.
    pub settle: Duration,
}

/// A view with an optional [`Surface`] above it.
pub(crate) struct Layer<'a, Message> {
    base: Element<'a, Message>,
    surface: Option<Surface<'a, Message>>,
}

impl<'a, Message> Layer<'a, Message> {
    /// `base` with `surface` above it.
    pub(crate) fn new(
        base: impl Into<Element<'a, Message>>,
        surface: Option<Surface<'a, Message>>,
    ) -> Self {
        Self {
            base: base.into(),
            surface,
        }
    }
}

/// A move from `from` to zero, with the cubic ease-out curve.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Tween {
    from: f32,
    start: Option<Instant>,
    duration: Duration,
}

impl Tween {
    /// A move from `from` that starts at the next frame.
    fn new(from: f32, duration: Duration) -> Self {
        Self {
            from,
            start: None,
            duration,
        }
    }

    /// Progress from 0 to 1 at `now`; 0 before the move starts.
    fn progress(&self, now: Option<Instant>) -> f32 {
        match (self.start, now) {
            (Some(_), _) if self.duration.is_zero() => 1.0,
            (Some(start), Some(now)) => {
                let elapsed = now.saturating_duration_since(start).as_secs_f32();

                ease_out_cubic((elapsed / self.duration.as_secs_f32()).clamp(0.0, 1.0))
            }
            _ => 0.0,
        }
    }

    /// The value at `now`.
    fn value(&self, now: Option<Instant>) -> f32 {
        self.from * (1.0 - self.progress(now))
    }
}

/// A press on the surface that may turn into a drag.
#[derive(Debug, Clone)]
struct Press {
    /// Position along the drag axis, toward the edge, at the press.
    origin: f32,
    /// Position along the drag axis at the last move.
    last: f32,
    /// Whether the press moved past the slop and drags the surface.
    dragging: bool,
    /// Recent positions along the drag axis with their time.
    samples: VecDeque<(Instant, f32)>,
}

/// The tree state of a [`Layer`].
#[derive(Debug)]
struct State {
    /// Whether a surface was shown at the last rebuild.
    open: bool,
    /// The slide in, as the hidden fraction of the surface.
    entrance: Option<Tween>,
    /// Whether the focus moves into the surface at the next event.
    focus_pending: bool,
    /// Drag distance toward the edge, in pixels, from the current resting
    /// height.
    drag: f32,
    /// The settling of the drag distance back to zero.
    settle: Option<Tween>,
    /// The resting height index of the last layout.
    snap_seen: Option<usize>,
    /// The press in progress.
    press: Option<Press>,
    /// Time of the last frame.
    now: Option<Instant>,
    /// Size of the surface at the last layout.
    panel: Size,
    /// Size of the layer at the last layout.
    window: Size,
}

impl State {
    /// The drag distance at the last frame, settling included.
    fn drag(&self) -> f32 {
        self.settle.map_or(self.drag, |tween| tween.value(self.now))
    }

    /// The hidden fraction of the surface at the last frame.
    fn hidden(&self) -> f32 {
        self.entrance.map_or(0.0, |tween| tween.value(self.now))
    }
}

/// The offset below the highest resting height of the resting height at
/// `index`, in pixels of a window `height` tall.
pub(crate) fn snap_offset(points: &[f32], index: usize, height: f32) -> f32 {
    match (points.last(), points.get(index)) {
        (Some(last), Some(point)) => (last - point) * height,
        _ => 0.0,
    }
}

/// What the end of a drag does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Release {
    /// Dismisses the surface.
    Dismiss,
    /// Moves to the resting height at the index, or stays when it is the
    /// current one.
    Snap(usize),
    /// Settles back in place.
    Back,
}

/// The outcome of a drag released `drag` pixels toward the edge at
/// `velocity` pixels per second (positive toward the edge), for a surface
/// `extent` long along the drag axis in a window `height` tall.
///
/// With resting heights, the nearest height to the release wins, moved one
/// step by a fling; dragging below the lowest one, or past half of it when
/// `dismissible`, dismisses. Without them, a drag past
/// [`DISMISS_FRACTION`] of the extent or a fling dismisses when
/// `dismissible`.
pub(crate) fn release(
    drag: f32,
    velocity: f32,
    extent: f32,
    height: f32,
    dismissible: bool,
    snaps: Option<(&[f32], usize)>,
) -> Release {
    let Some((points, current)) = snaps.filter(|(points, _)| !points.is_empty()) else {
        return if dismissible && (drag > extent * DISMISS_FRACTION || velocity > FLING_VELOCITY) {
            Release::Dismiss
        } else {
            Release::Back
        };
    };

    let top = snap_offset(points, current, height) + drag;
    let mut best = 0_isize;
    let mut best_distance = f32::INFINITY;

    for index in 0..points.len() {
        let distance = (snap_offset(points, index, height) - top).abs();

        if distance < best_distance {
            best = index as isize;
            best_distance = distance;
        }
    }

    let current = current as isize;

    if velocity > FLING_VELOCITY {
        best = best.min(current - 1);
    }
    if velocity < -FLING_VELOCITY {
        best = best.max(current + 1);
    }

    let last = points[points.len() - 1];
    let below_half = top > (last - points[0] / 2.0) * height;

    if best < 0 || (dismissible && below_half) {
        if dismissible {
            return Release::Dismiss;
        }
        best = 0;
    }

    Release::Snap(best.clamp(0, points.len() as isize - 1) as usize)
}

/// Position along the drag axis of `point`, growing toward `edge`.
fn along(edge: Edge, point: Point) -> f32 {
    match edge {
        Edge::Left => -point.x,
        Edge::Right => point.x,
        Edge::Bottom => point.y,
    }
}

/// The pointer position of a press, move or release event.
enum Pointer {
    Pressed(Option<Point>),
    Moved(Point),
    Released,
}

/// The pointer action of `event`, with the position of moves and touches.
fn pointer(event: &Event, cursor: mouse::Cursor) -> Option<Pointer> {
    match event {
        Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
            Some(Pointer::Pressed(cursor.position()))
        }
        Event::Touch(touch::Event::FingerPressed { position, .. }) => {
            Some(Pointer::Pressed(Some(*position)))
        }
        Event::Mouse(mouse::Event::CursorMoved { position })
        | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
            Some(Pointer::Moved(*position))
        }
        Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
        | Event::Touch(touch::Event::FingerLifted { .. } | touch::Event::FingerLost { .. }) => {
            Some(Pointer::Released)
        }
        _ => None,
    }
}

/// Speed along the drag axis over the recent `samples`, in pixels per
/// second, measured over at least the velocity window.
fn velocity(samples: &VecDeque<(Instant, f32)>) -> f32 {
    match (samples.front(), samples.back()) {
        (Some((start, from)), Some((end, to))) => {
            let elapsed = end
                .saturating_duration_since(*start)
                .max(VELOCITY_WINDOW)
                .as_secs_f32();

            (to - from) / elapsed
        }
        _ => 0.0,
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Layer<'_, Message>
where
    Message: Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        let open = self.surface.is_some();

        tree::State::new(State {
            open,
            entrance: None,
            focus_pending: self.surface.as_ref().is_some_and(|surface| surface.modal),
            drag: 0.0,
            settle: None,
            snap_seen: None,
            press: None,
            now: None,
            panel: Size::ZERO,
            window: Size::ZERO,
        })
    }

    fn children(&self) -> Vec<Tree> {
        match &self.surface {
            Some(surface) => vec![Tree::new(&self.base), Tree::new(&surface.content)],
            None => vec![Tree::new(&self.base)],
        }
    }

    fn diff(&self, tree: &mut Tree) {
        match &self.surface {
            Some(surface) => tree.diff_children(&[&self.base, &surface.content]),
            None => tree.diff_children(std::slice::from_ref(&self.base)),
        }

        let state = tree.state.downcast_mut::<State>();

        match &self.surface {
            Some(surface) if !state.open => {
                state.open = true;
                state.focus_pending = surface.modal;
                state.drag = 0.0;
                state.settle = None;
                state.snap_seen = None;
                state.press = None;
                state.entrance = (surface.placement.edge().is_some()
                    && !surface.entrance.is_zero())
                .then(|| Tween::new(1.0, surface.entrance));
            }
            Some(_) => {}
            None => {
                state.open = false;
                state.entrance = None;
                state.focus_pending = false;
                state.press = None;
            }
        }
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let base = self
            .base
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits);
        let window = limits.resolve(Length::Fill, Length::Fill, base.size());

        let Some(surface) = self.surface.as_mut() else {
            return layout::Node::with_children(window, vec![base]);
        };

        let state = tree.state.downcast_mut::<State>();
        let snaps = surface
            .snaps
            .as_ref()
            .filter(|snaps| !snaps.points.is_empty());
        let highest = snaps.and_then(|snaps| snaps.points.last().copied());

        let content_limits = match &surface.placement {
            Placement::Center { inset, min_width } => {
                let area = Size::new(
                    (window.width - inset.x()).max(0.0),
                    (window.height - inset.y()).max(0.0),
                );
                let intrinsic = surface
                    .content
                    .as_widget_mut()
                    .layout(
                        &mut tree.children[1],
                        renderer,
                        &layout::Limits::with_compression(Size::ZERO, area, Size::new(true, true)),
                    )
                    .size();
                let width = intrinsic.width.max(*min_width).min(area.width);

                layout::Limits::new(Size::new(width, 0.0), Size::new(width, area.height))
            }
            Placement::Side { width, margin, .. } => {
                let size = Size::new(width.min(window.width - margin).max(0.0), window.height);

                layout::Limits::new(size, size)
            }
            Placement::Bottom {
                max_width,
                max_height,
            } => {
                let width = window.width.min(*max_width);

                match highest {
                    Some(highest) => {
                        let size = Size::new(width, window.height * highest);

                        layout::Limits::new(size, size)
                    }
                    None => layout::Limits::new(
                        Size::new(width, 0.0),
                        Size::new(width, window.height * max_height),
                    ),
                }
            }
        };

        let node = surface.content.as_widget_mut().layout(
            &mut tree.children[1],
            renderer,
            &content_limits,
        );
        let panel = node.size();

        if let Some(snaps) = snaps {
            if let Some(seen) = state.snap_seen
                && seen != snaps.current
            {
                let shift = snap_offset(&snaps.points, seen, window.height)
                    - snap_offset(&snaps.points, snaps.current, window.height);
                let dragging = state.press.as_ref().is_some_and(|press| press.dragging);

                state.drag = state.drag() + shift;
                state.settle = (!dragging).then(|| Tween::new(state.drag, surface.settle));
            }

            state.snap_seen = Some(snaps.current);
        }

        state.panel = panel;
        state.window = window;

        let offset = snaps.map_or(0.0, |snaps| {
            snap_offset(&snaps.points, snaps.current, window.height)
        });
        let travel = (state.drag() + offset).max(0.0);
        let hidden = state.hidden();

        let position = match &surface.placement {
            Placement::Center { inset, .. } => Point::new(
                inset.left + ((window.width - inset.x() - panel.width) / 2.0).max(0.0),
                inset.top + ((window.height - inset.y() - panel.height) / 2.0).max(0.0),
            ),
            Placement::Side {
                edge: Edge::Left, ..
            } => Point::new(-(travel + hidden * panel.width), 0.0),
            Placement::Side { .. } => Point::new(
                window.width - panel.width + travel + hidden * panel.width,
                0.0,
            ),
            Placement::Bottom { .. } => Point::new(
                (window.width - panel.width) / 2.0,
                window.height - panel.height + travel + hidden * panel.height,
            ),
        };

        layout::Node::with_children(window, vec![base, node.move_to(position)])
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let mut children = layout.children();

        if let Some(base_layout) = children.next() {
            self.base.as_widget_mut().operate(
                &mut tree.children[0],
                base_layout,
                renderer,
                operation,
            );
        }

        if let (Some(surface), Some(surface_layout)) = (self.surface.as_mut(), children.next()) {
            surface.content.as_widget_mut().operate(
                &mut tree.children[1],
                surface_layout,
                renderer,
                operation,
            );
        }
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
        let mut children = layout.children();
        let (Some(base_layout), surface_layout) = (children.next(), children.next()) else {
            return;
        };

        let (Some(surface), Some(surface_layout)) = (self.surface.as_mut(), surface_layout) else {
            self.base.as_widget_mut().update(
                &mut tree.children[0],
                event,
                base_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
            return;
        };

        let Tree {
            state: tree_state,
            children: tree_children,
            ..
        } = tree;
        let [base_tree, surface_tree] = tree_children.as_mut_slice() else {
            return;
        };
        let state = tree_state.downcast_mut::<State>();
        let bounds = surface_layout.bounds();

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            state.now = Some(*now);

            let mut moving = false;

            if let Some(entrance) = state.entrance.as_mut() {
                entrance.start.get_or_insert(*now);
                moving = true;

                if entrance.progress(state.now) >= 1.0 {
                    state.entrance = None;
                }
            }

            if let Some(settle) = state.settle.as_mut() {
                settle.start.get_or_insert(*now);
                moving = true;

                if settle.progress(state.now) >= 1.0 {
                    state.settle = None;
                    state.drag = 0.0;
                }
            }

            if moving {
                shell.invalidate_layout();
            }
            if state.entrance.is_some() || state.settle.is_some() {
                shell.request_redraw();
            }
        }

        if state.focus_pending {
            state.focus_pending = false;

            let _ = crate::focus::run(
                &mut self.base,
                base_tree,
                base_layout,
                renderer,
                operation::focusable::unfocus::<()>(),
            );
            shell.request_redraw();
        }

        let mut content_cursor = cursor;
        let mut forward = true;

        if let (Some(edge), true) = (surface.placement.edge(), surface.draggable) {
            match pointer(event, cursor) {
                Some(Pointer::Pressed(Some(position))) if bounds.contains(position) => {
                    let at = along(edge, position);

                    state.drag = state.drag();
                    state.settle = None;
                    state.press = Some(Press {
                        origin: at,
                        last: at,
                        dragging: false,
                        samples: VecDeque::from([(Instant::now(), at)]),
                    });
                }
                Some(Pointer::Moved(position)) => {
                    if let Some(press) = state.press.as_mut() {
                        let at = along(edge, position);

                        if !press.dragging && (at - press.origin).abs() > DRAG_SLOP {
                            press.dragging = true;
                            press.last = press.origin + DRAG_SLOP.copysign(at - press.origin);
                        }

                        if press.dragging {
                            let floor = surface.snaps.as_ref().map_or(0.0, |snaps| {
                                -snap_offset(&snaps.points, snaps.current, state.window.height)
                            });
                            let now = Instant::now();

                            state.drag = (state.drag + at - press.last).max(floor);
                            press.last = at;
                            press.samples.push_back((now, at));

                            while press.samples.len() > 2
                                && press.samples.front().is_some_and(|(time, _)| {
                                    now.saturating_duration_since(*time) > VELOCITY_WINDOW
                                })
                            {
                                press.samples.pop_front();
                            }

                            content_cursor = mouse::Cursor::Unavailable;
                            shell.invalidate_layout();
                            shell.request_redraw();
                        }
                    }
                }
                Some(Pointer::Released) => {
                    if let Some(press) = state.press.take()
                        && press.dragging
                    {
                        let extent = match edge {
                            Edge::Bottom => state.panel.height,
                            Edge::Left | Edge::Right => state.panel.width,
                        };
                        let dismissible = surface.dismissible && surface.on_dismiss.is_some();
                        let outcome = release(
                            state.drag,
                            velocity(&press.samples),
                            extent,
                            state.window.height,
                            dismissible,
                            surface
                                .snaps
                                .as_ref()
                                .map(|snaps| (snaps.points.as_slice(), snaps.current)),
                        );

                        match outcome {
                            Release::Dismiss => {
                                if let Some(message) = surface.on_dismiss.clone() {
                                    shell.publish(message);
                                }
                            }
                            Release::Snap(index) => {
                                if let Some(snaps) = surface.snaps.as_ref()
                                    && index != snaps.current
                                {
                                    shell.publish((snaps.on_snap)(index));
                                }
                                state.settle = Some(Tween::new(state.drag, surface.settle));
                            }
                            Release::Back => {
                                state.settle = Some(Tween::new(state.drag, surface.settle));
                            }
                        }

                        content_cursor = mouse::Cursor::Unavailable;
                        forward = matches!(event, Event::Mouse(_));
                        shell.capture_event();
                        shell.request_redraw();
                    }
                }
                _ => {}
            }
        }

        if forward {
            let captured = shell.is_event_captured();

            surface.content.as_widget_mut().update(
                surface_tree,
                event,
                surface_layout,
                content_cursor,
                renderer,
                clipboard,
                shell,
                &layout.bounds(),
            );

            if captured {
                return;
            }
        }

        if !shell.is_event_captured() {
            match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    let inside = surface.modal
                        || crate::focus::run(
                            &mut surface.content,
                            surface_tree,
                            surface_layout,
                            renderer,
                            has_focus(),
                        ) == Some(true);

                    if inside && let Some(message) = surface.on_dismiss.clone() {
                        shell.publish(message);
                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) if surface.modal => {
                    cycle(
                        &mut surface.content,
                        surface_tree,
                        surface_layout,
                        renderer,
                        !modifiers.shift(),
                    );
                    shell.capture_event();
                    shell.request_redraw();
                }
                Event::Mouse(mouse::Event::ButtonPressed(_))
                | Event::Touch(touch::Event::FingerPressed { .. })
                    if surface.modal =>
                {
                    let outside = match event {
                        Event::Touch(touch::Event::FingerPressed { position, .. }) => {
                            !bounds.contains(*position)
                        }
                        _ => cursor
                            .position()
                            .is_some_and(|point| !bounds.contains(point)),
                    };

                    if outside {
                        if surface.dismissible
                            && let Some(message) = surface.on_dismiss.clone()
                        {
                            shell.publish(message);
                        }
                        shell.capture_event();
                    }
                }
                _ => {}
            }
        }

        let to_base = if surface.modal {
            matches!(event, Event::Window(_))
        } else {
            matches!(event, Event::Window(_)) || !shell.is_event_captured()
        };

        if to_base {
            let base_cursor = if surface.modal || cursor.is_over(bounds) {
                mouse::Cursor::Unavailable
            } else {
                cursor
            };

            self.base.as_widget_mut().update(
                base_tree,
                event,
                base_layout,
                base_cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
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

        let mut children = layout.children();
        let Some(base_layout) = children.next() else {
            return;
        };

        let (surface, surface_layout) = match (&self.surface, children.next()) {
            (Some(surface), Some(surface_layout)) => (surface, surface_layout),
            _ => {
                self.base.as_widget().draw(
                    &tree.children[0],
                    renderer,
                    theme,
                    style,
                    base_layout,
                    cursor,
                    viewport,
                );
                return;
            }
        };

        let state = tree.state.downcast_ref::<State>();
        let bounds = surface_layout.bounds();
        let base_cursor = if surface.modal || cursor.is_over(bounds) {
            mouse::Cursor::Unavailable
        } else {
            cursor
        };

        self.base.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            base_layout,
            base_cursor,
            viewport,
        );

        let window = layout.bounds();

        renderer.with_layer(window, |renderer| {
            if surface.modal {
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: window,
                        ..renderer::Quad::default()
                    },
                    theme
                        .roles()
                        .scrim
                        .scale_alpha(surface.scrim * (1.0 - state.hidden())),
                );
            }

            surface.content.as_widget().draw(
                &tree.children[1],
                renderer,
                theme,
                style,
                surface_layout,
                cursor,
                &window,
            );
        });
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &iced::Renderer,
    ) -> mouse::Interaction {
        let mut children = layout.children();
        let Some(base_layout) = children.next() else {
            return mouse::Interaction::None;
        };

        if let (Some(surface), Some(surface_layout)) = (&self.surface, children.next()) {
            let bounds = surface_layout.bounds();

            if cursor.is_over(bounds) {
                let interaction = surface.content.as_widget().mouse_interaction(
                    &tree.children[1],
                    surface_layout,
                    cursor,
                    &layout.bounds(),
                    renderer,
                );

                return if interaction == mouse::Interaction::None {
                    mouse::Interaction::Idle
                } else {
                    interaction
                };
            }

            if surface.modal {
                return mouse::Interaction::Idle;
            }
        }

        self.base.as_widget().mouse_interaction(
            &tree.children[0],
            base_layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &iced::Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, iced::Renderer>> {
        let mut children = layout.children();
        let base_layout = children.next()?;

        let (Some(surface), Some(surface_layout)) = (self.surface.as_mut(), children.next()) else {
            return self.base.as_widget_mut().overlay(
                &mut tree.children[0],
                base_layout,
                renderer,
                viewport,
                translation,
            );
        };

        let [base_tree, surface_tree] = tree.children.as_mut_slice() else {
            return None;
        };

        let surface_overlay = surface.content.as_widget_mut().overlay(
            surface_tree,
            surface_layout,
            renderer,
            &layout.bounds(),
            translation,
        );

        if surface.modal {
            return surface_overlay;
        }

        let base_overlay = self.base.as_widget_mut().overlay(
            base_tree,
            base_layout,
            renderer,
            viewport,
            translation,
        );

        let overlays: Vec<_> = base_overlay.into_iter().chain(surface_overlay).collect();

        (!overlays.is_empty()).then(|| overlay::Group::with_children(overlays).overlay())
    }
}

/// Moves the focus to the next focusable widget of `content`, or the
/// previous one when not `forward`, wrapping around at the ends.
fn cycle<Message>(
    content: &mut Element<'_, Message>,
    tree: &mut Tree,
    layout: Layout<'_>,
    renderer: &iced::Renderer,
    forward: bool,
) {
    let Some(count) = crate::focus::run(
        content,
        tree,
        layout,
        renderer,
        operation::focusable::count(),
    ) else {
        return;
    };

    let wrap = match (count.focused, forward) {
        (None, _) => true,
        (Some(focused), true) => focused + 1 >= count.total,
        (Some(focused), false) => focused == 0,
    };

    let _ = match (wrap, forward) {
        (true, true) => crate::focus::run(content, tree, layout, renderer, focus_first::<()>()),
        (true, false) => crate::focus::run(content, tree, layout, renderer, focus_last::<()>()),
        (false, true) => crate::focus::run(
            content,
            tree,
            layout,
            renderer,
            operation::focusable::focus_next::<()>(),
        ),
        (false, false) => crate::focus::run(
            content,
            tree,
            layout,
            renderer,
            operation::focusable::focus_previous::<()>(),
        ),
    };
}

impl<'a, Message> From<Layer<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(layer: Layer<'a, Message>) -> Self {
        Element::new(layer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEIGHT: f32 = 1000.0;

    #[test]
    fn a_short_drag_settles_back_and_a_long_one_dismisses() {
        assert_eq!(release(40.0, 0.0, 360.0, HEIGHT, true, None), Release::Back);
        assert_eq!(
            release(300.0, 0.0, 360.0, HEIGHT, true, None),
            Release::Dismiss
        );
        assert_eq!(
            release(40.0, 900.0, 360.0, HEIGHT, true, None),
            Release::Dismiss
        );
        assert_eq!(
            release(300.0, 900.0, 360.0, HEIGHT, false, None),
            Release::Back
        );
    }

    #[test]
    fn a_drag_rests_at_the_nearest_height() {
        let points = [0.3, 0.8];

        // From 80%: dragged 400 px down rests at 30%.
        assert_eq!(
            release(400.0, 0.0, 800.0, HEIGHT, true, Some((&points, 1))),
            Release::Snap(0)
        );
        // From 30%: dragged 450 px up rests at 80%.
        assert_eq!(
            release(-450.0, 0.0, 800.0, HEIGHT, true, Some((&points, 0))),
            Release::Snap(1)
        );
        // A small drag stays.
        assert_eq!(
            release(30.0, 0.0, 800.0, HEIGHT, true, Some((&points, 1))),
            Release::Snap(1)
        );
    }

    #[test]
    fn a_fling_moves_one_height_and_below_the_lowest_dismisses() {
        let points = [0.3, 0.8];

        assert_eq!(
            release(30.0, 900.0, 800.0, HEIGHT, true, Some((&points, 1))),
            Release::Snap(0)
        );
        assert_eq!(
            release(-30.0, -900.0, 800.0, HEIGHT, true, Some((&points, 0))),
            Release::Snap(1)
        );
        assert_eq!(
            release(30.0, 900.0, 800.0, HEIGHT, true, Some((&points, 0))),
            Release::Dismiss
        );
        assert_eq!(
            release(30.0, 900.0, 800.0, HEIGHT, false, Some((&points, 0))),
            Release::Snap(0)
        );
        // Past half of the lowest height.
        assert_eq!(
            release(200.0, 0.0, 800.0, HEIGHT, true, Some((&points, 0))),
            Release::Dismiss
        );
    }

    #[test]
    fn snap_offsets_measure_from_the_highest() {
        let points = [0.3, 0.8];

        assert_eq!(snap_offset(&points, 1, HEIGHT), 0.0);
        assert!((snap_offset(&points, 0, HEIGHT) - 500.0).abs() < 0.01);
        assert_eq!(snap_offset(&points, 5, HEIGHT), 0.0);
    }

    #[test]
    fn tweens_go_from_their_start_to_zero() {
        let start = Instant::now();
        let mut tween = Tween::new(1.0, Duration::from_millis(250));

        assert_eq!(tween.value(Some(start)), 1.0);
        tween.start = Some(start);
        assert_eq!(tween.value(Some(start)), 1.0);
        assert_eq!(tween.value(Some(start + Duration::from_millis(250))), 0.0);
        assert!(tween.value(Some(start + Duration::from_millis(100))) < 1.0);
    }

    #[test]
    fn the_release_speed_spans_at_least_the_window() {
        let start = Instant::now();
        let samples = VecDeque::from([(start, 0.0), (start + Duration::from_millis(1), 40.0)]);

        assert!((velocity(&samples) - 400.0).abs() < 0.01);
    }
}
