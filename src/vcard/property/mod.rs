//! The property module represents a single content property, as per [RFC 6350 Section 6](https://datatracker.ietf.org/doc/html/rfc6350#section-6)
//!
//! Properties can be created using [`Property::try_from`] or [`Property::default`]. The format should strictly follow RFC guidelines,
//! e.g. "FN:John Doe\n" for the FN property with "John Doe" as the value. Parsed properties must end with a LF or CRLF ending as per [RFC 6350 Section 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3).
//!
//! Note that content properties exclude BEGIN, VERSION, and END properties.
//!
//! See the [parameter](`super::parameter`) and [value](`super::value`) modules for more information on constructing property values.
//!
//! # Examples
//!
//! ## Creating a new property.
//! ```
//! use vcard_parser::vcard::property::Property;
//!
//! let mut property = Property::default("N");
//! ```
//!
//! ## Parsing a property.
//! ```
//! use vcard_parser::vcard::property::Property;
//!
//! let mut property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property.");
//! ```
//!
//! ## Updating a property.
//! ```
//! use vcard_parser::traits::HasValue;
//! use vcard_parser::vcard::property::Property;
//! use vcard_parser::vcard::value::Value;
//!
//! let mut property = Property::try_from("NICKNAME:Johnny\n").expect("Unable to parse property.");
//! let updated = Value::try_from(("TEXTLIST", "Johnny Be Good")).expect("Unable to parse value.");
//! property.set_value(updated).expect("Unable to update property.");
//! assert_eq!(property.get_value().to_string(), "Johnny Be Good");
//! ```

use std::fmt::{Debug, Display, Formatter};

use crate::constants::{ParameterName, PropertyName};
use crate::parse::value::utf8_to_string;
use crate::parse::PropertyData;
use crate::traits::HasGroup;
use crate::vcard::parameter::Parameter;
use crate::vcard::property::property_adr::PropertyAdrData;
use crate::vcard::property::property_anniversary::PropertyAnniversaryData;
use crate::vcard::property::property_bday::PropertyBDayData;
use crate::vcard::property::property_birthplace::PropertyBirthPlaceData;
use crate::vcard::property::property_caladruri::PropertyCalAdrUriData;
use crate::vcard::property::property_caluri::PropertyCalUriData;
use crate::vcard::property::property_categories::PropertyCategoriesData;
use crate::vcard::property::property_clientpidmap::PropertyClientPidMapData;
use crate::vcard::property::property_contacturi::PropertyContactUriData;
use crate::vcard::property::property_deathdate::PropertyDeathDateData;
use crate::vcard::property::property_deathplace::PropertyDeathPlaceData;
use crate::vcard::property::property_email::PropertyEmailData;
use crate::vcard::property::property_expertise::PropertyExpertiseData;
use crate::vcard::property::property_fburl::PropertyFbUrlData;
use crate::vcard::property::property_fn::PropertyFnData;
use crate::vcard::property::property_gender::PropertyGenderData;
use crate::vcard::property::property_geo::PropertyGeoData;
use crate::vcard::property::property_hobby::PropertyHobbyData;
use crate::vcard::property::property_impp::PropertyImppData;
use crate::vcard::property::property_interest::PropertyInterestData;
use crate::vcard::property::property_key::PropertyKeyData;
use crate::vcard::property::property_kind::PropertyKindData;
use crate::vcard::property::property_lang::PropertyLangData;
use crate::vcard::property::property_logo::PropertyLogoData;
use crate::vcard::property::property_member::PropertyMemberData;
use crate::vcard::property::property_n::PropertyNData;
use crate::vcard::property::property_nickname::PropertyNickNameData;
use crate::vcard::property::property_note::PropertyNoteData;
use crate::vcard::property::property_org::PropertyOrgData;
use crate::vcard::property::property_orgdirectory::PropertyOrgDirectoryData;
use crate::vcard::property::property_photo::PropertyPhotoData;
use crate::vcard::property::property_prodid::PropertyProdIdData;
use crate::vcard::property::property_related::PropertyRelatedData;
use crate::vcard::property::property_rev::PropertyRevData;
use crate::vcard::property::property_role::PropertyRoleData;
use crate::vcard::property::property_sound::PropertySoundData;
use crate::vcard::property::property_source::PropertySourceData;
use crate::vcard::property::property_tel::PropertyTelData;
use crate::vcard::property::property_title::PropertyTitleData;
use crate::vcard::property::property_tz::PropertyTzData;
use crate::vcard::property::property_uid::PropertyUidData;
use crate::vcard::property::property_url::PropertyUrlData;
use crate::vcard::property::property_xml::PropertyXmlData;
use crate::vcard::property::property_xname::PropertyXNameData;
use crate::vcard::value::Value;
use crate::vcard::value::Value::ValuePid;
use crate::{parse, HasCardinality, HasName, HasParameters, HasValue, VcardError};

pub mod property_adr;
pub mod property_anniversary;
pub mod property_bday;
pub mod property_birthplace;
pub mod property_caladruri;
pub mod property_caluri;
pub mod property_categories;
pub mod property_clientpidmap;
pub mod property_contacturi;
pub mod property_deathdate;
pub mod property_deathplace;
pub mod property_email;
pub mod property_expertise;
pub mod property_fburl;
pub mod property_fn;
pub mod property_gender;
pub mod property_geo;
pub mod property_hobby;
pub mod property_impp;
pub mod property_interest;
pub mod property_key;
pub mod property_kind;
pub mod property_lang;
pub mod property_logo;
pub mod property_member;
pub mod property_n;
pub mod property_nickname;
pub mod property_note;
pub mod property_org;
pub mod property_orgdirectory;
pub mod property_photo;
pub mod property_prodid;
pub mod property_related;
pub mod property_rev;
pub mod property_role;
pub mod property_sound;
pub mod property_source;
pub mod property_tel;
pub mod property_title;
pub mod property_tz;
pub mod property_uid;
pub mod property_url;
pub mod property_xml;
pub mod property_xname;

