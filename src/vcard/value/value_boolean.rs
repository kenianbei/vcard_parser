use std::fmt::{Display, Formatter};

use crate::VcardError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueBooleanData {
    pub value: bool,
}

impl TryFrom<&str> for ValueBooleanData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.to_uppercase().as_str() {
            "TRUE" => Ok(Self { value: true }),
            "FALSE" => Ok(Self { value: false }),
            _ => Err(VcardError::ValueMalformed(str.to_string())),
        }
    }
}

impl Display for ValueBooleanData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
