use std::{io::Write, path::Path};
use termion::{color, style};

fn reset_style(output: &mut dyn Write) {
    write!(output, "{}", style::Reset).unwrap();
}

fn print_prefix(output: &mut dyn Write, deep_tier: usize, is_last_item: bool) {
    reset_style(output);
    if deep_tier > 0 {
        write!(
            output,
            "{}{}",
            "│   ".repeat(deep_tier - 1),
            if is_last_item {
                "└── "
            } else {
                "├── "
            }
        )
        .unwrap();
    }
}

fn print_os_entry(
    output: &mut dyn Write,
    file_name: &str,
    deep_tier: usize,
    str_fg_color: color::Fg<&dyn color::Color>,
    is_last_item: bool,
) {
    reset_style(output);
    print_prefix(output, deep_tier, is_last_item);
    writeln!(
        output,
        "{color}{name}",
        color = str_fg_color,
        name = file_name
            .split('/')
            .into_iter()
            .last()
            .unwrap_or("STUPID, SMTH WENT WRONG"),
    )
    .unwrap();
}

fn print_file_name(output: &mut dyn Write, file_name: &str, deep_tier: usize, is_last_item: bool) {
    print_os_entry(
        output,
        file_name,
        deep_tier,
        color::Fg(&color::White),
        is_last_item,
    );
}

fn print_dir_name(output: &mut dyn Write, file_name: &str, deep_tier: usize) {
    print_os_entry(output, file_name, deep_tier, color::Fg(&color::Blue), false);
}

fn get_dir_name(path: &Path) -> &str {
    path.as_os_str().to_str().unwrap_or("__error__")
}

pub fn explore_dir(output: &mut dyn Write, path: &Path, mut deep_tier: usize) {
    print_dir_name(output, get_dir_name(path), deep_tier);
    deep_tier += 1;
    let dir_items: Vec<_> = path.read_dir().expect("read_dir call failed").collect();
    for (idx, entry) in dir_items.iter().enumerate() {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            let file_name = file_name.to_str().unwrap_or("__error__");
            if let Ok(entry_type) = entry.file_type() {
                if entry_type.is_file() {
                    print_file_name(output, file_name, deep_tier, idx + 1 == dir_items.len());
                }
                if entry_type.is_dir() {
                    explore_dir(output, entry.path().as_path(), deep_tier);
                }
            } else {
                writeln!(output, "Couldn't get file type for {:?}", entry.path()).unwrap();
            }
        }
    }
}
