🌐 [English](../../../en/guides/recipes/packaging.md) | **Português** | 🏠 [Índice](../../index.md)

---

# Empacotar uma Aplicação

Como uma aplicação construída sobre o `oito2_iced` é empacotada para Linux como `.deb`, `.rpm` e Flatpak, com o desktop entry, o metainfo AppStream e o ícone.

> **Estes são modelos.** O `oito2_iced` é uma biblioteca: nem o projeto nem a sua CI geram pacotes, e nada nesta página é testado pela CI. Substitua cada marcador — o crate `my-app`, o binário `my-app`, o id da aplicação `io.github.example.MyApp`, nomes, URLs, versões — e confira a documentação atual de cada ferramenta antes de confiar numa chave ou opção.

Versões das ferramentas conferidas quando este guia foi escrito: `cargo-deb` 3.8, `cargo-generate-rpm` 0.21, runtime Flatpak `org.freedesktop.Platform` 25.08.

## Arquivos que o pacote instala

| Arquivo | Instalado em | Função |
|---|---|---|
| Binário | `/usr/bin/my-app` | A aplicação. |
| Desktop entry | `/usr/share/applications/io.github.example.MyApp.desktop` | Item de menu e lançador. |
| Metainfo AppStream | `/usr/share/metainfo/io.github.example.MyApp.metainfo.xml` | Descrição para as centrais de software. |
| Ícone | `/usr/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg` | Ícone do menu, do lançador e da janela. |

Guarde os arquivos-fonte numa pasta do seu crate, por exemplo `packaging/`. O **id da aplicação** dá nome aos três arquivos de metadados e precisa coincidir com o id de aplicação Wayland da janela, que uma aplicação Iced define nas configurações de janela no Linux:

```rust
#[cfg(target_os = "linux")]
fn platform_specific() -> iced::window::settings::PlatformSpecific {
    iced::window::settings::PlatformSpecific {
        application_id: "io.github.example.MyApp".into(),
        ..Default::default()
    }
}
```

O showcase OITO2 faz o mesmo em [`apps/showcase/src/main.rs`](../../../../apps/showcase/src/main.rs). As fontes OITO2 são embutidas no binário pelo `oito2_iced`, então o pacote não precisa de dependência de fontes.

## Desktop entry

`packaging/io.github.example.MyApp.desktop`:

```ini
[Desktop Entry]
Type=Application
Name=My App
Comment=One-line description of My App
Exec=my-app
Icon=io.github.example.MyApp
Terminal=false
Categories=Utility;
StartupWMClass=io.github.example.MyApp
```

Valide com `desktop-file-validate packaging/io.github.example.MyApp.desktop` (pacote `desktop-file-utils`). A [receita de início automático](autostart.md) grava uma entrada menor no mesmo formato.

## Metainfo AppStream

`packaging/io.github.example.MyApp.metainfo.xml`:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<component type="desktop-application">
  <id>io.github.example.MyApp</id>
  <name>My App</name>
  <summary>One-line description of My App</summary>
  <metadata_license>CC0-1.0</metadata_license>
  <project_license>GPL-3.0-or-later</project_license>
  <developer id="io.github.example">
    <name>Example Developer</name>
  </developer>
  <description>
    <p>A paragraph that describes what My App does.</p>
  </description>
  <launchable type="desktop-id">io.github.example.MyApp.desktop</launchable>
  <url type="homepage">https://github.com/example/my-app</url>
  <content_rating type="oars-1.1"/>
  <releases>
    <release version="1.0.0" date="2026-01-01"/>
  </releases>
