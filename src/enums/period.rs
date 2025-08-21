//! Represent period predefined by Google Trend.   

use chrono::{DateTime, Utc};
use serde::Serialize;

/// Simplified date field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Date(String);

impl Date {
    pub fn new(date: &DateTime<Utc>) -> Self {
        Self(date.format("%Y-%m-%d").to_string())
    }
}

/// Simplified date with hour field
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateHour(String);

impl DateHour {
    pub fn new(date: &DateTime<Utc>) -> Self {
        Self(date.format("%Y-%m-%dT%H").to_string())
    }
}

/// Google Trend period of time
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Period {
    Dates(Date, Date),
    DatesHour(DateHour, DateHour),
    Predefined(PredefinedPeriod),
}

impl Serialize for Period {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
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
    #[serde(rename = "now 1-H")]
    OneHour,

    #[serde(rename = "now 4-H")]
    FourHour,

    #[serde(rename = "now 1-d")]
    OneDay,

    #[serde(rename = "now 7-d")]
    SevenDay,

    #[serde(rename = "today 1-m")]
    ThirtyDay,

    #[serde(rename = "today 3-m")]
    NinetyDay,

    #[serde(rename = "today 12-m")]
    OneYear,

    #[serde(rename = "today 5-y")]
    FiveYear,

    #[serde(rename = "all")]
    Since2004,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date() {
        let now = Utc::now();
        let date = Date::new(&now);
        let period = Period::Dates(date.clone(), date.clone());

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%d %Y-%m-%d\"").to_string()
        );
    }

    #[test]
    fn test_date_hour() {
        let now = Utc::now();
        let date = DateHour::new(&now);
        let period = Period::DatesHour(date.clone(), date.clone());

        assert_eq!(
            serde_json::to_string(&period).unwrap(),
            now.format("\"%Y-%m-%dT%H %Y-%m-%dT%H\"").to_string()
        );
    }
}
