use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_utcoffset::ValueUtcOffsetData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueUtcOffset;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterTzData {
    pub value: Value,
}

impl HasName for ParameterTzData {
    fn name(&self) -> &str {
        ParameterName::TZ
    }
}

impl HasValue for ParameterTzData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueUtcOffset(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterTzData {
    fn default() -> Self {
        Self {
            value: ValueUtcOffset(ValueUtcOffsetData::default()),
        }
    }
}

impl TryFrom<&str> for ParameterTzData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueUtcOffset(ValueUtcOffsetData::try_from(str)?),
        })
    }
}
