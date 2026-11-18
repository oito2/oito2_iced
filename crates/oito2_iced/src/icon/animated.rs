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

//! Animated icons drawn on a 24-unit grid.
//!
//! [`icon`] draws one of the animated icons, such as [`OK`] or [`COMPASS`],
//! in the stroke style of the Lucide icons: 2-unit round strokes on a
//! 24-unit grid scaled to the icon size. The `filled` variants cut their
//! inner mark out of the fill, so the background shows through it.
//!
//! The [`Mode`] sets how the icon plays: [`Mode::Once`] when it first shows
//! and again whenever its [`Data`] or mode changes, [`Mode::Repeat`] over
//! and over, and [`Mode::Still`] not at all. [`AnimatedIcon::animated`] set
//! to false shows the final frame, for reduced motion. The icon requests
//! redraws only while it plays.
//!
//! A custom icon is a [`Data`] made of a [`Painter`], which draws a frame
//! on a [`Canvas`] in grid units, and the length of one play.
//!
//! ```no_run
//! use oito2_iced::icon::animated::{self, Mode};
//!
//! # fn view<'a>() -> oito2_iced::Element<'a, ()> {
//! # let _: oito2_iced::Element<'a, ()> =
//! animated::icon(animated::OK).into();
//! animated::icon(animated::COMPASS)
//!     .size(40.0)
//!     .mode(Mode::Repeat)
//!     .into()
//! # }
//! ```

use std::f32::consts::PI;
use std::time::Duration;

use iced::advanced::graphics::geometry::Renderer as _;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Tree, Widget, tree};
use iced::advanced::{Clipboard, Renderer as _, Shell, renderer};
use iced::time::Instant;
use iced::widget::canvas::{self, Fill, LineCap, LineJoin, Path, Stroke, fill::Rule};
use iced::{
    Color, Element as IcedElement, Event, Length, Point, Rectangle, Size, Vector, mouse, window,
};

use crate::animation::{ease_out, ease_out_back};
use crate::{Element, Theme};

/// The side of the grid the painters draw on.
pub const GRID: f32 = 24.0;

/// The default length of one play.
pub const DURATION: Duration = Duration::from_millis(700);

/// How an [`AnimatedIcon`] plays its animation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Mode {
    /// Once, when the icon first shows and whenever its data or mode
    /// changes.
    #[default]
    Once,
    /// Over and over.
    Repeat,
    /// Not at all: the icon shows its final frame.
    Still,
}

/// Draws one frame of an animated icon on a [`Canvas`] in grid units: the
/// progress runs from 0 to 1, and the color is the icon color.
pub type Painter = fn(&mut Canvas<'_>, f32, Color);

/// The drawing and timing of an animated icon, such as [`OK`].
#[derive(Debug, Clone, Copy)]
pub struct Data {
    painter: Painter,
    duration: Duration,
}

impl Data {
    /// Data that draws with `painter` and plays in [`DURATION`].
    pub const fn new(painter: Painter) -> Self {
        Self {
            painter,
            duration: DURATION,
        }
    }

    /// Sets the length of one play.
    #[must_use]
    pub const fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    /// The painter that draws a frame.
    pub const fn painter(self) -> Painter {
        self.painter
    }

    /// The length of one play.
    pub const fn duration(self) -> Duration {
        self.duration
    }
}

impl PartialEq for Data {
    /// Two data are equal when they have the same painter function and the
    /// same duration.
    fn eq(&self, other: &Self) -> bool {
        std::ptr::fn_addr_eq(self.painter, other.painter) && self.duration == other.duration
    }
}

/// A circle that draws itself, then a check mark.
pub const OK: Data = Data::new(paint_ok);

/// A filled circle that pops in, then a check mark cut out of it.
pub const OK_FILLED: Data = Data::new(paint_ok_filled);

/// A heart outline that draws itself, then beats.
pub const HEART: Data = Data::new(paint_heart);

/// A filled heart that pops in.
pub const HEART_FILLED: Data = Data::new(paint_heart_filled);

/// A star outline that draws itself.
pub const STAR: Data = Data::new(paint_star);

/// A filled star that spins and pops in.
pub const STAR_FILLED: Data = Data::new(paint_star_filled);

/// A star outline whose left half fills.
pub const STAR_HALF_FILLED: Data = Data::new(paint_star_half_filled);

/// A compass whose needle, with a filled north half, swings and settles
/// north.
pub const COMPASS: Data = Data::new(paint_compass).with_duration(Duration::from_millis(1200));

/// A filled compass whose cut-out needle swings and settles north.
pub const COMPASS_FILLED: Data =
    Data::new(paint_compass_filled).with_duration(Duration::from_millis(1200));

/// Wi-Fi waves that appear one by one, then a slash across them.
pub const NO_NETWORK: Data = Data::new(paint_no_network).with_duration(Duration::from_millis(900));

/// The surface a [`Painter`] draws on: a canvas frame scaled so that the
/// icon box spans [`GRID`] units on each side.
///
/// The frame scales the geometry but not the stroke widths, so
/// [`Canvas::stroke`] takes its width in grid units and scales it.
pub struct Canvas<'a> {
    frame: &'a mut canvas::Frame,
    scale: f32,
}

