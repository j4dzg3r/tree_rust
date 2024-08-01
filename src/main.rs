mod directory;
mod parser;

use directory::explore_dir;
use parser::Config;
use std::{io::stdout, path::Path};

fn main() {
    let args = Config::parse_args();
    let path = Path::new(".");
    let mut stdout = stdout();
    explore_dir(&mut stdout, path, 0, &args);
}
