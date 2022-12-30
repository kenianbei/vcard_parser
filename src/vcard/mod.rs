use std::fmt::{Display, Formatter};

use uuid::Uuid;

use crate::vcard::property::types::{PropertyType, PROPERTY_TYPE_VERSION};
use crate::vcard::property::Property;
use crate::VcardError;

/// Stores parameter data such as type and value.
pub mod parameter;

/// Property specific handlers for each property type (e.g. Address, Birthday, etc...).
mod properties;

/// Store property data, including both parameters and property values.
pub mod property;

/// Store both property value data and parameter value data.
pub mod values;

/// List of required properties.
pub const REQUIRED: [PropertyType; 2] = [PropertyType::Version, PropertyType::Fn];

/// List of properties that can only be represented once in a vCard.
pub const SINGLE: [PropertyType; 11] = [PropertyType::Anniversary, PropertyType::BDay, PropertyType::BirthPlace, PropertyType::DeathDate, PropertyType::DeathPlace, PropertyType::Gender, PropertyType::Kind, PropertyType::N, PropertyType::ProdId, PropertyType::Rev, PropertyType::Uid];

#[derive(Debug, Clone)]
/// The main vCard object. It is a collection of properties that follow the RFC 6350 vCard specification.
///
/// A vCard can be created individually using [Vcard::default](Vcard::default), [Vcard::try_from](Vcard::try_from<&str>()) and [Vcard::from](Vcard::from) or as an array of vCard objects
/// using the main [parse_to_vcards](super::parse_to_vcards) and [parse_to_vcards_without_errors](super::parse_to_vcards_without_errors) functions.
///
/// # Examples
///
/// ## Creating a new vCard
/// ```
/// use vcard_parser::vcard::Vcard;
///
/// let mut vcard = Vcard::default();
/// vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
/// println!("{}", vcard.to_string());
/// ```
///
/// ## Parsing a single vCard with error checks.
/// ```
/// use vcard_parser::vcard::Vcard;
///
/// let mut vcard = Vcard::try_from("VERSION:4.0\nFN:John Doe\n").expect("Unable to parse input.");
/// vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
/// println!("{}", vcard.to_string());
/// ```
///
/// ## Parsing a single vCard without error checks.
/// ```
/// use vcard_parser::vcard::Vcard;
///
/// let mut vcard = Vcard::from("VERSION:4.0\nFN:John Doe\n");
/// vcard.validate_vcard().expect("Invalid vCard.");
/// vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
/// println!("{}", vcard.to_string());
/// ```
pub struct Vcard {
    properties: Vec<Property>,
}

impl Vcard {
    /// Create a new vCard from a string, ignoring all errors.
    /// After creation of the vCard with this method, it's a good idea to validate the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let vcard = Vcard::from("VERSION:4.0\nFN:John Doe");
    /// vcard.validate_vcard().expect("Invalid vCard.");
    /// ```
    pub fn from(str: &str) -> Self {
        let mut properties = Vec::new();

        for str in str.lines() {
            if let Ok(property) = Property::try_from((str, None)) {
                properties.push(property);
            }
        }

        Vcard { properties }
    }

    /// Create a new vCard from a fullname.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let vcard = Vcard::from_fullname("John Doe").expect("Unable to create vCard.");
    /// ```
    pub fn from_fullname(str: &str) -> Result<Vcard, VcardError> {
        let mut vcard = Vcard::default();

        if let Some(property) = vcard.get_property_by_type(&PropertyType::Fn) {
            vcard.update_property(property.get_uuid(), format!("FN:{}", str).as_str())?;
        }

        Ok(vcard)
    }

    /// Add a property to the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// ```
    pub fn add_property(&mut self, str: &str) -> Result<Uuid, VcardError> {
        let property = Property::try_from((str, None))?;
        let property_type = property.get_type();

        if self.get_property_by_type(property_type).is_some() {
            if SINGLE.iter().any(|p| p == property_type) {
                return Err(VcardError::PropertyExists(property_type.to_string()));
            }
            if REQUIRED.iter().any(|p| p == property_type) {
                return Err(VcardError::PropertyExists(property_type.to_string()));
            }
        }

        self.properties.push(property.clone());

        Ok(property.get_uuid())
    }

    /// Get a property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// let uuid = vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// let property = vcard.get_property(uuid).unwrap();
    /// ```
    pub fn get_property(&self, uuid: Uuid) -> Option<&Property> {
        match self.get_property_index(uuid) {
            None => None,
            Some(index) => self.properties.get(index),
        }
    }

    /// Update a property in the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// let uuid = vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// vcard.update_property(uuid, "NICKNAME:Johnny Five").expect("Unable to update property.");
    ///
    /// ```
    pub fn update_property(&mut self, uuid: Uuid, str: &str) -> Result<bool, VcardError> {
        if let Some(index) = self.get_property_index(uuid) {
            self.properties[index] = Property::try_from((str, Some(uuid)))?;
            self.validate_vcard()?;
            return Ok(true);
        }
        Ok(false)
    }

