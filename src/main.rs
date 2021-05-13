use std::io;
use std::io::Write;

use git2::{BranchType, Repository};

use handlers::user;

use crate::models::data::{Commands, HELP, Result};

mod errors;
mod handlers;
mod models;
mod handlers_test;

fn main() {
    let input = io::stdin();
    let output = io::stdout();
    let repo = Repository::open_from_env().unwrap();
    let mut handle_out = output.lock();

    let result = (|| -> Result<_> {
        writeln!(handle_out, "Type 'help' or 'h' to find all commands")?;

        loop {
            let action = user::handle_user_input(&input, &repo, &mut handle_out);

            match action {
                Commands::Quit() => {
                    break;
                }
                Commands::Delete(name) => {
                    let branches = user::get_all_branches(&repo);
                    let branch = user::get_branch_by_name(branches, name);
                    let mut branch = match branch {
                        Ok(branch) => { branch }
                        Err(err) => {
                            writeln!(handle_out, "{}", err)?;
                            handle_out.flush()?;
                            continue;
                        }
                    };

                    let result = match user::delete_branch(&mut branch) {
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
                    let branches = user::get_branches(
                        &repo, BranchType::Local);
                    user::view_branches(branches, &mut handle_out)
                }
                Commands::Remote() => {
                    let branches = user::get_branches(
                        &repo, BranchType::Remote);
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



