use std::fmt::{Display, Formatter};

use crate::VcardError;

pub const PROPERTY_TYPE_ADR: &str = "ADR";
pub const PROPERTY_TYPE_ANNIVERSARY: &str = "ANNIVERSARY";
pub const PROPERTY_TYPE_BDAY: &str = "BDAY";
pub const PROPERTY_TYPE_BIRTHPLACE: &str = "BIRTHPLACE";
pub const PROPERTY_TYPE_CALADRURI: &str = "CALADRURI";
pub const PROPERTY_TYPE_CALURI: &str = "CALURI";
pub const PROPERTY_TYPE_CATEGORIES: &str = "CATEGORIES";
pub const PROPERTY_TYPE_CLIENTPIDMAP: &str = "CLIENTPIDMAP";
pub const PROPERTY_TYPE_CONTACTURI: &str = "CONTACT-URI";
pub const PROPERTY_TYPE_DEATHDATE: &str = "DEATHDATE";
pub const PROPERTY_TYPE_DEATHPLACE: &str = "DEATHPLACE";
pub const PROPERTY_TYPE_EMAIL: &str = "EMAIL";
pub const PROPERTY_TYPE_EXPERTISE: &str = "EXPERTISE";
pub const PROPERTY_TYPE_FBURL: &str = "FBURL";
pub const PROPERTY_TYPE_FN: &str = "FN";
pub const PROPERTY_TYPE_GENDER: &str = "GENDER";
pub const PROPERTY_TYPE_GEO: &str = "GEO";
pub const PROPERTY_TYPE_HOBBY: &str = "HOBBY";
pub const PROPERTY_TYPE_IMPP: &str = "IMPP";
pub const PROPERTY_TYPE_INTEREST: &str = "INTEREST";
pub const PROPERTY_TYPE_KEY: &str = "KEY";
pub const PROPERTY_TYPE_KIND: &str = "KIND";
pub const PROPERTY_TYPE_LANG: &str = "LANG";
pub const PROPERTY_TYPE_LOGO: &str = "LOGO";
pub const PROPERTY_TYPE_MEMBER: &str = "MEMBER";
pub const PROPERTY_TYPE_NICKNAME: &str = "NICKNAME";
pub const PROPERTY_TYPE_NOTE: &str = "NOTE";
pub const PROPERTY_TYPE_N: &str = "N";
pub const PROPERTY_TYPE_ORGDIRECTORY: &str = "ORG-DIRECTORY";
pub const PROPERTY_TYPE_ORG: &str = "ORG";
pub const PROPERTY_TYPE_PHOTO: &str = "PHOTO";
pub const PROPERTY_TYPE_PRODID: &str = "PRODID";
pub const PROPERTY_TYPE_RELATED: &str = "RELATED";
pub const PROPERTY_TYPE_REV: &str = "REV";
pub const PROPERTY_TYPE_ROLE: &str = "ROLE";
pub const PROPERTY_TYPE_SOUND: &str = "SOUND";
pub const PROPERTY_TYPE_SOURCE: &str = "SOURCE";
pub const PROPERTY_TYPE_TEL: &str = "TEL";
pub const PROPERTY_TYPE_TITLE: &str = "TITLE";
pub const PROPERTY_TYPE_TZ: &str = "TZ";
pub const PROPERTY_TYPE_UID: &str = "UID";
pub const PROPERTY_TYPE_URL: &str = "URL";
pub const PROPERTY_TYPE_VERSION: &str = "VERSION";
pub const PROPERTY_TYPE_XML: &str = "XML";

/// A list of property types.
#[derive(Clone, Eq, PartialEq)]
pub enum PropertyType {
    Adr,
    Anniversary,
    BDay,
    BirthPlace,
    CalAdrUri,
    CalUri,
    Categories,
    ClientPidMap,
    ContactUri,
    DeathDate,
    DeathPlace,
    Email,
    Expertise,
    FbUrl,
    Fn,
    Gender,
    Geo,
    Hobby,
    Impp,
    Interest,
    Key,
    Kind,
    Lang,
    Logo,
    Member,
    NickName,
    Note,
    N,
    OrgDirectory,
    Org,
    Photo,
    ProdId,
    Related,
    Rev,
    Role,
    Sound,
    Source,
    Tel,
    Title,
    Tz,
    Uid,
    Url,
    Version,
    Xml,
}

