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

//! Localized strings in English and Brazilian Portuguese.

use std::ops::Deref;

use crate::date::{Date, Time};

/// Declares a table of strings with one `&'static str` field per key and the
/// English and Portuguese values of every field.
macro_rules! string_table {
    (
        $(#[$meta:meta])*
        struct $name:ident, en: $en:ident, pt: $pt:ident;
        $( $(#[$fmeta:meta])* $field:ident = $e:literal | $p:literal; )*
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub struct $name {
            $( $(#[$fmeta])* pub $field: &'static str, )*
        }

        static $en: $name = $name { $( $field: $e, )* };
        static $pt: $name = $name { $( $field: $p, )* };

        impl $name {
            /// The keys of the table, in snake_case, in declaration order.
            pub const KEYS: &'static [&'static str] = &[ $( stringify!($field), )* ];

            /// Returns the value of the field named `key` (snake_case), or
            /// `None` when the table has no such field.
            #[must_use]
            pub fn get(&self, key: &str) -> Option<&'static str> {
                $( if key == stringify!($field) { return Some(self.$field); } )*
                None
            }
        }
    };
}

string_table! {
/// Strings of the Material widgets that the OITO2 widgets reuse, such as
/// the date and time pickers.
struct MaterialStrings, en: MATERIAL_EN, pt: MATERIAL_PT_BR;
/// The Material "back" string.
back = "Back" | "Voltar";
/// The Material "searchField" string.
search_field = "Search" | "Pesquisa";
/// The Material "previousPage" string.
previous_page = "Previous page" | "Página anterior";
/// The Material "nextPage" string.
next_page = "Next page" | "Próxima página";
/// The Material "clearText" string.
clear_text = "Clear text" | "Limpar texto";
/// The Material "copy" string.
copy = "Copy" | "Copiar";
/// The Material "ok" string.
ok = "OK" | "OK";
/// The Material "cancel" string.
cancel = "Cancel" | "Cancelar";
/// The Material "dateInput" string.
date_input = "Enter Date" | "Inserir data";
/// The Material "dateHelp" string.
date_help = "mm/dd/yyyy" | "dd/mm/aaaa";
/// The Material "invalidDateFormat" string.
invalid_date_format = "Invalid format." | "Formato inválido.";
/// The Material "dateOutOfRange" string.
date_out_of_range = "Out of range." | "Fora de alcance.";
/// The Material "timeInput" string.
time_input = "Enter time" | "Insira o horário";
/// The Material "invalidTime" string.
invalid_time = "Enter a valid time" | "Insira um horário válido";
/// The Material "am" string.
am = "AM" | "AM";
/// The Material "pm" string.
pm = "PM" | "PM";
/// The Material "previousMonth" string.
previous_month = "Previous month" | "Mês anterior";
/// The Material "nextMonth" string.
next_month = "Next month" | "Próximo mês";
/// The Material "today" string.
today = "Today" | "Hoje";
/// The Material "selectDate" string.
select_date = "Select date" | "Selecione a data";
/// The Material "selectTime" string.
select_time = "Select time" | "Selecione o horário";
}

string_table! {
/// The plain strings of the OITO2 widgets: window controls, handles,
/// the command palette, form fields, pickers and file uploads.
struct Strings, en: STRINGS_EN, pt: STRINGS_PT_BR;
/// Label of the window button that minimizes the window.
minimize_window = "Minimize" | "Minimizar";
/// Label of the window button that maximizes the window.
maximize_window = "Maximize" | "Maximizar";
/// Label of the window button that restores a maximized window.
restore_window = "Restore" | "Restaurar";
/// Label of the window button that closes the window.
close_window = "Close" | "Fechar";
/// Label of the handle that resizes a side pane.
resize_pane = "Resize pane" | "Redimensionar painel";
/// Label of the arrow that opens a split button's menu.
more_options = "More options" | "Mais opções";
/// Tooltip of the arrow that scrolls a chip bar back.
scroll_back = "Scroll back" | "Rolar para trás";
/// Tooltip of the arrow that scrolls a chip bar forward.
scroll_forward = "Scroll forward" | "Rolar para frente";
/// A time so close to now that it reads as "now".
just_now = "now" | "agora";
/// The hint of the command palette search field.
command_palette_hint = "Type a command" | "Digite um comando";
/// Shown when no command matches the command palette search.
no_commands_found = "No commands found" | "Nenhum comando encontrado";
/// Shown by a shortcut recorder while it waits for keys.
press_shortcut = "Press a shortcut" | "Pressione um atalho";
/// Shown by a shortcut recorder without a shortcut.
no_shortcut = "No shortcut" | "Sem atalho";
/// Label of the button that clears a recorded shortcut.
clear_shortcut = "Clear shortcut" | "Limpar atalho";
/// Announced when a search or autocomplete finds nothing.
no_results = "No results" | "Nenhum resultado";
/// The label of a button that closes a panel, sheet or card.
close = "Close" | "Fechar";
/// The label of a button that dismisses a notification.
dismiss = "Dismiss" | "Dispensar";
/// The accessibility label of the notification area.
notifications = "Notifications" | "Notificações";
/// The accessibility label of a sheet's resize handle.
resize_sheet = "Resize sheet" | "Redimensionar painel";
/// The button that returns to the previous step of a tour.
tour_back = "Back" | "Voltar";
/// The button that moves to the next step of a tour.
tour_next = "Next" | "Próximo";
/// The button that finishes a tour on its last step.
tour_done = "Done" | "Concluir";
/// The button that ends a tour before its last step.
tour_skip = "Skip tour" | "Pular tour";
/// The button that shows a hidden password.
show_password = "Show password" | "Mostrar senha";
/// The button that hides a shown password.
hide_password = "Hide password" | "Ocultar senha";
/// Warns that Caps Lock is on while typing a password.
caps_lock_on = "Caps Lock is on" | "Caps Lock está ativado";
/// The button that raises a number.
increase = "Increase" | "Aumentar";
/// The button that lowers a number.
decrease = "Decrease" | "Diminuir";
/// The accessibility hint of a label that can be edited.
edit_text = "Press Enter to edit" | "Pressione Enter para editar";
/// The hue of a color.
hue = "Hue" | "Matiz";
/// The saturation of a color.
saturation = "Saturation" | "Saturação";
/// The brightness (HSV value) of a color.
brightness = "Brightness" | "Brilho";
/// The lightness of a color.
lightness = "Lightness" | "Luminosidade";
/// The opacity of a color.
opacity = "Opacity" | "Opacidade";
/// The red channel of a color.
red = "Red" | "Vermelho";
/// The green channel of a color.
green = "Green" | "Verde";
/// The blue channel of a color.
blue = "Blue" | "Azul";
/// The saturation and brightness area of a color picker.
saturation_and_brightness = "Saturation and brightness" | "Saturação e brilho";
/// The heading of the week number column of a calendar.
week_number_heading = "Wk" | "Sem";
/// Announced after a copy to the clipboard.
copied = "Copied" | "Copiado";
/// Announced when the platform refuses a copy to the clipboard.
copy_failed = "Couldn't copy" | "Não foi possível copiar";
/// The hint of a widget the arrow keys move.
move_with_arrow_keys = "Use the arrow keys to move it" | "Use as setas do teclado para mover";
/// The tooltip of the button that opens an app's main menu.
main_menu = "Main menu" | "Menu principal";
/// The accessibility action that moves a widget up.
move_up = "Move up" | "Mover para cima";
/// The accessibility action that moves a widget down.
move_down = "Move down" | "Mover para baixo";
/// The accessibility action that moves a widget left.
move_left = "Move left" | "Mover para a esquerda";
/// The accessibility action that moves a widget right.
move_right = "Move right" | "Mover para a direita";
/// The label of a page size selector.
items_per_page = "Items per page" | "Itens por página";
/// The state of a finished step.
step_completed = "Completed" | "Concluído";
/// The state of the current step.
step_current = "Current" | "Atual";
/// The state of a step that needs attention.
step_error = "Needs attention" | "Requer atenção";
/// How to remove a focused tag, for assistive technologies.
remove_tag_hint = "Press Delete to remove" | "Pressione Delete para remover";
/// The prefix of the hexadecimal color field.
hex_color = "Hex" | "Hex";
/// Label of an information message for assistive technologies.
info_information = "Information" | "Informação";
/// Label of a success message for assistive technologies.
info_success = "Success" | "Sucesso";
/// Label of a warning message for assistive technologies.
info_warning = "Warning" | "Aviso";
/// Label of an error message for assistive technologies.
info_danger = "Error" | "Erro";
/// Label of an important message for assistive technologies.
info_important = "Important" | "Importante";
/// Announced while content loads.
loading = "Loading" | "Carregando";
/// An online presence status.
status_online = "Online" | "Online";
/// An away presence status.
status_away = "Away" | "Ausente";
/// A busy presence status.
status_busy = "Busy" | "Ocupado";
/// An offline presence status.
status_offline = "Offline" | "Offline";
/// The accessibility label of a QR code.
qr_code = "QR code" | "Código QR";
/// Shown when data doesn't fit in a QR code.
qr_too_long = "Too much data for a QR code" | "Dados demais para um código QR";
/// The accessibility label of a signature pad.
signature = "Signature" | "Assinatura";
/// The placeholder of an empty signature pad.
sign_here = "Sign here" | "Assine aqui";
/// The state of a signature pad with strokes.
signed = "Signed" | "Assinado";
/// The state of something with no content.
empty = "Empty" | "Vazio";
/// The button that undoes the last change.
undo = "Undo" | "Desfazer";
/// The button that removes all content.
clear = "Clear" | "Limpar";
/// The accessibility label of an image cropper.
image_cropper = "Image cropper" | "Recorte de imagem";
/// The button that turns an image a quarter left.
rotate_left = "Rotate left" | "Girar à esquerda";
/// The button that turns an image a quarter right.
rotate_right = "Rotate right" | "Girar à direita";
/// The button that mirrors an image.
flip_horizontally = "Flip horizontally" | "Espelhar horizontalmente";
/// The button that enlarges a view.
zoom_in = "Zoom in" | "Ampliar";
/// The button that shrinks a view.
zoom_out = "Zoom out" | "Reduzir";
/// The button that restores the initial state.
reset = "Reset" | "Redefinir";
/// The invitation of a file drop zone where files can be dropped.
drop_files_here = "Drop files here" | "Solte os arquivos aqui";
/// The button that opens the file chooser for several files.
choose_files = "Choose files" | "Escolher arquivos";
/// The button that opens the file chooser for one file.
choose_file = "Choose a file" | "Escolher um arquivo";
/// The button that saves a file to the device.
download = "Download" | "Baixar";
/// Why a download button is disabled on platforms that can't save files.
download_unavailable = "Downloads are not available on this device" | "Downloads não estão disponíveis neste dispositivo";}

/// The units of a relative time, from [`TimeUnit::Second`] to
/// [`TimeUnit::Year`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TimeUnit {
    /// Seconds.
    Second,
    /// Minutes.
    Minute,
    /// Hours.
    Hour,
    /// Days.
    Day,
    /// Weeks.
    Week,
    /// Months.
    Month,
    /// Years.
    Year,
}

impl TimeUnit {
    /// Returns the unit named `name` in lowercase English, such as
    /// `"minute"`, or `None` for an unknown name.
    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        Some(match name {
            "second" => Self::Second,
            "minute" => Self::Minute,
            "hour" => Self::Hour,
            "day" => Self::Day,
            "week" => Self::Week,
            "month" => Self::Month,
            "year" => Self::Year,
            _ => return None,
        })
    }

    /// The singular and plural English names of the unit.
    fn english(self) -> (&'static str, &'static str) {
        match self {
            Self::Second => ("second", "seconds"),
            Self::Minute => ("minute", "minutes"),
            Self::Hour => ("hour", "hours"),
            Self::Day => ("day", "days"),
            Self::Week => ("week", "weeks"),
            Self::Month => ("month", "months"),
            Self::Year => ("year", "years"),
        }
    }

    /// The singular and plural Portuguese names of the unit.
    fn portuguese(self) -> (&'static str, &'static str) {
        match self {
            Self::Second => ("segundo", "segundos"),
            Self::Minute => ("minuto", "minutos"),
            Self::Hour => ("hora", "horas"),
            Self::Day => ("dia", "dias"),
            Self::Week => ("semana", "semanas"),
            Self::Month => ("mês", "meses"),
            Self::Year => ("ano", "anos"),
        }
    }
}

/// The languages with translated strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Language {
    English,
    Portuguese,
}