#[derive(Clone, Debug)]
pub enum Property {
    /// Represents an ADR parameter, see [RFC 6350 6.3.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.3.1).
    PropertyAdr(PropertyAdrData),
    /// Represents an ANNIVERSARY parameter, see [RFC 6350 6.2.6](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.6).
    PropertyAnniversary(PropertyAnniversaryData),
    /// Represents an BDAY parameter, see [RFC 6350 6.2.5](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.5).
    PropertyBDay(PropertyBDayData),
    /// Represents an BIRTHPLACE parameter, see [RFC 6474 2.1](https://datatracker.ietf.org/doc/html/rfc6474#section-2.1).
    PropertyBirthPlace(PropertyBirthPlaceData),
    /// Represents an CALADRURI parameter, see [RFC 6350 6.9.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.9.2).
    PropertyCalAdrUri(PropertyCalAdrUriData),
    /// Represents an CALURI parameter, see [RFC 6350 6.9.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.9.3).
    PropertyCalUri(PropertyCalUriData),
    /// Represents an CATEGORIES parameter, see [RFC 6350 6.7.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.1).
    PropertyCategories(PropertyCategoriesData),
    /// Represents an CLIENTPIDMAP parameter, see [RFC 6350 6.7.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.1).
    PropertyClientPidMap(PropertyClientPidMapData),
    /// Represents an CONTACT parameter, see [RFC 8605 2.1](https://datatracker.ietf.org/doc/html/rfc8605#section-2.1).
    PropertyContactUri(PropertyContactUriData),
    /// Represents an DEATHDATE parameter, see [RFC 6474 2.3](https://datatracker.ietf.org/doc/html/rfc6474#section-2.3).
    PropertyDeathDate(PropertyDeathDateData),
    /// Represents an DEATHPLACE parameter, see [RFC 6474 2.2](https://datatracker.ietf.org/doc/html/rfc6474#section-2.2).
    PropertyDeathPlace(PropertyDeathPlaceData),
    /// Represents an EMAIL parameter, see [RFC 6350 6.4.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.4.2).
    PropertyEmail(PropertyEmailData),
    /// Represents an EXPERTISE parameter, see [RFC 6715 2.1](https://datatracker.ietf.org/doc/html/rfc6715#section-2.1).
    PropertyExpertise(PropertyExpertiseData),
    /// Represents an FBURL parameter, see [RFC 6350 6.9.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.9.1).
    PropertyFbUrl(PropertyFbUrlData),
    /// Represents an FN parameter, see [RFC 6350 6.2.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.1).
    PropertyFn(PropertyFnData),
    /// Represents an GENDER parameter, see [RFC 6350 6.2.7](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.7).
    PropertyGender(PropertyGenderData),
    /// Represents an GEO parameter, see [RFC 6350 6.5.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.5.2).
    PropertyGeo(PropertyGeoData),
    /// Represents an HOBBY parameter, see [RFC 6715 2.2](https://datatracker.ietf.org/doc/html/rfc6715#section-2.2).
    PropertyHobby(PropertyHobbyData),
    /// Represents an IMPP parameter, see [RFC 6350 6.4.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.4.3).
    PropertyImpp(PropertyImppData),
    /// Represents an INTEREST parameter, see [RFC 6715 2.3](https://datatracker.ietf.org/doc/html/rfc6715#section-2.3).
    PropertyInterest(PropertyInterestData),
    /// Represents an KEY parameter, see [RFC 6350 6.8.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.8.1).
    PropertyKey(PropertyKeyData),
    /// Represents an KIND parameter, see [RFC 6350 6.1.4](https://datatracker.ietf.org/doc/html/rfc6350#section-6.1.4).
    PropertyKind(PropertyKindData),
    /// Represents an LANG parameter, see [RFC 6350 6.4.4](https://datatracker.ietf.org/doc/html/rfc6350#section-6.4.4).
    PropertyLang(PropertyLangData),
    /// Represents an LOGO parameter, see [RFC 6350 6.6.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.3).
    PropertyLogo(PropertyLogoData),
    /// Represents an MEMBER parameter, see [RFC 6350 6.6.5](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.5).
    PropertyMember(PropertyMemberData),
    /// Represents an NICKNAME parameter, see [RFC 6350 6.2.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.3).
    PropertyNickName(PropertyNickNameData),
    /// Represents an NOTE parameter, see [RFC 6350 6.7.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.2).
    PropertyNote(PropertyNoteData),
    /// Represents an N parameter, see [RFC 6350 6.2.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.2).
    PropertyN(PropertyNData),
    /// Represents an ORG-DIRECTORY parameter, see [RFC 6715 2.4](https://datatracker.ietf.org/doc/html/rfc6715#section-2.4).
    PropertyOrgDirectory(PropertyOrgDirectoryData),
    /// Represents an ORG parameter, see [RFC 6350 6.6.4](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.4).
    PropertyOrg(PropertyOrgData),
    /// Represents an PHOTO parameter, see [RFC 6350 6.2.4](https://datatracker.ietf.org/doc/html/rfc6350#section-6.2.4).
    PropertyPhoto(PropertyPhotoData),
    /// Represents an PRODID parameter, see [RFC 6350 6.7.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.3).
    PropertyProdId(PropertyProdIdData),
    /// Represents an RELATED parameter, see [RFC 6350 6.6.6](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.6).
    PropertyRelated(PropertyRelatedData),
    /// Represents an REV parameter, see [RFC 6350 6.7.4](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.4).
    PropertyRev(PropertyRevData),
    /// Represents an ROLE parameter, see [RFC 6350 6.6.2](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.2).
    PropertyRole(PropertyRoleData),
    /// Represents an SOUND parameter, see [RFC 6350 6.7.5](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.5).
    PropertySound(PropertySoundData),
    /// Represents an SOURCE parameter, see [RFC 6350 6.1.3](https://datatracker.ietf.org/doc/html/rfc6350#section-6.1.3).
    PropertySource(PropertySourceData),
    /// Represents an TEL parameter, see [RFC 6350 6.4.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.4.1).
    PropertyTel(PropertyTelData),
    /// Represents an TITLE parameter, see [RFC 6350 6.6.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.6.1).
    PropertyTitle(PropertyTitleData),
    /// Represents an TZ parameter, see [RFC 6350 6.5.1](https://datatracker.ietf.org/doc/html/rfc6350#section-6.5.1).
    PropertyTz(PropertyTzData),
    /// Represents an UID parameter, see [RFC 6350 6.7.6](https://datatracker.ietf.org/doc/html/rfc6350#section-6.5.1).
    PropertyUid(PropertyUidData),
    /// Represents an URL parameter, see [RFC 6350 6.7.8](https://datatracker.ietf.org/doc/html/rfc6350#section-6.7.8).
    PropertyUrl(PropertyUrlData),
    /// Represents an XML parameter, see [RFC 6350 6.1.5](https://datatracker.ietf.org/doc/html/rfc6350#section-6.1.5).
    PropertyXml(PropertyXmlData),
    /// Represents an XNAME parameter, see [RFC 6350 3.3](https://datatracker.ietf.org/doc/html/rfc6350#section-3.3).
    PropertyXName(PropertyXNameData),
}

