//! Structure log files to something that can be understood and parsed 
 
use std::io::BufReader;
use std::io::{Error, prelude::*};
use std::path::PathBuf;
use std::fs::File;
use log_query::*;
use log_query::Parser;
use log_query::OutputGenerator;
use log_query::JSONOutputGenerator;

use structopt::{StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(name="log-query", about="Parse log files")]
struct Args {
    
    /// Parser profile file to look for and load from disk
    #[structopt(short = "p", long = "parser_profile_path")]
    parser_profile_path: PathBuf,

    /// Handlebar template to look for and load from disk
    #[structopt(short = "h", long = "handlebars", conflicts_with="json")]
    handlebars_template: Option<PathBuf>,

    /// Output in json
    #[structopt(long = "json", conflicts_with="handlebars", required_unless="handlebars")]
    json: bool,

    /// File to parse
    file: PathBuf,

    /// Query to run on the log lines 
    query: String,
}

fn main() -> Result<(), Error> {
    let args: Args = Args::from_args();
    let parser_profile_path = args.parser_profile_path.as_path().to_str().unwrap();
    let parser = load_parser_from_file(parser_profile_path).unwrap();
    let output_generator: Box<dyn OutputGenerator> = if let Some(handlebars_template) = args.handlebars_template {
        let output_profile_path = handlebars_template.as_path().to_str().unwrap();
        load_output_generator_from_file(output_profile_path).unwrap()
    } else if args.json {
        Box::new(JSONOutputGenerator::new())
    } else {
        panic!("No output format specified")
    };

    let query = Query::new(&args.query).unwrap();

    let file_path = args.file.as_path().to_str().unwrap();
    let file = File::open(file_path)?;

    // let output_generator = load_output_generator_from_file(path: &str)

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line { 
            if let Ok(result) = parser.parse(&line){
                if process_query_on_log_line(&query, result.as_ref()) {
                    println!("{}", output_generator.get_str(&*result));
                }
            }
        }
    }

    Ok(())
}
