# Forms

oito2_iced fields report a value and, when the text doesn't parse, an error. A form is plain app state: keep the value and the error of each field in the model, show the errors under the fields, and check them when the form is submitted.

## Text fields

Use `text_field` for free text: it adds a label above the input, a helper or error text below it, a leading icon and a trailing element, with the OITO2 height, border and focus outline. For specific values, the OITO2 entries parse and validate the text for you:

| Field | Value |
|---|---|
| `number_entry` | A number within a range, with steps, arrow keys and the mouse wheel |
| `entries::date_entry`, `time_entry`, `date_time_entry` | Dates and times in the locale format, with a calendar or time popover |
| `password::password_field` | A password with a show button and a Caps Lock warning |
| `pin_entry` | A code of single characters, such as a verification code |
| `entries::segmented_entry` | Joined segments, such as an IP address |
| `tags_field` | A list of tags, with suggestions and validation |

## Values and errors

Entries publish `on_change` only with valid values, and `on_error_change` with the error message when the text stops parsing, or `None` when it parses again. Errors show under the field once it loses the focus or is submitted with Enter, so partial input isn't flagged while typing.

```rust
use oito2_iced::date::Date;
use oito2_iced::widget::entries;

let due = entries::date_entry(self.today, self.due)
    .label("Due date")
    .limits(Date::new(2020, 1, 1).expect("valid date"), Date::new(2030, 12, 31).expect("valid date"))
    .on_change(Message::Due)
    .on_error_change(Message::DueError)
    .open(self.calendar_open)
    .on_open_change(Message::CalendarOpen);
```

`today` comes from the app: oito2_iced has no clock or time zone of its own.

## Validating a form

Keep the latest value and error of each field in the model, validate free text in `update`, and enable the submit button only when everything is valid:

```rust
use iced::widget::column;
use oito2_iced::widget::{button, text_field};

fn view(&self) -> oito2_iced::Element<'_, Message> {
    let name_error = self.name.trim().is_empty().then_some("Enter a name");
    let valid = name_error.is_none() && self.due.is_some() && self.due_error.is_none();

    column![
        text_field::text_field("Your name", &self.name)
            .label("Name")
            .error(name_error)
            .on_input(Message::Name),
        self.due_field(),
        button::filled("Save").on_press_maybe(valid.then_some(Message::Save)),
    ]
    .spacing(12)
    .into()
}
```

## Labels and help

Give every field a visible label with `.label(…)`, and use `.helper(…)` for a hint that stays under the field. Iced 0.14 has no accessibility tree, so the label and the error text are what every user reads: keep them short and specific.
