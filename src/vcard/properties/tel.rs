use regex::Regex;
use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_TEL;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn tel_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text && kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    let trimmed = &str.replace(' ', "");

    match Url::parse(trimmed) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => {
            let regex = Regex::new(r"(?i)^((\+\d{1,3}(-|.)?\(?\d\)?(-|.)?\d{1,5})|(\(?\d{2,6}\)?))?(-|.)?(\d{3,4})(-|.)?(\d{4})((x|ext|;ext)\d{1,5})?$").unwrap();
            if !regex.is_match(trimmed) {
                return Err(PropertyValueInvalid(PROPERTY_TYPE_TEL.to_string()));
            }
            Ok(ValueData::Text(trimmed.to_string()))
        }
    }
}

pub fn tel_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::MediaType => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_TEL))),
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::tel::tel_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn tel_valid() {
        let result = tel_get_value("", &Some(ValueKind::Date));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = tel_get_value("5555", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = tel_get_value("5555;ext5555", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = tel_get_value("tel:+1-555-555-5555", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
        let result = tel_get_value("+1-555-555-5555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = tel_get_value("+15555555555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = tel_get_value("+1.555.555.5555;ext5555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = tel_get_value("555.5555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = tel_get_value("5555555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = tel_get_value("555.5555;ext5555", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
    }
}
