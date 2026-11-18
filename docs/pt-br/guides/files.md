🌐 [English](../../en/guides/files.md) | **Português** | 🏠 [Índice](../index.md)

---

# Arquivos

Como permitir que usuários escolham, soltem, enviem e salvem arquivos com o `oito2_iced_files`. Adicione o crate primeiro:

```toml
[dependencies]
oito2_iced_files = { git = "https://github.com/oito2/oito2_iced" }
```

Os widgets nunca abrem um diálogo sozinhos: os diálogos são `iced::Task`s que o app retorna de `update`.

## Escolher arquivos

`oito2_iced_files::pick(&types, multiple)` abre o seletor de arquivos da plataforma e entrega os `File`s escolhidos (uma lista vazia quando cancelado). No Linux ele passa pelo portal XDG do desktop; no Windows e no macOS, pelo diálogo nativo.

```rust
use iced::Task;
use oito2_iced_files::file::{File, TypeGroup};

#[derive(Debug, Clone)]
enum Message {
    Choose,
    Chosen(Vec<File>),
}

fn update(message: Message) -> Task<Message> {
    match message {
        Message::Choose => {
            let pdf = TypeGroup::new().label("PDF").extensions(["pdf"]);
            oito2_iced_files::pick(&[pdf], true).map(Message::Chosen)
        }
        Message::Chosen(files) => {
            println!("{} files", files.len());
            Task::none()
        }
    }
}
```

`File` guarda o nome, o tamanho, o tipo MIME e o caminho opcionais, e se é um diretório; `TypeGroup`, um rótulo, extensões e tipos MIME.

## Área de soltar

`drop_zone::drop_zone(&l10n)` é uma caixa tracejada que publica `on_choose` num clique, Enter ou Espaço, e `on_drop` com os arquivos soltos na janela.

```rust
use oito2_iced_files::drop_zone::drop_zone;

let zone = drop_zone(&l10n)
    .hint("PNG or JPEG, up to 5 MB")
    .on_choose(Message::Choose)
    .on_drop(Message::Files);
```

O que saber sobre arquivos soltos:

- A plataforma informa os arquivos soltos sem posição, então uma área não sabe onde os arquivos caíram: toda área com `on_drop` recebe o que for solto na **janela inteira**. Dê `on_drop` a uma única área por janela.
- Arquivos soltos chegam no Windows, no macOS e no Linux sob X11. **Sob Wayland a biblioteca de janelas (winit 0.30) não informa arquivos soltos**; `drop_zone::can_drop_files()` é então falso e a área só convida a um clique.
- Os caminhos soltos são lidos quando a ação de soltar termina: itens que não existem mais são ignorados e diretórios são marcados como tal.

## Envio com validação

`upload::Controller` guarda os arquivos aceitos e confere cada adição com os tipos aceitos, um limite de tamanho e um número máximo de arquivos; diretórios são sempre recusados. `upload::file_upload` mostra a área, as recusas com seus motivos e os arquivos aceitos com tamanhos, prévias de imagem e botões de remover.

```rust
use iced::Task;
use oito2_iced::l10n::Localizations;
use oito2_iced_files::file::TypeGroup;
use oito2_iced_files::upload::{self, Controller};

#[derive(Debug, Clone)]
enum Message {
    Upload(upload::Event),
}

struct App {
    images: Controller,
}

impl App {
    fn new() -> Self {
        let images = TypeGroup::new()
            .label("PNG, JPEG, WebP")
            .extensions(["png", "jpg", "jpeg", "webp"])
            .mime_types(["image/png", "image/jpeg", "image/webp"]);

        Self {
            images: Controller::new()
                .accept([images])
                .max_files(4)
                .max_file_size(5 * 1000 * 1000),
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Upload(event) => self.images.update(event).map(Message::Upload),
        }
    }

    fn view(&self) -> oito2_iced::Element<'_, Message> {
        upload::file_upload(&self.images, &Localizations::en())
            .on_event(Message::Upload)
            .into()
    }
}
```

`Controller::update` aplica cada evento e retorna a task do seletor de arquivos quando a área é ativada. `accepted()` e `rejected()` dão as listas atuais; `reason(&rejected, &l10n)`, a mensagem localizada de uma recusa.

## Salvar um arquivo

`download::save(file_name, bytes)` pergunta onde salvar com o diálogo "Salvar como" e grava os bytes; entrega `Ok(true)` depois de gravar, `Ok(false)` quando cancelado, ou um `Error`. `download::download_trigger(&l10n)` é um botão com contorno e um ícone de download; marque-o com `busy(true)` enquanto salva, o que mostra um spinner e ignora cliques.

```rust
Message::Download => {
    self.busy = true;
    download::save("report.txt", self.report.clone().into_bytes()).map(Message::Saved)
}
Message::Saved(_result) => {
    self.busy = false;
    Task::none()
}
```

## Notas por plataforma

| Plataforma | Seletor e "Salvar como" | Soltar arquivos |
|---|---|---|
| Linux (X11) | Portal XDG do desktop | Sim |
| Linux (Wayland) | Portal XDG do desktop | Não (winit 0.30) |
| Windows | Diálogo nativo | Sim |
| macOS | Diálogo nativo | Sim |

No Linux os diálogos precisam de um `xdg-desktop-portal` em execução com um backend para o desktop; veja [Problemas comuns](../troubleshooting/common-issues.md).
