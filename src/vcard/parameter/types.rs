use std::fmt::{Display, Formatter};

use crate::VcardError;

pub const PARAMETER_TYPE_ALTID: &str = "ALTID";
pub const PARAMETER_TYPE_ANY: &str = "ANY";
pub const PARAMETER_TYPE_CALSCALE: &str = "CALSCALE";
pub const PARAMETER_TYPE_CC: &str = "CC";
pub const PARAMETER_TYPE_GEO: &str = "GEO";
pub const PARAMETER_TYPE_INDEX: &str = "INDEX";
pub const PARAMETER_TYPE_LABEL: &str = "LABEL";
pub const PARAMETER_TYPE_LANGUAGE: &str = "LANGUAGE";
pub const PARAMETER_TYPE_LEVEL: &str = "LEVEL";
pub const PARAMETER_TYPE_MEDIATYPE: &str = "MEDIATYPE";
pub const PARAMETER_TYPE_PID: &str = "PID";
pub const PARAMETER_TYPE_PREF: &str = "PREF";
pub const PARAMETER_TYPE_SORTAS: &str = "SORT-AS";
pub const PARAMETER_TYPE_TYPE: &str = "TYPE";
pub const PARAMETER_TYPE_TZ: &str = "TZ";
pub const PARAMETER_TYPE_VALUE: &str = "VALUE";

/// A list of parameter types. See [RFC 6350 Section 5](https://datatracker.ietf.org/doc/html/rfc6350#section-5).
#[derive(Clone, Eq, PartialEq)]
pub enum ParameterType {
    AltId,
    Any,
    CalScale,
    Cc,
    Geo,
    Index,
    Label,
    Language,
    Level,
    MediaType,
    Pid,
    Pref,
    SortAs,
    Type,
    Tz,
    Value,
}

impl TryFrom<&str> for ParameterType {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        match str.to_uppercase().as_str() {
            PARAMETER_TYPE_ALTID => Ok(ParameterType::AltId),
            PARAMETER_TYPE_ANY => Ok(ParameterType::Any),
            PARAMETER_TYPE_CALSCALE => Ok(ParameterType::CalScale),
            PARAMETER_TYPE_CC => Ok(ParameterType::Cc),
            PARAMETER_TYPE_GEO => Ok(ParameterType::Geo),
            PARAMETER_TYPE_INDEX => Ok(ParameterType::Index),
            PARAMETER_TYPE_LABEL => Ok(ParameterType::Label),
            PARAMETER_TYPE_LANGUAGE => Ok(ParameterType::Language),
            PARAMETER_TYPE_LEVEL => Ok(ParameterType::Level),
            PARAMETER_TYPE_MEDIATYPE => Ok(ParameterType::MediaType),
            PARAMETER_TYPE_PID => Ok(ParameterType::Pid),
            PARAMETER_TYPE_PREF => Ok(ParameterType::Pref),
            PARAMETER_TYPE_SORTAS => Ok(ParameterType::SortAs),
            PARAMETER_TYPE_TYPE => Ok(ParameterType::Type),
            PARAMETER_TYPE_TZ => Ok(ParameterType::Tz),
            PARAMETER_TYPE_VALUE => Ok(ParameterType::Value),
            _ => Err(VcardError::ParameterTypeUnknown(str.to_string())),
        }
    }
}

impl Display for ParameterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            ParameterType::AltId => PARAMETER_TYPE_ALTID,
            ParameterType::Any => PARAMETER_TYPE_ANY,
            ParameterType::CalScale => PARAMETER_TYPE_CALSCALE,
            ParameterType::Cc => PARAMETER_TYPE_CC,
            ParameterType::Geo => PARAMETER_TYPE_GEO,
            ParameterType::Index => PARAMETER_TYPE_INDEX,
            ParameterType::Label => PARAMETER_TYPE_LABEL,
            ParameterType::Language => PARAMETER_TYPE_LANGUAGE,
            ParameterType::Level => PARAMETER_TYPE_LEVEL,
            ParameterType::MediaType => PARAMETER_TYPE_MEDIATYPE,
            ParameterType::Pid => PARAMETER_TYPE_PID,
            ParameterType::Pref => PARAMETER_TYPE_PREF,
            ParameterType::SortAs => PARAMETER_TYPE_SORTAS,
            ParameterType::Type => PARAMETER_TYPE_TYPE,
            ParameterType::Tz => PARAMETER_TYPE_TZ,
            ParameterType::Value => PARAMETER_TYPE_VALUE,
        };
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::parameter::types::ParameterType;

    #[test]
    pub fn parameter_types() {
        assert_eq!(ParameterType::try_from("ALTID").unwrap().to_string().as_str(), "ALTID");
        assert_eq!(ParameterType::try_from("ANY").unwrap().to_string().as_str(), "ANY");
        assert_eq!(ParameterType::try_from("CALSCALE").unwrap().to_string().as_str(), "CALSCALE");
        assert_eq!(ParameterType::try_from("CC").unwrap().to_string().as_str(), "CC");
        assert_eq!(ParameterType::try_from("GEO").unwrap().to_string().as_str(), "GEO");
        assert_eq!(ParameterType::try_from("INDEX").unwrap().to_string().as_str(), "INDEX");
        assert_eq!(ParameterType::try_from("LABEL").unwrap().to_string().as_str(), "LABEL");
        assert_eq!(ParameterType::try_from("LANGUAGE").unwrap().to_string().as_str(), "LANGUAGE");
        assert_eq!(ParameterType::try_from("LEVEL").unwrap().to_string().as_str(), "LEVEL");
        assert_eq!(ParameterType::try_from("MEDIATYPE").unwrap().to_string().as_str(), "MEDIATYPE");
        assert_eq!(ParameterType::try_from("PID").unwrap().to_string().as_str(), "PID");
        assert_eq!(ParameterType::try_from("PREF").unwrap().to_string().as_str(), "PREF");
        assert_eq!(ParameterType::try_from("SORT-AS").unwrap().to_string().as_str(), "SORT-AS");
        assert_eq!(ParameterType::try_from("TYPE").unwrap().to_string().as_str(), "TYPE");
        assert_eq!(ParameterType::try_from("TZ").unwrap().to_string().as_str(), "TZ");
        assert_eq!(ParameterType::try_from("VALUE").unwrap().to_string().as_str(), "VALUE");
    }
}
