use std::path::PathBuf;
use std::{env, fs, io};
use std::io::ErrorKind;

fn main() -> io::Result<()> {
    let path = get_dir_path()?;
    let entries = get_dir_contents(path)?;

    for entry in entries.iter() {
        println!("{}", entry.display());
    }

    Ok(())
}

fn get_dir_path() -> Result<PathBuf, io::Error> {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let path = match args.last() {
        Some(p) => { PathBuf::from(p) }
        None => { env::current_dir()? }
    };

    if path.is_dir() == false {
        return Err(io::Error::from(ErrorKind::InvalidInput));
    }

    return Ok(path);
}

fn get_dir_contents(dir: PathBuf) -> Result<Vec<PathBuf>, io::Error> {
    let mut entries = fs::read_dir(dir)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    entries.sort();

    return Ok(entries);
}
