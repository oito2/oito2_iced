🌐 [Português](../../pt-br/architecture/focus-and-keyboard.md) | **English** | 🏠 [Index](../index.md)

---

# Focus and keyboard

How every OITO2 control takes the keyboard focus, how the focus moves, and how shortcuts reach commands. Code: `crates/oito2_iced/src/{focus/,widget/focusable.rs,shortcuts/}`.

## What Iced 0.14 gives and what it lacks

Iced 0.14 implements the `Focusable` operation only in `text_input` and `text_editor`: buttons, checkboxes, radios and togglers never take the focus. Tab is not bound by default — an app has to run `focus_next` itself — and Escape only leaves a text input. oito2_iced fills these gaps with its own layer.

## `focusable`

`focus::focusable(content)` (also `widget::focusable`) is a custom widget that:

- implements `operation::Focusable`, so Iced's focus operations find it, and keeps `is_focused` in its tree state;
- draws the focus ring of the theme around its content, **outside** its bounds, while the keyboard is in use (a mouse press hides it, like Flutter's focus highlight mode);
- publishes its `on_activate` message on Enter and Space;
- loses the focus on a press outside it;
- can be given an `id`, a ring `radius`, and be disabled with `enabled(false)` (it then leaves the Tab order but stays findable by id).

Every OITO2 control is built on it or follows the same rules: buttons, toggles, chips, list tiles, tiles, menu buttons and the custom widgets.

## Tab traversal: `focus::navigation`

`focus::navigation(root)` wraps the root of the view and runs Iced's `focus_next` on Tab and `focus_previous` on Shift+Tab, over every focusable widget and text input, in layout order. It is a widget, not a keyboard subscription, so the `iced_test` simulator exercises it in the behavior tests.

## Groups and roving focus

| Tool | Behavior |
|---|---|
| Roving groups (tab bars, segment groups, toggle groups, navigation rail, list box, tree view, calendar) | The group is one Tab stop; the arrow keys, Home and End move inside it |
| `focus::edge_interceptor` | Arrow keys move the focus to the next or previous focusable widget inside a group and stop at its ends; Tab still leaves the group |
| `focus::ring` | Draws one focus ring around a group while any widget inside it has the focus |
| Modal hosts (dialogs, popovers, tour card) | Tab cycles inside the surface while it is open; the focus returns to the anchor or the previous widget when it closes |

`focus::operations` adds the operations these need: `focus_first`, `focus_last`, `focus_next_clamped`, `focus_previous_clamped` and `has_focus`, next to the ones Iced provides.

## Overlays

Iced doesn't move the focus into overlays, so menus keep the focus on their button and move a highlight with the arrow keys; popovers move the focus into the panel when it opens (`focus_on_open`) and back to the anchor when it closes. See [Overlay](./overlay.md).

## Shortcuts and commands

`oito2_iced::shortcuts` works without a user interface:

- `KeyChord` is one key press with modifiers; `Shortcut` is a chord or a sequence of chords, parsed from text such as `Mod+K` or `G H`. `Mod` means Command on Apple systems and Control elsewhere.
- `KeyChord::from_keyboard_event` and `KeyChord::matches_event` connect them to Iced keyboard events.
- `Command` and `CommandRegistry` register actions with optional shortcuts, search them without accents, and detect clashes (`conflicts`); `find_by_chord` returns the command of a chord; `SequenceMatcher` handles sequences with a timeout.
- `widget::command_palette` is the user interface over a registry.

The showcase subscribes to `keyboard::listen()`, turns each event into a chord and runs the matching command (Ctrl+K opens the palette, Ctrl+, the settings).

## Audits

The showcase tests walk every page with Tab and fail when a control never receives the focus, and check that every clickable control shows the hand cursor. See [Testing and goldens](./testing-and-goldens.md).

## Accessibility

There is no accessibility tree: Iced 0.14 has no AccessKit integration upstream. Labels that oito2_ui gives to assistive technologies exist as strings or functions (for example `avatar::description` or `steps::description`) so an app can show them, but no screen reader can read them yet.
