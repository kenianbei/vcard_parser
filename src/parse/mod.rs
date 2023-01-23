//! Parsing module that relies on nom for heavy lifting.

pub mod delimiters;
pub mod encoding;
pub mod parameter;
pub mod property;
pub mod value;
pub mod vcard;

/// Represents basic data type that nom will parse.
pub type Data<'a> = &'a [u8];
/// Represents a parsed property.
pub type PropertyData<'a> = (PropertyNameWithGroupData<'a>, PropertyParametersData<'a>, ValueFoldedData<'a>);
/// Represents a parsed property group (optional).
pub type PropertyGroupData<'a> = Option<Data<'a>>;
/// Represents a parsed property name.
pub type PropertyNameData<'a> = Data<'a>;
/// Represents a parsed property name with optional group.
pub type PropertyNameWithGroupData<'a> = (Option<Data<'a>>, PropertyNameData<'a>);
/// Represents a parsed property parameters array.
pub type PropertyParametersData<'a> = Vec<ParameterData<'a>>;
/// Represents a parsed property parameter name and value.
pub type ParameterData<'a> = (Data<'a>, Data<'a>);
/// Represents a parsed property value with folded values.
pub type ValueFoldedData<'a> = (ValueData<'a>, Option<Vec<ValueData<'a>>>);
/// Represents a parsed property value.
pub type ValueData<'a> = Data<'a>;
/// Represents a parsed vCard.
pub type VcardData<'a> = Vec<PropertyData<'a>>;
