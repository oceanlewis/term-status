use ansi_term::Style;
use derive_more::From;
use git2::Repository;
use std::env::current_dir;

#[derive(From)]
enum Error {
    Git(git2::Error),
    IO(std::io::Error),
    Var(std::env::VarError),
    ParseInt(std::num::ParseIntError),
    NoShorthand,
}

fn dingus_level() -> Result<u32, Error> {
    let dingus_level = std::env::var("DINGUS_LEVEL")?;
    Ok(dingus_level.parse::<u32>()?)
}

fn git_branch() -> Result<String, Error> {
    let current_dir = current_dir()?;
    let repo = Repository::discover(current_dir)?;
    let head = repo.head()?;
    Ok(head.shorthand().ok_or(Error::NoShorthand)?.to_string())
}

fn style_one() {
    let level = dingus_level().unwrap_or(0);

    let prompt = match level {
        0 => " > ",
        1 => " λ ",
        _ => " λ. ",
    };

    match git_branch().ok() {
        Some(git_branch) => println!(
            "{branch}{prompt}",
            branch = Style::new().bold().paint(format!("[{}]", git_branch)),
            prompt = prompt
        ),
        None => println!("{}", prompt),
    }
}

fn main() {
    style_one();
}