impl Canvas<'_> {
    /// The canvas frame, already scaled to the grid.
    pub fn frame(&mut self) -> &mut canvas::Frame {
        self.frame
    }

    /// The pixels per grid unit.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Strokes `path` in `color` with round caps and joins, `width` grid
    /// units wide.
    pub fn stroke(&mut self, path: &Path, color: Color, width: f32) {
        self.frame.stroke(
            path,
            Stroke::default()
                .with_color(color)
                .with_width(width * self.scale)
                .with_line_cap(LineCap::Round)
                .with_line_join(LineJoin::Round),
        );
    }

    /// Fills `path` in `color` with the non-zero rule.
    pub fn fill(&mut self, path: &Path, color: Color) {
        self.frame.fill(path, color);
    }

    /// Fills `path` in `color` with the even-odd rule, so its inner
    /// contours cut holes in the outer ones.
    pub fn fill_even_odd(&mut self, path: &Path, color: Color) {
        self.frame.fill(
            path,
            Fill {
                rule: Rule::EvenOdd,
                ..Fill::from(color)
            },
        );
    }
}

/// An animated icon drawing `data`, at the icon size of the current
/// [density](crate::density) in the text color of its parent, played
/// [`Mode::Once`].
pub fn icon(data: Data) -> AnimatedIcon {
    AnimatedIcon {
        data,
        size: None,
        color: None,
        mode: Mode::default(),
        duration: None,
        animated: true,
    }
}

/// An animated icon drawn as a widget. See [`icon`].
#[derive(Debug, Clone, Copy)]
pub struct AnimatedIcon {
    data: Data,
    size: Option<f32>,
    color: Option<Color>,
    mode: Mode,
    duration: Option<Duration>,
    animated: bool,
}

