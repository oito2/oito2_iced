🌐 [Português](../../pt-br/concepts/glossary.md) | **English** | 🏠 [Index](../index.md)

---

# Glossary

| Term | Meaning |
|---|---|
| **Accent** | The seed color of a theme's primary roles: one of the nine `tokens::Accent` values (blue, teal, green, yellow, orange, red, pink, purple, slate) or any color, such as the desktop accent. |
| **Anchored panel** | The overlay of `overlay::anchored`: a panel placed next to an anchor element, flipped and clamped to stay in the window, closed by Escape or a click outside. |
| **Catalog** | The Iced trait a theme implements to style a widget (`button::Catalog`, `container::Catalog`…). `oito2_iced::Theme` implements the catalog of every native Iced widget. |
| **Chrome colors** | The window surfaces of a theme: background, sidebar, mainbar, title bar and dialog (`ColorRoles::chrome`). |
| **Class** | The style variant a widget asks its catalog for, such as `button::Class::Tonal`. OITO2 classes are enums with a `Custom` variant for one-off styles. |
| **Color roles** | Every color a theme resolves (`ColorRoles`): the Material 3 roles (primary, surface containers, outline…) plus chrome, semantic and syntax colors. |
| **Contrast** | `Contrast::Standard` or `Contrast::High`; high contrast strengthens borders and text against their surfaces. |
| **Controller** | A plain struct with the state of a richer widget, kept in the app model: `SelectionController`, `TreeState`, `Tour`, `Toasts`, `upload::Controller`, `Crop`, `Signature`… |
| **CSD** | Client-side decorations: the app draws its own title bar and window controls in an undecorated window. |
| **Density** | Pointer (34-pixel controls) or touch (40-pixel controls) sizes, set per thread with `density::set`. |
| **Desktop environment** | The Linux desktop the app runs on (GNOME, KDE, Xfce, COSMIC, Cinnamon, MATE, Budgie, LXQt, Pantheon, Unity), detected by `oito2_foundation`. |
| **Focus ring** | The outline drawn around the element that has the keyboard focus, shown while the keyboard is in use. |
| **Focusable** | The `focus::focusable` wrapper that gives keyboard focus, a focus ring and Enter/Space activation to any element. |
| **Golden** | A reference image (`tests/goldens/<name>-tiny-skia.png`) a test compares a rendered snapshot with, within a 0.5 % tolerance. |
| **Header bar** | A title bar with start and end slots and an optional search field, built with `window::header_bar`. |
| **Host** | A widget that wraps the root of the view to draw a surface over the whole window: `toast::toaster`, `dialog::host`, `sheet::host`, `tour::host`. |
| **oito2_ui** | The Flutter design system, source of truth for tokens, colors, strings and behavior. |
| **oito2_uikit** | The Svelte 5 + Tauri v2 design system with the same identity. |
| **oito2_foundation, oito2_settings** | The crates of the `oito2_rs` repository with the platform rules and the desktop settings readers shared by the OITO2 Rust interfaces. |
| **Parity fixtures** | JSON files exported by oito2_ui (tokens, schemes, contrast, strings, formats, syntax colors, QR matrices) that the tests compare with the Rust implementation. |
| **Preset** | A named color theme of `oito2_iced_themes`: Everforest (light, dark), Rosé Pine (main, Moon, Dawn), Nord (dark, light), Dracula, Alucard, Catppuccin (Latte, Frappé, Macchiato, Mocha). |
| **Roving focus** | A group that is a single Tab stop, whose items move the focus with the arrow keys (tab bars, segment groups, toggle groups, rails, list boxes). |
| **Semantic colors** | Text-safe status and accent tones: primary text, error text, success, warning, info and link (`ColorRoles::semantic`). |
| **Simulator** | The headless Iced runtime of `iced_test` the behavior tests drive with clicks and keys. |
| **tiny-skia** | Iced's software renderer, used by the tests and available to apps with `ICED_BACKEND=tiny-skia`. |
| **Tokens** | The shared numbers of the design: spacing, radii, sizes and the type scale (`oito2_iced::tokens`). |
| **Window control style** | The visual family of the window buttons: GNOME, Breeze, Xfwm, COSMIC, Windows, macOS, or none. |
| **wgpu** | Iced's GPU renderer (Vulkan, Metal, DirectX 12, OpenGL), the default when available. |
