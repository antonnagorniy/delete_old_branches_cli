use std::io;
use std::io::{BufRead, Write};

use git2::{BranchType, Repository};

use errors::term_errors::Errors;
use handlers::git;

mod errors;
mod handlers;
mod models;

fn main() -> Result<(), Errors> {
    crossterm::terminal::enable_raw_mode()?;

    let input = io::stdin();
    let output = io::stdout();
    let repo = Repository::open_from_env().unwrap();

    loop {
        let mut handle_out = output.lock();
        write!(handle_out, "Type a command >")?;
        handle_out.flush()?;

        let line = input.lock().lines().next().unwrap()?;
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
            _ => {
                writeln!(handle_out, "Unknown command")?;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}

