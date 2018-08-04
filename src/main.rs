#![feature(rust_2018_preview)]

use ansi_term::{Colour, Style};
use git2::Repository;
use std::env::current_dir;

mod error;
use crate::error::*;

struct Repo {
    shorthand: String,
    files_changed: usize,
}

fn repo_stats() -> Result<Repo, Error> {
    let current_dir = current_dir()?;
    let repo = Repository::discover(current_dir)?;
    let head = repo.head()?;

    let shorthand = head.shorthand().ok_or(Error::NoShorthand)?.into();

    let files_changed = repo
        .diff_tree_to_index(None, None, None)?
        .stats()?
        .files_changed();

    Ok(Repo {
        shorthand: shorthand,
        files_changed: files_changed,
    })
}

static INACTIVE_PROMPT: &str = " > ";
static ACTIVE_PROMPT: &str = " => ";

fn main() {
    let prompt = match std::env::var("DINGUS_LEVEL") {
        Ok(_) => ACTIVE_PROMPT,
        Err(_) => INACTIVE_PROMPT,
    };

    match repo_stats() {
        Ok(stats) => {
            let style = if stats.files_changed == 0 {
                Style::new().bold()
            } else {
                Colour::Purple.bold()
            };

            println!(
                "{}{}",
                style.paint(format!("[{}]", stats.shorthand)),
                prompt
            );
        }
        Err(_) => println!("{}", prompt),
    }
}
