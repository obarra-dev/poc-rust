use chrono::{DateTime, Utc};
use clap::Parser;
use owo_colors::OwoColorize;
use serde::Serialize;
use std::fs;
use std::fs::DirEntry;
use std::path::{Path, PathBuf};
use strum_macros::Display;
use tabled::settings::object::{Columns, Rows};
use tabled::settings::{Color, Style};
use tabled::{Table, Tabled};

#[derive(Parser)]
#[command(version)]
struct CLI {
    path: Option<PathBuf>,
    #[arg(short, long)]
    json: bool,
}

#[derive(Debug, Display, Serialize)]
enum EntryType {
    File,
    Directory,
}

#[derive(Debug, Tabled, Serialize)]
struct FileEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Type")]
    e_type: EntryType,
    #[tabled(rename = "Size B")]
    len_byte: u64,
    modified: String,
}

fn main() {
    let cli = CLI::parse();
    // lazy evaluation, unwrap_or is not
    let path = cli.path.unwrap_or_else(|| PathBuf::from("."));
    match fs::exists(&path) {
        Ok(true) => {
            println!("{} exists", path.display());
            let files = get_files(&path);
            if cli.json {
                print_as_json(files);
            } else {
                print_as_table(files);
            }
        }
        Ok(false) => {
            eprintln!("{}", "Path does not exist.".red());
            // TODO 1 -1?/
            std::process::exit(1);
        }
        Err(_) => eprintln!("{}", "Error reading path.".red()),
    }
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
        files.push(FileEntry {
            name: file_entry
                .file_name()
                .into_string()
                .unwrap_or("unknown file".to_string()),
            e_type: if metadata.is_dir() {
                EntryType::Directory
            } else {
                EntryType::File
            },
            len_byte: metadata.len(),
            modified: if let Ok(modified) = metadata.modified() {
                let datetime: DateTime<Utc> = modified.into();
                format!("{}", datetime.format("%Y-%m-%d %H:%M"))
            } else {
                String::default()
            },
        })
    }
}

fn print_as_json(files: Vec<FileEntry>) {
    let file_json = serde_json::to_string(&files).unwrap_or("Cannot parse json".to_string());
    println!("{}", file_json);
}

fn print_as_table(files: Vec<FileEntry>) {
    let mut table = Table::new(files);
    table.with(Style::rounded());
    table.modify(Columns::first(), Color::FG_RED);
    table.modify(Columns::one(3), Color::FG_BRIGHT_MAGENTA);
    table.modify(Rows::first(), Color::FG_BRIGHT_GREEN);
    println!("{}", table);
}