use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use clap::Parser;
use owo_colors::OwoColorize;
use tabled::Tabled;

#[derive(Parser, Debug)]
#[command(version)]
struct CLI {
    path: Option<PathBuf>,
}

#[derive(Debug)]
enum EntryType {
    File,
    Directory,
}

#[derive(Debug, Tabled)]
struct FileEntry {
    name : String,
    e_type : EntryType,
    len_byte: u64,
    modified: String,
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
            println!("{:?}", file);
        }
    } else {
        eprintln!("{}", "Error reading path.".red());
    }

    println!("Hello, world!");
}

fn get_files(path: &Path) -> Vec<FileEntry> {
    let mut files = Vec::default();
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            // TODO what can I do with error?
            if let Ok(entry) = entry {
                add_file(entry, &mut files);
            }
        }
    }

    files
}

fn add_file(file_entry: DirEntry, files: &mut Vec<FileEntry>) {
    if let Ok(metadata) = fs::metadata(&file_entry.path()) {
        files.push(FileEntry{
            name: file_entry.file_name().into_string()
                .unwrap_or("unknown file".to_string()),
            e_type: if metadata.is_dir() {
                EntryType::Directory
            } else {
                EntryType::File
            },
            len_byte: metadata.len(),
            modified: "".to_string(),
        })
    }

}
