//! Constants for string matching.

#[non_exhaustive]
pub struct Cardinality;

impl Cardinality {
    pub const SINGLE: &'static str = "SINGLE";
    pub const MULTIPLE: &'static str = "MULTIPLE";
}

#[non_exhaustive]
pub struct Encoding;

/// Value parameter possible values, see [RFC 6350 5.2](https://datatracker.ietf.org/doc/html/rfc6350#section-5.2)
impl Encoding {
    pub const UNESCAPED_BACKSLASH: char = '\\';
    pub const UNESCAPED_CR: char = '\r';
    pub const UNESCAPED_COLON: char = ':';
    pub const UNESCAPED_COMMA: char = ',';
    pub const UNESCAPED_EQUALS: char = '=';
    pub const UNESCAPED_LF: char = '\n';
    pub const UNESCAPED_SEMICOLON: char = ';';
    pub const UNESCAPED_TAB: char = '\t';

    pub const ESCAPED_BACKSLASH: &'static str = r"\\";
    pub const ESCAPED_CR: &'static str = r"\\r";
    pub const ESCAPED_COLON: &'static str = r"\:";
    pub const ESCAPED_COMMA: &'static str = r"\,";
    pub const ESCAPED_EQUALS: &'static str = r"\=";
    pub const ESCAPED_LF: &'static str = r"\\n";
    pub const ESCAPED_SEMICOLON: &'static str = r"\;";
    pub const ESCAPED_TAB: &'static str = r"\\t";
}

#[non_exhaustive]
pub struct PropertyName;

impl PropertyName {
    pub const BEGIN: &'static str = "BEGIN";
    pub const VERSION: &'static str = "VERSION";
    pub const END: &'static str = "END";

    pub const ADR: &'static str = "ADR";
    pub const ANNIVERSARY: &'static str = "ANNIVERSARY";
    pub const BDAY: &'static str = "BDAY";
    pub const BIRTHPLACE: &'static str = "BIRTHPLACE";
    pub const CALADRURI: &'static str = "CALADRURI";
    pub const CALURI: &'static str = "CALURI";
    pub const CATEGORIES: &'static str = "CATEGORIES";
    pub const CLIENTPIDMAP: &'static str = "CLIENTPIDMAP";
    pub const CONTACTURI: &'static str = "CONTACT-URI";
    pub const DEATHDATE: &'static str = "DEATHDATE";
    pub const DEATHPLACE: &'static str = "DEATHPLACE";
    pub const EMAIL: &'static str = "EMAIL";
    pub const EXPERTISE: &'static str = "EXPERTISE";
    pub const FBURL: &'static str = "FBURL";
    pub const FN: &'static str = "FN";
    pub const GENDER: &'static str = "GENDER";
    pub const GEO: &'static str = "GEO";
    pub const HOBBY: &'static str = "HOBBY";
    pub const IMPP: &'static str = "IMPP";
    pub const INTEREST: &'static str = "INTEREST";
    pub const KEY: &'static str = "KEY";
    pub const KIND: &'static str = "KIND";
    pub const LANG: &'static str = "LANG";
    pub const LOGO: &'static str = "LOGO";
    pub const MEMBER: &'static str = "MEMBER";
    pub const NICKNAME: &'static str = "NICKNAME";
    pub const NOTE: &'static str = "NOTE";
    pub const N: &'static str = "N";
    pub const ORGDIRECTORY: &'static str = "ORG-DIRECTORY";
    pub const ORG: &'static str = "ORG";
    pub const PHOTO: &'static str = "PHOTO";
    pub const PRODID: &'static str = "PRODID";
    pub const RELATED: &'static str = "RELATED";
    pub const REV: &'static str = "REV";
    pub const ROLE: &'static str = "ROLE";
    pub const SOUND: &'static str = "SOUND";
    pub const SOURCE: &'static str = "SOURCE";
    pub const TEL: &'static str = "TEL";
    pub const TITLE: &'static str = "TITLE";
    pub const TZ: &'static str = "TZ";
    pub const UID: &'static str = "UID";
    pub const URL: &'static str = "URL";
    pub const XML: &'static str = "XML";
}

#[non_exhaustive]
pub struct ParameterName;

impl ParameterName {
    pub const ALTID: &'static str = "ALTID";
    pub const ANY: &'static str = "ANY";
    pub const CALSCALE: &'static str = "CALSCALE";
    pub const CC: &'static str = "CC";
    pub const GEO: &'static str = "GEO";
    pub const INDEX: &'static str = "INDEX";
    pub const LABEL: &'static str = "LABEL";
    pub const LANGUAGE: &'static str = "LANGUAGE";
    pub const LEVEL: &'static str = "LEVEL";
    pub const MEDIATYPE: &'static str = "MEDIATYPE";
    pub const PID: &'static str = "PID";
    pub const PREF: &'static str = "PREF";
    pub const SORTAS: &'static str = "SORT-AS";
    pub const TYPE: &'static str = "TYPE";
    pub const TZ: &'static str = "TZ";
    pub const VALUE: &'static str = "VALUE";
}