impl AnimatedIcon {
    /// Sets the width and height, instead of the icon size of the density.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the color, instead of the text color of the parent.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets how the animation plays ([`Mode::Once`] by default).
    #[must_use]
    pub fn mode(mut self, mode: Mode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the length of one play, instead of the duration of the data.
    #[must_use]
    pub fn duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Sets whether the icon animates (true by default); false shows the
    /// final frame, for reduced motion.
    #[must_use]
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    fn side(&self) -> f32 {
        self.size.unwrap_or_else(crate::density::icon_size)
    }

    fn play_duration(&self) -> Duration {
        self.duration.unwrap_or(self.data.duration)
    }

    /// Whether the icon moves at all: animated and not still.
    fn moving(&self) -> bool {
        self.animated && self.mode != Mode::Still
    }

    /// The progress of the frame to draw, from 0 to 1.
    fn progress(&self, state: &State) -> f32 {
        if !self.moving() || state.done {
            return 1.0;
        }

        let (Some(started), Some(now)) = (state.started, state.now) else {
            return 0.0;
        };
        let duration = self.play_duration().as_secs_f64();

        if duration <= 0.0 {
            return 1.0;
        }

        let elapsed = now.saturating_duration_since(started).as_secs_f64() / duration;

        match self.mode {
            Mode::Repeat => elapsed.fract() as f32,
            Mode::Once | Mode::Still => elapsed.min(1.0) as f32,
        }
    }
}

/// The tree state of an [`AnimatedIcon`].
#[derive(Debug, Clone, Copy)]
struct State {
    /// The data of the current play.
    data: Data,
    /// The mode of the current play.
    mode: Mode,
    /// The first frame of the play; `None` until it is drawn.
    started: Option<Instant>,
    /// The time of the latest frame.
    now: Option<Instant>,
    /// Whether a play [`Mode::Once`] has reached its end.
    done: bool,
}

impl State {
    fn new(data: Data, mode: Mode) -> Self {
        Self {
            data,
            mode,
            started: None,
            now: None,
            done: false,
        }
    }
}

impl<Message> Widget<Message, Theme, iced::Renderer> for AnimatedIcon {
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::new(self.data, self.mode))
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree.state.downcast_mut::<State>();

        // A new drawing or mode starts a new play.
        if state.data != self.data || state.mode != self.mode {
            *state = State::new(self.data, self.mode);
        }
    }

    fn size(&self) -> Size<Length> {
        let side = Length::Fixed(self.side());

        Size::new(side, side)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let side = self.side();

        layout::Node::new(limits.resolve(side, side, Size::new(side, side)))
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &iced::Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        if !self.moving() || state.done {
            return;
        }

        if let Event::Window(window::Event::RedrawRequested(now)) = event {
            let started = *state.started.get_or_insert(*now);

            state.now = Some(*now);

            if self.mode == Mode::Once
                && now.saturating_duration_since(started) >= self.play_duration()
            {
                state.done = true;
                return;
            }
        }

        shell.request_redraw();
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut iced::Renderer,
        _theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        if bounds.intersection(viewport).is_none() {
            return;
        }

        let state = tree.state.downcast_ref::<State>();
        let color = self.color.unwrap_or(style.text_color);
        let scale = bounds.width.min(bounds.height) / GRID;
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        frame.scale(scale);
        (self.data.painter)(
            &mut Canvas {
                frame: &mut frame,
                scale,
            },
            self.progress(state),
            color,
        );

        let geometry = frame.into_geometry();

        renderer.with_translation(bounds.position() - Point::ORIGIN, |renderer| {
            renderer.draw_geometry(geometry);
        });
    }
}

impl<'a, Message: 'a> From<AnimatedIcon> for Element<'a, Message> {
    fn from(icon: AnimatedIcon) -> Self {
        IcedElement::new(icon)
    }
}

// Geometry on the grid: shapes are flattened into polylines, so they can be
// cut by length, transformed and clipped before they become paths.

/// The center of the grid.
const CENTER: Point = Point::new(12.0, 12.0);

/// The stroke width, in grid units.
const STROKE: f32 = 2.0;

/// The segments of a full circle.
const CIRCLE_SEGMENTS: usize = 96;

/// The segments of a cubic Bézier curve.
const CUBIC_SEGMENTS: usize = 24;

/// A polyline, closed or open.
#[derive(Debug, Clone, PartialEq)]
struct Contour {
    points: Vec<Point>,
    closed: bool,
}

impl Contour {
    fn closed(points: Vec<Point>) -> Self {
        Self {
            points,
            closed: true,
        }
    }

    fn open(points: Vec<Point>) -> Self {
        Self {
            points,
            closed: false,
        }
    }

    /// The contour with `map` applied to every point.
    fn map(&self, map: impl Fn(Point) -> Point) -> Self {
        Self {
            points: self.points.iter().copied().map(map).collect(),
            closed: self.closed,
        }
    }

