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

//! Material motion: easing curves and durations for
//! [`iced::animation::Animation`].

use std::time::Duration;

pub use iced::animation::{Animation, Easing};

/// Standard durations.
pub mod duration {
    use std::time::Duration;

    /// 100 ms: small state changes (hover, press).
    pub const SHORT: Duration = Duration::from_millis(100);
    /// 200 ms: switches, checks and small panels.
    pub const MEDIUM: Duration = Duration::from_millis(200);
    /// 300 ms: popovers and expanding content.
    pub const LONG: Duration = Duration::from_millis(300);
    /// 500 ms: large transitions.
    pub const EXTRA_LONG: Duration = Duration::from_millis(500);
}

/// The Material "emphasized" curve: a fast start, then a long, gentle
/// deceleration.
pub const EMPHASIZED: Easing = Easing::Custom(emphasized);

/// Cubic ease-out.
pub const EASE_OUT_CUBIC: Easing = Easing::Custom(ease_out_cubic);

/// Cubic ease-in-out.
pub const EASE_IN_OUT_CUBIC: Easing = Easing::Custom(ease_in_out_cubic);

/// Returns `duration`, or zero when the user asked for reduced motion, so
/// the animation jumps to its end.
pub fn motion_duration(duration: Duration, reduce_motion: bool) -> Duration {
    if reduce_motion {
        Duration::ZERO
    } else {
        duration
    }
}

/// An [`Animation`] with the emphasized curve and the given duration.
pub fn emphasized_animation<T>(state: T, duration: Duration) -> Animation<T>
where
    T: Clone + Copy + PartialEq + iced::animation::Float,
{
    Animation::new(state).easing(EMPHASIZED).duration(duration)
}

/// The emphasized curve at progress `t` (0 to 1): two cubic segments joined
/// at (0.166666, 0.4).
pub fn emphasized(t: f32) -> f32 {
    const A1: (f32, f32) = (0.05, 0.0);
    const B1: (f32, f32) = (0.133333, 0.06);
    const MID: (f32, f32) = (0.166666, 0.4);
    const A2: (f32, f32) = (0.208333, 0.82);
    const B2: (f32, f32) = (0.25, 1.0);

    if t < MID.0 {
        let (sx, sy) = (MID.0, MID.1);

        cubic(A1.0 / sx, A1.1 / sy, B1.0 / sx, B1.1 / sy, t / sx) * sy
    } else {
        let (sx, sy) = (1.0 - MID.0, 1.0 - MID.1);

        cubic(
            (A2.0 - MID.0) / sx,
            (A2.1 - MID.1) / sy,
            (B2.0 - MID.0) / sx,
            (B2.1 - MID.1) / sy,
            (t - MID.0) / sx,
        ) * sy
            + MID.1
    }
}

/// Cubic ease-out: control points (0.215, 0.61) and (0.355, 1).
pub fn ease_out_cubic(t: f32) -> f32 {
    cubic(0.215, 0.61, 0.355, 1.0, t)
}

/// Cubic ease-in-out: control points (0.645, 0.045) and (0.355, 1).
pub fn ease_in_out_cubic(t: f32) -> f32 {
    cubic(0.645, 0.045, 0.355, 1.0, t)
}

/// A unit cubic Bézier from (0, 0) to (1, 1) with control points (a, b) and
/// (c, d), evaluated at x = `t` by bisection with an error below 0.001.
fn cubic(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    fn evaluate(p1: f32, p2: f32, m: f32) -> f32 {
        3.0 * p1 * (1.0 - m) * (1.0 - m) * m + 3.0 * p2 * (1.0 - m) * m * m + m * m * m
    }

    if t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }

    let (mut start, mut end) = (0.0_f32, 1.0_f32);

    for _ in 0..64 {
        let midpoint = (start + end) / 2.0;
        let estimate = evaluate(a, c, midpoint);

        if (t - estimate).abs() < 0.001 {
            return evaluate(b, d, midpoint);
        }

        if estimate < t {
            start = midpoint;
        } else {
            end = midpoint;
        }
    }

    evaluate(b, d, (start + end) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn curves_start_at_zero_and_end_at_one() {
        for curve in [emphasized, ease_out_cubic, ease_in_out_cubic] {
            assert_eq!(curve(0.0), 0.0);
            assert_eq!(curve(1.0), 1.0);
        }
    }

    #[test]
    fn emphasized_passes_through_its_midpoint() {
        assert!((emphasized(0.166666) - 0.4).abs() < 0.002);
    }

    #[test]
    fn emphasized_is_monotonic() {
        let mut previous = 0.0;

        for step in 1..=100 {
            let value = emphasized(step as f32 / 100.0);
            assert!(value + 0.002 >= previous, "decreases at step {step}");
            previous = value;
        }
    }
}
