pub mod git {
    use chrono::{Duration, NaiveDateTime};
    use git2::{BranchType, Repository};

    use crate::models::data::Branch;

    pub fn handle_branches(repo: &Repository, br_type: BranchType) -> Vec<Branch> {
        let mut branches: Vec<Branch> = Vec::new();

        for item in repo.branches(Some(br_type)).unwrap() {
            let (branch, _) = item.unwrap();
            let name = branch.name().unwrap().unwrap().to_string();
            let commit = branch.get().peel_to_commit().unwrap().clone();
            let time = commit.time();
            let offset = Duration::minutes(i64::from(time.offset_minutes()));
            let time = NaiveDateTime::from_timestamp(
                commit.time().seconds(), 0) + offset;
            branches.push(Branch::new(commit.id(), name, time, branch));
        }

        branches.sort_by_key(|branch| branch.time);
        branches
    }
}

pub mod user {
    use std::convert::TryFrom;
    use std::io::{BufRead, Stdin, StdoutLock, Write};

    use git2::{Repository, BranchType, Error};

    use crate::models::data::{Commands, Branch, Result};
    use crate::handlers::git;
    use crate::errors::term_errors::Errors;

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
        let args = line.split(" ").collect::<Vec<&str>>();
        let command = Commands::try_from(args);
        match command {
            Ok(result) => result,
            Err(err) => {
                writeln!(handle_out, "{}", err).unwrap();
                return handle_user_input(input, repo, handle_out);
            }
        }
    }

    pub fn view_branches(branches: Vec<Branch>, handle_out: &mut StdoutLock) {
        for item in branches {
            writeln!(handle_out, "{}", item).unwrap();
            handle_out.flush().unwrap();
        }
    }

    pub fn get_branches(repo: &Repository, br_type: BranchType) -> Vec<Branch> {
        git::handle_branches(&repo, br_type)
    }

    pub fn get_all_branches(repo: &Repository)-> Vec<Branch> {
        let local = get_branches(repo, BranchType::Local);
        let remote = get_branches(repo, BranchType::Remote);
        let mut result: Vec<Branch> = Vec::new();
        for item in local {
            result.push(item);
        };

        for item in remote {
            result.push(item);
        };
        result
    }

    pub fn get_branch_by_name(branches: Vec<Branch>, name: String) -> Result<Branch> {
        for item in branches {
            if item.name == name {
                return Ok(item);
            };
        }
        Err(Errors::BranchNotFound(name))
    }

    pub fn delete_branch(branch: &mut Branch) -> std::result::Result<(), Error> {
       branch.branch.delete()


        /*let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)*/
    }
}