use std::fs;
use std::fs::metadata;
use std::path::{Path, PathBuf};
use clap::Parser;
use owo_colors::OwoColorize;

#[derive(Parser, Debug)]
#[command(version)]
struct CLI {
    path: Option<PathBuf>,
}

fn main() {
    let cli = CLI::parse();
    // TODO the or is executed anyway? like java
    let path = cli.path.unwrap_or(PathBuf::from("."));
    if let Ok(exist) = fs::exists(&path) {
        if !exist {
            eprintln!("{}", "Path does not exist.".red());
            // TODO 1 -1?/
            std::process::exit(1);
        }

        println!("{} exists", path.display());
        for file in get_files(&path) {
            println!("{}", file);
        }
    } else {
        eprintln!("{}", "Error reading path.".red());
    }

    println!("Hello, world!");
}

fn get_files(path: &Path) -> Vec<String> {
    let mut files = Vec::default();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            // TODO what can I do with error?
            if let Ok(entry) = entry {
                files.push(entry.file_name()
                    .into_string()
                    .unwrap_or("unknown file".to_string()));
            }
        }
    }

    files
}
