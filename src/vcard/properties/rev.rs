use chrono::{DateTime, NaiveDateTime};

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_REV;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn rev_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Time && kind != &ValueKind::TimeStamp {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if let Ok(date) = DateTime::parse_from_rfc2822(str) {
        return Ok(ValueData::Integer(date.timestamp() as i32));
    }
    if let Ok(date) = DateTime::parse_from_rfc3339(str) {
        return Ok(ValueData::Integer(date.timestamp() as i32));
    }
    if let Ok(date) = NaiveDateTime::parse_from_str(str, "%Y%m%dT%H%M%S") {
        return Ok(ValueData::Integer(date.timestamp() as i32));
    }
    if let Ok(date) = NaiveDateTime::parse_from_str(str, "%Y%m%dT%H%M%SZ") {
        return Ok(ValueData::Integer(date.timestamp() as i32));
    }
    if let Ok(date) = NaiveDateTime::parse_from_str(str, "%Y%m%dT%H%M%S%#z") {
        return Ok(ValueData::Integer(date.timestamp() as i32));
    }

    Err(PropertyValueInvalid(PROPERTY_TYPE_REV.to_string()))
}

pub fn rev_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::Any => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_REV))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::rev::rev_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn rev_valid() {
        let result = rev_get_value("19951031T222710Z", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = rev_get_value("", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = rev_get_value("19961022T140000", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(_))));
        let result = rev_get_value("19961022T140000Z", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(_))));
        let result = rev_get_value("19961022T140000+05", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(_))));
        let result = rev_get_value("19961022T140000-0500", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(_))));
    }
}
