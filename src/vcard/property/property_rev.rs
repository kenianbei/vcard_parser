use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_timestamp::ValueTimestampData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueTimestamp;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyRevData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyRevData {
    fn cardinality(&self) -> &str {
        Cardinality::SINGLE
    }
}

impl HasGroup for PropertyRevData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyRevData {
    fn name(&self) -> &str {
        PropertyName::REV
    }
}

impl HasParameters for PropertyRevData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ANY,
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

impl HasValue for PropertyRevData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueTimestamp(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueTimestamp(_)) && value_type != ValueType::TIMESTAMP {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyRevData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueTimestamp(ValueTimestampData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyRevData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueTimestamp(ValueTimestampData::try_from(value)?))?;

        Ok(property)
    }
}
