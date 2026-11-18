🌐 [English](../../en/reference/crates-and-features.md) | **Português** | 🏠 [Índice](../index.md)

---

# Crates e features

Todos os crates do workspace, com seus módulos públicos, features do Cargo e dependências. Todos os crates compartilham a versão `0.0.0`, a edição 2024, `rust-version = "1.97"` e a licença GPL-3.0-or-later, e nenhum foi publicado ainda (`publish = false`). O rustdoc de cada item é a referência completa da API: `cargo doc --workspace --no-deps --all-features --open`.

## `oito2_iced`

Caminho `crates/oito2_iced`. O crate principal.

| Módulo | Conteúdo |
|---|---|
| `theme` | `Theme`, `ColorRoles` (com `ChromeColors`, `SemanticColors`, `SyntaxColors`), `Palette`, `FamilyColors`, `Brightness`, `Contrast`, funções auxiliares de `contrast`, um submódulo de catálogo por widget nativo com sua `Class` |
| `tokens` | `radius`, `spacing`, `sizes`, `typography`, `Accent`, `Neutrals`, `Status` |
| `font` | As oito faces Inter embutidas (`INTER_FONTS`), `INTER`, `inter(weight)`, `inter_italic(weight)`, `INTER_FAMILY` |
| `icon` | `Icon`, `lucide` (todos os ícones Lucide, gerado), `Freedesktop` (nomes do freedesktop.org), `icon` e `Glyph` (com `svg`), `placeholder`, `animated` |
| `density` | `set`, `get`, `control_height`, `icon_size` e o reexport de `Density` |
| `focus` | `navigation`, `focusable`, `ring`, `edge_interceptor`, `operations` |
| `overlay` | `anchored`, `place`, `Placement`, `Alignment`, `Side` |
| `widget` | Todos os widgets do OITO2, um módulo por família; veja [Componentes](./components.md) |
| `window` | `title_bar`, `header_bar`, `controls`, `window_control`, `resize_handles`, `Action`, `perform`, `perform_latest`, `WindowState`, `Labels` |
| `animation` | Curvas de easing e durações do Material sobre o `Animation` do Iced |
| `collections` | `ListCollection`, `SelectionController`, `TreeCollection`, `AsyncListController`, `fold_text` |
| `format` | `Format`: tamanhos em bytes, números compactos, tempos relativos |
| `shortcuts` | `KeyChord`, `Shortcut`, `Command`, `CommandRegistry`, `SequenceMatcher` |
| `l10n` | `Localizations` em inglês e português do Brasil |
| `date` | `Date` e `Time` |
| `foundation` | Reexport de `oito2_foundation`: plataforma, ambiente desktop, estilo dos controles da janela e layout dos botões |

A raiz do crate também exporta `Theme` e o alias `Element<'a, Message>` (um elemento do Iced com o tema do OITO2).

### Features

| Feature | Padrão | Habilita | Necessária para |
|---|---|---|---|
| `svg` | Sim | O `svg` do Iced: o catálogo de estilos de SVG e os glifos Lucide | Botões de ícone e FAB, `icon::icon`, botão e campo de cópia, campos de senha e de busca, calendário, seletor de data e entradas de data/hora, paginação, e os ícones de menus, abas, toasts, caixas de informação e outros widgets |
| `image` | Não | O `image` do Iced | Fotos em `avatar` |
| `qr_code` | Não | O `qr_code` do Iced | O catálogo de estilos de QR code (usado pelo `oito2_iced_extras`) |
| `markdown` | Não | O `markdown` do Iced | O catálogo de estilos de Markdown |

O Iced é uma dependência sem as features padrão, mais `advanced`, `canvas`, `x11`, `wayland` e `thread-pool`; o app habilita um renderizador (`wgpu` e/ou `tiny-skia`, ambos nas features padrão do Iced).

## `oito2_iced_settings`

