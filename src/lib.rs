//! # vCard Parser
//!
//! Parses and validates [vCard data](vcard) according to [RFC 6350](https://datatracker.ietf.org/doc/html/rfc6350) specification.
//!
//! ## Creating vCards
//!
//! ```rust
//! use vcard_parser::vcard::property::Property;
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcard = Vcard::new("John Doe");
//!
//! // Add a nickname property.
//! let mut property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property.");
//! vcard.set_property(&property).expect("Unable to add property.");
//!
//! // Print vCard with pids and clientpidmap info.
//! print!("{}", vcard);
//!
//! // Export vCard without any ids.
//! print!("{}", vcard.export());
//! ```
//!
//! ## Parsing vCards
//!
//! vCards can be parsed from a string containing multiple vCards using the main [`parse_vcards()`] or [`parse_vcards_with_client()`] functions,
//! or from a string containing one vCard using [`Vcard::try_from`].
//!
//! ```rust
//! use vcard_parser::parse_vcards;
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcards = parse_vcards("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n").expect("Unable to parse string.");
//! assert_eq!(vcards.len(), 1);
//!
//! let mut vcard = Vcard::try_from("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n").expect("Unable to parse string.");
//! assert_eq!(vcard.get_properties().len(), 1)
//! ```
//!
//! ## Parsing from file
//!
//! Read a vcf file and ignore invalid properties, update the vCard object, and write back to file.
//!
//! ```rust
//! use std::fs::{read_to_string, write};
//! use vcard_parser::parse_vcards;
//! use vcard_parser::traits::HasValue;
//! use vcard_parser::vcard::value::Value;
//! use vcard_parser::vcard::value::value_text::ValueTextData;
//!
//! let input = read_to_string("contacts.vcf").unwrap_or(String::from("BEGIN:VCARD\nVERSION:4.0\nFN:\nEND:VCARD\n"));
//! let mut vcards = parse_vcards(input.as_str()).expect("Unable to parse string.");
//!
//! let vcard = vcards.first_mut().unwrap();
//! let mut property = vcard.get_property_by_name("FN").unwrap();
//!
//! property.set_value(Value::from(ValueTextData::from("John Doe"))).unwrap();
//! vcard.set_property(&property).expect("Unable to update property.");
//!
//! let mut data = String::new();
//! for vcard in vcards {
//!     data.push_str(vcard.export().as_str())
//! }
//!
//! // write("contacts.vcf", data).expect("Unable to write file.");
//! ```

use crate::error::VcardError;
use crate::traits::{HasCardinality, HasName, HasParameters, HasValue};
use crate::vcard::property::Property;
use crate::vcard::Vcard;

pub mod constants;
pub mod error;
pub mod parse;
pub mod traits;
pub mod vcard;

/// Parses a string and returns either a [VcardError](VcardError) or an array of [Vcard](Vcard)s as the result.
///
/// The input string can be a single vCard or multiple vCards, formatted as per [RFC 6350 Section 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
///
/// # Examples
/// ```
/// use vcard_parser::parse_vcards;
///
/// let vcards = parse_vcards("BEGIN:VCARD\nVERSION:4.0\nFN:\nEND:VCARD\n").expect("Unable to parse text.");
/// assert_eq!(vcards.len(), 1);
/// ```
pub fn parse_vcards(input: &str) -> Result<Vec<Vcard>, VcardError> {
    let mut vcards = Vec::new();

    for data in parse::vcard::vcards(input.as_bytes())?.1 {
        vcards.push(Vcard::try_from((None, data))?);
    }

    Ok(vcards)
}

/// Takes a client and vcard string(s) and returns either a [VcardError](VcardError) or an array of [Vcard](Vcard)s as the result.
///
/// The input string can be a single vCard or multiple vCards, formatted as per [RFC 6350 Section 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3)
///
/// # Examples
/// ```
/// use vcard_parser::parse_vcards_with_client;
///
/// let vcards = parse_vcards_with_client("urn:uuid:someid", "BEGIN:VCARD\nVERSION:4.0\nFN:\nEND:VCARD\n").expect("Unable to parse text.");
/// assert_eq!(vcards.len(), 1);
/// ```
pub fn parse_vcards_with_client(client: &str, input: &str) -> Result<Vec<Vcard>, VcardError> {
    let mut vcards = Vec::new();

    for data in parse::vcard::vcards(input.as_bytes())?.1 {
        vcards.push(Vcard::try_from((Some(client.to_string()), data))?);
    }

    Ok(vcards)
}

#[cfg(test)]
mod tests {
    use crate::constants::{TestData, VcardParseError};
    use crate::{parse_vcards, VcardError};

    fn _match((a, b): (&str, &str)) {
        assert_eq!(parse_vcards(a).unwrap().first().unwrap().export(), b.to_string())
    }

    #[test]
    fn parse_no_version() {
        assert_eq!(parse_vcards(TestData::VCARD_ERROR_VERSION_MISSING).unwrap_err().parse_error().as_str(), VcardParseError::PROPERTY_VERSION_MISSING);
    }

    #[test]
    fn parse_no_fullname() {
        assert!(matches!(parse_vcards(TestData::VCARD_ERROR_FULLNAME_MISSING), Err(VcardError::PropertyFnMissing)));
    }

    #[test]
    fn parse_begin_missing() {
        assert_eq!(parse_vcards(TestData::VCARD_ERROR_BEGIN_MISSING).unwrap_err().parse_error().as_str(), VcardParseError::PROPERTY_BEGIN_MISSING);
    }

    #[test]
    fn parse_end_missing() {
        assert_eq!(parse_vcards(TestData::VCARD_ERROR_END_MISSING).unwrap_err().parse_error().as_str(), VcardParseError::PROPERTY_END_MISSING);
    }

    #[test]
    fn parse_version_3() {
        assert_eq!(parse_vcards(TestData::VCARD_ERROR_VERSION_INCORRECT).unwrap_err().parse_error().as_str(), VcardParseError::PROPERTY_VERSION);
    }

    #[test]
    fn parse_sample_minimal() {
        _match(TestData::VCARD_MATCH_MINIMAL);
    }

    #[test]
    fn parse_concat() {
        _match(TestData::VCARD_MATCH_CONCAT);
    }

    #[test]
    fn parse_xname() {
        _match(TestData::VCARD_MATCH_XNAME);
    }

    #[test]
    fn sample_compound() {
        _match(TestData::VCARD_MATCH_COMPOUND);
    }
}
