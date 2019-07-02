extern crate ansi_term;
extern crate git2;

use std::env::current_dir;

use ansi_term::Style;
use git2::Repository;

enum Error {
    Git(git2::Error),
    IO(std::io::Error),
    Var(std::env::VarError),
    ParseInt(std::num::ParseIntError),
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

impl From<std::env::VarError> for Error {
    fn from(error: std::env::VarError) -> Error {
        Error::VarError(error)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Error {
        Error::ParseIntError(error)
    }
}

fn dingus_level() -> Result<u32, Error> {
    let dingus_level = std::env::var("DINGUS_LEVEL")?;
    Ok(dingus_level.parse::<u32>()?)
}

fn print_prompt(prompt: &str) -> Result<(), Error> {
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
    let level = dingus_level().unwrap_or(0);

    let prompt = {
            if level == 0 {
                " > "
            } else if level == 1 {
                " λ "
            } else {
                " λ. "
            }
    };

    match print_prompt(&prompt) {
        Ok(_) => {}
        Err(_) => println!("{}", prompt),
    }
}
