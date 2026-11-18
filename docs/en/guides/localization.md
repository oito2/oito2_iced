🌐 [Português](../../pt-br/guides/localization.md) | **English** | 🏠 [Index](../index.md)

---

# Localization

How OITO2 widgets get their texts, numbers and dates in English and Brazilian Portuguese, and how they mirror for right-to-left languages.

## The strings

`oito2_iced::l10n::Localizations` holds every text the widgets show — tooltips of window buttons, "Copied", "Clear", calendar labels, file upload messages — with the same keys as oito2_ui, checked against its `l10n.json` fixture.

| Constructor | Language |
|---|---|
| `Localizations::en()` | English, also the fallback |
| `Localizations::pt_br()` | Brazilian Portuguese |
| `Localizations::for_locale(tag)` | Portuguese for any `pt` tag (`pt-BR`, `pt_BR`, `pt`), English otherwise |

oito2_iced doesn't detect the system language: the app decides, for example from its own settings or from the `LANG` environment variable, and passes the result to the widgets that show text.

```rust
use oito2_iced::l10n::Localizations;
use oito2_iced::widget::{button, password};

let l10n = Localizations::for_locale("pt-BR");

let close = button::close_button(&l10n);
let field = password::password_field(&l10n, &self.password).on_input(Message::Password);
```

Widgets that take a `&Localizations` include the close and back buttons, the search, password, number, PIN and tags fields, the date and time entries, the calendar, the color picker, the copy button and field, toasts, sheets, the command palette, pagination, steps, the tree view, the carousel, the tour host, the window control labels (`window::Labels::new`), the code block, and the extras and file widgets. The strings are also fields of `Localizations` (through `Deref`), such as `l10n.minimize_window`, for texts of your own that should match.

## Numbers, sizes and relative times

`oito2_iced::format::Format` formats for a language tag, with the rules of oito2_ui (checked against `format.json`):

```rust
use std::time::{Duration, SystemTime};
use oito2_iced::format::{ByteUnits, Format};

let format = Format::new("pt-BR");
let size = format.bytes(1_536_000, ByteUnits::Si, 1);
let count = format.compact_number(12_400.0);
let now = SystemTime::now();
let when = format.relative_time(now - Duration::from_secs(300), now);
```

## Dates and times

`oito2_iced::date::{Date, Time}` are plain values without time zone; convert them from the date library your app uses. `Localizations` formats and parses them in the compact form of each language (`mm/dd/yyyy` in English, `dd/mm/aaaa` in Portuguese):

| Method | Does |
|---|---|
| `compact_date(date)`, `parse_compact_date(text)` | Compact date to and from text |
| `parse_date(text, reference)` | Like `parse_compact_date`, expanding one- or two-digit years around `reference` |
| `time_of_day(time, use_24_hour)`, `parse_time(text)` | `h:mm AM` or `HH:mm`, and the formats a time entry accepts |
| `month_name`, `month_year`, `narrow_weekdays`, `first_day_of_week` | Calendar labels |

The date and time entries, the calendar and the date picker use them; "today" always comes from the app.

## Reading direction

Text in right-to-left scripts is shaped by Iced's text stack. Layout mirroring is explicit: widgets with a direction take `.direction(ReadingDirection::RightToLeft)` (from `oito2_iced::widget::helpers`), which flips chevrons, arrows, sides and keyboard arrows. `overlay::Placement::Start` and `End` follow the same direction.

```rust
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::rating;

let stars = rating::rating(self.score)
    .direction(ReadingDirection::RightToLeft)
    .on_change(Message::Score);
```

## Limits

- Only English and Brazilian Portuguese are bundled; there is no way to register another language yet.
- Number formats in the number entry follow English and Portuguese only.
- There is no plural or grammar engine beyond the functions of `Localizations` (`time_ago`, `result_count`, `page_of`…).
