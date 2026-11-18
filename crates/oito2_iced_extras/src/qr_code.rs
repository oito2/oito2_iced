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

//! QR codes: the module grid of a text and a widget that paints it.
//!
//! [`modules`] encodes a text into its square grid of dark and light
//! modules: one segment for the whole text (numeric when it holds only
//! digits, alphanumeric when it holds only the QR alphanumeric characters,
//! otherwise UTF-8 bytes, announced by an ECI designator when a character
//! is beyond Latin-1), the smallest version that fits and the mask with the
//! lowest penalty. The same text gives the same grid as the OITO2 Flutter
//! widgets.
//!
//! [`qr_code`] paints that grid, a quiet zone of four modules included, in
//! a square of [`QrCode::size`] pixels: black on white by default, which
//! every scanner reads, or in the colors of a style class. A center image,
//! such as a logo, covers the middle of the modules and raises the error
//! level to [`ErrorLevel::High`]. When the text doesn't fit, the localized
//! "too much data" message, or the element given to [`QrCode::error`],
//! shows instead. Encoded grids are kept for reuse, so a view that is
//! built again doesn't encode the text again.
//!
//! ```no_run
//! use oito2_iced::l10n::Localizations;
//! use oito2_iced_extras::qr_code::{self, ErrorLevel};
//!
//! # fn view<'a, Message: 'a>(l10n: &Localizations) -> oito2_iced::Element<'a, Message> {
//! qr_code::qr_code("https://oito2.github.io/oito2_ui/")
//!     .error_level(ErrorLevel::Quartile)
//!     .rounded_modules(true)
//!     .localizations(l10n)
//!     .into()
//! # }
//! ```

use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;

use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::{Tree, Widget};
use iced::advanced::{Renderer as _, renderer};
use iced::alignment::{Horizontal, Vertical};
use iced::widget::qr_code::{Catalog, Style};
use iced::widget::{container, image, stack};
use iced::{Border, Color, Element as IcedElement, Length, Rectangle, Size, mouse};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::qr_code::Class;
use oito2_iced::tokens::typography;
use oito2_iced::widget::styled_text;
use oito2_iced::{Element, Theme};
use qrcode::bits::Bits;
use qrcode::canvas::{Canvas, MaskPattern};
use qrcode::{EcLevel, Version};

/// The default width and height of a [`QrCode`], quiet zone included.
pub const SIZE: f32 = 160.0;

/// The width of the quiet zone around the modules, in modules.
pub const QUIET_ZONE: usize = 4;

/// The default share of the module area's width cleared for a center
/// image.
pub const CENTER_IMAGE_FRACTION: f32 = 0.22;

/// The largest share of the module area's width a center image may clear.
pub const MAX_CENTER_IMAGE_FRACTION: f32 = 0.3;

/// How many encoded grids are kept for reuse.
const CACHE_SIZE: usize = 32;

/// The ECI designator of UTF-8.
const UTF8_ECI: u32 = 26;

/// How much of a QR code can be damaged or covered and still read, as
/// error correction data takes more room.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ErrorLevel {
    /// About 7%.
    Low,
    /// About 15%.
    #[default]
    Medium,
    /// About 25%.
    Quartile,
    /// About 30%.
    High,
}

impl ErrorLevel {
    /// Every level, from the lowest to the highest.
    pub const ALL: [ErrorLevel; 4] = [
        ErrorLevel::Low,
        ErrorLevel::Medium,
        ErrorLevel::Quartile,
        ErrorLevel::High,
    ];

    /// The level's name, as the OITO2 Flutter widgets spell it.
    #[must_use]
    pub fn name(self) -> &'static str {
        match self {
            ErrorLevel::Low => "low",
            ErrorLevel::Medium => "medium",
            ErrorLevel::Quartile => "quartile",
            ErrorLevel::High => "high",
        }
    }

    fn ec_level(self) -> EcLevel {
        match self {
            ErrorLevel::Low => EcLevel::L,
            ErrorLevel::Medium => EcLevel::M,
            ErrorLevel::Quartile => EcLevel::Q,
            ErrorLevel::High => EcLevel::H,
        }
    }
}

