use std::fmt::{Display, Formatter};

use crate::vcard::parameter::types::ParameterType;
use crate::vcard::properties::adr::adr_allowed_parameter;
use crate::vcard::properties::anniversary::anniversary_allowed_parameter;
use crate::vcard::properties::bday::bday_allowed_parameter;
use crate::vcard::properties::birthplace::birthplace_allowed_parameter;
use crate::vcard::properties::caladruri::caladruri_allowed_parameter;
use crate::vcard::properties::caluri::caluri_allowed_parameter;
use crate::vcard::properties::categories::categories_allowed_parameter;
use crate::vcard::properties::clientpidmap::clientpidmap_allowed_parameter;
use crate::vcard::properties::contacturi::contacturi_allowed_parameter;
use crate::vcard::properties::deathdate::deathdate_allowed_parameter;
use crate::vcard::properties::deathplace::deathplace_allowed_parameter;
use crate::vcard::properties::email::email_allowed_parameter;
use crate::vcard::properties::expertise::expertise_allowed_parameter;
use crate::vcard::properties::fburl::fburl_allowed_parameter;
use crate::vcard::properties::gender::gender_allowed_parameter;
use crate::vcard::properties::geo::geo_allowed_parameter;
use crate::vcard::properties::hobby::hobby_allowed_parameter;
use crate::vcard::properties::impp::impp_allowed_parameter;
use crate::vcard::properties::interest::interest_allowed_parameter;
use crate::vcard::properties::key::key_allowed_parameter;
use crate::vcard::properties::kind::kind_allowed_parameter;
use crate::vcard::properties::lang::lang_allowed_parameter;
use crate::vcard::properties::logo::logo_allowed_parameter;
use crate::vcard::properties::member::member_allowed_parameter;
use crate::vcard::properties::n::n_allowed_parameter;
use crate::vcard::properties::nickname::nickname_allowed_parameter;
use crate::vcard::properties::note::note_allowed_parameter;
use crate::vcard::properties::org::org_allowed_parameter;
use crate::vcard::properties::orgdirectory::orgdirectory_allowed_parameter;
use crate::vcard::properties::photo::photo_allowed_parameter;
use crate::vcard::properties::prodid::prodid_allowed_parameter;
use crate::vcard::properties::r#fn::fn_allowed_parameter;
use crate::vcard::properties::related::related_allowed_parameter;
use crate::vcard::properties::rev::rev_allowed_parameter;
use crate::vcard::properties::role::role_allowed_parameter;
use crate::vcard::properties::sound::sound_allowed_parameter;
use crate::vcard::properties::source::source_allowed_parameter;
use crate::vcard::properties::tel::tel_allowed_parameter;
use crate::vcard::properties::title::title_allowed_parameter;
use crate::vcard::properties::tz::tz_allowed_parameter;
use crate::vcard::properties::uid::uid_allowed_parameter;
use crate::vcard::properties::url::url_allowed_parameter;
use crate::vcard::properties::version::version_allowed_parameter;
use crate::vcard::properties::xml::xml_allowed_parameter;
use crate::vcard::property::types::PropertyType;
use crate::vcard::values::Value;
use crate::VcardError;

/// Stores the parameter type as an enum variant.
pub mod types;

/// Stores parameter data including type and value.  Normally you won't create this manually, as
/// parameter type and value must be validated based on property type.
///
/// # Examples
/// ```
/// use vcard_parser::vcard::parameter::Parameter;
/// use vcard_parser::vcard::property::types::PropertyType;
///
/// let parameter = Parameter::try_from((&PropertyType::Tel, "TYPE=WORK")).expect("Unable to parse parameter.");
/// assert_eq!(parameter.to_string(), "TYPE=WORK");
/// ```
#[derive(Clone, Debug)]
pub struct Parameter {
    parameter_type: ParameterType,
    parameter_value: Value,
}

