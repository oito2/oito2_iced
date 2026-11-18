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

//! The Format page: byte sizes, compact numbers and relative times in the
//! language of the application.

use std::time::{Duration, SystemTime};

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task};

use oito2_iced::Theme;
use oito2_iced::font;
use oito2_iced::format::{ByteUnits, Format};
use oito2_iced::tokens::spacing;

use crate::catalog::Shell;
use crate::doc::{self, Context};

/// The text of this file, for the source of the examples.
const SOURCE: &str = include_str!("format.rs");

/// The state of the page.
#[derive(Debug, Default)]
pub struct State;

/// A message of the page.
#[derive(Debug, Clone)]
pub enum Message {}

/// Applies a message.
pub fn update(_state: &mut State, message: Message, _shell: &mut Shell) -> Task<Message> {
    match message {}
}

/// A line with an input and the formatted output.
fn line<'a>(input: String, output: String) -> Element<'a, Message, Theme> {
    container(
        row![
            text(input).width(Length::Fill),
            text(output).font(font::inter(iced::font::Weight::Semibold)),
        ]
        .spacing(spacing::MD),
    )
    .width(Length::Fixed(360.0))
    .into()
}

/// The page.
pub fn view<'a>(_state: &'a State, ctx: &Context) -> Element<'a, Message, Theme> {
    // #region usage
    let format = Format::new(ctx.l10n.locale_name);
    // #endregion
    let now = SystemTime::now();

    let preview = doc::example(
        ctx,
        SOURCE,
        &["usage", "preview"],
        // #region preview
        column![
            text(format.bytes(1536, ByteUnits::Si, 1)),
            text(format.compact_number(1234.0)),
            text(format.relative_time(now - Duration::from_secs(180), now)),
        ]
        .spacing(spacing::XS),
        // #endregion
    );

    let bytes = doc::example(
        ctx,
        SOURCE,
        &["bytes"],
        // #region bytes
        column([512, 1536, 2_500_000, 7_340_032_000].map(|bytes| {
            line(
                format!("{bytes} bytes"),
                format.bytes(bytes, ByteUnits::Si, 1),
            )
        })),
        // #endregion
    );

    let iec = doc::example(
        ctx,
        SOURCE,
        &["iec"],
        // #region iec
        column![
            line(
                "1536 bytes, IEC units".to_owned(),
                format.bytes(1536, ByteUnits::Iec, 1),
            ),
            line(
                "1234567 bytes, 2 decimals".to_owned(),
                format.bytes(1_234_567, ByteUnits::Si, 2),
            ),
        ],
        // #endregion
    );

    let compact = doc::example(
        ctx,
        SOURCE,
        &["compact"],
        // #region compact
        column(
            [999.0, 1234.0, 56789.0, 1_250_000.0]
                .map(|value| line(format!("{value}"), format.compact_number(value))),
        ),
        // #endregion
    );

    let relative = doc::example(
        ctx,
        SOURCE,
        &["relative"],
        // #region relative
        column(
            [-20, -180, 5 * 3600, -2 * 86_400, 40 * 86_400, -800 * 86_400].map(|seconds| {
                line(
                    format!("{seconds} seconds"),
                    format.relative_seconds(seconds),
                )
            }),
        ),
        // #endregion
    );

    let portuguese = doc::example(
        ctx,
        SOURCE,
        &["portuguese"],
        // #region portuguese
        column![
            line(
                "1536 bytes".to_owned(),
                Format::new("pt-BR").bytes(1536, ByteUnits::Si, 1),
            ),
            line(
                "1234".to_owned(),
                Format::new("pt-BR").compact_number(1234.0),
            ),
        ],
        // #endregion
    );

    doc::page(
        "Format",
        "Formats byte sizes, compact numbers and relative times in the language of the \
         application.",
        [
            preview,
            doc::heading("Usage"),
            doc::prose(
                "`Format::new` creates a formatter for a language tag such as `en`, `pt-BR` or \
                 `pt_BR`, falling back to English for unknown locales; pass the `locale_name` \
                 of the `Localizations` the application shows. Numbers follow the locale \
                 separators and relative times use the localized strings, so switching the \
                 language switches every output.",
            ),
            doc::code(ctx, "use oito2_iced::format::{ByteUnits, Format};"),
            doc::note(
                "`Format` covers sizes, compact numbers and relative times only; dates, times \
                 and plain numbers belong to the widgets and crates that show them.",
            ),
            doc::heading("Examples"),
            doc::subheading("Byte sizes"),
            doc::prose(
                "`bytes` picks the largest unit, up to petabytes, that keeps the value at least \
                 1, with at most the decimals given; counts below one kilobyte keep no \
                 decimals. `ByteUnits::Si` uses powers of 1000.",
            ),
            bytes,
            doc::subheading("Binary units and decimals"),
            doc::prose(
                "`ByteUnits::Iec` uses powers of 1024 and the KiB, MiB and GiB names. The last \
                 argument sets the largest number of decimals.",
            ),
            iec,
            doc::subheading("Compact numbers"),
            doc::prose(
                "`compact_number` shortens a number to three significant digits with a locale \
                 suffix, such as \"1.23K\".",
            ),
            compact,
            doc::subheading("Relative time"),
            doc::prose(
                "`relative_time` describes a `SystemTime` against `now`, such as \"3 minutes \
                 ago\" or \"in 2 days\", and `relative_seconds` an offset in seconds, negative \
                 for the past. Offsets within 45 seconds read as now; larger ones round to \
                 minutes, hours, days, weeks, months of 30 days or years.",
            ),
            relative,
            doc::subheading("Another language"),
            doc::prose(
                "A formatter for another tag formats in that locale whatever the language of \
                 the application.",
            ),
            portuguese,
            doc::api(
                ctx,
                &[
                    "oito2_iced::format::Format",
                    "oito2_iced::format::ByteUnits",
                ],
            ),
        ],
    )
}
