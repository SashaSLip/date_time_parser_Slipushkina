// Performed by Slipushkina Oleksandra

use thiserror::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct DateSimple {
    pub year: i32,
    pub month: u32,
    pub day: u32,
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

