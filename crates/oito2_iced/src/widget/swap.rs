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

//! Swap: an animated change between two visual states, such as a copy
//! icon that turns into a check mark or a play button that becomes pause.
//!
//! [`swap`] shows `on` while `swapped` is true and `off` otherwise. When
//! `swapped` changes, the two children overlap, centered, for
//! [`Swap::duration`] while the [`SwapEffect`] runs: the incoming child
//! follows an ease-out curve and the outgoing one an ease-in curve. A swap
//! shows its final state the first time it is drawn, and
//! [`Swap::animated`] set to false changes at once, for reduced motion.
//!
//! Iced 0.14 draws a child with neither opacity nor rotation, so the
//! effects keep what it can draw: [`SwapEffect::Scale`] grows the incoming
//! child and shrinks the outgoing one around their centers;
//! [`SwapEffect::Flip`] narrows the outgoing child to its vertical center
//! line and widens the incoming one from it, each shown for its half of
//! the turn; [`SwapEffect::Fade`] and [`SwapEffect::Rotate`] show the child
//! that is more visible at each moment.
//!
//! ```no_run
//! use oito2_iced::widget::swap::{self, SwapEffect};
//!
//! # fn view<'a>(playing: bool) -> oito2_iced::Element<'a, ()> {
//! swap::swap(playing, iced::widget::text("Play"), iced::widget::text("Pause"))
//!     .effect(SwapEffect::Scale)
//!     .into()
//! # }
//! ```

use std::f32::consts::PI;
use std::time::Duration;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::operation::Operation;
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, overlay, renderer};
use iced::time::Instant;
use iced::{
    Element as IcedElement, Event, Length, Point, Rectangle, Size, Transformation, Vector, mouse,
    window,
};

use crate::{Element, Theme};

/// The default length of the change.
pub const DURATION: Duration = Duration::from_millis(200);

/// How a [`Swap`] changes between its two children.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum SwapEffect {
    /// A cross-fade, drawn as a change to the more visible child.
    #[default]
    Fade,
    /// The incoming child grows in while the outgoing one shrinks away.
    Scale,
    /// A quarter turn with a fade, drawn as a change to the more visible
    /// child.
    Rotate,
    /// The children flip around the vertical axis.
    Flip,
}

/// Two children with an animated change between them. See [`swap`].
pub struct Swap<'a, Message> {
    swapped: bool,
    off: Element<'a, Message>,
    on: Element<'a, Message>,
    effect: SwapEffect,
    duration: Duration,
    animated: bool,
}

/// A swap showing `on` while `swapped` is true and `off` otherwise.
pub fn swap<'a, Message>(
    swapped: bool,
    off: impl Into<Element<'a, Message>>,
    on: impl Into<Element<'a, Message>>,
) -> Swap<'a, Message> {
    Swap {
        swapped,
        off: off.into(),
        on: on.into(),
        effect: SwapEffect::default(),
        duration: DURATION,
        animated: true,
    }
}

impl<Message> Swap<'_, Message> {
    /// Sets the change effect ([`SwapEffect::Fade`] by default).
    #[must_use]
    pub fn effect(mut self, effect: SwapEffect) -> Self {
        self.effect = effect;
        self
    }

    /// Sets the length of the change ([`DURATION`] by default).
    #[must_use]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// Sets whether the change animates (true by default); false changes
    /// at once, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// The index of the child shown for `swapped`: 1 for `on`, 0 for
    /// `off`.
    fn index(swapped: bool) -> usize {
        usize::from(swapped)
    }
}

/// A running change.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Transition {
    /// The child that leaves.
    from: bool,
    /// The first frame of the change; `None` until it is drawn.
    started: Option<Instant>,
}

/// The tree state of a [`Swap`].
#[derive(Debug, Clone, Default)]
struct State {
    /// The child shown at rest.
    shown: bool,
    transition: Option<Transition>,
    now: Option<Instant>,
}

impl State {
    /// The linear progress of the change, from 0 to 1.
    fn progress(&self, duration: Duration) -> f32 {
        match self.transition {
            Some(Transition {
                started: Some(started),
                ..
            }) => match self.now {
                Some(now) if !duration.is_zero() => {
                    (now.saturating_duration_since(started).as_secs_f32() / duration.as_secs_f32())
                        .clamp(0.0, 1.0)
                }
                _ => 1.0,
            },
            Some(_) => 0.0,
            None => 1.0,
        }
    }
}

