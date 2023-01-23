//! The parameter module represents a single property parameter, as per [RFC 6350 Section 5](https://datatracker.ietf.org/doc/html/rfc6350#section-5)
//!
//! Parameters can be created using [`Parameter::try_from`] or [`Parameter::default`]. The format should strictly follow RFC guidelines,
//! e.g. ";ALTID=1" for the ALTID parameter with "1" as the value. Parsed parameters must begin with a semi-colon as per [RFC 6350 Section 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3).
//!
//! For more information on values, see the [values](super::value) module.
//!
//! # Examples
//!
//! ## Creating a new parameter.
//! ```
//! use vcard_parser::vcard::parameter::Parameter;
//!
//! let mut parameter = Parameter::default("ALTID");
//! ```
//!
//! ## Parsing a parameter.
//! ```
//! use vcard_parser::vcard::parameter::Parameter;
//!
//! let mut parameter = Parameter::try_from(";ALTID=1").expect("Unable to parse parameter.");
//! ```
//!
//! ## Updating a parameter.
//! ```
//! use vcard_parser::traits::HasValue;
//! use vcard_parser::vcard::parameter::Parameter;
//! use vcard_parser::vcard::value::Value;
//!
//! let mut parameter = Parameter::try_from(";ALTID=1").expect("Unable to parse parameter.");
//! let updated = Value::try_from(("INTEGER", "1")).expect("Unable to parse value.");
//! parameter.set_value(updated).expect("Unable to update parameter.");
//! assert_eq!(parameter.get_value().to_string(), "1");
//! ```

use std::fmt::{Display, Formatter};

use crate::constants::ParameterName;
use crate::parse::value::utf8_to_string;
use crate::vcard::parameter::parameter_altid::ParameterAltIdData;
use crate::vcard::parameter::parameter_calscale::ParameterCalScaleData;
use crate::vcard::parameter::parameter_cc::ParameterCcData;
use crate::vcard::parameter::parameter_geo::ParameterGeoData;
use crate::vcard::parameter::parameter_index::ParameterIndexData;
use crate::vcard::parameter::parameter_label::ParameterLabelData;
use crate::vcard::parameter::parameter_language::ParameterLanguageData;
use crate::vcard::parameter::parameter_level::ParameterLevelData;
use crate::vcard::parameter::parameter_mediatype::ParameterMediaTypeData;
use crate::vcard::parameter::parameter_pid::ParameterPidData;
use crate::vcard::parameter::parameter_pref::ParameterPrefData;
use crate::vcard::parameter::parameter_sortas::ParameterSortAsData;
use crate::vcard::parameter::parameter_type::ParameterTypeData;
use crate::vcard::parameter::parameter_tz::ParameterTzData;
use crate::vcard::parameter::parameter_value::ValueParameterData;
use crate::vcard::parameter::parameter_xname::XNameParameterData;
use crate::vcard::value::Value;
use crate::{parse, HasName, HasValue, VcardError};

pub mod parameter_altid;
pub mod parameter_calscale;
pub mod parameter_cc;
pub mod parameter_geo;
pub mod parameter_index;
pub mod parameter_label;
pub mod parameter_language;
pub mod parameter_level;
pub mod parameter_mediatype;
pub mod parameter_pid;
pub mod parameter_pref;
pub mod parameter_sortas;
pub mod parameter_type;
pub mod parameter_tz;
pub mod parameter_value;
pub mod parameter_xname;

