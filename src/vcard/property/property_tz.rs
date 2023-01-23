use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::value_uri::ValueUriData;
use crate::vcard::value::value_utcoffset::ValueUtcOffsetData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::{ValueText, ValueUri, ValueUtcOffset};
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyTzData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyTzData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyTzData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyTzData {
    fn name(&self) -> &str {
        PropertyName::TZ
    }
}

impl HasParameters for PropertyTzData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::ANY,
            ParameterName::INDEX,
            ParameterName::MEDIATYPE,
            ParameterName::PID,
            ParameterName::PREF,
            ParameterName::TYPE,
            ParameterName::VALUE,
        ])
    }

    fn get_parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, parameters: Vec<Parameter>) {
        self.parameters = parameters;
    }
}

impl HasValue for PropertyTzData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) && !matches!(value, ValueUtcOffset(_)) && !matches!(value, ValueUri(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueText(_)) && value_type != ValueType::TEXT {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
            if matches!(value, ValueUtcOffset(_)) && value_type != ValueType::UTC_OFFSET {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
            if matches!(value, ValueUri(_)) && value_type != ValueType::URI {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyTzData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueText(ValueTextData::from("America/Los_Angeles")),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyTzData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;

        if let Some(value_type) = property.has_value_type() {
            if value_type == ValueType::TEXT {
                property.set_value(ValueText(ValueTextData::from(value)))?;
            } else if value_type == ValueType::UTC_OFFSET {
                property.set_value(ValueUtcOffset(ValueUtcOffsetData::try_from(value)?))?;
            } else if value_type == ValueType::URI {
                property.set_value(ValueUri(ValueUriData::try_from(value)?))?;
            }
        } else {
            property.set_value(match ValueUriData::try_from(value) {
                Ok(data) => ValueUri(data),
                Err(_) => match ValueUtcOffsetData::try_from(value) {
                    Ok(data) => ValueUtcOffset(data),
                    Err(_) => ValueText(ValueTextData::from(value)),
                },
            })?;
        }

        Ok(property)
    }
}
