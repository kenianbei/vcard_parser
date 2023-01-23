use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_integer::ValueIntegerData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueInteger;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterIndexData {
    pub value: Value,
}

impl HasName for ParameterIndexData {
    fn name(&self) -> &str {
        ParameterName::INDEX
    }
}

impl HasValue for ParameterIndexData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueInteger(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterIndexData {
    fn default() -> Self {
        Self {
            value: ValueInteger(ValueIntegerData::from(1)),
        }
    }
}

impl TryFrom<&str> for ParameterIndexData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueInteger(ValueIntegerData::try_from(str)?),
        })
    }
}
