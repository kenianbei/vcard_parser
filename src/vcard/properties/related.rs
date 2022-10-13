use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_RELATED;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn related_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text && kind != &ValueKind::Uri {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    match Url::parse(str) {
        Ok(url) => Ok(ValueData::Uri(url.to_string())),
        Err(_) => Ok(ValueData::Text(str.to_string())),
    }
}

pub fn related_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
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
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_RELATED))),
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::related::related_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn related_valid() {
        let result = related_get_value("", &Some(ValueKind::Date));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = related_get_value("Please contact my assistant Jane Doe for any inquiries.", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = related_get_value("Please contact my assistant Jane Doe for any inquiries.", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = related_get_value("https://example.com/directory/jdoe.vcf", &None);
        assert!(matches!(result, Ok(ValueData::Uri(_))));
        let result = related_get_value("urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6", &Some(ValueKind::Uri));
        assert!(matches!(result, Ok(ValueData::Uri(_))));
    }
}