#[derive(Clone, Debug, PartialEq)]
pub enum Parameter {
    /// Represents an ALTID parameter, see [RFC 6350 5.4](https://datatracker.ietf.org/doc/html/rfc6350#section-5.4).
    ParameterAltId(ParameterAltIdData),
    /// Represents an CALSCALE parameter, see [RFC 6350 5.8](https://datatracker.ietf.org/doc/html/rfc6350#section-5.8).
    ParameterCalScale(ParameterCalScaleData),
    /// Represents an CC parameter, see [RFC 8605 3.1](https://datatracker.ietf.org/doc/html/rfc8605#section-3.1).
    ParameterCc(ParameterCcData),
    /// Represents an GEO parameter, see [RFC 6350 5.10](https://datatracker.ietf.org/doc/html/rfc6350#section-5.10).
    ParameterGeo(ParameterGeoData),
    /// Represents an INDEX parameter, see [RFC 6715 3.1](https://datatracker.ietf.org/doc/html/rfc6715#section-3.1).
    ParameterIndex(ParameterIndexData),
    /// Represents an LABEL parameter, see [RFC 6350](https://datatracker.ietf.org/doc/html/rfc6350).
    ParameterLabel(ParameterLabelData),
    /// Represents an LANGUAGE parameter, see [RFC 6350 5.1](https://datatracker.ietf.org/doc/html/rfc6350#section-5.1).
    ParameterLanguage(ParameterLanguageData),
    /// Represents an LEVEL parameter, see [RFC 6715 3.2](https://datatracker.ietf.org/doc/html/rfc6715#section-3.2).
    ParameterLevel(ParameterLevelData),
    /// Represents an MEDIATYPE parameter, see [RFC 6350 5.7](https://datatracker.ietf.org/doc/html/rfc6350#section-5.7).
    ParameterMediaType(ParameterMediaTypeData),
    /// Represents an PID parameter, see [RFC 6350 5.5](https://datatracker.ietf.org/doc/html/rfc6350#section-5.5).
    ParameterPid(ParameterPidData),
    /// Represents an PREF parameter, see [RFC 6350 5.3](https://datatracker.ietf.org/doc/html/rfc6350#section-5.3).
    ParameterPref(ParameterPrefData),
    /// Represents an SORT-AS parameter, see [RFC 6350 5.9](https://datatracker.ietf.org/doc/html/rfc6350#section-5.9).
    ParameterSortAs(ParameterSortAsData),
    /// Represents an TYPE parameter, see [RFC 6350 5.6](https://datatracker.ietf.org/doc/html/rfc6350#section-5.6).
    ParameterType(ParameterTypeData),
    /// Represents an TZ parameter, see [RFC 6350 5.11](https://datatracker.ietf.org/doc/html/rfc6350#section-5.11).
    ParameterTz(ParameterTzData),
    /// Represents an VALUE parameter, see [RFC 6350 5.2](https://datatracker.ietf.org/doc/html/rfc6350#section-5.2).
    ParameterValue(ValueParameterData),
    /// Represents an VALUE parameter, see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3).
    ParameterXName(XNameParameterData),
}

impl Parameter {
    pub fn default(name: &str) -> Self {
        match name.to_uppercase().as_str() {
            ParameterName::ALTID => Self::ParameterAltId(ParameterAltIdData::default()),
            ParameterName::CALSCALE => Self::ParameterCalScale(ParameterCalScaleData::default()),
            ParameterName::CC => Self::ParameterCc(ParameterCcData::default()),
            ParameterName::GEO => Self::ParameterGeo(ParameterGeoData::default()),
            ParameterName::INDEX => Self::ParameterIndex(ParameterIndexData::default()),
            ParameterName::LABEL => Self::ParameterLabel(ParameterLabelData::default()),
            ParameterName::LANGUAGE => Self::ParameterLanguage(ParameterLanguageData::default()),
            ParameterName::LEVEL => Self::ParameterLevel(ParameterLevelData::default()),
            ParameterName::MEDIATYPE => Self::ParameterMediaType(ParameterMediaTypeData::default()),
            ParameterName::PID => Self::ParameterPid(ParameterPidData::default()),
            ParameterName::PREF => Self::ParameterPref(ParameterPrefData::default()),
            ParameterName::SORTAS => Self::ParameterSortAs(ParameterSortAsData::default()),
            ParameterName::TYPE => Self::ParameterType(ParameterTypeData::default()),
            ParameterName::TZ => Self::ParameterTz(ParameterTzData::default()),
            ParameterName::VALUE => Self::ParameterValue(ValueParameterData::default()),
            _ => Self::ParameterXName(XNameParameterData::default(name)),
        }
    }
}

