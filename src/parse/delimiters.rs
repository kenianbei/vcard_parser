//! Delimiter parsing functions.

use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space1};
use nom::error::context;
use nom::sequence::tuple;
use nom::IResult;

use crate::constants::VcardParseError;
use crate::parse::Data;
use crate::VcardError;

pub fn colon(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::DELIMITER_COLON, tag(":"))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

pub fn comma(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::DELIMITER_COMMA, tag(","))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

pub fn fold(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::DELIMITER_CONCAT, tuple((line_ending, space1)))(i) {
        Ok((i, (_, s))) => Ok((i, s)),
        Err(err) => Err(err),
    }
}

pub fn equals(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::DELIMITER_EQUALS, tag("="))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

pub fn semicolon(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::DELIMITER_SEMI_COLON, tag(";"))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use nom::Parser;

    use crate::parse::delimiters::{colon, comma, equals, fold, semicolon};

    #[test]
    fn parse_delimiters() {
        assert_eq!(String::from_utf8(colon.parse(":".as_bytes()).unwrap().1.to_vec()).unwrap(), ":");
        assert_eq!(String::from_utf8(comma.parse(",".as_bytes()).unwrap().1.to_vec()).unwrap(), ",");
        assert_eq!(String::from_utf8(equals.parse("=".as_bytes()).unwrap().1.to_vec()).unwrap(), "=");
        assert_eq!(String::from_utf8(fold.parse("\n\t".as_bytes()).unwrap().1.to_vec()).unwrap(), "\t");
        assert_eq!(String::from_utf8(fold.parse("\n ".as_bytes()).unwrap().1.to_vec()).unwrap(), " ");
        assert_eq!(String::from_utf8(semicolon.parse(";".as_bytes()).unwrap().1.to_vec()).unwrap(), ";");
    }
}
