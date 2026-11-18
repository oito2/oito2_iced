🌐 [English](../../../en/guides/recipes/autostart.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Iniciar com a Sessão

Um switch numa janela com o estilo OITO2 grava ou remove uma **entrada XDG autostart**: um arquivo desktop entry no diretório de autostart, que as sessões freedesktop.org (GNOME, KDE Plasma, Xfce, Cinnamon, MATE, LXQt) executam no login.

Código:

- [`examples/desktop/src/autostart.rs`](../../../../examples/desktop/src/autostart.rs) — a lógica, só com a biblioteca padrão, com testes unitários.
- [`examples/desktop/src/bin/autostart.rs`](../../../../examples/desktop/src/bin/autostart.rs) — a janela.

```bash
cargo run -p oito2_iced_desktop_example --bin autostart
```

## O diretório de autostart

Seguindo a especificação XDG Base Directory, o diretório é:

| Condição | Diretório |
|---|---|
| `XDG_CONFIG_HOME` contém um caminho absoluto | `$XDG_CONFIG_HOME/autostart` |
| caso contrário (ausente, vazio ou relativo) | `$HOME/.config/autostart` |

`autostart::directory(xdg_config_home, home)` recebe os dois valores como argumentos, então é testado sem tocar no ambiente; `autostart::directory_from_env()` os lê do processo.

## A entrada

`autostart::Entry` guarda o id do arquivo, o nome e o comando. `Entry::contents()` escreve:

```ini
[Desktop Entry]
Type=Application
Name=OITO2 Iced autostart example
Exec=/path/to/autostart
Terminal=false
```

- Os argumentos de `Exec` seguem as regras de aspas da especificação Desktop Entry: `%` é duplicado, e um argumento com espaço ou outro caractere reservado vai entre aspas duplas, com `"`, `` ` ``, `$` e `\` escapados.
- O nome do arquivo é `<id>.desktop`; use o id da aplicação em notação DNS reversa, o mesmo do seu arquivo `.desktop` e do metainfo AppStream.
- O exemplo aponta `Exec` para o executável em execução (`std::env::current_exe()`). Uma aplicação empacotada deve usar o comando instalado, por exemplo `Exec=my-app --minimized`.

## Ligar e desligar

| Chamada | Efeito |
|---|---|
| `entry.enable(&dir)` | Cria o diretório se faltar e grava o arquivo. |
| `entry.disable(&dir)` | Remove o arquivo; arquivo ausente não é erro. |
| `entry.is_enabled(&dir)` | Se o arquivo existe. |

Toda função recebe o diretório, então os testes unitários rodam num diretório temporário e nunca tocam a home real:

```bash
cargo test -p oito2_iced_desktop_example
```

## Plataformas e sandboxes

- **Somente Linux.** No Windows e no macOS o switch fica desabilitado e a janela explica o motivo; esses sistemas usam outros mecanismos (a chave de registro `Run`, os itens de login) que esta receita não cobre.
- **Flatpak**: uma aplicação no sandbox não pode gravar em `~/.config/autostart`; ela pede ao portal *Background* (`org.freedesktop.portal.Background`, `RequestBackground` com `autostart`).
- Remover a aplicação também deve remover a entrada: inclua `~/.config/autostart/<id>.desktop` nas suas instruções de desinstalação.
