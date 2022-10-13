use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_CALURI;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn caluri_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    match Url::parse(str) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => Err(VcardError::PropertyValueInvalid(PROPERTY_TYPE_CALURI.to_string())),
    }
}

pub fn caluri_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::MediaType => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_CALURI))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::caluri::caluri_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn caluri_valid() {
        let result = caluri_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = caluri_get_value("", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = caluri_get_value("https://cal.example.com/calA", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
        let result = caluri_get_value("ftp://ftp.example.com/calA.ics", &None);
        assert!(matches!(result, Ok(ValueData::Uri(_))));
    }
}