/// How a child is drawn in a frame of the change.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Look {
    /// Not drawn.
    Hidden,
    /// Drawn at its size.
    Whole,
    /// Drawn scaled around its center by the factor.
    Scaled(f32),
    /// Drawn clipped to the centered fraction of its width.
    Narrowed(f32),
}

/// How the child entering at `incoming` and the one leaving at `outgoing`
/// look with `effect`; the values are the curved animation values, 1 for
/// fully shown.
fn looks(effect: SwapEffect, incoming: f32, outgoing: f32) -> (Look, Look) {
    match effect {
        SwapEffect::Fade | SwapEffect::Rotate => {
            if incoming >= outgoing {
                (Look::Whole, Look::Hidden)
            } else {
                (Look::Hidden, Look::Whole)
            }
        }
        SwapEffect::Scale => (Look::Scaled(incoming), Look::Scaled(outgoing)),
        SwapEffect::Flip => {
            let flip = |value: f32| {
                if value < 0.5 {
                    Look::Hidden
                } else {
                    Look::Narrowed(((1.0 - value) * PI).cos().abs())
                }
            };

            (flip(incoming), flip(outgoing))
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Swap<'_, Message> {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            shown: self.swapped,
            ..State::default()
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.off), Tree::new(&self.on)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.off, &self.on]);

        let state = tree.state.downcast_mut::<State>();

        if state.shown != self.swapped {
            state.transition = (self.animated && !self.duration.is_zero()).then_some(Transition {
                from: state.shown,
                started: None,
            });
            state.shown = self.swapped;
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
        let state = tree.state.downcast_ref::<State>();
        let shown = Self::index(self.swapped);
        let leaving = state
            .transition
            .map(|transition| Self::index(transition.from));
        let limits = limits.loose();

        let mut nodes: Vec<layout::Node> = [&mut self.off, &mut self.on]
            .into_iter()
            .zip(tree.children.iter_mut())
            .map(|(child, tree)| child.as_widget_mut().layout(tree, renderer, &limits))
            .collect();

        // At rest the swap takes the size of the shown child; during a
        // change it takes the larger of both.
        let mut size = nodes[shown].size();

        if let Some(leaving) = leaving {
            let other = nodes[leaving].size();

            size = Size::new(size.width.max(other.width), size.height.max(other.height));
        }

        for node in &mut nodes {
            let child = node.size();

            node.move_to_mut(Point::new(
                (size.width - child.width) / 2.0,
                (size.height - child.height) / 2.0,
            ));
        }

        layout::Node::with_children(size, nodes)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &iced::Renderer,
        operation: &mut dyn Operation,
    ) {
        let shown = Self::index(self.swapped);
        let Some(child_layout) = layout.children().nth(shown) else {
            return;
        };
        let child = if self.swapped {
            &mut self.on
        } else {
            &mut self.off
        };

        operation.container(None, layout.bounds());
        operation.traverse(&mut |operation| {
            child.as_widget_mut().operate(
                &mut tree.children[shown],
                child_layout,
                renderer,
                operation,
            );
        });
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
        let shown = Self::index(self.swapped);

        if let Some(child_layout) = layout.children().nth(shown) {
            let child = if self.swapped {
                &mut self.on
            } else {
                &mut self.off
            };

            child.as_widget_mut().update(
                &mut tree.children[shown],
                event,
                child_layout,
                cursor,
                renderer,
                clipboard,
                shell,
                viewport,
            );
        }

        let state = tree.state.downcast_mut::<State>();

        let Some(transition) = &mut state.transition else {
            return;
        };

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let started = *transition.started.get_or_insert(*now);

            state.now = Some(*now);

            if now.saturating_duration_since(started) >= self.duration {
                state.transition = None;
                shell.invalidate_layout();
            }
        }

        shell.request_redraw();
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
        let state = tree.state.downcast_ref::<State>();
        let children: Vec<Layout<'_>> = layout.children().collect();
        let shown = Self::index(self.swapped);
        let elements = [&self.off, &self.on];

        let draw = |renderer: &mut iced::Renderer, index: usize, look: Look| {
            let (Some(child), Some(child_layout)) =
                (elements.get(index), children.get(index).copied())
            else {
                return;
            };
            let bounds = child_layout.bounds();
            let paint = |renderer: &mut iced::Renderer, cursor: mouse::Cursor| {
                child.as_widget().draw(
                    &tree.children[index],
                    renderer,
                    theme,
                    style,
                    child_layout,
                    cursor,
                    viewport,
                );
            };

            match look {
                Look::Hidden => {}
                Look::Whole => paint(renderer, cursor),
                Look::Scaled(scale) if scale > 0.001 => {
                    let center = bounds.center();
                    let transformation = Transformation::translate(center.x, center.y)
                        * Transformation::scale(scale)
                        * Transformation::translate(-center.x, -center.y);

                    renderer.with_transformation(transformation, |renderer| {
                        paint(renderer, mouse::Cursor::Unavailable);
                    });
                }
                Look::Scaled(_) => {}
                Look::Narrowed(fraction) => {
                    let width = bounds.width * fraction;
                    let clip = Rectangle {
                        x: bounds.center_x() - width / 2.0,
                        width,
                        ..bounds
                    };

                    renderer.with_layer(clip, |renderer| {
                        paint(renderer, mouse::Cursor::Unavailable);
                    });
                }
            }
        };

        match state.transition {
            Some(transition) if transition.from != self.swapped => {
                let progress = state.progress(self.duration);
                let incoming = ease_out(progress);
                let outgoing = ease_in(1.0 - progress);
                let (look_in, look_out) = looks(self.effect, incoming, outgoing);

                draw(renderer, Self::index(transition.from), look_out);
                draw(renderer, shown, look_in);
            }
            _ => draw(renderer, shown, Look::Whole),
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
        let shown = Self::index(self.swapped);
        let child = if self.swapped { &self.on } else { &self.off };

        layout
            .children()
            .nth(shown)
            .map(|child_layout| {
                child.as_widget().mouse_interaction(
                    &tree.children[shown],
                    child_layout,
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
        let shown = Self::index(self.swapped);
        let child_layout = layout.children().nth(shown)?;
        let child = if self.swapped {
            &mut self.on
        } else {
            &mut self.off
        };

        child.as_widget_mut().overlay(
            &mut tree.children[shown],
            child_layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Message: 'a> From<Swap<'a, Message>> for Element<'a, Message> {
    fn from(swap: Swap<'a, Message>) -> Self {
        IcedElement::new(swap)
    }
}

/// The ease-out curve: control points (0, 0) and (0.58, 1).
pub(crate) fn ease_out(t: f32) -> f32 {
    cubic((0.0, 0.0), (0.58, 1.0), t)
}

/// The ease-in curve: control points (0.42, 0) and (1, 1).
pub(crate) fn ease_in(t: f32) -> f32 {
    cubic((0.42, 0.0), (1.0, 1.0), t)
}

/// A cubic Bézier from (0, 0) to (1, 1) with the control points `a` and
/// `b`, evaluated at x = `t` by bisection.
fn cubic(a: (f32, f32), b: (f32, f32), t: f32) -> f32 {
    let at = |p1: f32, p2: f32, m: f32| {
        3.0 * p1 * (1.0 - m) * (1.0 - m) * m + 3.0 * p2 * (1.0 - m) * m * m + m * m * m
    };

    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let (mut low, mut high) = (0.0_f32, 1.0_f32);

    for _ in 0..24 {
        let middle = (low + high) / 2.0;

        if at(a.0, b.0, middle) < t {
            low = middle;
        } else {
            high = middle;
        }
    }

    at(a.1, b.1, (low + high) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_start_and_end_in_place() {
        assert_eq!(ease_out(0.0), 0.0);
        assert_eq!(ease_in(1.0), 1.0);
        assert!(ease_out(0.5) > 0.5);
        assert!(ease_in(0.5) < 0.5);
    }

    #[test]
    fn a_flip_shows_each_child_for_its_half() {
        assert_eq!(
            looks(SwapEffect::Flip, 0.2, 0.8).0,
            Look::Hidden,
            "the incoming child waits for its half"
        );
        assert!(
            matches!(looks(SwapEffect::Flip, 0.2, 0.8).1, Look::Narrowed(width) if width > 0.5)
        );
        assert_eq!(looks(SwapEffect::Flip, 1.0, 0.0).0, Look::Narrowed(1.0));
        assert_eq!(
            looks(SwapEffect::Fade, 0.7, 0.3),
            (Look::Whole, Look::Hidden)
        );
    }
}
