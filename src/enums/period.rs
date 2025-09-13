//! Represent period predefined by Google Trend.   

use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;
use std::result;

use crate::error::{Error, Result};

/// Simplified date field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(String);

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self> {
        Ok(Date::from(
            &NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| Error::params_error("Invalid date"))?,
        ))
    }
}

impl From<&NaiveDate> for Date {
    fn from(value: &NaiveDate) -> Self {
        Self(value.format("%Y-%m-%d").to_string())
    }
}

/// Simplified date with hour field
///
/// Seems to be limited to the span of a week.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateHour(String);

impl DateHour {
    pub fn new(year: i32, month: u32, day: u32, hour: u32) -> Result<Self> {
        Ok(DateHour::from(
            &NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| Error::params_error("Invalid date"))?
                .and_hms_opt(hour, 0, 0)
                .ok_or_else(|| Error::params_error("Invalid time"))?,
        ))
    }
}

impl From<&NaiveDateTime> for DateHour {
    fn from(value: &NaiveDateTime) -> Self {
        Self(value.format("%Y-%m-%dT%H").to_string())
    }
}

/// Google Trend period of time
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Period {
    /// Dates
    Dates(Date, Date),

    /// Dates with specified hours.
    ///
    /// Might fail if the delta is too big. Max seems to be 7 days at September 2025.
    DatesHour(DateHour, DateHour),

    /// Google Trend predefined periods
    Predefined(PredefinedPeriod),
}

impl Serialize for Period {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Period::Dates(d1, d2) => serializer.serialize_str(&format!("{} {}", d1.0, d2.0)),
            Period::DatesHour(d1, d2) => serializer.serialize_str(&format!("{} {}", d1.0, d2.0)),
            Period::Predefined(p) => p.serialize(serializer),
        }
    }
}

/// Google Trend predefined periods
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PredefinedPeriod {
    /// One hour
    #[serde(rename = "now 1-H")]
    OneHour,

    /// Four hours
    #[serde(rename = "now 4-H")]
    FourHour,

    /// One day
    #[serde(rename = "now 1-d")]
    OneDay,

    /// Seven days
    #[serde(rename = "now 7-d")]
    SevenDay,

    /// Thirty days
    #[serde(rename = "today 1-m")]
    ThirtyDay,

    /// Ninety days
    #[serde(rename = "today 3-m")]
    NinetyDay,

    /// One year
    #[serde(rename = "today 12-m")]
    OneYear,

    /// Five years
    #[serde(rename = "today 5-y")]
    FiveYear,

    /// Since 2004
    #[serde(rename = "all")]
    Since2004,
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn test_date() {
        let now = Utc::now();
        let date = Date::from(&now.naive_utc().date());
        let period = Period::Dates(date.clone(), date);

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%d %Y-%m-%d\"").to_string()
        );
    }

    #[test]
    fn test_date_hour() {
        let now = Utc::now();
        let date = DateHour::from(&now.naive_utc());
        let period = Period::DatesHour(date.clone(), date);

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%dT%H %Y-%m-%dT%H\"").to_string()
        );
    }
}
