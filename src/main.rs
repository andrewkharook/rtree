use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

const PRFX: &str = "├──";
const PRFX_LAST: &str = "└──";

fn main() -> io::Result<()> {
    let dir = get_dir_path()?;
    let indent = "";

    iterate_dir(&dir, &indent)
}

// @TODO Fix prefix for the first item in a directory
fn iterate_dir(dir: &PathBuf, indent: &str) -> io::Result<()> {
    let entries = get_dir_contents(&dir)?;
    println!("{indent}{}", format_dir_name(dir.as_path()));

    for entry in entries.iter() {
        let mut prefix = PRFX;
        if entry == entries.last().unwrap() {
            prefix = PRFX_LAST;
        }

        if entry.is_dir() {
            let new_indent = String::from(indent) + "│  ";
            iterate_dir(&entry, &new_indent)?;
        }

        println!("{indent}{prefix}{}", format_dir_name(entry.as_path()));
    }

    Ok(())
}

fn get_dir_path() -> Result<PathBuf, io::Error> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let path = match args.last() {
        Some(p) => PathBuf::from(p),
        None => env::current_dir()?,
    };

    if !path.is_dir() {
        return Err(io::Error::from(ErrorKind::InvalidInput));
    }

    Ok(path)
}

fn get_dir_contents(dir: &PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    Ok(entries)
}

fn format_dir_name(dir: &Path) -> &str {
    return dir
        .components()
        .last()
        .unwrap()
        .as_os_str()
        .to_str()
        .unwrap();
}