/// The error of [`modules`] when a text doesn't fit in a QR code at the
/// chosen error level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataTooLong {
    /// The bits the text needs in the largest version.
    pub required_bits: usize,
    /// The data bits the largest version holds at the chosen level.
    pub max_bits: usize,
}

impl fmt::Display for DataTooLong {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the data needs {} bits, more than the {} bits a QR code holds",
            self.required_bits, self.max_bits
        )
    }
}

impl std::error::Error for DataTooLong {}

/// The dark and light modules of a QR code: a square grid of
/// [`Modules::count`] rows and columns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modules {
    count: usize,
    dark: Vec<bool>,
}

impl Modules {
    /// The number of modules on each side.
    #[must_use]
    pub fn count(&self) -> usize {
        self.count
    }

    /// Whether the module at `row` and `column` is dark; false outside the
    /// grid.
    #[must_use]
    pub fn is_dark(&self, row: usize, column: usize) -> bool {
        row < self.count && column < self.count && self.dark[row * self.count + column]
    }

    /// The rows of the grid, from the top, each from the left, with true
    /// for a dark module.
    pub fn rows(&self) -> impl Iterator<Item = &[bool]> {
        self.dark.chunks(self.count.max(1))
    }
}

/// The encoding mode of the single segment that holds a text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Mode {
    Numeric,
    Alphanumeric,
    Byte { utf8_eci: bool },
}

impl Mode {
    /// The mode of `data`: numeric for digits only, alphanumeric for the QR
    /// alphanumeric characters only, otherwise bytes with the UTF-8 ECI
    /// designator when a character is beyond Latin-1.
    fn of(data: &str) -> Mode {
        if !data.is_empty() && data.bytes().all(|b| b.is_ascii_digit()) {
            Mode::Numeric
        } else if !data.is_empty() && data.bytes().all(is_alphanumeric) {
            Mode::Alphanumeric
        } else {
            Mode::Byte {
                utf8_eci: data.chars().any(|c| u32::from(c) > 0xFF),
            }
        }
    }

    /// The width of the character count of the segment at `version`.
    fn count_bits(self, version: i16) -> usize {
        let group = match version {
            ..10 => 0,
            10..27 => 1,
            _ => 2,
        };
        match self {
            Mode::Numeric => [10, 12, 14][group],
            Mode::Alphanumeric => [9, 11, 13][group],
            Mode::Byte { .. } => [8, 16, 16][group],
        }
    }

    /// The bits of the segment's content.
    fn content_bits(self, data: &str) -> usize {
        let length = data.len();
        match self {
            Mode::Numeric => length / 3 * 10 + [0, 4, 7][length % 3],
            Mode::Alphanumeric => length / 2 * 11 + 6 * (length % 2),
            Mode::Byte { .. } => length * 8,
        }
    }

    /// The bits `data` takes at `version`: the ECI designator, if any, and
    /// the segment's mode, count and content.
    fn required_bits(self, data: &str, version: i16) -> usize {
        let eci = match self {
            Mode::Byte { utf8_eci: true } => 4 + 8,
            _ => 0,
        };
        eci + 4 + self.count_bits(version) + self.content_bits(data)
    }
}

/// Whether `byte` is one of the QR alphanumeric characters.
fn is_alphanumeric(byte: u8) -> bool {
    byte.is_ascii_digit() || byte.is_ascii_uppercase() || b" $%*+-./:".contains(&byte)
}

/// The data bits a version holds at a level.
fn capacity(version: i16, level: EcLevel) -> usize {
    Bits::new(Version::Normal(version))
        .max_len(level)
        .unwrap_or(0)
}