/// Localized strings of the OITO2 widgets in English or Brazilian
/// Portuguese.
///
/// The plain strings are fields reached through `Deref` (for example
/// `localizations.close_window`); the messages that take arguments are
/// methods. English is the fallback of [`Localizations::for_locale`].
#[derive(Debug, Clone, Copy)]
pub struct Localizations {
    language: Language,
    /// The locale the strings are written for: `en` or `pt_BR`.
    pub locale_name: &'static str,
    /// The strings of the Material widgets.
    pub material: &'static MaterialStrings,
    strings: &'static Strings,
}

impl Default for Localizations {
    fn default() -> Self {
        Self::en()
    }
}

impl Deref for Localizations {
    type Target = Strings;

    fn deref(&self) -> &Strings {
        self.strings
    }
}

impl Localizations {
    /// The locales with translated strings: English and Brazilian
    /// Portuguese.
    pub const SUPPORTED_LOCALES: [&'static str; 2] = ["en", "pt_BR"];

    /// Returns the English strings.
    #[must_use]
    pub fn en() -> Self {
        Self {
            language: Language::English,
            locale_name: "en",
            material: &MATERIAL_EN,
            strings: &STRINGS_EN,
        }
    }

    /// Returns the Brazilian Portuguese strings.
    #[must_use]
    pub fn pt_br() -> Self {
        Self {
            language: Language::Portuguese,
            locale_name: "pt_BR",
            material: &MATERIAL_PT_BR,
            strings: &STRINGS_PT_BR,
        }
    }

    /// Returns the strings for a language tag such as `pt-BR`, `pt_BR` or
    /// `pt`: Portuguese for any `pt` language, English otherwise.
    #[must_use]
    pub fn for_locale(tag: &str) -> Self {
        let language = tag.split(['-', '_']).next().unwrap_or_default();

        if language.eq_ignore_ascii_case("pt") {
            Self::pt_br()
        } else {
            Self::en()
        }
    }

    /// Whether `count` takes the singular form.
    fn singular(count: i64) -> bool {
        count == 1
    }

    /// Returns `count` followed by the singular or plural name of `unit`.
    fn unit(&self, count: i64, unit: TimeUnit) -> String {
        let (one, many) = match self.language {
            Language::English => unit.english(),
            Language::Portuguese => unit.portuguese(),
        };

        if Self::singular(count) {
            format!("1 {one}")
        } else {
            format!("{count} {many}")
        }
    }

    /// Picks the English or the Portuguese text.
    fn pick(&self, en: String, pt: String) -> String {
        match self.language {
            Language::English => en,
            Language::Portuguese => pt,
        }
    }

    /// Label of a page indicator dot: page `page` of `count`, counted from 1.
    #[must_use]
    pub fn page_of(&self, page: i64, count: i64) -> String {
        self.pick(
            format!("Page {page} of {count}"),
            format!("Página {page} de {count}"),
        )
    }

    /// Label of a segment of a segmented entry: part `part` of `count`,
    /// counted from 1.
    #[must_use]
    pub fn part_of(&self, part: i64, count: i64) -> String {
        self.pick(
            format!("Part {part} of {count}"),
            format!("Parte {part} de {count}"),
        )
    }

    /// Returns a time `count` `unit`s in the past, such as "3 minutes ago".
    #[must_use]
    pub fn time_ago(&self, count: i64, unit: TimeUnit) -> String {
        let text = self.unit(count, unit);
        self.pick(format!("{text} ago"), format!("há {text}"))
    }

    /// Returns a time `count` `unit`s in the future, such as "in 3 minutes".
    #[must_use]
    pub fn time_from_now(&self, count: i64, unit: TimeUnit) -> String {
        let text = self.unit(count, unit);
        self.pick(format!("in {text}"), format!("em {text}"))
    }

    /// Announced with the number of search or autocomplete results.
    #[must_use]
    pub fn result_count(&self, count: i64) -> String {
        if Self::singular(count) {
            self.pick("1 result".into(), "1 resultado".into())
        } else {
            self.pick(format!("{count} results"), format!("{count} resultados"))
        }
    }

    /// The position of a tour step, such as "Step 2 of 5".
    #[must_use]
    pub fn tour_progress(&self, step: i64, total: i64) -> String {
        self.pick(
            format!("Step {step} of {total}"),
            format!("Passo {step} de {total}"),
        )
    }

    /// The accessibility label of a code entry with `length` characters.
    #[must_use]
    pub fn code_entry(&self, length: i64) -> String {
        self.pick(
            format!("Code of {length} characters"),
            format!("Código de {length} caracteres"),
        )
    }

    /// The accessibility value of a rating, such as "3.5 of 5 stars".
    #[must_use]
    pub fn rating_value(&self, value: &str, max: i64) -> String {
        self.pick(
            format!("{value} of {max} stars"),
            format!("{value} de {max} estrelas"),
        )
    }

    /// The button that removes `label` from a list, such as a tag.
    #[must_use]
    pub fn remove_item(&self, label: &str) -> String {
        self.pick(format!("Remove {label}"), format!("Remover {label}"))
    }

    /// The accessibility label of a week number.
    #[must_use]
    pub fn week_number(&self, week: i64) -> String {
        self.pick(format!("Week {week}"), format!("Semana {week}"))
    }

    /// The label of a page button, such as "Page 3".
    #[must_use]
    pub fn page_number(&self, page: i64) -> String {
        self.pick(format!("Page {page}"), format!("Página {page}"))
    }

    /// The position of a step, such as "Step 2 of 4".
    #[must_use]
    pub fn step_of(&self, step: i64, count: i64) -> String {
        self.pick(
            format!("Step {step} of {count}"),
            format!("Passo {step} de {count}"),
        )
    }

    /// A range of two formatted dates, such as "3/1/2026 – 3/7/2026".
    #[must_use]
    pub fn date_range(&self, start: &str, end: &str) -> String {
        format!("{start} – {end}")
    }

    /// The label of a bubble standing for `count` hidden items.
    #[must_use]
    pub fn more_count(&self, count: i64) -> String {
        self.pick(format!("{count} more"), format!("Mais {count}"))
    }

    /// The file types a file upload accepts, listed in `types`.
    #[must_use]
    pub fn accepted_file_types(&self, types: &str) -> String {
        self.pick(format!("Accepted: {types}"), format!("Aceitos: {types}"))
    }

    /// The largest file a file upload accepts, as a formatted `size`.
    #[must_use]
    pub fn max_file_size_hint(&self, size: &str) -> String {
        self.pick(format!("Up to {size} each"), format!("Até {size} cada"))
    }

    /// Why the file `name` was refused: its type isn't accepted.
    #[must_use]
    pub fn file_type_not_accepted(&self, name: &str) -> String {
        self.pick(
            format!("{name}: this file type is not accepted"),
            format!("{name}: este tipo de arquivo não é aceito"),
        )
    }

    /// Why the file `name` was refused: it is larger than `size`.
    #[must_use]
    pub fn file_too_large(&self, name: &str, size: &str) -> String {
        self.pick(
            format!("{name}: larger than {size}"),
            format!("{name}: maior que {size}"),
        )
    }

    /// Why the file `name` was refused: the upload already holds `max` files.
    #[must_use]
    pub fn too_many_files(&self, name: &str, max: i64) -> String {
        if Self::singular(max) {
            self.pick(
                format!("{name}: only 1 file can be added"),
                format!("{name}: só é possível adicionar 1 arquivo"),
            )
        } else {
            self.pick(
                format!("{name}: only {max} files can be added"),
                format!("{name}: só é possível adicionar {max} arquivos"),
            )
        }
    }

    /// Why `name` was refused: it is a folder, not a file.
    #[must_use]
    pub fn folder_not_accepted(&self, name: &str) -> String {
        self.pick(
            format!("{name}: folders are not accepted"),
            format!("{name}: pastas não são aceitas"),
        )
    }
}

/// English month names, January first.
const MONTHS_EN: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

/// Portuguese month names, January first.
const MONTHS_PT: [&str; 12] = [
    "janeiro",
    "fevereiro",
    "março",
    "abril",
    "maio",
    "junho",
    "julho",
    "agosto",
    "setembro",
    "outubro",
    "novembro",
    "dezembro",
];

/// Calendar data and date and time formats of the date and time widgets.
impl Localizations {
    /// The first day of a calendar week, from 0 (Sunday) to 6 (Saturday).
    #[must_use]
    pub fn first_day_of_week(&self) -> u32 {
        0
    }

