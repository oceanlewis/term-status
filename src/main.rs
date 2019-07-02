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

fn prompt_01() -> String {
    match dingus_level().unwrap_or(0) {
        0 => " > ",
        1 => " λ ",
        _ => " λ. ",
    }
    .to_string()
}

fn style_fallback() -> String {
    prompt_01()
}

fn classic() -> Result<String, Error> {
    git_branch().map(|git_branch| {
        format!(
            "{branch}{prompt}",
            branch = Style::new().bold().paint(format!("[{}]", git_branch)),
            prompt = prompt_01()
        )
    })
}

fn new() -> Result<String, Error> {
    let cwd = current_dir()?;
    let home_dir = dirs::home_dir().expect("home directory should exist");


    if cwd.starts_with(&home_dir) {
        let until_home = cwd.iter().fold(0, |home_is_x_directories_deep, current_path_part| {
            
            unimplemented!()
        });

        cwd.iter().skip(until_home);
    }


    unimplemented!()
}

enum Status {
    Classic,
    New,
}

fn term_status(style: Status) -> Result<String, Error> {
    match style {
        Classic => classic(),
        New => new(),
    }
}

fn main() {
    println!(
        "{}",
        match term_status(Status::Classic) {
            Ok(status) => status,
            Err(_) => style_fallback(),
        }
    );
}
