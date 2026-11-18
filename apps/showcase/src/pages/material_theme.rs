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

//! The Material theme page: the color roles of the active theme, the
//! accents, the type scale with the Inter weights, the tokens, the density,
//! the preset themes and the motion curves.

use std::sync::LazyLock;

use iced::font::Weight;
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Color, Element, Length, Task, border};

use oito2_iced::animation;
use oito2_iced::density::{self, Density};
use oito2_iced::theme::ColorRoles;
use oito2_iced::tokens::{Accent, radius, sizes, spacing, typography};
use oito2_iced::widget::button;
use oito2_iced::widget::styled_text;
use oito2_iced::{Theme, font};
use oito2_iced_themes::Preset;

use crate::catalog::{Action, Shell};
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("material_theme.rs");

/// A color of the theme, read from its roles.
type RoleColor = fn(&ColorRoles) -> Color;

/// A role of the theme: its name, the color it fills with and the color of
/// the text on it.
type Role = (&'static str, RoleColor, RoleColor);

/// The accent and error fills.
const FILLS: [Role; 8] = [
    ("primary", |r| r.primary, |r| r.on_primary),
    (
        "primaryContainer",
        |r| r.primary_container,
        |r| r.on_primary_container,
    ),
    ("secondary", |r| r.secondary, |r| r.on_secondary),
    (
        "secondaryContainer",
        |r| r.secondary_container,
        |r| r.on_secondary_container,
    ),
    ("tertiary", |r| r.tertiary, |r| r.on_tertiary),
    (
        "tertiaryContainer",
        |r| r.tertiary_container,
        |r| r.on_tertiary_container,
    ),
    ("error", |r| r.error, |r| r.on_error),
    (
        "errorContainer",
        |r| r.error_container,
        |r| r.on_error_container,
    ),
];

/// The surfaces, from the page background to the inverse surface.
const SURFACES: [Role; 9] = [
    ("background", |r| r.background, |r| r.on_surface),
    ("lowest", |r| r.surface_container_lowest, |r| r.on_surface),
    ("low", |r| r.surface_container_low, |r| r.on_surface),
    ("surface", |r| r.surface, |r| r.on_surface),
    ("container", |r| r.surface_container, |r| r.on_surface),
    ("high", |r| r.surface_container_high, |r| r.on_surface),
    (
        "highest",
        |r| r.surface_container_highest,
        |r| r.on_surface_variant,
    ),
    (
        "inverseSurface",
        |r| r.inverse_surface,
        |r| r.on_inverse_surface,
    ),
    ("outline", |r| r.outline, |r| r.surface),
];

/// The colors of the window chrome.
const CHROME: [Role; 5] = [
    ("titleBar", |r| r.chrome.title_bar, |r| r.on_surface),
    ("sidebar", |r| r.chrome.sidebar, |r| r.on_surface),
    ("mainbar", |r| r.chrome.mainbar, |r| r.on_surface),
    ("background", |r| r.chrome.background, |r| r.on_surface),
    ("dialog", |r| r.chrome.dialog, |r| r.on_surface),
];

/// The colors that stay readable as text on every surface.
const TEXT: [(&str, RoleColor); 8] = [
    ("onSurface", |r| r.on_surface),
    ("onSurfaceVariant", |r| r.on_surface_variant),
    ("primaryText", |r| r.semantic.primary_text),
    ("errorText", |r| r.semantic.error_text),
    ("success", |r| r.semantic.success),
    ("warning", |r| r.semantic.warning),
    ("info", |r| r.semantic.info),
    ("link", |r| r.semantic.link),
];

/// The size tokens with their values.
static SIZES: LazyLock<Vec<[String; 2]>> = LazyLock::new(|| {
    [
        ("BUTTON_HEIGHT", sizes::BUTTON_HEIGHT),
        ("TOUCH_BUTTON_HEIGHT", sizes::TOUCH_BUTTON_HEIGHT),
        ("TITLE_BAR_HEIGHT", sizes::TITLE_BAR_HEIGHT),
        ("TITLE_BAR_ITEM_HEIGHT", sizes::TITLE_BAR_ITEM_HEIGHT),
        ("ICON_SIZE", sizes::ICON_SIZE),
        ("FOCUS_BORDER_WIDTH", sizes::FOCUS_BORDER_WIDTH),
        ("WINDOW_CONTROL_SIZE", sizes::WINDOW_CONTROL_SIZE),
        (
            "WINDOWS_WINDOW_CONTROL_WIDTH",
            sizes::WINDOWS_WINDOW_CONTROL_WIDTH,
        ),
        ("MIN_POINTER_TARGET", sizes::MIN_POINTER_TARGET),
        ("MIN_TOUCH_TARGET", sizes::MIN_TOUCH_TARGET),
    ]
    .map(|(name, value)| [format!("`sizes::{name}`"), value.to_string()])
    .to_vec()
});

/// The control height and icon size of each density.
static DENSITIES: LazyLock<Vec<[String; 3]>> = LazyLock::new(|| {
    [Density::Pointer, Density::Touch]
        .map(|density| {
            [
                format!("`Density::{density:?}`"),
                density::control_height_of(density).to_string(),
                density::icon_size_of(density).to_string(),
            ]
        })
        .to_vec()
});

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {
    /// Opens the settings of the showcase.
    Settings,
}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, shell: &mut Shell) -> Task<Message> {
    match message {
        Message::Settings => shell.request(Action::OpenSettings),
    }

    Task::none()
}

