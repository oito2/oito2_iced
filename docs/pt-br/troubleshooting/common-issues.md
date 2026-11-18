🌐 [English](../../en/troubleshooting/common-issues.md) | **Português** | 🏠 [Índice](../index.md)

---

# Problemas comuns

Problemas que aparecem ao compilar ou executar apps com o oito2_iced, organizados por sintoma.

## Compilação

### O `pkg-config` não encontra `glib-2.0`, ou a compilação falha em `glib-sys` / `gio-sys`

No Linux, o `oito2_iced_settings` lê as configurações do desktop pelo GIO, que precisa dos arquivos de desenvolvimento da GLib. Instale-os:

```bash
sudo apt install libglib2.0-dev        # Debian, Ubuntu
sudo dnf install glib2-devel           # Fedora
```

### A compilação falha procurando `xkbcommon`

No Linux, a biblioteca de janelas precisa dos arquivos de desenvolvimento da `libxkbcommon`:

```bash
sudo apt install libxkbcommon-dev      # Debian, Ubuntu
sudo dnf install libxkbcommon-devel    # Fedora
```

### `error: package requires rustc 1.97` ou erros da edição 2024

Os crates precisam do Rust 1.97 ou mais recente. Atualize com `rustup update`. Dentro de um clone do repositório, o rustup usa automaticamente a versão fixada 1.99.0.

### `no method named 'class' found` em um widget do Iced

Escolher uma classe de estilo com `.class(…)` faz parte da feature `advanced` do Iced. O `oito2_iced` a habilita na sua dependência do Iced, então o método aparece quando o `iced` do app é a mesma versão 0.14 que o Cargo unifica com ela; confira se o app depende de `iced = "0.14"`, e não de outra versão minor.

### A máquina trava ou fica sem memória ao compilar os testes

Cada binário de teste linka todo o Iced, e muitos linkers rodando em paralelo precisam de muita memória. Limite os jobs, por exemplo `cargo test -j 6 …`. Para quem contribui: nunca adicione um `tests/<name>.rs` no nível de cima; use `tests/all/<area>.rs` (veja [Testes e goldens](../architecture/testing-and-goldens.md)).

## Execução

### A janela fica preta, o app fecha ao iniciar ou aparecem erros de GPU

O renderizador padrão é o wgpu (Vulkan, OpenGL, Metal ou DirectX 12). Em máquinas sem um driver de GPU funcionando, em máquinas virtuais ou em sistemas live, use o renderizador por software:

```bash
ICED_BACKEND=tiny-skia cargo run -p oito2_iced_showcase
```

O app precisa ter a feature `tiny-skia` do Iced, que está nas features padrão do Iced.

### O texto aparece em outra fonte, ou negrito e seminegrito ficam iguais

Os widgets do OITO2 esperam as faces Inter embutidas. Registre-as e faça da Inter a fonte padrão:

```rust
font::INTER_FONTS
    .into_iter()
    .fold(app, |app, bytes| app.font(bytes))
    .default_font(oito2_iced::font::INTER)
```

A pilha de texto do Iced escolhe uma face pelo peso exato; sem as faces estáticas, os pesos diferentes do regular caem em uma fonte do sistema. Pelo mesmo motivo, não as substitua por uma Inter variável. A família embutida se chama `OITO2 Inter`, então uma Inter instalada no sistema não interfere.

### Os blocos de código mostram uma fonte proporcional

Registre também as faces JetBrains Mono: `.font(…)` para cada item de `oito2_iced_syntax::font::JETBRAINS_MONO_FONTS`.

### Tab não move o foco

O Iced 0.14 não associa nenhuma ação ao Tab. Envolva a raiz da view em `oito2_iced::focus::navigation(…)`. Um controle sem a sua mensagem (`on_press`, `on_toggle`, `on_change`…) fica desabilitado e é pulado pelo Tab.

### Os controles ficam com a altura errada depois de alternar entre ponteiro e toque

Os widgets leem a densidade enquanto fazem o layout. Chame `oito2_iced::density::set(…)` no início de `view` com a densidade do tema.

### O tema não acompanha a cor de destaque ou o estilo escuro do desktop

- Assine `oito2_iced_settings::subscription()` e reconstrua o tema a partir das `SystemSettings` mais recentes.
- O desktop precisa expor a configuração: no Linux, os leitores usam o GSettings, os arquivos de configuração do KDE, do Xfce e do COSMIC, e o portal de configurações. Quando uma fonte não pode ser lida, os valores ficam vazios e os padrões se aplicam.
- Quando a plataforma não pode ser monitorada, só a primeira leitura chega: o app acompanha as mudanças depois de reiniciar.

### A janela não tem barra de título, ou tem duas barras de título

O oito2_iced desenha o próprio chrome. Abra a janela com `decorations: false` e coloque um `window::header_bar` ou `window::title_bar` na view; com `decorations: true`, a barra do sistema aparece acima da do OITO2.

### A janela não pode ser redimensionada

Uma janela sem decorações não tem bordas do sistema. Envolva a view inteira em `oito2_iced::window::resize_handles(view, Message::Window)` e execute as ações publicadas com `window::perform_latest`.

## Arquivos

### Soltar arquivos na janela não faz nada (Wayland)

O backend Wayland do winit 0.30 não informa arquivos soltos, então ali o `drop_zone` só oferece o clique (`drop_zone::can_drop_files()` é falso). Use o seletor de arquivos ou rode o app pelo XWayland, removendo as variáveis do Wayland:

```bash
env -u WAYLAND_DISPLAY -u WAYLAND_SOCKET cargo run -p oito2_iced_showcase
```

### Arquivos soltos em qualquer ponto da janela vão para uma única área de soltar

A plataforma informa os arquivos soltos sem posição, então toda área com `on_drop` recebe o que for solto na janela inteira. Dê `on_drop` a uma única área por janela.

### O seletor de arquivos ou o diálogo "Salvar como" não abre no Linux

O `oito2_iced_files` abre os diálogos pelo portal XDG do desktop. Confira se o `xdg-desktop-portal` e um backend para o seu desktop (por exemplo `xdg-desktop-portal-gnome`, `-kde` ou `-gtk`) estão instalados e em execução.

## Testes

### Um teste de golden falha em uma máquina onde todo o resto passa

- Rode os testes com `ICED_TEST_BACKEND=tiny-skia`: os goldens são feitos com o renderizador por software.
- Compare o `<name>.actual.png`, escrito ao lado do golden, com o próprio golden. A tolerância é de 0,5 % dos pixels.

### `missing golden … run with OITO2_UPDATE_GOLDENS=1`

Um teste de golden novo ainda não tem imagem de referência. Rode-o uma vez com `OITO2_UPDATE_GOLDENS=1`, examine a imagem escrita e faça o commit dela se estiver correta.

## Acessibilidade

### Os leitores de tela não leem os widgets

O Iced 0.14 não tem árvore de acessibilidade (não há integração com o AccessKit no upstream), então nenhuma tecnologia assistiva consegue ler uma janela do Iced. Esse é um limite conhecido do toolkit; se o seu app precisa de suporte a leitores de tela, veja [oito2_iced ou oito2_uikit?](../concepts/why-oito2_iced.md#oito2_iced-ou-oito2_uikit).
