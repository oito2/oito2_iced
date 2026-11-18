🌐 [English](../../en/getting-started/quickstart.md) | **Português** | 🏠 [Índice](../index.md)

---

# Início rápido

O caminho mais curto até um app Iced com a aparência do OITO2: as fontes embutidas, um tema que acompanha as configurações do desktop, uma janela sem decorações com a barra de cabeçalho do OITO2 e alças de redimensionamento, foco de teclado e um primeiro componente. Ele supõe que os [pré-requisitos](./prerequisites.md) estejam instalados.

## 1. Crie o app

```bash
cargo new hello-oito2
cd hello-oito2
```

Adicione as dependências ao `Cargo.toml`:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
```

## 2. Escreva o app

Substitua o `src/main.rs`:

```rust
use iced::widget::{column, container};
use iced::{Element, Length, Subscription, Task, window};
use oito2_iced::l10n::Localizations;
use oito2_iced::theme::Brightness;
use oito2_iced::tokens::spacing;
use oito2_iced::widget::{controls, toggles};
use oito2_iced::window::{Labels, Placement, WindowState};
use oito2_iced::{Theme, density, focus, font};
use oito2_iced_settings::SystemSettings;

fn main() -> iced::Result {
    let app = iced::application(App::new, App::update, App::view)
        .title("Hello OITO2")
        .theme(App::theme)
        .subscription(App::subscription);

    // As faces Inter embutidas, para que todo peso seja renderizado na fonte do OITO2.
    font::INTER_FONTS
        .into_iter()
        .fold(app, |app, bytes| app.font(bytes))
        .default_font(font::INTER)
        .window(window::Settings {
            decorations: false,
            ..window::Settings::default()
        })
        .run()
}

struct App {
    settings: SystemSettings,
    window: WindowState,
    wifi: bool,
}

#[derive(Debug, Clone)]
enum Message {
    Settings(SystemSettings),
    Window(oito2_iced::window::Action),
    WindowEvent(window::Id, window::Event),
    Placement(Placement),
    Wifi(bool),
}

impl App {
    fn new() -> Self {
        Self {
            settings: oito2_iced_settings::read(),
            window: WindowState::default(),
            wifi: true,
        }
    }

    fn theme(&self) -> Theme {
        oito2_iced_settings::theme(&self.settings, None, Brightness::Light)
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            oito2_iced_settings::subscription().map(Message::Settings),
            window::events().map(|(id, event)| Message::WindowEvent(id, event)),
        ])
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Settings(settings) => self.settings = settings,
            Message::Window(action) => return oito2_iced::window::perform_latest(action),
            Message::WindowEvent(id, event) => {
                if self.window.update(&event) {
                    return WindowState::query(id).map(Message::Placement);
                }
            }
            Message::Placement(placement) => self.window.apply(placement),
            Message::Wifi(wifi) => self.wifi = wifi,
        }

        Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme> {
        // Os widgets se medem com a densidade do tema.
        density::set(self.settings.platform.density());

        let header = oito2_iced::window::header_bar(Message::Window)
            .title("Hello OITO2")
            .control_style(self.settings.window_control_style)
            .button_layout(self.settings.window_button_layout.clone())
            .state(self.window)
            .labels(Labels::new(&Localizations::en()));

        let content = container(controls::switch_button(
            toggles::switch(self.wifi).on_toggle(Message::Wifi),
            "Wi-Fi",
        ))
        .padding(spacing::PAGE)
        .width(Length::Fill)
        .height(Length::Fill);

        oito2_iced::window::resize_handles(
            focus::navigation(column![header, content]),
            Message::Window,
        )
    }
}
```

## 3. Rode o app

```bash
cargo run
```

A janela abre sem as decorações do sistema, com a barra de cabeçalho no estilo e no layout de botões do seu desktop. Arraste o cabeçalho para mover a janela, clique duas vezes nele para maximizar e arraste as bordas para redimensionar. Pressione **Tab** para levar o foco ao interruptor e depois **Espaço** para alterná-lo. Mude a cor de destaque, o estilo escuro ou o alto contraste nas configurações do desktop: o app os acompanha enquanto roda.

## O que cada peça faz

| Peça | Papel |
|---|---|
| `font::INTER_FONTS`, `font::INTER` | Registra as faces estáticas Inter embutidas (família `OITO2 Inter`) e as torna a fonte padrão |
| `oito2_iced_settings::theme` | Monta o tema a partir da cor de destaque do desktop, da preferência por claro ou escuro e do alto contraste |
| `oito2_iced_settings::subscription` | Entrega as configurações imediatamente e depois de cada mudança |
| `density::set` | Informa aos widgets os tamanhos de ponteiro ou de toque com que eles fazem o layout |
| `focus::navigation` | Move o foco de teclado com Tab e Shift+Tab |
| `window::header_bar`, `resize_handles` | O chrome no lado do cliente da janela sem decorações |
| `WindowState` | Se a janela está maximizada e com foco, para o chrome |

## Próximos passos

- Navegue pelos componentes no showcase (`cargo run -p oito2_iced_showcase` em um clone do repositório); a [referência de componentes](../reference/components.md) associa cada página ao seu módulo.
- Leia [Temas](../guides/theming.md) e [Integração com o desktop](../guides/desktop-integration.md).
