use std::fmt::{Display, Formatter};

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::properties::adr::adr_get_value;
use crate::vcard::properties::anniversary::anniversary_get_value;
use crate::vcard::properties::bday::bday_get_value;
use crate::vcard::properties::birthplace::birthplace_get_value;
use crate::vcard::properties::caladruri::caladruri_get_value;
use crate::vcard::properties::caluri::caluri_get_value;
use crate::vcard::properties::categories::categories_get_value;
use crate::vcard::properties::clientpidmap::clientpidmap_get_value;
use crate::vcard::properties::contacturi::contacturi_get_value;
use crate::vcard::properties::deathdate::deathdate_get_value;
use crate::vcard::properties::deathplace::deathplace_get_value;
use crate::vcard::properties::email::email_get_value;
use crate::vcard::properties::expertise::expertise_get_value;
use crate::vcard::properties::fburl::fburl_get_value;
use crate::vcard::properties::gender::gender_get_value;
use crate::vcard::properties::geo::geo_get_value;
use crate::vcard::properties::hobby::hobby_get_value;
use crate::vcard::properties::impp::impp_get_value;
use crate::vcard::properties::interest::interest_get_value;
use crate::vcard::properties::key::key_get_value;
use crate::vcard::properties::kind::kind_get_value;
use crate::vcard::properties::lang::lang_get_value;
use crate::vcard::properties::logo::logo_get_value;
use crate::vcard::properties::member::member_get_value;
use crate::vcard::properties::n::n_get_value;
use crate::vcard::properties::nickname::nickname_get_value;
use crate::vcard::properties::note::note_get_value;
use crate::vcard::properties::org::org_get_value;
use crate::vcard::properties::orgdirectory::orgdirectory_get_value;
use crate::vcard::properties::photo::photo_get_value;
use crate::vcard::properties::prodid::prodid_get_value;
use crate::vcard::properties::r#fn::fn_get_value;
use crate::vcard::properties::related::related_get_value;
use crate::vcard::properties::rev::rev_get_value;
use crate::vcard::properties::role::role_get_value;
use crate::vcard::properties::sound::sound_get_value;
use crate::vcard::properties::source::source_get_value;
use crate::vcard::properties::tel::tel_get_value;
use crate::vcard::properties::title::title_get_value;
use crate::vcard::properties::tz::tz_get_value;
use crate::vcard::properties::uid::uid_get_value;
use crate::vcard::properties::url::url_get_value;
use crate::vcard::properties::version::version_get_value;
use crate::vcard::properties::xml::xml_get_value;
use crate::vcard::property::types::PropertyType;
use crate::vcard::values::kind::ValueKind;
use crate::VcardError;

/// Stores the value data as an enum variant for convenience of matching. As with [Value](super::Value),
/// you normally wouldn't directly use this struct. See [RFC 6350 Section 4](https://datatracker.ietf.org/doc/html/rfc6350#section-4).
///
/// # Examples
/// ```
/// use vcard_parser::vcard::values::data::ValueData;
///
/// let data = ValueData::TextList(vec![String::from("Work"), String::from("Work")]);
/// ```
#[derive(Clone, PartialEq)]
pub enum ValueData {
    /// Represents a boolean value, see [RFC 6350 4.4](https://datatracker.ietf.org/doc/html/rfc6350#section-4.4).
    Boolean(bool),
    /// Represents a client pid, see: [RFC 6350 6.7.7](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.7).
    ClientPidMap((f32, String)),
    /// Represents a date value, see [RFC 6350 4.3](https://datatracker.ietf.org/doc/html/rfc6350#section-4.3).
    Date((i32, u8, u8)),
    /// Represents a multiple data values, see [RFC 6350 4.3](https://datatracker.ietf.org/doc/html/rfc6350#section-4.3).
    DateList(Vec<(i32, u8, u8)>),
    /// Represents a float number, see [RFC 6350 4.6](https://datatracker.ietf.org/doc/html/rfc6350#section-4.6).
    Float(f32),
    /// Represents multiple float numbers, see [RFC 6350 4.6](https://datatracker.ietf.org/doc/html/rfc6350#section-4.6).
    FloatList(Vec<f32>),
    /// Represents a an integer, see [RFC 6350 ](https://datatracker.ietf.org/doc/html/rfc6350#section-4.5).
    Integer(i32),
    /// Represents a multiple integers, see [RFC 6350 ](https://datatracker.ietf.org/doc/html/rfc6350#section-4.5).
    IntegerList(Vec<i32>),
    /// Represents a language tag, see [RFC 6350 4.8](https://datatracker.ietf.org/doc/html/rfc6350#section-4.8).
    LanguageTag(String),
    /// Represents a text value, see [RFC 6350 4.1](https://datatracker.ietf.org/doc/html/rfc6350#section-4.1).
    Text(String),
    /// Represents a multiple text values, see [RFC 6350 4.1](https://datatracker.ietf.org/doc/html/rfc6350#section-4.1).
    TextList(Vec<String>),
    /// Represents a uri, see [RFC 6350 4.2](https://datatracker.ietf.org/doc/html/rfc6350#section-4.2).
    Uri(String),
    /// Represents a UTC offset, see [RFC 6350 4.7](https://datatracker.ietf.org/doc/html/rfc6350#section-4.7).
    UtcOffset(String),
}

impl Eq for ValueData {}

