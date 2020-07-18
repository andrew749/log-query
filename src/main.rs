//! Structure log files to something that can be understood and parsed 
 
use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(name = "FILE", parse(from_os_str))]
    /// Files to process
    file: Vec<PathBuf>,
}

fn main() {
    let args: Args = Args::from_args();
    println!("Arguments provided: {:?}", args)
}
