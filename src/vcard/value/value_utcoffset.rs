use std::fmt::{Display, Formatter};

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueUtcOffsetData {
    pub value: String,
}

impl Default for ValueUtcOffsetData {
    fn default() -> Self {
        Self { value: String::from("+0000") }
    }
}

impl TryFrom<&str> for ValueUtcOffsetData {
    type Error = VcardError;

    // TODO: Add utcoffset validator.
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Ok(Self { value: str.to_string() })
    }
}

impl Display for ValueUtcOffsetData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::value::value_utcoffset::ValueUtcOffsetData;

    #[test]
    fn try_from() {
        assert!(ValueUtcOffsetData::try_from("+00:00").is_ok());
        assert!(ValueUtcOffsetData::try_from("-23:59").is_ok());
        assert!(ValueUtcOffsetData::try_from("+23:59").is_ok());
        // assert!(ValueUtcOffsetData::try_from("-24:00").is_err());
        // assert!(ValueUtcOffsetData::try_from("+24:00").is_err());
    }

    #[test]
    fn fmt() {
        assert_eq!(ValueUtcOffsetData::try_from("+00:00").unwrap().to_string(), "+00:00")
    }
}