impl TryFrom<&str> for PropertyType {
    type Error = VcardError;
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        let str = match str.split_once('.') {
            None => str,
            Some((_, str)) => str,
        };

        match str.to_uppercase().as_str() {
            PROPERTY_TYPE_ADR => Ok(PropertyType::Adr),
            PROPERTY_TYPE_ANNIVERSARY => Ok(PropertyType::Anniversary),
            PROPERTY_TYPE_BDAY => Ok(PropertyType::BDay),
            PROPERTY_TYPE_BIRTHPLACE => Ok(PropertyType::BirthPlace),
            PROPERTY_TYPE_CALADRURI => Ok(PropertyType::CalAdrUri),
            PROPERTY_TYPE_CALURI => Ok(PropertyType::CalUri),
            PROPERTY_TYPE_CATEGORIES => Ok(PropertyType::Categories),
            PROPERTY_TYPE_CLIENTPIDMAP => Ok(PropertyType::ClientPidMap),
            PROPERTY_TYPE_CONTACTURI => Ok(PropertyType::ContactUri),
            PROPERTY_TYPE_DEATHDATE => Ok(PropertyType::DeathDate),
            PROPERTY_TYPE_DEATHPLACE => Ok(PropertyType::DeathPlace),
            PROPERTY_TYPE_EMAIL => Ok(PropertyType::Email),
            PROPERTY_TYPE_EXPERTISE => Ok(PropertyType::Expertise),
            PROPERTY_TYPE_FBURL => Ok(PropertyType::FbUrl),
            PROPERTY_TYPE_FN => Ok(PropertyType::Fn),
            PROPERTY_TYPE_GENDER => Ok(PropertyType::Gender),
            PROPERTY_TYPE_GEO => Ok(PropertyType::Geo),
            PROPERTY_TYPE_HOBBY => Ok(PropertyType::Hobby),
            PROPERTY_TYPE_IMPP => Ok(PropertyType::Impp),
            PROPERTY_TYPE_INTEREST => Ok(PropertyType::Interest),
            PROPERTY_TYPE_KEY => Ok(PropertyType::Key),
            PROPERTY_TYPE_KIND => Ok(PropertyType::Kind),
            PROPERTY_TYPE_LANG => Ok(PropertyType::Lang),
            PROPERTY_TYPE_LOGO => Ok(PropertyType::Logo),
            PROPERTY_TYPE_MEMBER => Ok(PropertyType::Member),
            PROPERTY_TYPE_NICKNAME => Ok(PropertyType::NickName),
            PROPERTY_TYPE_NOTE => Ok(PropertyType::Note),
            PROPERTY_TYPE_N => Ok(PropertyType::N),
            PROPERTY_TYPE_ORGDIRECTORY => Ok(PropertyType::OrgDirectory),
            PROPERTY_TYPE_ORG => Ok(PropertyType::Org),
            PROPERTY_TYPE_PHOTO => Ok(PropertyType::Photo),
            PROPERTY_TYPE_PRODID => Ok(PropertyType::ProdId),
            PROPERTY_TYPE_RELATED => Ok(PropertyType::Related),
            PROPERTY_TYPE_REV => Ok(PropertyType::Rev),
            PROPERTY_TYPE_ROLE => Ok(PropertyType::Role),
            PROPERTY_TYPE_SOUND => Ok(PropertyType::Sound),
            PROPERTY_TYPE_SOURCE => Ok(PropertyType::Source),
            PROPERTY_TYPE_TEL => Ok(PropertyType::Tel),
            PROPERTY_TYPE_TITLE => Ok(PropertyType::Title),
            PROPERTY_TYPE_TZ => Ok(PropertyType::Tz),
            PROPERTY_TYPE_UID => Ok(PropertyType::Uid),
            PROPERTY_TYPE_URL => Ok(PropertyType::Url),
            PROPERTY_TYPE_VERSION => Ok(PropertyType::Version),
            PROPERTY_TYPE_XML => Ok(PropertyType::Xml),
            _ => Err(VcardError::PropertyTypeUnknown(str.to_string())),
        }
    }
}

impl From<&PropertyType> for String {
    fn from(property_type: &PropertyType) -> Self {
        format!("{}", property_type)
    }
}

