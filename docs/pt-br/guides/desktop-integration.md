🌐 [English](../../en/guides/desktop-integration.md) | **Português** | 🏠 [Índice](../index.md)

---

# Integração com o desktop

Como um app Iced com o oito2_iced se encaixa no desktop: uma janela sem decorações com o chrome do OITO2, as configurações do desktop acompanhadas ao vivo, um application id, atalhos de teclado e onde procurar ícones de bandeja e empacotamento. O [início rápido](../getting-started/quickstart.md) junta os três primeiros num só arquivo.

## A janela

Abra a janela sem as decorações do sistema e dê a ela um application id no Linux, que compositores Wayland e gerenciadores de janelas X11 usam para agrupar a janela e associá-la à sua desktop entry:

```rust
use iced::window;

fn settings() -> window::Settings {
    window::Settings {
        size: iced::Size::new(960.0, 640.0),
        min_size: Some(iced::Size::new(480.0, 360.0)),
        decorations: false,
        platform_specific: platform_specific(),
        ..window::Settings::default()
    }
}

#[cfg(target_os = "linux")]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific {
        application_id: "org.example.MyApp".into(),
        ..Default::default()
    }
}

#[cfg(not(target_os = "linux"))]
fn platform_specific() -> window::settings::PlatformSpecific {
    window::settings::PlatformSpecific::default()
}
```

Passe-a à aplicação com `.window(settings())`. Use o mesmo id como nome do arquivo `.desktop` do app.

## O chrome

Desenhe você mesmo a barra de título e as bordas de redimensionamento:

1. Coloque um `window::header_bar(Message::Window)` (ou `window::title_bar`) no topo da view, com os controles do app nos slots `start` e `end`.
2. Envolva a view inteira em `window::resize_handles(view, Message::Window)`.
3. No `update`, execute `oito2_iced::window::perform_latest(action)` para `Message::Window(action)` (ou `perform(id, action)` com várias janelas).
4. Mantenha um `WindowState` a partir de `iced::window::events()` e passe-o à barra com `.state(…)`, para que ela mostre o glifo de restaurar quando maximizada e esmaeça quando a janela perde o foco.

Arrastar, clicar duas vezes para maximizar e o menu da janela no clique direito funcionam na área vazia da barra.

## As configurações do desktop

Com o `oito2_iced_settings`:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
        oito2_iced_settings::subscription().map(Message::Settings),
        iced::window::events().map(|(id, event)| Message::WindowEvent(id, event)),
    ])
}
```

e use as configurações em três lugares:

| Configuração | Uso |
|---|---|
| `accent_color`, `color_scheme`, `high_contrast` | `oito2_iced_settings::theme(&settings, None, Brightness::Light)` |
| `platform.density()` | `density::set(…)` na `view`, e `theme.with_density(…)` |
| `window_control_style`, `window_button_layout` | `.control_style(…)` e `.button_layout(…)` da header bar |

O chrome então fica como o GNOME no GNOME, o Breeze no KDE Plasma, o Xfwm no Xfce, o COSMIC no COSMIC, e segue o lado e a ordem dos botões configurados em cada desktop.

## Atalhos de teclado

O Iced informa as teclas pressionadas via `iced::keyboard::listen()`. Transforme-as em combinações e procure-as num `CommandRegistry`, como o showcase faz para Ctrl+K e Ctrl+,:

```rust
use oito2_iced::shortcuts::{Command, CommandRegistry, KeyChord, OperatingSystem, Shortcut};

let mut commands = CommandRegistry::new();
if let Ok(shortcut) = Shortcut::parse("Mod+K") {
    commands.register(
        Command::new("palette", "Command palette", Message::OpenPalette).with_shortcut(shortcut),
    );
}

// No update, para Message::Key(event) vindo de keyboard::listen():
if let Some(chord) = KeyChord::from_keyboard_event(&event)
    && let Some(command) = commands.find_by_chord(&chord, OperatingSystem::current())
{
    let action = command.action.clone();
    return self.update(action);
}
```

`Mod` é Command no macOS e Control nos demais. `widget::command_palette` mostra o mesmo registro como uma lista pesquisável.

## Fontes

Registre as fontes embutidas, para que o texto fique igual em todos os sistemas:

```rust
let app = font::INTER_FONTS
    .into_iter()
    .chain(oito2_iced_syntax::font::JETBRAINS_MONO_FONTS) // só com blocos de código
    .fold(app, |app, bytes| app.font(bytes))
    .default_font(font::INTER);
```

## Ícones de bandeja, notificações, autostart e empacotamento

Apps residentes, notificações, entradas de autostart e pacotes ficam fora do design system; as [receitas](./recipes/index.md) mostram como montá-los em torno do oito2_iced, incluindo um app de bandeja com `iced::daemon`.

## Notas por plataforma

- **Wayland**: sem arrastar e soltar arquivos no winit 0.30 (veja [Arquivos](./files.md)).
- **Windows e macOS**: melhor esforço. O chrome é desenhado no estilo deles; não há integração nativa com os semáforos do macOS.
- **Sem árvore de acessibilidade**: o Iced 0.14 não expõe nada aos leitores de tela.
