use std::io;
use std::io::Write;

use git2::{BranchType};

use handlers::user;

use crate::models::data::{Commands, HELP, Result};
use crate::handlers::git;

mod errors;
mod handlers;
mod models;
mod handlers_test;

fn main() {
    let input = io::stdin();
    let output = io::stdout();
    let mut handle_out = output.lock();

    let result = (|| -> Result<_> {
        let repo = git::get_repo(&input, &mut handle_out)?;
        writeln!(handle_out, "Type 'help' or 'h' to find all commands")?;

        loop {
            let action = user::handle_user_input(&input, &repo, &mut handle_out);

            match action {
                Commands::All() => {
                    let all_branches = git::get_all_branches(&repo)?;
                    user::view_branches(all_branches, &mut handle_out)
                }
                Commands::Quit() => {
                    break;
                }
                Commands::Delete(name) => {
                    let result = git::get_branch_by_name(&repo, name);
                    let mut branch = match result {
                        Ok(branch) => { branch }
                        Err(err) => {
                            writeln!(handle_out, "{}", err)?;
                            handle_out.flush()?;
                            continue;
                        }
                    };

                    let result = match git::delete_branch(&mut branch) {
                        Ok(res) => {
                            writeln!(handle_out, "{} deleted", &branch.name)?;
                            handle_out.flush()?;
                            res
                        }
                        Err(err) => {
                            writeln!(handle_out, "{}", err)?;
                            handle_out.flush()?;
                            continue;
                        }
                    };
                    result
                }
                Commands::Local() => {
                    let branches = git::get_branches_by_type(
                        &repo, BranchType::Local)?;
                    user::view_branches(branches, &mut handle_out)
                }
                Commands::Remote() => {
                    let branches = git::get_branches_by_type(
                        &repo, BranchType::Remote)?;
                    user::view_branches(branches, &mut handle_out)
                }
                Commands::Help() => {
                    writeln!(handle_out, "{}", HELP)?;
                    handle_out.flush()?;
                }
                Commands::Check(name) => {
                    let branch = git::get_branch_by_name(&repo, name)?;
                    writeln!(handle_out, "{}", branch)?;
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