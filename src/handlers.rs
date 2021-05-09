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