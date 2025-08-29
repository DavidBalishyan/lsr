use std::fs;
use std::path::Path;
use colored::Colorize;

fn main() -> std::io::Result<()> {
    let path = Path::new("./");
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let name = entry.file_name();

        if file_type.is_dir() {
            println!("{} {}", "[DIR]".green(), name.to_string_lossy());
        } else {
            println!("{} {}", "[FILE]".yellow(), name.to_string_lossy());
        }
    }

    Ok(())
}