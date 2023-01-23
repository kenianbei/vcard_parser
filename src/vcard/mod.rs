//! The vcard module represents data that has been parsed as per the [RFC 6350](https://datatracker.ietf.org/doc/html/rfc6350) vCard specification.
//!
//! The main [`Vcard`] object contains an optional client id and an array of [properties](property).
//! Each property holds an optional group name, an array of [parameters](parameter), and a property
//! [value](value).
//!
//! A vCard can be created individually using [`Vcard::new`] or [`Vcard::try_from`].
//!
//! # Examples
//!
//! ## Creating a new vCard
//! ```
//! use vcard_parser::vcard::Vcard;
//!
//! let mut vcard = Vcard::new("John Doe");
//! ```
//!
//! ## Parsing a single vCard.
//! ```
//! use vcard_parser::vcard::Vcard;
//!
//! let text = "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n";
//!
//! // Create a basic vCard without attached client device uuid.
//! let mut vcard = Vcard::try_from(text).expect("Unable to parse input.");
//!
//! // Create a basic vCard with attached client device uuid.
//! let mut vcard = Vcard::try_from(("urn:uuid:some-uuid", text)).expect("Unable to parse input.");
//! ```

use std::fmt::{Display, Formatter};

use crate::constants::{ParameterName, PropertyName};
use crate::parse::VcardData;
use crate::vcard::parameter::Parameter;
use crate::vcard::property::property_fn::PropertyFnData;
use crate::vcard::value::value_clientpidmap::ValueClientPidMapData;
use crate::vcard::value::Value::ValueClientPidMap;
use crate::Property::PropertyFn;
use crate::{parse, HasCardinality, HasName, HasParameters, HasValue, Property, VcardError};

pub mod parameter;
pub mod property;
pub mod value;

#[derive(Clone, Debug)]
pub struct Vcard {
    client: Option<String>,
    properties: Vec<Property>,
}

impl Vcard {
    /// Create a new vCard from the FN property.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// assert_eq!(vcard.get_properties().len(), 1);
    /// ```
    pub fn new(str: &str) -> Self {
        Vcard {
            client: None,
            properties: Vec::from([PropertyFn(
                PropertyFnData::from(str),
            )]),
        }
    }

    /// Export a vcard without any clientpidmap or pid information.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let text = "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n";
    ///
    /// let mut vcard = Vcard::try_from(text).expect("Unable to parse vCard.");
    /// assert_eq!(vcard.export(), text);
    /// ```
    pub fn export(&self) -> String {
        let mut string = String::new();

        string.push_str("BEGIN:VCARD\n");
        string.push_str("VERSION:4.0\n");

        for property in self.get_properties().iter() {
            if property.name() != PropertyName::CLIENTPIDMAP {
                string.push_str(&property.export())
            }
        }

        string.push_str("END:VCARD\n");

        string
    }

    /// Get a single cloned property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let property = vcard.get_property(&property);
    /// assert!(property.is_some());
    /// ```
    pub fn get_property(&mut self, property: &Property) -> Option<Property> {
        if let Some(i) = self.get_property_index(property) {
            return self.properties.get(i).cloned();
        }
        None
    }

    /// Get a reference to a single property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let property = vcard.get_property(&property);
    /// assert!(property.is_some());
    /// ```
    pub fn get_property_ref(&mut self, property: &Property) -> Option<&Property> {
        if let Some(i) = self.get_property_index(property) {
            return self.properties.get(i);
        }
        None
    }

    /// Get a mutable reference to a single property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let property = vcard.get_property(&property);
    /// assert!(property.is_some());
    /// ```
    pub fn get_property_mut(&mut self, property: &Property) -> Option<&mut Property> {
        if let Some(i) = self.get_property_index(property) {
            return self.properties.get_mut(i);
        }
        None
    }