impl Property {
    /// Create a new property from required information (group, name, parameters, and value).
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::traits::HasValue;
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::value::Value;
    /// use vcard_parser::vcard::value::Value::ValueText;
    /// use vcard_parser::vcard::value::value_text::ValueTextData;
    ///
    /// let mut property = Property::default("FN");
    /// assert_eq!(property.export(), "FN:\n");
    ///
    /// property.set_value(Value::from(ValueTextData::from("John Doe"))).expect("Unable to set value.");
    /// assert_eq!(property.export(), "FN:John Doe\n");
    /// ```
    pub fn create((property_group, property_name, property_parameters, property_value): (Option<String>, &str, Vec<Parameter>, &str)) -> Result<Self, VcardError> {
        match property_name.to_uppercase().as_str() {
            PropertyName::ADR => Ok(Property::PropertyAdr(PropertyAdrData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::ANNIVERSARY => Ok(Property::PropertyAnniversary(PropertyAnniversaryData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::BDAY => Ok(Property::PropertyBDay(PropertyBDayData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::BIRTHPLACE => Ok(Property::PropertyBirthPlace(PropertyBirthPlaceData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::CALADRURI => Ok(Property::PropertyCalAdrUri(PropertyCalAdrUriData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::CALURI => Ok(Property::PropertyCalUri(PropertyCalUriData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::CATEGORIES => Ok(Property::PropertyCategories(PropertyCategoriesData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::CLIENTPIDMAP => Ok(Property::PropertyClientPidMap(PropertyClientPidMapData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::CONTACTURI => Ok(Property::PropertyContactUri(PropertyContactUriData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::DEATHDATE => Ok(Property::PropertyDeathDate(PropertyDeathDateData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::DEATHPLACE => Ok(Property::PropertyDeathPlace(PropertyDeathPlaceData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::EMAIL => Ok(Property::PropertyEmail(PropertyEmailData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::EXPERTISE => Ok(Property::PropertyExpertise(PropertyExpertiseData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::FBURL => Ok(Property::PropertyFbUrl(PropertyFbUrlData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::FN => Ok(Property::PropertyFn(PropertyFnData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::GENDER => Ok(Property::PropertyGender(PropertyGenderData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::GEO => Ok(Property::PropertyGeo(PropertyGeoData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::HOBBY => Ok(Property::PropertyHobby(PropertyHobbyData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::IMPP => Ok(Property::PropertyImpp(PropertyImppData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::INTEREST => Ok(Property::PropertyInterest(PropertyInterestData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::KEY => Ok(Property::PropertyKey(PropertyKeyData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::KIND => Ok(Property::PropertyKind(PropertyKindData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::LANG => Ok(Property::PropertyLang(PropertyLangData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::LOGO => Ok(Property::PropertyLogo(PropertyLogoData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::MEMBER => Ok(Property::PropertyMember(PropertyMemberData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::NICKNAME => Ok(Property::PropertyNickName(PropertyNickNameData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::NOTE => Ok(Property::PropertyNote(PropertyNoteData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::N => Ok(Property::PropertyN(PropertyNData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::ORGDIRECTORY => Ok(Property::PropertyOrgDirectory(PropertyOrgDirectoryData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::ORG => Ok(Property::PropertyOrg(PropertyOrgData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::PHOTO => Ok(Property::PropertyPhoto(PropertyPhotoData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::PRODID => Ok(Property::PropertyProdId(PropertyProdIdData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::RELATED => Ok(Property::PropertyRelated(PropertyRelatedData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::REV => Ok(Property::PropertyRev(PropertyRevData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::ROLE => Ok(Property::PropertyRole(PropertyRoleData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::SOUND => Ok(Property::PropertySound(PropertySoundData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::SOURCE => Ok(Property::PropertySource(PropertySourceData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::TEL => Ok(Property::PropertyTel(PropertyTelData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::TITLE => Ok(Property::PropertyTitle(PropertyTitleData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::TZ => Ok(Property::PropertyTz(PropertyTzData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::UID => Ok(Property::PropertyUid(PropertyUidData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::URL => Ok(Property::PropertyUrl(PropertyUrlData::try_from((property_group, property_value, property_parameters))?)),
            PropertyName::XML => Ok(Property::PropertyXml(PropertyXmlData::try_from((property_group, property_value, property_parameters))?)),
            _ => Ok(Property::PropertyXName(PropertyXNameData::try_from((property_group, property_name, property_value, property_parameters))?)),
        }
    }

    pub fn create_from_data(((group, name), parameters, (value, folds)): PropertyData) -> Result<Self, VcardError> {
        let property_name = utf8_to_string(name)?;

        let property_group = {
            if let Some(data) = group {
                Some(utf8_to_string(data)?)
            } else {
                None
            }
        };

        let mut property_parameters: Vec<Parameter> = Vec::new();
        for datum in parameters {
            property_parameters.push(Parameter::try_from(datum)?)
        }

        let mut property_value = Vec::from([utf8_to_string(value)?]);
        if let Some(v) = folds {
            for u in v {
                if let Ok(string) = utf8_to_string(u) {
                    property_value.push(string);
                }
            }
        }

        Self::create((property_group, property_name.as_str(), property_parameters, property_value.join("").as_str()))
    }

    pub fn create_from_str(str: &str) -> Result<Self, VcardError> {
        Self::create_from_data(parse::property::property(str.as_bytes())?.1)
    }

    /// Create a new property with default values.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::traits::HasValue;
    /// use vcard_parser::vcard::property::Property;
    /// use vcard_parser::vcard::value::Value;
    /// use vcard_parser::vcard::value::Value::ValueText;
    /// use vcard_parser::vcard::value::value_text::ValueTextData;
    ///
    /// let mut property = Property::default("FN");
    /// assert_eq!(property.export(), "FN:\n");
    ///
    /// property.set_value(Value::from(ValueTextData::from("John Doe"))).expect("Unable to set value.");
    /// assert_eq!(property.export(), "FN:John Doe\n");
    /// ```
    pub fn default(name: &str) -> Self {
        match name.to_uppercase().as_str() {
            PropertyName::ADR => Property::PropertyAdr(PropertyAdrData::default()),
            PropertyName::ANNIVERSARY => Property::PropertyAnniversary(PropertyAnniversaryData::default()),
            PropertyName::BDAY => Property::PropertyBDay(PropertyBDayData::default()),
            PropertyName::BIRTHPLACE => Property::PropertyBirthPlace(PropertyBirthPlaceData::default()),
            PropertyName::CALADRURI => Property::PropertyCalAdrUri(PropertyCalAdrUriData::default()),
            PropertyName::CALURI => Property::PropertyCalUri(PropertyCalUriData::default()),
            PropertyName::CATEGORIES => Property::PropertyCategories(PropertyCategoriesData::default()),
            PropertyName::CLIENTPIDMAP => Property::PropertyClientPidMap(PropertyClientPidMapData::default()),
            PropertyName::CONTACTURI => Property::PropertyContactUri(PropertyContactUriData::default()),
            PropertyName::DEATHDATE => Property::PropertyDeathDate(PropertyDeathDateData::default()),
            PropertyName::DEATHPLACE => Property::PropertyDeathPlace(PropertyDeathPlaceData::default()),
            PropertyName::EMAIL => Property::PropertyEmail(PropertyEmailData::default()),
            PropertyName::EXPERTISE => Property::PropertyExpertise(PropertyExpertiseData::default()),
            PropertyName::FBURL => Property::PropertyFbUrl(PropertyFbUrlData::default()),
            PropertyName::FN => Property::PropertyFn(PropertyFnData::default()),
            PropertyName::GENDER => Property::PropertyGender(PropertyGenderData::default()),
            PropertyName::GEO => Property::PropertyGeo(PropertyGeoData::default()),
            PropertyName::HOBBY => Property::PropertyHobby(PropertyHobbyData::default()),
            PropertyName::IMPP => Property::PropertyImpp(PropertyImppData::default()),
            PropertyName::INTEREST => Property::PropertyInterest(PropertyInterestData::default()),
            PropertyName::KEY => Property::PropertyKey(PropertyKeyData::default()),
            PropertyName::KIND => Property::PropertyKind(PropertyKindData::default()),
            PropertyName::LANG => Property::PropertyLang(PropertyLangData::default()),
            PropertyName::LOGO => Property::PropertyLogo(PropertyLogoData::default()),
            PropertyName::MEMBER => Property::PropertyMember(PropertyMemberData::default()),
            PropertyName::NICKNAME => Property::PropertyNickName(PropertyNickNameData::default()),
            PropertyName::NOTE => Property::PropertyNote(PropertyNoteData::default()),
            PropertyName::N => Property::PropertyN(PropertyNData::default()),
            PropertyName::ORGDIRECTORY => Property::PropertyOrgDirectory(PropertyOrgDirectoryData::default()),
            PropertyName::ORG => Property::PropertyOrg(PropertyOrgData::default()),
            PropertyName::PHOTO => Property::PropertyPhoto(PropertyPhotoData::default()),
            PropertyName::PRODID => Property::PropertyProdId(PropertyProdIdData::default()),
            PropertyName::RELATED => Property::PropertyRelated(PropertyRelatedData::default()),
            PropertyName::REV => Property::PropertyRev(PropertyRevData::default()),
            PropertyName::ROLE => Property::PropertyRole(PropertyRoleData::default()),
            PropertyName::SOUND => Property::PropertySound(PropertySoundData::default()),
            PropertyName::SOURCE => Property::PropertySource(PropertySourceData::default()),
            PropertyName::TEL => Property::PropertyTel(PropertyTelData::default()),
            PropertyName::TITLE => Property::PropertyTitle(PropertyTitleData::default()),
            PropertyName::TZ => Property::PropertyTz(PropertyTzData::default()),
            PropertyName::UID => Property::PropertyUid(PropertyUidData::default()),
            PropertyName::URL => Property::PropertyUrl(PropertyUrlData::default()),
            PropertyName::XML => Property::PropertyXml(PropertyXmlData::default()),
            _ => Property::PropertyXName(PropertyXNameData::default(name)),
        }
    }

    /// Export a property without any pid information.
    ///
    /// # Examples
    /// ```
    /// use vcard_parser::parse::vcard::vcard;
    /// use vcard_parser::vcard::property::Property;
    ///
    /// let mut property = Property::try_from("FN;PID=1:John Doe\n").expect("Unable to parse property.");
    /// assert_eq!(property.to_string(), "FN;PID=1:John Doe\n");
    /// assert_eq!(property.export(), "FN:John Doe\n");
    /// ```
    pub fn export(&self) -> String {
        let mut property = self.clone();

        // Remove all pids from property.
        property.set_parameters(property.get_parameters().into_iter().filter(|p| p.name() != ParameterName::PID).collect());

        property.to_string()
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(group) = self.group() {
            write!(f, "{}.", group)?;
        }

        write!(f, "{}", self.name())?;

        for parameter in self.get_parameters() {
            write!(f, "{}", parameter)?;
        }

        write!(f, ":{}", self.get_value())?;
        write!(f, "\n")?;

        Ok(())
    }
}

/// Matches properties based on [RFC 6350 7.1.2](https://datatracker.ietf.org/doc/html/rfc6350#section-7.1.2) and [RFC 6350 7.1.3](https://datatracker.ietf.org/doc/html/rfc6350#section-7.1.3).
impl PartialEq<Property> for Property {
    fn eq(&self, other: &Property) -> bool {
        // Property instances whose name is CLIENTPIDMAP are handled separately
        // and MUST NOT be matched.  The synchronization MUST ensure that there
        // is consistency of CLIENTPIDMAPs among matched vCard instances.
        if self.name() == PropertyName::CLIENTPIDMAP {
            return false;
        }

        // Property instances whose name (e.g., EMAIL, TEL, etc.) is not the
        // same MUST NOT be matched.
        if self.name() != other.name() {
            return false;
        }

        // Property instances belonging to matched vCards, whose name is the
        // same, and whose maximum cardinality is 1, MUST be matched.
        if self.is_single() && self.name() == other.name() {
            return true;
        }

        // Property instances belonging to matched vCards, whose name is the
        // same, and whose PID parameters match, MUST be matched. See
        // Section 7.1.3 for details on PID matching.
        if self.is_multiple() && self.name() == other.name() {
            fn _pids_get(property: &Property) -> Option<Vec<(i32, Option<i32>)>> {
                for parameter in property.get_parameters() {
                    if parameter.name() == ParameterName::PID {
                        if let ValuePid(data) = parameter.get_value() {
                            return Some(data.value.clone());
                        }
                    }
                }
                None
            }

            if let (Some(a), Some(b)) = (_pids_get(self), _pids_get(other)) {
                for (a1, a2) in &a {
                    for (b1, b2) in &b {
                        if a1 == b1 && a2 == b2 {
                            return true;
                        }
                    }
                }
            }
        }

        // In all other cases, property instances MAY be matched at the
        // discretion of the synchronization engine.
        false
    }
}

impl TryFrom<&str> for Property {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        Self::create_from_str(str)
    }
}

impl<'a> TryFrom<PropertyData<'a>> for Property {
    type Error = VcardError;
    fn try_from(data: PropertyData) -> Result<Self, Self::Error> {
        Self::create_from_data(data)
    }
}

impl TryFrom<(Option<String>, &str, Vec<Parameter>, &str)> for Property {
    type Error = VcardError;
    fn try_from(data: (Option<String>, &str, Vec<Parameter>, &str)) -> Result<Self, Self::Error> {
        Self::create(data)
    }
}

impl HasGroup for Property {
    fn group(&self) -> &Option<String> {
        match self {
            Property::PropertyAdr(property) => property.group(),
            Property::PropertyAnniversary(property) => property.group(),
            Property::PropertyBDay(property) => property.group(),
            Property::PropertyBirthPlace(property) => property.group(),
            Property::PropertyCalAdrUri(property) => property.group(),
            Property::PropertyCalUri(property) => property.group(),
            Property::PropertyCategories(property) => property.group(),
            Property::PropertyClientPidMap(property) => property.group(),
            Property::PropertyContactUri(property) => property.group(),
            Property::PropertyDeathDate(property) => property.group(),
            Property::PropertyDeathPlace(property) => property.group(),
            Property::PropertyEmail(property) => property.group(),
            Property::PropertyExpertise(property) => property.group(),
            Property::PropertyFbUrl(property) => property.group(),
            Property::PropertyFn(property) => property.group(),
            Property::PropertyGender(property) => property.group(),
            Property::PropertyGeo(property) => property.group(),
            Property::PropertyHobby(property) => property.group(),
            Property::PropertyImpp(property) => property.group(),
            Property::PropertyInterest(property) => property.group(),
            Property::PropertyKey(property) => property.group(),
            Property::PropertyKind(property) => property.group(),
            Property::PropertyLang(property) => property.group(),
            Property::PropertyLogo(property) => property.group(),
            Property::PropertyMember(property) => property.group(),
            Property::PropertyNickName(property) => property.group(),
            Property::PropertyNote(property) => property.group(),
            Property::PropertyN(property) => property.group(),
            Property::PropertyOrgDirectory(property) => property.group(),
            Property::PropertyOrg(property) => property.group(),
            Property::PropertyPhoto(property) => property.group(),
            Property::PropertyProdId(property) => property.group(),
            Property::PropertyRelated(property) => property.group(),
            Property::PropertyRev(property) => property.group(),
            Property::PropertyRole(property) => property.group(),
            Property::PropertySound(property) => property.group(),
            Property::PropertySource(property) => property.group(),
            Property::PropertyTel(property) => property.group(),
            Property::PropertyTitle(property) => property.group(),
            Property::PropertyTz(property) => property.group(),
            Property::PropertyUid(property) => property.group(),
            Property::PropertyUrl(property) => property.group(),
            Property::PropertyXml(property) => property.group(),
            Property::PropertyXName(property) => property.group(),
        }
    }
}

impl HasName for Property {
    fn name(&self) -> &str {
        match self {
            Property::PropertyAdr(property) => property.name(),
            Property::PropertyAnniversary(property) => property.name(),
            Property::PropertyBDay(property) => property.name(),
            Property::PropertyBirthPlace(property) => property.name(),
            Property::PropertyCalAdrUri(property) => property.name(),
            Property::PropertyCalUri(property) => property.name(),
            Property::PropertyCategories(property) => property.name(),
            Property::PropertyClientPidMap(property) => property.name(),
            Property::PropertyContactUri(property) => property.name(),
            Property::PropertyDeathDate(property) => property.name(),
            Property::PropertyDeathPlace(property) => property.name(),
            Property::PropertyEmail(property) => property.name(),
            Property::PropertyExpertise(property) => property.name(),
            Property::PropertyFbUrl(property) => property.name(),
            Property::PropertyFn(property) => property.name(),
            Property::PropertyGender(property) => property.name(),
            Property::PropertyGeo(property) => property.name(),
            Property::PropertyHobby(property) => property.name(),
            Property::PropertyImpp(property) => property.name(),
            Property::PropertyInterest(property) => property.name(),
            Property::PropertyKey(property) => property.name(),
            Property::PropertyKind(property) => property.name(),
            Property::PropertyLang(property) => property.name(),
            Property::PropertyLogo(property) => property.name(),
            Property::PropertyMember(property) => property.name(),
            Property::PropertyNickName(property) => property.name(),
            Property::PropertyNote(property) => property.name(),
            Property::PropertyN(property) => property.name(),
            Property::PropertyOrgDirectory(property) => property.name(),
            Property::PropertyOrg(property) => property.name(),
            Property::PropertyPhoto(property) => property.name(),
            Property::PropertyProdId(property) => property.name(),
            Property::PropertyRelated(property) => property.name(),
            Property::PropertyRev(property) => property.name(),
            Property::PropertyRole(property) => property.name(),
            Property::PropertySound(property) => property.name(),
            Property::PropertySource(property) => property.name(),
            Property::PropertyTel(property) => property.name(),
            Property::PropertyTitle(property) => property.name(),
            Property::PropertyTz(property) => property.name(),
            Property::PropertyUid(property) => property.name(),
            Property::PropertyUrl(property) => property.name(),
            Property::PropertyXml(property) => property.name(),
            Property::PropertyXName(property) => property.name(),
        }
    }
}

impl HasCardinality for Property {
    fn cardinality(&self) -> &str {
        match self {
            Property::PropertyAdr(property) => property.cardinality(),
            Property::PropertyAnniversary(property) => property.cardinality(),
            Property::PropertyBDay(property) => property.cardinality(),
            Property::PropertyBirthPlace(property) => property.cardinality(),
            Property::PropertyCalAdrUri(property) => property.cardinality(),
            Property::PropertyCalUri(property) => property.cardinality(),
            Property::PropertyCategories(property) => property.cardinality(),
            Property::PropertyClientPidMap(property) => property.cardinality(),
            Property::PropertyContactUri(property) => property.cardinality(),
            Property::PropertyDeathDate(property) => property.cardinality(),
            Property::PropertyDeathPlace(property) => property.cardinality(),
            Property::PropertyEmail(property) => property.cardinality(),
            Property::PropertyExpertise(property) => property.cardinality(),
            Property::PropertyFbUrl(property) => property.cardinality(),
            Property::PropertyFn(property) => property.cardinality(),
            Property::PropertyGender(property) => property.cardinality(),
            Property::PropertyGeo(property) => property.cardinality(),
            Property::PropertyHobby(property) => property.cardinality(),
            Property::PropertyImpp(property) => property.cardinality(),
            Property::PropertyInterest(property) => property.cardinality(),
            Property::PropertyKey(property) => property.cardinality(),
            Property::PropertyKind(property) => property.cardinality(),
            Property::PropertyLang(property) => property.cardinality(),
            Property::PropertyLogo(property) => property.cardinality(),
            Property::PropertyMember(property) => property.cardinality(),
            Property::PropertyNickName(property) => property.cardinality(),
            Property::PropertyNote(property) => property.cardinality(),
            Property::PropertyN(property) => property.cardinality(),
            Property::PropertyOrgDirectory(property) => property.cardinality(),
            Property::PropertyOrg(property) => property.cardinality(),
            Property::PropertyPhoto(property) => property.cardinality(),
            Property::PropertyProdId(property) => property.cardinality(),
            Property::PropertyRelated(property) => property.cardinality(),
            Property::PropertyRev(property) => property.cardinality(),
            Property::PropertyRole(property) => property.cardinality(),
            Property::PropertySound(property) => property.cardinality(),
            Property::PropertySource(property) => property.cardinality(),
            Property::PropertyTel(property) => property.cardinality(),
            Property::PropertyTitle(property) => property.cardinality(),
            Property::PropertyTz(property) => property.cardinality(),
            Property::PropertyUid(property) => property.cardinality(),
            Property::PropertyUrl(property) => property.cardinality(),
            Property::PropertyXml(property) => property.cardinality(),
            Property::PropertyXName(property) => property.cardinality(),
        }
    }
}

impl HasValue for Property {
    fn get_value(&self) -> &Value {
        match self {
            Property::PropertyAdr(property) => property.get_value(),
            Property::PropertyAnniversary(property) => property.get_value(),
            Property::PropertyBDay(property) => property.get_value(),
            Property::PropertyBirthPlace(property) => property.get_value(),
            Property::PropertyCalAdrUri(property) => property.get_value(),
            Property::PropertyCalUri(property) => property.get_value(),
            Property::PropertyCategories(property) => property.get_value(),
            Property::PropertyClientPidMap(property) => property.get_value(),
            Property::PropertyContactUri(property) => property.get_value(),
            Property::PropertyDeathDate(property) => property.get_value(),
            Property::PropertyDeathPlace(property) => property.get_value(),
            Property::PropertyEmail(property) => property.get_value(),
            Property::PropertyExpertise(property) => property.get_value(),
            Property::PropertyFbUrl(property) => property.get_value(),
            Property::PropertyFn(property) => property.get_value(),
            Property::PropertyGender(property) => property.get_value(),
            Property::PropertyGeo(property) => property.get_value(),
            Property::PropertyHobby(property) => property.get_value(),
            Property::PropertyImpp(property) => property.get_value(),
            Property::PropertyInterest(property) => property.get_value(),
            Property::PropertyKey(property) => property.get_value(),
            Property::PropertyKind(property) => property.get_value(),
            Property::PropertyLang(property) => property.get_value(),
            Property::PropertyLogo(property) => property.get_value(),
            Property::PropertyMember(property) => property.get_value(),
            Property::PropertyNickName(property) => property.get_value(),
            Property::PropertyNote(property) => property.get_value(),
            Property::PropertyN(property) => property.get_value(),
            Property::PropertyOrgDirectory(property) => property.get_value(),
            Property::PropertyOrg(property) => property.get_value(),
            Property::PropertyPhoto(property) => property.get_value(),
            Property::PropertyProdId(property) => property.get_value(),
            Property::PropertyRelated(property) => property.get_value(),
            Property::PropertyRev(property) => property.get_value(),
            Property::PropertyRole(property) => property.get_value(),
            Property::PropertySound(property) => property.get_value(),
            Property::PropertySource(property) => property.get_value(),
            Property::PropertyTel(property) => property.get_value(),
            Property::PropertyTitle(property) => property.get_value(),
            Property::PropertyTz(property) => property.get_value(),
            Property::PropertyUid(property) => property.get_value(),
            Property::PropertyUrl(property) => property.get_value(),
            Property::PropertyXml(property) => property.get_value(),
            Property::PropertyXName(property) => property.get_value(),
        }
    }

    fn set_value(&mut self, value: Value) -> Result<(), VcardError> {
        match self {
            Property::PropertyAdr(property) => property.set_value(value),
            Property::PropertyAnniversary(property) => property.set_value(value),
            Property::PropertyBDay(property) => property.set_value(value),
            Property::PropertyBirthPlace(property) => property.set_value(value),
            Property::PropertyCalAdrUri(property) => property.set_value(value),
            Property::PropertyCalUri(property) => property.set_value(value),
            Property::PropertyCategories(property) => property.set_value(value),
            Property::PropertyClientPidMap(property) => property.set_value(value),
            Property::PropertyContactUri(property) => property.set_value(value),
            Property::PropertyDeathDate(property) => property.set_value(value),
            Property::PropertyDeathPlace(property) => property.set_value(value),
            Property::PropertyEmail(property) => property.set_value(value),
            Property::PropertyExpertise(property) => property.set_value(value),
            Property::PropertyFbUrl(property) => property.set_value(value),
            Property::PropertyFn(property) => property.set_value(value),
            Property::PropertyGender(property) => property.set_value(value),
            Property::PropertyGeo(property) => property.set_value(value),
            Property::PropertyHobby(property) => property.set_value(value),
            Property::PropertyImpp(property) => property.set_value(value),
            Property::PropertyInterest(property) => property.set_value(value),
            Property::PropertyKey(property) => property.set_value(value),
            Property::PropertyKind(property) => property.set_value(value),
            Property::PropertyLang(property) => property.set_value(value),
            Property::PropertyLogo(property) => property.set_value(value),
            Property::PropertyMember(property) => property.set_value(value),
            Property::PropertyNickName(property) => property.set_value(value),
            Property::PropertyNote(property) => property.set_value(value),
            Property::PropertyN(property) => property.set_value(value),
            Property::PropertyOrgDirectory(property) => property.set_value(value),
            Property::PropertyOrg(property) => property.set_value(value),
            Property::PropertyPhoto(property) => property.set_value(value),
            Property::PropertyProdId(property) => property.set_value(value),
            Property::PropertyRelated(property) => property.set_value(value),
            Property::PropertyRev(property) => property.set_value(value),
            Property::PropertyRole(property) => property.set_value(value),
            Property::PropertySound(property) => property.set_value(value),
            Property::PropertySource(property) => property.set_value(value),
            Property::PropertyTel(property) => property.set_value(value),
            Property::PropertyTitle(property) => property.set_value(value),
            Property::PropertyTz(property) => property.set_value(value),
            Property::PropertyUid(property) => property.set_value(value),
            Property::PropertyUrl(property) => property.set_value(value),
            Property::PropertyXml(property) => property.set_value(value),
            Property::PropertyXName(property) => property.set_value(value),
        }
    }
}

impl HasParameters for Property {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str> {
        match self {
            Property::PropertyAdr(property) => property.allowed_parameters(),
            Property::PropertyAnniversary(property) => property.allowed_parameters(),
            Property::PropertyBDay(property) => property.allowed_parameters(),
            Property::PropertyBirthPlace(property) => property.allowed_parameters(),
            Property::PropertyCalAdrUri(property) => property.allowed_parameters(),
            Property::PropertyCalUri(property) => property.allowed_parameters(),
            Property::PropertyCategories(property) => property.allowed_parameters(),
            Property::PropertyClientPidMap(property) => property.allowed_parameters(),
            Property::PropertyContactUri(property) => property.allowed_parameters(),
            Property::PropertyDeathDate(property) => property.allowed_parameters(),
            Property::PropertyDeathPlace(property) => property.allowed_parameters(),
            Property::PropertyEmail(property) => property.allowed_parameters(),
            Property::PropertyExpertise(property) => property.allowed_parameters(),
            Property::PropertyFbUrl(property) => property.allowed_parameters(),
            Property::PropertyFn(property) => property.allowed_parameters(),
            Property::PropertyGender(property) => property.allowed_parameters(),
            Property::PropertyGeo(property) => property.allowed_parameters(),
            Property::PropertyHobby(property) => property.allowed_parameters(),
            Property::PropertyImpp(property) => property.allowed_parameters(),
            Property::PropertyInterest(property) => property.allowed_parameters(),
            Property::PropertyKey(property) => property.allowed_parameters(),
            Property::PropertyKind(property) => property.allowed_parameters(),
            Property::PropertyLang(property) => property.allowed_parameters(),
            Property::PropertyLogo(property) => property.allowed_parameters(),
            Property::PropertyMember(property) => property.allowed_parameters(),
            Property::PropertyNickName(property) => property.allowed_parameters(),
            Property::PropertyNote(property) => property.allowed_parameters(),
            Property::PropertyN(property) => property.allowed_parameters(),
            Property::PropertyOrgDirectory(property) => property.allowed_parameters(),
            Property::PropertyOrg(property) => property.allowed_parameters(),
            Property::PropertyPhoto(property) => property.allowed_parameters(),
            Property::PropertyProdId(property) => property.allowed_parameters(),
            Property::PropertyRelated(property) => property.allowed_parameters(),
            Property::PropertyRev(property) => property.allowed_parameters(),
            Property::PropertyRole(property) => property.allowed_parameters(),
            Property::PropertySound(property) => property.allowed_parameters(),
            Property::PropertySource(property) => property.allowed_parameters(),
            Property::PropertyTel(property) => property.allowed_parameters(),
            Property::PropertyTitle(property) => property.allowed_parameters(),
            Property::PropertyTz(property) => property.allowed_parameters(),
            Property::PropertyUid(property) => property.allowed_parameters(),
            Property::PropertyUrl(property) => property.allowed_parameters(),
            Property::PropertyXml(property) => property.allowed_parameters(),
            Property::PropertyXName(property) => property.allowed_parameters(),
        }
    }

    fn get_parameters(&self) -> Vec<Parameter> {
        match self {
            Property::PropertyAdr(property) => property.get_parameters(),
            Property::PropertyAnniversary(property) => property.get_parameters(),
            Property::PropertyBDay(property) => property.get_parameters(),
            Property::PropertyBirthPlace(property) => property.get_parameters(),
            Property::PropertyCalAdrUri(property) => property.get_parameters(),
            Property::PropertyCalUri(property) => property.get_parameters(),
            Property::PropertyCategories(property) => property.get_parameters(),
            Property::PropertyClientPidMap(property) => property.get_parameters(),
            Property::PropertyContactUri(property) => property.get_parameters(),
            Property::PropertyDeathDate(property) => property.get_parameters(),
            Property::PropertyDeathPlace(property) => property.get_parameters(),
            Property::PropertyEmail(property) => property.get_parameters(),
            Property::PropertyExpertise(property) => property.get_parameters(),
            Property::PropertyFbUrl(property) => property.get_parameters(),
            Property::PropertyFn(property) => property.get_parameters(),
            Property::PropertyGender(property) => property.get_parameters(),
            Property::PropertyGeo(property) => property.get_parameters(),
            Property::PropertyHobby(property) => property.get_parameters(),
            Property::PropertyImpp(property) => property.get_parameters(),
            Property::PropertyInterest(property) => property.get_parameters(),
            Property::PropertyKey(property) => property.get_parameters(),
            Property::PropertyKind(property) => property.get_parameters(),
            Property::PropertyLang(property) => property.get_parameters(),
            Property::PropertyLogo(property) => property.get_parameters(),
            Property::PropertyMember(property) => property.get_parameters(),
            Property::PropertyNickName(property) => property.get_parameters(),
            Property::PropertyNote(property) => property.get_parameters(),
            Property::PropertyN(property) => property.get_parameters(),
            Property::PropertyOrgDirectory(property) => property.get_parameters(),
            Property::PropertyOrg(property) => property.get_parameters(),
            Property::PropertyPhoto(property) => property.get_parameters(),
            Property::PropertyProdId(property) => property.get_parameters(),
            Property::PropertyRelated(property) => property.get_parameters(),
            Property::PropertyRev(property) => property.get_parameters(),
            Property::PropertyRole(property) => property.get_parameters(),
            Property::PropertySound(property) => property.get_parameters(),
            Property::PropertySource(property) => property.get_parameters(),
            Property::PropertyTel(property) => property.get_parameters(),
            Property::PropertyTitle(property) => property.get_parameters(),
            Property::PropertyTz(property) => property.get_parameters(),
            Property::PropertyUid(property) => property.get_parameters(),
            Property::PropertyUrl(property) => property.get_parameters(),
            Property::PropertyXml(property) => property.get_parameters(),
            Property::PropertyXName(property) => property.get_parameters(),
        }
    }

    fn set_parameters(&mut self, parameters: Vec<Parameter>) {
        match self {
            Property::PropertyAdr(property) => property.set_parameters(parameters),
            Property::PropertyAnniversary(property) => property.set_parameters(parameters),
            Property::PropertyBDay(property) => property.set_parameters(parameters),
            Property::PropertyBirthPlace(property) => property.set_parameters(parameters),
            Property::PropertyCalAdrUri(property) => property.set_parameters(parameters),
            Property::PropertyCalUri(property) => property.set_parameters(parameters),
            Property::PropertyCategories(property) => property.set_parameters(parameters),
            Property::PropertyClientPidMap(property) => property.set_parameters(parameters),
            Property::PropertyContactUri(property) => property.set_parameters(parameters),
            Property::PropertyDeathDate(property) => property.set_parameters(parameters),
            Property::PropertyDeathPlace(property) => property.set_parameters(parameters),
            Property::PropertyEmail(property) => property.set_parameters(parameters),
            Property::PropertyExpertise(property) => property.set_parameters(parameters),
            Property::PropertyFbUrl(property) => property.set_parameters(parameters),
            Property::PropertyFn(property) => property.set_parameters(parameters),
            Property::PropertyGender(property) => property.set_parameters(parameters),
            Property::PropertyGeo(property) => property.set_parameters(parameters),
            Property::PropertyHobby(property) => property.set_parameters(parameters),
            Property::PropertyImpp(property) => property.set_parameters(parameters),
            Property::PropertyInterest(property) => property.set_parameters(parameters),
            Property::PropertyKey(property) => property.set_parameters(parameters),
            Property::PropertyKind(property) => property.set_parameters(parameters),
            Property::PropertyLang(property) => property.set_parameters(parameters),
            Property::PropertyLogo(property) => property.set_parameters(parameters),
            Property::PropertyMember(property) => property.set_parameters(parameters),
            Property::PropertyNickName(property) => property.set_parameters(parameters),
            Property::PropertyNote(property) => property.set_parameters(parameters),
            Property::PropertyN(property) => property.set_parameters(parameters),
            Property::PropertyOrgDirectory(property) => property.set_parameters(parameters),
            Property::PropertyOrg(property) => property.set_parameters(parameters),
            Property::PropertyPhoto(property) => property.set_parameters(parameters),
            Property::PropertyProdId(property) => property.set_parameters(parameters),
            Property::PropertyRelated(property) => property.set_parameters(parameters),
            Property::PropertyRev(property) => property.set_parameters(parameters),
            Property::PropertyRole(property) => property.set_parameters(parameters),
            Property::PropertySound(property) => property.set_parameters(parameters),
            Property::PropertySource(property) => property.set_parameters(parameters),
            Property::PropertyTel(property) => property.set_parameters(parameters),
            Property::PropertyTitle(property) => property.set_parameters(parameters),
            Property::PropertyTz(property) => property.set_parameters(parameters),
            Property::PropertyUid(property) => property.set_parameters(parameters),
            Property::PropertyUrl(property) => property.set_parameters(parameters),
            Property::PropertyXml(property) => property.set_parameters(parameters),
            Property::PropertyXName(property) => property.set_parameters(parameters),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::constants::{PropertyName, TestDataPropertyValues};
    use crate::vcard::property::property_adr::PropertyAdrData;
    use crate::vcard::property::property_anniversary::PropertyAnniversaryData;
    use crate::vcard::property::property_bday::PropertyBDayData;
    use crate::vcard::property::property_birthplace::PropertyBirthPlaceData;
    use crate::vcard::property::property_caladruri::PropertyCalAdrUriData;
    use crate::vcard::property::property_caluri::PropertyCalUriData;
    use crate::vcard::property::property_categories::PropertyCategoriesData;
    use crate::vcard::property::property_clientpidmap::PropertyClientPidMapData;
    use crate::vcard::property::property_contacturi::PropertyContactUriData;
    use crate::vcard::property::property_deathdate::PropertyDeathDateData;
    use crate::vcard::property::property_deathplace::PropertyDeathPlaceData;
    use crate::vcard::property::property_email::PropertyEmailData;
    use crate::vcard::property::property_expertise::PropertyExpertiseData;
    use crate::vcard::property::property_fburl::PropertyFbUrlData;
    use crate::vcard::property::property_fn::PropertyFnData;
    use crate::vcard::property::property_gender::PropertyGenderData;
    use crate::vcard::property::property_geo::PropertyGeoData;
    use crate::vcard::property::property_hobby::PropertyHobbyData;
    use crate::vcard::property::property_impp::PropertyImppData;
    use crate::vcard::property::property_interest::PropertyInterestData;
    use crate::vcard::property::property_key::PropertyKeyData;
    use crate::vcard::property::property_kind::PropertyKindData;
    use crate::vcard::property::property_lang::PropertyLangData;
    use crate::vcard::property::property_logo::PropertyLogoData;
    use crate::vcard::property::property_member::PropertyMemberData;
    use crate::vcard::property::property_n::PropertyNData;
    use crate::vcard::property::property_nickname::PropertyNickNameData;
    use crate::vcard::property::property_note::PropertyNoteData;
    use crate::vcard::property::property_org::PropertyOrgData;
    use crate::vcard::property::property_orgdirectory::PropertyOrgDirectoryData;
    use crate::vcard::property::property_photo::PropertyPhotoData;
    use crate::vcard::property::property_prodid::PropertyProdIdData;
    use crate::vcard::property::property_related::PropertyRelatedData;
    use crate::vcard::property::property_rev::PropertyRevData;
    use crate::vcard::property::property_role::PropertyRoleData;
    use crate::vcard::property::property_sound::PropertySoundData;
    use crate::vcard::property::property_source::PropertySourceData;
    use crate::vcard::property::property_tel::PropertyTelData;
    use crate::vcard::property::property_title::PropertyTitleData;
    use crate::vcard::property::property_tz::PropertyTzData;
    use crate::vcard::property::property_uid::PropertyUidData;
    use crate::vcard::property::property_url::PropertyUrlData;
    use crate::vcard::property::property_xml::PropertyXmlData;
    use crate::{HasCardinality, HasName, HasValue, Property, Vcard};

    #[test]
    pub fn property_cardinality() {
        assert!(PropertyAnniversaryData::default().is_single());
        assert!(PropertyBDayData::default().is_single());
        assert!(PropertyBirthPlaceData::default().is_single());
        assert!(PropertyDeathDateData::default().is_single());
        assert!(PropertyDeathPlaceData::default().is_single());
        assert!(PropertyFnData::default().is_single());
        assert!(PropertyGenderData::default().is_single());
        assert!(PropertyKindData::default().is_single());
        assert!(PropertyNData::default().is_single());
        assert!(PropertyProdIdData::default().is_single());
        assert!(PropertyRevData::default().is_single());
        assert!(PropertyUidData::default().is_single());

        assert!(PropertyAdrData::default().is_multiple());
        assert!(PropertyCalAdrUriData::default().is_multiple());
        assert!(PropertyCalUriData::default().is_multiple());
        assert!(PropertyCategoriesData::default().is_multiple());
        assert!(PropertyClientPidMapData::default().is_multiple());
        assert!(PropertyContactUriData::default().is_multiple());
        assert!(PropertyEmailData::default().is_multiple());
        assert!(PropertyExpertiseData::default().is_multiple());
        assert!(PropertyFbUrlData::default().is_multiple());
        assert!(PropertyGeoData::default().is_multiple());
        assert!(PropertyHobbyData::default().is_multiple());
        assert!(PropertyImppData::default().is_multiple());
        assert!(PropertyInterestData::default().is_multiple());
        assert!(PropertyKeyData::default().is_multiple());
        assert!(PropertyLangData::default().is_multiple());
        assert!(PropertyLogoData::default().is_multiple());
        assert!(PropertyMemberData::default().is_multiple());
        assert!(PropertyNickNameData::default().is_multiple());
        assert!(PropertyNoteData::default().is_multiple());
        assert!(PropertyOrgDirectoryData::default().is_multiple());
        assert!(PropertyOrgData::default().is_multiple());
        assert!(PropertyPhotoData::default().is_multiple());
        assert!(PropertyRelatedData::default().is_multiple());
        assert!(PropertyRoleData::default().is_multiple());
        assert!(PropertySoundData::default().is_multiple());
        assert!(PropertySourceData::default().is_multiple());
        assert!(PropertyTelData::default().is_multiple());
        assert!(PropertyTitleData::default().is_multiple());
        assert!(PropertyTzData::default().is_multiple());
        assert!(PropertyUrlData::default().is_multiple());
        assert!(PropertyXmlData::default().is_multiple());
    }

    #[test]
    pub fn property_names() {
        assert_eq!(PropertyAdrData::default().name(), PropertyName::ADR);
        assert_eq!(PropertyAnniversaryData::default().name(), PropertyName::ANNIVERSARY);
        assert_eq!(PropertyBDayData::default().name(), PropertyName::BDAY);
        assert_eq!(PropertyBirthPlaceData::default().name(), PropertyName::BIRTHPLACE);
        assert_eq!(PropertyCalAdrUriData::default().name(), PropertyName::CALADRURI);
        assert_eq!(PropertyCalUriData::default().name(), PropertyName::CALURI);
        assert_eq!(PropertyCategoriesData::default().name(), PropertyName::CATEGORIES);
        assert_eq!(PropertyClientPidMapData::default().name(), PropertyName::CLIENTPIDMAP);
        assert_eq!(PropertyContactUriData::default().name(), PropertyName::CONTACTURI);
        assert_eq!(PropertyDeathDateData::default().name(), PropertyName::DEATHDATE);
        assert_eq!(PropertyDeathPlaceData::default().name(), PropertyName::DEATHPLACE);
        assert_eq!(PropertyEmailData::default().name(), PropertyName::EMAIL);
        assert_eq!(PropertyExpertiseData::default().name(), PropertyName::EXPERTISE);
        assert_eq!(PropertyFbUrlData::default().name(), PropertyName::FBURL);
        assert_eq!(PropertyFnData::default().name(), PropertyName::FN);
        assert_eq!(PropertyGenderData::default().name(), PropertyName::GENDER);
        assert_eq!(PropertyGeoData::default().name(), PropertyName::GEO);
        assert_eq!(PropertyHobbyData::default().name(), PropertyName::HOBBY);
        assert_eq!(PropertyImppData::default().name(), PropertyName::IMPP);
        assert_eq!(PropertyInterestData::default().name(), PropertyName::INTEREST);
        assert_eq!(PropertyKeyData::default().name(), PropertyName::KEY);
        assert_eq!(PropertyKindData::default().name(), PropertyName::KIND);
        assert_eq!(PropertyLangData::default().name(), PropertyName::LANG);
        assert_eq!(PropertyLogoData::default().name(), PropertyName::LOGO);
        assert_eq!(PropertyMemberData::default().name(), PropertyName::MEMBER);
        assert_eq!(PropertyNickNameData::default().name(), PropertyName::NICKNAME);
        assert_eq!(PropertyNoteData::default().name(), PropertyName::NOTE);
        assert_eq!(PropertyNData::default().name(), PropertyName::N);
        assert_eq!(PropertyOrgDirectoryData::default().name(), PropertyName::ORGDIRECTORY);
        assert_eq!(PropertyOrgData::default().name(), PropertyName::ORG);
        assert_eq!(PropertyPhotoData::default().name(), PropertyName::PHOTO);
        assert_eq!(PropertyProdIdData::default().name(), PropertyName::PRODID);
        assert_eq!(PropertyRelatedData::default().name(), PropertyName::RELATED);
        assert_eq!(PropertyRevData::default().name(), PropertyName::REV);
        assert_eq!(PropertyRoleData::default().name(), PropertyName::ROLE);
        assert_eq!(PropertySoundData::default().name(), PropertyName::SOUND);
        assert_eq!(PropertySourceData::default().name(), PropertyName::SOURCE);
        assert_eq!(PropertyTelData::default().name(), PropertyName::TEL);
        assert_eq!(PropertyTitleData::default().name(), PropertyName::TITLE);
        assert_eq!(PropertyTzData::default().name(), PropertyName::TZ);
        assert_eq!(PropertyUidData::default().name(), PropertyName::UID);
        assert_eq!(PropertyUrlData::default().name(), PropertyName::URL);
        assert_eq!(PropertyXmlData::default().name(), PropertyName::XML);
    }

    #[test]
    pub fn property_equality() {
        let a = Property::try_from("TEL;PID=1.1:555-5555\n").expect("Unable to parse property string.");
        let b = Property::try_from("TEL;PID=1.1:555-5556\n").expect("Unable to parse property string.");
        let c = Property::try_from("TEL;PID=1.2:555-5555\n").expect("Unable to parse property string.");
        let d = Property::try_from("TEL;PID=2.1:555-5557\n").expect("Unable to parse property string.");

        assert_eq!(a, b);
        assert_eq!(b, a);
        assert_ne!(a, c);
        assert_ne!(a, d);
        assert_ne!(b, c);
        assert_ne!(b, d);
        assert_ne!(c, d);

        assert_ne!(a.get_value(), b.get_value());
        assert_eq!(a.get_value(), c.get_value());
    }

    #[test]
    pub fn property_matching() {
        pub fn _property_matching(name: &str, value: &str) {
            let mut vcard = Vcard::new("John Doe");

            let str = format!("{}:{}\n", name, value);

            let a = Property::try_from(str.as_str()).unwrap();
            let b = Property::try_from(str.as_str()).unwrap();

            if a.name() == PropertyName::CLIENTPIDMAP {
                // TODO: Figure out CLIENTPIDMAP matching and adding.
            } else if a.name() == PropertyName::FN {
                vcard.set_property(&a).unwrap();
                assert_eq!(vcard.properties.len(), 1);
                vcard.set_property(&b).unwrap();
                assert_eq!(vcard.properties.len(), 1);
            } else if a.is_multiple() && b.is_multiple() {
                vcard.set_property(&a).unwrap();
                assert_eq!(vcard.get_properties_by_name(name).len(), 1);
                vcard.set_property(&b).unwrap();
                assert_eq!(vcard.get_properties_by_name(name).len(), 2);
            } else if a.is_single() && b.is_single() {
                vcard.set_property(&a).unwrap();
                assert_eq!(vcard.properties.len(), 2);
                vcard.set_property(&b).unwrap();
                assert_eq!(vcard.properties.len(), 2);
            }
        }

        _property_matching(PropertyName::ADR, TestDataPropertyValues::ADR);
        _property_matching(PropertyName::ANNIVERSARY, TestDataPropertyValues::ANNIVERSARY);
        _property_matching(PropertyName::BDAY, TestDataPropertyValues::BDAY);
        _property_matching(PropertyName::BIRTHPLACE, TestDataPropertyValues::BIRTHPLACE);
        _property_matching(PropertyName::CALADRURI, TestDataPropertyValues::CALADRURI);
        _property_matching(PropertyName::CALURI, TestDataPropertyValues::CALURI);
        _property_matching(PropertyName::CATEGORIES, TestDataPropertyValues::CATEGORIES);
        _property_matching(PropertyName::CLIENTPIDMAP, TestDataPropertyValues::CLIENTPIDMAP);
        _property_matching(PropertyName::CONTACTURI, TestDataPropertyValues::CONTACTURI);
        _property_matching(PropertyName::DEATHDATE, TestDataPropertyValues::DEATHDATE);
        _property_matching(PropertyName::DEATHPLACE, TestDataPropertyValues::DEATHPLACE);
        _property_matching(PropertyName::EMAIL, TestDataPropertyValues::EMAIL);
        _property_matching(PropertyName::EXPERTISE, TestDataPropertyValues::EXPERTISE);
        _property_matching(PropertyName::FBURL, TestDataPropertyValues::FBURL);
        _property_matching(PropertyName::FN, TestDataPropertyValues::FN);
        _property_matching(PropertyName::GENDER, TestDataPropertyValues::GENDER);
        _property_matching(PropertyName::GEO, TestDataPropertyValues::GEO);
        _property_matching(PropertyName::HOBBY, TestDataPropertyValues::HOBBY);
        _property_matching(PropertyName::IMPP, TestDataPropertyValues::IMPP);
        _property_matching(PropertyName::INTEREST, TestDataPropertyValues::INTEREST);
        _property_matching(PropertyName::KEY, TestDataPropertyValues::KEY);
        _property_matching(PropertyName::KIND, TestDataPropertyValues::KIND);
        _property_matching(PropertyName::LANG, TestDataPropertyValues::LANG);
        _property_matching(PropertyName::LOGO, TestDataPropertyValues::LOGO);
        _property_matching(PropertyName::MEMBER, TestDataPropertyValues::MEMBER);
        _property_matching(PropertyName::NICKNAME, TestDataPropertyValues::NICKNAME);
        _property_matching(PropertyName::NOTE, TestDataPropertyValues::NOTE);
        _property_matching(PropertyName::N, TestDataPropertyValues::N);
        _property_matching(PropertyName::ORGDIRECTORY, TestDataPropertyValues::ORGDIRECTORY);
        _property_matching(PropertyName::ORG, TestDataPropertyValues::ORG);
        _property_matching(PropertyName::PHOTO, TestDataPropertyValues::PHOTO);
        _property_matching(PropertyName::PRODID, TestDataPropertyValues::PRODID);
        _property_matching(PropertyName::RELATED, TestDataPropertyValues::RELATED);
        _property_matching(PropertyName::REV, TestDataPropertyValues::REV);
        _property_matching(PropertyName::ROLE, TestDataPropertyValues::ROLE);
        _property_matching(PropertyName::SOUND, TestDataPropertyValues::SOUND);
        _property_matching(PropertyName::SOURCE, TestDataPropertyValues::SOURCE);
        _property_matching(PropertyName::TEL, TestDataPropertyValues::TEL);
        _property_matching(PropertyName::TITLE, TestDataPropertyValues::TITLE);
        _property_matching(PropertyName::TZ, TestDataPropertyValues::TZ);
        _property_matching(PropertyName::UID, TestDataPropertyValues::UID);
        _property_matching(PropertyName::URL, TestDataPropertyValues::URL);
        _property_matching(PropertyName::XML, TestDataPropertyValues::XML);
    }
}
