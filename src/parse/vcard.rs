//! Vcard functions.

use nom::error::context;
use nom::multi::{many0, many1};
use nom::sequence::tuple;
use nom::IResult;

use crate::constants::VcardParseError;
use crate::parse::property::{property, property_begin, property_end, property_version};
use crate::parse::VcardData;
use crate::VcardError;

/// Parse a vcard string and return an array of content properties.
pub fn vcards(i: &[u8]) -> IResult<&[u8], Vec<VcardData>, VcardError> {
    context(VcardParseError::VCARDS, many1(vcard))(i)
}

/// Parse a vcard string and return an array of content properties.
pub fn vcard(i: &[u8]) -> IResult<&[u8], VcardData, VcardError> {
    match context(VcardParseError::VCARD, tuple((property_begin, property_version, many0(property), property_end)))(i) {
        Ok((i, (_, _, properties, _))) => Ok((i, properties)),
        Err(err) => Err(err),
    }
}
