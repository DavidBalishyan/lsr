mod utils;

use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::time::UNIX_EPOCH;

use colored::Colorize;
use chrono::{DateTime, Local};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "lsr", version, about = "A minimal ls replacement in Rust", disable_help_flag = true)]
pub struct Args {
    // Show all files including hidden
    #[arg(short = 'a')]
    all: bool,

    // Long listing format
    #[arg(short = 'l')]
    long: bool,

    // Human-readable sizes (with -l)
    #[arg(short = 'h', long = "human")]
    human: bool,

    // Show help
    #[arg(long = "help", action = clap::ArgAction::Help)]
    help: Option<bool>,

    // Reverse order
    #[arg(short = 'r')]
    reverse: bool,

    // Sort by size
    #[arg(short = 'S')]
    sort_size: bool,

    // Sort by modification time
    #[arg(short = 't')]
    sort_time: bool,

    // Path to list
    path: Option<String>,
}


fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let path = Path::new(args.path.as_deref().unwrap_or("./"));

    let mut entries: Vec<_> = fs::read_dir(path)?
        .filter_map(|e| e.ok())
        .collect();

    // Sorting
    if args.sort_size {
        entries.sort_by_key(|e| e.metadata().map(|m| m.len()).unwrap_or(0));
    } else if args.sort_time {
        entries.sort_by_key(|e| e.metadata().ok()
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0));
    } else {
        entries.sort_by_key(|e| e.file_name());
    }

    if args.reverse {
        entries.reverse();
    }

    for entry in entries {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();

        if !args.all && name_str.starts_with('.') {
            continue; // skip hidden
        }

        let metadata = entry.metadata()?;
        let file_type = metadata.file_type();

        if args.long {
            let permissions = utils::mode_to_string(metadata.mode(), file_type.is_dir());
            let size = if args.human {
                utils::human_size(metadata.len())
            } else {
                metadata.len().to_string()
            };
            let modified: DateTime<Local> = metadata.modified()?.into();
            let time_str = modified.format("%b %d %H:%M").to_string();

            if file_type.is_dir() {
                println!(
                    "d{} {:>5} {} {}",
                    permissions.green(),
                    size,
                    time_str,
                    name_str.blue()
                );
            } else {
                println!(
                    "-{} {:>5} {} {}",
                    permissions.yellow(),
                    size,
                    time_str,
                    name_str
                );
            }
        } else {
            if file_type.is_dir() {
                println!("{}", name_str.blue());
            } else {
                println!("{}", name_str);
            }
        }
    }

    Ok(())
}
