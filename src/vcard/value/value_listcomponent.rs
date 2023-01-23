use std::fmt::{Display, Formatter};

use crate::parse::encoding::escape;
use crate::vcard::value::value_textlist::ValueTextListData;
use crate::VcardError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueListComponentData {
    pub delimiter_child: char,
    pub delimiter_parent: char,
    pub value: Vec<Vec<String>>,
}

impl Default for ValueListComponentData {
    fn default() -> Self {
        Self {
            delimiter_child: ',',
            delimiter_parent: ';',
            value: Vec::new(),
        }
    }
}

impl TryFrom<(&str, char, char)> for ValueListComponentData {
    type Error = VcardError;
    fn try_from((str, delimiter_parent, delimiter_child): (&str, char, char)) -> Result<Self, Self::Error> {
        let mut value = Vec::new();

        for string in ValueTextListData::from((str, delimiter_parent)).value {
            value.push(ValueTextListData::from((string.as_str(), delimiter_child)).value);
        }

        Ok(ValueListComponentData {
            delimiter_child,
            delimiter_parent,
            value,
        })
    }
}

impl Display for ValueListComponentData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value.iter().map(|child| { child.iter().map(|s| { escape(s) }).collect::<Vec<String>>().join(self.delimiter_child.to_string().as_str()) }).collect::<Vec<String>>().join(self.delimiter_parent.to_string().as_str()))
    }
}
