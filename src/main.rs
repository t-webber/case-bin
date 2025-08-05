use std::io::{self, BufRead as _};

use case::CaseExt;
use clap::{CommandFactory as _, Parser};

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
    /// THIS_IS_UPPER_CASE (aka. constant-case)
    #[arg(short, long)]
    upper: bool,
    /// this-is-kebab-case (aka. dashed-case)
    #[arg(short, long)]
    kebab: bool,
    /// Adds a capital to the beginning of the sentence.
    #[arg(short = 'a', long)]
    capitalise: bool,
}

impl Args {
    fn apply_case(&self, value: &str) -> String {
        if self.camel {
            return value.to_camel_lowercase();
        }
        if self.pascal {
            return value.to_camel();
        }
        if self.snake {
            return value.to_snake();
        }
        if self.kebab {
            return value.to_dashed();
        }
        if self.capitalise {
            return value.to_capitalized();
        }
        panic("No output case provided.")
    }

    fn run(&self) -> Result<(), io::Error> {
        if [
            self.camel,
            self.pascal,
            self.snake,
            self.kebab,
            self.capitalise,
        ]
        .iter()
        .filter(|x| **x)
        .count()
            >= 2
        {
            panic("You must provide 1 output case")
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
