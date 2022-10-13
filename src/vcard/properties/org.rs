use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_ORG;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn org_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    let data: Vec<String> = str.split(';').map(|s| s.to_string()).collect();

    Ok(ValueData::TextList(data))
}

pub fn org_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::SortAs => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_ORG))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::org::org_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn org_valid() {
        let result = org_get_value("ABC, Inc.;North American Division;Marketing", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = org_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = org_get_value("ABC, Inc.;North American Division;Marketing", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = org_get_value("ABC, Inc.;North American Division;Marketing", &None);
        assert!(matches!(result, Ok(ValueData::TextList(_))));
    }
}
