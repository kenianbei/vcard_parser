use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_date::ValueDateData;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::{ValueDate, ValueText};
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyAnniversaryData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyAnniversaryData {
    fn cardinality(&self) -> &str {
        Cardinality::SINGLE
    }
}

impl HasGroup for PropertyAnniversaryData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyAnniversaryData {
    fn name(&self) -> &str {
        PropertyName::ANNIVERSARY
    }
}

impl HasParameters for PropertyAnniversaryData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::ANY,
            ParameterName::CALSCALE,
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

impl HasValue for PropertyAnniversaryData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) && !matches!(value, ValueDate(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyAnniversaryData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyAnniversaryData {
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
