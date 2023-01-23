use std::fmt::{Display, Formatter};

use time::format_description::well_known::{Iso8601, Rfc2822, Rfc3339};
use time::{format_description, OffsetDateTime, PrimitiveDateTime, UtcOffset};

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueTimestampData {
    pub value: OffsetDateTime,
}

impl TryFrom<&str> for ValueTimestampData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if let Ok(time) = OffsetDateTime::parse(str, &Rfc3339) {
            return Ok(Self { value: time });
        }
        if let Ok(time) = OffsetDateTime::parse(str, &Rfc2822) {
            return Ok(Self { value: time });
        }
        if let Ok(time) = OffsetDateTime::parse(str, &Iso8601::DEFAULT) {
            return Ok(Self { value: time });
        }
        if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]").unwrap()) {
            return Ok(Self {
                value: datetime.assume_offset(UtcOffset::UTC),
            });
        }
        if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year]-[month]-[day]T[hour]:[minute]:[second]Z").unwrap()) {
            return Ok(Self {
                value: datetime.assume_offset(UtcOffset::UTC),
            });
        }
        if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year][month][day]T[hour][minute][second]").unwrap()) {
            return Ok(Self {
                value: datetime.assume_offset(UtcOffset::UTC),
            });
        }
        if let Ok(datetime) = PrimitiveDateTime::parse(str, &format_description::parse("[year][month][day]T[hour][minute][second]Z").unwrap()) {
            return Ok(Self {
                value: datetime.assume_offset(UtcOffset::UTC),
            });
        }
        Err(VcardError::ValueMalformed(str.to_string()))
    }
}

impl Default for ValueTimestampData {
    fn default() -> Self {
        Self { value: OffsetDateTime::now_utc() }
    }
}

impl Display for ValueTimestampData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Ok(string) = self.value.format(&Iso8601::DEFAULT) {
            write!(f, "{}", string)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

#[cfg(test)]
mod tests {
    use time::format_description::well_known::Iso8601;
    use time::OffsetDateTime;

    use crate::vcard::value::value_timestamp::ValueTimestampData;

    #[test]
    fn try_from() {
        assert!(ValueTimestampData::try_from("20000101T000000").is_ok());
        assert!(ValueTimestampData::try_from("20000101T000000Z").is_ok());
        assert!(ValueTimestampData::try_from("20000101T000000-0500").is_ok());
    }

    #[test]
    fn fmt() {
        let timestamp = OffsetDateTime::now_utc().format(&Iso8601::DEFAULT).unwrap();
        let data = ValueTimestampData::try_from(timestamp.as_str()).unwrap();
        assert_eq!(data.to_string(), timestamp)
    }
}
