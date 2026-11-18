🌐 [English](../../en/concepts/how-oito2_iced-works.md) | **Português** | 🏠 [Índice](../index.md)

---

# Como o oito2_iced funciona

Um passeio pelas partes com que um app lida, do tema à janela. A [visão geral da arquitetura](./architecture.md) cobre o fluxo de dados e as decisões de design; as [páginas de arquitetura](../architecture/theme-and-tokens.md) entram em cada subsistema.

![O showcase com uma página de componente aberta](../../img/showcase-component.png)
<!-- TODO: capturar o showcase com uma página de componente aberta, tema claro -->

## 1. O tema

O `oito2_iced::Theme` é o tema Iced do app. Ele é montado a partir de um brilho, uma cor de destaque e um nível de contraste — ou de um preset do `oito2_iced_themes`, ou das configurações do desktop com `oito2_iced_settings::theme` — e resolve todos os papéis de cor: os papéis do Material 3, as superfícies da janela (`chrome`), as cores de status (`semantic`) e as cores de código (`syntax`).

O tema implementa o `Catalog` de estilos de todo widget nativo do Iced. Cada widget escolhe uma **classe**: `button::Class::Filled`, `container::Class::Sidebar`, `text::Class::Variant` e assim por diante, ou `Custom` com uma closure para um estilo avulso.

```rust
use oito2_iced::Theme;
use oito2_iced::theme::{Brightness, Contrast};
use oito2_iced::tokens::Accent;

let theme = Theme::base(Brightness::Dark, Accent::Green.color(), Contrast::High);
let success = theme.roles().semantic.success;
```

## 2. Tokens e densidade

O `oito2_iced::tokens` guarda os números que todos os widgets compartilham: `spacing` (de 2 a 48 pixels), `radius` (6, 8, 12 e o raio da janela, 14), `sizes` (controles de 34 pixels para ponteiro, 40 no toque) e a escala `typography`. O Iced faz o layout dos widgets sem o tema, então a densidade fica em `oito2_iced::density`: o app chama `density::set` no início da `view` e os widgets a leem enquanto se medem.

## 3. Widgets

Os widgets ficam em `oito2_iced::widget`, um módulo por família (`button`, `toggles`, `entries`, `menu`, `tab_bar`…). Cada um é uma função que devolve um builder:

```rust
use oito2_iced::widget::button;

let save = button::filled("Save").on_press(Message::Save);
```

O app é dono de todos os valores: um widget mostra o que recebe e publica mensagens (`on_press`, `on_change`, `on_toggle`…). O estado com várias partes fica em structs simples que o app guarda no modelo, como `SelectionController`, `TreeState`, `Tour` ou `Toasts`, que o app atualiza a partir dos eventos que os widgets publicam.

## 4. Foco de teclado

O Iced 0.14 só dá foco de teclado a campos de texto. O oito2_iced o adiciona a tudo:

- `focus::focusable` envolve qualquer elemento para que ele receba o foco, desenhe o anel de foco e publique uma mensagem com Enter e Espaço; todo controle do OITO2 o usa.
- `focus::navigation` envolve a raiz da view e move o foco com Tab e Shift+Tab.
- Grupos como barras de abas, grupos de segmentos e caixas de lista são uma única parada de Tab, cujos itens se movem com as setas.

## 5. Overlays e hosts da raiz

Popovers, menus, selects, hover cards, tooltips, listas de autocompletar e cartões do tour abrem em um overlay **ancorado** junto ao elemento a que pertencem, passam para o outro lado quando falta espaço, ficam dentro da janela e fecham com Escape ou com um clique fora.

As superfícies que cobrem a janela inteira envolvem a raiz da view: `toast::toaster`, `dialog::host`, `sheet::host`, `tour::host` e a paleta de comandos. O showcase os empilha assim:

```text
resize_handles
└── focus::navigation
    └── toast::toaster
        └── command_palette
            └── dialog::host
                └── sheet::host
                    └── tour::host
                        └── barra de cabeçalho + página
```

## 6. Chrome da janela

Um app abre a janela com `decorations: false` e desenha o chrome ele mesmo: `window::header_bar` (título, áreas de início e de fim, busca) ou `window::title_bar`, controles da janela em seis estilos e `window::resize_handles` em volta da view inteira. O chrome publica `window::Action`s que o app executa com `window::perform_latest` e acompanha o `WindowState` que o app mantém a partir dos eventos da janela.

![A barra de cabeçalho em cada estilo de controles da janela](../../img/window-controls.png)
<!-- TODO: capturar a barra de cabeçalho nos estilos GNOME, Breeze, Xfwm, COSMIC, Windows e macOS -->

## 7. Configurações do sistema

O `oito2_iced_settings` lê o desktop por meio do `oito2_settings`: cor de destaque, preferência por claro ou escuro, alto contraste, estilo dos controles da janela e layout dos botões. O `subscription()` as entrega de imediato e depois de cada mudança, então o tema e o chrome acompanham o desktop enquanto o app roda.

## 8. O showcase

O `apps/showcase` é um app Iced com uma página por componente, agrupadas como no showcase em Flutter (Visão geral, Guias, Componentes, Extras, Arquivos, Utilitários, Tema), um campo de busca, uma paleta de comandos (Ctrl+K) e um diálogo de configurações (Ctrl+,) para o tema, preset, contraste, densidade, idioma e desktop simulado.

![O diálogo de configurações do showcase](../../img/showcase-settings.png)
<!-- TODO: capturar o diálogo de configurações do showcase -->
