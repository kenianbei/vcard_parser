use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueText;
use crate::{HasName, HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct XNameParameterData {
    pub name: String,
    pub value: Value,
}

impl XNameParameterData {
    pub fn default(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: Value::from(ValueTextData::default()),
        }
    }
}

impl HasName for XNameParameterData {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasValue for XNameParameterData {
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

impl TryFrom<(&str, &str)> for XNameParameterData {
    type Error = VcardError;
    fn try_from((name, value): (&str, &str)) -> Result<Self, Self::Error> {
        Ok(Self {
            name: name.to_string(),
            value: ValueText(ValueTextData::from(value)),
        })
    }
}
