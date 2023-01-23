//! Parameter functions.

use nom::branch::alt;
use nom::bytes::complete::{tag_no_case, take_while1};
use nom::combinator::recognize;
use nom::error::context;
use nom::sequence::tuple;
use nom::IResult;

use crate::constants::{ParameterName, VcardParseError};
use crate::parse::delimiters::{equals, semicolon};
use crate::parse::value::{is_alphanumeric_dash, value_qsafe, value_safe};
use crate::parse::{Data, ParameterData};
use crate::VcardError;

/// Parse any parameter.
pub fn parameter(i: Data) -> IResult<Data, ParameterData, VcardError> {
    match context(VcardParseError::PARAMETER, tuple((semicolon, parameter_name, equals, parameter_value)))(i) {
        Ok((i, (_, parameter_name, _, parameter_value))) => Ok((i, (parameter_name, parameter_value))),
        Err(err) => Err(err),
    }
}

/// Parse parameter value.
pub fn parameter_value(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PARAMETER_VALUE, alt((value_qsafe, value_safe)))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse parameter name, including x-names.
pub fn parameter_name(i: Data) -> IResult<Data, Data, VcardError> {
    match context(
        "Unable to parse parameter type.",
        alt((
            parameter_name_altid,
            parameter_name_calscale,
            parameter_name_cc,
            parameter_name_geo,
            parameter_name_index,
            parameter_name_label,
            parameter_name_language,
            parameter_name_level,
            parameter_name_mediatype,
            parameter_name_pid,
            parameter_name_pref,
            parameter_name_sortas,
            parameter_name_type,
            parameter_name_tz,
            parameter_name_value,
            parameter_x_name,
        )),
    )(i)
    {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ALTID parameter name.
pub fn parameter_name_altid(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::ALTID)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CALSCALE parameter name.
pub fn parameter_name_calscale(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::CALSCALE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CC parameter name.
pub fn parameter_name_cc(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::CC)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse GEO parameter name.
pub fn parameter_name_geo(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::GEO)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse INDEX parameter name.
pub fn parameter_name_index(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::INDEX)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse LABEL parameter name.
pub fn parameter_name_label(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::LABEL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse LANGUAGE parameter name.
pub fn parameter_name_language(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::LANGUAGE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse LEVEL parameter name.
pub fn parameter_name_level(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::LEVEL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse MEDIATYPE parameter name.
pub fn parameter_name_mediatype(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::MEDIATYPE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse PID parameter name.
pub fn parameter_name_pid(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::PID)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse PREF parameter name.
pub fn parameter_name_pref(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::PREF)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse SORTAS parameter name.
pub fn parameter_name_sortas(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::SORTAS)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse TYPE parameter name.
pub fn parameter_name_type(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::TYPE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse TZ parameter name.
pub fn parameter_name_tz(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::TZ)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse VALUE parameter name.
pub fn parameter_name_value(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(ParameterName::VALUE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse x-name parameter name.
pub fn parameter_x_name(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PARAMETER_XNAME, recognize(tuple((tag_no_case("x-"), take_while1(is_alphanumeric_dash)))))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::constants::ParameterName;
    use crate::parse::parameter::{parameter, parameter_name};

    #[test]
    fn parse_parameter() {
        assert_eq!(String::from_utf8(parameter.parse(r#";ALTID=1"#.as_bytes()).unwrap().1 .0.to_vec()).unwrap(), r#"ALTID"#);
        assert_eq!(String::from_utf8(parameter.parse(r#";ALTID=1"#.as_bytes()).unwrap().1 .1.to_vec()).unwrap(), r#"1"#);
        assert_eq!(String::from_utf8(parameter.parse(r#";ALTID="1"#.as_bytes()).unwrap().1 .1.to_vec()).unwrap(), r#""#);
        assert_eq!(String::from_utf8(parameter.parse(r#";ALTID="1""#.as_bytes()).unwrap().1 .1.to_vec()).unwrap(), r#""1""#);
    }

    #[test]
    fn parse_parameter_name() {
        assert_ne!(String::from_utf8(parameter_name.parse(ParameterName::ALTID.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::CALSCALE);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::ALTID.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::ALTID);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::CALSCALE.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::CALSCALE);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::CC.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::CC);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::GEO.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::GEO);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::INDEX.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::INDEX);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::LABEL.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::LABEL);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::LANGUAGE.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::LANGUAGE);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::LEVEL.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::LEVEL);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::MEDIATYPE.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::MEDIATYPE);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::PID.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::PID);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::PREF.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::PREF);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::SORTAS.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::SORTAS);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::TYPE.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::TYPE);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::TZ.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::TZ);
        assert_eq!(String::from_utf8(parameter_name.parse(ParameterName::VALUE.as_bytes()).unwrap().1.to_vec()).unwrap(), ParameterName::VALUE);
        assert_eq!(String::from_utf8(parameter_name.parse("X-SERVICE-TYPE".as_bytes()).unwrap().1.to_vec()).unwrap(), "X-SERVICE-TYPE");
    }
}
