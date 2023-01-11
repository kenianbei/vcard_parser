//! # vCard Parser
//!
//! Parses and validates vCard data according to RFC 6350 specification.
//!
//! ## Creating vCards
//!
//! ```rust
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcard = Vcard::default();
//! vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
//! println!("{}", vcard.to_string());
//! ```
//!
//! ## Parsing vCards
//!
//! vCards can be parsed from a string containing multiple vCards or a single vCard.
//!
//! When parsing you typically would just use the [parse_to_vcards](parse_to_vcards) or [parse_to_vcards_without_errors](parse_to_vcards_without_errors)
//! functions. You can also use [Vcard::from](Vcard::from) and [Vcard::try_from](Vcard::try_from<&str>()), but without passing
//! BEGIN:VCARD and END:VCARD delimiters. See the main [Vcard](Vcard) struct for more information.
//!
//! ### Parsing from file
//!
//! Read a vcf file and ignore invalid properties, update the vCard object, and write back to file.
//!
//! ```rust
//! use std::fs;
//! use std::fs::read_to_string;
//! use vcard_parser::parse_to_vcards_without_errors;
//! use vcard_parser::vcard::property::types::PropertyType;
//!
//! if let Ok(string) = read_to_string("contacts.vcf") {
//!     let mut vcards = parse_to_vcards_without_errors(string.as_str());
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

/// Parse a string and return an array of individual vCard strings as the result.
///
/// This function will capture all strings delimited by BEGIN:VCARD and END:VCARD, returning an array
/// of strings without the delimiters.
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
/// let vcard_strings = parse_to_strings(text);
/// assert_eq!(vcard_strings.len(), 1);
/// ```
pub fn parse_to_strings(input: &str) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();

    let input = Regex::new(r"(?mi)^\s*(BEGIN|END):VCARD\s*?$").unwrap().replace_all(input, "$1:VCARD");
    let input = Regex::new(r"(?mi)\n\s").unwrap().replace_all(&*input, "");

    if let Ok(regex) = Regex::new(r"(?mi)\s*?BEGIN:VCARD\s*?$\n([\s\S]*?)\s*?END:VCARD\s*?$\n?") {
        for captures in regex.captures_iter(&*input) {
            if let Some(capture) = captures.get(1) {
                data.push(capture.as_str().to_string())
            }
        }
    }

    data
}

/// Parses a string and returns either a [VcardError](VcardError) or an array of [Vcard](Vcard)s as the result.
///
/// The input string can be a single vCard or multiple vCards, and should be delimited by BEGIN:VCARD and END:VCARD.
///
/// This function will fail on vCards with x-param or iana-token types, as well as custom parameters. Use the other
/// version [parse_to_vcards_without_errors](parse_to_vcards_without_errors) to ignore properties that are unknown or contain unknown properties
/// or custom parameters.
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

    for string in parse_to_strings(input) {
        vcards.push(Vcard::try_from(string.as_str())?);
    }

    Ok(vcards)
}

/// Parses a vCard string and returns an array of vCards.
///
/// The input string can be a single vCard or multiple vCards, and should be delimited by BEGIN:VCARD and END:VCARD.
///
/// This function will ignore x-param or iana-token properties, as well as custom parameters. Use the other version
/// [parse_to_vcards](parse_to_vcards) if you vcards with x-param, iana-token, or other custom types to fail completely.
///
/// # Examples
/// ```
/// use vcard_parser::parse_to_vcards_without_errors;
///
/// let text = r#"
/// BEGIN:VCARD
/// VERSION:4.0
/// FN:John Doe
/// END:VCARD
/// "#;
///
/// let vcards = parse_to_vcards_without_errors(text);
/// assert_eq!(vcards.len(), 1);
/// ```
pub fn parse_to_vcards_without_errors(input: &str) -> Vec<Vcard> {
    let mut vcards = Vec::new();

    for string in parse_to_strings(input) {
        vcards.push(Vcard::from(string.as_str()));
    }

    vcards
}

#[cfg(test)]
mod tests {
    use crate::{parse_to_vcards, parse_to_vcards_without_errors, VcardError};
    use crate::vcard::property::types::PropertyType;

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
    fn vcard_x_param_single() {
        let text = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
X-ABADR:us
END:VCARD
"#;
        let expected = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
ADR;TYPE=HOME;TYPE=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
END:VCARD
"#;
        let result = parse_to_vcards_without_errors(text);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), expected);
    }

    #[test]
    fn vcard_x_param_grouped() {
        let text = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
item1.ADR;type=HOME;type=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
item1.X-ABADR:us
END:VCARD
"#;
        let expected = r#"BEGIN:VCARD
VERSION:4.0
FN:John Doe
ADR;TYPE=HOME;TYPE=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
END:VCARD
"#;
        let result = parse_to_vcards_without_errors(text);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].to_string(), expected);
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

    #[test]
    fn sample_with_compound() {
        let text = r#"
BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;type="INTERNET,HOME,pref":user@example.com
EMAIL;type="INTERNET,WORK":acme@example.com
TEL;type="CELL,VOICE,pref":+1 (555) 555-5555
TEL;type="IPHONE,CELL,VOICE":+1 (555) 555-5550
ADR;type="HOME,pref":;;1600 Pennsylvania Avenue NW;
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