    /// The segments of the contour, the closing one included.
    fn segments(&self) -> impl Iterator<Item = (Point, Point)> + '_ {
        let closing = self
            .closed
            .then(|| {
                self.points
                    .last()
                    .copied()
                    .zip(self.points.first().copied())
            })
            .flatten();

        self.points
            .windows(2)
            .map(|pair| (pair[0], pair[1]))
            .chain(closing)
    }

    /// The length of the contour.
    fn length(&self) -> f32 {
        self.segments().map(|(a, b)| a.distance(b)).sum()
    }

    /// The first `fraction` of the contour, by length, as an open
    /// polyline; `None` while `fraction` is 0, so round caps draw no dot.
    /// The whole contour for a `fraction` of 1.
    fn partial(&self, fraction: f32) -> Option<Self> {
        if fraction <= 0.0 {
            return None;
        }
        if fraction >= 1.0 {
            return Some(self.clone());
        }

        let first = *self.points.first()?;
        let target = self.length() * fraction;
        let mut points = vec![first];
        let mut walked = 0.0;

        for (a, b) in self.segments() {
            let length = a.distance(b);

            if length <= 0.0 {
                continue;
            }

            if walked + length >= target {
                points.push(a + (b - a) * ((target - walked) / length));
                break;
            }

            points.push(b);
            walked += length;
        }

        Some(Self::open(points))
    }

    /// The part of the closed contour on the left of the vertical line at
    /// `x`, as one polygon.
    fn clip_left(&self, x: f32) -> Self {
        let mut points = Vec::new();
        let cross = |a: Point, b: Point| {
            let t = (x - a.x) / (b.x - a.x);

            Point::new(x, a.y + (b.y - a.y) * t)
        };

        for (previous, current) in self.segments() {
            let (inside_previous, inside_current) = (previous.x <= x, current.x <= x);

            if inside_current {
                if !inside_previous {
                    points.push(cross(previous, current));
                }
                points.push(current);
            } else if inside_previous {
                points.push(cross(previous, current));
            }
        }

        Self::closed(points)
    }
}

/// Builds a path from `contours`.
fn path(contours: &[Contour]) -> Path {
    Path::new(|builder| {
        for contour in contours {
            let mut points = contour.points.iter();

            let Some(first) = points.next() else {
                continue;
            };

            builder.move_to(*first);

            for point in points {
                builder.line_to(*point);
            }

            if contour.closed {
                builder.close();
            }
        }
    })
}

/// The point at `angle` radians on the circle around `center` of `radius`;
/// angles grow clockwise from the right, as the y axis points down.
fn polar(center: Point, radius: f32, angle: f32) -> Point {
    Point::new(
        center.x + radius * angle.cos(),
        center.y + radius * angle.sin(),
    )
}

/// The points of the arc around `center` of `radius` from `start` sweeping
/// `sweep` radians, both ends included.
fn arc(center: Point, radius: f32, start: f32, sweep: f32) -> Vec<Point> {
    let segments = ((sweep.abs() / (2.0 * PI) * CIRCLE_SEGMENTS as f32).ceil() as usize).max(4);

    (0..=segments)
        .map(|step| {
            polar(
                center,
                radius,
                start + sweep * step as f32 / segments as f32,
            )
        })
        .collect()
}

/// A circle around `center` of `radius`, starting on its right and going
/// clockwise.
fn circle(center: Point, radius: f32) -> Contour {
    let mut points = arc(center, radius, 0.0, 2.0 * PI);

    points.pop();

    Contour::closed(points)
}

/// The points of the cubic Bézier curve from `from` with the controls `a`
/// and `b` to `to`, without `from`.
fn cubic(from: Point, a: Point, b: Point, to: Point) -> impl Iterator<Item = Point> {
    (1..=CUBIC_SEGMENTS).map(move |step| {
        let t = step as f32 / CUBIC_SEGMENTS as f32;
        let u = 1.0 - t;
        let weights = [u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t];
        let points = [from, a, b, to];

        let (x, y) = weights
            .iter()
            .zip(points)
            .fold((0.0, 0.0), |(x, y), (weight, point)| {
                (x + weight * point.x, y + weight * point.y)
            });

        Point::new(x, y)
    })
}

