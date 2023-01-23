use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_textlist::ValueTextListData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueTextList;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterTypeData {
    pub value: Value,
}

impl HasName for ParameterTypeData {
    fn name(&self) -> &str {
        ParameterName::TYPE
    }
}

impl HasValue for ParameterTypeData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueTextList(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterTypeData {
    fn default() -> Self {
        Self {
            value: ValueTextList(ValueTextListData::default()),
        }
    }
}

impl TryFrom<&str> for ParameterTypeData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueTextList(ValueTextListData::from((str, ','))),
        })
    }
}
