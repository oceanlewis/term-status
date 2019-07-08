use term::{term_status, Status};

fn main() {
  let status_style = match std::env::args()
    .collect::<Vec<String>>()
    .get(1)
    .map(String::as_str)
  {
    Some("new") => Status::New,
    _ => Status::Classic,
  };

  println!("{}", term_status(status_style));
}

mod term {
  use ansi_term::{Color, Style};
  use derive_more::From;
  use std::{collections::VecDeque, env::current_dir, ffi::OsStr};

  #[derive(From)]
  pub enum Error {
    Git(git2::Error),
    IO(std::io::Error),
    Var(std::env::VarError),
    ParseInt(std::num::ParseIntError),
    NoShorthand,
  }

  pub enum Status {
    Classic,
    New,
  }
  impl Status {
    fn status(&self) -> Result<String, Error> {
      match self {
        Status::Classic => git_branch_01().map(|git_branch| {
          format!(
            "{branch}{prompt}",
            branch = git_branch,
            prompt = prompt_01()
          )
        }),
        Status::New => {
          let cwd = current_dir()?;
          let home_dir = dirs::home_dir().expect("home directory should exist");

          let directory_part =
            Style::new()
              .fg(Color::Purple)
              .paint(if cwd.starts_with(&home_dir) {
                let mut path_parts = cwd
                  .iter()
                  .skip(home_dir.iter().count())
                  .map(OsStr::to_string_lossy)
                  .map(String::from)
                  .collect::<VecDeque<String>>();
                path_parts.push_front("~".to_string());
                util::shortened_path(path_parts)
              } else {
                util::shortened_path(
                  cwd
                    .iter()
                    .map(OsStr::to_string_lossy)
                    .map(String::from)
                    .collect::<VecDeque<String>>(),
                )
              });
          let git_branch = git_branch_01().unwrap_or_default();

          match directory_part.len() + git_branch.len() {
            length if length <= 3 => Ok(format!(
              "{directory}{git}{prompt}",
              directory = directory_part,
              git = git_branch_01().unwrap_or_default(),
              prompt = prompt_01()
            )),
            _ => Ok(format!(
              "{directory}\n{git}{prompt}",
              directory = directory_part,
              git = git_branch,
              prompt = prompt_01()
            )),
          }
        }
      }
    }
  }

  pub fn term_status(style: Status) -> String {
    style.status().unwrap_or_else(|_| style_fallback())
  }

  fn prompt_01() -> String {
    match util::dingus_level().unwrap_or(0) {
      0 => " > ",
      1 => " λ ",
      _ => " λ. ",
    }
    .to_string()
  }

  fn style_fallback() -> String {
    prompt_01()
  }

  fn git_branch_01() -> Result<String, Error> {
    util::git_branch().map(|git_branch| {
      Style::new()
        .bold()
        .paint(format!("[{}]", git_branch))
        .to_string()
    })
  }

  mod util {
    use super::{current_dir, Error, VecDeque};
    use git2::Repository;

    pub fn dingus_level() -> Result<u32, Error> {
      let dingus_level = std::env::var("DINGUS_LEVEL")?;
      Ok(dingus_level.parse::<u32>()?)
    }

    pub fn git_branch() -> Result<String, Error> {
      let current_dir = current_dir()?;
      let repo = Repository::discover(current_dir)?;
      let head = repo.head()?;
      Ok(head.shorthand().ok_or(Error::NoShorthand)?.to_string())
    }

    pub fn shortened_path(parts: VecDeque<String>) -> String {
      let length = parts.len();
      parts
        .into_iter()
        .enumerate()
        .fold(String::new(), |acc, (index, part)| {
          if index == length - 1 || part.as_str() == "/" {
            format!("{}{}", acc, part)
          } else {
            format!("{}{}/", acc, part.get(..=0).unwrap_or("?"))
          }
        })
    }
  }
}
