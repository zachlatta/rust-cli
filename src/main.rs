use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    _pattern: String,

    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();

    let _content = std::fs::read_to_string(&args.path).expect("could not read file");
}
