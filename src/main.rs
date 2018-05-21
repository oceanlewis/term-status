extern crate ansi_term;
extern crate git2;

use std::env::current_dir;

use ansi_term::Style;
use git2::Repository;

enum Error {
    GitError(git2::Error),
    IOError(std::io::Error),
    NoShorthand,
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

#[inline]
fn active_dingus_session() -> bool {
    match std::env::var("DINGUS") {
        Ok(_) => true,
        Err(_) => false
    }
}

fn print_prompt(prompt: &str) -> Result<()> {
    let current_dir = current_dir()?;
    let repo = Repository::discover(current_dir)?;
    let head = repo.head()?;
    let shorthand = head.shorthand().ok_or(Error::NoShorthand)?;

    println!(
        "{}{}",
        Style::new().bold().paint(format!("[{}]", shorthand)),
        prompt
    );

    Ok(())
}

fn main() {
    let prompt =
        if active_dingus_session() { " =>> " }
        else { " => " };

    match print_prompt(&prompt) {
        Ok(_) => {}
        Err(_) => println!("{}", prompt),
    }
}
