use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_textlist::ValueTextListData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueTextList;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyOrgData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyOrgData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyOrgData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyOrgData {
    fn name(&self) -> &str {
        PropertyName::ORG
    }
}

impl HasParameters for PropertyOrgData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::ANY,
            ParameterName::INDEX,
            ParameterName::LANGUAGE,
            ParameterName::PID,
            ParameterName::PREF,
            ParameterName::SORTAS,
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

impl HasValue for PropertyOrgData {
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

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyOrgData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueTextList(ValueTextListData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyOrgData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueTextList(ValueTextListData::from((value, ';'))))?;

        Ok(property)
    }
}
