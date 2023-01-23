//! The value module represents a single value, as per [RFC 6350 Section 4](https://datatracker.ietf.org/doc/html/rfc6350#section-4)
//!
//! Values can be created using [`Value::try_from`], [`Value::from`] or directly via the [`Value`] enum variant and the respective data struct.
//!
//! # Examples
//!
//! ## Creating a new value.
//! ```
//! use vcard_parser::vcard::value::Value;
//! use vcard_parser::vcard::value::value_date::ValueDateData;
//! use vcard_parser::vcard::value::value_text::ValueTextData;
//!
//! let mut value = Value::try_from(("TEXT", "John Doe")).expect("Unable to parse value.");
//! let mut value = Value::try_from(("DATE", "2000-01-01")).expect("Unable to parse value.");
//!
//! let mut value = Value::from(ValueTextData::from("John Doe"));
//! let mut value = Value::from(ValueDateData::try_from("2000-01-01").expect("Unable to parse value."));
//! ```

use std::fmt::{Display, Formatter};

use crate::constants::ValueName;
use crate::vcard::value::value_boolean::ValueBooleanData;
use crate::vcard::value::value_clientpidmap::ValueClientPidMapData;
use crate::vcard::value::value_date::ValueDateData;
use crate::vcard::value::value_float::ValueFloatData;
use crate::vcard::value::value_integer::ValueIntegerData;
use crate::vcard::value::value_languagetag::ValueLanguageTagData;
use crate::vcard::value::value_listcomponent::ValueListComponentData;
use crate::vcard::value::value_pid::ValuePidData;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::value_textlist::ValueTextListData;
use crate::vcard::value::value_timestamp::ValueTimestampData;
use crate::vcard::value::value_uri::ValueUriData;
use crate::vcard::value::value_utcoffset::ValueUtcOffsetData;
use crate::vcard::value::Value::{ValueBoolean, ValueClientPidMap, ValueDate, ValueFloat, ValueInteger, ValueLanguageTag, ValueListComponent, ValuePid, ValueText, ValueTextList, ValueTimestamp, ValueUri, ValueUtcOffset};
use crate::VcardError;

pub mod value_boolean;
pub mod value_clientpidmap;
pub mod value_date;
pub mod value_float;
pub mod value_integer;
pub mod value_languagetag;
pub mod value_listcomponent;
pub mod value_pid;
pub mod value_text;
pub mod value_textlist;
pub mod value_timestamp;
pub mod value_uri;
pub mod value_utcoffset;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    /// Represents a boolean value, see [RFC 6350 4.4](https://datatracker.ietf.org/doc/html/rfc6350#section-4.4).
    ValueBoolean(ValueBooleanData),
    /// Represents a client pid, see: [RFC 6350 6.7.7](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.7).
    ValueClientPidMap(ValueClientPidMapData),
    /// Represents a date value, see [RFC 6350 4.3](https://datatracker.ietf.org/doc/html/rfc6350#section-4.3).
    ValueDate(ValueDateData),
    /// Represents a float number, see [RFC 6350 4.6](https://datatracker.ietf.org/doc/html/rfc6350#section-4.6).
    ValueFloat(ValueFloatData),
    /// Represents a an integer, see [RFC 6350 ](https://datatracker.ietf.org/doc/html/rfc6350#section-4.5).
    ValueInteger(ValueIntegerData),
    /// Represents a language tag, see [RFC 6350 4.8](https://datatracker.ietf.org/doc/html/rfc6350#section-4.8).
    ValueLanguageTag(ValueLanguageTagData),
    /// Represents a list of text lists, see [ADR](https://datatracker.ietf.org/doc/html/rfc6350#section-6.3.1) and [N](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.2) properties.
    ValueListComponent(ValueListComponentData),
    /// Represents a pid value, see [RFC 6350 5.5](https://datatracker.ietf.org/doc/html/rfc6350#section-5.5).
    ValuePid(ValuePidData),
    /// Represents a text value, see [RFC 6350 4.1](https://datatracker.ietf.org/doc/html/rfc6350#section-4.1).
    ValueText(ValueTextData),
    /// Represents a list of text values, see [RFC 6350 4.1](https://datatracker.ietf.org/doc/html/rfc6350#section-4.1).
    ValueTextList(ValueTextListData),
    /// Represents a timestamp, see [RFC 6350 4.3.5](https://datatracker.ietf.org/doc/html/rfc6350#section-4.3.5).
    ValueTimestamp(ValueTimestampData),
    /// Represents a uri, see [RFC 6350 4.2](https://datatracker.ietf.org/doc/html/rfc6350#section-4.2).
    ValueUri(ValueUriData),
    /// Represents a UTC offset, see [RFC 6350 4.7](https://datatracker.ietf.org/doc/html/rfc6350#section-4.7).
    ValueUtcOffset(ValueUtcOffsetData),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueBoolean(data) => data.fmt(f),
            ValueClientPidMap(data) => data.fmt(f),
            ValueDate(data) => data.fmt(f),
            ValueFloat(data) => data.fmt(f),
            ValueInteger(data) => data.fmt(f),
            ValueLanguageTag(data) => data.fmt(f),
            ValueListComponent(data) => data.fmt(f),
            ValuePid(data) => data.fmt(f),
            ValueText(data) => data.fmt(f),
            ValueTextList(data) => data.fmt(f),
            ValueTimestamp(data) => data.fmt(f),
            ValueUri(data) => data.fmt(f),
            ValueUtcOffset(data) => data.fmt(f),
        }
    }
}

