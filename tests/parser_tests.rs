// Performed by Slipushkina Oleksandra

use anyhow::Result;
use date_time_parser_Slipushkina::{DateSimple, parse_date};

#[test]
fn test_parse_iso() -> Result<()> {
    let d = parse_date("2025-11-04")?;
    assert_eq!(
        d,
        DateSimple {
            year: 2025,
            month: 11,
            day: 4
        }
    );
    Ok(())
}

#[test]
fn test_parse_euro() -> Result<()> {
    let d = parse_date("04/11/2025")?;
    assert_eq!(
        d,
        DateSimple {
            year: 2025,
            month: 11,
            day: 4
        }
    );
    Ok(())
}
