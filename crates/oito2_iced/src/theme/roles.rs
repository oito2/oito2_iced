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

//! The resolved color roles of a theme: the Material 3 roles, the page
//! background, the window chrome, the semantic text colors and the syntax
//! colors.

use iced::Color;
use material_colors::color::Rgb;
use material_colors::hct::Hct;
use material_colors::scheme::Scheme;
use material_colors::scheme::variant::SchemeTonalSpot;

use super::contrast::{self, NON_TEXT_AA, TEXT_AA, TEXT_AAA};
use super::{Brightness, Contrast};
use crate::tokens::{Neutrals, Status};

/// Every color role a theme resolves.
///
/// Text roles reach WCAG AA against every surface of the theme:
/// `on_surface`, `on_surface_variant` and every [`SemanticColors`] and
/// [`SyntaxColors`] color reach 4.5:1, and `outline` reaches 3:1. With
/// [`Contrast::High`] the targets rise to 7:1 and 4.5:1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ColorRoles {
    /// Light or dark.
    pub brightness: Brightness,
    /// Whether the high-contrast targets apply.
    pub high_contrast: bool,
    /// Page background.
    pub background: Color,
    /// Primary color: filled buttons, selection, focus.
    pub primary: Color,
    /// Text and icons on `primary`.
    pub on_primary: Color,
    /// Primary container.
    pub primary_container: Color,
    /// Text and icons on `primary_container`.
    pub on_primary_container: Color,
    /// Primary tone that stays the same in light and dark themes.
    pub primary_fixed: Color,
    /// Dimmer variant of `primary_fixed`.
    pub primary_fixed_dim: Color,
    /// Text and icons on `primary_fixed`.
    pub on_primary_fixed: Color,
    /// Lower-emphasis text and icons on `primary_fixed`.
    pub on_primary_fixed_variant: Color,
    /// Secondary color.
    pub secondary: Color,
    /// Text and icons on `secondary`.
    pub on_secondary: Color,
    /// Secondary container: tonal buttons.
    pub secondary_container: Color,
    /// Text and icons on `secondary_container`.
    pub on_secondary_container: Color,
    /// Secondary tone that stays the same in light and dark themes.
    pub secondary_fixed: Color,
    /// Dimmer variant of `secondary_fixed`.
    pub secondary_fixed_dim: Color,
    /// Text and icons on `secondary_fixed`.
    pub on_secondary_fixed: Color,
    /// Lower-emphasis text and icons on `secondary_fixed`.
    pub on_secondary_fixed_variant: Color,
    /// Tertiary color.
    pub tertiary: Color,
    /// Text and icons on `tertiary`.
    pub on_tertiary: Color,
    /// Tertiary container.
    pub tertiary_container: Color,
    /// Text and icons on `tertiary_container`.
    pub on_tertiary_container: Color,
    /// Tertiary tone that stays the same in light and dark themes.
    pub tertiary_fixed: Color,
    /// Dimmer variant of `tertiary_fixed`.
    pub tertiary_fixed_dim: Color,
    /// Text and icons on `tertiary_fixed`.
    pub on_tertiary_fixed: Color,
    /// Lower-emphasis text and icons on `tertiary_fixed`.
    pub on_tertiary_fixed_variant: Color,
    /// Error color.
    pub error: Color,
    /// Text and icons on `error`.
    pub on_error: Color,
    /// Error container.
    pub error_container: Color,
    /// Text and icons on `error_container`.
    pub on_error_container: Color,
    /// Surface of cards and content.
    pub surface: Color,
    /// Text and icons on surfaces.
    pub on_surface: Color,
    /// The dimmest surface.
    pub surface_dim: Color,
    /// The brightest surface.
    pub surface_bright: Color,
    /// Lowest surface container.
    pub surface_container_lowest: Color,
    /// Low surface container.
    pub surface_container_low: Color,
    /// Surface container.
    pub surface_container: Color,
    /// High surface container.
    pub surface_container_high: Color,
    /// Highest surface container.
    pub surface_container_highest: Color,
    /// Secondary text and icons on surfaces.
    pub on_surface_variant: Color,
    /// Borders of controls.
    pub outline: Color,
    /// Dividers and decorative borders.
    pub outline_variant: Color,
    /// Shadows.
    pub shadow: Color,
    /// Scrim behind modal surfaces.
    pub scrim: Color,
    /// Inverse surface: toasts and tooltips.
    pub inverse_surface: Color,
    /// Text on `inverse_surface`.
    pub on_inverse_surface: Color,
    /// Primary color on `inverse_surface`.
    pub inverse_primary: Color,
    /// Tint laid over raised surfaces; transparent, so surfaces keep their
    /// color.
    pub surface_tint: Color,
    /// Window chrome colors.
    pub chrome: ChromeColors,
    /// Semantic text colors.
    pub semantic: SemanticColors,
    /// Syntax highlighting colors.
    pub syntax: SyntaxColors,
}