/// The point `point` scaled by `scale` around `center`.
fn scale_around(point: Point, center: Point, scale: f32) -> Point {
    center + (point - center) * scale
}

/// The point `point` turned by `angle` radians around `center`, clockwise
/// for a positive angle.
fn rotate_around(point: Point, center: Point, angle: f32) -> Point {
    let (sin, cos) = angle.sin_cos();
    let offset = point - center;

    center
        + Vector::new(
            offset.x * cos - offset.y * sin,
            offset.x * sin + offset.y * cos,
        )
}

/// How far `t` has gone through the part of the animation from `start` to
/// `end`, from 0 to 1.
fn phase(t: f32, start: f32, end: f32) -> f32 {
    ((t - start) / (end - start)).clamp(0.0, 1.0)
}

/// The circle of the ok and compass icons.
fn ring() -> Contour {
    circle(CENTER, 10.0)
}

/// The check mark of the ok icons.
fn check() -> Contour {
    Contour::open(vec![
        Point::new(7.5, 12.5),
        Point::new(10.5, 15.5),
        Point::new(16.5, 9.0),
    ])
}

/// The heart outline.
fn heart() -> Contour {
    let curves = [
        [(6.0, 16.0), (2.0, 12.5), (2.0, 8.5)],
        [(2.0, 5.5), (4.5, 3.5), (7.2, 3.5)],
        [(9.2, 3.5), (10.9, 4.6), (12.0, 6.3)],
        [(13.1, 4.6), (14.8, 3.5), (16.8, 3.5)],
        [(19.5, 3.5), (22.0, 5.5), (22.0, 8.5)],
        [(22.0, 12.5), (18.0, 16.0), (12.0, 20.5)],
    ];
    let mut points = vec![Point::new(12.0, 20.5)];

    for [a, b, to] in curves {
        let from = points.last().copied().unwrap_or(Point::ORIGIN);

        points.extend(cubic(from, a.into(), b.into(), to.into()));
    }

    // The last curve ends at the start, which the closing segment joins.
    points.pop();

    Contour::closed(points)
}

/// The center of the star.
const STAR_CENTER: Point = Point::new(12.0, 12.8);

/// The five-pointed star outline.
fn star() -> Contour {
    Contour::closed(
        (0..10)
            .map(|i| {
                let radius = if i % 2 == 0 { 10.0 } else { 4.4 };

                polar(STAR_CENTER, radius, -PI / 2.0 + i as f32 * PI / 5.0)
            })
            .collect(),
    )
}

/// The compass needle pointing north, turned by `angle` radians around the
/// grid center.
fn needle(angle: f32) -> Contour {
    Contour::closed(vec![
        Point::new(12.0, 5.5),
        Point::new(14.5, 12.0),
        Point::new(12.0, 18.5),
        Point::new(9.5, 12.0),
    ])
    .map(|point| rotate_around(point, CENTER, angle))
}

/// The north half of the needle turned by `angle` radians: the triangle
/// of its tip and its two side corners.
fn needle_north(angle: f32) -> Contour {
    Contour::closed(vec![
        Point::new(12.0, 5.5),
        Point::new(14.5, 12.0),
        Point::new(9.5, 12.0),
    ])
    .map(|point| rotate_around(point, CENTER, angle))
}

/// The needle angle, in radians, of a damped swing at `t`: it starts and
/// ends at 0 and stays within 50 degrees, fading as `t` approaches 1.
fn swing(t: f32) -> f32 {
    (1.0 - t) * (1.0 - t) * (t * 3.5 * PI).sin() * 50.0_f32.to_radians()
}

/// The outline of a stroke `width` wide along the open polyline `line` of
/// one or two segments, with round caps and a round join, as one simple
/// polygon; empty for other polylines.
fn stroke_outline(line: &Contour, width: f32) -> Contour {
    let half = width / 2.0;

    match line.points.as_slice() {
        [a, b] => capsule(*a, *b, half),
        [a, joint, b] => bent(*a, *joint, *b, half),
        _ => Contour::closed(Vec::new()),
    }
}

