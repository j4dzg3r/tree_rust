use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short)]
    all: bool,

    #[arg(short)]
    directories: bool,

    #[arg(long)]
    gitignore: bool,
}

pub struct Config<'a> {
    pub all: bool,
    pub directories: bool,
    pub gitignore_patterns: Vec<&'a str>,
}

impl<'a> Config<'a> {
    pub fn parse_args() -> Self {
        let args = Cli::parse();
        if args.gitignore {
            if let Some(res) = Self::find_gitignore_file(Some(Path::new(".").into())) {
                Self::parse_gitignore(res.as_ref());
            }
        }
        Config {
            all: args.all,
            directories: args.directories,
            gitignore_patterns: vec![],
        }
    }

    fn parse_gitignore(file_path: &Path) {
        let file_content = {
            let mut file = File::open(file_path).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        };
        let mut patterns: Vec<String> = Vec::new();
        let mut word = String::new();
        for row in file_content.lines() {
            for char in row.chars() {
                match char {
                    '*' => word.push_str(".*"),
                    '#' => break,
                    _ => {}
                }
            }
        }
    }

    fn find_gitignore_file(path: Option<Box<Path>>) -> Option<Box<Path>> {
        let path = {
            match path {
                Some(path) => path,
                None => Path::new(".").into(),
            }
        };
        let list_dir: Vec<_> = path.read_dir().expect("something went wrong").collect();
        let result = {
            let mut dir_iter = list_dir.iter();
            loop {
                let entry = dir_iter.next();
                if let Some(entry) = entry {
                    if let Ok(entry) = entry {
                        if entry.file_name() == ".gitignore" {
                            break Some(entry.path().as_path().into());
                        } else {
                            break Self::find_gitignore_file(Some(
                                entry.path().parent().unwrap().into(),
                            ));
                        }
                    } else {
                        break None;
                    }
                } else {
                    break None;
                }
            }
        };
        result
    }
}