    /// Get cloned copy of a single property by property name.
    ///
    /// This will only match properties that have a single cardinality.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("BDAY:20000101\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let property = vcard.get_property_by_name("BDAY");
    /// assert!(property.is_some());
    /// ```
    pub fn get_property_by_name(&mut self, str: &str) -> Option<Property> {
        if let Some(property) = self.properties.iter_mut().find(|p| p.name() == str && p.is_single()) {
            return Some(property.clone());
        }

        None
    }

    /// Get a cloned copy of properties filtered by name from the vCard.
    ///
    /// This will only match properties that have multiple cardinality.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    ///
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let properties = vcard.get_properties_by_name("NICKNAME");
    /// assert_eq!(properties.len(), 1);
    ///
    /// let property = Property::try_from("NICKNAME:Jonathon\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// let properties = vcard.get_properties_by_name("NICKNAME");
    /// assert_eq!(properties.len(), 2);
    /// ```
    pub fn get_properties_by_name(&self, str: &str) -> Vec<Property> {
        self.get_properties().iter().cloned().filter(|p| p.name() == str && p.is_multiple()).collect()
    }

    /// Get a cloned copy of all properties from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::try_from("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n").expect("Unable to parse vCard.");
    /// let properties = vcard.get_properties();
    /// assert_eq!(properties.len(), 1);
    /// ```
    pub fn get_properties(&self) -> Vec<Property> {
        self.properties.clone()
    }

    /// Remove a property from the vCard.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// if vcard.remove_property(&property).expect("Unable to remove property.") {
    ///     assert!(vcard.get_property(&property).is_none());
    /// }
    /// ```
    pub fn remove_property(&mut self, property: &Property) -> Result<bool, VcardError> {
        if property.name() == PropertyName::FN {
            return Err(VcardError::PropertyFnRequired);
        }

        if let Some(index) = self.get_property_index(property) {
            self.properties.remove(index);
            return Ok(true);
        }

        Ok(false)
    }

    /// Sets a property. If the property matches an existing property, the existing property will be replaced.
    /// If there is no match, a new property will be added.
    ///
    /// Returns a clone of the property which will include pid information for later matching.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::Vcard;
    ///
    /// let mut vcard = Vcard::new("John Doe");
    /// let property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property string.");
    /// let property = vcard.set_property(&property).expect("Unable to add property.");
    /// assert!(vcard.get_property(&property).is_some());
    /// ```
    pub fn set_property(&mut self, property: &Property) -> Result<Property, VcardError> {
        let mut property = property.clone();

        // Add pid information to the property if it doesn't match an existing property.
        if property.is_multiple() && property.name() != PropertyName::CLIENTPIDMAP && property.allowed_parameters().contains(&ParameterName::PID) {
            if None == self.get_property_index(&property) {
                let count = self.get_properties_by_name(property.name()).len();
                let string = {
                    if let Some(clientpidmap) = self.get_clientpidmap() {
                        format!(";PID={}.{}", count + 1, clientpidmap.id)
                    } else {
                        format!(";PID={}", count + 1)
                    }
                };
                property.add_parameter(Parameter::try_from(string.as_str())?)?;
            }
        }

        // Update or add property depending on match.
        if let Some(i) = self.get_property_index(&property) {
            self.properties[i] = property.clone();
            Ok(property)
        } else {
            self.properties.push(property.clone());
            Ok(property)
        }
    }

    /// Helper function for matching properties and returning their index in the properties array.
    fn get_property_index(&self, property: &Property) -> Option<usize> {
        for (i, other) in self.properties.iter().enumerate() {
            if property == other {
                return Some(i);
            }
        }
        None
    }

    /// Get the clientpidmap matching the client managing this vCard.
    fn get_clientpidmap(&self) -> Option<ValueClientPidMapData> {
        if let Some(client) = &self.client {
            for property in self.get_properties_by_name(PropertyName::CLIENTPIDMAP) {
                if let ValueClientPidMap(clientpidmap) = property.get_value() {
                    if clientpidmap.client.as_str() == client {
                        return Some(clientpidmap.clone());
                    }
                }
            }
        }
        None
    }
}