// #region swatch
/// A swatch of `role` in the active theme, labeled in its text color.
fn swatch<'a>((name, fill, on): Role) -> Element<'a, Message, Theme> {
    container(styled_text(name, typography::LABEL_MEDIUM))
        .padding(spacing::SM)
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(56.0))
        .style(move |theme: &Theme| {
            let roles = theme.roles();
            container::Style::default()
                .background(fill(roles))
                .color(on(roles))
                .border(
                    border::rounded(radius::MEDIUM)
                        .color(roles.outline_variant)
                        .width(1),
                )
        })
        .into()
}
// #endregion

/// A wrapping row of the swatches of `roles`.
fn swatches<'a>(roles: &[Role]) -> Element<'a, Message, Theme> {
    row(roles.iter().copied().map(swatch))
        .spacing(spacing::SM)
        .wrap()
        .vertical_spacing(spacing::SM)
        .into()
}

/// A swatch of a fixed `color`, labeled `name`, with `on` for the label.
fn fixed<'a>(
    name: impl text::IntoFragment<'a>,
    color: Color,
    on: Color,
) -> Element<'a, Message, Theme> {
    container(styled_text(name, typography::LABEL_MEDIUM).color(on))
        .padding(spacing::SM)
        .width(Length::Fixed(150.0))
        .height(Length::Fixed(56.0))
        .style(move |_: &Theme| {
            container::Style::default()
                .background(color)
                .border(border::rounded(radius::MEDIUM))
        })
        .into()
}

/// The number of a font weight, such as 600 for semibold.
fn weight_value(weight: Weight) -> u16 {
    match weight {
        Weight::Thin => 100,
        Weight::ExtraLight => 200,
        Weight::Light => 300,
        Weight::Normal => 400,
        Weight::Medium => 500,
        Weight::Semibold => 600,
        Weight::Bold => 700,
        Weight::ExtraBold => 800,
        Weight::Black => 900,
    }
}

