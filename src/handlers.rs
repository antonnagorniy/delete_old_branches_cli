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
            branches.push(Branch::new(commit.id(), name, time));
        }

        branches.sort_by_key(|branch| branch.time);
        branches
    }
}

pub mod user {
    use std::convert::TryFrom;
    use std::io::{BufRead, Stdin, StdoutLock, Write};

    use git2::Repository;

    use crate::models::data::Commands;

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

        let command = Commands::try_from(line);
        match command {
            Ok(result) => result,
            Err(err) => {
                writeln!(handle_out, "{}", err).unwrap();
                return handle_user_input(input, repo, handle_out);
            }
        }
    }
}