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
    eprintln!("Starting");
    let args: Args = Args::from_args();
    eprintln!("Arguments provided: {:?}", args);
    let profile_path = args.profile.as_path().to_str().unwrap();
    eprintln!("Loading parser using profile {}", profile_path);
    let parser = load_parser_from_file(profile_path).unwrap();
    eprintln!("Loaded parser");

    let query = Query::new(&args.query);

    let mut parse_cache: Vec<Box<dyn LogLine>> = vec![];
    let file_path = args.file.as_path().to_str().unwrap();
    eprintln!("Processing file {}", file_path);
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    // TODO: replace with more efficient memory aware window
    for line in reader.lines() {
        if let Ok(line) = line { 
            if let Ok(result) = parser.parse(&line){
                parse_cache.push(Box::new(result));
            }
        }
    }
    let filtered = parse_cache.iter().filter(|log| process_query_on_log_line(&query, log.as_ref()));
    filtered.for_each(|x| println!("{:?}", x.get_content()));

    Ok(())
}
