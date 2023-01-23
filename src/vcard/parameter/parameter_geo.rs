use crate::constants::ParameterName;
use crate::traits::HasName;
use crate::vcard::value::value_uri::ValueUriData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValueUri;
use crate::{HasValue, VcardError};

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterGeoData {
    pub value: Value,
}

impl HasName for ParameterGeoData {
    fn name(&self) -> &str {
        ParameterName::GEO
    }
}

impl HasValue for ParameterGeoData {
    fn get_value(&self) -> &Value {
        &self.value
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        if !matches!(value, ValueUri(_)) {
            return Err(VcardError::ValueNotAllowed(value.to_string(), self.name().to_string()));
        }

        self.value = value;

        Ok(())
    }
}

impl Default for ParameterGeoData {
    fn default() -> Self {
        Self {
            value: ValueUri(ValueUriData::default()),
        }
    }
}

impl TryFrom<&str> for ParameterGeoData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        // TODO: Remove trim when proper escaping is done.
        Ok(Self {
            value: ValueUri(ValueUriData::try_from(str.trim_matches(|c| c == '"'))?),
        })
    }
}
