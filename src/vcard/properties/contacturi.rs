use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_CONTACTURI;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn contacturi_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    match Url::parse(str) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => Err(VcardError::PropertyValueInvalid(PROPERTY_TYPE_CONTACTURI.to_string())),
    }
}

pub fn contacturi_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_CONTACTURI))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::contacturi::contacturi_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn contacturi_valid() {
        let result = contacturi_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = contacturi_get_value("", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = contacturi_get_value("https://contact.example.com", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
        let result = contacturi_get_value("mailto:contact@example.com", &None);
        assert!(matches!(result, Ok(ValueData::Uri(_))));
    }
}
