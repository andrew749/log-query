use crate::output::output_profile::OutputProfile;
use std::collections::HashMap;
use handlebars::Handlebars;
use simple_error::{try_with, SimpleError};

pub trait OutputGenerator {
    fn template(&self, args: &HashMap<String, String>) -> String;
}

pub struct  HandlebarsOutputGenerator<'a> {
    registry: Handlebars<'a>,
}

impl<'a> HandlebarsOutputGenerator<'a> {
    pub fn new(template: OutputProfile) -> Result<HandlebarsOutputGenerator<'a>, SimpleError> {
        let mut registry = Handlebars::new();
        try_with!(registry.register_template_string("default", &template.output_template), "Unable to register template");
        Ok(HandlebarsOutputGenerator {
            registry,
        })
    }
}

impl<'a> OutputGenerator for HandlebarsOutputGenerator<'a> {
    fn template(&self, args: &HashMap<String, String>) -> String {
        self.registry.render("default", args).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_handlebars_templating() -> Result<(), SimpleError> {
        let profile = OutputProfile::from_str("{{test}}");
        let output_generator = HandlebarsOutputGenerator::new(profile)?;
        assert_eq!(output_generator.template(&vec![(String::from("test"), String::from("test value"))].into_iter().collect()), "test value");
        Ok(())
    }
}