impl Display for PropertyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            PropertyType::Adr => PROPERTY_TYPE_ADR,
            PropertyType::Anniversary => PROPERTY_TYPE_ANNIVERSARY,
            PropertyType::BDay => PROPERTY_TYPE_BDAY,
            PropertyType::BirthPlace => PROPERTY_TYPE_BIRTHPLACE,
            PropertyType::CalAdrUri => PROPERTY_TYPE_CALADRURI,
            PropertyType::CalUri => PROPERTY_TYPE_CALURI,
            PropertyType::Categories => PROPERTY_TYPE_CATEGORIES,
            PropertyType::ClientPidMap => PROPERTY_TYPE_CLIENTPIDMAP,
            PropertyType::ContactUri => PROPERTY_TYPE_CONTACTURI,
            PropertyType::DeathDate => PROPERTY_TYPE_DEATHDATE,
            PropertyType::DeathPlace => PROPERTY_TYPE_DEATHPLACE,
            PropertyType::Email => PROPERTY_TYPE_EMAIL,
            PropertyType::Expertise => PROPERTY_TYPE_EXPERTISE,
            PropertyType::FbUrl => PROPERTY_TYPE_FBURL,
            PropertyType::Fn => PROPERTY_TYPE_FN,
            PropertyType::Gender => PROPERTY_TYPE_GENDER,
            PropertyType::Geo => PROPERTY_TYPE_GEO,
            PropertyType::Hobby => PROPERTY_TYPE_HOBBY,
            PropertyType::Impp => PROPERTY_TYPE_IMPP,
            PropertyType::Interest => PROPERTY_TYPE_INTEREST,
            PropertyType::Key => PROPERTY_TYPE_KEY,
            PropertyType::Kind => PROPERTY_TYPE_KIND,
            PropertyType::Lang => PROPERTY_TYPE_LANG,
            PropertyType::Logo => PROPERTY_TYPE_LOGO,
            PropertyType::Member => PROPERTY_TYPE_MEMBER,
            PropertyType::NickName => PROPERTY_TYPE_NICKNAME,
            PropertyType::Note => PROPERTY_TYPE_NOTE,
            PropertyType::N => PROPERTY_TYPE_N,
            PropertyType::OrgDirectory => PROPERTY_TYPE_ORGDIRECTORY,
            PropertyType::Org => PROPERTY_TYPE_ORG,
            PropertyType::Photo => PROPERTY_TYPE_PHOTO,
            PropertyType::ProdId => PROPERTY_TYPE_PRODID,
            PropertyType::Related => PROPERTY_TYPE_RELATED,
            PropertyType::Rev => PROPERTY_TYPE_REV,
            PropertyType::Role => PROPERTY_TYPE_ROLE,
            PropertyType::Sound => PROPERTY_TYPE_SOUND,
            PropertyType::Source => PROPERTY_TYPE_SOURCE,
            PropertyType::Tel => PROPERTY_TYPE_TEL,
            PropertyType::Title => PROPERTY_TYPE_TITLE,
            PropertyType::Tz => PROPERTY_TYPE_TZ,
            PropertyType::Uid => PROPERTY_TYPE_UID,
            PropertyType::Url => PROPERTY_TYPE_URL,
            PropertyType::Version => PROPERTY_TYPE_VERSION,
            PropertyType::Xml => PROPERTY_TYPE_XML,
        };
        write!(f, "{}", text)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::property::types::PropertyType;

    #[test]
    pub fn property_types() {
        assert_eq!(PropertyType::try_from("ADR").unwrap().to_string().as_str(), "ADR");
        assert_eq!(PropertyType::try_from("ANNIVERSARY").unwrap().to_string().as_str(), "ANNIVERSARY");
        assert_eq!(PropertyType::try_from("BDAY").unwrap().to_string().as_str(), "BDAY");
        assert_eq!(PropertyType::try_from("BIRTHPLACE").unwrap().to_string().as_str(), "BIRTHPLACE");
        assert_eq!(PropertyType::try_from("CALADRURI").unwrap().to_string().as_str(), "CALADRURI");
        assert_eq!(PropertyType::try_from("CALURI").unwrap().to_string().as_str(), "CALURI");
        assert_eq!(PropertyType::try_from("CATEGORIES").unwrap().to_string().as_str(), "CATEGORIES");
        assert_eq!(PropertyType::try_from("CLIENTPIDMAP").unwrap().to_string().as_str(), "CLIENTPIDMAP");
        assert_eq!(PropertyType::try_from("CONTACT-URI").unwrap().to_string().as_str(), "CONTACT-URI");
        assert_eq!(PropertyType::try_from("DEATHDATE").unwrap().to_string().as_str(), "DEATHDATE");
        assert_eq!(PropertyType::try_from("DEATHPLACE").unwrap().to_string().as_str(), "DEATHPLACE");
        assert_eq!(PropertyType::try_from("EMAIL").unwrap().to_string().as_str(), "EMAIL");
        assert_eq!(PropertyType::try_from("EXPERTISE").unwrap().to_string().as_str(), "EXPERTISE");
        assert_eq!(PropertyType::try_from("FBURL").unwrap().to_string().as_str(), "FBURL");
        assert_eq!(PropertyType::try_from("FN").unwrap().to_string().as_str(), "FN");
        assert_eq!(PropertyType::try_from("GENDER").unwrap().to_string().as_str(), "GENDER");
        assert_eq!(PropertyType::try_from("GEO").unwrap().to_string().as_str(), "GEO");
        assert_eq!(PropertyType::try_from("HOBBY").unwrap().to_string().as_str(), "HOBBY");
        assert_eq!(PropertyType::try_from("IMPP").unwrap().to_string().as_str(), "IMPP");
        assert_eq!(PropertyType::try_from("INTEREST").unwrap().to_string().as_str(), "INTEREST");
        assert_eq!(PropertyType::try_from("KEY").unwrap().to_string().as_str(), "KEY");
        assert_eq!(PropertyType::try_from("KIND").unwrap().to_string().as_str(), "KIND");
        assert_eq!(PropertyType::try_from("LANG").unwrap().to_string().as_str(), "LANG");
        assert_eq!(PropertyType::try_from("LOGO").unwrap().to_string().as_str(), "LOGO");
        assert_eq!(PropertyType::try_from("MEMBER").unwrap().to_string().as_str(), "MEMBER");
        assert_eq!(PropertyType::try_from("NICKNAME").unwrap().to_string().as_str(), "NICKNAME");
        assert_eq!(PropertyType::try_from("NOTE").unwrap().to_string().as_str(), "NOTE");
        assert_eq!(PropertyType::try_from("N").unwrap().to_string().as_str(), "N");
        assert_eq!(PropertyType::try_from("ORG-DIRECTORY").unwrap().to_string().as_str(), "ORG-DIRECTORY");
        assert_eq!(PropertyType::try_from("ORG").unwrap().to_string().as_str(), "ORG");
        assert_eq!(PropertyType::try_from("PHOTO").unwrap().to_string().as_str(), "PHOTO");
        assert_eq!(PropertyType::try_from("PRODID").unwrap().to_string().as_str(), "PRODID");
        assert_eq!(PropertyType::try_from("RELATED").unwrap().to_string().as_str(), "RELATED");
        assert_eq!(PropertyType::try_from("REV").unwrap().to_string().as_str(), "REV");
        assert_eq!(PropertyType::try_from("ROLE").unwrap().to_string().as_str(), "ROLE");
        assert_eq!(PropertyType::try_from("SOUND").unwrap().to_string().as_str(), "SOUND");
        assert_eq!(PropertyType::try_from("SOURCE").unwrap().to_string().as_str(), "SOURCE");
        assert_eq!(PropertyType::try_from("TEL").unwrap().to_string().as_str(), "TEL");
        assert_eq!(PropertyType::try_from("TITLE").unwrap().to_string().as_str(), "TITLE");
        assert_eq!(PropertyType::try_from("TZ").unwrap().to_string().as_str(), "TZ");
        assert_eq!(PropertyType::try_from("UID").unwrap().to_string().as_str(), "UID");
        assert_eq!(PropertyType::try_from("URL").unwrap().to_string().as_str(), "URL");
        assert_eq!(PropertyType::try_from("VERSION").unwrap().to_string().as_str(), "VERSION");
        assert_eq!(PropertyType::try_from("XML").unwrap().to_string().as_str(), "XML");
    }

    #[test]
    pub fn property_type_grouped() {
        assert_eq!(PropertyType::try_from("item1.URL").unwrap().to_string().as_str(), "URL");
    }
}