Caminho `crates/oito2_iced_settings`. Depende de `oito2_iced` e de `oito2_settings` (feature `watch`).

| Item | Descrição |
|---|---|
| `read()` | As `SystemSettings` atuais |
| `subscription()` | As configurações logo de início e após cada mudança |
| `theme(&settings, accent, fallback)` | O tema base que acompanha as configurações |
| `brightness`, `contrast`, `accent` | Cada regra isoladamente |
| `SystemSettings`, `ColorScheme`, `Platform` | Reexports de `oito2_settings` |

## `oito2_iced_themes`

Caminho `crates/oito2_iced_themes`. Depende de `oito2_iced` e de `catppuccin`.

| Item | Descrição |
|---|---|
| `Preset` | Os 13 presets: `ALL`, `label`, `name`, `from_name`, `is_dark`, `brightness`, `with_brightness`, `family`, `palette`, `theme(contrast)` |
| `palettes` | As paletas oficiais de Catppuccin, Dracula/Alucard, Everforest, Nord e Rosé Pine |

## `oito2_iced_syntax`

Caminho `crates/oito2_iced_syntax`. Depende de `oito2_iced`, `syntect` e `two-face`.

| Módulo | Conteúdo |
|---|---|
| `code_block` | `code_block(&l10n, code)`: números de linha, botão de cópia, rolagem horizontal, JetBrains Mono |
| `highlighter` | `highlight`, `highlight_cached`, `Highlighter` para o `text_editor::highlight_with` do Iced |
| `language` | `LANGUAGES` (27 linguagens com aliases), `find`, `supports` |
| `role` | `Role` (11 papéis de token) e a tabela de escopos TextMate |
| `font` | Quatro faces JetBrains Mono (`JETBRAINS_MONO_FONTS`), `JETBRAINS_MONO`, `JETBRAINS_MONO_LICENSE` |

## `oito2_iced_extras`

Caminho `crates/oito2_iced_extras`. Depende de `oito2_iced` (features `image` e `qr_code`), `qrcode` e `image`.

| Módulo | Conteúdo |
|---|---|
| `qr_code` | `qr_code`: módulos quadrados ou arredondados, imagem central, níveis de correção de erro |
| `signature_pad` | `signature_pad` e o modelo `Signature`, com exportação para SVG e PNG |
| `image_cropper` | `image_cropper`, `preview` e o modelo `Crop`; `crop()` devolve a imagem recortada |
| `navigation_menu` | `navigation_menu` com itens de link e de painel, e o modelo `MenuState` |

## `oito2_iced_files`

Caminho `crates/oito2_iced_files`. Depende de `oito2_iced` e de `rfd` (features `xdg-portal` e `wayland`).

| Item | Conteúdo |
|---|---|
| `pick` | O seletor de arquivos, como uma `Task` |
| `file` | `File`, `TypeGroup`, `matches_types` |
| `drop_zone` | `drop_zone`, `can_drop_files` |
| `upload` | `Controller`, `file_upload`, `RejectedFile`, `Rejection` |
| `download` | `save`, `download_trigger`, `is_supported`, `Error` |

## Não publicados

| Crate | Caminho | Finalidade |
|---|---|---|
| `oito2_iced_test_utils` | `crates/oito2_iced_test_utils` | Configurações do simulador com as fontes embutidas, pixels de snapshot, goldens com tolerância (`golden!`), leitura do cursor |
| `oito2_iced_showcase` | `apps/showcase` | O app showcase |
| `oito2_iced_desktop_example` | `examples/desktop` | O código das [receitas](../guides/recipes/index.md) de desktop: ícone na bandeja, notificações, início automático |
| `oito2_iced_parity` | `tools/parity` | Fixtures de paridade |
| `oito2_iced_icons` | `tools/icons` | Módulo de ícones Lucide |
| `oito2_iced_gen_docs` | `tools/gen_docs` | Referência da API do showcase |
