use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum VcardError {
    #[doc = "Signifies that the parameter value wasn't able to be parsed."]
    #[error("Paramater string (`{0}`) malformed for type: `{1}`.")]
    ParameterMalformed(String, String),
    #[doc = "Signifies that the parameter type isn't allowed for the property type."]
    #[error("Parameter {0} is not allowed for {1}.")]
    ParameterTypeNotAllowed(String, String),
    #[doc = "Signifies that the parameter type wasn't a known type."]
    #[error("Unknown parameter type {0}.")]
    ParameterTypeUnknown(String),
    #[doc = "Signifies an attempt to add a property that has a single cardinality."]
    #[error("Attempted to add property `{0}`, which already exists.")]
    PropertyExists(String),
    #[doc = "Signifies that the vCard string was malformed. This is usually because the string is missing a colon or semi-colon."]
    #[error("String is malformed `{0}`.")]
    PropertyMalformedString(String),
    #[doc = "Signifies that the vCard was created without required properties (FN & VERSION)."]
    #[error("vCard is missing `{0}` property.")]
    PropertyMissing(String),
    #[doc = "Signifies attempted removal of a required property."]
    #[error("Property is required `{0}`.")]
    PropertyRequired(String),
    #[doc = "Signifies attempt to  create a property with an unknown property type."]
    #[error("Property not found for uuid: `{0}`.")]
    PropertyTypeUnknown(String),
    #[doc = "Signifies a validation error for a specific property type."]
    #[error("Invalid property value `{0}`.")]
    PropertyValueInvalid(String),
    #[doc = "Signifies attempted creation of a property with the parameter VALUE set to a restricted value type."]
    #[error("Value type `{0}` not allowed for `{1}`.")]
    ValueKindNotAllowed(String, String),
    #[doc = "Signifies attempt to create a vCard with a non compatible version."]
    #[error("Invalid vCard version `{0}`")]
    VersionInvalid(String),
}