impl TryFrom<&str> for Vcard {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let (_, properties) = parse::vcard::vcard(str.as_bytes())?;
        Self::try_from((None, properties))
    }
}

impl TryFrom<(&str, &str)> for Vcard {
    type Error = VcardError;
    fn try_from((client, str): (&str, &str)) -> Result<Self, Self::Error> {
        let (_, properties) = parse::vcard::vcard(str.as_bytes())?;
        Self::try_from((Some(client.to_string()), properties))
    }
}

impl<'a> TryFrom<(Option<String>, VcardData<'a>)> for Vcard {
    type Error = VcardError;
    fn try_from((client, data): (Option<String>, VcardData<'a>)) -> Result<Self, Self::Error> {
        let mut properties = Vec::new();

        for datum in data {
            properties.push(Property::create_from_data(datum)?)
        }

        Self::try_from((client, properties))
    }
}

impl TryFrom<(Option<String>, Vec<Property>)> for Vcard {
    type Error = VcardError;
    fn try_from((client, properties): (Option<String>, Vec<Property>)) -> Result<Self, Self::Error> {
        let mut vcard = Self { client, properties: Vec::new() };

        if let Some(client) = &vcard.client {
            vcard.set_property(&Property::create_from_str(format!("CLIENTPIDMAP:1;{}\n", client).as_str())?)?;
        }

        for property in properties {
            vcard.set_property(&property)?;
        }

        if vcard.get_property_by_name(PropertyName::FN).is_none() {
            return Err(VcardError::PropertyFnMissing);
        }

        Ok(vcard)
    }
}

impl Display for Vcard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "BEGIN:VCARD\n")?;
        write!(f, "VERSION:4.0\n")?;
        for property in self.get_properties().iter() {
            write!(f, "{}", property)?;
        }
        write!(f, "END:VCARD\n")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::ValueName;
    use crate::vcard::value::Value;
    use crate::{HasValue, Property, Vcard};

    #[test]
    pub fn vcard_new() {
        assert_eq!(Vcard::new("John Doe").to_string(), "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n");
    }

    #[test]
    pub fn vcard_export() {
        assert_eq!(Vcard::new("John Doe").export(), "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n");
    }

    #[test]
    pub fn vcard_property_operations() {
        let mut vcard = Vcard::new("John Doe");

        // Test getting properties.
        assert_eq!(vcard.get_properties().len(), 1);
        assert_eq!(vcard.get_properties_by_name("FN").len(), 0);

        // Test setting single property via set property.
        let n_property = vcard.set_property(&Property::try_from("N:Doe;John;;;\n").unwrap()).unwrap();
        assert_eq!(vcard.get_property_by_name("N").unwrap().to_string(), "N:Doe;John;;;\n");

        // Test directly setting single property value on mut ref.
        let n_property = vcard.get_property_mut(&n_property).unwrap();
        n_property.set_value(Value::try_from((ValueName::LISTCOMPONENT, "Doe;Jonathan;;;")).unwrap()).unwrap();
        assert_eq!(vcard.get_property_by_name("N").unwrap().to_string(), "N:Doe;Jonathan;;;\n");

        // Test setting multiple property via set property.
        let nickname_property = vcard.set_property(&Property::try_from("NICKNAME:Johnny\n").unwrap()).unwrap();
        assert_eq!(vcard.get_properties_by_name("NICKNAME").len(), 1);
        assert_eq!(vcard.get_properties_by_name("NICKNAME").first().unwrap().export(), "NICKNAME:Johnny\n");

        // Test removing a multiple property.
        vcard.remove_property(&nickname_property).unwrap();
        assert_eq!(vcard.get_properties_by_name("NICKNAME").len(), 0);

        // Test removing a fn property.
        assert!(Vcard::new("John Doe").remove_property(&vcard.get_property_by_name("FN").unwrap()).is_err());
    }
}