impl TryFrom<(&str, &str)> for Value {
    type Error = VcardError;
    fn try_from((name, str): (&str, &str)) -> Result<Self, Self::Error> {
        match name.to_uppercase().as_str() {
            ValueName::BOOLEAN => Ok(ValueBoolean(ValueBooleanData::try_from(str)?)),
            ValueName::CLIENTPIDMAP => Ok(ValueClientPidMap(ValueClientPidMapData::try_from(str)?)),
            ValueName::DATE => Ok(ValueDate(ValueDateData::try_from(str)?)),
            ValueName::FLOAT => Ok(ValueFloat(ValueFloatData::try_from(str)?)),
            ValueName::INTEGER => Ok(ValueInteger(ValueIntegerData::try_from(str)?)),
            ValueName::LANGUAGE_TAG => Ok(ValueLanguageTag(ValueLanguageTagData::try_from(str)?)),
            ValueName::LISTCOMPONENT => Ok(ValueListComponent(ValueListComponentData::try_from((str, ';', ','))?)),
            ValueName::PID => Ok(ValuePid(ValuePidData::try_from(str)?)),
            ValueName::TEXT => Ok(ValueText(ValueTextData::from(str))),
            ValueName::TEXTLIST => Ok(ValueTextList(ValueTextListData::from((str, ',')))),
            ValueName::TIMESTAMP => Ok(ValueTimestamp(ValueTimestampData::try_from(str)?)),
            ValueName::URI => Ok(ValueUri(ValueUriData::try_from(str)?)),
            ValueName::UTCOFFSET => Ok(ValueUtcOffset(ValueUtcOffsetData::try_from(str)?)),
            _ => Err(VcardError::ValueNameUnknown(name.to_string())),
        }
    }
}

/// Convenience method for creating ValueBoolean values.
impl From<ValueBooleanData> for Value {
    fn from(data: ValueBooleanData) -> Self {
        ValueBoolean(data)
    }
}

/// Convenience method for creating ValueClientPidMap values.
impl From<ValueClientPidMapData> for Value {
    fn from(data: ValueClientPidMapData) -> Self {
        ValueClientPidMap(data)
    }
}

/// Convenience method for creating ValueDate values.
impl From<ValueDateData> for Value {
    fn from(data: ValueDateData) -> Self {
        ValueDate(data)
    }
}

/// Convenience method for creating ValueFloat values.
impl From<ValueFloatData> for Value {
    fn from(data: ValueFloatData) -> Self {
        ValueFloat(data)
    }
}

/// Convenience method for creating ValueInteger values.
impl From<ValueIntegerData> for Value {
    fn from(data: ValueIntegerData) -> Self {
        ValueInteger(data)
    }
}

/// Convenience method for creating ValueLanguageTag values.
impl From<ValueLanguageTagData> for Value {
    fn from(data: ValueLanguageTagData) -> Self {
        ValueLanguageTag(data)
    }
}

/// Convenience method for creating ValueListComponent values.
impl From<ValueListComponentData> for Value {
    fn from(data: ValueListComponentData) -> Self {
        ValueListComponent(data)
    }
}

/// Convenience method for creating ValuePid values.
impl From<ValuePidData> for Value {
    fn from(data: ValuePidData) -> Self {
        ValuePid(data)
    }
}

/// Convenience method for creating ValueText values.
impl From<ValueTextData> for Value {
    fn from(data: ValueTextData) -> Self {
        ValueText(data)
    }
}

