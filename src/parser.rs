use clap::Parser;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short)]
    all: bool,

    #[arg(short)]
    directories: bool,
}

pub struct Config {
    pub all: bool,
    pub directories: bool,
}

impl Config {
    pub fn parse_args() -> Self {
        let args = Cli::parse();
        Config {
            all: args.all,
            directories: args.directories,
        }
    }
}
