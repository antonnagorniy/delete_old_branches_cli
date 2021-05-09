pub mod git {
    use chrono::NaiveDateTime;
    use git2::{BranchType, Repository};

    use crate::models::data::Branch;

    pub fn handle_branches(br_type: BranchType) -> Vec<Branch> {
        let repo = Repository::open_from_env().unwrap();
        let mut branches: Vec<Branch> = Vec::new();

        for item in repo.branches(Some(br_type)).unwrap() {
            let (branch, _) = item.unwrap();
            let name = branch.name().unwrap().unwrap().to_string();
            let commit = branch.get().peel_to_commit().unwrap().clone();
            let time = NaiveDateTime::from_timestamp(
                commit.time().seconds(), 0);

            branches.push(Branch::new(commit.id(), name, time));
        }

        branches
    }
}