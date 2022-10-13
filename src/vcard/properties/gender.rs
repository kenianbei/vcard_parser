use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_GENDER;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn gender_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if let Some((sex, gender)) = str.split_once(';') {
        if !sex.is_empty() && sex != "M" && sex != "F" && sex != "N" && sex != "O" && sex != "U" {
            return Err(PropertyValueInvalid(PROPERTY_TYPE_GENDER.to_string()));
        }
        return Ok(ValueData::TextList(Vec::from([sex.to_string(), gender.to_string()])));
    }

    Err(PropertyValueInvalid(PROPERTY_TYPE_GENDER.to_string()))
}

pub fn gender_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::Any => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_GENDER))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::gender::gender_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn gender_valid() {
        let result = gender_get_value("", &Some(ValueKind::Uri));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
        let result = gender_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = gender_get_value("M;Guy", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = gender_get_value("O;They", &None);
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = gender_get_value(";None", &None);
        assert!(matches!(result, Ok(ValueData::TextList(_))));
    }
}
