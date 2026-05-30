//! Vcard functions.

use nom::error::context;
use nom::multi::{many0, many1};
use nom::{IResult, Parser};

use crate::constants::VcardParseError;
use crate::parse::property::{property, property_begin, property_end, property_version};
use crate::parse::VcardData;
use crate::VcardError;

/// Parse a vcard string and return an array of content properties.
pub fn vcards(i: &[u8]) -> IResult<&[u8], Vec<VcardData<'_>>, VcardError> {
    context(VcardParseError::VCARDS, many1(vcard)).parse(i)
}

/// Parse a vcard string and return an array of content properties.
pub fn vcard(i: &[u8]) -> IResult<&[u8], VcardData<'_>, VcardError> {
    match context(VcardParseError::VCARD, (property_begin, property_version, many0(property), property_end)).parse(i) {
        Ok((i, (_, _, properties, _))) => Ok((i, properties)),
        Err(err) => Err(err),
    }
}