impl Parameter {
    pub fn get_type(&self) -> &ParameterType {
        &self.parameter_type
    }
    pub fn get_value(&self) -> &Value {
        &self.parameter_value
    }
    pub fn is_type(&self, parameter_type: ParameterType) -> bool {
        self.parameter_type == parameter_type
    }
    pub fn build_parameters(property_type: &PropertyType, option: Option<&str>) -> Result<Vec<Parameter>, VcardError> {
        let mut parameters = Vec::new();

        if let Some(str) = option {
            for str in str.split(';') {
                parameters.push(Parameter::try_from((property_type, str))?)
            }
        }

        Ok(parameters)
    }
    fn validate(&self, property_type: &PropertyType) -> Result<(), VcardError> {
        match property_type {
            PropertyType::Adr => adr_allowed_parameter(&self.parameter_type),
            PropertyType::Anniversary => anniversary_allowed_parameter(&self.parameter_type),
            PropertyType::BDay => bday_allowed_parameter(&self.parameter_type),
            PropertyType::BirthPlace => birthplace_allowed_parameter(&self.parameter_type),
            PropertyType::CalAdrUri => caladruri_allowed_parameter(&self.parameter_type),
            PropertyType::CalUri => caluri_allowed_parameter(&self.parameter_type),
            PropertyType::Categories => categories_allowed_parameter(&self.parameter_type),
            PropertyType::ClientPidMap => clientpidmap_allowed_parameter(&self.parameter_type),
            PropertyType::ContactUri => contacturi_allowed_parameter(&self.parameter_type),
            PropertyType::DeathDate => deathdate_allowed_parameter(&self.parameter_type),
            PropertyType::DeathPlace => deathplace_allowed_parameter(&self.parameter_type),
            PropertyType::Email => email_allowed_parameter(&self.parameter_type),
            PropertyType::Expertise => expertise_allowed_parameter(&self.parameter_type),
            PropertyType::FbUrl => fburl_allowed_parameter(&self.parameter_type),
            PropertyType::Fn => fn_allowed_parameter(&self.parameter_type),
            PropertyType::Gender => gender_allowed_parameter(&self.parameter_type),
            PropertyType::Geo => geo_allowed_parameter(&self.parameter_type),
            PropertyType::Hobby => hobby_allowed_parameter(&self.parameter_type),
            PropertyType::Impp => impp_allowed_parameter(&self.parameter_type),
            PropertyType::Interest => interest_allowed_parameter(&self.parameter_type),
            PropertyType::Key => key_allowed_parameter(&self.parameter_type),
            PropertyType::Kind => kind_allowed_parameter(&self.parameter_type),
            PropertyType::Lang => lang_allowed_parameter(&self.parameter_type),
            PropertyType::Logo => logo_allowed_parameter(&self.parameter_type),
            PropertyType::Member => member_allowed_parameter(&self.parameter_type),
            PropertyType::NickName => nickname_allowed_parameter(&self.parameter_type),
            PropertyType::Note => note_allowed_parameter(&self.parameter_type),
            PropertyType::N => n_allowed_parameter(&self.parameter_type),
            PropertyType::OrgDirectory => orgdirectory_allowed_parameter(&self.parameter_type),
            PropertyType::Org => org_allowed_parameter(&self.parameter_type),
            PropertyType::Photo => photo_allowed_parameter(&self.parameter_type),
            PropertyType::ProdId => prodid_allowed_parameter(&self.parameter_type),
            PropertyType::Related => related_allowed_parameter(&self.parameter_type),
            PropertyType::Rev => rev_allowed_parameter(&self.parameter_type),
            PropertyType::Role => role_allowed_parameter(&self.parameter_type),
            PropertyType::Sound => sound_allowed_parameter(&self.parameter_type),
            PropertyType::Source => source_allowed_parameter(&self.parameter_type),
            PropertyType::Tel => tel_allowed_parameter(&self.parameter_type),
            PropertyType::Title => title_allowed_parameter(&self.parameter_type),
            PropertyType::Tz => tz_allowed_parameter(&self.parameter_type),
            PropertyType::Uid => uid_allowed_parameter(&self.parameter_type),
            PropertyType::Url => url_allowed_parameter(&self.parameter_type),
            PropertyType::Version => version_allowed_parameter(&self.parameter_type),
            PropertyType::Xml => xml_allowed_parameter(&self.parameter_type),
        }
    }
}

impl TryFrom<(&PropertyType, &str)> for Parameter {
    type Error = VcardError;
    fn try_from((property_type, str): (&PropertyType, &str)) -> Result<Self, Self::Error> {
        match str.split_once('=') {
            None => Err(VcardError::ParameterMalformed(str.to_string(), property_type.to_string())),
            Some((pt, pv)) => {
                let parameter_type = ParameterType::try_from(pt)?;
                let parameter_value = Value::try_from((&parameter_type, pv))?;
                let parameter = Parameter {
                    parameter_type,
                    parameter_value,
                };
                parameter.validate(property_type)?;
                Ok(parameter)
            }
        }
    }
}

impl Display for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.parameter_type, self.parameter_value)
    }
}

#[cfg(test)]
mod tests {
    use crate::vcard::parameter::Parameter;
    use crate::vcard::property::types::PropertyType;

    #[test]
    pub fn parameter_formatting() {
        assert_eq!(Parameter::try_from((&PropertyType::Tel, "TYPE=WORK")).unwrap().to_string(), "TYPE=WORK");
    }
}
