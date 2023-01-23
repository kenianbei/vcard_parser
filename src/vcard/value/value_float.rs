use std::fmt::{Display, Formatter};

use crate::VcardError;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ValueFloatData {
    pub value: f32,
}

impl From<f32> for ValueFloatData {
    fn from(value: f32) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for ValueFloatData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.parse::<f32>() {
            Ok(value) => Ok(Self { value }),
            Err(_) => Err(VcardError::ValueMalformed(str.to_string())),
        }
    }
}

impl Display for ValueFloatData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
