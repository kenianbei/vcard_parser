use crate::util::parse_time;
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

    if let Some(timestamp) = parse_time(str) {
        return Ok(ValueData::Integer(timestamp));
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
        let result = rev_get_value("20000101T000000Z", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = rev_get_value("", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = rev_get_value("2000-01-01", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = rev_get_value("20000101T000000", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
        let result = rev_get_value("20000101T000000Z", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
        let result = rev_get_value("20000101T000000-0000", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
        let result = rev_get_value("2000-01-01T00:00:00", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
        let result = rev_get_value("2000-01-01T00:00:00Z", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
        let result = rev_get_value("2000-01-01T00:00:00-00:00", &Some(ValueKind::TimeStamp));
        assert!(matches!(result, Ok(ValueData::Integer(946684800))));
    }
}
