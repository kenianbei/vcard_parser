use std::fmt::{Display, Formatter};

use url::Url;

use crate::VcardError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueUriData {
    pub value: String,
}

impl TryFrom<&str> for ValueUriData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match Url::parse(str) {
            Ok(url) => Ok(Self { value: url.to_string() }),
            Err(_) => Err(VcardError::ValueMalformed(str.to_string())),
        }
    }
}

impl Display for ValueUriData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
