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

//! The embedded JetBrains Mono font of code blocks.
//!
//! JetBrains Mono is embedded as four static instances: Regular and Bold,
//! upright and italic, all in the family `OITO2 JetBrains Mono`, so a
//! JetBrains Mono installed on the system never takes the place of the
//! embedded faces. Register every entry of [`JETBRAINS_MONO_FONTS`] with
//! the application (`iced::application(..).font(..)` once per entry);
//! [`JETBRAINS_MONO`] is the regular face, and [`jetbrains_mono`] and
//! [`jetbrains_mono_italic`] select the others. Weights other than 400 and
//! 700 resolve to the nearest embedded instance.
//!
//! ```no_run
//! use oito2_iced_syntax::font;
//!
//! let settings = iced::Settings {
//!     fonts: font::JETBRAINS_MONO_FONTS.into_iter().map(Into::into).collect(),
//!     ..iced::Settings::default()
//! };
//! # let _ = settings;
//! ```

use iced::Font;
use iced::font::{Style, Weight};

macro_rules! jetbrains_mono_file {
    ($file:literal) => {
        include_bytes!(concat!("../assets/fonts/", $file))
    };
}

/// The bytes of `JetBrainsMono-Regular.ttf` (weight 400, SIL Open Font
/// License 1.1).
pub const JETBRAINS_MONO_REGULAR: &[u8] = jetbrains_mono_file!("JetBrainsMono-Regular.ttf");

/// The bytes of `JetBrainsMono-Bold.ttf` (weight 700, SIL Open Font License
/// 1.1).
pub const JETBRAINS_MONO_BOLD: &[u8] = jetbrains_mono_file!("JetBrainsMono-Bold.ttf");

/// The bytes of `JetBrainsMono-Italic.ttf` (weight 400, SIL Open Font
/// License 1.1).
pub const JETBRAINS_MONO_ITALIC: &[u8] = jetbrains_mono_file!("JetBrainsMono-Italic.ttf");

/// The bytes of `JetBrainsMono-BoldItalic.ttf` (weight 700, SIL Open Font
/// License 1.1).
pub const JETBRAINS_MONO_BOLD_ITALIC: &[u8] = jetbrains_mono_file!("JetBrainsMono-BoldItalic.ttf");

/// Every embedded JetBrains Mono instance, to register with the
/// application.
pub const JETBRAINS_MONO_FONTS: [&[u8]; 4] = [
    JETBRAINS_MONO_REGULAR,
    JETBRAINS_MONO_BOLD,
    JETBRAINS_MONO_ITALIC,
    JETBRAINS_MONO_BOLD_ITALIC,
];

/// The family name of the embedded JetBrains Mono instances.
pub const JETBRAINS_MONO_FAMILY: &str = "OITO2 JetBrains Mono";

/// The license of the embedded JetBrains Mono font, the SIL Open Font
/// License 1.1, for an application's license page.
pub const JETBRAINS_MONO_LICENSE: &str = include_str!("../assets/fonts/OFL.txt");

/// JetBrains Mono at regular weight.
pub const JETBRAINS_MONO: Font = Font::with_name(JETBRAINS_MONO_FAMILY);

/// JetBrains Mono at the given weight.
pub const fn jetbrains_mono(weight: Weight) -> Font {
    Font {
        weight,
        ..JETBRAINS_MONO
    }
}

/// JetBrains Mono italic at the given weight.
pub const fn jetbrains_mono_italic(weight: Weight) -> Font {
    Font {
        weight,
        style: Style::Italic,
        ..JETBRAINS_MONO
    }
}
