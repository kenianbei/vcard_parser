use std::fmt::{Display, Formatter};

use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use time::{format_description, Date, OffsetDateTime};

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueDateData {
    pub day: u8,
    pub month: u8,
    pub year: i32,
}

impl TryFrom<&str> for ValueDateData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if let Ok(date) = Date::parse(str, &Rfc3339) {
            return Ok(Self {
                day: date.day(),
                month: date.month().into(),
                year: date.year(),
            });
        }
        if let Ok(date) = Date::parse(str, &Rfc2822) {
            return Ok(Self {
                day: date.day(),
                month: date.month().into(),
                year: date.year(),
            });
        }
        if let Ok(date) = Date::parse(str, &Iso8601::DEFAULT) {
            return Ok(Self {
                day: date.day(),
                month: date.month().into(),
                year: date.year(),
            });
        }
        if let Ok(date) = Date::parse(str, &format_description::parse("[year][month][day]").unwrap()) {
            return Ok(Self {
                day: date.day(),
                month: date.month().into(),
                year: date.year(),
            });
        }
        if let Ok(date) = Date::parse(str, &format_description::parse("[year]-[month]-[day]").unwrap()) {
            return Ok(Self {
                year: date.year(),
                month: date.month().into(),
                day: date.day(),
            });
        }
        Err(VcardError::ValueMalformed(str.to_string()))
    }
}

impl Default for ValueDateData {
    fn default() -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            day: now.day(),
            month: now.month().into(),
            year: now.year(),
        }
    }
}

impl Display for ValueDateData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{:02}-{:02}", self.year, self.month, self.day)
    }
}