/// Background colors of the window chrome.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChromeColors {
    /// Page background.
    pub background: Color,
    /// A narrow sidebar column, such as an icon rail.
    pub sidebar: Color,
    /// Main bar: the pane beside a sidebar.
    pub mainbar: Color,
    /// Title bar.
    pub title_bar: Color,
    /// Dialogs, sheets and drawers.
    pub dialog: Color,
}

impl ChromeColors {
    /// The smallest contrast ratio between adjacent chrome surfaces: the
    /// page background, the sidebar and the main bar.
    pub const MINIMUM_STEP: f32 = 1.04;
}

/// Text colors that reach the contrast target on every surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemanticColors {
    /// Primary color used as text, indicators and focus rings.
    pub primary_text: Color,
    /// Error color used as text.
    pub error_text: Color,
    /// Success.
    pub success: Color,
    /// Warning.
    pub warning: Color,
    /// Information.
    pub info: Color,
    /// Links.
    pub link: Color,
}

/// Colors of the token roles of syntax-highlighted code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SyntaxColors {
    /// Plain code text.
    pub foreground: Color,
    /// Comments.
    pub comment: Color,
    /// Keywords and control flow.
    pub keyword: Color,
    /// String literals.
    pub string: Color,
    /// Numeric literals.
    pub number: Color,
    /// Function and method names.
    pub function: Color,
    /// Type and class names.
    pub r#type: Color,
    /// Markup tags.
    pub tag: Color,
    /// Markup attributes.
    pub attribute: Color,
    /// Constants and symbols.
    pub constant: Color,
    /// Invalid code.
    pub invalid: Color,
}

impl SyntaxColors {
    /// Syntax colors of the light base theme.
    pub const LIGHT: SyntaxColors = SyntaxColors {
        foreground: Color::from_rgb8(0x20, 0x20, 0x20),
        comment: Color::from_rgb8(0x77, 0x76, 0x7B),
        keyword: Color::from_rgb8(0x91, 0x41, 0xAC),
        string: Color::from_rgb8(0x26, 0xA2, 0x69),
        number: Color::from_rgb8(0xC6, 0x46, 0x00),
        function: Color::from_rgb8(0x1C, 0x71, 0xD8),
        r#type: Color::from_rgb8(0x21, 0x90, 0xA4),
        tag: Color::from_rgb8(0xC0, 0x1C, 0x28),
        attribute: Color::from_rgb8(0x98, 0x6A, 0x44),
        constant: Color::from_rgb8(0xD5, 0x61, 0x99),
        invalid: Color::from_rgb8(0xE0, 0x1B, 0x24),
    };

    /// Syntax colors of the dark base theme.
    pub const DARK: SyntaxColors = SyntaxColors {
        foreground: Color::from_rgb8(0xF7, 0xF7, 0xF7),
        comment: Color::from_rgb8(0x9A, 0x99, 0x96),
        keyword: Color::from_rgb8(0xDC, 0x8A, 0xDD),
        string: Color::from_rgb8(0x8F, 0xF0, 0xA4),
        number: Color::from_rgb8(0xFF, 0xA3, 0x48),
        function: Color::from_rgb8(0x99, 0xC1, 0xF1),
        r#type: Color::from_rgb8(0x93, 0xDD, 0xC2),
        tag: Color::from_rgb8(0xF6, 0x61, 0x51),
        attribute: Color::from_rgb8(0xF9, 0xF0, 0x6B),
        constant: Color::from_rgb8(0xF4, 0x9A, 0xC1),
        invalid: Color::from_rgb8(0xFF, 0x7B, 0x63),
    };

    /// Returns the palette with every color raised to `target` contrast
    /// against all `backgrounds`.
    pub fn ensure_contrast(&self, backgrounds: &[Color], target: f32) -> Self {
        let fix = |color| contrast::ensure(color, backgrounds, target);

        Self {
            foreground: fix(self.foreground),
            comment: fix(self.comment),
            keyword: fix(self.keyword),
            string: fix(self.string),
            number: fix(self.number),
            function: fix(self.function),
            r#type: fix(self.r#type),
            tag: fix(self.tag),
            attribute: fix(self.attribute),
            constant: fix(self.constant),
            invalid: fix(self.invalid),
        }
    }
}

