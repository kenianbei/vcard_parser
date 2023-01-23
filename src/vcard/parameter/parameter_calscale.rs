use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_text::ValueTextData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueText;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterCalScaleData {
    pub value: Value,
}

impl HasName for ParameterCalScaleData {
    fn name(&self) -> &str {
        ParameterName::CALSCALE
    }
}

impl HasValue for ParameterCalScaleData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueText(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        if let ValueText(text) = &value {
            if text.value != "gregorian" {
                return Err(VcardError::ValueInvalid(value.to_string(), self.name().to_string()));
            }
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterCalScaleData {
    fn default() -> Self {
        Self {
            value: ValueText(ValueTextData::from("gregorian")),
        }
    }
}

impl TryFrom<&str> for ParameterCalScaleData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueText(ValueTextData::from(str)),
        })
    }
}
