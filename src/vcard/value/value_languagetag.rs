use std::fmt::{Display, Formatter};

use language_tags::LanguageTag;

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueLanguageTagData {
    pub value: String,
}

impl Default for ValueLanguageTagData {
    fn default() -> Self {
        Self { value: String::from("en") }
    }
}

impl TryFrom<&str> for ValueLanguageTagData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match LanguageTag::parse(str) {
            Ok(tag) => Ok(Self { value: tag.to_string() }),
            Err(_) => Err(VcardError::ValueMalformed(str.to_string())),
        }
    }
}

impl Display for ValueLanguageTagData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
