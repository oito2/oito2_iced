# Composition

oito2_iced components are built to be combined. They take elements for their parts, an anchor and a panel for the things that pop up, and collections for their data, so a few pieces cover many layouts.

## Slots

Most components take their parts as builder methods, such as `title`, `subtitle`, `leading` or `trailing`. Any element fits a slot that takes one: an icon, a text, a badge or a whole row.

```rust
use oito2_iced::icon::{self, lucide};
use oito2_iced::widget::{list_tile, toggles};

let wifi = list_tile::list_tile()
    .leading(icon::icon(lucide::WIFI))
    .title("Wi-Fi")
    .subtitle("Connected")
    .trailing(toggles::switch(enabled).on_toggle(Message::Wifi));
```

## Anchors and panels

Components that open something next to an element take that element as their anchor and the content as their panel. The app keeps whether the panel is open: a message from the anchor opens it, and the dismiss message, sent on Escape or a click outside, closes it.

```rust
use iced::widget::column;
use oito2_iced::widget::{button, popover::popover};

let filters = popover(
    button::filled("Filters").on_press(Message::ToggleFilters),
    column![
        filter_options,
        button::text("Done").on_press(Message::CloseFilters),
    ],
)
.open(self.filters_open)
.on_dismiss(Message::CloseFilters);
```

The same pattern opens menus, selects, hover cards, date pickers and color pickers. Dialogs, sheets, toasts and tours cover the whole window instead: their host wraps the root of the view.

## Collections

Lists of choices are described once as a `ListCollection` and shared by the components that show them. A collection says how to read each item's value, label, group and whether it is disabled.

```rust
use oito2_iced::collections::ListCollection;
use oito2_iced::widget::menu;

let desktops = ListCollection::with_value_of(
    [("gnome", "GNOME"), ("kde", "KDE Plasma"), ("xfce", "Xfce")],
    |item| item.0.to_owned(),
)
.expect("unique values")
.with_label_of(|item| item.1.to_owned());

let select = menu::select(&desktops, Some(self.desktop.as_str()))
    .open(self.desktop_open)
    .on_open_change(Message::DesktopOpen)
    .on_change(Message::Desktop);
```

The same collection feeds the list box (through a `SelectionController`), the autocomplete and the suggestions of the tags field; `TreeCollection` does the same for the tree view.

## Wrappers

Some components add one behavior to any child instead of drawing their own content: `focus::focusable` gives it the keyboard focus, `focus::ring` draws the focus ring around a group, `containers::watermark` puts a faint mark behind it, and `scroll_area::undershoot` shades the edges of a scroll view.

```rust
use iced::widget::scrollable;
use oito2_iced::widget::scroll_area;

let list = scroll_area::undershoot(scrollable(items));
```

## Iced widgets

The theme also styles the native Iced widgets, so `container`, `text`, `text_input`, `pick_list` or `pane_grid` sit next to OITO2 components without extra work. Use a native widget when it already does the job, and an OITO2 component when it adds behavior, keyboard focus or the desktop look. A native control doesn't take the keyboard focus; wrap it in `focus::focusable` with an `on_activate` message to make it reachable with Tab.
