use crate::constants::{Cardinality, ParameterName, PropertyName};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::value_uri::ValueUriData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::{ValueText, ValueUri};
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyUidData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyUidData {
    fn cardinality(&self) -> &str {
        Cardinality::SINGLE
    }
}

impl HasGroup for PropertyUidData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyUidData {
    fn name(&self) -> &str {
        PropertyName::UID
    }
}

impl HasParameters for PropertyUidData {
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

impl HasValue for PropertyUidData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) && !matches!(value, ValueUri(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyUidData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyUidData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self::default();

        let value = match ValueUriData::try_from(value) {
            Ok(uri) => ValueUri(uri),
            Err(_) => ValueText(ValueTextData::from(value)),
        };

        property.group = group;
        property.add_parameters(parameters)?;
        property.set_value(value)?;

        Ok(property)
    }
}
