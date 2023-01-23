use crate::constants::{Cardinality, ParameterName, PropertyGenderValues, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_textlist::ValueTextListData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueTextList;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyGenderData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyGenderData {
    fn cardinality(&self) -> &str {
        Cardinality::SINGLE
    }
}

impl HasGroup for PropertyGenderData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyGenderData {
    fn name(&self) -> &str {
        PropertyName::GENDER
    }
}

impl HasParameters for PropertyGenderData {
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

impl HasValue for PropertyGenderData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueTextList(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueTextList(_)) && value_type != ValueType::TEXT {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        if let ValueTextList(data) = &value {
            if data.value.is_empty() || data.value.len() > 2 {
                return Err(VcardError::ValueInvalid(value.to_string(), self.name().to_string()));
            }
            if let Some(string) = data.value.first() {
                if !string.is_empty() && !PropertyGenderValues::TYPES.contains(&string.to_uppercase().as_str()) {
                    return Err(VcardError::ValueInvalid(value.to_string(), self.name().to_string()));
                }
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyGenderData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueTextList(ValueTextListData::from(("M", ';'))),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyGenderData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueTextList(ValueTextListData::from((value, ';'))))?;

        Ok(property)
    }
}
