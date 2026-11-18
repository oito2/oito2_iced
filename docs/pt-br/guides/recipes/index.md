🌐 [English](../../../en/guides/recipes/index.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Receitas de Integração com o Desktop

Receitas para as partes de uma aplicação desktop que **não** fazem parte do design system OITO2: ícone na bandeja, notificações, iniciar com a sessão e empacotamento. O `oito2_iced` estiliza as janelas; estas receitas mostram como uma aplicação construída sobre ele conversa com o desktop.

As receitas com código são compiladas pela CI como o crate do workspace [`examples/desktop`](../../../../examples/desktop/Cargo.toml) (pacote `oito2_iced_desktop_example`, não publicado). Cada receita é um binário pequeno:

| Receita | Binário | Plataformas | Crates |
|---|---|---|---|
| [Ícone na bandeja](tray.md) | `tray` | Linux (StatusNotifierItem) | `ksni` |
| [Notificações](notifications.md) | `notify` | Linux (notificações freedesktop) | `notify-rust` |
| [Início automático](autostart.md) | `autostart` | Linux (XDG autostart) | só a biblioteca padrão |
| [Empacotamento](packaging.md) | — (só guia) | Linux | `cargo-deb`, `cargo-generate-rpm`, Flatpak |

Execute uma receita a partir da raiz do repositório:

```bash
cargo run -p oito2_iced_desktop_example --bin tray
cargo run -p oito2_iced_desktop_example --bin notify
cargo run -p oito2_iced_desktop_example --bin autostart
```

Os binários também compilam no Windows e no macOS, onde explicam que a receita é exclusiva do Linux. O que eles compartilham — o tema OITO2, as configurações de janela e um layout de página com navegação de foco pelo teclado — fica em [`examples/desktop/src/lib.rs`](../../../../examples/desktop/src/lib.rs).
