use std::fmt::{Display, Formatter};

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::parameter::Parameter;
use crate::vcard::values::data::ValueData;

pub const VALUE_KIND_BOOLEAN: &str = "BOOLEAN";
pub const VALUE_KIND_DATE: &str = "DATE";
pub const VALUE_KIND_DATEANDORTIME: &str = "DATE-AND-OR-TIME";
pub const VALUE_KIND_DATETIME: &str = "DATE-TIME";
pub const VALUE_KIND_FLOAT: &str = "FLOAT";
pub const VALUE_KIND_IANATOKEN: &str = "IANA-TOKEN";
pub const VALUE_KIND_INTEGER: &str = "INTEGER";
pub const VALUE_KIND_LANGUAGETAG: &str = "LANGUAGE-TAG";
pub const VALUE_KIND_TEXT: &str = "TEXT";
pub const VALUE_KIND_TIME: &str = "TIME";
pub const VALUE_KIND_TIMESTAMP: &str = "TIMESTAMP";
pub const VALUE_KIND_URI: &str = "URI";
pub const VALUE_KIND_UTCOFFSET: &str = "UTC-OFFSET";

/// A list of value types taken from the VALUE parameter. See [RFC 6350 Section 5.2](https://datatracker.ietf.org/doc/html/rfc6350#section-5.2).
#[derive(Clone, Eq, PartialEq)]
pub enum ValueKind {
    Boolean,
    Date,
    DateAndOrTime,
    DateTime,
    Float,
    IanaToken,
    Integer,
    LanguageTag,
    Text,
    Time,
    TimeStamp,
    Uri,
    UtcOffset,
}

impl ValueKind {
    /// Get the value type based on the VALUE parameter if present.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::parameter::Parameter;
    /// use vcard_parser::vcard::property::types::PropertyType;
    /// use vcard_parser::vcard::values::kind::ValueKind;
    ///
    /// let parameter = Parameter::try_from((&PropertyType::Tel, "VALUE=URI")).expect("Unable to create parameter.");
    /// let kind = ValueKind::get_kind_from_parameters(&[parameter]);
    /// assert!(matches!(kind, Some(_)));
    /// ```
    pub fn get_kind_from_parameters(parameters: &[Parameter]) -> Option<ValueKind> {
        match parameters.iter().cloned().find(|p| p.is_type(ParameterType::Value)) {
            None => None,
            Some(parameter) => match parameter.get_value().get_data() {
                ValueData::Text(s) => Self::get_kind_from_string(s),
                _ => None,
            },
        }
    }

    /// Get the value type based on the the parameter VALUE string value.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::parameter::Parameter;
    /// use vcard_parser::vcard::property::types::PropertyType;
    /// use vcard_parser::vcard::values::kind::ValueKind;
    ///
    /// let kind = ValueKind::get_kind_from_string("TEXT");
    /// assert!(matches!(kind, Some(ValueKind::Text)));
    /// ```
    pub fn get_kind_from_string(s: &str) -> Option<ValueKind> {
        match s.to_uppercase().as_str() {
            VALUE_KIND_BOOLEAN => Some(ValueKind::Boolean),
            VALUE_KIND_DATE => Some(ValueKind::Date),
            VALUE_KIND_DATEANDORTIME => Some(ValueKind::DateAndOrTime),
            VALUE_KIND_DATETIME => Some(ValueKind::DateTime),
            VALUE_KIND_FLOAT => Some(ValueKind::Float),
            VALUE_KIND_IANATOKEN => Some(ValueKind::IanaToken),
            VALUE_KIND_INTEGER => Some(ValueKind::Integer),
            VALUE_KIND_LANGUAGETAG => Some(ValueKind::LanguageTag),
            VALUE_KIND_TEXT => Some(ValueKind::Text),
            VALUE_KIND_TIME => Some(ValueKind::Time),
            VALUE_KIND_TIMESTAMP => Some(ValueKind::TimeStamp),
            VALUE_KIND_URI => Some(ValueKind::Uri),
            VALUE_KIND_UTCOFFSET => Some(ValueKind::UtcOffset),
            _ => None,
        }
    }
}

impl Display for ValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ValueKind::Boolean => VALUE_KIND_BOOLEAN,
            ValueKind::Date => VALUE_KIND_DATE,
            ValueKind::DateAndOrTime => VALUE_KIND_DATEANDORTIME,
            ValueKind::DateTime => VALUE_KIND_DATETIME,
            ValueKind::Float => VALUE_KIND_FLOAT,
            ValueKind::IanaToken => VALUE_KIND_IANATOKEN,
            ValueKind::Integer => VALUE_KIND_INTEGER,
            ValueKind::LanguageTag => VALUE_KIND_LANGUAGETAG,
            ValueKind::Text => VALUE_KIND_TEXT,
            ValueKind::Time => VALUE_KIND_TIME,
            ValueKind::TimeStamp => VALUE_KIND_TIMESTAMP,
            ValueKind::Uri => VALUE_KIND_URI,
            ValueKind::UtcOffset => VALUE_KIND_UTCOFFSET,
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::values::kind::ValueKind;

    #[test]
    pub fn case() {
        assert!(matches!(ValueKind::get_kind_from_string("boolean").unwrap(), ValueKind::Boolean));
        assert!(matches!(ValueKind::get_kind_from_string("TExt").unwrap(), ValueKind::Text));
        assert!(matches!(ValueKind::get_kind_from_string("LANGUAGE-TAG").unwrap(), ValueKind::LanguageTag));
    }

    #[test]
    pub fn display() {
        assert_eq!(ValueKind::Boolean.to_string().as_str(), "BOOLEAN");
    }
}
