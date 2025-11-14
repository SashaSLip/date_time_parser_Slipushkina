/// Performed by Slipushkina Oleksandra

use anyhow::Result;
use date_time_parser_Slipushkina::{DateSimple, TimeSimple, parse_date, parse_time};

#[test]
fn test_parse_iso() -> Result<()> {
    let d = parse_date("2005-04-12")?;
    assert_eq!(
        d,
        DateSimple {
            year: 2005,
            month: 4,
            day: 12
        }
    );
    Ok(())
}

#[test]
fn test_parse_euro() -> Result<()> {
    let d = parse_date("12/04/2005")?;
    assert_eq!(
        d,
        DateSimple {
            year: 2005,
            month: 4,
            day: 12
        }
    );
    Ok(())
}

#[test]
fn test_parse_time_24() -> Result<()> {
    let t = parse_time("14:30")?;
    assert_eq!(
        t,
        TimeSimple {
            hour: 14,
            minute: 30
        }
    );

    let err = parse_time("25:00").unwrap_err();
    assert_eq!(err.to_string(), "out of range: hour 25 out of range");
    Ok(())
}

#[test]
fn test_parse_time_12() -> Result<()> {
    let t_pm = parse_time("02:30 PM")?;
    assert_eq!(
        t_pm,
        TimeSimple {
            hour: 14,
            minute: 30
        }
    );

    let t_am = parse_time("12:00 AM")?;
    assert_eq!(
        t_am,
        TimeSimple {
            hour: 0,
            minute: 0
        }
    );

    let err = parse_time("13:00 PM").unwrap_err();
    assert_eq!(err.to_string(), "out of range: hour 13 out of range");
    Ok(())
}