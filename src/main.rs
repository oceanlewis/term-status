extern crate ansi_term;
use ansi_term::Style;

fn main() {
    println!("{}", Style::new().bold().paint("bold"));
}
