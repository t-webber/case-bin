use std::io::{self, BufRead as _};
mod case;
use clap::{CommandFactory as _, Parser};

use crate::case::{to_camel, to_pascal};

/// Convert the case of the output. Choose one of the options below:
#[derive(Parser, Debug)]
struct Args {
    /// If no value is provided, reads from stdin (e.g. for pipes).
    value: Option<String>,
    /// thisIsCamelCase
    #[arg(short, long)]
    camel: bool,
    /// ThisIsPascalCase
    #[arg(short, long)]
    pascal: bool,
    /// this_is_snake_case
    #[arg(short, long)]
    snake: bool,
    /// THIS_IS_UPPER_CASE (or CONSTANT_CASE)
    #[arg(short, long)]
    upper: bool,
    /// this-is-kebab-case (or dashed-case)
    #[arg(short, long)]
    kebab: bool,
    /// This Is Capital Case
    #[arg(short = 'a', long)]
    capitalise: bool,
    /// This is sentence case
    #[arg(short = 'e', long)]
    sentence: bool,
    /// this.is.dot.case
    #[arg(short, long)]
    dot: bool,
}

impl Args {
    fn apply_case(&self, value: &str) -> String {
        if self.camel {
            return to_camel(value);
        }
        if self.pascal {
            return to_pascal(value);
        }
        //         if self.snake {}
        //         if self.kebab {}
        //         if self.capitalise {}
        panic("No output case provided.")
    }

    fn run(&self) -> Result<(), io::Error> {
        let nb = [
            self.camel,
            self.pascal,
            self.snake,
            self.kebab,
            self.capitalise,
        ]
        .iter()
        .filter(|x| **x)
        .count();

        if nb >= 2 {
            panic("You must provide 1 and 1 only output case.")
        } else if nb == 0 {
            panic("No output case provided. Please choose the option for your output.")
        }

        if let Some(value) = &self.value {
            println!("{}", self.apply_case(value));
        } else {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                println!("{}", self.apply_case(&line?));
            }
        }

        Ok(())
    }
}

fn panic(msg: &str) -> ! {
    Args::command().print_help().unwrap();
    panic!("\x1b[31m{msg}\x1b[0m");
}

fn main() {
    if let Err(err) = Args::parse().run() {
        panic(&format!("Failed to read input: pipe broken: {err}.\n"));
    }
}
