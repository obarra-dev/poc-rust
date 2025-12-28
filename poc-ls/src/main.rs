use std::fs;
use std::path::PathBuf;
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
    } else {
        eprintln!("{}", "Error reading path.".red());
    }

    println!("Hello, world!");
}