impl TryFrom<&str> for Parameter {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let (_, (parameter_name, parameter_value)) = parse::parameter::parameter(str.as_bytes())?;
        Parameter::try_from((utf8_to_string(parameter_name)?.as_str(), utf8_to_string(parameter_value)?.as_str()))
    }
}

impl TryFrom<(&[u8], &[u8])> for Parameter {
    type Error = VcardError;
    fn try_from((parameter_name, parameter_value): (&[u8], &[u8])) -> Result<Self, Self::Error> {
        Parameter::try_from((utf8_to_string(parameter_name)?.as_str(), utf8_to_string(parameter_value)?.as_str()))
    }
}

impl TryFrom<(&str, &str)> for Parameter {
    type Error = VcardError;
    fn try_from((parameter_name, parameter_value): (&str, &str)) -> Result<Self, Self::Error> {
        match parameter_name.to_uppercase().as_str() {
            ParameterName::ALTID => Ok(Self::ParameterAltId(ParameterAltIdData::try_from(parameter_value)?)),
            ParameterName::CALSCALE => Ok(Self::ParameterCalScale(ParameterCalScaleData::try_from(parameter_value)?)),
            ParameterName::CC => Ok(Self::ParameterCc(ParameterCcData::try_from(parameter_value)?)),
            ParameterName::GEO => Ok(Self::ParameterGeo(ParameterGeoData::try_from(parameter_value)?)),
            ParameterName::INDEX => Ok(Self::ParameterIndex(ParameterIndexData::try_from(parameter_value)?)),
            ParameterName::LABEL => Ok(Self::ParameterLabel(ParameterLabelData::try_from(parameter_value)?)),
            ParameterName::LANGUAGE => Ok(Self::ParameterLanguage(ParameterLanguageData::try_from(parameter_value)?)),
            ParameterName::LEVEL => Ok(Self::ParameterLevel(ParameterLevelData::try_from(parameter_value)?)),
            ParameterName::MEDIATYPE => Ok(Self::ParameterMediaType(ParameterMediaTypeData::try_from(parameter_value)?)),
            ParameterName::PID => Ok(Self::ParameterPid(ParameterPidData::try_from(parameter_value)?)),
            ParameterName::PREF => Ok(Self::ParameterPref(ParameterPrefData::try_from(parameter_value)?)),
            ParameterName::SORTAS => Ok(Self::ParameterSortAs(ParameterSortAsData::try_from(parameter_value)?)),
            ParameterName::TYPE => Ok(Self::ParameterType(ParameterTypeData::try_from(parameter_value)?)),
            ParameterName::TZ => Ok(Self::ParameterTz(ParameterTzData::try_from(parameter_value)?)),
            ParameterName::VALUE => Ok(Self::ParameterValue(ValueParameterData::try_from(parameter_value)?)),
            _ => Ok(Self::ParameterXName(XNameParameterData::try_from((parameter_name, parameter_value))?)),
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, ";{}={}", self.name(), self.get_value())
    }
}

impl HasName for Parameter {
    fn name(&self) -> &str {
        match self {
            Parameter::ParameterAltId(parameter) => parameter.name(),
            Parameter::ParameterCalScale(parameter) => parameter.name(),
            Parameter::ParameterCc(parameter) => parameter.name(),
            Parameter::ParameterGeo(parameter) => parameter.name(),
            Parameter::ParameterIndex(parameter) => parameter.name(),
            Parameter::ParameterLabel(parameter) => parameter.name(),
            Parameter::ParameterLanguage(parameter) => parameter.name(),
            Parameter::ParameterLevel(parameter) => parameter.name(),
            Parameter::ParameterMediaType(parameter) => parameter.name(),
            Parameter::ParameterPid(parameter) => parameter.name(),
            Parameter::ParameterPref(parameter) => parameter.name(),
            Parameter::ParameterSortAs(parameter) => parameter.name(),
            Parameter::ParameterType(parameter) => parameter.name(),
            Parameter::ParameterTz(parameter) => parameter.name(),
            Parameter::ParameterValue(parameter) => parameter.name(),
            Parameter::ParameterXName(parameter) => parameter.name(),
        }
    }
}