    /// Remove a property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// let uuid = vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// vcard.remove_property(uuid).expect("Unable to remove property.");
    /// ```
    pub fn remove_property(&mut self, uuid: Uuid) -> Result<bool, VcardError> {
        if let Some(property) = self.get_property(uuid) {
            if REQUIRED.iter().any(|p| p == property.get_type()) {
                return Err(VcardError::PropertyRequired(property.get_type().to_string()));
            }
            if let Some(index) = self.get_property_index(uuid) {
                self.properties.remove(index);
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Get a property by type from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::types::PropertyType;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// vcard.add_property("BDAY:20000101").expect("Unable to add property.");
    /// let property = vcard.get_property_by_type(&PropertyType::BDay).unwrap();
    /// ```
    pub fn get_property_by_type(&self, property_type: &PropertyType) -> Option<&Property> {
        if !SINGLE.iter().any(|p| p == property_type) && !REQUIRED.iter().any(|p| p == property_type) {
            return None;
        }

        if let Some(property) = self.properties.iter().find(|p| p.get_type() == property_type) {
            return Some(property);
        }

        None
    }

    /// Get the index of a property by uuid.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// let uuid = vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// let index = vcard.get_property_index(uuid).unwrap();
    /// ```
    pub fn get_property_index(&self, uuid: Uuid) -> Option<usize> {
        self.properties.iter().position(|p| p.get_uuid() == uuid)
    }

    /// Get all properties from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// let properties = vcard.get_properties();
    /// ```
    pub fn get_properties(&self) -> &Vec<Property> {
        &self.properties
    }

    /// Get properties by type from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::types::PropertyType;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// vcard.add_property("NICKNAME:Johnny").expect("Unable to add property.");
    /// let property = vcard.get_properties_by_type(&PropertyType::NickName);
    /// ```
    pub fn get_properties_by_type(&self, property_type: &PropertyType) -> Vec<Property> {
        self.get_properties().iter().cloned().filter(|property| property.get_type() == property_type).collect()
    }

    /// Validate that the vCard has all required properties and that the version is valid.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::default();
    /// vcard.validate_vcard().expect("Invalid vCard.");
    /// ```
    pub fn validate_vcard(&self) -> Result<(), VcardError> {
        self.validate_vcard_required_properties()?;
        self.validate_vcard_version()?;
        Ok(())
    }

    /// Validates a vCard has all required properties.
    fn validate_vcard_required_properties(&self) -> Result<(), VcardError> {
        let mut properties = self.get_properties().iter();
        for property_type in REQUIRED {
            if !properties.any(|property| property.get_type() == &property_type) {
                return Err(VcardError::PropertyMissing(property_type.to_string()));
            }
        }
        Ok(())
    }

    /// Validates a vCard is version 4.0.
    fn validate_vcard_version(&self) -> Result<(), VcardError> {
        if let Some(version) = self.get_property_by_type(&PropertyType::Version) {
            let value = version.get_value().to_string();
            if value == "4.0" {
                return Ok(());
            }
            return Err(VcardError::VersionInvalid(value));
        }
        Err(VcardError::PropertyMissing(PROPERTY_TYPE_VERSION.to_string()))
    }
}

impl Default for Vcard {
    fn default() -> Self {
        Vcard {
            properties: Vec::from([Property::from(PropertyType::Version), Property::from(PropertyType::Fn), Property::from(PropertyType::Uid)]),
        }
    }
}

impl TryFrom<&str> for Vcard {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let mut properties = Vec::new();

        for str in str.lines() {
            properties.push(Property::try_from((str, None))?);
        }

        let vcard = Vcard { properties };
        vcard.validate_vcard()?;

        Ok(vcard)
    }
}

impl Display for Vcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "BEGIN:VCARD")?;
        for property in self.get_properties().iter() {
            writeln!(f, "{}", property)?;
        }
        writeln!(f, "END:VCARD")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_to_strings, Vcard};

    #[test]
    pub fn vcard_import_export() {
        let text = r#"BEGIN:VCARD
VERSION:4.0
N:Doe;John;;;
FN:John Doe
ORG:ACME Inc.;
EMAIL;TYPE=INTERNET;TYPE=HOME;TYPE=pref:user@example.com
EMAIL;TYPE=INTERNET;TYPE=WORK:acme@example.com
TEL;TYPE=CELL;TYPE=VOICE;TYPE=pref:+1(555)555-5555
TEL;TYPE=IPHONE;TYPE=CELL;TYPE=VOICE:+1(555)555-5550
ADR;TYPE=HOME;TYPE=pref:;;1600 Pennsylvania Avenue NW;Washington;DC;20500;United States
ADR;TYPE=WORK:;;First St SE;Washington;DC;20004;United States
NOTE:Lorem ipsum dolor sit amet\, consectetur adipiscing elit\, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam\, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident\, sunt in culpa qui officia deserunt mollit anim id est laborum.
BDAY;VALUE=DATE-AND-OR-TIME:2000-01-01
END:VCARD
"#;
        let parsed_input = parse_to_strings(text).first().cloned().unwrap();
        let vcard = Vcard::try_from(parsed_input.as_str()).unwrap();
        assert_eq!(text, vcard.to_string());
    }

    #[test]
    pub fn vcard_crud() {
        let mut vcard = Vcard::default();

        let a = "TEL:+1 (555) 555-5555";
        let b = "TEL:+1 (555) 555-5556";

        let uuid = vcard.add_property(a).unwrap();
        assert_eq!(vcard.get_property(uuid).unwrap().get_value().to_string().as_str(), "+1(555)555-5555");

        vcard.update_property(uuid, b).unwrap();
        assert_eq!(vcard.get_property(uuid).unwrap().get_value().to_string().as_str(), "+1(555)555-5556");

        vcard.remove_property(uuid).unwrap();
        assert!(matches!(vcard.get_property(uuid), None));
    }
}
