pub mod git {
    use std::io::{BufRead, Stdin, StdoutLock, Write};

    use chrono::{Duration, NaiveDateTime};
    use git2::{BranchType, Error, Repository};

    use crate::errors::term_errors::Errors;
    use crate::models::data::{Branch, Result};

    pub fn get_repo(input: &Stdin, handle_out: &mut StdoutLock) -> Result<Repository> {
        write!(handle_out, "Input a repo path > ")?;
        handle_out.flush().unwrap();

        let req = input.lock().lines().next();
        let line = match req {
            Some(line) => line,
            None => {
                return get_repo(input, handle_out);
            }
        };

        let result = match line {
            Ok(path) => {
                let repo = match Repository::open(path) {
                    Ok(repo) => {
                        repo
                    }
                    Err(err) => {
                        writeln!(handle_out, "{}", err)?;
                        get_repo(input, handle_out)?
                    }
                };
                repo
            }
            Err(err) => {
                writeln!(handle_out, "{}", err)?;
                get_repo(input, handle_out)?
            }
        };
        Ok(result)
    }

    pub fn get_branches_by_type(repo: &Repository, br_type: BranchType) -> Result<Vec<Branch>> {
        let mut branches: Vec<Branch> = Vec::new();

        for item in repo.branches(Some(br_type))? {
            let (branch, _) = item?;
            let name = branch.name()?.unwrap().to_string();
            let time = get_time_from_branch(&branch)?;
            branches.push(Branch::new(name, time, branch));
        }

        branches.sort_by_key(|branch| branch.time);
        Ok(branches)
    }

    pub fn get_all_branches(repo: &Repository) -> Result<Vec<Branch>> {
        let local = get_branches_by_type(repo, BranchType::Local)?;
        let remote = get_branches_by_type(repo, BranchType::Remote)?;
        let mut result: Vec<Branch> = Vec::new();

        result.extend(local);
        result.extend(remote);
        result.sort_by_key(|branch| branch.time);
        Ok(result)
    }

    pub fn get_branch_by_name(repo: &Repository, name: String) -> Result<Branch> {
        match repo.find_branch(name.as_str(), BranchType::Local) {
            Ok(branch) => {
                let time = get_time_from_branch(&branch)?;
                Ok(Branch {
                    name,
                    time,
                    branch,
                })
            }
            Err(f_err) => {
                match repo.find_branch(name.as_str(), BranchType::Remote) {
                    Ok(branch) => {
                        let time = get_time_from_branch(&branch)?;
                        Ok(Branch {
                            name,
                            time,
                            branch,
                        })
                    }
                    Err(s_err) => {
                        Err(Errors::BranchNotFound(format!(
                            "\n{}\n{}", f_err.message(), s_err.message()).to_string()))
                    }
                }
            }
        }
    }

    fn get_time_from_branch(branch: &git2::Branch) -> Result<NaiveDateTime> {
        let commit = branch.get().peel_to_commit()?;
        let time = commit.time();
        let offset = Duration::minutes(i64::from(time.offset_minutes()));
        let time = NaiveDateTime::from_timestamp(
            commit.time().seconds(), 0) + offset;
        Ok(time)
    }

    pub fn delete_branch(branch: &mut Branch) -> Result<(), Error> {
        branch.branch.delete()
    }
}

pub mod user {
    use std::convert::TryFrom;
    use std::io::{BufRead, Stdin, StdoutLock, Write};

    use git2::Repository;

    use crate::models::data::{Branch, Commands};

    pub fn handle_user_input(
        input: &Stdin,
        repo: &Repository,
        handle_out: &mut StdoutLock) -> Commands {
        write!(handle_out, "Type a command > ").unwrap();
        handle_out.flush().unwrap();

        let req = input.lock().lines().next();
        let line = match req {
            Some(line) => line.unwrap(),
            None => {
                return handle_user_input(input, repo, handle_out);
            }
        };
        let args = line.split(' ').collect::<Vec<&str>>();
        let command = Commands::try_from(args);
        match command {
            Ok(result) => result,
            Err(err) => {
                writeln!(handle_out, "{}", err).unwrap();
                handle_user_input(input, repo, handle_out)
            }
        }
    }

    pub fn view_branches(branches: Vec<Branch>, handle_out: &mut StdoutLock) {
        for item in branches {
            writeln!(handle_out, "{}", item).unwrap();
            handle_out.flush().unwrap();
        }
    }
}