use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_date::ValueDateData;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::{ValueDate, ValueText};
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyDeathDateData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyDeathDateData {
    fn cardinality(&self) -> &str {
        Cardinality::SINGLE
    }
}

impl HasGroup for PropertyDeathDateData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyDeathDateData {
    fn name(&self) -> &str {
        PropertyName::DEATHDATE
    }
}

impl HasParameters for PropertyDeathDateData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::CALSCALE,
            ParameterName::LANGUAGE,
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

impl HasValue for PropertyDeathDateData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) && !matches!(value, ValueDate(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueText(_)) && value_type != ValueType::TEXT {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
            if matches!(value, ValueDate(_)) && !(value_type == ValueType::DATE || value_type == ValueType::DATE_TIME || value_type == ValueType::DATE_AND_OR_TIME) {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyDeathDateData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyDeathDateData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;

        if let Some(value_type) = property.has_value_type() {
            if value_type == ValueType::TEXT {
                property.set_value(ValueText(ValueTextData::from(value)))?;
            } else if value_type == ValueType::DATE || value_type == ValueType::DATE_TIME || value_type == ValueType::DATE_AND_OR_TIME {
                property.set_value(ValueDate(ValueDateData::try_from(value)?))?;
            }
        } else {
            property.set_value(match ValueDateData::try_from(value) {
                Ok(data) => ValueDate(data),
                Err(_) => ValueText(ValueTextData::from(value)),
            })?;
        }

        Ok(property)
    }
}
