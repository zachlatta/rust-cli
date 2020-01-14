use structopt::StructOpt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let file = File::open(&args.path).expect("could not find file");
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        let line = raw_line.expect("error reading line");

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
