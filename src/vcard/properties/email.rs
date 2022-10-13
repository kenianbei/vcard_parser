use regex::Regex;
use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_EMAIL;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn email_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text && kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    match Url::parse(str) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => {
            let regex = Regex::new(r"(?i)^[a-zA-Z0-9.!#$%&'*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$").unwrap();
            if !regex.is_match(str) {
                return Err(PropertyValueInvalid(PROPERTY_TYPE_EMAIL.to_string()));
            }
            Ok(ValueData::Text(str.to_string()))
        }
    }
}

pub fn email_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_EMAIL))),
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::email::email_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn email_valid() {
        let result = email_get_value("", &Some(ValueKind::Date));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = email_get_value("user@example.com", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = email_get_value("user@sub.example.com", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = email_get_value("mailto:user@example.com", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
    }
}
