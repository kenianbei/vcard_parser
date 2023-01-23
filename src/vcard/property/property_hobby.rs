use crate::constants::{Cardinality, ParameterName, PropertyHobbyValues, PropertyName};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::parameter::Parameter::ParameterLevel;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueText;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyHobbyData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyHobbyData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyHobbyData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyHobbyData {
    fn name(&self) -> &str {
        PropertyName::HOBBY
    }
}

impl HasParameters for PropertyHobbyData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::INDEX,
            ParameterName::LANGUAGE,
            ParameterName::LEVEL,
            ParameterName::PID,
            ParameterName::PREF,
            ParameterName::TYPE,
        ])
    }

    fn get_parameters(&self) -> Vec<Parameter> {
        self.parameters.clone()
    }

    fn set_parameters(&mut self, parameters: Vec<Parameter>) {
        self.parameters = parameters;
    }

    fn add_parameter(&mut self, parameter: Parameter) -> Result<(), VcardError> {
        let mut parameters = self.get_parameters();

        if let ParameterLevel(data) = &parameter {
            if let ValueText(text) = &data.value {
                if !PropertyHobbyValues::TYPES.contains(&text.value.to_uppercase().as_str()) {
                    return Err(VcardError::ValueInvalid(data.value.to_string(), self.name().to_string()));
                }
            }
        }

        if !self.allowed_parameters().contains(&parameter.name()) {
            return Err(VcardError::ParameterTypeNotAllowed(parameter.name().to_string(), self.name().to_string()));
        }

        parameters.push(parameter);
        self.set_parameters(parameters);

        Ok(())
    }
}

impl HasValue for PropertyHobbyData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyHobbyData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyHobbyData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueText(ValueTextData::from(value)))?;

        Ok(property)
    }
}
