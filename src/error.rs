//! Error types and handling.

use std::fmt::{Display, Formatter};

use nom::error::{ContextError, ErrorKind, ParseError};

#[derive(Debug, Eq, PartialEq)]
pub enum VcardError {
    #[doc = "Signifies that a u8 array was not converted to UTF-8."]
    ConversionFailure,
    #[doc = "Signifies a parsing error."]
    ParseError(Vec<String>),
    #[doc = "Signifies that the parameter type isn't allowed for the property type."]
    ParameterTypeNotAllowed(String, String),
    #[doc = "Signifies that the vCard was parsed without FN property."]
    PropertyFnMissing,
    #[doc = "Signifies attempted removal of a required property."]
    PropertyFnRequired,
    #[doc = "Signifies an error retrieving a property after setting it."]
    PropertySetError(String),
    #[doc = "Signifies a validation error for a value."]
    ValueInvalid(String, String),
    #[doc = "Signifies value name is not known."]
    ValueNameUnknown(String),
    #[doc = "Signifies attempted creation of a property with the parameter VALUE set to a another value type."]
    ValueNotAllowed(String, String),
    #[doc = "Signifies attempted creation of a property with the parameter VALUE set to a another value type."]
    ValueMismatch(String, String, String),
    #[doc = "Signifies that a value string was malformed."]
    ValueMalformed(String),
}

impl VcardError {
    pub fn parse_error(&self) -> String {
        match self {
            VcardError::ParseError(v) => {
                if let Some(s) = v.get(1) {
                    s.to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        }
    }
}

impl From<nom::Err<VcardError>> for VcardError {
    fn from(err: nom::Err<VcardError>) -> Self {
        let mut errors = Vec::new();

        err.map(|e| {
            if let VcardError::ParseError(e) = e {
                for a in e {
                    errors.push(a)
                }
            }
        });

        Self::ParseError(errors)
    }
}

impl From<String> for VcardError {
    fn from(err: String) -> Self {
        Self::ParseError(Vec::from([err]))
    }
}

impl ParseError<&[u8]> for VcardError {
    fn from_error_kind(input: &[u8], _: ErrorKind) -> Self {
        if let Ok(string) = String::from_utf8(input.to_vec()) {
            Self::ParseError(Vec::from([string]))
        } else {
            Self::ParseError(Vec::new())
        }
    }

    fn append(_: &[u8], _: ErrorKind, other: Self) -> Self {
        if let VcardError::ParseError(v) = other {
            Self::ParseError(v)
        } else {
            Self::ParseError(Vec::new())
        }
    }
}

impl ContextError<&[u8]> for VcardError {
    fn add_context(_: &[u8], ctx: &'static str, other: Self) -> Self {
        if let VcardError::ParseError(mut v) = other {
            v.push(ctx.to_string());
            Self::ParseError(v)
        } else {
            Self::ParseError(Vec::from([ctx.to_string()]))
        }
    }
}

impl Display for VcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VcardError::ConversionFailure => write!(f, "Unable to convert string to UTF8."),
            VcardError::ParseError(v) => write!(f, "{}", v.join(",")),
            VcardError::ParameterTypeNotAllowed(parameter_name, property_name) => write!(f, "Parameter {} is not allowed for {}.", parameter_name, property_name),
            VcardError::PropertyFnMissing => write!(f, "vCard is missing FN property."),
            VcardError::PropertyFnRequired => write!(f, "Property FN is required."),
            VcardError::PropertySetError(property) => write!(f, "There was an issue setting {} property.", property),
            VcardError::ValueInvalid(property_value, property_name) => write!(f, "Invalid value {} for {}.", property_value, property_name),
            VcardError::ValueNotAllowed(string, property_name) => write!(f, "Value type {} not allowed for {}.", string, property_name),
            VcardError::ValueMismatch(property_value, a, b) => write!(f, "Value {} does not match required type {} for {}.", property_value, a, b),
            VcardError::ValueMalformed(property_value) => write!(f, "Unable to parse value from {}.", property_value),
            VcardError::ValueNameUnknown(name) => write!(f, "Unknown value name: {}.", name),
        }
    }
}
