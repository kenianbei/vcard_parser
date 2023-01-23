use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_languagetag::ValueLanguageTagData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueLanguageTag;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterLanguageData {
    pub value: Value,
}

impl HasName for ParameterLanguageData {
    fn name(&self) -> &str {
        ParameterName::LANGUAGE
    }
}

impl HasValue for ParameterLanguageData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueLanguageTag(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterLanguageData {
    fn default() -> Self {
        Self {
            value: ValueLanguageTag(ValueLanguageTagData::default()),
        }
    }
}

impl TryFrom<&str> for ParameterLanguageData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: ValueLanguageTag(ValueLanguageTagData::try_from(str)?),
        })
    }
}
