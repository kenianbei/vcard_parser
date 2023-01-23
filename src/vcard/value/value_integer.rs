use std::fmt::{Display, Formatter};

use crate::VcardError;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueIntegerData {
    pub value: i32,
}

impl From<i32> for ValueIntegerData {
    fn from(value: i32) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for ValueIntegerData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.parse::<i32>() {
            Ok(value) => Ok(Self { value }),
            Err(_) => Err(VcardError::ValueMalformed(str.to_string())),
        }
    }
}

impl Display for ValueIntegerData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
