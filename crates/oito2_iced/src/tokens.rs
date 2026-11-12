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

//! Design tokens: radii, spacing, sizes, accent colors and the neutral and
//! status palettes. The values equal `tokens.json` of the `oito2_ui` parity
//! fixtures.

use iced::Color;

/// Corner radii, in logical pixels.
pub mod radius {
    /// Radius of check boxes.
    pub const CHECK: f32 = 4.0;
    /// Small radius.
    pub const SMALL: f32 = 6.0;
    /// Medium radius: buttons and text inputs.
    pub const MEDIUM: f32 = 8.0;
    /// Large radius: cards, popovers and dialogs.
    pub const LARGE: f32 = 12.0;
    /// Radius of the window corners.
    pub const WINDOW: f32 = 14.0;
}

/// Spacing steps, in logical pixels.
pub mod spacing {
    /// 2 px.
    pub const XXS: f32 = 2.0;
    /// 4 px.
    pub const XS: f32 = 4.0;
    /// 8 px.
    pub const SM: f32 = 8.0;
    /// 12 px.
    pub const MD: f32 = 12.0;
    /// 16 px.
    pub const LG: f32 = 16.0;
    /// 20 px: page padding.
    pub const PAGE: f32 = 20.0;
    /// 24 px.
    pub const XL: f32 = 24.0;
    /// 32 px.
    pub const XXL: f32 = 32.0;
    /// 48 px.
    pub const XXXL: f32 = 48.0;
}

/// Component sizes, in logical pixels.
pub mod sizes {
    /// Minimum height of a control with a pointer.
    pub const BUTTON_HEIGHT: f32 = 34.0;
    /// Minimum height of a control on a touch screen.
    pub const TOUCH_BUTTON_HEIGHT: f32 = 40.0;
    /// Height of the window title bar.
    pub const TITLE_BAR_HEIGHT: f32 = 46.0;
    /// Height of the items inside the title bar.
    pub const TITLE_BAR_ITEM_HEIGHT: f32 = 34.0;
    /// Size of icons.
    pub const ICON_SIZE: f32 = 20.0;
    /// Width of the focus border and of the focus ring.
    pub const FOCUS_BORDER_WIDTH: f32 = 2.0;
    /// Size of the window control buttons.
    pub const WINDOW_CONTROL_SIZE: f32 = 24.0;
}

/// Text sizes, in logical pixels.
pub mod typography {
    /// Body text, large.
    pub const BODY_LARGE_SIZE: f32 = 16.0;
    /// Body text, medium: the default size.
    pub const BODY_MEDIUM_SIZE: f32 = 14.0;
    /// Body text, small.
    pub const BODY_SMALL_SIZE: f32 = 12.0;
}

/// The accent colors a theme can use as its primary color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Accent {
    /// Blue, the fallback accent.
    #[default]
    Blue,
    /// Teal.
    Teal,
    /// Green.
    Green,
    /// Yellow.
    Yellow,
    /// Orange.
    Orange,
    /// Red.
    Red,
    /// Pink.
    Pink,
    /// Purple.
    Purple,
    /// Slate.
    Slate,
}

impl Accent {
    /// Every accent, in the order of the design tokens.
    pub const ALL: [Accent; 9] = [
        Accent::Blue,
        Accent::Teal,
        Accent::Green,
        Accent::Yellow,
        Accent::Orange,
        Accent::Red,
        Accent::Pink,
        Accent::Purple,
        Accent::Slate,
    ];

    /// The raw accent color, before any contrast adjustment.
    pub const fn color(self) -> Color {
        match self {
            Accent::Blue => Color::from_rgb8(0x35, 0x84, 0xE4),
            Accent::Teal => Color::from_rgb8(0x21, 0x90, 0xA4),
            Accent::Green => Color::from_rgb8(0x3A, 0x94, 0x4A),
            Accent::Yellow => Color::from_rgb8(0xC8, 0x88, 0x00),
            Accent::Orange => Color::from_rgb8(0xED, 0x5B, 0x00),
            Accent::Red => Color::from_rgb8(0xE6, 0x2D, 0x42),
            Accent::Pink => Color::from_rgb8(0xD5, 0x61, 0x99),
            Accent::Purple => Color::from_rgb8(0x91, 0x41, 0xAC),
            Accent::Slate => Color::from_rgb8(0x6F, 0x83, 0x96),
        }
    }

