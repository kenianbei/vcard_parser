//! Property functions.

use nom::branch::alt;
use nom::bytes::complete::{tag, tag_no_case, take_while1};
use nom::character::complete::line_ending;
use nom::combinator::{not, opt, peek, recognize};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::tuple;
use nom::{IResult, Parser};

use crate::constants::{PropertyName, VcardParseError};
use crate::parse::delimiters::colon;
use crate::parse::parameter::parameter;
use crate::parse::value::{is_alphanumeric_dash, value};
use crate::parse::{Data, PropertyData, PropertyNameData, PropertyNameWithGroupData, PropertyParametersData, ValueData, ValueFoldedData};
use crate::VcardError;

/// Parse all properties that aren't delimiters (BEGIN, VERSION, END).
pub fn property(i: Data) -> IResult<Data, PropertyData, VcardError> {
    match context(VcardParseError::PROPERTY, tuple((property_name, many0(parameter), colon, property_value, line_ending)))(i) {
        Ok((i, (property_name, parameters, _, value, _))) => Ok((i, (property_name, parameters, value))),
        Err(err) => Err(err),
    }
}

/// Parse BEGIN property.
pub fn property_begin(i: Data) -> IResult<Data, (PropertyNameData, PropertyParametersData, ValueData), VcardError> {
    match context(VcardParseError::PROPERTY_BEGIN, tuple((property_name_begin, colon, tag("VCARD"), line_ending)))(i) {
        Ok((i, (property_name, _, value, _))) => Ok((i, (property_name, Vec::new(), value))),
        Err(err) => Err(err),
    }
}

/// Parse VERSION property.
pub fn property_version(i: Data) -> IResult<Data, (PropertyNameData, PropertyParametersData, ValueData), VcardError> {
    match context(VcardParseError::PROPERTY_VERSION, tuple((property_name_version, colon, tag("4.0"), line_ending)))(i) {
        Ok((i, (property_name, _, value, _))) => Ok((i, (property_name, Vec::new(), value))),
        Err(err) => Err(err),
    }
}

/// Parse END property.
pub fn property_end(i: Data) -> IResult<Data, (PropertyNameData, PropertyParametersData, ValueData), VcardError> {
    match context(VcardParseError::PROPERTY_END, tuple((property_name_end, colon, tag("VCARD"), line_ending)))(i) {
        Ok((i, (property_name, _, value, _))) => Ok((i, (property_name, Vec::new(), value))),
        Err(err) => Err(err),
    }
}

/// Parse property value.
/// TODO: Decide whether to add escaping here.
pub fn property_value(i: Data) -> IResult<Data, ValueFoldedData, VcardError> {
    match context(VcardParseError::PROPERTY_VALUE, tuple((value, peek(line_ending))))(i) {
        Ok((i, (data, _))) => Ok((i, data)),
        Err(err) => Err(err),
    }
}

