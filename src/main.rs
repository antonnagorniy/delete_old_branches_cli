use std::io;
use std::io::Write;

use git2::{BranchType, Repository};

use errors::term_errors::Errors;
use handlers::git;
use handlers::user;

use crate::models::data::{Commands, HELP, Result};

mod errors;
mod handlers;
mod models;

fn main() {
    let mut input = io::stdin();
    let output = io::stdout();
    let repo = Repository::open_from_env().unwrap();
    let mut handle_out = output.lock();

    let result = (|| -> Result<_> {
        writeln!(handle_out, "Type 'help' or 'h' to find all commands")?;

        loop {
            let action = user::handle_user_input(&mut input, &repo, &mut handle_out);

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
            };
        };
        Ok(())
    })();

    match result {
        Ok(()) => {}
        Err(error) => {
            match error {
                Errors::InvalidInput(..) => {
                    writeln!(handle_out, "{}", error).unwrap();
                }
                _ => {
                    eprintln!("{}", error);
                    std::process::exit(1);
                }
            }
        }
    }
}


