🌐 [English](../../../en/guides/recipes/tray.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Ícone na Bandeja

Uma **aplicação residente** inicia sem janela e vive na bandeja do sistema; o menu da bandeja abre e fecha uma janela com o estilo OITO2 e encerra a aplicação. Fechar a janela mantém a aplicação em execução.

Código: [`examples/desktop/src/bin/tray.rs`](../../../../examples/desktop/src/bin/tray.rs).

```bash
cargo run -p oito2_iced_desktop_example --bin tray
```

## Como funciona

| Peça | O que faz |
|---|---|
| `iced::daemon` | Executa a aplicação sem janela no início. Ao contrário de `iced::application`, não termina quando a última janela fecha. |
| `ksni` | Publica o ícone como um [StatusNotifierItem](https://www.freedesktop.org/wiki/Specifications/StatusNotifierItem/) via D-Bus. O serviço roda na própria thread. |
| Um canal e uma `Subscription` | Levam ao daemon, como mensagens, o que o menu da bandeja pede. |
| `window::open` / `window::close` | Abrem e fecham a janela sob demanda. |

### 1. Iniciar sem janela

`iced::daemon` recebe as mesmas funções de boot, update e view que `iced::application`, mas `view`, `title` e `theme` também recebem o `window::Id`, porque um daemon pode ter várias janelas:

```rust
let daemon = iced::daemon(Resident::default, Resident::update, Resident::view)
    .title("OITO2 Iced tray")
    .theme(Resident::theme)
    .subscription(Resident::subscription);
```

As fontes OITO2 são registradas com `.font(...)` para cada item de `oito2_iced::font::INTER_FONTS`, e `.default_font(oito2_iced::font::INTER)`, como em qualquer aplicação OITO2.

### 2. Da thread da bandeja para o daemon

O `ksni` chama a bandeja (`impl ksni::Tray`) a partir da thread do seu serviço D-Bus. A bandeja guarda a ponta de envio de um canal `iced::futures::channel::mpsc`, e cada item do menu envia um evento:

```rust
StandardItem {
    label: "Open window".into(),
    icon_name: "window-new".into(),
    activate: Box::new(|tray: &mut Self| tray.send(TrayEvent::Open)),
    ..Default::default()
}
.into(),
```

`iced::stream::channel` constrói o stream: entrega o seu sender à bandeja, inicia o serviço da bandeja e mantém o handle do serviço vivo enquanto o stream roda. `Subscription::run` transforma o stream em mensagens:

```rust
fn subscription(&self) -> Subscription<Message> {
    Subscription::batch([
        Subscription::run(tray_events).map(Message::Tray),
        window::close_events().map(Message::Closed),
    ])
}
```

`window::close_events()` informa uma janela fechada pelo gerenciador de janelas, para que o estado a esqueça.

### 3. Abrir, fechar e encerrar

- **Abrir**: `window::open(settings)` devolve o novo `window::Id` e uma task; o daemon guarda o id. Se a janela já está aberta, `window::gain_focus(id)` a traz para a frente.
- **Fechar**: `window::close(id)`.
- **Encerrar**: `iced::exit()`.

Se o serviço da bandeja não consegue iniciar (sem host de StatusNotifierItem, sem barramento de sessão), o stream envia `TrayEvent::Unavailable` e o daemon abre a janela com o motivo e um botão **Quit**, para que a aplicação nunca rode sem uma forma de encerrá-la.

## Dependência

```toml
[target.'cfg(target_os = "linux")'.dependencies]
ksni = { version = "0.3.7", default-features = false, features = ["async-io"] }
```

- O `ksni` fala D-Bus pelo `zbus`, puro Rust; nenhuma `libdbus` é necessária na compilação nem na execução.
- A feature `async-io` substitui o runtime padrão `tokio`: o `ksni` roda o serviço no seu próprio executor, independente do executor do Iced.
- O `ksni` é distribuído sob a **Unlicense**; o `deny.toml` do workspace a permite só para esse crate.

## Plataformas e desktops

- **Somente Linux.** No Windows e no macOS o binário imprime uma mensagem e termina; um ícone de bandeja nesses sistemas precisa de outro crate (por exemplo `tray-icon`), que esta receita não cobre.
- KDE Plasma, Xfce, LXQt, Cinnamon e Budgie mostram StatusNotifierItems sem configuração. O **GNOME** precisa da extensão *AppIndicator and KStatusNotifierItem Support*.
- Dentro do sandbox do Flatpak, inicie a bandeja com `.disable_dbus_name(true)` (veja a documentação do `ksni`) e conceda `--talk-name=org.kde.StatusNotifierWatcher` — veja [Empacotamento](packaging.md).
