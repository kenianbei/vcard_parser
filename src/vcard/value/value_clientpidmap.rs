use std::fmt::{Display, Formatter};

use url::Url;

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueClientPidMapData {
    pub id: i32,
    pub client: String,
}

impl TryFrom<&str> for ValueClientPidMapData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if let Some((a, b)) = str.split_once(';') {
            if let (Ok(id), Ok(url)) = (a.parse::<i32>(), Url::parse(b)) {
                return Ok(Self { id, client: url.to_string() });
            }
        }

        Err(VcardError::ValueMalformed(str.to_string()))
    }
}

impl Default for ValueClientPidMapData {
    fn default() -> Self {
        Self { id: 1, client: String::new() }
    }
}

impl Display for ValueClientPidMapData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{}", self.id, self.client)
    }
}