/// The raw colors of a theme family, before the contrast adjustments.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Palette {
    /// Light or dark.
    pub brightness: Brightness,
    /// Primary fill.
    pub primary: Color,
    /// Text and icons on `primary`.
    pub on_primary: Color,
    /// Error fill.
    pub error: Color,
    /// Page background.
    pub background: Color,
    /// Surface of cards and content.
    pub surface: Color,
    /// Lowest surface container.
    pub surface_container_lowest: Color,
    /// Low surface container.
    pub surface_container_low: Color,
    /// Surface container.
    pub surface_container: Color,
    /// High surface container.
    pub surface_container_high: Color,
    /// Highest surface container.
    pub surface_container_highest: Color,
    /// Title bar.
    pub title_bar: Color,
    /// Text and icons on surfaces.
    pub on_surface: Color,
    /// Secondary text and icons on surfaces.
    pub on_surface_variant: Color,
    /// Borders of controls.
    pub outline: Color,
    /// Dividers and decorative borders.
    pub outline_variant: Color,
    /// Success.
    pub success: Color,
    /// Warning.
    pub warning: Color,
    /// Information.
    pub info: Color,
    /// Syntax colors.
    pub syntax: SyntaxColors,
}

impl Palette {
    /// The palette of the base theme: the neutral and status colors of
    /// `brightness` with `accent` as the primary hue, darkened as far as
    /// needed for white text to reach the text target of `contrast`.
    pub fn base(brightness: Brightness, accent: Color, contrast: Contrast) -> Self {
        let is_dark = brightness == Brightness::Dark;
        let neutrals = if is_dark {
            Neutrals::DARK
        } else {
            Neutrals::LIGHT
        };
        let status = if is_dark { Status::DARK } else { Status::LIGHT };
        let high = contrast == Contrast::High;

        let on_surface = match (high, is_dark) {
            (true, true) => Color::WHITE,
            (true, false) => Color::BLACK,
            (false, _) => neutrals.on_surface,
        };

        Self {
            brightness,
            primary: contrast::ensure(accent, &[Color::WHITE], text_target(contrast)),
            on_primary: Color::WHITE,
            error: status.error,
            background: neutrals.surface_container_low,
            surface: neutrals.surface,
            surface_container_lowest: neutrals.surface_container_lowest,
            surface_container_low: neutrals.surface_container_low,
            surface_container: neutrals.surface_container,
            surface_container_high: neutrals.surface_container_high,
            surface_container_highest: neutrals.surface_container_highest,
            title_bar: neutrals.title_bar,
            on_surface,
            on_surface_variant: neutrals.on_surface_variant,
            outline: neutrals.outline,
            outline_variant: neutrals.outline_variant,
            success: status.success,
            warning: status.warning,
            info: status.info,
            syntax: if is_dark {
                SyntaxColors::DARK
            } else {
                SyntaxColors::LIGHT
            },
        }
    }
}

/// The few role colors a color theme family publishes, from which
/// [`Palette::from_family`] derives a complete palette.
///
/// The roles follow the vocabulary most theme families use: a page
/// background, a card surface, a secondary surface, a border, the text
/// colors and the accents. Values are taken as given; the contrast
/// adjustments happen when the palette is resolved.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FamilyColors {
    /// Light or dark.
    pub brightness: Brightness,
    /// The page background.
    pub background: Color,
    /// The main surface of cards, panes and fields.
    pub card: Color,
    /// The body text color.
    pub foreground: Color,
    /// The signature accent, used as the primary fill.
    pub primary: Color,
    /// A surface one step above `card`, such as a selected row.
    pub secondary: Color,
    /// Secondary text, such as captions and placeholders.
    pub muted_foreground: Color,
    /// Errors and destructive actions.
    pub destructive: Color,
    /// Borders and dividers.
    pub border: Color,
    /// Success states.
    pub success: Color,
    /// Warning states.
    pub warning: Color,
    /// Informational states.
    pub info: Color,
    /// Syntax highlighting colors.
    pub syntax: SyntaxColors,
}