impl HasValue for Parameter {
    fn get_value(&self) -> &Value {
        match self {
            Parameter::ParameterAltId(parameter) => parameter.get_value(),
            Parameter::ParameterCalScale(parameter) => parameter.get_value(),
            Parameter::ParameterCc(parameter) => parameter.get_value(),
            Parameter::ParameterGeo(parameter) => parameter.get_value(),
            Parameter::ParameterIndex(parameter) => parameter.get_value(),
            Parameter::ParameterLabel(parameter) => parameter.get_value(),
            Parameter::ParameterLanguage(parameter) => parameter.get_value(),
            Parameter::ParameterLevel(parameter) => parameter.get_value(),
            Parameter::ParameterMediaType(parameter) => parameter.get_value(),
            Parameter::ParameterPid(parameter) => parameter.get_value(),
            Parameter::ParameterPref(parameter) => parameter.get_value(),
            Parameter::ParameterSortAs(parameter) => parameter.get_value(),
            Parameter::ParameterType(parameter) => parameter.get_value(),
            Parameter::ParameterTz(parameter) => parameter.get_value(),
            Parameter::ParameterValue(parameter) => parameter.get_value(),
            Parameter::ParameterXName(parameter) => parameter.get_value(),
        }
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        match self {
            Parameter::ParameterAltId(parameter) => parameter.set_value(value),
            Parameter::ParameterCalScale(parameter) => parameter.set_value(value),
            Parameter::ParameterCc(parameter) => parameter.set_value(value),
            Parameter::ParameterGeo(parameter) => parameter.set_value(value),
            Parameter::ParameterIndex(parameter) => parameter.set_value(value),
            Parameter::ParameterLabel(parameter) => parameter.set_value(value),
            Parameter::ParameterLanguage(parameter) => parameter.set_value(value),
            Parameter::ParameterLevel(parameter) => parameter.set_value(value),
            Parameter::ParameterMediaType(parameter) => parameter.set_value(value),
            Parameter::ParameterPid(parameter) => parameter.set_value(value),
            Parameter::ParameterPref(parameter) => parameter.set_value(value),
            Parameter::ParameterSortAs(parameter) => parameter.set_value(value),
            Parameter::ParameterType(parameter) => parameter.set_value(value),
            Parameter::ParameterTz(parameter) => parameter.set_value(value),
            Parameter::ParameterValue(parameter) => parameter.set_value(value),
            Parameter::ParameterXName(parameter) => parameter.set_value(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::parameter::Parameter;

    #[test]
    fn parameter_try_from() {
        assert!(Parameter::try_from(";ALTID=1").is_ok());
        assert!(Parameter::try_from(";CALSCALE=gregorian").is_ok());
        assert!(Parameter::try_from(";CC=us").is_ok());
        assert!(Parameter::try_from(";GEO=\"geo:0.0,-0.0\"").is_ok());
        assert!(Parameter::try_from(";INDEX=1").is_ok());
        assert!(Parameter::try_from(";LABEL=WORK").is_ok());
        assert!(Parameter::try_from(";LANGUAGE=en").is_ok());
        assert!(Parameter::try_from(";LEVEL=1").is_ok());
        assert!(Parameter::try_from(";MEDIATYPE=1").is_ok());
        assert!(Parameter::try_from(";PID=1").is_ok());
        assert!(Parameter::try_from(";PREF=1").is_ok());
        assert!(Parameter::try_from(";SORT-AS=1").is_ok());
        assert!(Parameter::try_from(";TYPE=1").is_ok());
        assert!(Parameter::try_from(";TZ=1").is_ok());
        assert!(Parameter::try_from(";VALUE=1").is_ok());
        assert!(Parameter::try_from(";X-VALUE=1").is_ok());
    }
}
