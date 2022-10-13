use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_SOUND;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn sound_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    match Url::parse(str) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => Err(VcardError::PropertyValueInvalid(PROPERTY_TYPE_SOUND.to_string())),
    }
}

pub fn sound_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::MediaType => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_SOUND))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::sound::sound_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn sound_valid() {
        let result = sound_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = sound_get_value("", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = sound_get_value("CID:JOHNQPUBLIC.part8.19960229T080000.xyzMail@example.com", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
        let result = sound_get_value("data:audio/basic;base64,MIICajCCAdOgAwIBAgICBEUwDQYJKoZIhAQEEBQAwdzELMAkGA1UEBhMCVVMxLDAqBgNVBAoTI05ldHNjYXBlIENvbW11bmljYXRpb25zIENvcnBvcmF0aW9uMRwwGgYDVQQLExNJbmZvcm1hdGlvbiBTeXN0", &None);
        assert!(matches!(result, Ok(ValueData::Uri(_))));
    }
}