impl Palette {
    /// The palette of a theme family.
    ///
    /// The primary fill keeps `primary` as given, with black or white text,
    /// whichever contrasts more. The surface ladder runs from the lower of
    /// the background and the card up to the secondary surface:
    ///
    /// - the lowest container is whichever of `background` and `card` sits
    ///   lower (darker in a dark theme, lighter in a light one);
    /// - the low container is halfway from it to `card`;
    /// - the container and the high container are one third and two thirds
    ///   of the way from `card` to `secondary`, and the highest container is
    ///   `secondary`;
    /// - the title bar is the container;
    /// - `outline` is `border` mixed 35 % toward `foreground`, and
    ///   `outline_variant` is `border`.
    pub fn from_family(family: &FamilyColors) -> Self {
        let is_dark = family.brightness == Brightness::Dark;
        let card = family.card;
        let secondary = family.secondary;

        let background_luminance = contrast::luminance(family.background);
        let card_luminance = contrast::luminance(card);
        let background_is_lower = if is_dark {
            background_luminance < card_luminance
        } else {
            background_luminance > card_luminance
        };
        let lowest = if background_is_lower {
            family.background
        } else {
            card
        };
        let container = lerp(card, secondary, 1.0 / 3.0);

        Self {
            brightness: family.brightness,
            primary: family.primary,
            on_primary: contrast::foreground_for(family.primary),
            error: family.destructive,
            background: family.background,
            surface: card,
            surface_container_lowest: lowest,
            surface_container_low: lerp(lowest, card, 0.5),
            surface_container: container,
            surface_container_high: lerp(card, secondary, 2.0 / 3.0),
            surface_container_highest: secondary,
            title_bar: container,
            on_surface: family.foreground,
            on_surface_variant: family.muted_foreground,
            outline: lerp(family.border, family.foreground, 0.35),
            outline_variant: family.border,
            success: family.success,
            warning: family.warning,
            info: family.info,
            syntax: family.syntax,
        }
    }
}

/// The color `t` of the way from `a` to `b`, channel by channel.
fn lerp(a: Color, b: Color, t: f32) -> Color {
    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}

impl ColorRoles {
    /// Resolves the roles of the base theme: GTK-style neutrals with
    /// `accent` as the primary hue.
    pub fn base(brightness: Brightness, accent: Color, contrast: Contrast) -> Self {
        Self::resolve(&Palette::base(brightness, accent, contrast), contrast)
    }

