/// Performed by Slipushkina Oleksandra

use anyhow::Result;
use date_time_parser_slipushkina::{DateSimple, TimeSimple, parse_date, parse_time};

#[test]
fn test_full_iso_and_time_24() -> Result<()> {
    let date = parse_date("2023-12-31")?;
    let time = parse_time("23:59")?;

    assert_eq!(date, DateSimple { year: 2023, month: 12, day: 31 });
    assert_eq!(time, TimeSimple { hour: 23, minute: 59 });
    Ok(())
}

#[test]
fn test_full_euro_and_time_12() -> Result<()> {
    let date = parse_date("01/01/2023")?;
    let time = parse_time("12:00 PM")?;

    assert_eq!(date, DateSimple { year: 2023, month: 1, day: 1 });
    assert_eq!(time, TimeSimple { hour: 12, minute: 0 });
    Ok(())
}

#[test]
fn test_leap_year_parsing() -> Result<()> {
    let date = parse_date("29/02/2024")?;
    assert_eq!(date, DateSimple { year: 2024, month: 2, day: 29 });

    let err = parse_date("29/02/2023").unwrap_err();
    assert_eq!(err.to_string(), "out of range: day 29 out of range for month 2");
    Ok(())
}

#[test]
fn test_invalid_date_format() -> Result<()> {
    let err = parse_date("2023.12.01").unwrap_err();
    assert_eq!(err.to_string(), "invalid format: unknown date format: 2023.12.01");
    Ok(())
}

#[test]
fn test_invalid_time_format() -> Result<()> {
    let err = parse_time("12:30 something").unwrap_err();
    assert_eq!(err.to_string(), "invalid format: unexpected space in time format: 12:30 something");
    Ok(())
}

#[test]
fn test_out_of_range_values() -> Result<()> {
    let err = parse_time("25:00").unwrap_err();
    assert_eq!(err.to_string(), "out of range: hour 25 out of range");

    let err = parse_time("13:00 PM").unwrap_err();
    assert_eq!(err.to_string(), "out of range: hour 13 out of range");

    let err = parse_date("12/13/2023").unwrap_err();
    assert_eq!(err.to_string(), "out of range: month 13 out of range");

    let err = parse_date("31/04/2023").unwrap_err();
    assert_eq!(err.to_string(), "out of range: day 31 out of range for month 4");

    Ok(())
}