/// Encodes `data` into its QR code modules at `level`.
///
/// The text takes a single segment and the smallest version that holds
/// it; of the eight masks, the one with the lowest penalty wins (the
/// first one on a tie). The penalty counts, with the format and version
/// information still light: 3 plus one per extra neighbor for each module
/// with more than five of its eight neighbors of its color, 3 for each
/// two-by-two block of one color, 40 for each dark-light-dark-dark-dark-
/// light-dark run in a row or column, and ten times the distance of the
/// dark share from half, in steps of five percent.
///
/// # Errors
///
/// Returns [`DataTooLong`] when `data` doesn't fit in a QR code at
/// `level`.
pub fn modules(data: &str, level: ErrorLevel) -> Result<Modules, DataTooLong> {
    let mode = Mode::of(data);
    let ec = level.ec_level();
    let too_long = DataTooLong {
        required_bits: mode.required_bits(data, 40),
        max_bits: capacity(40, ec),
    };
    let version = (1..=40)
        .find(|&version| mode.required_bits(data, version) <= capacity(version, ec))
        .ok_or(too_long)?;
    let version = Version::Normal(version);

    let mut bits = Bits::new(version);
    let pushed = match mode {
        Mode::Numeric => bits.push_numeric_data(data.as_bytes()),
        Mode::Alphanumeric => bits.push_alphanumeric_data(data.as_bytes()),
        Mode::Byte { utf8_eci } => (if utf8_eci {
            bits.push_eci_designator(UTF8_ECI)
        } else {
            Ok(())
        })
        .and_then(|()| bits.push_byte_data(data.as_bytes())),
    };
    pushed
        .and_then(|()| bits.push_terminator(ec))
        .map_err(|_| too_long)?;
    let (data_codewords, ec_codewords) =
        qrcode::ec::construct_codewords(&bits.into_bytes(), version, ec).map_err(|_| too_long)?;

    let mut canvas = Canvas::new(version, ec);
    canvas.draw_all_functional_patterns();
    canvas.draw_data(&data_codewords, &ec_codewords);

    let count = usize::try_from(version.width()).unwrap_or(0);
    let mut best: Option<(f64, Vec<bool>)> = None;

    for pattern in MASKS {
        let mut masked = canvas.clone();
        masked.apply_mask(pattern);
        let dark = grid(&masked, count);
        let score = penalty(&scoring_grid(&dark, count), count);

        if best.as_ref().is_none_or(|(lowest, _)| score < *lowest) {
            best = Some((score, dark));
        }
    }

    Ok(Modules {
        count,
        dark: best.map(|(_, dark)| dark).unwrap_or_default(),
    })
}

/// The eight masks, in the order of their pattern numbers.
const MASKS: [MaskPattern; 8] = [
    MaskPattern::Checkerboard,
    MaskPattern::HorizontalLines,
    MaskPattern::VerticalLines,
    MaskPattern::DiagonalLines,
    MaskPattern::LargeCheckerboard,
    MaskPattern::Fields,
    MaskPattern::Diamonds,
    MaskPattern::Meadow,
];

/// The modules of `canvas`, row by row, with true for a dark module.
fn grid(canvas: &Canvas, count: usize) -> Vec<bool> {
    let side = i16::try_from(count).unwrap_or(0);
    (0..side)
        .flat_map(|y| (0..side).map(move |x| canvas.get(x, y).is_dark()))
        .collect()
}

/// A copy of `dark` with the format information, the dark module and the
/// version information light, as the penalty is counted.
fn scoring_grid(dark: &[bool], count: usize) -> Vec<bool> {
    let mut grid = dark.to_vec();
    let mut clear = |row: usize, column: usize| grid[row * count + column] = false;

    for row in (0..9).filter(|&row| row != 6).chain(count - 8..count) {
        clear(row, 8);
    }
    for column in (0..9).filter(|&column| column != 6).chain(count - 8..count) {
        clear(8, column);
    }
    // Versions 7 and up carry their number in two 6 by 3 blocks.
    if count >= 45 {
        for a in 0..6 {
            for b in count - 11..count - 8 {
                clear(a, b);
                clear(b, a);
            }
        }
    }
    grid
}

