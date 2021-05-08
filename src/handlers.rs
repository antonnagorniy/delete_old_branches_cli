pub mod git {
    use git2::{BranchType, Repository};

    pub fn handle_branches(br_type: BranchType) -> Vec<String> {
        let mut branches: Vec<String> = Vec::new();
        let repo = Repository::open_from_env().unwrap();

        for item in repo.branches(Some(br_type)).unwrap() {
            let (branch, _) = item.unwrap();
            let name = branch.name().unwrap().unwrap();
            branches.push(name.to_string());
        }

        return branches;
    }
}