🌐 [English](../../en/concepts/why-oito2_iced.md) | **Português** | 🏠 [Índice](../index.md)

---

# Por que o oito2_iced

## O problema

Os apps OITO2 compartilham uma identidade visual em três runtimes: Dart + Flutter ([oito2_ui](https://github.com/oito2/oito2_ui)), Rust com Tauri v2 e Svelte ([oito2_uikit](https://github.com/oito2/oito2_uikit)) e Rust puro. Os dois primeiros tinham um design system; os apps desktop em Rust puro, não. Um app Rust sem webview precisava escolher entre um toolkit que traz o design de outro desktop e construir cada widget à mão.

Também não havia um design system Material 3 completo para o Iced: os widgets nativos vêm com um tema embutido pequeno, e o ecossistema em volta deles não tem um catálogo de componentes desktop com foco de teclado, overlays e chrome da janela.

## O que o oito2_iced faz

- Leva a identidade do OITO2 ao Iced: os mesmos tokens, papéis de cor, presets, regras de contraste, strings e chrome da janela do oito2_ui, conferidos com as fixtures de paridade dele nos testes.
- Estiliza todo widget nativo do Iced por meio de um único `Theme`, então widgets nativos e do OITO2 se misturam.
- Adiciona 84 componentes principais, de botões e campos a árvores, tours e painéis flutuantes, cada um com foco de teclado e navegação por Tab.
- Desenha o chrome da janela no lado do cliente no estilo do GNOME, KDE (Breeze), Xfce (Xfwm), COSMIC, Windows e macOS, e acompanha as configurações do desktop ao vivo.

## Por que Iced

| Toolkit | Por que foi ou não escolhido |
|---|---|
| **Iced 0.14** | **Escolhido.** 100 % Rust, a arquitetura Elm, renderizador próprio (wgpu com fallback por software tiny-skia), controle total de cada pixel e um `Catalog` de estilos por widget que um tema próprio pode implementar. O desktop COSMIC é construído sobre um fork dele. |
| Slint | Linguagem de marcação própria (uma quarta linguagem para o projeto) e menos válvulas de escape para overlays. |
| GPUI | Feito para o editor Zed, com API instável e quase nenhuma acessibilidade. |
| egui | Modo imediato; uma aparência fiel ao Material 3 é difícil. |
| Xilem, Dioxus Native | Experimentais, com poucos widgets. |
| gtk4-rs + libadwaita, libcosmic | Maduros, mas trazem o design de outro desktop. |

## Compromissos

- **Sem árvore de acessibilidade.** O Iced 0.14 não tem integração com o AccessKit no upstream, então leitores de tela não conseguem ler os widgets. O uso pelo teclado é completo: todo controle recebe o foco, o Tab o move e o anel de foco mostra onde ele está.
- **Toolkit pré-1.0.** O Iced muda a API entre versões menores. O workspace fixa o Iced 0.14 no `Cargo.lock` e passa para uma nova versão menor em bloco.
- **Linux primeiro.** Windows e macOS compilam e passam nos testes na CI, com o chrome da janela desenhado no estilo deles, mas não há promessa de integração nativa (sem os semáforos nativos do macOS, por exemplo).
- **Ainda não publicado.** Os crates são usados como dependências Git até a primeira release.

## oito2_iced ou oito2_uikit?

Os dois dão a um app desktop Rust a aparência do OITO2; a escolha depende do app:

| Prefira | Quando |
|---|---|
| **oito2_uikit** (Tauri v2 + Svelte) | A interface é grande e a acessibilidade importa: os componentes rodam em um webview, com suporte a leitores de tela. |
| **oito2_iced** (Iced) | O app é residente e leve, como um daemon de bandeja com janelas sob demanda; deve rodar sem webview; ou as distribuições-alvo não garantem o WebKitGTK. |

## Quando não usar

- O app precisa de suporte a leitores de tela hoje: use o oito2_uikit ou o oito2_ui.
- O app tem como alvo o celular ou a web: o oito2_iced é só para desktop; o oito2_ui cobre celular e web.
- O app deve se parecer com o desktop dele (GNOME, KDE) e não com o OITO2: use o toolkit desse desktop.
