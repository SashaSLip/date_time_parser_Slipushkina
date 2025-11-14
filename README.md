# time_date_parser_Slipushkina
=======
# date_time_parser_Slipushkina

## Brief Description

`date_time_parser_Slipushkina` — це парсер, який розпізнає дати і час у двох найпоширеніших форматах:
1. **ISO формат:** `YYYY-MM-DD`
2. **Європейський формат:** `DD/MM/YYYY`
3. **24-hour format for time:** `HH:MM` 
4. **12-hour format for time:** `HH:MM AM/PM` 

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
