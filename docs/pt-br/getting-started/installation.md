🌐 [English](../../en/getting-started/installation.md) | **Português** | 🏠 [Índice](../index.md)

---

# Instalação

O oito2_iced é um conjunto de crates Rust, não um aplicativo: "instalá-lo" significa adicionar os crates a um app. O repositório também contém um app showcase, que você compila a partir do código. O projeto ainda não produz binários, instaladores nem pacotes, e os crates não estão no crates.io: todo crate está na versão `0.0.0` com `publish = false`.

## Plataformas suportadas

| SO | Status | Observações |
|---|---|---|
| Linux | Suportado (principal) | X11 e Wayland; todo ambiente desktop. Compilado e testado na CI no Ubuntu. |
| Windows | Melhor esforço | Compilado e testado na CI no `windows-latest`; chrome da janela no estilo Windows; sem promessa de integração nativa. |
| macOS | Melhor esforço | Compilado e testado na CI no `macos-latest`; chrome da janela no estilo macOS, sem os semáforos nativos; sem promessa de integração nativa. |

Instale primeiro os [pré-requisitos](./prerequisites.md) do seu sistema.

## Adicionar os crates a um app

Adicione os crates de que você precisa a partir do repositório Git, junto com o Iced 0.14:

```toml
[dependencies]
iced = "0.14"
oito2_iced = { git = "https://github.com/oito2/oito2_iced" }

# Crates opcionais, conforme a necessidade:
oito2_iced_settings = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_themes = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_syntax = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_extras = { git = "https://github.com/oito2/oito2_iced" }
oito2_iced_files = { git = "https://github.com/oito2/oito2_iced" }
```

O Cargo encontra cada crate dentro do repositório e registra o commit no `Cargo.lock`. Para fixar um commit explicitamente, adicione `rev = "<commit>"` a cada entrada.

| Crate | Adicione para |
|---|---|
| `oito2_iced` | O tema, os tokens, as fontes, os ícones, o foco, o overlay, todos os widgets principais e o chrome da janela |
| `oito2_iced_settings` | Acompanhar a cor de destaque do desktop, a preferência por claro ou escuro, o alto contraste e os botões da janela |
| `oito2_iced_themes` | Os 13 presets de cor |
| `oito2_iced_syntax` | Blocos de código e um realçador para o editor de texto do Iced |
| `oito2_iced_extras` | QR code, painel de assinatura, recortador de imagens e menu de navegação |
| `oito2_iced_files` | Área de soltar, envio de arquivos, seletor de arquivos e download |

### Renderizadores e features

O `oito2_iced` depende do Iced sem as features padrão dele, então o app escolhe os renderizadores. `iced = "0.14"` com as features padrão habilita tanto o renderizador `wgpu` quanto o fallback por software `tiny-skia`. O crate principal tem estas features próprias:

| Feature | Padrão | Habilita |
|---|---|---|
| `svg` | Sim | O catálogo de estilos de SVG e os glifos de ícones Lucide, usados por muitos widgets |
| `image` | Não | Fotos em avatares |
| `qr_code` | Não | O catálogo de estilos do widget de QR code do Iced |
| `markdown` | Não | O catálogo de estilos do widget de Markdown do Iced |

Veja [Crates e features](../reference/crates-and-features.md) para todos os crates.

## Linux

1. Instale os pacotes de compilação dos [pré-requisitos](./prerequisites.md#linux) (`libxkbcommon-dev` e `libglib2.0-dev` no Debian e no Ubuntu).
2. Adicione os crates como acima e compile o app com `cargo build`.

Os mesmos passos compilam o showcase:

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

Para uma compilação otimizada, rode `cargo build --release -p oito2_iced_showcase`; o binário é `target/release/oito2_iced_showcase`. Defina `ICED_BACKEND=tiny-skia` para forçar o renderizador por software.

## Windows

Melhor esforço: os passos são os mesmos do Linux, sem pacotes de sistema. No PowerShell:

```powershell
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

O binário de release é `target\release\oito2_iced_showcase.exe`.

## macOS

Melhor esforço: os passos são os mesmos do Linux, sem pacotes de sistema:

```bash
git clone https://github.com/oito2/oito2_iced.git
cd oito2_iced
cargo run -p oito2_iced_showcase
```

## O que a instalação deixa no sistema

Nada fora dos lugares que o Cargo sempre usa: o clone, o diretório de compilação `target/` dele e as dependências baixadas no cache do Cargo (`~/.cargo/registry` e `~/.cargo/git`). O showcase não instala entrada de desktop, ícone nem serviço, e não grava arquivos de configuração nem de dados.

Para remover o showcase, apague o clone. Para remover os crates de um app, apague as linhas deles do `Cargo.toml` do app; apagar os diretórios `oito2_iced-*` em `~/.cargo/git/checkouts` e `~/.cargo/git/db` recupera o espaço do checkout Git em cache.
