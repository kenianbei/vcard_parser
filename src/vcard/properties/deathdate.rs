use crate::util::parse_date;
use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_DEATHDATE;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn deathdate_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text && kind != &ValueKind::Date && kind != &ValueKind::DateTime && kind != &ValueKind::DateAndOrTime {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if let Some(date) = parse_date(str) {
        return Ok(ValueData::Date(date));
    }

    Ok(ValueData::Text(str.to_string()))
}

pub fn deathdate_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::CalScale => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_DEATHDATE))),
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::deathdate::deathdate_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;

    #[test]
    pub fn deathdate_valid() {
        let result = deathdate_get_value("", &Some(ValueKind::Date));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = deathdate_get_value("2000-01-01", &Some(ValueKind::Date));
        assert!(matches!(result, Ok(ValueData::Date((2000, 1, 1)))));
        let result = deathdate_get_value("20000101", &Some(ValueKind::Date));
        assert!(matches!(result, Ok(ValueData::Date((2000, 1, 1)))));
        let result = deathdate_get_value("Sat, 01 Jan 2000 00:00:00 GMT", &Some(ValueKind::Date));
        assert!(matches!(result, Ok(ValueData::Date((2000, 1, 1)))));
        let result = deathdate_get_value("2000-01-01T00:00:00.000000000-00:00", &Some(ValueKind::Date));
        assert!(matches!(result, Ok(ValueData::Date((2000, 1, 1)))));
    }
}
