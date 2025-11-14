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
        return Err(ParseError::OutOfRange(format!(
            "month {} out of range",
            month
        )));
    }
    if !(1..=31).contains(&day) {
        return Err(ParseError::OutOfRange(format!("day {} out of range", day)));
    }
    if !(1..=9999).contains(&year) {
        return Err(ParseError::OutOfRange(format!(
            "year {} out of range",
            year
        )));
    }
    Ok(())
}

pub fn parse_time_24(s: &str) -> Result<TimeSimple, ParseError> {
    let parts: Vec<&str> = s.trim().split(':').collect();
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