mod directory;

use directory::explore_dir;
use std::{io::stdout, path::Path};

fn main() {
    let path = Path::new(".");
    let mut stdout = stdout();
    explore_dir(&mut stdout, path, 0);
}
