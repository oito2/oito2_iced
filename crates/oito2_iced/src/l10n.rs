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
select_date = "Select date" | "Selecione a data";}

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