/// The 2D cross product of `a` and `b`.
fn cross(a: Vector, b: Vector) -> f32 {
    a.x * b.y - a.y * b.x
}

/// The unit vector of `vector`, and its length.
fn unit(vector: Vector) -> (Vector, f32) {
    let length = vector.x.hypot(vector.y);

    if length > 0.0 {
        (vector * (1.0 / length), length)
    } else {
        (Vector::ZERO, 0.0)
    }
}

/// The normal of the unit vector `vector`, turned a quarter.
fn normal(vector: Vector) -> Vector {
    Vector::new(-vector.y, vector.x)
}

/// The outline of a stroke from `a` to `b` with round caps, `half` on each
/// side of the line.
fn capsule(a: Point, b: Point, half: f32) -> Contour {
    let (direction, _) = unit(b - a);
    let normal = normal(direction);
    let angle = normal.y.atan2(normal.x);
    let mut points = arc(b, half, angle, -PI);

    points.extend(arc(a, half, angle + PI, -PI));

    Contour::closed(points)
}

/// The outline of a stroke from `a` through `joint` to `b` with round caps
/// and a round outer join, `half` on each side of the line. Where the
/// second segment is too short to reach the inner corner, its end cap
/// joins the inner side of the first segment.
fn bent(a: Point, joint: Point, b: Point, half: f32) -> Contour {
    let (u, first) = unit(joint - a);
    let (v, second) = unit(b - joint);

    if second < 1e-4 {
        return capsule(a, joint, half);
    }
    if first < 1e-4 {
        return capsule(joint, b, half);
    }

    let turn = cross(u, v);

    if turn.abs() < 1e-6 {
        return capsule(a, b, half);
    }

    // The outline turning the other way is the mirror image of this one.
    if turn < 0.0 {
        let mirror = |point: Point| Point::new(point.x, -point.y);

        return bent(mirror(a), mirror(joint), mirror(b), half).map(mirror);
    }

    let (nu, nv) = (normal(u), normal(v));
    let angle_u = nu.y.atan2(nu.x);
    let angle_v = nv.y.atan2(nv.x);
    let bend = turn.atan2(u.x * v.x + u.y * v.y);
    let start = a + nu * half;
    let mut points = vec![start];

    // The inner corner where the inner sides of both segments meet.
    let along_second = cross((joint + nv * half) - start, u) / cross(u, v);

    if along_second <= second {
        let along_first = cross((joint + nv * half) - start, v) / cross(u, v);

        points.push(start + u * along_first);
        points.extend(arc(b, half, angle_v, -PI));
    } else {
        // The end cap meets the inner side of the first segment: of the two
        // crossings, the first one along the cap.
        let offset = start - b;
        let projection = offset.x * u.x + offset.y * u.y;
        let discriminant =
            projection * projection - (offset.x * offset.x + offset.y * offset.y - half * half);
        let travel = [1.0, -1.0]
            .into_iter()
            .map(|sign| {
                let crossing = start + u * (-projection + sign * discriminant.max(0.0).sqrt());
                let at = (crossing.y - b.y).atan2(crossing.x - b.x);

                (angle_v - at).rem_euclid(2.0 * PI)
            })
            .filter(|travel| *travel <= PI)
            .fold(PI, f32::min);

        points.extend(arc(b, half, angle_v - travel, -(PI - travel)));
    }

    points.push(joint - nv * half);
    points.extend(arc(joint, half, angle_v + PI, -bend));
    points.push(a - nu * half);
    points.extend(arc(a, half, angle_u + PI, -PI));

    Contour::closed(points)
}

// The painters.

fn paint_ok(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let parts = [
        ring().partial(phase(t, 0.0, 0.6)),
        check().partial(phase(t, 0.45, 1.0)),
    ];

    for part in parts.into_iter().flatten() {
        canvas.stroke(&path(&[part]), color, STROKE);
    }
}

