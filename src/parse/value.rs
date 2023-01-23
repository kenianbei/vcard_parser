//! Value functions.

use nom::bytes::complete::take_while;
use nom::character::complete::char;
use nom::character::is_alphanumeric;
use nom::combinator::{opt, recognize};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::IResult;

use crate::constants::VcardParseError;
use crate::parse::delimiters::fold;
use crate::parse::{Data, ValueData, ValueFoldedData};
use crate::VcardError;

pub fn value(i: Data) -> IResult<Data, ValueFoldedData, VcardError> {
    match context(VcardParseError::VALUE, tuple((take_while(is_value_char), opt(many0(value_folded)))))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

pub fn value_folded(i: Data) -> IResult<Data, ValueData, VcardError> {
    match context(VcardParseError::VALUE_FOLDED, tuple((fold, take_while(is_value_char))))(i) {
        Ok((i, (_, s))) => Ok((i, s)),
        Err(err) => Err(err),
    }
}

pub fn value_qsafe(i: Data) -> IResult<Data, ValueData, VcardError> {
    match context(VcardParseError::VALUE_QSAFE, recognize(tuple((char('"'), take_while(is_qsafe_char), char('"')))))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

pub fn value_safe(i: Data) -> IResult<Data, ValueData, VcardError> {
    match context(VcardParseError::VALUE_SAFE, take_while(is_safe_char))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Any character except CTLs, DQUOTE see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
pub fn is_qsafe_char(c: u8) -> bool {
    if c == b'\n' || c == b'\r' || c == b'\t' {
        return false;
    }
    if c == b'"' {
        return false;
    }

    true
}

/// Any character except CTLs, DQUOTE, ";", ":", see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
pub fn is_safe_char(c: u8) -> bool {
    if c == b'\n' || c == b'\r' || c == b'\t' {
        return false;
    }
    if c == b'"' {
        return false;
    }
    if c == b';' {
        return false;
    }
    if c == b':' {
        return false;
    }

    true
}

/// Any textual character, see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
pub fn is_value_char(c: u8) -> bool {
    if c == b'\n' || c == b'\r' || c == b'\t' {
        return false;
    }

    true
}

/// Groups, Property, Parameter, Iana Token and X-Names, see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
pub fn is_alphanumeric_dash(c: u8) -> bool {
    if c == b'-' {
        return true;
    }

    is_alphanumeric(c)
}

pub fn utf8_to_string(u8: &[u8]) -> Result<String, VcardError> {
    if let Ok(string) = String::from_utf8(u8.to_vec()) {
        Ok(string)
    } else {
        Err(VcardError::ConversionFailure)
    }
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use nom::Parser;

    use crate::parse::value::{value, value_folded, value_qsafe, value_safe};

    #[test]
    fn parse_value() {
        assert!(value.parse("\"\n".as_bytes()).is_ok());
        assert!(value.parse(":\n".as_bytes()).is_ok());
        assert!(value.parse(";\n".as_bytes()).is_ok());
        assert!(value.parse(",\n".as_bytes()).is_ok());
        assert!(value.parse(" \n".as_bytes()).is_ok());
        assert!(value.parse("\\r".as_bytes()).is_ok());
        assert!(value.parse("\\n".as_bytes()).is_ok());
        assert!(value.parse("\\t".as_bytes()).is_ok());
        assert!(value.parse("\r".as_bytes()).is_ok());
        assert!(value.parse("\n".as_bytes()).is_ok());
        assert!(value.parse("\t".as_bytes()).is_ok());
        assert!(value.parse("Hello\n\tWorld\n\tAgain\n".as_bytes()).is_ok());
    }

    #[test]
    fn parse_value_folded() {
        assert_eq!(value_folded.parse("\n\t".as_bytes()).unwrap().1, "".as_bytes());
        assert_eq!(value_folded.parse("\n ".as_bytes()).unwrap().1, "".as_bytes());
        assert_eq!(value_folded.parse("\r\n\t".as_bytes()).unwrap().1, "".as_bytes());
        assert_eq!(value_folded.parse("\r\n ".as_bytes()).unwrap().1, "".as_bytes());
    }

    #[test]
    fn parse_value_qsafe() {
        assert!(value_qsafe.parse("\":;\"".as_bytes()).is_ok());
        assert!(value_qsafe.parse("\":;\"".as_bytes()).is_ok());
        assert!(value_qsafe.parse("\"123456789\"".as_bytes()).is_ok());
        assert!(value_qsafe.parse("\"ABCDEFGHI\"".as_bytes()).is_ok());
        assert!(value_qsafe.parse("\"".as_bytes()).is_err());
        assert!(value_qsafe.parse(":".as_bytes()).is_err());
        assert!(value_qsafe.parse(";".as_bytes()).is_err());
        assert!(value_qsafe.parse("\";".as_bytes()).is_err());
        assert!(value_qsafe.parse(";\"".as_bytes()).is_err());
        assert!(value_qsafe.parse("\"".as_bytes()).is_err());
        assert!(value_qsafe.parse("\n".as_bytes()).is_err());
        assert!(value_qsafe.parse("\t".as_bytes()).is_err());
        assert!(value_qsafe.parse("\r".as_bytes()).is_err());
    }

    #[test]
    fn parse_value_safe() {
        assert_eq!(String::from_utf8(value_safe.parse(r#"ABCDEFGHI"#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#"ABCDEFGHI"#);
        assert_eq!(String::from_utf8(value_safe.parse(r#"123456789"#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#"123456789"#);
        assert_eq!(String::from_utf8(value_safe.parse(r#"""#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#""#);
        assert_eq!(String::from_utf8(value_safe.parse(r#":"#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#""#);
        assert_eq!(String::from_utf8(value_safe.parse(r#";"#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#""#);
        assert_eq!(String::from_utf8(value_safe.parse(r#","#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#","#);
        assert_eq!(String::from_utf8(value_safe.parse(r#"TEST,TEST"#.as_bytes()).unwrap().1.to_vec()).unwrap(), r#"TEST,TEST"#);
    }
}
