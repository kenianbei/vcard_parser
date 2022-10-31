use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::parameter::Parameter;
use crate::vcard::property::types::PropertyType;
use crate::vcard::values::data::ValueData;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

/// Stores value data in various formats.
pub mod data;

/// Helper module for determining value type from parsed input.
pub mod kind;

/// Stores value data for both properties and parameters.
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value {
    data: ValueData,
}

/// Stores and validates both property and parameter values. Values are immutable and must
/// be re-created in order to update them. Typically you wouldn't interact directly with value
/// structs, instead using [Property](super::Property) to create or update values.
///
/// # Examples
/// ```
/// use vcard_parser::vcard::parameter::types::ParameterType;
/// use vcard_parser::vcard::property::types::PropertyType;
/// use vcard_parser::vcard::values::Value;
///
/// // Get a default value for a property type.
/// let value = Value::from(&PropertyType::Version);
///
/// // Create a value for a property.
/// let value = Value::try_from((&PropertyType::Version, &vec![], "4.0"));
///
/// // Create a value for a parameter.
/// let value = Value::try_from((&ParameterType::Value, "TEXT"));
/// ```
impl Value {
    pub fn get_data(&self) -> &ValueData {
        &self.data
    }
}

impl From<&PropertyType> for Value {
    fn from(property_type: &PropertyType) -> Self {
        match property_type {
            PropertyType::Adr => Self {
                data: ValueData::TextList(";;;;;;".split(';').map(|s| s.to_string()).collect::<Vec<String>>()),
            },
            PropertyType::Anniversary => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::BDay => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::BirthPlace => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::CalAdrUri => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::CalUri => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Categories => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::ClientPidMap => Self {
                data: ValueData::ClientPidMap((0.0, String::new())),
            },
            PropertyType::ContactUri => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::DeathDate => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::DeathPlace => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Email => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Expertise => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::FbUrl => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Fn => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Gender => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Geo => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Hobby => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Impp => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Interest => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Key => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Kind => Self {
                data: ValueData::Text(String::from("individual")),
            },
            PropertyType::Lang => Self {
                data: ValueData::Text(String::from("en")),
            },
            PropertyType::Logo => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Member => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::NickName => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Note => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::N => Self {
                data: ValueData::TextList(";;;;".split(';').map(|s| s.to_string()).collect::<Vec<String>>()),
            },
            PropertyType::OrgDirectory => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Org => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Photo => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::ProdId => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Related => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Rev => Self {
                data: ValueData::Integer(0),
            },
            PropertyType::Role => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Sound => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Source => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Tel => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Title => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Tz => Self {
                data: ValueData::Text(String::new()),
            },
            PropertyType::Uid => Self {
                data: ValueData::Text(Uuid::new_v4().to_string()),
            },
            PropertyType::Url => Self {
                data: ValueData::Uri(String::new()),
            },
            PropertyType::Version => Self {
                data: ValueData::Text(String::from("4.0")),
            },
            PropertyType::Xml => Self {
                data: ValueData::Text(String::new()),
            },
        }
    }
}

impl TryFrom<(&PropertyType, &Vec<Parameter>, &str)> for Value {
    type Error = VcardError;
    fn try_from((property_type, parameters, str): (&PropertyType, &Vec<Parameter>, &str)) -> Result<Self, Self::Error> {
        let kind = ValueKind::get_kind_from_parameters(parameters);
        let data = ValueData::try_from((property_type, &kind, str))?;
        Ok(Value { data })
    }
}

impl TryFrom<(&ParameterType, &str)> for Value {
    type Error = VcardError;
    fn try_from((parameter_type, str): (&ParameterType, &str)) -> Result<Self, Self::Error> {
        let data = ValueData::try_from((parameter_type, str))?;
        Ok(Value { data })
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)
    }
}
