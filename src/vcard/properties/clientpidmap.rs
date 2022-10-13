use url::Url;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_CLIENTPIDMAP;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;
use crate::VcardError::PropertyValueInvalid;

pub fn clientpidmap_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
    }

    if let Some((f, u)) = str.split_once(';') {
        if let (Ok(f), Ok(u)) = (f.parse::<f32>(), Url::parse(u)) {
            return Ok(ValueData::ClientPidMap((f, u.to_string())));
        }
    }

    Err(PropertyValueInvalid(PROPERTY_TYPE_CLIENTPIDMAP.to_string()))
}

pub fn clientpidmap_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::Any => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_CLIENTPIDMAP))),
    }
}
