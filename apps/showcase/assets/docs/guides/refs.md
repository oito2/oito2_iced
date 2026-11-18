# Refs

Some tasks need a handle on a component: moving the focus to it, finding where it is on screen, or changing its state from elsewhere. Iced does this with widget ids and tasks, and oito2_iced follows the same pattern.

## Widget ids

Components that take the keyboard focus accept an `id`. An id is any string, or an `iced::widget::Id`; keep it in a constant so the view and `update` agree.

```rust
use oito2_iced::widget::search;

const SEARCH: &str = "search";

let field = search::search_field(&l10n, &self.query)
    .id(SEARCH)
    .on_input(Message::Query);
```

## Moving the focus

Focus is a task: return `iced::widget::operation::focus(id)` from `update`, for example from a keyboard shortcut. It reaches text inputs and every OITO2 control, since they all implement Iced's focusable operation.

```rust
use iced::Task;
use iced::widget::operation;

fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::FocusSearch => operation::focus(SEARCH),
        Message::Query(query) => {
            self.query = query;
            Task::none()
        }
    }
}
```

`operation::focus_next()` and `focus_previous()` move the focus like Tab and Shift+Tab; `oito2_iced::focus::operations` adds `focus_first`, `focus_last` and `has_focus`.

## Ids to find widgets

An id also names an element so another component can find it on screen. The table of contents uses the ids of the section headings to know which one is in view, and the tour uses them to spotlight the element each step explains.

```rust
use iced::widget::container;
use oito2_iced::widget::button;
use oito2_iced::widget::tour::{Step, Tour};

let tour = Tour::new(vec![
    Step::new("Create a project")
        .description("Start here to set up your first project.")
        .target("new-project"),
]);

let new_project = container(button::filled("New").on_press(Message::Create)).id("new-project");
```

`table_of_contents::scroll_to(scrollable_id, target_id)` returns the task that scrolls a scrollable to the element with an id.

## Component state

There is no handle to a component's internal state: what other parts of the app need to change lives in the model. To show a toast from anywhere, push it to the `Toasts` queue the app keeps; to start a tour, call `start` on the `Tour` in the model; the next `view` shows the change.

```rust
use oito2_iced::widget::toast::Toast;

self.toasts.push(Toast::new("Saved"));
self.tour.start();
```

See **Component State** for the structs that hold richer state, such as `SelectionController` and `TreeState`.