/// The penalty of the masked `dark` modules; see [`modules`].
fn penalty(dark: &[bool], count: usize) -> f64 {
    let at = |row: usize, column: usize| dark[row * count + column];
    let finder = [true, false, true, true, true, false, true];
    let mut points = 0u32;
    let mut dark_count = 0usize;

    for row in 0..count {
        for column in 0..count {
            let color = at(row, column);
            dark_count += usize::from(color);

            let mut same = 0;
            for r in row.saturating_sub(1)..=(row + 1).min(count - 1) {
                for c in column.saturating_sub(1)..=(column + 1).min(count - 1) {
                    if (r, c) != (row, column) && at(r, c) == color {
                        same += 1;
                    }
                }
            }
            if same > 5 {
                points += 3 + same - 5;
            }

            if row + 1 < count
                && column + 1 < count
                && at(row, column + 1) == color
                && at(row + 1, column) == color
                && at(row + 1, column + 1) == color
            {
                points += 3;
            }

            if color {
                if column + 6 < count
                    && finder
                        .iter()
                        .enumerate()
                        .all(|(i, &dark)| at(row, column + i) == dark)
                {
                    points += 40;
                }
                if row + 6 < count
                    && finder
                        .iter()
                        .enumerate()
                        .all(|(i, &dark)| at(row + i, column) == dark)
                {
                    points += 40;
                }
            }
        }
    }

    let side = count as f64;
    let ratio = (100.0 * dark_count as f64 / side / side - 50.0).abs() / 5.0;
    f64::from(points) + ratio * 10.0
}

/// An encoded grid by text and level; `None` marks a text that doesn't
/// fit.
type CacheEntry = ((String, ErrorLevel), Option<Rc<Modules>>);

thread_local! {
    /// The last encoded grids, the oldest first.
    static CACHE: RefCell<VecDeque<CacheEntry>> =
        const { RefCell::new(VecDeque::new()) };
}

/// The encoded grid of `data` at `level`, from the cache when it was
/// encoded before, or `None` when it doesn't fit.
fn cached(data: &str, level: ErrorLevel) -> Option<Rc<Modules>> {
    CACHE.with_borrow_mut(|cache| {
        if let Some((_, modules)) = cache
            .iter()
            .find(|((text, l), _)| *l == level && text == data)
        {
            return modules.clone();
        }
        if cache.len() >= CACHE_SIZE {
            cache.pop_front();
        }
        let modules = modules(data, level).ok().map(Rc::new);
        cache.push_back(((data.to_owned(), level), modules.clone()));
        modules
    })
}

/// A QR code of a text. See [`qr_code`].
pub struct QrCode<'a, Message> {
    data: String,
    size: f32,
    error_level: ErrorLevel,
    color: Option<Color>,
    background: Option<Color>,
    class: Class<'a>,
    rounded: bool,
    center_image: Option<image::Handle>,
    center_image_fraction: f32,
    localizations: Localizations,
    error: Option<Element<'a, Message>>,
}

/// Creates a QR code of `data`, such as a link.
pub fn qr_code<'a, Message>(data: impl Into<String>) -> QrCode<'a, Message> {
    QrCode {
        data: data.into(),
        size: SIZE,
        error_level: ErrorLevel::Medium,
        color: None,
        background: None,
        class: Class::Default,
        rounded: false,
        center_image: None,
        center_image_fraction: CENTER_IMAGE_FRACTION,
        localizations: Localizations::default(),
        error: None,
    }
}

