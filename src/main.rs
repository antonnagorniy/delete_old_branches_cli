use std::io;
use std::io::Write;

use git2::{BranchType, Repository};

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
                    let branches = user::get_branches(&repo, BranchType::Local);
                    user::view_branches(branches, &mut handle_out)
                }
                Commands::Remote() => {
                    let branches = user::get_branches(&repo, BranchType::Remote);
                    user::view_branches(branches, &mut handle_out)
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
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}



