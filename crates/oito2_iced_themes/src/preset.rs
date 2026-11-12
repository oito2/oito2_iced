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

//! The named color presets and their role colors.

use iced::Color;
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast, FamilyColors, Palette, SyntaxColors};

use crate::palettes::catppuccin::{self, Flavor};
use crate::palettes::dracula::Dracula;
use crate::palettes::everforest::Everforest;
use crate::palettes::nord;
use crate::palettes::rose_pine::RosePine;

/// A named color preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Preset {
    /// Everforest Light: a light preset with a green primary.
    EverforestLight,
    /// Everforest Dark: a dark preset with a green primary.
    EverforestDark,
    /// Rosé Pine: the main dark variant, with a pine primary.
    RosePine,
    /// Rosé Pine Moon: the second dark variant, with a pine primary.
    RosePineMoon,
    /// Rosé Pine Dawn: the light variant, with a pine primary.
    RosePineDawn,
    /// Nord Dark: the official dark Nord, with a `nord8` frost primary.
    NordDark,
    /// Nord Light: Polar Night and Snow Storm swapped, with a `nord10`
    /// primary.
    NordLight,
    /// Dracula: a dark preset with a purple primary.
    Dracula,
    /// Alucard: the light variant of Dracula, with a purple primary.
    Alucard,
    /// Catppuccin Latte: the light flavor, with a mauve primary.
    CatppuccinLatte,
    /// Catppuccin Frappé: the lightest dark flavor, with a mauve primary.
    CatppuccinFrappe,
    /// Catppuccin Macchiato: a dark flavor between Frappé and Mocha.
    CatppuccinMacchiato,
    /// Catppuccin Mocha: the darkest flavor, with a mauve primary.
    CatppuccinMocha,
}

impl Preset {
    /// Every preset, in display order.
    pub const ALL: [Preset; 13] = [
        Preset::EverforestLight,
        Preset::EverforestDark,
        Preset::RosePine,
        Preset::RosePineMoon,
        Preset::RosePineDawn,
        Preset::NordDark,
        Preset::NordLight,
        Preset::Dracula,
        Preset::Alucard,
        Preset::CatppuccinLatte,
        Preset::CatppuccinFrappe,
        Preset::CatppuccinMacchiato,
        Preset::CatppuccinMocha,
    ];

