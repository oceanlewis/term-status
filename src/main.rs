extern crate ansi_term;
extern crate git2;

use std::env::current_dir;

use ansi_term::Style;
use git2::Repository;

enum Error {
    GitError(git2::Error),
    IOError(std::io::Error),
}

impl From<git2::Error> for Error {
    fn from(error: git2::Error) -> Error {
        Error::GitError(error)
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Error {
        Error::IOError(error)
    }
}

type Result<T> = std::result::Result<T, Error>;

fn get_branch_name() -> Result<Option<String>> {
    let current_dir = current_dir()?;
    let repo = Repository::open(current_dir)?;
    let head = repo.head()?;
    match head.shorthand() {
        Some(shorthand) => Ok(Some(shorthand.to_owned())),
        None => Ok(None),
    }
}

fn main() {
    let prompt = " => ";

    match get_branch_name() {
        Ok(Some(name)) => println!(
            "{}{}",
            Style::new().bold().paint(format!("[{}]", name)),
            prompt
        ),
        _ => println!("{}", prompt),
    }
}
