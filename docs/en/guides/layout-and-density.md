🌐 [Português](../../pt-br/guides/layout-and-density.md) | **English** | 🏠 [Index](../index.md)

---

# Layout and density

How to size and space an app so it lines up with the OITO2 widgets, on a pointer or a touch screen, in a wide or a narrow window.

## Density

OITO2 controls are 34 pixels tall for a pointer and 40 on a touch screen; check boxes, radios and switches have square targets of 34 and 48 pixels. Iced lays widgets out without the theme, so the density lives in `oito2_iced::density`, and the app sets it at the start of every `view`:

```rust
use oito2_iced::density;

fn view(&self) -> Element<'_, Message, Theme> {
    density::set(self.theme().density());
    // build the view
}
```

| Function | Returns |
|---|---|
| `density::set(Density)`, `density::get()` | Stores and reads the density of the current thread |
| `density::control_height()` | 34 (pointer) or 40 (touch) |
| `density::icon_size()` | The icon size of the density |
| `control_height_of(density)`, `icon_size_of(density)` | The same, for a given density |

Pick the density from the platform (`settings.platform.density()` in `oito2_iced_settings`), let the user choose it, and carry it in the theme with `theme.with_density(density)`, so the theme and the layout agree.

## Spacing, radii and sizes

Use the tokens instead of numbers, so custom layouts line up with the widgets:

| Module | Values |
|---|---|
| `tokens::spacing` | `XXS` 2, `XS` 4, `SM` 8, `MD` 12, `LG` 16, `PAGE` 20, `XL` 24, `XXL` 32, `XXXL` 48 |
| `tokens::radius` | `SMALL` 6, `MEDIUM` 8, `LARGE` 12, `WINDOW` 14 |
| `tokens::sizes` | `BUTTON_HEIGHT` 34, `TOUCH_BUTTON_HEIGHT` 40, `TITLE_BAR_HEIGHT` 46, `ICON_SIZE` 20, `MIN_POINTER_TARGET` 24, `MIN_TOUCH_TARGET` 48 |

```rust
use iced::widget::{column, container};
use oito2_iced::tokens::spacing;

let page = container(column![title, body].spacing(spacing::MD)).padding(spacing::PAGE);
```

## Surfaces

The window surfaces come from the theme's chrome colors, through container classes:

| Area | Class |
|---|---|
| Sidebar | `container::Class::Sidebar` |
| Main toolbar area | `container::Class::Mainbar` |
| Cards and groups | `container::Class::Card`, or `containers::border_container` |
| Title bar | drawn by `window::title_bar` and `window::header_bar` |

`widget::containers` also has `section` (a headline over grouped content), `tile` and `tile_list` (rows of a settings page) and `watermark`.

## Scrolling

`scroll_area::scroll_area(content)` is a `scrollable` with the OITO2 scrollbars; `.undershoot(true)` adds soft shadows at the edges that have more content beyond them. `scroll_area::undershoot` adds them to any scrollable built by hand.

## Adaptive layouts

Several widgets switch layout by their own width:

| Widget | Wide | Narrow |
|---|---|---|
| `master_detail` | List and detail side by side, with a resizable divider (from 620 px, `DEFAULT_BREAKPOINT`) | List, then detail with a back button |
| `navigation_page` | Labelled or extended navigation rail beside the content | Below 600 px (`COMPACT_BREAKPOINT`): a compact rail, or a bottom navigation bar with the touch density |
| `pagination` | Page numbers with ellipses | Compact form below 420 px (`COMPACT_WIDTH`) |

For layouts of your own, use Iced's `responsive` widget, which gives the available size to a closure.

## Resizable panes

- `splitter::splitter(&state, panes)` lays two or more panes side by side or stacked, with draggable, keyboard-operable dividers, minimum and maximum sizes and collapsible panes; the sizes live in a `splitter::State` the app keeps.
- `paned_view` is the two-pane form with a fixed side.
- Iced's `pane_grid` is styled too, for free-form tiling.

## Right-to-left

Widgets that point somewhere (sheets, splitters, menus, popovers, calendars, carousels, ratings, progress indicators and more) take `.direction(ReadingDirection::RightToLeft)` to mirror; see [Localization](./localization.md#reading-direction).
