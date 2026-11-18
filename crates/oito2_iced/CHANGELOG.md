# Changelog

All notable changes to `oito2_iced` are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- **Theme**: `Theme` with the OITO2 color roles of oito2_ui — the Material 3 roles plus window surfaces, status colors and syntax colors — for nine accents or any accent color, light and dark, standard and high contrast; builders from a palette or from hand-made roles; a density per theme.
- **Style catalogs** for every native Iced widget, with classes as enums (`Filled`, `Tonal`, `Sidebar`, `Variant`…) plus `Custom`, so Iced's `style` builders keep working.
- **Contrast** helpers with the WCAG formulas: ratio, foreground choice and `ensure`.
- **Tokens**: radii, spacing, sizes, the Material type scale, accents, neutrals and status colors.
- **Fonts**: Inter embedded as eight static faces (four weights, upright and italic) in the family `OITO2 Inter`.
- **Icons**: every Lucide 1.70.0 icon as a constant, freedesktop.org icon names mapped to Lucide, a placeholder icon, and ten animated icons drawn on a canvas.
- **Density**: pointer (34 px) and touch (40 px) control sizes, set per thread with `density::set`.
- **Keyboard focus** for every control: the `focusable` wrapper with focus ring and Enter/Space activation, Tab and Shift+Tab with `focus::navigation`, arrow-key groups, and extra focus operations.
- **Anchored overlay**: placement on four sides, alignment, flip, clamping to the window, optional arrow, dismissal on Escape or a click outside, and nesting.
- **Window chrome** for undecorated windows: title bar, header bar with start, end and search slots, window controls in the GNOME, Breeze, Xfwm, COSMIC, Windows and macOS styles, resize handles on every edge and corner, and the window state.
- **Buttons and controls**: filled, tonal, elevated, outlined, text and danger buttons, floating action button, icon, back and close buttons, checkbox, radio and switch, toggle, check, radio, switch and option buttons, list tiles with controls, toggle group, segment group, choice chip bar, slider and range slider, angle slider, rating and color disk.
- **Entries**: text field with label, helper, error and icons, search and title search fields, password field with Caps Lock warning, number entry, editable label, PIN entry, tags field, segmented entry, date, time and date-time entries, autocomplete, copy button and copy field, and highlighted text.
- **Pickers**: calendar and date picker, color picker and color picker button.
- **Containers and layout**: border container, section, tile and tile list, watermark, info box, info badge and banner, selectable container, expandable and expansion panel, scroll area with undershoot shadows, splitter, paned view, master-detail page, diagonal clip and draggable.
- **Navigation**: navigation rail, navigation page, tab bar and vertical tabs, steps, pagination and table of contents.
- **Menus and overlays**: popup menus with checked and multi-select items, select, context menu, split button, popover, hover card, tooltip, dialogs with a title bar, side and bottom sheets, toasts, command palette, guided tour and floating panel.
- **Data and display**: list box, virtualized tree view, JSON tree view, avatar and avatar group, linear and circular progress indicators, timer text, swap, carousel with page indicator, and marquee.
- **Animation**: the Material easing curves and durations over Iced's `Animation`.
- **Utilities without a user interface**: list, selection, tree and asynchronous list collections with text folding; byte size, compact number and relative time formats; key chords, shortcuts and a command registry; strings in English and Brazilian Portuguese; `Date` and `Time` types with localized formats.
- **Platform rules**: re-export of `oito2_foundation` (desktop environment, window control style, button layout).
- **Features**: `svg` (default), `image` (avatar photos), `qr_code` and `markdown` style catalogs.
