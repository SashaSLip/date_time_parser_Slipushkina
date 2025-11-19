/// Performed by Slipushkina Oleksandra

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct DateSimple {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TimeSimple {
    pub hour: u8,
    pub minute: u8,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid format: {0}")]
    InvalidFormat(String),
    #[error("numeric error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("out of range: {0}")]
    OutOfRange(String),
}

pub fn parse_date(input: &str) -> Result<DateSimple, ParseError> {
    let s = input.trim();
    if s.contains('-') {
        parse_iso(s)
    } else if s.contains('/') {
        parse_euro(s)
    } else {
        Err(ParseError::InvalidFormat(format!(
            "unknown date format: {}",
            s
        )))
    }
}

pub fn parse_time(input: &str) -> Result<TimeSimple, ParseError> {
    let s = input.trim();

    if s.contains(" ") && !(s.ends_with("AM") || s.ends_with("PM")) {
        return Err(ParseError::InvalidFormat(format!(
            "unexpected space in time format: {}",
            s
        )));
    }

    if s.ends_with("AM") || s.ends_with("PM") {
        parse_time_12(s)
    } else {
        parse_time_24(s)
    }
}

fn parse_iso(s: &str) -> Result<DateSimple, ParseError> {
    let parts: Vec<&str> = s.split('-').collect();
    if parts.len() != 3 {
        return Err(ParseError::InvalidFormat(format!(
            "expected YYYY-MM-DD: {}",
            s
        )));
    }
    let year: i32 = parts[0].parse()?;
    let month: u32 = parts[1].parse()?;
    let day: u32 = parts[2].parse()?;
    validate_date(year, month, day)?;
    Ok(DateSimple { year, month, day })
}

fn parse_euro(s: &str) -> Result<DateSimple, ParseError> {
    let parts: Vec<&str> = s.split('/').collect();
    if parts.len() != 3 {
        return Err(ParseError::InvalidFormat(format!(
            "expected DD/MM/YYYY: {}",
            s
        )));
    }
    let day: u32 = parts[0].parse()?;
    let month: u32 = parts[1].parse()?;
    let year: i32 = parts[2].parse()?;
    validate_date(year, month, day)?;
    Ok(DateSimple { year, month, day })
}

fn validate_date(year: i32, month: u32, day: u32) -> Result<(), ParseError> {
    if !(1..=12).contains(&month) {
        return Err(ParseError::OutOfRange(format!("month {} out of range", month)));
    }
    if !(1..=9999).contains(&year) {
        return Err(ParseError::OutOfRange(format!("year {} out of range", year)));
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    };

    if day == 0 || day > max_day {
        return Err(ParseError::OutOfRange(format!(
            "day {} out of range for month {}",
            day, month
        )));
    }
    Ok(())
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn parse_time_24(s: &str) -> Result<TimeSimple, ParseError> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat(format!(
            "expected HH:MM 24-hour format: {}",
            s
        )));
    }
    let hour: u8 = parts[0].parse()?;
    let minute: u8 = parts[1].parse()?;
    if hour > 23 {
        return Err(ParseError::OutOfRange(format!("hour {} out of range", hour)));
    }
    if minute > 59 {
        return Err(ParseError::OutOfRange(format!("minute {} out of range", minute)));
    }
    Ok(TimeSimple { hour, minute })
}

pub fn parse_time_12(s: &str) -> Result<TimeSimple, ParseError> {
    let s = s.trim();
    let am_pm = if s.ends_with("AM") {
        "AM"
    } else if s.ends_with("PM") {
        "PM"
    } else {
        return Err(ParseError::InvalidFormat(format!(
            "missing AM/PM in 12-hour format: {}",
            s
        )));
    };

    let time_part = s.trim_end_matches(am_pm).trim();
    let parts: Vec<&str> = time_part.split(':').collect();
    if parts.len() != 2 {
        return Err(ParseError::InvalidFormat(format!(
            "expected HH:MM 12-hour format: {}",
            s
        )));
    }
    let mut hour: u8 = parts[0].parse()?;
    let minute: u8 = parts[1].parse()?;
    if hour == 0 || hour > 12 {
        return Err(ParseError::OutOfRange(format!("hour {} out of range", hour)));
    }
    if minute > 59 {
        return Err(ParseError::OutOfRange(format!("minute {} out of range", minute)));
    }

    if am_pm == "PM" && hour != 12 {
        hour += 12;
    }
    if am_pm == "AM" && hour == 12 {
        hour = 0;
    }

    Ok(TimeSimple { hour, minute })
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_parse_iso() -> Result<(), anyhow::Error> {
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
    fn test_parse_euro() -> Result<(), anyhow::Error> {
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
    fn test_parse_time_24() -> Result<(), anyhow::Error> {
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
    fn test_parse_time_12() -> Result<(), anyhow::Error> {
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

    #[test]
fn test_time_unexpected_space() {
    let err = parse_time("12:30 something").unwrap_err();
    assert_eq!(
        err.to_string(),
        "invalid format: unexpected space in time format: 12:30 something"
    );
}

    #[test]
    fn test_unknown_date_format() {
        let err = parse_date("2024.12.01").unwrap_err();
        assert_eq!(err.to_string(), "invalid format: unknown date format: 2024.12.01");
    }

    #[test]
    fn test_leap_year() -> Result<(), anyhow::Error> {
        let d = parse_date("29/02/2024")?;
        assert_eq!(
            d,
            DateSimple {
                year: 2024,
                month: 2,
                day: 29
            }
        );

        let err = parse_date("29/02/2023").unwrap_err();
        assert_eq!(err.to_string(), "out of range: day 29 out of range for month 2");
        Ok(())
    }
}