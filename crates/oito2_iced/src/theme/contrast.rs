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

//! WCAG contrast: ratios between colors and the lightness search that raises
//! a color to a target ratio against a set of backgrounds.

use iced::Color;

/// Minimum ratio for normal text (WCAG AA).
pub const TEXT_AA: f32 = 4.5;
/// Minimum ratio for non-text elements (WCAG AA).
pub const NON_TEXT_AA: f32 = 3.0;
/// Minimum ratio for normal text (WCAG AAA).
pub const TEXT_AAA: f32 = 7.0;

/// Relative luminance of a color, ignoring its alpha. Channels at or below
/// 0.03928 are linearized by division, as in the WCAG 2.0 definition.
pub fn luminance(color: Color) -> f32 {
    luminance_f64(color) as f32
}

fn luminance_f64(color: Color) -> f64 {
    fn linear(channel: f32) -> f64 {
        let channel = f64::from(channel);

        if channel <= 0.03928 {
            channel / 12.92
        } else {
            ((channel + 0.055) / 1.055).powf(2.4)
        }
    }

    0.2126 * linear(color.r) + 0.7152 * linear(color.g) + 0.0722 * linear(color.b)
}

/// Contrast ratio between two colors, from 1 to 21.
pub fn ratio(a: Color, b: Color) -> f32 {
    ratio_f64(a, b) as f32
}

fn ratio_f64(a: Color, b: Color) -> f64 {
    let la = luminance_f64(a);
    let lb = luminance_f64(b);
    let (lighter, darker) = if la > lb { (la, lb) } else { (lb, la) };

    (lighter + 0.05) / (darker + 0.05)
}

/// Black or white, whichever contrasts more with `background`.
pub fn foreground_for(background: Color) -> Color {
    if ratio(Color::BLACK, background) >= ratio(Color::WHITE, background) {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

/// The lowest contrast ratio of `color` against `backgrounds` (21 when empty).
pub fn min_ratio(color: Color, backgrounds: &[Color]) -> f32 {
    min_ratio_f64(color, backgrounds) as f32
}

fn min_ratio_f64(color: Color, backgrounds: &[Color]) -> f64 {
    backgrounds
        .iter()
        .map(|background| ratio_f64(color, *background))
        .fold(21.0, f64::min)
}

/// Returns `color` when it reaches `target` against every background;
/// otherwise the closest color in HSL lightness (steps of 0.5 %, darker
/// first, each candidate rounded to 8 bits per channel) that reaches it, or
/// black or white as a last resort.
pub fn ensure(color: Color, backgrounds: &[Color], target: f32) -> Color {
    let target = f64::from(target);

    if min_ratio_f64(color, backgrounds) >= target {
        return color;
    }

    let (hue, saturation, lightness) = to_hsl(color);

    for step in 1..=200 {
        for sign in [-1.0, 1.0] {
            let candidate_lightness = lightness + sign * f64::from(step) * 0.005;

            if !(0.0..=1.0).contains(&candidate_lightness) {
                continue;
            }

            let candidate = Color {
                a: color.a,
                ..from_hsl(hue, saturation, candidate_lightness)
            };

            if min_ratio_f64(candidate, backgrounds) >= target {
                return candidate;
            }
        }
    }

    if min_ratio_f64(Color::BLACK, backgrounds) >= min_ratio_f64(Color::WHITE, backgrounds) {
        Color::BLACK
    } else {
        Color::WHITE
    }
}

/// Hue (degrees), saturation and lightness (0 to 1) of a color, computed
/// from its 8-bit channels.
fn to_hsl(color: Color) -> (f64, f64, f64) {
    let [r, g, b, _] = color.into_rgba8();
    let (r, g, b) = (
        f64::from(r) / 255.0,
        f64::from(g) / 255.0,
        f64::from(b) / 255.0,
    );

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let lightness = (max + min) / 2.0;

    let hue = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let saturation = if delta == 0.0 {
        0.0
    } else {
        (delta / (1.0 - (2.0 * lightness - 1.0).abs())).clamp(0.0, 1.0)
    };

    (hue, saturation, lightness)
}

/// Color from hue (degrees), saturation and lightness (0 to 1), rounded to
/// 8 bits per channel.
fn from_hsl(hue: f64, saturation: f64, lightness: f64) -> Color {
    let chroma = (1.0 - (2.0 * lightness - 1.0).abs()) * saturation;
    let secondary = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let offset = lightness - chroma / 2.0;

    let (r, g, b) = match hue {
        h if h < 60.0 => (chroma, secondary, 0.0),
        h if h < 120.0 => (secondary, chroma, 0.0),
        h if h < 180.0 => (0.0, chroma, secondary),
        h if h < 240.0 => (0.0, secondary, chroma),
        h if h < 300.0 => (secondary, 0.0, chroma),
        _ => (chroma, 0.0, secondary),
    };

    let channel = |value: f64| ((value + offset) * 255.0).round().clamp(0.0, 255.0) as u8;

    Color::from_rgb8(channel(r), channel(g), channel(b))
}