/// A row of bars that plots `easing` from 0 to 1, 48 pixels tall at 1.
fn curve<'a>(easing: fn(f32) -> f32) -> Element<'a, Message, Theme> {
    row((0..=24).map(|step| {
        let value = easing(step as f32 / 24.0).clamp(0.0, 1.2);

        container(Space::new())
            .width(Length::Fixed(6.0))
            .height(Length::Fixed(2.0 + value * 46.0))
            .style(|theme: &Theme| {
                container::Style::default()
                    .background(theme.roles().primary)
                    .border(border::rounded(radius::CHECK / 2.0))
            })
            .into()
    }))
    .spacing(spacing::XXS)
    .align_y(Alignment::End)
    .height(Length::Fixed(56.0))
    .into()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    let preview = doc::example(
        ctx,
        SOURCE,
        &["swatch"],
        column![
            swatches(&FILLS),
            button::tonal("Change the theme").on_press(Message::Settings),
        ]
        .spacing(spacing::MD)
        .align_x(Alignment::Center),
    );

    let text_colors = row(TEXT.iter().map(|&(name, color)| {
        styled_text(name, typography::TITLE_SMALL)
            .style(move |theme: &Theme| text::Style {
                color: Some(color(theme.roles())),
            })
            .into()
    }))
    .spacing(spacing::LG)
    .wrap()
    .vertical_spacing(spacing::SM);

    let accents = row(Accent::ALL.map(|accent| fixed(accent.name(), accent.color(), Color::WHITE)))
        .spacing(spacing::SM)
        .wrap()
        .vertical_spacing(spacing::SM);

    let scale = doc::example(
        ctx,
        SOURCE,
        &["scale"],
        column(
            // #region scale
            typography::SCALE.map(|(name, style)| {
                column![
                    doc::caption(format!(
                        "{name} · {} · w{}",
                        style.size,
                        weight_value(style.weight)
                    )),
                    styled_text("The quick brown fox jumps over the lazy dog", style),
                ]
                .into()
            }),
            // #endregion
        )
        .spacing(spacing::SM)
        .width(Length::Fill),
    );

    let weights = doc::example(
        ctx,
        SOURCE,
        &["weights"],
        column(
            // #region weights
            [
                ("Regular 400", Weight::Normal),
                ("Medium 500", Weight::Medium),
                ("SemiBold 600", Weight::Semibold),
                ("Bold 700", Weight::Bold),
            ]
            .map(|(label, weight)| {
                text(format!("Inter — {label}"))
                    .font(font::inter(weight))
                    .size(18)
                    .into()
            }),
            // #endregion
        )
        .spacing(spacing::XS),
    );

    let radii = row([
        ("check", radius::CHECK),
        ("small", radius::SMALL),
        ("medium", radius::MEDIUM),
        ("large", radius::LARGE),
        ("window", radius::WINDOW),
    ]
    .map(|(name, value)| {
        container(styled_text(
            format!("{name}\n{value}"),
            typography::LABEL_MEDIUM,
        ))
        .center(Length::Fixed(72.0))
        .style(move |theme: &Theme| {
            container::Style::default()
                .background(theme.roles().secondary_container)
                .color(theme.roles().on_secondary_container)
                .border(border::rounded(value))
        })
        .into()
    }))
    .spacing(spacing::MD)
    .wrap()
    .vertical_spacing(spacing::MD);

    let spacings = column(
        [
            ("xxs", spacing::XXS),
            ("xs", spacing::XS),
            ("sm", spacing::SM),
            ("md", spacing::MD),
            ("lg", spacing::LG),
            ("page", spacing::PAGE),
            ("xl", spacing::XL),
            ("xxl", spacing::XXL),
            ("xxxl", spacing::XXXL),
        ]
        .map(|(name, value)| {
            row![
                text(format!("{name} {value}")).width(Length::Fixed(80.0)),
                container(Space::new())
                    .width(Length::Fixed(value * 4.0))
                    .height(Length::Fixed(12.0))
                    .style(|theme: &Theme| {
                        container::Style::default().background(theme.roles().primary)
                    }),
            ]
            .align_y(Alignment::Center)
            .into()
        }),
    )
    .spacing(spacing::XS);

    let size_rows: Vec<Vec<&str>> = SIZES
        .iter()
        .map(|row| row.iter().map(String::as_str).collect())
        .collect();
    let density_rows: Vec<Vec<&str>> = DENSITIES
        .iter()
        .map(|row| row.iter().map(String::as_str).collect())
        .collect();
    let density_table = doc::table(&["Density", "Control height", "Icon size"], &density_rows);

    let presets = doc::example(
        ctx,
        SOURCE,
        &["presets"],
        row(
            // #region presets
            Preset::ALL.map(|preset| {
                let palette = preset.palette();
                fixed(preset.label(), palette.primary, palette.on_primary)
            }),
            // #endregion
        )
        .spacing(spacing::SM)
        .wrap()
        .vertical_spacing(spacing::SM),
    );

    let curves = row([
        ("emphasized", animation::emphasized as fn(f32) -> f32),
        ("ease_out_cubic", animation::ease_out_cubic),
        ("ease_in_out_cubic", animation::ease_in_out_cubic),
        ("ease_out_back", animation::ease_out_back),
    ]
    .map(|(name, easing)| {
        column![curve(easing), doc::caption(name)]
            .spacing(spacing::XS)
            .into()
    }))
    .spacing(spacing::XL)
    .wrap()
    .vertical_spacing(spacing::MD);

    doc::page(
        "Material theme",
        "The colors, type scale, tokens, density and preset themes of the OITO2 theme.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`Theme` resolves every color role from a brightness and an accent: \
                 `Theme::new(Brightness::Light, Accent::Blue)`, or `Theme::light()` and \
                 `Theme::dark()` with the default accent. `Theme::base` takes any accent color \
                 and a `Contrast`, `Theme::from_palette` a full `Palette`, and \
                 `with_density` the control sizes. Return it from the `theme` function of the \
                 application; the widgets read the roles through `theme.roles()`.",
            ),
            doc::code(
                ctx,
                "use oito2_iced::Theme;\n\
                 use oito2_iced::theme::{Brightness, Contrast};\n\
                 use oito2_iced::tokens::Accent;\n\n\
                 let theme = Theme::base(Brightness::Dark, Accent::Teal.color(), Contrast::High);",
            ),
            doc::prose(
                "Style closures read the roles too, so custom styles follow the theme, its \
                 brightness and its contrast. The swatches on this page are drawn that way: \
                 change the theme in the settings to see them change.",
            ),
            doc::heading("Colors"),
            doc::subheading("Accent and error fills"),
            doc::prose(
                "The preview shows the accent and error fills. Each fill has an `on_` color for the text and icons on it. The containers are \
                 the softer fills of tonal buttons, chips and selections.",
            ),
            doc::subheading("Surfaces"),
            swatches(&SURFACES),
            doc::subheading("Text-safe colors"),
            doc::prose(
                "`on_surface`, `on_surface_variant` and the `semantic` colors reach 4.5:1 \
                 against every surface, 7:1 with `Contrast::High`.",
            ),
            text_colors.into(),
            doc::subheading("Window chrome"),
            doc::prose(
                "`chrome` holds the colors of the title bar, sidebar, main bar, window \
                 background and dialogs, each a step apart from its neighbors.",
            ),
            swatches(&CHROME),
            doc::prose(
                "`syntax` holds the colors of code highlighting, used by the code blocks of \
                 `oito2_iced_syntax`.",
            ),
            doc::subheading("Accents"),
            doc::prose(
                "`Accent` lists the nine accents a theme can take as its primary color; the \
                 theme adjusts the raw color for contrast.",
            ),
            accents.into(),
            doc::heading("Typography"),
            doc::prose(
                "The type scale has fifteen `TextStyle`s in `typography`, each with a size, a \
                 line height, a weight and a letter spacing. `styled_text` shows a text in a \
                 style.",
            ),
            scale,
            doc::subheading("Inter weights"),
            doc::prose(
                "Inter is embedded as static instances in the family `OITO2 Inter`: Regular, \
                 Medium, SemiBold and Bold, upright and italic. Register `font::INTER_FONTS` \
                 with the application and use `font::INTER` as the default font; \
                 `font::inter(weight)` selects a weight, and other weights resolve to the \
                 nearest embedded one.",
            ),
            weights,
            doc::heading("Tokens"),
            doc::subheading("Radii"),
            radii.into(),
            doc::subheading("Spacing"),
            spacings.into(),
            doc::subheading("Sizes"),
            doc::table(&["Token", "Pixels"], &size_rows),
            doc::subheading("Density"),
            doc::prose(
                "The density sets the size of the controls: pointer sizes for a mouse, touch \
                 sizes for fingers. Iced lays widgets out without the theme, so the \
                 application calls `density::set` with the density of its theme at the start \
                 of `view`, and the widgets read it with `density::control_height` and \
                 `density::icon_size`.",
            ),
            density_table,
            doc::heading("Preset themes"),
            doc::prose(
                "`oito2_iced_themes::Preset` lists thirteen named themes: Everforest, Rosé \
                 Pine, Nord, Dracula and Catppuccin in their light and dark variants. \
                 `preset.theme(contrast)` builds the theme and `preset.palette()` gives its \
                 colors.",
            ),
            presets,
            doc::heading("Motion"),
            doc::prose(
                "`animation` holds the Material easing curves for `iced::Animation` and the \
                 durations, from `duration::SHORT` (100 ms) to `duration::EXTRA_LONG` (500 \
                 ms). `emphasized_animation` builds an animation with the emphasized curve, \
                 and `motion_duration` drops the duration when the system asks for reduced \
                 motion. The bars below plot each curve from start to end.",
            ),
            curves.into(),
            doc::api(
                ctx,
                &[
                    "oito2_iced::Theme",
                    "oito2_iced::theme::ColorRoles",
                    "oito2_iced::theme::Brightness",
                    "oito2_iced::theme::Contrast",
                    "oito2_iced::theme::Palette",
                    "oito2_iced::tokens::Accent",
                    "oito2_iced::tokens::typography::TextStyle",
                    "oito2_iced::widget::styled_text",
                    "oito2_iced::font::inter",
                    "oito2_iced::density::set",
                    "oito2_iced_themes::Preset",
                    "oito2_iced::animation::emphasized_animation",
                ],
            ),
        ],
    )
}
