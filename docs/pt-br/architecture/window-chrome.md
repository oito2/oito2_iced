🌐 [English](../../en/architecture/window-chrome.md) | **Português** | 🏠 [Índice](../index.md)

---

# Chrome da janela

Como uma janela do Iced sem decorações ganha barra de título, controles da janela e bordas de redimensionamento no estilo de cada desktop. Código: `crates/oito2_iced/src/window/`.

## Ações em vez de efeitos colaterais

O chrome nunca mexe na janela diretamente. Todo builder recebe um mapeamento `on_action: impl Fn(window::Action) -> Message` e publica:

| `Action` | Publicada por | Task da janela |
|---|---|---|
| `Drag` | Clique na área vazia da barra | `iced::window::drag` |
| `ToggleMaximize` | Clique duplo na barra, botão de maximizar/restaurar | `iced::window::toggle_maximize` |
| `ShowSystemMenu` | Clique com o botão direito na barra | `iced::window::show_system_menu` |
| `Minimize` | Botão de minimizar | `iced::window::minimize` |
| `Close` | Botão de fechar | `iced::window::close` |
| `Resize(Direction)` | Clique em uma das oito alças de redimensionamento | `iced::window::drag_resize` |

O app as transforma em tasks com `window::perform(id, action)`, ou `window::perform_latest(action)` em um app de janela única. Manter o efeito colateral no `update` permite que os testes no simulador confiram exatamente qual ação cada gesto publica.

## Barra de título e barra de cabeçalho

`window::title_bar(title, start, on_action)` e `window::header_bar(on_action)` montam uma `TitleBar`:

- `title` ou `title_element` no meio, substituído por `search` enquanto o app mostra um campo de busca;
- itens `start` depois dos botões da janela do início e itens `end` antes dos do fim;
- `control_style(WindowControlStyle)` e `button_layout(WindowButtonLayout)` vindos das configurações do desktop (ou escolhidos pelo app);
- `state(WindowState)` para o glifo de maximizar/restaurar e a aparência inativa;
- `labels(Labels::new(&localizations))` para as tooltips dos botões;
- `center_title`, `leading_inset`, `height` (46 por padrão) e `radius`.

O título fica centralizado, exceto no estilo Windows. Enquanto a janela não está em foco, o título e os botões são desenhados com `INACTIVE_OPACITY` (0.7). Uma linha no papel `outline_variant` percorre a borda inferior.

## Controles da janela

`window::controls` e `window::window_control` desenham os botões de sete `WindowControlStyle`s — GNOME, Breeze, Xfwm, COSMIC, Windows, macOS e nenhum — como geometria de canvas (por isso o crate principal ativa a feature `canvas` do Iced). O `WindowButtonLayout` (do `oito2_foundation`) define quais botões aparecem e em que lado, lido da configuração do próprio desktop quando ela existe (por exemplo o `button-layout` do GNOME).

## Alças de redimensionamento

O Iced upstream não tem borda de redimensionamento para janelas sem decorações. `window::resize_handles(content, on_action)` envolve a view inteira com oito alças invisíveis — bordas com `RESIZE_BORDER` (6) pixels de espessura e cantos com `RESIZE_CORNER` (12) pixels de largura — que mostram o cursor de redimensionamento correspondente e publicam `Action::Resize(direction)`. O conteúdo continua totalmente utilizável embaixo delas.

## Estado da janela

`WindowState { maximized, focused, fullscreen }` fica no modelo do app:

1. Assine `iced::window::events()`.
2. Chame `state.update(&event)`; ele devolve `true` quando o posicionamento pode ter mudado.
3. Em seguida rode `WindowState::query(id)` e aplique o `Placement` resultante com `state.apply(placement)`.

## Notas por plataforma

- **Linux** é o alvo principal, no X11 e no Wayland, em todo ambiente desktop. Defina o application id do Wayland por meio de `window::Settings::platform_specific` (o showcase usa `io.github.oito2.oito2_iced.showcase`).
- **Windows** recebe o estilo de controles do Windows, com o título no início.
- **macOS** recebe o estilo de controles do macOS desenhado pelo chrome; não há semáforos nativos.

O comportamento real em cada gerenciador de janelas (arrastar, clique duplo, menu do sistema, cursores de redimensionamento) é conferido à mão; os testes automatizados conferem as ações publicadas no simulador.
