🌐 [Português](../../pt-br/architecture/overlay.md) | **English** | 🏠 [Index](../index.md)

---

# Overlay

How popups open next to their anchor, and how window-wide surfaces sit over the view. Code: `crates/oito2_iced/src/overlay/anchored.rs` and the hosts in `crates/oito2_iced/src/widget/{toast,dialog,sheet,tour,command_palette}.rs`.

## Why a foundation of its own

Iced's `float` widget has no placement, no dismissal and draws below other overlays, and `pick_list` can't take the focus. oito2_iced implements `iced::advanced::overlay::Overlay` once, in `overlay::anchored`, and builds every popup on it.

## The anchored panel

`overlay::anchored(anchor, panel)` keeps the anchor in the layout and, while `open(true)`, shows the panel in an overlay:

| Concern | Behavior |
|---|---|
| Placement | `Placement::{Top, Bottom, Start, End}`; start and end follow the reading direction (`direction`) |
| Alignment | `Alignment::{Start, Center, End}` along the anchor |
| Flip | When the panel doesn't fit on the preferred side, it opens on the opposite side if that side has more room |
| Clamping | The panel moves to stay inside the window, `margin` away from its edges; `max_width` and `max_height` limit it |
| Width | `match_anchor_width` makes it as wide as the anchor; `fit_anchor_width` at least as wide |
| Arrow | `arrow(true)` draws a pointer toward the anchor, on the side the panel ended on |
| Surface | Popover by default; `.menu()` and `.tooltip()` switch to the menu and tooltip surfaces |
| Dismissal | A mouse press outside the panel and the anchor, or Escape, publishes `on_dismiss`; the app closes the panel by rendering `open(false)` |
| Nesting | A panel inside another opens above it; a press inside the inner panel doesn't dismiss the outer one |

The overlay reports an index of 2.0, above Iced's own overlays. The geometry (`Geometry::side_for`, `placed_at`) is pure and tested without a window; `overlay::place` is reused by the tour card.

Two details learned the hard way:

- Iced recreates an overlay to draw it without calling `layout` again, so the side and the arrow are derived from the bounds in `draw`.
- Iced treats the pointer as over a child overlay only when its `mouse_interaction` isn't `None`; a panel with inert content claims the pointer on its own area, so a click inside a nested panel doesn't close its parent.

## Built on it

| Widget | Module |
|---|---|
| Popover | `widget::popover` |
| Hover card | `widget::hover_card` |
| Tooltip | `widget::tooltip` |
| Menus, popup menu button, select | `widget::menu` |
| Context menu | `widget::context_menu` |
| Split button | `widget::split_button` |
| Autocomplete and tag suggestions | `widget::autocomplete`, `widget::tags_field` |
| Date and color pickers | `widget::calendar`, `widget::entries`, `widget::color_picker` |
| Navigation menu panels | `oito2_iced_extras::navigation_menu` |

## Root hosts

Surfaces that cover the window don't anchor to an element; they wrap the root of the view with a `stack`:

| Host | Surface | Owned by the app |
|---|---|---|
| `toast::toaster(view, &toasts, …)` | Toast stack on the inverse surface, in one of six positions | The `Toasts` queue |
| `dialog::host(view, Option<Dialog>)` | Centered dialog over a scrim (0.54) | Whether a dialog is shown |
| `sheet::host(view, Option<Sheet>)` | Side or bottom sheet, modal or not, with snap points | Whether it is open, its resting height |
| `tour::host(view, &tour)` | Scrim with a spotlight on the target and a card | The `Tour` value |
| `command_palette::command_palette(view, …)` | Searchable command list | Whether it is open, the query |

While a modal host is open, the view below receives no input and Tab cycles inside the surface.
