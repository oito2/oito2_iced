🌐 [English](../../en/reference/showcase-pages.md) | **Português** | 🏠 [Índice](../index.md)

---

# Páginas do showcase

O showcase (`apps/showcase`, pacote `oito2_iced_showcase`) lista suas páginas em `apps/showcase/src/catalog.rs`, em sete grupos, na ordem da barra lateral. Cada página é um módulo de `apps/showcase/src/pages/` com o mesmo nome do seu identificador, que a paleta de comandos também usa (`Go to <page>`).

```bash
cargo run -p oito2_iced_showcase
```

## Visão geral

| Página | Identificador | Conteúdo |
|---|---|---|
| Getting Started | `getting_started` | Como configurar um app (`apps/showcase/assets/docs/getting_started.md`) |
| Changelog | `changelog` | O `CHANGELOG.md` de cada crate publicado |
| About | `about` | O projeto (`apps/showcase/assets/docs/about.md`) |

## Guias

Os padrões que todos os componentes compartilham, a partir de `apps/showcase/assets/docs/guides/`.

| Página | Identificador |
|---|---|
| Styling | `styling` |
| Composition | `composition` |
| Component State | `component_state` |
| Animations | `animations` |
| Forms | `forms` |
| Refs | `refs` |

## Componentes, Extras e Arquivos

84 páginas de componentes do `oito2_iced`, quatro do `oito2_iced_extras` e duas do `oito2_iced_files`. [Componentes](./components.md) lista todas com seus módulos Rust.

## Utilitários

| Página | Identificador | Módulo Rust |
|---|---|---|
| Collections | `collections` | `oito2_iced::collections` |
| Commands | `commands` | `oito2_iced::shortcuts`, `oito2_iced::widget::command_palette` |
| Format | `format` | `oito2_iced::format` |
| FreedesktopIcons | `freedesktop_icons` | `oito2_iced::icon::Freedesktop` |

## Tema

| Página | Identificador | Módulo Rust |
|---|---|---|
| Material theme | `material_theme` | `oito2_iced::theme`, `oito2_iced::tokens` |
| Foundation | `foundation` | `oito2_iced::foundation` (reexport de `oito2_foundation`) |

## Recursos de toda a janela

| Recurso | Como abrir |
|---|---|
| Busca da barra lateral | O campo no topo da barra lateral filtra as páginas pelo título |
| Paleta de comandos | Ctrl+K (Command+K no macOS) |
| Diálogo de configurações | Ctrl+, (Command+, no macOS), ou o botão Settings da barra de cabeçalho: tema, tema de cores (preset), contraste, densidade, idioma e desktop simulado |
| Renderizador por software | Inicie com `ICED_BACKEND=tiny-skia` |