fn paint_ok_filled(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let pop = ease_out_back(phase(t, 0.0, 0.5));

    if pop <= 0.0 {
        return;
    }

    let mut contours = vec![ring().map(|point| scale_around(point, CENTER, pop))];

    if let Some(mark) = check().partial(phase(t, 0.4, 1.0)) {
        contours.push(stroke_outline(&mark, STROKE));
    }

    canvas.fill_even_odd(&path(&contours), color);
}

fn paint_heart(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let beat = 1.0 + 0.15 * (PI * phase(t, 0.7, 1.0)).sin();

    if let Some(outline) = heart().partial(phase(t, 0.0, 0.7)) {
        let outline = outline.map(|point| scale_around(point, CENTER, beat));

        canvas.stroke(&path(&[outline]), color, STROKE * beat);
    }
}

fn paint_heart_filled(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let pop = ease_out_back(t);

    if pop > 0.0 {
        canvas.fill(
            &path(&[heart().map(|point| scale_around(point, CENTER, pop))]),
            color,
        );
    }
}

fn paint_star(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    if let Some(outline) = star().partial(t) {
        canvas.stroke(&path(&[outline]), color, STROKE);
    }
}

fn paint_star_filled(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let pop = ease_out_back(t);
    let turn = (1.0 - ease_out(t)) * -2.0 * PI / 5.0;

    if pop > 0.0 {
        let shape =
            star().map(|point| rotate_around(scale_around(point, CENTER, pop), STAR_CENTER, turn));

        canvas.fill(&path(&[shape]), color);
    }
}

fn paint_star_half_filled(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let outline = star();
    let edge = 12.0 * ease_out(t);

    canvas.stroke(&path(std::slice::from_ref(&outline)), color, STROKE);

    if edge > 0.0 {
        canvas.fill(&path(&[outline.clip_left(edge)]), color);
    }
}

fn paint_compass(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let angle = swing(t);

    canvas.stroke(&path(&[ring()]), color, STROKE);
    canvas.stroke(&path(&[needle(angle)]), color, STROKE);
    canvas.fill(&path(&[needle_north(angle)]), color);
}

fn paint_compass_filled(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    canvas.fill_even_odd(&path(&[ring(), needle(swing(t))]), color);
}

