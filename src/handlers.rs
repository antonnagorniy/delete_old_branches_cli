pub mod git {
    use std::collections::HashMap;

    use chrono::NaiveDateTime;
    use git2::{BranchType, Commit, Oid, Repository};

    pub fn handle_branches(br_type: BranchType) -> HashMap<String, NaiveDateTime> {
        let repo = Repository::open_from_env().unwrap();
        let mut branches: HashMap<String, NaiveDateTime> = HashMap::new();

        for item in repo.branches(Some(br_type)).unwrap() {
            let (branch, _) = item.unwrap();
            let name = branch.name().unwrap().unwrap().to_string();
            let commit = branch.get().peel_to_commit().unwrap().clone();
            let time = NaiveDateTime::from_timestamp(
                commit.time().seconds(), 0);
            branches.insert(name, time);
        }

        return branches;
    }
}