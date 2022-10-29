//! # vCard Parser
//!
//! Parses and validates vCard data according to RFC 6350 specification.
//!
//! ## Installation
//!
//! Add the library to the dependencies section of your cargo.toml file.
//!
//! ```toml
//! [dependencies]
//! vcard_parser = "0.1.0"
//! ```
//!
//! ## Parsing vCards
//!
//! Reading a vcf file, updating the vCard object, and writing back to the file.
//!
//! ```rust
//! use std::fs;
//! use std::fs::read_to_string;
//! use vcard_parser::parse_to_vcards;
//! use vcard_parser::vcard::property::types::PropertyType;
//!
//! if let Ok(string) = read_to_string("contacts.vcf") {
//!     let mut vcards = parse_to_vcards(string.as_str()).unwrap();
//!
//!     let mut vcard = vcards.first().unwrap().clone();
//!     let property = vcard.get_property_by_type(&PropertyType::Fn).unwrap();
//!
//!     vcard.update_property(property.get_uuid(), "FN:John Doe").expect("Unable to update property.");
//!     vcards[0] = vcard;
//!
//!     let mut data = String::new();
//!     for vcard in vcards {
//!         data.push_str(vcard.to_string().as_str())
//!     }
//!     fs::write("contacts.vcf", data).expect("Unable to write file.");
//! }
//! ```
//!
//! ## Parsing a single vCard
//!
//! ```rust
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcard = Vcard::try_from("VERSION:4.0\nFN:John Doe\n").unwrap();
//! vcard.add_property("NICKNAME:Johnny").unwrap();
//! println!("{}", vcard.to_string());
//! ```
//!
//! ### Creating a new vCard
//!
//! ```rust
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcard = Vcard::default();
//! vcard.add_property("NICKNAME:Johnny").unwrap();
//! println!("{}", vcard.to_string());
//! ```

pub extern crate uuid;

use regex::Regex;

use crate::error::VcardError;
use crate::vcard::Vcard;

/// Contains API error types.
pub mod error;

/// Contains utility functions.
pub mod util;

/// Main vCard object for parsing, storing, and exporting vCard data.
pub mod vcard;

/// Parses a vCard string and returns either a [VcardError](VcardError) or an array of individual vCard strings as the result.
///
/// # Examples
/// ```
/// use vcard_parser::{parse_to_vcards, parse_to_strings};
///
/// let text = r#"
/// BEGIN:VCARD
/// VERSION:4.0
/// FN:John Doe
/// END:VCARD
/// "#;
///
/// let vcard_strings = parse_to_strings(text).expect("Unable to parse text.");
/// assert_eq!(vcard_strings.len(), 1);
/// ```
pub fn parse_to_strings(input: &str) -> Result<Vec<String>, VcardError> {
    let mut data: Vec<String> = Vec::new();

    let input = Regex::new(r"(?mi)^\s*(BEGIN|END):VCARD\s*?$").unwrap().replace_all(input, "$1:VCARD");
    let input = Regex::new(r"(?mi)\n\s").unwrap().replace_all(&*input, "");

    let regex = Regex::new(r"(?mi)\s*?BEGIN:VCARD\s*?$\n([\s\S]*?)\s*?END:VCARD\s*?$\n?").unwrap();
    for cap in regex.captures_iter(&*input) {
        data.push(cap[1].to_string())
    }

    Ok(data)
}

/// Parses a vCard string and returns either a [VcardError](VcardError) or an array of [Vcard](Vcard)s as the result.
///
/// # Examples
/// ```
/// use vcard_parser::parse_to_vcards;
///
/// let text = r#"
/// BEGIN:VCARD
/// VERSION:4.0
/// FN:John Doe
/// END:VCARD
/// "#;
///
/// let vcards = parse_to_vcards(text).expect("Unable to parse text.");
/// assert_eq!(vcards.len(), 1);
/// ```
pub fn parse_to_vcards(input: &str) -> Result<Vec<Vcard>, VcardError> {
    let mut vcards = Vec::new();

    for string in parse_to_strings(input)? {
        vcards.push(Vcard::try_from(string.as_str())?);
    }

    Ok(vcards)
}

#[cfg(test)]
mod tests {
    use crate::{parse_to_vcards, VcardError};