#[non_exhaustive]
pub struct ValueName;

impl ValueName {
    pub const BOOLEAN: &'static str = "BOOLEAN";
    pub const CLIENTPIDMAP: &'static str = "CLIENTPIDMAP";
    pub const DATE: &'static str = "DATE";
    pub const FLOAT: &'static str = "FLOAT";
    pub const INTEGER: &'static str = "INTEGER";
    pub const LANGUAGE_TAG: &'static str = "LANGUAGETAG";
    pub const LISTCOMPONENT: &'static str = "LISTCOMPONENT";
    pub const PID: &'static str = "PID";
    pub const TEXT: &'static str = "TEXT";
    pub const TEXTLIST: &'static str = "TEXTLIST";
    pub const TIMESTAMP: &'static str = "TIMESTAMP";
    pub const URI: &'static str = "URI";
    pub const UTCOFFSET: &'static str = "UTCOFFSET";
}

#[non_exhaustive]
pub struct ValueType;

/// Value parameter possible values, see [RFC 6350 5.2](https://datatracker.ietf.org/doc/html/rfc6350#section-5.2)
impl ValueType {
    pub const BOOLEAN: &'static str = "BOOLEAN";
    pub const DATE_AND_OR_TIME: &'static str = "DATE-AND-OR-TIME";
    pub const DATE_TIME: &'static str = "DATE-TIME";
    pub const DATE: &'static str = "DATE";
    pub const FLOAT: &'static str = "FLOAT";
    pub const INTEGER: &'static str = "INTEGER";
    pub const LANGUAGE_TAG: &'static str = "LANGUAGE-TAG";
    pub const TEXT: &'static str = "TEXT";
    pub const TIME: &'static str = "TIME";
    pub const TIMESTAMP: &'static str = "TIMESTAMP";
    pub const URI: &'static str = "URI";
    pub const UTC_OFFSET: &'static str = "UTC-OFFSET";

    pub const TYPES: [&'static str; 12] = [
        ValueType::BOOLEAN,
        ValueType::DATE_AND_OR_TIME,
        ValueType::DATE_TIME,
        ValueType::DATE,
        ValueType::FLOAT,
        ValueType::INTEGER,
        ValueType::LANGUAGE_TAG,
        ValueType::TEXT,
        ValueType::TIME,
        ValueType::TIMESTAMP,
        ValueType::URI,
        ValueType::UTC_OFFSET,
    ];
}

#[non_exhaustive]
pub struct PropertyExpertiseValues;

impl PropertyExpertiseValues {
    pub const BEGINNER: &'static str = "BEGINNER";
    pub const AVERAGE: &'static str = "AVERAGE";
    pub const EXPERT: &'static str = "EXPERT";
    pub const LOW: &'static str = "LOW";
    pub const MEDIUM: &'static str = "MEDIUM";
    pub const HIGH: &'static str = "HIGH";

    pub const TYPES: [&'static str; 6] = [
        PropertyExpertiseValues::BEGINNER,
        PropertyExpertiseValues::AVERAGE,
        PropertyExpertiseValues::EXPERT,
        PropertyExpertiseValues::LOW,
        PropertyExpertiseValues::MEDIUM,
        PropertyExpertiseValues::HIGH,
    ];
}

#[non_exhaustive]
pub struct PropertyGenderValues;

impl PropertyGenderValues {
    pub const M: &'static str = "M";
    pub const F: &'static str = "F";
    pub const N: &'static str = "N";
    pub const O: &'static str = "O";
    pub const U: &'static str = "U";

    pub const TYPES: [&'static str; 5] = [
        PropertyGenderValues::M,
        PropertyGenderValues::F,
        PropertyGenderValues::N,
        PropertyGenderValues::O,
        PropertyGenderValues::U,
    ];
}

#[non_exhaustive]
pub struct PropertyHobbyValues;

impl PropertyHobbyValues {
    pub const LOW: &'static str = "LOW";
    pub const MEDIUM: &'static str = "MEDIUM";
    pub const HIGH: &'static str = "HIGH";

    pub const TYPES: [&'static str; 3] = [
        PropertyHobbyValues::LOW,
        PropertyHobbyValues::MEDIUM,
        PropertyHobbyValues::HIGH,
    ];
}