</component>
```

Valide com `appstreamcli validate packaging/io.github.example.MyApp.metainfo.xml` (pacote `appstream`). O Flathub também exige capturas de tela e cores de marca; veja as diretrizes de submissão dele.

## Ícone

Distribua um SVG escalável em `share/icons/hicolor/scalable/apps/<id da aplicação>.svg`. Acrescente tamanhos PNG (por exemplo `256x256/apps/<id da aplicação>.png`) para desktops que não desenham ícones SVG.

## `.deb` com cargo-deb

Instale a ferramenta com `cargo install cargo-deb`. Acrescente ao `Cargo.toml` do crate da aplicação:

```toml
[package.metadata.deb]
maintainer = "Example Developer <dev@example.com>"
copyright = "2026, Example Developer"
license-file = ["LICENSE", "0"]
extended-description = "A longer description of My App."
section = "utils"
priority = "optional"
depends = "$auto"
assets = [
    # cargo-deb replaces target/release with the real target directory.
    ["target/release/my-app", "usr/bin/", "755"],
    ["packaging/io.github.example.MyApp.desktop", "usr/share/applications/", "644"],
    ["packaging/io.github.example.MyApp.metainfo.xml", "usr/share/metainfo/", "644"],
    ["packaging/io.github.example.MyApp.svg", "usr/share/icons/hicolor/scalable/apps/", "644"],
]
```

```bash
cargo deb -p my-app
```

O pacote é gravado em `target/debian/my-app_<versão>-1_<arquitetura>.deb`. Mantenha o prefixo `target/release/` em `assets` como está: o `cargo-deb` o resolve sozinho, inclusive em workspaces.

`$auto` calcula as dependências a partir das bibliotecas ligadas ao binário. Bibliotecas que o sistema de janelas do Iced carrega em tempo de execução (como a `libxkbcommon` e os drivers Vulkan/OpenGL) não são ligadas, então não são detectadas: teste o pacote numa instalação limpa de cada distribuição-alvo e acrescente o que faltar em `depends`.

## `.rpm` com cargo-generate-rpm

Instale a ferramenta com `cargo install cargo-generate-rpm`. Ela não compila o binário, então compile e remova os símbolos antes. Acrescente ao `Cargo.toml` do crate da aplicação:

```toml
[package.metadata.generate-rpm]
assets = [
    { source = "target/release/my-app", dest = "/usr/bin/my-app", mode = "755" },
    { source = "packaging/io.github.example.MyApp.desktop", dest = "/usr/share/applications/io.github.example.MyApp.desktop", mode = "644" },
    { source = "packaging/io.github.example.MyApp.metainfo.xml", dest = "/usr/share/metainfo/io.github.example.MyApp.metainfo.xml", mode = "644" },
    { source = "packaging/io.github.example.MyApp.svg", dest = "/usr/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg", mode = "644" },
]
```

```bash
cargo build --release -p my-app
strip -s target/release/my-app
cargo generate-rpm -p my-app
```

O pacote é gravado em `target/generate-rpm/`. Nome, versão, licença, resumo e URL vêm de `[package]`. As dependências automáticas têm o mesmo limite do `cargo-deb`; acrescente as que faltarem em `[package.metadata.generate-rpm.requires]`.

## Flatpak

As compilações Flatpak rodam offline, então os crates são baixados antes: gere `cargo-sources.json` a partir do `Cargo.lock` com o `flatpak-cargo-generator.py` do [flatpak-builder-tools](https://github.com/flatpak/flatpak-builder-tools):

```bash
python3 flatpak-cargo-generator.py Cargo.lock -o cargo-sources.json
```

`io.github.example.MyApp.yml`, um esboço sobre o runtime freedesktop.org com a extensão Rust do SDK (uma aplicação que precisa de bibliotecas do GNOME usa `org.gnome.Platform`):

```yaml
id: io.github.example.MyApp
runtime: org.freedesktop.Platform
runtime-version: '25.08'
sdk: org.freedesktop.Sdk
sdk-extensions:
  - org.freedesktop.Sdk.Extension.rust-stable
command: my-app
finish-args:
  - --socket=wayland
  - --socket=fallback-x11
  - --share=ipc
  - --device=dri
  # Only with the notifications recipe:
  - --talk-name=org.freedesktop.Notifications
  # Only with the tray recipe:
  - --talk-name=org.kde.StatusNotifierWatcher
build-options:
  append-path: /usr/lib/sdk/rust-stable/bin
  env:
    CARGO_HOME: /run/build/my-app/cargo
modules:
  - name: my-app
    buildsystem: simple
    build-commands:
      - cargo --offline fetch --manifest-path Cargo.toml --verbose
      - cargo --offline build --release -p my-app
      - install -Dm755 target/release/my-app /app/bin/my-app
      - install -Dm644 packaging/io.github.example.MyApp.desktop /app/share/applications/io.github.example.MyApp.desktop
      - install -Dm644 packaging/io.github.example.MyApp.metainfo.xml /app/share/metainfo/io.github.example.MyApp.metainfo.xml
      - install -Dm644 packaging/io.github.example.MyApp.svg /app/share/icons/hicolor/scalable/apps/io.github.example.MyApp.svg
    sources:
      - type: dir
        path: .
      - cargo-sources.json
```

```bash
flatpak-builder --user --install --force-clean build-dir io.github.example.MyApp.yml
flatpak run io.github.example.MyApp
```

Use o `runtime-version` atual (confira com `flatpak remote-info flathub org.freedesktop.Platform//<branch>` ou no Flathub). Dentro do sandbox:

- o ícone da bandeja precisa de `.disable_dbus_name(true)` ao iniciar a bandeja do `ksni` ([receita da bandeja](tray.md));
- o início automático passa pelo portal Background, não por `~/.config/autostart` ([receita de início automático](autostart.md)).

## Desinstalação

O gerenciador de pacotes remove o que instalou (`sudo apt remove my-app`, `sudo dnf remove my-app`, `flatpak uninstall io.github.example.MyApp`). Os arquivos que a própria aplicação grava ficam — os diretórios de configuração e de dados e, com a receita de início automático, `~/.config/autostart/<id da aplicação>.desktop`; documente-os nas suas instruções de desinstalação.