impl<'a, Message: 'a> QrCode<'a, Message> {
    /// Sets the width and height, quiet zone included; [`SIZE`] by
    /// default.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Sets the error correction level; [`ErrorLevel::Medium`] by default
    /// and ignored with a [center image](Self::center_image), which uses
    /// [`ErrorLevel::High`].
    #[must_use]
    pub fn error_level(mut self, level: ErrorLevel) -> Self {
        self.error_level = level;
        self
    }

    /// Sets the dark module color over the style's.
    ///
    /// Custom colors need strong contrast, with the dark modules darker
    /// than the background.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the light module and quiet zone color over the style's.
    #[must_use]
    pub fn background_color(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the style class; [`Class::Default`], black on white, by
    /// default.
    #[must_use]
    pub fn class(mut self, class: impl Into<Class<'a>>) -> Self {
        self.class = class.into();
        self
    }

    /// Sets the style from a closure.
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme) -> Style + 'a) -> Self {
        self.class = Class::Custom(Box::new(style));
        self
    }

    /// Sets whether the dark modules are drawn as rounded squares.
    #[must_use]
    pub fn rounded_modules(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    /// Sets an image drawn in the middle, such as a logo; the error level
    /// is then [`ErrorLevel::High`].
    #[must_use]
    pub fn center_image(mut self, image: impl Into<image::Handle>) -> Self {
        self.center_image = Some(image.into());
        self
    }

    /// Sets the share of the module area's width cleared for the center
    /// image, clamped above 0 and at most [`MAX_CENTER_IMAGE_FRACTION`];
    /// [`CENTER_IMAGE_FRACTION`] by default.
    #[must_use]
    pub fn center_image_fraction(mut self, fraction: f32) -> Self {
        self.center_image_fraction = fraction.clamp(0.01, MAX_CENTER_IMAGE_FRACTION);
        self
    }

    /// Sets the strings of the "too much data" message.
    #[must_use]
    pub fn localizations(mut self, localizations: &Localizations) -> Self {
        self.localizations = *localizations;
        self
    }

    /// Sets what shows, instead of the localized message, when the text
    /// doesn't fit in a QR code.
    #[must_use]
    pub fn error(mut self, error: impl Into<Element<'a, Message>>) -> Self {
        self.error = Some(error.into());
        self
    }

    fn into_element(self) -> Element<'a, Message> {
        let level = if self.center_image.is_some() {
            ErrorLevel::High
        } else {
            self.error_level
        };
        let size = self.size;

        let Some(modules) = cached(&self.data, level) else {
            return self.error.unwrap_or_else(|| {
                container(
                    styled_text(self.localizations.qr_too_long, typography::BODY_MEDIUM)
                        .align_x(Horizontal::Center),
                )
                .width(size)
                .height(size)
                .align_x(Horizontal::Center)
                .align_y(Vertical::Center)
                .into()
            });
        };

        let count = modules.count();
        let fraction = self.center_image_fraction;
        let code = Grid {
            modules,
            size,
            color: self.color,
            background: self.background,
            class: self.class,
            rounded: self.rounded,
            hole: if self.center_image.is_some() {
                fraction
            } else {
                0.0
            },
        };

        match self.center_image {
            None => IcedElement::new(code),
            Some(handle) => {
                let side = module_extent(size, count) * fraction * 0.86;
                stack![
                    IcedElement::new(code),
                    container(
                        image(handle)
                            .width(side)
                            .height(side)
                            .content_fit(iced::ContentFit::Contain)
                    )
                    .width(size)
                    .height(size)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
                ]
                .into()
            }
        }
    }
}

impl<'a, Message: 'a> From<QrCode<'a, Message>> for Element<'a, Message> {
    fn from(qr_code: QrCode<'a, Message>) -> Self {
        qr_code.into_element()
    }
}

/// The width of the modules of a code `size` wide with `count` modules a
/// side, without the quiet zone.
fn module_extent(size: f32, count: usize) -> f32 {
    size * count as f32 / (count + QUIET_ZONE * 2) as f32
}

