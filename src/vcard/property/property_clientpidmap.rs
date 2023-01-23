use crate::constants::{Cardinality, ParameterName, PropertyName};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_clientpidmap::ValueClientPidMapData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueClientPidMap;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyClientPidMapData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyClientPidMapData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyClientPidMapData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyClientPidMapData {
    fn name(&self) -> &str {
        PropertyName::CLIENTPIDMAP
    }
}

impl HasParameters for PropertyClientPidMapData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([ParameterName::ANY])
    }

    fn get_parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, parameters: Vec<Parameter>) {
        self.parameters = parameters;
    }
}

impl HasValue for PropertyClientPidMapData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueClientPidMap(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyClientPidMapData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueClientPidMap(ValueClientPidMapData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyClientPidMapData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueClientPidMap(ValueClientPidMapData::try_from(value)?))?;

        Ok(property)
    }
}
