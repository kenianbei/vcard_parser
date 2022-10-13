use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_VERSION;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::VersionInvalid;

pub fn version_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if str != "4.0" {
        return Err(VersionInvalid(str.to_string()));
    }

    Ok(ValueData::Text(str.to_string()))
}

pub fn version_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_VERSION)))
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::version::version_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn version_valid() {
        let result = version_get_value("", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = version_get_value("3.0", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::VersionInvalid(_))));
        let result = version_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::VersionInvalid(_))));
        let result = version_get_value("4.0", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = version_get_value("4.0", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
    }
}
