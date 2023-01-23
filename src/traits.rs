//! Utility traits.

use crate::constants::{Cardinality, ParameterName};
use crate::vcard::parameter::Parameter;
use crate::vcard::value::Value;
use crate::VcardError;

pub trait HasCardinality {
    fn cardinality(&self) -> &str;
    fn is_multiple(&self) -> bool {
        self.cardinality() == Cardinality::MULTIPLE
    }
    fn is_single(&self) -> bool {
        self.cardinality() == Cardinality::SINGLE
    }
}

pub trait HasGroup {
    fn group(&self) -> &Option<String>;
}

pub trait HasName {
    fn name(&self) -> &str;
}

pub trait HasParameters: HasName {
    fn allowed_parameters<'a>(&self) -> Vec<&'a str>;
    fn get_parameters(&self) -> Vec<Parameter>;
    fn set_parameters(&mut self, parameters: Vec<Parameter>);
    fn add_parameters(&mut self, parameters: Vec<Parameter>) -> Result<(), VcardError> {
        for parameter in parameters {
            self.add_parameter(parameter)?
        }
        Ok(())
    }
    fn add_parameter(&mut self, parameter: Parameter) -> Result<(), VcardError> {
        let mut parameters = self.get_parameters();

        if !self.allowed_parameters().contains(&parameter.name()) && !matches!(parameter, Parameter::ParameterXName(_)) && !self.allowed_parameters().contains(&ParameterName::ANY) {
            return Err(VcardError::ParameterTypeNotAllowed(parameter.name().to_string(), self.name().to_string()));
        }

        parameters.push(parameter);
        self.set_parameters(parameters);

        Ok(())
    }
    fn remove_parameter(&mut self, index: usize) -> Result<(), VcardError> {
        let mut parameters = self.get_parameters();

        parameters.remove(index);
        self.set_parameters(parameters);

        Ok(())
    }
    fn has_value_type(&mut self) -> Option<String> {
        self.get_parameters().iter().cloned().find(|p| p.name() == ParameterName::VALUE).map(|parameter| parameter.get_value().to_string())
    }
}

pub trait HasValue {
    fn get_value(&self) -> &Value;
    fn set_value(&mut self, value: Value) -> Result<(), VcardError>;
}