/// Paints the modules inside the quiet zone.
struct Grid<'a> {
    modules: Rc<Modules>,
    size: f32,
    color: Option<Color>,
    background: Option<Color>,
    class: Class<'a>,
    rounded: bool,
    /// The share of the module area cleared in the middle, or 0.
    hole: f32,
}

impl<Message> Widget<Message, Theme, iced::Renderer> for Grid<'_> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(self.size), Length::Fixed(self.size))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &iced::Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(self.size, self.size))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut iced::Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.style(&self.class);
        let color = self.color.unwrap_or(style.cell);
        let background = self.background.unwrap_or(style.background);
        let count = self.modules.count();
        let unit = bounds.width / (count + QUIET_ZONE * 2) as f32;

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                ..renderer::Quad::default()
            },
            background,
        );

        let side = unit * count as f32 * self.hole;
        let clear = Rectangle {
            x: bounds.center_x() - side / 2.0,
            y: bounds.center_y() - side / 2.0,
            width: side,
            height: side,
        };

        for (row, cells) in self.modules.rows().enumerate() {
            for (column, _) in cells.iter().enumerate().filter(|(_, dark)| **dark) {
                let cell = Rectangle {
                    x: bounds.x + (column + QUIET_ZONE) as f32 * unit,
                    y: bounds.y + (row + QUIET_ZONE) as f32 * unit,
                    width: unit,
                    height: unit,
                };
                if self.hole > 0.0 && overlaps(clear, inset(cell, unit * 0.1)) {
                    continue;
                }
                let (cell, radius) = if self.rounded {
                    (inset(cell, unit * 0.06), unit * 0.32)
                } else {
                    // Slightly larger squares hide hairline seams between
                    // modules.
                    (inset(cell, -0.25), 0.0)
                };
                renderer.fill_quad(
                    renderer::Quad {
                        bounds: cell,
                        border: Border::default().rounded(radius),
                        ..renderer::Quad::default()
                    },
                    color,
                );
            }
        }
    }
}

/// `rectangle` moved in by `amount` on every side (out when negative).
fn inset(rectangle: Rectangle, amount: f32) -> Rectangle {
    Rectangle {
        x: rectangle.x + amount,
        y: rectangle.y + amount,
        width: rectangle.width - amount * 2.0,
        height: rectangle.height - amount * 2.0,
    }
}

/// Whether two rectangles share some area.
fn overlaps(a: Rectangle, b: Rectangle) -> bool {
    a.x < b.x + b.width && b.x < a.x + a.width && a.y < b.y + b.height && b.y < a.y + a.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn picks_the_mode_of_the_whole_text() {
        assert_eq!(Mode::of("0123"), Mode::Numeric);
        assert_eq!(Mode::of("HELLO WORLD"), Mode::Alphanumeric);
        assert_eq!(Mode::of("Hello"), Mode::Byte { utf8_eci: false });
        assert_eq!(Mode::of("ação"), Mode::Byte { utf8_eci: false });
        assert_eq!(Mode::of("こんにちは"), Mode::Byte { utf8_eci: true });
        assert_eq!(Mode::of(""), Mode::Byte { utf8_eci: false });
    }

    #[test]
    fn grows_with_the_level() {
        let low = modules("https://oito2.github.io/oito2_ui/", ErrorLevel::Low);
        let high = modules("https://oito2.github.io/oito2_ui/", ErrorLevel::High);
        assert_eq!(low.as_ref().map(Modules::count), Ok(29));
        assert!(high.map(|m| m.count()).unwrap_or(0) > 29);
        assert!(modules(&"x".repeat(8000), ErrorLevel::Medium).is_err());
    }

    #[test]
    fn the_cache_reuses_grids() {
        let first = cached("oito2", ErrorLevel::Medium);
        let second = cached("oito2", ErrorLevel::Medium);
        assert!(matches!((first, second), (Some(a), Some(b)) if Rc::ptr_eq(&a, &b)));
        assert!(cached(&"x".repeat(8000), ErrorLevel::Medium).is_none());
    }
}
