# Component State

Every interactive component follows the Iced pattern for state: the app owns the value in its model, the component shows it, and a user action becomes a message that `update` applies. Components with more state than a value work with a plain struct the app keeps in its model.

## Controlled values

Most components take the current value and a function that turns the next value into a message. The component doesn't change on its own: update the value in `update` and the next `view` shows it.

```rust
use oito2_iced::widget::toggles;

#[derive(Debug, Clone)]
enum Message {
    Enabled(bool),
}

struct App {
    enabled: bool,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Enabled(enabled) => self.enabled = enabled,
        }
    }

    fn view(&self) -> oito2_iced::Element<'_, Message> {
        toggles::switch(self.enabled).on_toggle(Message::Enabled).into()
    }
}
```

| Approach | When to use it |
|---|---|
| A value and `on_change`, `on_toggle` or `on_input` | Simple values: switches, sliders, selects, entries |
| A model struct and events | Selection, trees, carousels, tours, toasts, uploads: state with several parts or methods |
| State inside the widget | Transient details the app doesn't need: focus, hover, animation progress, the text being typed in an entry |

## Disabling

A component without its message is disabled: it ignores input, looks disabled and leaves the Tab order. The `_maybe` builders take an `Option`, which is handy when the condition is computed.

```rust
use oito2_iced::widget::{button, slider};

let volume = slider::slider(self.volume)
    .on_change_maybe((!self.muted).then_some(Message::Volume));
let save = button::filled("Save").on_press_maybe(self.dirty.then_some(Message::Save));
```

## Model structs

Components with richer state read a struct that lives in the app model, such as `SelectionController`, `TreeState`, `carousel::Controller`, `tour::Tour`, `toast::Toasts` or `upload::Controller`. The component publishes events; `update` applies them to the struct.

```rust
use oito2_iced::collections::{ListCollection, SelectionController, SelectionMode, SelectionOptions};
use oito2_iced::widget::list_box::{self, SelectionChange};

#[derive(Debug, Clone)]
enum Message {
    Selection(SelectionChange),
}

struct Fruits {
    selection: SelectionController<&'static str>,
}

impl Fruits {
    fn new() -> Self {
        let fruits = ListCollection::new(["Apple", "Banana", "Cherry"]).expect("unique values");
        let options = SelectionOptions {
            mode: SelectionMode::Multiple,
            ..SelectionOptions::default()
        };

        Self {
            selection: SelectionController::new(fruits, options),
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Selection(change) => {
                change.apply(&mut self.selection);
            }
        }
    }

    fn view(&self) -> oito2_iced::Element<'_, Message> {
        list_box::list_box(&self.selection)
            .on_change(Message::Selection)
            .into()
    }
}
```

## Reading the state

The model is the single source of truth, so any part of the view can read it directly; there is nothing to subscribe to.

```rust
use iced::widget::text;

let count = text(format!("{} selected", self.selection.selected_values().len()));
```
