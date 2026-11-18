🌐 [English](../../en/getting-started/prerequisites.md) | **Português** | 🏠 [Índice](../index.md)

---

# Pré-requisitos

O que uma máquina precisa para compilar um app com o oito2_iced, ou o showcase. O Linux é a plataforma principal; Windows e macOS são de melhor esforço: os crates compilam e passam nos testes nesses sistemas na CI, sem promessa de integração nativa.

## Rust

| Ferramenta | Versão | Por quê |
|---|---|---|
| [rustup](https://rustup.rs) | qualquer | Instala o toolchain que o repositório fixa |
| Rust | **1.99.0** neste repositório; **1.97** ou mais nova para apps | O `rust-toolchain.toml` fixa a 1.99.0 com `rustfmt` e `clippy`; os crates declaram `rust-version = "1.97"`, o mínimo que o `material-colors` 0.5 exige |

Dentro de um clone do repositório, o primeiro comando `cargo` instala o Rust 1.99.0 pelo rustup. Um app que usa os crates precisa do Rust 1.97 ou mais novo e do suporte à edição 2024 que vem com ele.

## Linux

A compilação precisa dos arquivos de desenvolvimento do `libxkbcommon` (tratamento de teclado da biblioteca de janelas) e da GLib (GIO, que lê as configurações do desktop). Estes são os pacotes que a CI instala no Ubuntu:

```bash
sudo apt install libxkbcommon-dev libglib2.0-dev
```

No Fedora, os mesmos arquivos vêm de:

```bash
sudo dnf install libxkbcommon-devel glib2-devel
```

Em tempo de execução, um app precisa de:

- uma sessão X11 ou Wayland;
- um driver Vulkan ou OpenGL para o renderizador padrão `wgpu`, ou o renderizador por software (`ICED_BACKEND=tiny-skia`) quando o caminho pela GPU falha;
- para os diálogos de seleção de arquivos e "Salvar como" do `oito2_iced_files`, um [XDG desktop portal](https://flatpak.github.io/xdg-desktop-portal/) em execução, com um backend para o desktop, que a maioria dos ambientes desktop instala por padrão.

## Windows

Melhor esforço. O Rust precisa das ferramentas de compilação MSVC que o rustup pede durante a instalação; nenhum outro pacote de sistema é necessário. A CI compila e testa o workspace no `windows-latest`.

## macOS

Melhor esforço. O Rust precisa das Xcode Command Line Tools (`xcode-select --install`); nenhum outro pacote de sistema é necessário. A CI compila e testa o workspace no `macos-latest`.

## Para quem contribui

As verificações do [guia de contribuição](../contribuindo.md#verificações) também precisam do `cargo-deny` e do toolchain 1.97:

```bash
cargo install --locked cargo-deny
rustup toolchain install 1.97 --profile minimal
```

Regenerar as fontes exige Python 3 com o [fontTools](https://github.com/fonttools/fonttools); veja [Ferramentas do repositório](../reference/repository-tools.md).
