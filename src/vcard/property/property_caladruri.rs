use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_uri::ValueUriData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueUri;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyCalAdrUriData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyCalAdrUriData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyCalAdrUriData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyCalAdrUriData {
    fn name(&self) -> &str {
        PropertyName::CALADRURI
    }
}

impl HasParameters for PropertyCalAdrUriData {
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

impl HasValue for PropertyCalAdrUriData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueUri(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueUri(_)) && value_type != ValueType::URI {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyCalAdrUriData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueUri(ValueUriData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyCalAdrUriData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueUri(ValueUriData::try_from(value)?))?;

        Ok(property)
    }
}