    /// One-letter weekday names, Sunday first.
    #[must_use]
    pub fn narrow_weekdays(&self) -> [&'static str; 7] {
        match self.language {
            Language::English => ["S", "M", "T", "W", "T", "F", "S"],
            Language::Portuguese => ["D", "S", "T", "Q", "Q", "S", "S"],
        }
    }

    /// The name of `month` (1 to 12): "November" in English, "novembro" in
    /// Portuguese. Out-of-range months are clamped.
    #[must_use]
    pub fn month_name(&self, month: u32) -> &'static str {
        let index = usize::try_from(month.clamp(1, 12) - 1).unwrap_or(0);

        match self.language {
            Language::English => MONTHS_EN[index],
            Language::Portuguese => MONTHS_PT[index],
        }
    }

    /// The month and year of `date`, as a calendar heading: "November 2026"
    /// or "novembro de 2026".
    #[must_use]
    pub fn month_year(&self, date: Date) -> String {
        let month = self.month_name(date.month());

        self.pick(
            format!("{month} {}", date.year()),
            format!("{month} de {}", date.year()),
        )
    }

    /// `date` in the compact numeric format: `mm/dd/yyyy` in English,
    /// `dd/mm/yyyy` in Portuguese.
    #[must_use]
    pub fn compact_date(&self, date: Date) -> String {
        let (year, month, day) = (date.year(), date.month(), date.day());

        self.pick(
            format!("{month:02}/{day:02}/{year:04}"),
            format!("{day:02}/{month:02}/{year:04}"),
        )
    }

