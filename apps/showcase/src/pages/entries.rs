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

//! The entries page: text fields and the fields built on them.

use iced::widget::{column, row, text};
use iced::{Alignment, Element, Length};

use oito2_iced::Theme;
use oito2_iced::icon::lucide;
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::text::Class as TextClass;
use oito2_iced::tokens::{spacing, typography};
use oito2_iced::widget::copy::{copy_button, copy_field};
use oito2_iced::widget::editable_label::editable_label;
use oito2_iced::widget::highlighted_text::highlighted_text;
use oito2_iced::widget::number_entry::number_entry;
use oito2_iced::widget::password::password_field;
use oito2_iced::widget::search::{search_field, search_title_field};
use oito2_iced::widget::text_field::text_field;

use super::{Gallery, Message, card, page};

/// The rows the search field filters on this page.
const RESULTS: [&str; 5] = [
    "Configurações de ação",
    "Show hidden files",
    "Network settings",
    "Quarterly report",
    "Invoices 2026",
];

/// The entries page.
pub fn view<'a>(gallery: &'a Gallery, l10n: &Localizations) -> Element<'a, Message, Theme> {
    let email_error = (!gallery.email.is_empty() && !gallery.email.contains('@'))
        .then_some("An email address has an @");

    let fields = column![
        text_field("Your name", &gallery.name)
            .label("Name")
            .helper("As shown to other people")
            .on_input(Message::Name),
        text_field("name@example.com", &gallery.email)
            .label("Email")
            .leading(lucide::MAIL)
            .error(email_error)
            .on_input(Message::Email),
        password_field(l10n, &gallery.password)
            .label("Password")
            .on_input(Message::Password),
        text_field("Read only", "Not editable").label("Disabled"),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(360.0));

    let results = column(
        RESULTS
            .iter()
            .filter(|result| {
                gallery.search.trim().is_empty()
                    || !oito2_iced::widget::highlighted_text::matches(
                        result,
                        &gallery.search,
                        &[],
                        Default::default(),
                    )
                    .is_empty()
            })
            .map(|result| highlighted_text(*result, &gallery.search).into()),
    )
    .spacing(spacing::XS);

    let search = column![
        search_field(l10n, &gallery.search)
            .on_input(Message::Search)
            .on_clear(Message::Search(String::new())),
        results,
        search_title_field(
            l10n,
            text("Documents"),
            &gallery.title_search,
            gallery.title_search_active,
            Message::TitleSearchActive,
        )
        .on_input(Message::TitleSearch),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(360.0));

    let numbers = row![
        number_entry(l10n, gallery.copies)
            .label("Copies")
            .range(1.0, 99.0)
            .on_change(Message::Copies),
        number_entry(l10n, gallery.price)
            .label("Price")
            .range(0.0, 10_000.0)
            .step(0.5)
            .decimals(2)
            .on_change(Message::Price),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(480.0));

    let labels = column![
        editable_label(&gallery.document_name)
            .style(typography::TITLE_LARGE)
            .on_submit(Message::Rename),
        text("Double click, Enter or F2 to rename; Escape restores.").class(TextClass::Variant),
    ]
    .spacing(spacing::XS)
    .width(Length::Fixed(360.0));

    let copy = column![
        row![
            text("Token: 7f3a-91c2"),
            copy_button(l10n, "7f3a-91c2").on_copied(Message::Press),
        ]
        .spacing(spacing::SM)
        .align_y(Alignment::Center),
        copy_field(l10n, "https://oito2.dev/share/7f3a")
            .label("Share link")
            .on_copied(Message::Press),
        copy_field(l10n, "secret-api-key")
            .label("API key")
            .obscure(true)
            .on_copied(Message::Press),
    ]
    .spacing(spacing::MD)
    .width(Length::Fixed(360.0));

    page(
        "Entries",
        "Fields are as tall as the buttons; the outline thickens while focused and turns to the error tone with an error.",
        [
            card("Text and password fields", fields),
            card("Search and highlighted text", search),
            card("Number entries", numbers),
            card("Editable label", labels),
            card("Copy", copy),
        ],
    )
}