impl TryFrom<(&PropertyType, &Option<ValueKind>, &str)> for ValueData {
    type Error = VcardError;
    fn try_from((property_type, kind, str): (&PropertyType, &Option<ValueKind>, &str)) -> Result<Self, Self::Error> {
        Ok(match property_type {
            PropertyType::Adr => adr_get_value(str, kind)?,
            PropertyType::Anniversary => anniversary_get_value(str, kind)?,
            PropertyType::BDay => bday_get_value(str, kind)?,
            PropertyType::BirthPlace => birthplace_get_value(str, kind)?,
            PropertyType::CalAdrUri => caladruri_get_value(str, kind)?,
            PropertyType::CalUri => caluri_get_value(str, kind)?,
            PropertyType::Categories => categories_get_value(str, kind)?,
            PropertyType::ClientPidMap => clientpidmap_get_value(str, kind)?,
            PropertyType::ContactUri => contacturi_get_value(str, kind)?,
            PropertyType::DeathDate => deathdate_get_value(str, kind)?,
            PropertyType::DeathPlace => deathplace_get_value(str, kind)?,
            PropertyType::Email => email_get_value(str, kind)?,
            PropertyType::Expertise => expertise_get_value(str, kind)?,
            PropertyType::FbUrl => fburl_get_value(str, kind)?,
            PropertyType::Fn => fn_get_value(str, kind)?,
            PropertyType::Gender => gender_get_value(str, kind)?,
            PropertyType::Geo => geo_get_value(str, kind)?,
            PropertyType::Hobby => hobby_get_value(str, kind)?,
            PropertyType::Impp => impp_get_value(str, kind)?,
            PropertyType::Interest => interest_get_value(str, kind)?,
            PropertyType::Key => key_get_value(str, kind)?,
            PropertyType::Kind => kind_get_value(str, kind)?,
            PropertyType::Lang => lang_get_value(str, kind)?,
            PropertyType::Logo => logo_get_value(str, kind)?,
            PropertyType::Member => member_get_value(str, kind)?,
            PropertyType::NickName => nickname_get_value(str, kind)?,
            PropertyType::Note => note_get_value(str, kind)?,
            PropertyType::N => n_get_value(str, kind)?,
            PropertyType::OrgDirectory => orgdirectory_get_value(str, kind)?,
            PropertyType::Org => org_get_value(str, kind)?,
            PropertyType::Photo => photo_get_value(str, kind)?,
            PropertyType::ProdId => prodid_get_value(str, kind)?,
            PropertyType::Related => related_get_value(str, kind)?,
            PropertyType::Rev => rev_get_value(str, kind)?,
            PropertyType::Role => role_get_value(str, kind)?,
            PropertyType::Sound => sound_get_value(str, kind)?,
            PropertyType::Source => source_get_value(str, kind)?,
            PropertyType::Tel => tel_get_value(str, kind)?,
            PropertyType::Title => title_get_value(str, kind)?,
            PropertyType::Tz => tz_get_value(str, kind)?,
            PropertyType::Uid => uid_get_value(str, kind)?,
            PropertyType::Url => url_get_value(str, kind)?,
            PropertyType::Version => version_get_value(str, kind)?,
            PropertyType::Xml => xml_get_value(str, kind)?,
        })
    }
}

impl TryFrom<(&ParameterType, &str)> for ValueData {
    type Error = VcardError;
    fn try_from((parameter_type, str): (&ParameterType, &str)) -> Result<Self, Self::Error> {
        // TODO: Need to implement per type get value like above.
        Ok(match parameter_type {
            ParameterType::AltId => ValueData::Text(str.to_string()),
            ParameterType::Any => ValueData::Text(str.to_string()),
            ParameterType::CalScale => ValueData::Text(str.to_string()),
            ParameterType::Cc => ValueData::Text(str.to_string()),
            ParameterType::Geo => ValueData::Text(str.to_string()),
            ParameterType::Index => ValueData::Text(str.to_string()),
            ParameterType::Label => ValueData::Text(str.to_string()),
            ParameterType::Language => ValueData::Text(str.to_string()),
            ParameterType::Level => ValueData::Text(str.to_string()),
            ParameterType::MediaType => ValueData::Text(str.to_string()),
            ParameterType::Pid => ValueData::Text(str.to_string()),
            ParameterType::Pref => ValueData::Text(str.to_string()),
            ParameterType::SortAs => ValueData::Text(str.to_string()),
            ParameterType::Type => ValueData::Text(str.to_string()),
            ParameterType::Tz => ValueData::Text(str.to_string()),
            ParameterType::Value => ValueData::Text(str.to_string()),
        })
    }
}

impl Display for ValueData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueData::Boolean(value) => write!(f, "{}", value),
            ValueData::Date((year, month, day)) => write!(f, "{}-{:02}-{:02}", year, month, day),
            ValueData::DateList(value) => write!(f, "{}", value.iter().map(|(year, month, day)| format!("{}-{:02}-{:02}", year, month, day)).collect::<Vec<String>>().join(";")),
            ValueData::Float(value) => write!(f, "{}", value),
            ValueData::FloatList(value) => write!(f, "{}", value.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(";")),
            ValueData::Integer(value) => write!(f, "{}", value),
            ValueData::IntegerList(value) => write!(f, "{}", value.iter().map(|f| f.to_string()).collect::<Vec<String>>().join(";")),
            ValueData::LanguageTag(value) => write!(f, "{}", value),
            ValueData::Text(value) => write!(f, "{}", value),
            ValueData::TextList(value) => write!(f, "{}", value.join(";")),
            ValueData::Uri(value) => write!(f, "{}", value),
            ValueData::UtcOffset(value) => write!(f, "{}", value),
            ValueData::ClientPidMap((i, s)) => write!(f, "{};{}", i, s),
        }
    }
}