    /// The display name, such as `Catppuccin Mocha`.
    pub fn label(self) -> &'static str {
        match self {
            Preset::EverforestLight => "Everforest Light",
            Preset::EverforestDark => "Everforest Dark",
            Preset::RosePine => "Rosé Pine",
            Preset::RosePineMoon => "Rosé Pine Moon",
            Preset::RosePineDawn => "Rosé Pine Dawn",
            Preset::NordDark => "Nord Dark",
            Preset::NordLight => "Nord Light",
            Preset::Dracula => "Dracula",
            Preset::Alucard => "Alucard",
            Preset::CatppuccinLatte => "Catppuccin Latte",
            Preset::CatppuccinFrappe => "Catppuccin Frappé",
            Preset::CatppuccinMacchiato => "Catppuccin Macchiato",
            Preset::CatppuccinMocha => "Catppuccin Mocha",
        }
    }

    /// The camelCase identifier, such as `catppuccinMocha`.
    pub fn name(self) -> &'static str {
        match self {
            Preset::EverforestLight => "everforestLight",
            Preset::EverforestDark => "everforestDark",
            Preset::RosePine => "rosePine",
            Preset::RosePineMoon => "rosePineMoon",
            Preset::RosePineDawn => "rosePineDawn",
            Preset::NordDark => "nordDark",
            Preset::NordLight => "nordLight",
            Preset::Dracula => "dracula",
            Preset::Alucard => "alucard",
            Preset::CatppuccinLatte => "catppuccinLatte",
            Preset::CatppuccinFrappe => "catppuccinFrappe",
            Preset::CatppuccinMacchiato => "catppuccinMacchiato",
            Preset::CatppuccinMocha => "catppuccinMocha",
        }
    }

    /// The preset with the given camelCase [`name`](Self::name), if any.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|preset| preset.name() == name)
    }

    /// Whether this preset is a dark theme.
    pub fn is_dark(self) -> bool {
        self.brightness() == Brightness::Dark
    }

    /// The brightness of this preset.
    pub fn brightness(self) -> Brightness {
        match self {
            Preset::EverforestLight
            | Preset::RosePineDawn
            | Preset::NordLight
            | Preset::Alucard
            | Preset::CatppuccinLatte => Brightness::Light,
            _ => Brightness::Dark,
        }
    }

    /// The presets of the given brightness, in display order.
    pub fn with_brightness(brightness: Brightness) -> impl Iterator<Item = Preset> {
        Self::ALL
            .into_iter()
            .filter(move |preset| preset.brightness() == brightness)
    }

    /// The role colors of this preset, mapped from the palette of its
    /// theme family.
    pub fn family(self) -> FamilyColors {
        let brightness = self.brightness();

        match self {
            Preset::EverforestLight => everforest(Everforest::LIGHT, brightness),
            Preset::EverforestDark => everforest(Everforest::DARK, brightness),
            Preset::RosePine => rose_pine(RosePine::MAIN, brightness),
            Preset::RosePineMoon => rose_pine(RosePine::MOON, brightness),
            Preset::RosePineDawn => rose_pine(RosePine::DAWN, brightness),
            Preset::NordDark => nord_dark(brightness),
            Preset::NordLight => nord_light(brightness),
            Preset::Dracula => dracula(brightness),
            Preset::Alucard => alucard(brightness),
            Preset::CatppuccinLatte => catppuccin(catppuccin::latte(), brightness),
            Preset::CatppuccinFrappe => catppuccin(catppuccin::frappe(), brightness),
            Preset::CatppuccinMacchiato => catppuccin(catppuccin::macchiato(), brightness),
            Preset::CatppuccinMocha => catppuccin(catppuccin::mocha(), brightness),
        }
    }

    /// The palette derived from [`family`](Self::family).
    pub fn palette(self) -> Palette {
        Palette::from_family(&self.family())
    }

    /// The theme of this preset at a contrast level, named after the
    /// [`label`](Self::label), with ` high contrast` appended for
    /// [`Contrast::High`].
    pub fn theme(self, contrast: Contrast) -> Theme {
        let name = match contrast {
            Contrast::Standard => self.label().to_owned(),
            Contrast::High => format!("{} high contrast", self.label()),
        };

        Theme::from_palette(name, &self.palette(), contrast)
    }
}

impl std::fmt::Display for Preset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
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

fn everforest(p: Everforest, brightness: Brightness) -> FamilyColors {
    FamilyColors {
        brightness,
        background: p.bg_dim,
        card: p.bg0,
        foreground: p.fg,
        primary: p.green,
        secondary: p.bg1,
        muted_foreground: p.grey1,
        destructive: p.red,
        border: if brightness == Brightness::Dark {
            p.bg2
        } else {
            p.bg3
        },
        success: p.green,
        warning: p.yellow,
        info: p.blue,
        syntax: SyntaxColors {
            foreground: p.fg,
            comment: p.grey1,
            keyword: p.red,
            string: p.aqua,
            number: p.purple,
            function: p.green,
            r#type: p.yellow,
            tag: p.orange,
            attribute: p.blue,
            constant: p.purple,
            invalid: p.red,
        },
    }
}

fn rose_pine(p: RosePine, brightness: Brightness) -> FamilyColors {
    FamilyColors {
        brightness,
        background: p.base,
        card: p.surface,
        foreground: p.text,
        primary: p.pine,
        secondary: p.overlay,
        muted_foreground: p.muted,
        destructive: p.love,
        border: p.overlay,
        success: p.foam,
        warning: p.gold,
        info: p.iris,
        syntax: SyntaxColors {
            foreground: p.text,
            comment: p.muted,
            keyword: p.pine,
            string: p.gold,
            number: p.gold,
            function: p.rose,
            r#type: p.foam,
            tag: p.foam,
            attribute: p.iris,
            constant: p.gold,
            invalid: p.love,
        },
    }
}