/// Convenience method for creating ValueTextList values.
impl From<ValueTextListData> for Value {
    fn from(data: ValueTextListData) -> Self {
        ValueTextList(data)
    }
}

/// Convenience method for creating ValueTimestamp values.
impl From<ValueTimestampData> for Value {
    fn from(data: ValueTimestampData) -> Self {
        ValueTimestamp(data)
    }
}

/// Convenience method for creating ValueUri values.
impl From<ValueUriData> for Value {
    fn from(data: ValueUriData) -> Self {
        ValueUri(data)
    }
}

/// Convenience method for creating ValueUtcOffset values.
impl From<ValueUtcOffsetData> for Value {
    fn from(data: ValueUtcOffsetData) -> Self {
        ValueUtcOffset(data)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::value::value_listcomponent::ValueListComponentData;
    use crate::vcard::value::value_textlist::ValueTextListData;

    #[test]
    fn util_parse_textlist_value() {
        assert_eq!(ValueTextListData::from(("", ';')).to_string(), "");
        assert_eq!(ValueTextListData::from(("A", ';')).to_string(), "A");
        assert_eq!(ValueTextListData::from((";", ';')).to_string(), ";");
        assert_eq!(ValueTextListData::from(("FOO;", ';')).to_string(), "FOO;");
        assert_eq!(ValueTextListData::from((";BAR", ';')).to_string(), ";BAR");
        assert_eq!(ValueTextListData::from(("FOO;BAR", ';')).to_string(), "FOO;BAR");
        assert_eq!(ValueTextListData::from(("FOO;BAR;AGAIN", ';')).to_string(), "FOO;BAR;AGAIN");

        assert_eq!(ValueTextListData::from(("FOO\\;TEST;BAR", ';')).to_string(), "FOO\\;TEST;BAR");
        assert_eq!(ValueTextListData::from(("FOO\\;TEST;BAR\\;TEST", ';')).to_string(), "FOO\\;TEST;BAR\\;TEST");
    }

    #[test]
    fn util_parse_textlist_compound_value() {
        assert_eq!(ValueListComponentData::try_from(("", ';', ',')).unwrap().to_string(), "");
        assert_eq!(ValueListComponentData::try_from(("A", ';', ',')).unwrap().to_string(), "A");
        assert_eq!(ValueListComponentData::try_from((";", ';', ',')).unwrap().to_string(), ";");
        assert_eq!(ValueListComponentData::try_from(("FOO;", ';', ',')).unwrap().to_string(), "FOO;");
        assert_eq!(ValueListComponentData::try_from((";BAR", ';', ',')).unwrap().to_string(), ";BAR");
        assert_eq!(ValueListComponentData::try_from(("FOO;BAR", ';', ',')).unwrap().to_string(), "FOO;BAR");
        assert_eq!(ValueListComponentData::try_from(("FOO;BAR;AGAIN", ';', ',')).unwrap().to_string(), "FOO;BAR;AGAIN");

        assert_eq!(ValueListComponentData::try_from((",", ';', ',')).unwrap().to_string(), ",");
        assert_eq!(ValueListComponentData::try_from((",;", ';', ',')).unwrap().to_string(), ",;");
        assert_eq!(ValueListComponentData::try_from((",;,", ';', ',')).unwrap().to_string(), ",;,");
        assert_eq!(ValueListComponentData::try_from((";,", ';', ',')).unwrap().to_string(), ";,");
        assert_eq!(ValueListComponentData::try_from(("A,;", ';', ',')).unwrap().to_string(), "A,;");
        assert_eq!(ValueListComponentData::try_from((",B;", ';', ',')).unwrap().to_string(), ",B;");
        assert_eq!(ValueListComponentData::try_from(("A,B;", ';', ',')).unwrap().to_string(), "A,B;");
        assert_eq!(ValueListComponentData::try_from((";C,", ';', ',')).unwrap().to_string(), ";C,");
        assert_eq!(ValueListComponentData::try_from((";,D", ';', ',')).unwrap().to_string(), ";,D");
        assert_eq!(ValueListComponentData::try_from((";C,D", ';', ',')).unwrap().to_string(), ";C,D");
        assert_eq!(ValueListComponentData::try_from(("A,B;C", ';', ',')).unwrap().to_string(), "A,B;C");
        assert_eq!(ValueListComponentData::try_from(("A,B;C,D", ';', ',')).unwrap().to_string(), "A,B;C,D");
        assert_eq!(ValueListComponentData::try_from(("FOO,BAR;FOO,BAR", ';', ',')).unwrap().to_string(), "FOO,BAR;FOO,BAR");
    }
}
