# time_date_parser_Slipushkina
=======
# date_time_parser_Slipushkina

## Brief Description

crates.io - https://crates.io/crates/date_time_parser_slipushkina

`date_time_parser_Slipushkina` — це парсер, який розпізнає дати і час у двох найпоширеніших форматах:
1. **ISO формат:** `YYYY-MM-DD`
2. **Європейський формат:** `DD/MM/YYYY`
3. **24-hour формат:** `HH:MM` 
4. **12-hour формат:** `HH:MM AM/PM` 

## Technical Description


Парсер приймає рядок з датою і визначає формат автоматично:
- Якщо містить `-` → використовується правило **DATE_ISO**
- Якщо містить `/` → використовується правило **DATE_EURO**
- Якщо містить `:` → використовується правило **TIME_24** 
- Якщо містить `AM` або `PM` → використовується правило **TIME_12** 

Використано:
- **`thiserror`** — для зручного опису помилок у бібліотеці (`lib.rs`)
- **`anyhow`** — для обробки помилок у модульних тестах (`tests/`)
- **`cargo fmt`** — для автоматичного форматування коду
- **`cargo clippy`** — для перевірки якості коду перед комітом
- **`cargo test`** — для перевірки правильності парсингу


Після розбору повертається структура:
```rust
DateSimple {
  year: i32,
  month: u32,
  day: u32
}

TimeSimple {
  hour: u32,
  minute: u32
}

WHITESPACE = _{ " " | "\t" | "\n" }

DIGIT = _{ '0'..'9' }

Назви місяців і днів не використовуються, тому просто числа
YEAR = @{ DIGIT{4} }         # YYYY
MONTH = @{ DIGIT{1,2} }      # 1..12
DAY = @{ DIGIT{1,2} }        # 1..31
HOUR_24 = @{ DIGIT{1,2} }    # 0..23
HOUR_12 = @{ DIGIT{1,2} }    # 1..12
MINUTE = @{ DIGIT{1,2} }     # 0..59

Дата у форматі ISO: YYYY-MM-DD
DATE_ISO = { YEAR ~ "-" ~ MONTH ~ "-" ~ DAY }

Дата у європейському форматі: DD/MM/YYYY
DATE_EURO = { DAY ~ "/" ~ MONTH ~ "/" ~ YEAR }

Час у 24-годинному форматі: HH:MM
TIME_24 = { HOUR_24 ~ ":" ~ MINUTE }

Час у 12-годинному форматі: HH:MM AM/PM
TIME_12 = { HOUR_12 ~ ":" ~ MINUTE ~ WHITESPACE? ~ ("AM" | "PM") }

Загальний рядок для парсера: автоматично визначає формат
DATETIME_STRING = _{ DATE_ISO | DATE_EURO | TIME_24 | TIME_12 }
