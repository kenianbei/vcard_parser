use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_pid::ValuePidData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValuePid;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterPidData {
    pub value: Value,
}

impl HasName for ParameterPidData {
    fn name(&self) -> &str {
        ParameterName::PID
    }
}

impl HasValue for ParameterPidData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValuePid(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterPidData {
    fn default() -> Self {
        Self {
            value: ValuePid(ValuePidData::from(Vec::from([(1, None)]))),
        }
    }
}

impl TryFrom<&str> for ParameterPidData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValuePid(ValuePidData::try_from(str)?),
        })
    }
}