    /// Parses a date in the compact numeric format of
    /// [`compact_date`](Self::compact_date): three groups of digits split by
    /// `/`, in the language's order, without surrounding spaces. Returns
    /// `None` for any other text, a day that does not exist or a year
    /// outside [`Date::MIN`]..=[`Date::MAX`].
    #[must_use]
    pub fn parse_compact_date(&self, text: &str) -> Option<Date> {
        let (year, month, day) = self.compact_parts(text)?;

        Date::new(year, month, day)
    }

    /// Parses a date as a date entry does: the compact numeric format of
    /// [`parse_compact_date`](Self::parse_compact_date), ignoring
    /// surrounding spaces.
    ///
    /// A year written with one or two digits is taken as the year with
    /// those last digits that falls from 80 years before to 19 years after
    /// the year of `reference`, so `26` reads as 2026 in 2026 and `99` as
    /// 1999. A year written with three or more digits is used as written.
    /// Returns `None` when the text does not parse.
    #[must_use]
    pub fn parse_date(&self, text: &str, reference: Date) -> Option<Date> {
        let text = text.trim();
        let (year, month, day) = self.compact_parts(text)?;

        if year >= 100 || has_long_number(text) {
            return Date::new(year, month, day);
        }

        let reference = reference.year();
        let mut year = reference - reference % 100 + year;

        if year > reference + 19 {
            year -= 100;
        } else if year < reference - 80 {
            year += 100;
        }

        Date::new(year, month, day)
    }

