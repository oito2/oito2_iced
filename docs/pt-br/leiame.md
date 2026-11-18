# oito2_iced

Um design system pensado primeiro para o Linux, para apps desktop nativos em Rust feitos com [Iced](https://iced.rs) 0.14, com a identidade do [oito2_ui](https://github.com/oito2/oito2_ui): papéis de cor do Material 3, densidade desktop de 34 px, 13 presets de cor, contraste verificado pela WCAG e chrome de janela no lado do cliente em seis estilos.

[![CI](https://github.com/oito2/oito2_iced/actions/workflows/ci.yml/badge.svg)](https://github.com/oito2/oito2_iced/actions/workflows/ci.yml)
[![Rust](https://img.shields.io/badge/rust-1.97%2B-orange?logo=rust)](https://www.rust-lang.org)
[![Iced](https://img.shields.io/badge/iced-0.14-3645FF)](https://iced.rs)
[![Plataforma](https://img.shields.io/badge/plataforma-Linux-informational?logo=linux)](getting-started/installation.md)
[![Licença](https://img.shields.io/github/license/oito2/oito2_iced?label=licen%C3%A7a)](../../LICENSE)

🌐 **Idioma:** [English](../../README.md) · Português

## Índice

- [Visão geral](#visão-geral)
- [Pré-requisitos e instalação rápida](#pré-requisitos-e-instalação-rápida)
- [Atualização e manutenção](#atualização-e-manutenção)
- [Documentação](#documentação)
- [Licença](#licença)

## Visão geral

Os apps OITO2 são escritos em Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)), em Rust com Tauri v2 e Svelte ([oito2_uikit](https://github.com/oito2/oito2_uikit)) e em Rust puro. O oito2_iced cobre o último caso: apps desktop sem webview, desenhados pelo Iced, com a mesma aparência dos outros dois. O oito2_ui continua sendo a fonte de verdade do design; suas fixtures de paridade (tokens, esquemas de cor, contraste, strings, formatos, cores de sintaxe, matrizes de QR) são copiadas para este repositório e conferidas pelos testes, e um workflow semanal as compara com o `main` do oito2_ui.

O que ele oferece:

- **Um tema para cada widget do Iced** — `oito2_iced::Theme` implementa o catálogo de estilos de todos os widgets nativos do Iced, com os papéis de cor do Material 3, as superfícies da janela, as cores de status e as cores de sintaxe do oito2_ui, claro e escuro, nove cores de destaque, contraste padrão e alto.
- **84 componentes no crate principal** — botões, alternadores, campos, seletores, menus, popovers, diálogos, painéis, toasts, abas, trilho de navegação, mestre-detalhe, caixa de lista, árvore, carrossel, tour, painel flutuante e mais, cada um com foco de teclado, navegação com Tab e anel de foco.
- **Chrome da janela** — barra de título, barra de cabeçalho e controles da janela nos estilos GNOME, Breeze, Xfwm, COSMIC, Windows e macOS, e alças de redimensionamento para janelas sem decoração.
- **Configurações do sistema** — cor de destaque, preferência por claro ou escuro, alto contraste, estilo dos controles da janela e layout dos botões, lidos do desktop e acompanhados enquanto o app roda.
- **Presets** — Everforest, Rosé Pine, Nord, Dracula e Alucard, e Catppuccin, 13 no total.
- **Crates satélites** — realce de sintaxe com blocos de código, QR codes, um painel de assinatura, um recortador de imagens, um menu de navegação, e envio de arquivos, área de soltar e download.
- **Ainda sem suporte a tecnologias assistivas** — o Iced 0.14 não tem árvore de acessibilidade (não há AccessKit no upstream), então leitores de tela não conseguem ler os widgets. O uso pelo teclado é totalmente suportado.

O oito2_iced é licenciado sob a GPL-3.0-or-later: um app que o usa precisa ser distribuído sob uma licença compatível.

![O showcase do oito2_iced](../img/showcase.png)
<!-- TODO: capturar uma imagem da janela principal do showcase -->

### Crates

| Crate | Conteúdo |
|---|---|
| [`oito2_iced`](../../crates/oito2_iced) | Tema, tokens, fontes, ícones, foco, overlay, widgets, chrome da janela, animação, coleções, formatos, atalhos e strings |
| [`oito2_iced_settings`](../../crates/oito2_iced_settings) | Configurações do desktop como uma subscription do Iced, e o tema que as acompanha |
| [`oito2_iced_themes`](../../crates/oito2_iced_themes) | Os 13 presets de cor |
| [`oito2_iced_syntax`](../../crates/oito2_iced_syntax) | Realce de sintaxe, o bloco de código e a fonte JetBrains Mono |
| [`oito2_iced_extras`](../../crates/oito2_iced_extras) | QR code, painel de assinatura, recortador de imagens e menu de navegação |
| [`oito2_iced_files`](../../crates/oito2_iced_files) | Área de soltar, envio de arquivos, seletor de arquivos e download |

## Pré-requisitos e instalação rápida

### Sistemas suportados

| SO | Status | Versão mínima / distribuições | Dependências de execução |
|---|---|---|---|
| Linux | Suportado (principal) | X11 ou Wayland; compilado e testado no Ubuntu (`ubuntu-latest` do GitHub) | `libxkbcommon`, GLib, um driver Vulkan ou OpenGL (ou o renderizador por software) |
| Windows | Melhor esforço | Compila e passa nos testes na CI (`windows-latest`); sem promessa de integração nativa | — |
| macOS | Melhor esforço | Compila e passa nos testes na CI (`macos-latest`); sem promessa de integração nativa | — |

O projeto ainda não produz instaladores nem binários: os crates são usados como dependências Git e o showcase é compilado a partir do código-fonte.

### Requisitos

- [rustup](https://rustup.rs): o repositório fixa o Rust **1.99.0** em `rust-toolchain.toml`, que o rustup instala no primeiro uso. Os crates compilam com o Rust **1.97** ou mais recente.
- No Debian e no Ubuntu, os arquivos de desenvolvimento da `libxkbcommon` e da GLib:

  ```bash
  sudo apt install libxkbcommon-dev libglib2.0-dev
  ```

O [guia de instalação](getting-started/installation.md) cobre todos os sistemas e o [início rápido](getting-started/quickstart.md) monta um primeiro app.

### Experimente o showcase

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

Defina `ICED_BACKEND=tiny-skia` para forçar o renderizador por software.

### Adicione os crates a um app

Os crates ainda não estão no crates.io; adicione-os a partir do repositório Git:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }
# Crates opcionais, conforme a necessidade:
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_themes = { git = "https://github.com/oito2/oito2_iced" }
```

Registre as fontes embutidas, use o tema OITO2 e envolva a view na navegação de foco:

```rust
use iced::Element;
use oito2_iced::{Theme, focus, font};
use oito2_iced::widget::button;

fn main() -> iced::Result {
    font::INTER_FONTS
        .into_iter()
        .fold(iced::application(App::default, App::update, App::view), |app, bytes| {
            app.font(bytes)
        })
        .default_font(font::INTER)
        .theme(|_: &App| Theme::light())
        .run()
}

#[derive(Default)]
struct App {
    count: u32,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.count += 1,
        }
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        focus::navigation(
            button::filled(format!("Clicked {} times", self.count)).on_press(Message::Increment),
        )
        .into()
    }
}
```

## Atualização e manutenção

- **Um app** que depende do repositório Git passa para o commit mais recente com `cargo update -p oito2_iced` (um `-p` para cada crate OITO2 que ele usa).
- **O showcase** compilado a partir de um clone é atualizado com `git pull` seguido de `cargo run -p oito2_iced_showcase`.

Cada crate registra suas mudanças no próprio `CHANGELOG.md` (por exemplo, [`crates/oito2_iced/CHANGELOG.md`](../../crates/oito2_iced/CHANGELOG.md)).

## Documentação

- [Índice da documentação](index.md) — tudo o que está abaixo, e mais.
- [Instalação](getting-started/installation.md) e [Início rápido](getting-started/quickstart.md).
- [Por que o oito2_iced](concepts/why-oito2_iced.md) — incluindo quando escolher o oito2_uikit no lugar dele.
- [Visão geral da arquitetura](concepts/architecture.md).
- [Guias](index.md#-guias) — temas, layout e densidade, localização, arquivos e integração com o desktop.
- [Componentes](reference/components.md) — todas as páginas do showcase e o módulo Rust por trás de cada uma.
- [Como contribuir](contribuindo.md), [Código de Conduta](codigo-de-conduta.md) e [Política de Segurança](seguranca.md).

## Licença

GPL-3.0 — veja [LICENSE](../../LICENSE).