/// Parse property names, including x-name and iana-tokens.
pub fn property_name(i: Data) -> IResult<Data, PropertyNameWithGroupData, VcardError> {
    match context(
        VcardParseError::PROPERTY_NAME,
        tuple((
            opt(property_group),
            alt((
                alt((property_name_adr, property_name_anniversary, property_name_bday, property_name_birthplace, property_name_caladruri, property_name_caluri, property_name_categories, property_name_clientpidmap, property_name_contacturi, property_name_deathdate, property_name_deathplace)),
                alt((property_name_email, property_name_expertise, property_name_fburl, property_name_fn, property_name_gender, property_name_geo, property_name_hobby, property_name_impp, property_name_interest, property_name_key, property_name_kind)),
                alt((property_name_lang, property_name_logo, property_name_member, property_name_nickname, property_name_note, property_name_n, property_name_orgdirectory, property_name_org, property_name_photo, property_name_prodid, property_name_related)),
                alt((property_name_rev, property_name_role, property_name_sound, property_name_source, property_name_tel, property_name_title, property_name_tz, property_name_uid, property_name_url, property_name_xml)),
                alt((property_x_name, property_iana_token)),
            )),
        )),
    )(i)
    {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse property group name.
pub fn property_group(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_GROUP, tuple((take_while1(is_alphanumeric_dash), tag("."), peek(property_name))))(i) {
        Ok((i, (s, _, _))) => Ok((i, s)),
        Err(err) => Err(err),
    }
}

/// Parse BEGIN property name.
pub fn property_name_begin(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_BEGIN_MISSING, tag_no_case(PropertyName::BEGIN))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse VERSION property name.
pub fn property_name_version(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_VERSION_MISSING, tag_no_case(PropertyName::VERSION))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse END property name.
pub fn property_name_end(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_END_MISSING, tag_no_case(PropertyName::END))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ADR property name.
pub fn property_name_adr(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::ADR)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ANNIVERSARY property name.
pub fn property_name_anniversary(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::ANNIVERSARY)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse BDAY property name.
pub fn property_name_bday(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::BDAY)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse BIRTHPLACE property name.
pub fn property_name_birthplace(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::BIRTHPLACE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CALADRURI property name.
pub fn property_name_caladruri(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::CALADRURI)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CALURI property name.
pub fn property_name_caluri(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::CALURI)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CATEGORIES property name.
pub fn property_name_categories(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::CATEGORIES)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CLIENTPIDMAP property name.
pub fn property_name_clientpidmap(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::CLIENTPIDMAP)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse CONTACTURI property name.
pub fn property_name_contacturi(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::CONTACTURI)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse DEATHDATE property name.
pub fn property_name_deathdate(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::DEATHDATE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse DEATHPLACE property name.
pub fn property_name_deathplace(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::DEATHPLACE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse EMAIL property name.
pub fn property_name_email(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::EMAIL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse EXPERTISE property name.
pub fn property_name_expertise(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::EXPERTISE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse FBURL property name.
pub fn property_name_fburl(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::FBURL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse FN property name.
pub fn property_name_fn(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::FN)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse GENDER property name.
pub fn property_name_gender(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::GENDER)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse GEO property name.
pub fn property_name_geo(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::GEO)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse HOBBY property name.
pub fn property_name_hobby(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::HOBBY)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse IMPP property name.
pub fn property_name_impp(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::IMPP)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse INTEREST property name.
pub fn property_name_interest(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::INTEREST)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse KEY property name.
pub fn property_name_key(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::KEY)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse KIND property name.
pub fn property_name_kind(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::KIND)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse LANG property name.
pub fn property_name_lang(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::LANG)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse LOGO property name.
pub fn property_name_logo(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::LOGO)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse MEMBER property name.
pub fn property_name_member(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::MEMBER)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse NICKNAME property name.
pub fn property_name_nickname(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::NICKNAME)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse NOTE property name.
pub fn property_name_note(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::NOTE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse N property name.
pub fn property_name_n(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::N)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ORGDIRECTORY property name.
pub fn property_name_orgdirectory(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::ORGDIRECTORY)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ORG property name.
pub fn property_name_org(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::ORG)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse PHOTO property name.
pub fn property_name_photo(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::PHOTO)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse PRODID property name.
pub fn property_name_prodid(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::PRODID)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse RELATED property name.
pub fn property_name_related(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::RELATED)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse REV property name.
pub fn property_name_rev(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::REV)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse ROLE property name.
pub fn property_name_role(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::ROLE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse SOUND property name.
pub fn property_name_sound(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::SOUND)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse SOURCE property name.
pub fn property_name_source(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::SOURCE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse TEL property name.
pub fn property_name_tel(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::TEL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse TITLE property name.
pub fn property_name_title(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::TITLE)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse TZ property name.
pub fn property_name_tz(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::TZ)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse UID property name.
pub fn property_name_uid(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::UID)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse URL property name.
pub fn property_name_url(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::URL)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse XML property name.
pub fn property_name_xml(i: Data) -> IResult<Data, Data, VcardError> {
    match tag_no_case(PropertyName::XML)(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

/// Parse iana-token property name.
pub fn property_iana_token(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_IANA_TOKEN, not(property_name_begin).and(not(property_name_version).and(not(property_name_end).and(take_while1(is_alphanumeric_dash)))))(i) {
        Ok((i, (_, (_, (_, s))))) => Ok((i, s)),
        Err(err) => Err(err),
    }
}

/// Parse x-name property name.
pub fn property_x_name(i: Data) -> IResult<Data, Data, VcardError> {
    match context(VcardParseError::PROPERTY_XNAME, recognize(tuple((tag_no_case("x-"), take_while1(is_alphanumeric_dash)))))(i) {
        Ok(data) => Ok(data),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use nom::{AsBytes, Parser};

    use crate::constants::{PropertyName, TestDataPropertyValues};
    use crate::parse::property::{property, property_begin, property_iana_token, property_x_name};

    #[test]
    fn parse_property() {
        assert!(property.parse("FN:John Doe\n".as_bytes()).is_ok());
        assert!(property.parse("X-FN:John Doe\n".as_bytes()).is_ok());
        assert!(property.parse("IANA-ETC:John Doe\n".as_bytes()).is_ok());
    }

    #[test]
    fn parse_property_basic() {
        let ((_, property_name), property_parameters, (property_value, _)) = property.parse("FN:John Doe\n".as_bytes()).unwrap().1;
        assert_eq!(property_name.as_bytes(), "FN".as_bytes());
        assert_eq!(property_parameters.len(), 0);
        assert_eq!(property_value, "John Doe".as_bytes());
    }

    #[test]
    fn parse_property_begin() {
        let (property_name, _, property_value) = property_begin.parse("BEGIN:VCARD\n".as_bytes()).unwrap().1;
        assert_eq!(property_name, "BEGIN".as_bytes());
        assert_eq!(property_value, "VCARD".as_bytes());
    }

    #[test]
    fn parse_property_properties() {
        fn _parse_property_properties(name: &str, value: &str) {
            let string = format!("{}:{}\n", name, value);
            let ((_, property_name), _, (property_value, _)) = property.parse(string.as_bytes()).unwrap().1;
            assert_eq!(property_name, name.as_bytes());
            assert_eq!(property_value, value.as_bytes());
        }

        _parse_property_properties(PropertyName::ADR, TestDataPropertyValues::ADR);
        _parse_property_properties(PropertyName::ANNIVERSARY, TestDataPropertyValues::ANNIVERSARY);
        _parse_property_properties(PropertyName::BDAY, TestDataPropertyValues::BDAY);
        _parse_property_properties(PropertyName::BIRTHPLACE, TestDataPropertyValues::BIRTHPLACE);
        _parse_property_properties(PropertyName::CALADRURI, TestDataPropertyValues::CALADRURI);
        _parse_property_properties(PropertyName::CALURI, TestDataPropertyValues::CALURI);
        _parse_property_properties(PropertyName::CATEGORIES, TestDataPropertyValues::CATEGORIES);
        _parse_property_properties(PropertyName::CLIENTPIDMAP, TestDataPropertyValues::CLIENTPIDMAP);
        _parse_property_properties(PropertyName::CONTACTURI, TestDataPropertyValues::CONTACTURI);
        _parse_property_properties(PropertyName::DEATHDATE, TestDataPropertyValues::DEATHDATE);
        _parse_property_properties(PropertyName::DEATHPLACE, TestDataPropertyValues::DEATHPLACE);
        _parse_property_properties(PropertyName::EMAIL, TestDataPropertyValues::EMAIL);
        _parse_property_properties(PropertyName::EXPERTISE, TestDataPropertyValues::EXPERTISE);
        _parse_property_properties(PropertyName::FBURL, TestDataPropertyValues::FBURL);
        _parse_property_properties(PropertyName::FN, TestDataPropertyValues::FN);
        _parse_property_properties(PropertyName::GENDER, TestDataPropertyValues::GENDER);
        _parse_property_properties(PropertyName::GEO, TestDataPropertyValues::GEO);
        _parse_property_properties(PropertyName::HOBBY, TestDataPropertyValues::HOBBY);
        _parse_property_properties(PropertyName::IMPP, TestDataPropertyValues::IMPP);
        _parse_property_properties(PropertyName::INTEREST, TestDataPropertyValues::INTEREST);
        _parse_property_properties(PropertyName::KEY, TestDataPropertyValues::KEY);
        _parse_property_properties(PropertyName::KIND, TestDataPropertyValues::KIND);
        _parse_property_properties(PropertyName::LANG, TestDataPropertyValues::LANG);
        _parse_property_properties(PropertyName::LOGO, TestDataPropertyValues::LOGO);
        _parse_property_properties(PropertyName::MEMBER, TestDataPropertyValues::MEMBER);
        _parse_property_properties(PropertyName::NICKNAME, TestDataPropertyValues::NICKNAME);
        _parse_property_properties(PropertyName::NOTE, TestDataPropertyValues::NOTE);
        _parse_property_properties(PropertyName::N, TestDataPropertyValues::N);
        _parse_property_properties(PropertyName::ORGDIRECTORY, TestDataPropertyValues::ORGDIRECTORY);
        _parse_property_properties(PropertyName::ORG, TestDataPropertyValues::ORG);
        _parse_property_properties(PropertyName::PHOTO, TestDataPropertyValues::PHOTO);
        _parse_property_properties(PropertyName::PRODID, TestDataPropertyValues::PRODID);
        _parse_property_properties(PropertyName::RELATED, TestDataPropertyValues::RELATED);
        _parse_property_properties(PropertyName::REV, TestDataPropertyValues::REV);
        _parse_property_properties(PropertyName::ROLE, TestDataPropertyValues::ROLE);
        _parse_property_properties(PropertyName::SOUND, TestDataPropertyValues::SOUND);
        _parse_property_properties(PropertyName::SOURCE, TestDataPropertyValues::SOURCE);
        _parse_property_properties(PropertyName::TEL, TestDataPropertyValues::TEL);
        _parse_property_properties(PropertyName::TITLE, TestDataPropertyValues::TITLE);
        _parse_property_properties(PropertyName::TZ, TestDataPropertyValues::TZ);
        _parse_property_properties(PropertyName::UID, TestDataPropertyValues::UID);
        _parse_property_properties(PropertyName::URL, TestDataPropertyValues::URL);
        _parse_property_properties(PropertyName::XML, TestDataPropertyValues::XML);
    }

    #[test]
    fn parse_property_x_name() {
        assert_eq!(property_x_name.parse("X-ABADR".as_bytes()).unwrap().1, "X-ABADR".as_bytes());
        assert_eq!(property_x_name.parse("X-ABADR:TEST".as_bytes()).unwrap().1, "X-ABADR".as_bytes());
        assert_eq!(property_x_name.parse("X-ABADR;TEST".as_bytes()).unwrap().1, "X-ABADR".as_bytes());
    }

    #[test]
    fn parse_property_iana_token() {
        assert!(property_iana_token.parse(PropertyName::BEGIN.as_bytes()).is_err());
        assert!(property_iana_token.parse(PropertyName::VERSION.as_bytes()).is_err());
        assert!(property_iana_token.parse(PropertyName::END.as_bytes()).is_err());
        assert_eq!(property_iana_token.parse("AB-ADR".as_bytes()).unwrap().1, "AB-ADR".as_bytes());
        assert_eq!(property_iana_token.parse("AB-ADR:TEST".as_bytes()).unwrap().1, "AB-ADR".as_bytes());
        assert_eq!(property_iana_token.parse("AB-ADR;TEST".as_bytes()).unwrap().1, "AB-ADR".as_bytes());
    }
}
