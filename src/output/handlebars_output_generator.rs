use std::fs;
use handlebars::Handlebars;
use crate::output::output_generator::OutputGenerator;
use crate::parser::log_line_parse_result::LogLineParseResult;
use simple_error::{try_with, SimpleError};

pub struct  HandlebarsOutputGenerator<'a> {
    registry: Handlebars<'a>,
}

impl<'a> HandlebarsOutputGenerator<'a> {
    pub fn new(template: &str) -> Result<HandlebarsOutputGenerator<'a>, SimpleError> {
        let mut registry = Handlebars::new();
        try_with!(registry.register_template_string("default", template), "Unable to register template");
        Ok(HandlebarsOutputGenerator {
            registry,
        })
    }
    pub fn from_file(path: &str) -> Result<Box<dyn OutputGenerator>, SimpleError>  {
        let data = fs::read_to_string(path).expect("Unable to read file");
        let generator = try_with!(HandlebarsOutputGenerator::new(data.as_ref()), "Unable to construct parser");
        Ok(Box::new(generator)) 
    }
}

impl<'a> OutputGenerator for HandlebarsOutputGenerator<'a> {
    fn get_str(&self, log_line: &dyn LogLineParseResult) -> String {
        self.registry.render("default", log_line.get_content()).unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::default_log_line_parse_result::DefaultLogLineParseResult;

    #[test]
    fn test_simple_handlebars_templating() -> Result<(), SimpleError> {
        let output_generator = HandlebarsOutputGenerator::new("{{test}}")?;
        let log_line = DefaultLogLineParseResult::new(vec![(String::from("test"), String::from("test value"))].into_iter().collect());
        assert_eq!(output_generator.get_str(&log_line), "test value");
        Ok(())
    }
}