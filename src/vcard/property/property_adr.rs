use crate::constants::{Cardinality, ParameterName, PropertyName, ValueType};
use crate::traits::{HasCardinality, HasGroup, HasName, HasParameters, HasValue};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_listcomponent::ValueListComponentData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueListComponent;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyAdrData {
    group: Option<String>,
    parameters: Vec<Parameter>,
    value: Value,
}

impl HasCardinality for PropertyAdrData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyAdrData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyAdrData {
    fn name(&self) -> &str {
        PropertyName::ADR
    }
}

impl HasParameters for PropertyAdrData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::ANY,
            ParameterName::CC,
            ParameterName::GEO,
            ParameterName::INDEX,
            ParameterName::LABEL,
            ParameterName::LANGUAGE,
            ParameterName::PID,
            ParameterName::PREF,
            ParameterName::TYPE,
            ParameterName::TZ,
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

impl HasValue for PropertyAdrData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueListComponent(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueListComponent(_)) && value_type != ValueType::TEXT {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        if let ValueListComponent(list) = &value {
            if list.value.len() != 7 {
                return Err(VcardError::ValueInvalid(value.to_string(), self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for PropertyAdrData {
    fn default() -> Self {
        Self {
            group: None,
            parameters: Vec::new(),
            value: ValueListComponent(ValueListComponentData {
                delimiter_child: ',',
                delimiter_parent: ';',
                value: Vec::from([
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                    Vec::from([String::new()]),
                ]),
            }),
        }
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>)> for PropertyAdrData {
    type Error = VcardError;
    fn try_from((group, value, parameters): (Option<String>, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self { group, ..Self::default() };

        property.add_parameters(parameters)?;
        property.set_value(ValueListComponent(ValueListComponentData::try_from((value, ';', ','))?))?;

        Ok(property)
    }
}
