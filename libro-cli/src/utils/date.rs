use crate::lib::errors::{LibroError, LibroResult};
use chrono::{Datelike, NaiveDate, Utc};

/// Parse a date string in various formats
pub fn parse_date(date_str: &str) -> LibroResult<NaiveDate> {
    // Try different date formats
    let formats = [
        "%Y-%m-%d", // 2023-12-01
        "%Y/%m/%d", // 2023/12/01
        "%d/%m/%Y", // 01/12/2023
        "%d-%m-%Y", // 01-12-2023
        "%m/%d/%Y", // 12/01/2023
        "%m-%d-%Y", // 12-01-2023
    ];

    for format in &formats {
        if let Ok(date) = NaiveDate::parse_from_str(date_str, format) {
            return Ok(date);
        }
    }

    Err(LibroError::invalid_input(format!(
        "Invalid date format: '{}'. Expected formats: YYYY-MM-DD, YYYY/MM/DD, DD/MM/YYYY, etc.",
        date_str
    )))
}

/// Format a date for display
pub fn format_date(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

/// Format a date for display with day name
#[allow(dead_code)]
pub fn format_date_with_day(date: &NaiveDate) -> String {
    date.format("%Y-%m-%d (%A)").to_string()
}

/// Get current date
pub fn current_date() -> NaiveDate {
    Utc::now().date_naive()
}

/// Get current year
#[allow(dead_code)]
pub fn current_year() -> i32 {
    Utc::now().year()
}

/// Validate that a date is not in the future
pub fn validate_date_not_future(date: &NaiveDate) -> LibroResult<()> {
    let today = current_date();
    if *date > today {
        Err(LibroError::validation(
            "Date cannot be in the future".to_string(),
        ))
    } else {
        Ok(())
    }
}

/// Parse a date string with validation
pub fn parse_and_validate_date(date_str: &str) -> LibroResult<NaiveDate> {
    let date = parse_date(date_str)?;
    validate_date_not_future(&date)?;
    Ok(date)
}

/// Get a user-friendly relative date description
pub fn relative_date_description(date: &NaiveDate) -> String {
    let today = current_date();
    let days_diff = (today - *date).num_days();

    match days_diff {
        0 => "Today".to_string(),
        1 => "Yesterday".to_string(),
        2..=7 => format!("{} days ago", days_diff),
        8..=30 => format!("{} weeks ago", days_diff / 7),
        31..=365 => format!("{} months ago", days_diff / 30),
        _ => format!("{} years ago", days_diff / 365),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date_formats() {
        assert!(parse_date("2023-12-01").is_ok());
        assert!(parse_date("2023/12/01").is_ok());
        assert!(parse_date("01/12/2023").is_ok());
        assert!(parse_date("01-12-2023").is_ok());
        assert!(parse_date("invalid").is_err());
    }

    #[test]
    fn test_format_date() {
        let date = NaiveDate::from_ymd_opt(2023, 12, 1).unwrap();
        assert_eq!(format_date(&date), "2023-12-01");
    }

    #[test]
    fn test_current_date() {
        let date = current_date();
        assert!(date.year() >= 2023);
    }

    #[test]
    fn test_validate_date_not_future() {
        let past_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        assert!(validate_date_not_future(&past_date).is_ok());

        let future_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        assert!(validate_date_not_future(&future_date).is_err());
    }
}
