use std::fs;
use simple_error::{SimpleError};

pub struct OutputProfile {
    pub output_template: String,
}

impl OutputProfile {
    pub fn from_str(template: &str) -> Self {
        OutputProfile{
            output_template: String::from(template),
        }
    }
}

/// Get a output profile, describing how the output should be formatted, from a file
pub fn load_output_profile_from_file(path: &str) -> Result<OutputProfile, SimpleError>  {
    let data = fs::read_to_string(path).expect("Unable to read file");
    let profile = OutputProfile::from_str(data.as_ref());
    Ok(profile) 
}
