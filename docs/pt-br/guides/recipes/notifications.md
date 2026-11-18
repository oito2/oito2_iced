🌐 [English](../../../en/guides/recipes/notifications.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Notificações do Desktop

Um botão numa janela com o estilo OITO2 envia uma notificação do desktop. A chamada D-Bus roda como uma `iced::Task`, então a janela continua desenhando e respondendo enquanto o servidor de notificações responde.

Código: [`examples/desktop/src/bin/notify.rs`](../../../../examples/desktop/src/bin/notify.rs).

```bash
cargo run -p oito2_iced_desktop_example --bin notify
```

## Como funciona

O clique no botão devolve uma task que executa o envio assíncrono e transforma o resultado numa mensagem:

```rust
Message::Send => {
    self.sending = true;
    Task::perform(send(), Message::Sent)
}
```

`send` monta a notificação com o `notify-rust` e aguarda `show_async`, que fala com o serviço `org.freedesktop.Notifications` via D-Bus:

```rust
let mut notification = notify_rust::Notification::new();
notification
    .appname("OITO2 Iced")
    .summary("Hello from OITO2 Iced")
    .body("This notification was sent from an iced::Task.")
    .icon("dialog-information");

notification
    .show_async()
    .await
    .map(|_handle| ())
    .map_err(|error| format!("The notification was not sent: {error}"))
```

- O erro vira uma `String` porque uma mensagem precisa ser `Clone`; a janela o mostra no estilo de texto de erro.
- O botão fica desabilitado (`on_press_maybe(None)`) enquanto um envio está em andamento.
- Nunca chame o `show()` bloqueante dentro de `update`: ele congelaria a janela até o servidor responder.

## Dependência

```toml
[target.'cfg(target_os = "linux")'.dependencies]
notify-rust = { version = "4.19", default-features = false, features = ["z"] }
```

- A feature `z` seleciona o backend `zbus`, puro Rust (o padrão; a alternativa, a feature `d`, liga a `libdbus`). Com as features padrão desligadas, nada mais é incluído — sem decodificação de imagens, sem `chrono`.
- O `notify-rust` tem licença dupla MIT ou Apache-2.0.

## Plataformas

- **Linux** (e outros desktops freedesktop.org): qualquer servidor de notificações — GNOME Shell, KDE Plasma, `mako`, `dunst`, `xfce4-notifyd`.
- **Windows e macOS**: o `notify-rust` os suporta com outros backends, mas este exemplo declara o crate só para Linux, para que as compilações de Windows e macOS não precisem de SDK da plataforma; nelas o botão informa que a receita é exclusiva do Linux. Para suportá-los, mova o `notify-rust` para `[dependencies]` geral e remova o `#[cfg]` em volta de `send`.
- **Flatpak**: conceda `--talk-name=org.freedesktop.Notifications` — veja [Empacotamento](packaging.md).