#[non_exhaustive]
pub struct PropertyInterestValues;

impl PropertyInterestValues {
    pub const LOW: &'static str = "LOW";
    pub const MEDIUM: &'static str = "MEDIUM";
    pub const HIGH: &'static str = "HIGH";

    pub const TYPES: [&'static str; 3] = [
        PropertyInterestValues::LOW,
        PropertyInterestValues::MEDIUM,
        PropertyInterestValues::HIGH,
    ];
}

#[non_exhaustive]
pub struct PropertyKindValues;

impl PropertyKindValues {
    pub const INDIVIDUAL: &'static str = "INDIVIDUAL";
    pub const GROUP: &'static str = "GROUP";
    pub const ORG: &'static str = "ORG";
    pub const LOCATION: &'static str = "LOCATION";

    pub const TYPES: [&'static str; 4] = [
        PropertyKindValues::INDIVIDUAL,
        PropertyKindValues::GROUP,
        PropertyKindValues::ORG,
        PropertyKindValues::LOCATION,
    ];
}

#[non_exhaustive]
pub struct TestData;

impl TestData {
    pub const VCARD_ERROR_VERSION_INCORRECT: &'static str = "BEGIN:VCARD\nVERSION:3.0\nFN:John Doe\nEND:VCARD\n";
    pub const VCARD_ERROR_VERSION_MISSING: &'static str = "BEGIN:VCARD\nFN:John Doe\nEND:VCARD\n";
    pub const VCARD_ERROR_BEGIN_MISSING: &'static str = "VERSION:4.0\nFN:John Doe\nEND:VCARD\n";
    pub const VCARD_ERROR_END_MISSING: &'static str = "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\n";
    pub const VCARD_ERROR_FULLNAME_MISSING: &'static str = "BEGIN:VCARD\nVERSION:4.0\nEND:VCARD\n";
    pub const VCARD_MATCH_MINIMAL: (&'static str, &'static str) = ("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n", "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEND:VCARD\n");
    pub const VCARD_MATCH_CONCAT: (&'static str, &'static str) = ("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nN:Doe;\n John\n\t;Jr.;;\nEND:VCARD\n", "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nN:Doe;John;Jr.;;\nEND:VCARD\n");
    pub const VCARD_MATCH_XNAME: (&'static str, &'static str) = ("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nitem1.X-ABADR;X-SERVICE=TEST:us\nEND:VCARD\n", "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nitem1.X-ABADR;X-SERVICE=TEST:us\nEND:VCARD\n");
    pub const VCARD_MATCH_COMPOUND: (&'static str, &'static str) = ("BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEMAIL;TYPE=\"INTERNET,HOME\":user@example.com\nEND:VCARD\n", "BEGIN:VCARD\nVERSION:4.0\nFN:John Doe\nEMAIL;TYPE=\"INTERNET,HOME\":user@example.com\nEND:VCARD\n");
}

#[non_exhaustive]
pub struct TestDataPropertyValues;

