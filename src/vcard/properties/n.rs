use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_N;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn n_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    let data: Vec<String> = str.split(';').map(|s| s.to_string()).collect();
    if data.len() != 5 {
        return Err(PropertyValueInvalid(PROPERTY_TYPE_N.to_string()));
    }

    Ok(ValueData::TextList(data))
}

pub fn n_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::SortAs => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_N))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::n::n_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn n_valid() {
        let result = n_get_value("Public;John;Quinlan;Mr.;Esq.", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = n_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = n_get_value("John;Quinlan;Mr.;Esq.", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = n_get_value("Public;John;Quinlan;Mr.;Esq.", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = n_get_value(";;;;", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
    }
}