fn nord_dark(brightness: Brightness) -> FamilyColors {
    FamilyColors {
        brightness,
        background: nord::NORD0,
        card: nord::NORD1,
        foreground: nord::NORD4,
        primary: nord::NORD8,
        secondary: nord::NORD2,
        muted_foreground: nord::NORD3,
        destructive: nord::NORD11,
        border: nord::NORD3,
        success: nord::NORD14,
        warning: nord::NORD13,
        info: nord::NORD9,
        syntax: SyntaxColors {
            foreground: nord::NORD4,
            comment: nord::NORD3,
            keyword: nord::NORD9,
            string: nord::NORD14,
            number: nord::NORD15,
            function: nord::NORD8,
            r#type: nord::NORD7,
            tag: nord::NORD9,
            attribute: nord::NORD7,
            constant: nord::NORD12,
            invalid: nord::NORD11,
        },
    }
}

fn nord_light(brightness: Brightness) -> FamilyColors {
    FamilyColors {
        brightness,
        background: nord::NORD6,
        card: nord::NORD5,
        foreground: nord::NORD0,
        primary: nord::NORD10,
        secondary: nord::NORD4,
        muted_foreground: nord::NORD3,
        destructive: nord::NORD11,
        border: nord::NORD4,
        success: nord::NORD14,
        warning: nord::NORD12,
        info: nord::NORD10,
        syntax: SyntaxColors {
            foreground: nord::NORD0,
            comment: nord::NORD3,
            keyword: nord::NORD10,
            string: nord::NORD14,
            number: nord::NORD15,
            function: nord::NORD8,
            r#type: nord::NORD7,
            tag: nord::NORD10,
            attribute: nord::NORD7,
            constant: nord::NORD12,
            invalid: nord::NORD11,
        },
    }
}

fn dracula(brightness: Brightness) -> FamilyColors {
    let p = Dracula::DRACULA;

    FamilyColors {
        brightness,
        background: p.background,
        card: p.current_line,
        foreground: p.foreground,
        primary: p.purple,
        secondary: lerp(p.current_line, p.foreground, 0.12),
        muted_foreground: p.comment,
        destructive: p.red,
        border: lerp(p.current_line, p.foreground, 0.15),
        success: p.green,
        warning: p.yellow,
        info: p.cyan,
        syntax: dracula_syntax(p),
    }
}

fn alucard(brightness: Brightness) -> FamilyColors {
    let p = Dracula::ALUCARD;

    FamilyColors {
        brightness,
        background: p.background,
        card: p.background,
        foreground: p.foreground,
        primary: p.purple,
        secondary: lerp(p.background, p.foreground, 0.06),
        muted_foreground: p.comment,
        destructive: p.red,
        border: lerp(p.background, p.foreground, 0.15),
        success: p.green,
        warning: p.orange,
        info: p.cyan,
        syntax: dracula_syntax(p),
    }
}

fn dracula_syntax(p: Dracula) -> SyntaxColors {
    SyntaxColors {
        foreground: p.foreground,
        comment: p.comment,
        keyword: p.pink,
        string: p.yellow,
        number: p.purple,
        function: p.green,
        r#type: p.cyan,
        tag: p.pink,
        attribute: p.green,
        constant: p.purple,
        invalid: p.red,
    }
}

fn catppuccin(f: Flavor, brightness: Brightness) -> FamilyColors {
    FamilyColors {
        brightness,
        background: f.mantle,
        card: f.base,
        foreground: f.text,
        primary: f.mauve,
        secondary: f.surface0,
        muted_foreground: f.overlay0,
        destructive: f.red,
        border: f.surface1,
        success: f.green,
        warning: f.yellow,
        info: f.blue,
        syntax: SyntaxColors {
            foreground: f.text,
            comment: f.overlay2,
            keyword: f.mauve,
            string: f.green,
            number: f.peach,
            function: f.blue,
            r#type: f.yellow,
            tag: f.blue,
            attribute: f.yellow,
            constant: f.peach,
            invalid: f.red,
        },
    }
}
