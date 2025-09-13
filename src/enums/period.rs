//! Represent period predefined by Google Trend.   

use std::result;
use chrono::{offset::LocalResult, DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};
use serde::Serialize;

use crate::error::{Result, Error};

/// Simplified date field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(String);

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Result<Self> {
        
        if LocalResult::None == Utc.with_ymd_and_hms(year, month, day, 0, 0, 0) {
            return Err(Error::params_error("Invalid date"));
        }

        Ok(Self(format!("{year:04}-{month:02}-{day:02}")))
    }
}

impl<Tz: TimeZone> From<&DateTime<Tz>> for Date {
    fn from(value: &DateTime<Tz>) -> Self {
        Self(value.date_naive().format("%Y-%m-%d").to_string())
    }
}

impl From<&NaiveDate> for Date {
    fn from(value: &NaiveDate) -> Self {
        Self(value.format("%Y-%m-%d").to_string())
    }
}

impl From<&NaiveDateTime> for Date {
    fn from(value: &NaiveDateTime) -> Self {
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
        
        if LocalResult::None == Utc.with_ymd_and_hms(year, month, day, hour, 0, 0) {
            return Err(Error::params_error("Invalid date or hour"));
        }

        Ok(Self(format!("{year:04}-{month:02}-{day:02}T{hour:02}")))
    }
}

impl<Tz: TimeZone> From<&DateTime<Tz>> for DateHour {
    fn from(value: &DateTime<Tz>) -> Self {
        Self(value.naive_utc().format("%Y-%m-%dT%H").to_string())
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
    use super::*;

    #[test]
    fn test_date() {
        let now = Utc::now();
        let date= Date::from(&now);
        let period = Period::Dates(date.clone(), date);

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%d %Y-%m-%d\"").to_string()
        );
    }

    #[test]
    fn test_date_hour() {
        let now = Utc::now();
        let date = DateHour::from(&now);
        let period = Period::DatesHour(date.clone(), date);

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%dT%H %Y-%m-%dT%H\"").to_string()
        );
    }
}