impl TestDataPropertyValues {
    pub const ADR: &'static str = r";;123 Main Street;Any Town;CA;91921-1234;U.S.A.";
    pub const ANNIVERSARY: &'static str = r"19960415";
    pub const BDAY: &'static str = r"19531015T231000Z";
    pub const BIRTHPLACE: &'static str = r"geo:46.769307,-71.283079";
    pub const CALADRURI: &'static str = r"mailto:janedoe@example.com";
    pub const CALURI: &'static str = r"ftp://ftp.example.com/calA.ics";
    pub const CATEGORIES: &'static str = r"INTERNET,IETF,INDUSTRY,INFORMATION TECHNOLOGY";
    pub const CLIENTPIDMAP: &'static str = r"1;urn:uuid:3df403f4-5924-4bb7-b077-3c711d9eb34b";
    pub const CONTACTURI: &'static str = r"https://contact.example.com";
    pub const DEATHDATE: &'static str = r"circa 1800";
    pub const DEATHPLACE: &'static str = r"Aboard the Titanic\, near Newfoundland";
    pub const EMAIL: &'static str = r"jqpublic@xyz.example.com";
    pub const EXPERTISE: &'static str = r"chemistry";
    pub const FBURL: &'static str = r"ftp://example.com/busy/project-a.ifb";
    pub const FN: &'static str = r"Mr. John Q. Public\, Esq.";
    pub const GENDER: &'static str = r"M;Fellow";
    pub const GEO: &'static str = r"geo:37.386013,-122.082932";
    pub const HOBBY: &'static str = r"reading";
    pub const IMPP: &'static str = r"xmpp:alice@example.com";
    pub const INTEREST: &'static str = r"rock 'n' roll music";
    pub const KEY: &'static str = r"ftp://example.com/keys/jdoe";
    pub const KIND: &'static str = r"individual";
    pub const LANG: &'static str = r"en";
    pub const LOGO: &'static str = r"https://www.example.com/pub/logos/abccorp.jpg";
    pub const MEMBER: &'static str = r"mailto:subscriber1@example.com";
    pub const NICKNAME: &'static str = r"Jim,Jimmie";
    pub const NOTE: &'static str = r"This fax number is operational 0800 to 1715\\nEST\, Mon-Fri.";
    pub const N: &'static str = r"N:Public;John;Quinlan;Mr.;Esq.";
    pub const ORGDIRECTORY: &'static str = r"ldap://ldap.tech.example/o=Example%20Tech,ou=Engineering";
    pub const ORG: &'static str = r"ABC\, Inc.;North American Division;Marketing";
    pub const PHOTO: &'static str = r"data:image/jpeg;base64,MIICajCCAdOgAwIBAgICBEUwDQYJKoZIhv";
    pub const PRODID: &'static str = r"-//ONLINE DIRECTORY//NONSGML Version 1//EN";
    pub const RELATED: &'static str = r"contact:https://example.com/directory/jdoe.vcf";
    pub const REV: &'static str = r"19951031T222710Z";
    pub const ROLE: &'static str = r"Project Leader";
    pub const SOUND: &'static str = r"CID:JOHNQPUBLIC.part8.19960229T080000.xyzMail@example.com";
    pub const SOURCE: &'static str = r"ldap://ldap.example.com/cn=Babs%20Jensen,%20o=Babsco,%20c=US";
    pub const TEL: &'static str = r"tel:+1-555-555-5555;ext=5555";
    pub const TITLE: &'static str = r"Research Scientist";
    pub const TZ: &'static str = r"Raleigh/North America";
    pub const UID: &'static str = r"urn:uuid:f81d4fae-7dec-11d0-a765-00a0c91e6bf6";
    pub const URL: &'static str = r"https://example.org/restaurant.french/~chezchic.html";
    pub const XML: &'static str = r#"<?xml version=\"1.0\" encoding=\"UTF-8\"?><vcards xmlns=\"urn:ietf:params:xml:ns:vcard-4.0\"><vcard></vcard></vcards>"#;
}

#[non_exhaustive]
pub struct VcardParseError;

impl VcardParseError {
    pub const DELIMITER_COLON: &'static str = "DELIMITER_COLON";
    pub const DELIMITER_COMMA: &'static str = "DELIMITER_COMMA";
    pub const DELIMITER_CONCAT: &'static str = "DELIMITER_CONCAT";
    pub const DELIMITER_EQUALS: &'static str = "DELIMITER_EQUALS";
    pub const DELIMITER_SEMI_COLON: &'static str = "DELIMITER_SEMI_COLON";
    pub const PARAMETER: &'static str = "PARAMETER";
    pub const PARAMETER_TYPE: &'static str = "PARAMETER_TYPE";
    pub const PARAMETER_VALUE: &'static str = "PARAMETER_VALUE";
    pub const PARAMETER_XNAME: &'static str = "PARAMETER_XNAME";
    pub const PROPERTY: &'static str = "PROPERTY";
    pub const PROPERTY_BEGIN: &'static str = "PROPERTY_BEGIN";
    pub const PROPERTY_BEGIN_MISSING: &'static str = "PROPERTY_BEGIN_MISSING";
    pub const PROPERTY_END: &'static str = "PROPERTY_END";
    pub const PROPERTY_END_MISSING: &'static str = "PROPERTY_END_MISSING";
    pub const PROPERTY_GROUP: &'static str = "PROPERTY_GROUP";
    pub const PROPERTY_IANA_TOKEN: &'static str = "PROPERTY_IANA_TOKEN";
    pub const PROPERTY_NAME: &'static str = "PROPERTY_NAME";
    pub const PROPERTY_VALUE: &'static str = "PROPERTY_VALUE";
    pub const PROPERTY_VERSION: &'static str = "PROPERTY_VERSION";
    pub const PROPERTY_VERSION_MISSING: &'static str = "PROPERTY_VERSION_MISSING";
    pub const PROPERTY_XNAME: &'static str = "PROPERTY_XNAME";
    pub const VALUE: &'static str = "VALUE";
    pub const VALUE_FOLDED: &'static str = "VALUE_FOLDED";
    pub const VALUE_QSAFE: &'static str = "VALUE_QSAFE";
    pub const VALUE_SAFE: &'static str = "VALUE_SAFE";
    pub const VCARD: &'static str = "VCARD";
    pub const VCARDS: &'static str = "VCARDS";
}
