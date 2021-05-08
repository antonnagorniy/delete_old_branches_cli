use std::io;
use std::io::{BufRead, Write};

use git2::{BranchType, Repository};

use errors::term_errors::Error;

mod errors;

fn main() -> Result<(), Error> {
    crossterm::terminal::enable_raw_mode()?;

    let input = io::stdin();
    let output = io::stdout();

    loop {
        let mut handle_out = output.lock();
        write!(handle_out, "Type a command >")?;
        handle_out.flush()?;

        let line = input.lock().lines().next().unwrap()?;
        match line.as_str() {
            "quit" => {
                break;
            }
            "local" => {
                let repo = Repository::open_from_env()?;
                for item in repo.branches(Some(BranchType::Local))? {
                    let (branch, _) = item?;
                    let name = branch.name().unwrap().unwrap();
                    write!(handle_out, "{}\n", name)?;
                }
            }
            "remote" => {
                let repo = Repository::open_from_env()?;
                for item in repo.branches(Some(BranchType::Remote))? {
                    let (branch, _) = item?;
                    let name = branch.name().unwrap().unwrap();
                    write!(handle_out, "{}\n", name)?;
                }
            }
            _ => {
                write!(handle_out, "{}\n", "Unknown command")?;
            }
        }
    }

    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
