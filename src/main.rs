//! Structure log files to something that can be understood and parsed 
 
use std::io::BufReader;
use std::io::{Error, prelude::*};
use std::path::PathBuf;
use std::fs::File;
use log_analyzer::*;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name="log-analyzer", about="Parse log files")]
struct Args {
    
    /// Profile file to look for and load from disk
    #[structopt(short = "p", long = "profile")]
    profile: PathBuf,

    /// File to parse
    file: PathBuf,

    /// Query to run on the log lines 
    query: String,
}

fn main() -> Result<(), Error> {
    println!("Starting");
    let args: Args = Args::from_args();
    println!("Arguments provided: {:?}", args);
    let profile_path = args.profile.as_path().to_str().unwrap();
    println!("Loading parser using profile {}", profile_path);
    let parser = load_parser_from_file(profile_path).unwrap();
    println!("Loaded parser");

    let mut parse_cache = vec![];
    let file_path = args.file.as_path().to_str().unwrap();
    println!("Processing file {}", file_path);
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = String::with_capacity(4096);
    while reader.read_line(&mut buffer)? > 0 {
        let parsed_result = parser.parse(&buffer);
        parse_cache.push(parsed_result);
    }
    Ok(())
}
