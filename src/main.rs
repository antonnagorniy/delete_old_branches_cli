use std::io;
use std::io::{BufRead, Write};

use git2::{BranchType, Repository};

use errors::term_errors::Errors;
use handlers::git;

use crate::models::data;

mod errors;
mod handlers;
mod models;


fn main() -> Result<(), Errors> {
    let input = io::stdin();
    let output = io::stdout();
    let repo = Repository::open_from_env().unwrap();
    let mut handle_out = output.lock();

    writeln!(handle_out, "Type 'help' or 'h' to find all commands")?;

    loop {
        write!(handle_out, "Type a command > ")?;
        handle_out.flush()?;

        let line = input.lock().lines().next().unwrap()?.to_lowercase();
        match line.as_str() {
            "quit" | "q" => {
                break;
            }
            "local" | "l" => {
                let branches = git::handle_branches(&repo, BranchType::Local);
                for item in branches {
                    writeln!(handle_out, "{}", item)?;
                }
            }
            "remote" | "r" => {
                let branches = git::handle_branches(&repo, BranchType::Remote);
                for item in branches {
                    writeln!(handle_out, "{}", item)?;
                }
            }
            "help" | "h" => {
                writeln!(handle_out, "{}", data::HELP)?;
            }
            _ => {
                writeln!(handle_out, "Unknown command")?;
            }
        }
    }

    Ok(())
}

