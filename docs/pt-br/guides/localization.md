🌐 [English](../../en/guides/localization.md) | **Português** | 🏠 [Índice](../index.md)

---

# Localização

Como os widgets do OITO2 obtêm seus textos, números e datas em inglês e em português do Brasil, e como se espelham para idiomas da direita para a esquerda.

## As strings

`oito2_iced::l10n::Localizations` guarda todos os textos que os widgets mostram — tooltips dos botões da janela, "Copiado", "Limpar", rótulos do calendário, mensagens de envio de arquivos — com as mesmas chaves do oito2_ui, conferidas com a fixture `l10n.json` dele.

| Construtor | Idioma |
|---|---|
| `Localizations::en()` | Inglês, também o fallback |
| `Localizations::pt_br()` | Português do Brasil |
| `Localizations::for_locale(tag)` | Português para qualquer tag `pt` (`pt-BR`, `pt_BR`, `pt`), inglês nos demais casos |

O oito2_iced não detecta o idioma do sistema: o app decide, por exemplo a partir das próprias configurações ou da variável de ambiente `LANG`, e passa o resultado aos widgets que mostram texto.

```rust
use oito2_iced::l10n::Localizations;
use oito2_iced::widget::{button, password};

let l10n = Localizations::for_locale("pt-BR");

let close = button::close_button(&l10n);
let field = password::password_field(&l10n, &self.password).on_input(Message::Password);
```

Entre os widgets que recebem um `&Localizations` estão os botões de fechar e de voltar, os campos de busca, senha, número, PIN e tags, as entradas de data e hora, o calendário, o seletor de cores, o botão e o campo de copiar, os toasts, as sheets, a paleta de comandos, a paginação, os passos, a árvore, o carrossel, o host do tour, os rótulos dos controles da janela (`window::Labels::new`), o bloco de código e os widgets de extras e de arquivos. As strings também são campos de `Localizations` (via `Deref`), como `l10n.minimize_window`, para textos seus que devam combinar.

## Números, tamanhos e tempos relativos

`oito2_iced::format::Format` formata para uma tag de idioma, com as regras do oito2_ui (conferidas com `format.json`):

```rust
use std::time::{Duration, SystemTime};
use oito2_iced::format::{ByteUnits, Format};

let format = Format::new("pt-BR");
let size = format.bytes(1_536_000, ByteUnits::Si, 1);
let count = format.compact_number(12_400.0);
let now = SystemTime::now();
let when = format.relative_time(now - Duration::from_secs(300), now);
```

## Datas e horas

`oito2_iced::date::{Date, Time}` são valores simples, sem fuso horário; converta-os a partir da biblioteca de datas que seu app usa. `Localizations` os formata e interpreta na forma compacta de cada idioma (`mm/dd/yyyy` em inglês, `dd/mm/aaaa` em português):

| Método | Faz |
|---|---|
| `compact_date(date)`, `parse_compact_date(text)` | Data compacta para texto e de volta |
| `parse_date(text, reference)` | Como `parse_compact_date`, expandindo anos de um ou dois dígitos em torno de `reference` |
| `time_of_day(time, use_24_hour)`, `parse_time(text)` | `h:mm AM` ou `HH:mm`, e os formatos que uma entrada de hora aceita |
| `month_name`, `month_year`, `narrow_weekdays`, `first_day_of_week` | Rótulos do calendário |

As entradas de data e hora, o calendário e o seletor de datas os usam; o "hoje" sempre vem do app.

## Direção de leitura

Texto em escritas da direita para a esquerda é moldado pela pilha de texto do Iced. O espelhamento do layout é explícito: widgets com direção aceitam `.direction(ReadingDirection::RightToLeft)` (de `oito2_iced::widget::helpers`), que inverte chevrons, setas, lados e as setas do teclado. `overlay::Placement::Start` e `End` seguem a mesma direção.

```rust
use oito2_iced::widget::helpers::ReadingDirection;
use oito2_iced::widget::rating;

let stars = rating::rating(self.score)
    .direction(ReadingDirection::RightToLeft)
    .on_change(Message::Score);
```

## Limites

- Só inglês e português do Brasil vêm incluídos; ainda não há como registrar outro idioma.
- Os formatos numéricos na entrada de número seguem apenas o inglês e o português.
- Não há mecanismo de plural ou de gramática além das funções de `Localizations` (`time_ago`, `result_count`, `page_of`…).