    /// The lowercase name used by the design tokens.
    pub const fn name(self) -> &'static str {
        match self {
            Accent::Blue => "blue",
            Accent::Teal => "teal",
            Accent::Green => "green",
            Accent::Yellow => "yellow",
            Accent::Orange => "orange",
            Accent::Red => "red",
            Accent::Pink => "pink",
            Accent::Purple => "purple",
            Accent::Slate => "slate",
        }
    }
}

/// The neutral surfaces and foregrounds of one brightness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Neutrals {
    /// Surface of cards and content.
    pub surface: Color,
    /// Lowest surface container.
    pub surface_container_lowest: Color,
    /// Low surface container; also the window background.
    pub surface_container_low: Color,
    /// Surface container: sidebars and popovers.
    pub surface_container: Color,
    /// High surface container: text inputs.
    pub surface_container_high: Color,
    /// Highest surface container: main bars.
    pub surface_container_highest: Color,
    /// Background of the window title bar.
    pub title_bar: Color,
    /// Text and icons on surfaces.
    pub on_surface: Color,
    /// Secondary text and icons on surfaces.
    pub on_surface_variant: Color,
    /// Borders of controls.
    pub outline: Color,
    /// Dividers and decorative borders.
    pub outline_variant: Color,
}

impl Neutrals {
    /// The light neutrals.
    pub const LIGHT: Neutrals = Neutrals {
        surface: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        surface_container_lowest: Color::from_rgb8(0xFF, 0xFF, 0xFF),
        surface_container_low: Color::from_rgb8(0xFA, 0xFA, 0xFA),
        surface_container: Color::from_rgb8(0xF5, 0xF5, 0xF5),
        surface_container_high: Color::from_rgb8(0xEF, 0xEF, 0xEF),
        surface_container_highest: Color::from_rgb8(0xE8, 0xE8, 0xE8),
        title_bar: Color::from_rgb8(0xEB, 0xEB, 0xEB),
        on_surface: Color::from_rgb8(0x20, 0x20, 0x20),
        on_surface_variant: Color::from_rgb8(0x5E, 0x5E, 0x5E),
        outline: Color::from_rgb8(0x7E, 0x7E, 0x7E),
        outline_variant: Color::from_rgb8(0xD9, 0xD9, 0xD9),
    };

    /// The dark neutrals.
    pub const DARK: Neutrals = Neutrals {
        surface: Color::from_rgb8(0x20, 0x20, 0x20),
        surface_container_lowest: Color::from_rgb8(0x1A, 0x1A, 0x1A),
        surface_container_low: Color::from_rgb8(0x24, 0x24, 0x24),
        surface_container: Color::from_rgb8(0x2A, 0x2A, 0x2A),
        surface_container_high: Color::from_rgb8(0x30, 0x30, 0x30),
        surface_container_highest: Color::from_rgb8(0x38, 0x38, 0x38),
        title_bar: Color::from_rgb8(0x30, 0x30, 0x30),
        on_surface: Color::from_rgb8(0xF7, 0xF7, 0xF7),
        on_surface_variant: Color::from_rgb8(0xBD, 0xBD, 0xBD),
        outline: Color::from_rgb8(0x8A, 0x8A, 0x8A),
        outline_variant: Color::from_rgb8(0x3D, 0x3D, 0x3D),
    };
}

/// The status colors of one brightness.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Status {
    /// Success.
    pub success: Color,
    /// Warning.
    pub warning: Color,
    /// Error.
    pub error: Color,
    /// Information.
    pub info: Color,
}

impl Status {
    /// The light status colors.
    pub const LIGHT: Status = Status {
        success: Color::from_rgb8(0x18, 0x75, 0x49),
        warning: Color::from_rgb8(0x88, 0x60, 0x03),
        error: Color::from_rgb8(0xC0, 0x1C, 0x28),
        info: Color::from_rgb8(0x19, 0x66, 0xC2),
    };

    /// The dark status colors.
    pub const DARK: Status = Status {
        success: Color::from_rgb8(0x8F, 0xF0, 0xA4),
        warning: Color::from_rgb8(0xF8, 0xE4, 0x5C),
        error: Color::from_rgb8(0xFF, 0x7B, 0x63),
        info: Color::from_rgb8(0x78, 0xAE, 0xED),
    };
}
