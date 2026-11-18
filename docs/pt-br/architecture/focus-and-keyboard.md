🌐 [English](../../en/architecture/focus-and-keyboard.md) | **Português** | 🏠 [Índice](../index.md)

---

# Foco e teclado

Como cada controle do OITO2 recebe o foco do teclado, como o foco se move e como os atalhos chegam aos comandos. Código: `crates/oito2_iced/src/{focus/,widget/focusable.rs,shortcuts/}`.

## O que o Iced 0.14 oferece e o que falta

O Iced 0.14 implementa a operação `Focusable` só no `text_input` e no `text_editor`: botões, checkboxes, radios e togglers nunca recebem o foco. O Tab não tem ação por padrão — o app precisa rodar `focus_next` por conta própria — e o Escape só sai de um campo de texto. O oito2_iced preenche essas lacunas com uma camada própria.

## `focusable`

`focus::focusable(content)` (também `widget::focusable`) é um widget próprio que:

- implementa `operation::Focusable`, para que as operações de foco do Iced o encontrem, e guarda `is_focused` no estado da sua árvore;
- desenha o anel de foco do tema em volta do conteúdo, **fora** dos seus limites, enquanto o teclado está em uso (um clique do mouse o esconde, como o modo de destaque de foco do Flutter);
- publica sua mensagem `on_activate` com Enter e Espaço;
- perde o foco com um clique fora dele;
- pode receber um `id` e um `radius` para o anel, e ser desativado com `enabled(false)` (aí sai da ordem do Tab, mas continua localizável pelo id).

Todo controle do OITO2 é construído sobre ele ou segue as mesmas regras: botões, toggles, chips, list tiles, tiles, botões de menu e os widgets próprios.

## Navegação com Tab: `focus::navigation`

`focus::navigation(root)` envolve a raiz da view e roda o `focus_next` do Iced no Tab e o `focus_previous` no Shift+Tab, sobre todo widget focável e campo de texto, na ordem do layout. É um widget, não uma subscription de teclado, então o simulador `iced_test` o exercita nos testes de comportamento.

## Grupos e foco itinerante

| Ferramenta | Comportamento |
|---|---|
| Grupos itinerantes (barras de abas, grupos de segmentos, grupos de toggles, trilho de navegação, list box, tree view, calendário) | O grupo é uma parada só do Tab; as setas, Home e End se movem dentro dele |
| `focus::edge_interceptor` | As setas movem o foco para o widget focável seguinte ou anterior dentro de um grupo e param nas pontas; o Tab continua saindo do grupo |
| `focus::ring` | Desenha um anel de foco em volta de um grupo enquanto algum widget dentro dele tem o foco |
| Hosts modais (diálogos, popovers, cartão do tour) | O Tab circula dentro da superfície enquanto ela está aberta; o foco volta para a âncora ou para o widget anterior quando ela fecha |

`focus::operations` acrescenta as operações de que eles precisam: `focus_first`, `focus_last`, `focus_next_clamped`, `focus_previous_clamped` e `has_focus`, ao lado das que o Iced oferece.

## Overlays

O Iced não move o foco para dentro de overlays, então os menus mantêm o foco no seu botão e movem um destaque com as setas; os popovers movem o foco para o painel quando abrem (`focus_on_open`) e de volta para a âncora quando fecham. Veja [Overlay](./overlay.md).

## Atalhos e comandos

O `oito2_iced::shortcuts` funciona sem interface de usuário:

- `KeyChord` é um pressionamento de tecla com modificadores; `Shortcut` é um chord ou uma sequência de chords, lida de um texto como `Mod+K` ou `G H`. `Mod` significa Command nos sistemas da Apple e Control nos demais.
- `KeyChord::from_keyboard_event` e `KeyChord::matches_event` os ligam aos eventos de teclado do Iced.
- `Command` e `CommandRegistry` registram ações com atalhos opcionais, buscam sem acentos e detectam conflitos (`conflicts`); `find_by_chord` devolve o comando de um chord; `SequenceMatcher` trata sequências com um tempo limite.
- `widget::command_palette` é a interface de usuário sobre um registro.

O showcase assina `keyboard::listen()`, transforma cada evento em um chord e roda o comando correspondente (Ctrl+K abre a paleta, Ctrl+, as configurações).

## Auditorias

Os testes do showcase percorrem cada página com Tab e falham quando um controle nunca recebe o foco, e conferem se todo controle clicável mostra o cursor de mão. Veja [Testes e goldens](./testing-and-goldens.md).

## Acessibilidade

Não há árvore de acessibilidade: o Iced 0.14 não tem integração com o AccessKit no upstream. Os rótulos que o oito2_ui dá às tecnologias assistivas existem como strings ou funções (por exemplo `avatar::description` ou `steps::description`) para que um app possa mostrá-los, mas nenhum leitor de tela consegue lê-los ainda.
