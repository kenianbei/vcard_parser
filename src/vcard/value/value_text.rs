use std::fmt::{Display, Formatter};

use crate::parse::encoding::{escape, unescape};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValueTextData {
    pub value: String,
}

impl From<&str> for ValueTextData {
    fn from(str: &str) -> Self {
        ValueTextData { value: unescape(str) }
    }
}

impl Display for ValueTextData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", escape(self.value.as_str()))
    }
}
