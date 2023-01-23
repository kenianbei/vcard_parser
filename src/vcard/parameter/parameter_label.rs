use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueText;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterLabelData {
    pub value: Value,
}

impl HasName for ParameterLabelData {
    fn name(&self) -> &str {
        ParameterName::LABEL
    }
}

impl HasValue for ParameterLabelData {
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

impl Default for ParameterLabelData {
    fn default() -> Self {
        Self {
            value: ValueText(ValueTextData::default()),
        }
    }
}

impl TryFrom<&str> for ParameterLabelData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueText(ValueTextData::from(str)),
        })
    }
}
