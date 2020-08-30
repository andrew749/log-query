//! Structure log files to something that can be understood and parsed 
 
use std::io::BufReader;
use std::io::{Error, prelude::*};
use std::path::PathBuf;
use std::fs::File;
use log_analyzer::*;
use log_analyzer::Parser;

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
    let args: Args = Args::from_args();
    let profile_path = args.profile.as_path().to_str().unwrap();
    let parser = load_parser_from_file(profile_path).unwrap();

    let query = Query::new(&args.query);

    let file_path = args.file.as_path().to_str().unwrap();
    let file = File::open(file_path)?;

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line { 
            if let Ok(result) = parser.parse(&line){
                if process_query_on_log_line(&query, result.as_ref()) {
                    println!("{:?}", result.get_content());
                }
            }
        }
    }

    Ok(())
}
