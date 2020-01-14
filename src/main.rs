use exitfailure::ExitFailure;
use failure::ResultExt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();

    let file = File::open(&args.path)
        .with_context(|_| format!("could not read file `{}`", args.path.display()))?;
    let reader = BufReader::new(file);

    for raw_line in reader.lines() {
        let line = raw_line.with_context(|_| format!("could not read line"))?;

        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
