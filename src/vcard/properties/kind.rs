use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_KIND;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn kind_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if str != "individual" && str != "group" && str != "org" && str != "location" {
        return Err(PropertyValueInvalid(PROPERTY_TYPE_KIND.to_string()));
    }

    Ok(ValueData::Text(str.to_string()))
}

pub fn kind_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::Any => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_KIND))),
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::properties::kind::kind_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn kind_valid() {
        let result = kind_get_value("individual", &Some(ValueKind::Date));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = kind_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = kind_get_value("individual", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = kind_get_value("individual", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = kind_get_value("group", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = kind_get_value("org", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
        let result = kind_get_value("location", &None);
        assert!(matches!(result, Ok(ValueData::Text(_))));
    }
}
