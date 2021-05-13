#[cfg(test)]
pub mod handlers_test {
    use git2::{BranchType, Repository};

    use crate::handlers::git;


    #[test]
    fn handle_branches_should_return_local_branches() {
        let repo = Repository::open_from_env().unwrap();
        let mut branch_names: Vec<String> = Vec::new();
        let mut expected_branch_names = Vec::new();

        expected_branch_names.push("master".to_string());
        expected_branch_names.push("develop".to_string());

        let branches = git::handle_branches(&repo, BranchType::Local);

        for item in branches {
            branch_names.push(item.name)
        }

        for (index, item) in expected_branch_names.iter().enumerate() {
            assert_eq!(item.as_str(), expected_branch_names[index].as_str());
        }
    }

    #[test]
    fn handle_branches_should_return_remote_branches() {
        let repo = Repository::open_from_env().unwrap();
        let mut branch_names: Vec<String> = Vec::new();
        let mut expected_branch_names = Vec::new();

        expected_branch_names.push("origin/HEAD".to_string());
        expected_branch_names.push("origin/master".to_string());
        expected_branch_names.push("origin/develop".to_string());

        let branches = git::handle_branches(&repo, BranchType::Local);

        for item in branches {
            branch_names.push(item.name)
        }

        for (index, item) in expected_branch_names.iter().enumerate() {
            assert_eq!(item.as_str(), expected_branch_names[index].as_str());
        }
    }
}