    #[test]
    fn vcard_no_version() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nFN:John Doe\r\nEND:VCARD");
        assert!(matches!(result, Err(VcardError::PropertyMissing(_))));
    }

    #[test]
    fn vcard_no_fullname() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nEND:VCARD");
        assert!(matches!(result, Err(VcardError::PropertyMissing(_))));
    }

    #[test]
    fn vcard_missing_begin() {
        let result = parse_to_vcards("VERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn vcard_missing_end() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\n");
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn vcard_missing_begin_valid() {
        let result = parse_to_vcards("BEGIN:\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert!(result.unwrap().is_empty());
        let result = parse_to_vcards("BEGIN:VCARDS\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert!(result.unwrap().is_empty());
        let result = parse_to_vcards("   BEGIN:VCARD   \r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert_eq!(result.unwrap().len(), 1);
        let result = parse_to_vcards("   \tBEGIN:VCARD\t   \r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn vcard_missing_end_valid() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:");
        assert!(result.unwrap().is_empty());
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARDS");
        assert!(result.unwrap().is_empty());
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert_eq!(result.unwrap().len(), 1);
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\n   \tEND:VCARD   \t");
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn vcard_version_3() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:3.0\r\nFN:John Doe\r\nEND:VCARD");
        assert!(matches!(result, Err(VcardError::VersionInvalid(_))));
    }

    #[test]
    fn vcard_version_4() {
        let result = parse_to_vcards("BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nEND:VCARD");
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn vcard_concat_tab() {
        let text = "BEGIN:VCARD\r\nVERSION:4.0\r\nFN:John Doe\r\nN:Doe;John\r\n\t;Jr.;;\r\nEND:VCARD";
        let result = parse_to_vcards(text);
        assert!(matches!(result, Ok(_)));
        let vcards = result.unwrap();
        assert_eq!(vcards.len(), 1);
    }

    #[test]
    fn sample_minimal() {
        let text = r#"
BEGIN:VCARD
VERSION:4.0
FN:John Doe
END:VCARD
"#;
        let result = parse_to_vcards(text);
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn sample_single() {
        let text = r#"
BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;type=INTERNET;type=HOME;type=pref:user@example.com
EMAIL;type=INTERNET;type=WORK:acme@example.com
TEL;type=CELL;type=VOICE;type=pref:+1 (555) 555-5555
TEL;type=IPHONE;type=CELL;type=VOICE:+1 (555) 555-5550
ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
ADR;type=WORK:;;First St SE;Washington;DC;20004;United States
NOTE:Lorem ipsum dolor sit amet\, consectetur adipiscing elit\, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam\, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident\, sunt in culpa qui officia deserunt mollit anim id est laborum.
BDAY:2000-01-01
END:VCARD
"#;
        let result = parse_to_vcards(text);
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn sample_multiple() {
        let text = r#"BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;type=INTERNET;type=HOME;type=pref:user@example.com
EMAIL;type=INTERNET;type=WORK:acme@example.com
TEL;type=CELL;type=VOICE;type=pref:+1 (555) 555-5555
TEL;type=IPHONE;type=CELL;type=VOICE:+1 (555) 555-5550
ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
ADR;type=WORK:;;First St SE;Washington;DC;20004;United States
NOTE:Lorem ipsum dolor sit amet\, consectetur adipiscing elit\, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam\, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident\, sunt in culpa qui officia deserunt mollit anim id est laborum.
BDAY:2000-01-01
END:VCARD
BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;type=INTERNET;type=HOME;type=pref:user@example.com
EMAIL;type=INTERNET;type=WORK:acme@example.com
TEL;type=CELL;type=VOICE;type=pref:+1 (555) 555-5555
TEL;type=IPHONE;type=CELL;type=VOICE:+1 (555) 555-5550
ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
ADR;type=WORK:;;First St SE;Washington;DC;20004;United States
NOTE:Lorem ipsum dolor sit amet\, consectetur adipiscing elit\, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam\, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident\, sunt in culpa qui officia deserunt mollit anim id est laborum.
BDAY:2000-01-01
END:VCARD
"#;
        let result = parse_to_vcards(text);
        match result {
            Ok(_) => {
                assert_eq!(result.unwrap().len(), 2);
            }
            Err(_) => {
                assert!(matches!(result, Err(VcardError::PropertyValueInvalid(_))));
            }
        }
    }

    #[test]
    fn sample_with_concat() {
        let text = r#"
BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;type=INTERNET;type=HOME;type=pref:user@example.com
EMAIL;type=INTERNET;type=WORK:acme@example.com
TEL;type=CELL;type=VOICE;type=pref:+1 (555) 555-5555
TEL;type=IPHONE;type=CELL;type=VOICE:+1 (555) 555-5550
ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;
 Washington;DC;20500;
 United States
ADR;type=WORK:;;First St SE;Washington;DC;20004;United States
NOTE:Lorem ipsum dolor sit amet\, consectetur adipiscing elit\, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam\, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident\, sunt in culpa qui officia deserunt mollit anim id est laborum.
BDAY:2000-01-01
END:VCARD
"#;
        let result = parse_to_vcards(text);
        assert_eq!(result.unwrap().len(), 1);
    }
}