    /// The year, month and day of a date in the compact numeric format,
    /// checked against the calendar with the year as written (year 0
    /// included), or `None` when the text does not parse.
    fn compact_parts(&self, text: &str) -> Option<(i32, u32, u32)> {
        let mut parts = text.split('/');
        let (first, second, year) = (parts.next()?, parts.next()?, parts.next()?);

        if parts.next().is_some() {
            return None;
        }

        let number = |part: &str| -> Option<u32> {
            if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
                return None;
            }
            part.parse().ok()
        };
        let (first, second, year) = (number(first)?, number(second)?, number(year)?);
        let (month, day) = match self.language {
            Language::English => (first, second),
            Language::Portuguese => (second, first),
        };
        let year = i32::try_from(year).ok()?;

        // Year 0 is a leap year; any other year shares its leap rule with
        // the dates `Date` supports.
        let leap_year = if year == 0 { 2000 } else { year };
        let valid = (1..=12).contains(&month)
            && day >= 1
            && Date::new(leap_year.min(9999), month, 1)
                .is_some_and(|first| day <= first.days_in_month());

        valid.then_some((year, month, day))
    }

    /// `time` as text: `h:mm AM` in English unless `use_24_hour` is set,
    /// `HH:mm` otherwise and always in Portuguese.
    #[must_use]
    pub fn time_of_day(&self, time: Time, use_24_hour: bool) -> String {
        let (hour, minute) = (time.hour(), time.minute());

        if use_24_hour || self.language == Language::Portuguese {
            return format!("{hour:02}:{minute:02}");
        }

        let marker = if hour < 12 {
            self.material.am
        } else {
            self.material.pm
        };
        let hour = match hour % 12 {
            0 => 12,
            h => h,
        };

        format!("{hour}:{minute:02} {marker}")
    }

    /// Parses a time typed in a time entry: an hour of one or two digits, a
    /// separator (`:`, `.`, `h` or `H`), two minute digits and an optional
    /// AM or PM marker before or after (the language's markers, or `a`,
    /// `am`, `p`, `pm`, in any case). Without a marker the hour is 0 to 23;
    /// with one it is 1 to 12. Returns `None` for any other text.
    #[must_use]
    pub fn parse_time(&self, text: &str) -> Option<Time> {
        let text = text.trim();
        let digits_at = text.find(|c: char| c.is_ascii_digit())?;
        let prefix = text[..digits_at].trim_end().to_lowercase();
        let rest = &text[digits_at..];

        let hour_len = rest.bytes().take_while(u8::is_ascii_digit).count();
        if hour_len > 2 {
            return None;
        }
        let hour: u32 = rest[..hour_len].parse().ok()?;

        let rest = rest[hour_len..].trim_start();
        let rest = rest.strip_prefix([':', '.', 'h', 'H'])?.trim_start();

        let minute_digits = rest.get(..2)?;
        if !minute_digits.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let minute: u32 = minute_digits.parse().ok()?;

        let suffix = rest[2..].trim_start();
        if suffix.chars().any(char::is_whitespace) {
            return None;
        }
        let suffix = suffix.to_lowercase();

        if minute > 59 || (!prefix.is_empty() && !suffix.is_empty()) {
            return None;
        }

        let marker = if suffix.is_empty() { prefix } else { suffix };
        if marker.is_empty() {
            return Time::new(hour, minute);
        }

        let am = self.material.am.to_lowercase();
        let pm = self.material.pm.to_lowercase();
        let is_am = marker == am || marker == "a" || marker == "am";
        let is_pm = marker == pm || marker == "p" || marker == "pm";

        if !is_am && !is_pm || !(1..=12).contains(&hour) {
            return None;
        }

        Time::new(hour % 12 + if is_pm { 12 } else { 0 }, minute)
    }
}

/// Whether `text` holds a run of three or more ASCII digits.
fn has_long_number(text: &str) -> bool {
    text.split(|c: char| !c.is_ascii_digit())
        .any(|run| run.len() >= 3)
}