    /// Resolves the roles of a palette: derives the chrome colors, raises
    /// the text, outline, semantic and syntax colors to the contrast
    /// targets against every surface, and fills the remaining Material
    /// roles from a tonal-spot scheme seeded by the primary color.
    pub fn resolve(palette: &Palette, contrast: Contrast) -> Self {
        let is_dark = palette.brightness == Brightness::Dark;
        let high_contrast = contrast == Contrast::High;
        let text_target = text_target(contrast);
        let non_text_target = if high_contrast { TEXT_AA } else { NON_TEXT_AA };

        let seed = tonal_spot(palette.primary, is_dark);

        let mainbar = contrast::ensure(
            palette.surface_container_highest,
            &[palette.background],
            ChromeColors::MINIMUM_STEP,
        );
        let chrome = ChromeColors {
            background: palette.background,
            sidebar: contrast::ensure(
                palette.surface_container,
                &[palette.background, mainbar],
                ChromeColors::MINIMUM_STEP,
            ),
            mainbar,
            title_bar: palette.title_bar,
            dialog: if is_dark {
                palette.surface_container_high
            } else {
                palette.surface
            },
        };

        let surfaces = [
            palette.background,
            palette.surface,
            palette.surface_container_lowest,
            palette.surface_container_low,
            palette.surface_container,
            palette.surface_container_high,
            palette.surface_container_highest,
            chrome.title_bar,
            chrome.sidebar,
            chrome.mainbar,
            chrome.dialog,
        ];
        let text = |color| contrast::ensure(color, &surfaces, text_target);

        let on_surface = text(palette.on_surface);
        let outline = contrast::ensure(palette.outline, &surfaces, non_text_target);
        let primary_text = text(palette.primary);

        Self {
            brightness: palette.brightness,
            high_contrast,
            background: palette.background,
            primary: palette.primary,
            on_primary: palette.on_primary,
            primary_container: color(seed.primary_container),
            on_primary_container: color(seed.on_primary_container),
            primary_fixed: color(seed.primary_fixed),
            primary_fixed_dim: color(seed.primary_fixed_dim),
            on_primary_fixed: color(seed.on_primary_fixed),
            on_primary_fixed_variant: color(seed.on_primary_fixed_variant),
            secondary: color(seed.secondary),
            on_secondary: color(seed.on_secondary),
            secondary_container: color(seed.secondary_container),
            on_secondary_container: color(seed.on_secondary_container),
            secondary_fixed: color(seed.secondary_fixed),
            secondary_fixed_dim: color(seed.secondary_fixed_dim),
            on_secondary_fixed: color(seed.on_secondary_fixed),
            on_secondary_fixed_variant: color(seed.on_secondary_fixed_variant),
            tertiary: color(seed.tertiary),
            on_tertiary: color(seed.on_tertiary),
            tertiary_container: color(seed.tertiary_container),
            on_tertiary_container: color(seed.on_tertiary_container),
            tertiary_fixed: color(seed.tertiary_fixed),
            tertiary_fixed_dim: color(seed.tertiary_fixed_dim),
            on_tertiary_fixed: color(seed.on_tertiary_fixed),
            on_tertiary_fixed_variant: color(seed.on_tertiary_fixed_variant),
            error: palette.error,
            on_error: contrast::foreground_for(palette.error),
            error_container: color(seed.error_container),
            on_error_container: color(seed.on_error_container),
            surface: palette.surface,
            on_surface,
            surface_dim: if is_dark {
                palette.surface_container_lowest
            } else {
                palette.surface_container_highest
            },
            surface_bright: if is_dark {
                palette.surface_container_highest
            } else {
                palette.surface_container_lowest
            },
            surface_container_lowest: palette.surface_container_lowest,
            surface_container_low: palette.surface_container_low,
            surface_container: palette.surface_container,
            surface_container_high: palette.surface_container_high,
            surface_container_highest: palette.surface_container_highest,
            on_surface_variant: text(palette.on_surface_variant),
            outline,
            outline_variant: if high_contrast {
                outline
            } else {
                palette.outline_variant
            },
            shadow: Color::BLACK,
            scrim: Color::BLACK,
            inverse_surface: on_surface,
            on_inverse_surface: palette.surface,
            inverse_primary: contrast::ensure(palette.primary, &[on_surface], TEXT_AA),
            surface_tint: Color::TRANSPARENT,
            chrome,
            semantic: SemanticColors {
                primary_text,
                error_text: text(palette.error),
                success: text(palette.success),
                warning: text(palette.warning),
                info: text(palette.info),
                link: primary_text,
            },
            syntax: palette.syntax.ensure_contrast(&surfaces, text_target),
        }
    }

    /// Every surface of the theme: the page background, the chrome
    /// surfaces and the Material surface roles.
    pub fn surfaces(&self) -> [Color; 13] {
        [
            self.background,
            self.surface,
            self.surface_dim,
            self.surface_bright,
            self.surface_container_lowest,
            self.surface_container_low,
            self.surface_container,
            self.surface_container_high,
            self.surface_container_highest,
            self.chrome.title_bar,
            self.chrome.sidebar,
            self.chrome.mainbar,
            self.chrome.dialog,
        ]
    }

    /// The color of selected toggles, indicators and focus rings: the
    /// text-safe primary tone.
    pub fn indicator(&self) -> Color {
        self.semantic.primary_text
    }

    /// The content color drawn on top of [`ColorRoles::indicator`].
    pub fn on_indicator(&self) -> Color {
        contrast::foreground_for(self.indicator())
    }

    /// The fill of selected controls and progress: `primary` when it stands
    /// out 3:1 from the surface, the page background and the surface
    /// container, otherwise the [`ColorRoles::indicator`].
    pub fn selected_fill(&self) -> Color {
        let backgrounds = [self.surface, self.background, self.surface_container];

        if contrast::min_ratio(self.primary, &backgrounds) >= NON_TEXT_AA {
            self.primary
        } else {
            self.indicator()
        }
    }
}

/// The text contrast target of a contrast level.
fn text_target(contrast: Contrast) -> f32 {
    match contrast {
        Contrast::Standard => TEXT_AA,
        Contrast::High => TEXT_AAA,
    }
}

/// The Material 3 tonal-spot scheme of a seed color, at standard contrast.
pub fn tonal_spot(seed: Color, is_dark: bool) -> Scheme {
    let [r, g, b, _] = seed.into_rgba8();
    let hct = Hct::new(Rgb::new(r, g, b));

    Scheme::from(SchemeTonalSpot::new(hct, is_dark, Some(0.0)).scheme)
}

/// Converts a material-colors color into an Iced color.
fn color(rgb: Rgb) -> Color {
    Color::from_rgb8(rgb.red, rgb.green, rgb.blue)
}
