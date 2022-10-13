use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_ADR;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn adr_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    let data: Vec<String> = str.split(';').map(|s| s.to_string()).collect();
    if data.len() != 7 {
        return Err(PropertyValueInvalid(PROPERTY_TYPE_ADR.to_string()));
    }

    Ok(ValueData::TextList(data))
}

pub fn adr_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::Cc => Ok(()),
        ParameterType::Geo => Ok(()),
        ParameterType::Index => Ok(()),
        ParameterType::Label => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::Pid => Ok(()),
        ParameterType::Pref => Ok(()),
        ParameterType::Type => Ok(()),
        ParameterType::Tz => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_ADR))),
    }
}

#[cfg(test)]
mod tests {
    use std::matches;

    use crate::vcard::properties::adr::adr_get_value;
    use crate::vcard::values::data::ValueData;
    use crate::vcard::values::kind::ValueKind;
    use crate::VcardError;

    #[test]
    pub fn adr_valid() {
        let result = adr_get_value("", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = adr_get_value("1600 Pennsylvania Avenue NW;Washington;DC;20500;United States", &Some(ValueKind::Text));
        assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
        let result = adr_get_value(";;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States", &Some(ValueKind::Text));
        assert!(matches!(result, Ok(ValueData::TextList(_))));
        let result = adr_get_value(";;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States", &Some(ValueKind::Date));
        assert!(matches!(result, Err(VcardError::ValueKindNotAllowed(_, _))));
    }
}
