use std::convert::TryFrom;
use std::io;
use std::io::{BufRead, Write, StdoutLock, Stdin};

use git2::{BranchType, Repository};

use errors::term_errors::Errors;
use handlers::git;

use crate::models::data::{Commands, HELP};

mod errors;
mod handlers;
mod models;

fn main() {
    (|| {
        let mut input = io::stdin();
        let output = io::stdout();
        let repo = Repository::open_from_env().unwrap();
        let mut handle_out = output.lock();

        writeln!(handle_out, "Type 'help' or 'h' to find all commands")?;

        loop {
            let action = match get_action_from_user(&mut input, &repo, &mut handle_out) {
                Ok(action) => action,
                Err(_) => {
                    get_action_from_user(&mut input, &repo, &mut handle_out)?
                }
            };

            match action {
                Commands::Quit() => {
                    break;
                }
                Commands::Delete() => {}
                Commands::Local() => {
                    let branches = git::handle_branches(&repo, BranchType::Local);
                    for item in branches {
                        writeln!(handle_out, "{}", item)?;
                        handle_out.flush()?;
                    }
                }
                Commands::Remote() => {
                    let branches = git::handle_branches(&repo, BranchType::Remote);
                    for item in branches {
                        writeln!(handle_out, "{}", item)?;
                        handle_out.flush()?;
                    }
                }
                Commands::Help() => {
                    writeln!(handle_out, "{}", HELP)?;
                    handle_out.flush()?;
                }
            }
        }
        Result::<_, Errors>::Ok(())
    })().unwrap();
}

fn get_action_from_user(
    input: &Stdin,
    repo: &Repository,
    handle_out: &mut StdoutLock) -> Result<Commands, Errors> {
    write!(handle_out, "Type a command > ")?;
    handle_out.flush()?;

    let req = input.lock().lines().next();
    let line = match req {
        Some(line) => line.unwrap(),
        None => {
            write!(handle_out, "Incorrect command: {}", req.unwrap()?)?;
            handle_out.flush()?;
            return get_action_from_user(input, repo, handle_out);
        }
    };

    Commands::try_from(line)
}