fn paint_no_network(canvas: &mut Canvas<'_>, t: f32, color: Color) {
    let base = Point::new(12.0, 19.0);
    let faded = |amount: f32| Color {
        a: color.a * amount,
        ..color
    };
    let dot = phase(t, 0.0, 0.15);

    if dot > 0.0 {
        canvas.fill(&path(&[circle(base, 1.2)]), faded(dot));
    }

    for i in 0..3 {
        let step = i as f32 * 0.15;
        let appear = phase(t, 0.1 + step, 0.25 + step);

        if appear > 0.0 {
            let wave = Contour::open(arc(base, 4.5 + i as f32 * 4.5, -PI * 3.0 / 4.0, PI / 2.0));

            canvas.stroke(&path(&[wave]), faded(appear), STROKE);
        }
    }

    let slash = Contour::open(vec![Point::new(3.0, 3.0), Point::new(21.0, 21.0)]);

    if let Some(slash) = slash.partial(phase(t, 0.65, 1.0)) {
        canvas.stroke(&path(&[slash]), color, STROKE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Whether `point` is inside the polygon `contour`, by the even-odd
    /// rule.
    fn inside(contour: &Contour, point: Point) -> bool {
        contour
            .segments()
            .filter(|(a, b)| {
                (a.y > point.y) != (b.y > point.y)
                    && point.x < a.x + (point.y - a.y) / (b.y - a.y) * (b.x - a.x)
            })
            .count()
            % 2
            == 1
    }

    /// The distance from `point` to the open polyline `line`.
    fn distance(line: &Contour, point: Point) -> f32 {
        line.points
            .windows(2)
            .map(|pair| {
                let (a, b) = (pair[0], pair[1]);
                let ab = b - a;
                let ap = point - a;
                let t = ((ap.x * ab.x + ap.y * ab.y) / (ab.x * ab.x + ab.y * ab.y)).clamp(0.0, 1.0);

                point.distance(a + ab * t)
            })
            .fold(f32::INFINITY, f32::min)
    }

    #[test]
    fn a_partial_contour_follows_the_length() {
        let line = Contour::open(vec![Point::new(0.0, 0.0), Point::new(10.0, 0.0)]);

        assert_eq!(line.partial(0.0), None);
        assert_eq!(
            line.partial(0.25).map(|part| part.points),
            Some(vec![Point::new(0.0, 0.0), Point::new(2.5, 0.0)])
        );
        assert_eq!(line.partial(1.0), Some(line.clone()));

        let square = Contour::closed(vec![
            Point::new(0.0, 0.0),
            Point::new(1.0, 0.0),
            Point::new(1.0, 1.0),
            Point::new(0.0, 1.0),
        ]);
        let three_quarters = square.partial(0.875).map(|part| part.points);

        assert_eq!(
            three_quarters.as_ref().and_then(|points| points.last()),
            Some(&Point::new(0.0, 0.5)),
            "the closing segment counts"
        );
    }

    #[test]
    fn the_stroke_outline_covers_the_stroke_of_every_partial_check() {
        for step in 1..=40 {
            let Some(line) = check().partial(step as f32 / 40.0) else {
                continue;
            };
            let outline = stroke_outline(&line, STROKE);

            for x in 0..=80 {
                for y in 0..=80 {
                    let point = Point::new(4.0 + x as f32 * 0.2, 4.0 + y as f32 * 0.2);
                    let away = distance(&line, point);

                    if away < 0.95 {
                        assert!(inside(&outline, point), "{point:?} at step {step}");
                    } else if away > 1.05 {
                        assert!(!inside(&outline, point), "{point:?} at step {step}");
                    }
                }
            }
        }
    }

    #[test]
    fn clipping_keeps_the_left_part() {
        let star = star();
        let half = star.clip_left(12.0);

        assert!(half.points.iter().all(|point| point.x <= 12.0 + 1e-4));
        assert!(inside(&half, Point::new(10.0, 13.0)));
        assert!(!inside(&half, Point::new(14.0, 13.0)));
    }

    #[test]
    fn the_north_half_is_inside_the_needle() {
        for angle in [0.0, 0.4, -0.7] {
            let north = needle_north(angle);
            let whole = needle(angle);
            let tip = rotate_around(Point::new(12.0, 7.0), CENTER, angle);
            let tail = rotate_around(Point::new(12.0, 17.0), CENTER, angle);

            assert!(inside(&north, tip) && inside(&whole, tip));
            assert!(!inside(&north, tail) && inside(&whole, tail));
        }
    }

    #[test]
    fn the_swing_settles_north() {
        assert_eq!(swing(0.0), 0.0);
        assert!(swing(1.0).abs() < 1e-6);
        assert!((0..=100).all(|step| swing(step as f32 / 100.0).abs() <= 50.0_f32.to_radians()));
        assert!(swing(0.1) > 0.3, "it swings clockwise first");
    }

    #[test]
    fn the_heart_and_star_are_closed() {
        let heart = heart();

        assert!(heart.closed && heart.points.first() == Some(&Point::new(12.0, 20.5)));
        assert_eq!(star().points.len(), 10);
        assert!((star().points[0].y - 2.8).abs() < 1e-4);
    }

    #[test]
    fn data_compare_by_painter_and_duration() {
        assert_eq!(OK, OK);
        assert_ne!(OK, OK_FILLED);
        assert_ne!(OK, OK.with_duration(Duration::from_secs(1)));
        assert_eq!(COMPASS.duration(), Duration::from_millis(1200));
        assert_eq!(NO_NETWORK.duration(), Duration::from_millis(900));
        assert_eq!(HEART.duration(), DURATION);
    }
}
