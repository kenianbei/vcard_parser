use chrono::{DateTime, Datelike, NaiveDate};

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::property::types::PROPERTY_TYPE_DEATHDATE;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

pub fn deathdate_get_value(str: &str, kind: &Option<ValueKind>) -> Result<ValueData, VcardError> {
    if let Some(kind) = kind {
        if kind != &ValueKind::Text && kind != &ValueKind::Date && kind != &ValueKind::DateTime && kind != &ValueKind::DateAndOrTime {
            return Err(VcardError::ValueKindNotAllowed(kind.to_string(), str.to_string()));
        }
    }

    if let Ok(date) = DateTime::parse_from_rfc3339(str) {
        return Ok(ValueData::Date((date.year(), date.month(), date.day())));
    }
    if let Ok(date) = NaiveDate::parse_from_str(str, "%Y%m%d") {
        return Ok(ValueData::Date((date.year(), date.month(), date.day())));
    }
    if let Ok(date) = NaiveDate::parse_from_str(str, "%Y-%m-%d") {
        return Ok(ValueData::Date((date.year(), date.month(), date.day())));
    }

    Ok(ValueData::Text(str.to_string()))
}

pub fn deathdate_allowed_parameter(parameter_type: &ParameterType) -> Result<(), VcardError> {
    match parameter_type {
        ParameterType::AltId => Ok(()),
        ParameterType::Any => Ok(()),
        ParameterType::CalScale => Ok(()),
        ParameterType::Language => Ok(()),
        ParameterType::Value => Ok(()),
        _ => Err(VcardError::ParameterTypeNotAllowed(parameter_type.to_string(), String::from(PROPERTY_TYPE_DEATHDATE))),
    }
}
