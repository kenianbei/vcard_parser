use crate::constants::{Cardinality, ParameterName, ValueType};
use crate::traits::HasGroup;
use crate::vcard::parameter::Parameter;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueText;
use crate::{HasCardinality, HasName, HasParameters, HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct PropertyXNameData {
    group: Option<String>,
    name: String,
    parameters: Vec<Parameter>,
    value: Value,
}

impl PropertyXNameData {
    pub fn default(name: &str) -> Self {
        Self {
            group: None,
            name: name.to_string(),
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl HasCardinality for PropertyXNameData {
    fn cardinality(&self) -> &str {
        Cardinality::MULTIPLE
    }
}

impl HasGroup for PropertyXNameData {
    fn group(&self) -> &Option<String> {
        &self.group
    }
}

impl HasName for PropertyXNameData {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasParameters for PropertyXNameData {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        Vec::from([
            ParameterName::ALTID,
            ParameterName::ANY,
            ParameterName::CALSCALE,
            ParameterName::CC,
            ParameterName::GEO,
            ParameterName::INDEX,
            ParameterName::LABEL,
            ParameterName::LANGUAGE,
            ParameterName::LEVEL,
            ParameterName::MEDIATYPE,
            ParameterName::PID,
            ParameterName::PREF,
            ParameterName::SORTAS,
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

impl HasValue for PropertyXNameData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let Some(value_type) = self.has_value_type() {
            if matches!(value, ValueText(_)) && value_type != ValueType::TEXT {
                return Err(VcardError::ValueMismatch(value.to_string(), value_type, self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl TryFrom<(Option<String>, &str, &str, Vec<Parameter>)> for PropertyXNameData {
    type Error = VcardError;
    fn try_from((group, name, value, parameters): (Option<String>, &str, &str, Vec<Parameter>)) -> Result<Self, Self::Error> {
        let mut property = Self {
            group,
            name: name.to_string(),
            parameters: Vec::new(),
            value: ValueText(ValueTextData::default()),
        };

        property.add_parameters(parameters)?;
        property.set_value(ValueText(ValueTextData::from(value)))?;

        Ok(property)
    }
}
