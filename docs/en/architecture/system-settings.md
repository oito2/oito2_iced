🌐 [Português](../../pt-br/architecture/system-settings.md) | **English** | 🏠 [Index](../index.md)

---

# System settings

How an app learns the desktop's accent, light or dark preference, high contrast and window buttons, and keeps following them. Code: `crates/oito2_iced_settings/src/lib.rs`, over the `oito2_settings` and `oito2_foundation` crates (version 0.1.1, from the `oito2_rs` repository, on crates.io).

## Layers

```text
oito2_settings (platform readers)
  Linux:   GSettings, configuration files (KDE, Xfce, COSMIC…) and the
           settings portal, through GIO
  Windows: Windows Runtime
  macOS:   AppKit and the user defaults
        │  RawSettings
        ▼
oito2_foundation (rules, no I/O)
  desktop environment, window control style, button layout,
  accent color, color scheme, high contrast  ──►  SystemSettings
        │
        ▼
oito2_iced_settings (Iced glue)
  read(), subscription(), theme(), brightness(), contrast(), accent()
```

Every reader is safe to call from any thread and never fails: a store that can't be read leaves its values empty, and the composition falls back to the defaults of the desktop. The same two crates serve the Tauri plugins of oito2_uikit, so every OITO2 Rust app reads the system the same way.

## `SystemSettings`

| Field | Meaning |
|---|---|
| `platform` | Operating system and touch input; `platform.density()` gives the pointer or touch density |
| `desktop_environment` | GNOME, KDE, Xfce, Budgie, COSMIC, Cinnamon, MATE, LXQt, Pantheon, Unity or Unknown; `None` outside native Linux |
| `window_control_style` | The visual family of the window buttons |
| `window_button_layout` | Which buttons appear and on which side |
| `accent_color` | The system accent as `#RRGGBB`, or `None` |
| `color_scheme` | `Some(Light)`, `Some(Dark)` or `None` when the system states no preference |
| `high_contrast` | Whether the system asks for high contrast |

## The Iced side

- `read()` returns the current settings; call it when the app starts.
- `subscription()` is a `Subscription::run` over a stream that yields the current settings at once, then the settings after each change reported by `oito2_settings::watch`, a background watcher that lives as long as the subscription. When the platform can't be watched, only the first reading arrives.
- `theme(&settings, accent, fallback)` builds the base theme: the explicit `accent`, else the system accent, else the fallback (blue) accent; the system light or dark preference, else `fallback`; high contrast when the system asks; and the density of the platform.
- `brightness`, `contrast` and `accent` expose each rule on its own, for apps that build their theme differently (for example from a preset: `Preset::theme(contrast(&settings))`).

## What the app does with them

The app keeps `SystemSettings` in its model and rebuilds what depends on it in `theme` and `view`: the theme, the density passed to `density::set`, and the `control_style` and `button_layout` of the header bar. The showcase also lets the user override each choice in its settings dialog (mode, preset, accent, contrast, density, simulated desktop).

## Tests

The rules live in `oito2_foundation` and are tested there. `oito2_iced_settings` has unit tests for the theme rules (fallbacks, system preferences, explicit accent) and a doc test of the subscription.
