use std::fmt::{Display, Formatter};

use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValuePidData {
    pub value: Vec<(i32, Option<i32>)>,
}

impl From<Vec<(i32, Option<i32>)>> for ValuePidData {
    fn from(value: Vec<(i32, Option<i32>)>) -> Self {
        Self { value }
    }
}

impl TryFrom<&str> for ValuePidData {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let mut value = Vec::new();

        for datum in str.split(';').map(|s| s.to_string()).collect::<Vec<String>>() {
            if let Some((a, b)) = datum.split_once('.') {
                if let (Ok(id), Ok(cid)) = (a.parse::<i32>(), b.parse::<i32>()) {
                    value.push((id, Some(cid)))
                }
            } else if let Ok(id) = datum.parse::<i32>() {
                value.push((id, None))
            }
        }

        if !value.is_empty() {
            return Ok(Self { value });
        }

        Err(VcardError::ValueMalformed(str.to_string()))
    }
}

impl Display for ValuePidData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.value
                .iter()
                .map(|(id, cid_option)| {
                    if let Some(cid) = cid_option {
                        format!("{}.{}", id, cid)
                    } else {
                        format!("{}", id)
                    }
                })
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}
