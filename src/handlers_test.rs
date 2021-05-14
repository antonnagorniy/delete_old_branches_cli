#[cfg(test)]
pub mod handlers_test {
    use git2::{BranchType, Repository};

    use crate::handlers::git;

    #[test]
    fn get_branches_by_type_should_return_local_branches() {
        let repo = Repository::open_from_env().unwrap();
        let mut branch_names: Vec<String> = Vec::new();
        let mut expected_branch_names = Vec::new();

        expected_branch_names.push("master".to_string());
        expected_branch_names.push("develop".to_string());

        let branches = git::get_branches_by_type(&repo, BranchType::Local);

        for item in branches {
            branch_names.push(item.name)
        }

        for (index, item) in expected_branch_names.iter().enumerate() {
            assert_eq!(branch_names[index], item.clone());
        }
    }

    #[test]
    fn get_branches_by_type_should_return_remote_branches() {
        let repo = Repository::open_from_env().unwrap();
        let mut branch_names: Vec<String> = Vec::new();
        let mut expected_branch_names = Vec::new();

        expected_branch_names.push("origin/HEAD".to_string());
        expected_branch_names.push("origin/master".to_string());
        expected_branch_names.push("origin/develop".to_string());

        let branches = git::get_branches_by_type(&repo, BranchType::Remote);

        for item in branches {
            branch_names.push(item.name)
        }

        for (index, item) in expected_branch_names.iter().enumerate() {
            assert_eq!(branch_names[index], item.clone());
        }
    }

    #[test]
    fn get_branch_by_name_should_return_branch() {
        let repo = Repository::open_from_env().unwrap();
        let branches = git::get_all_branches(&repo);
        let expected_name = String::from("develop");
        let branch = git::get_branch_by_name(
            branches, expected_name.clone()).unwrap();

        assert_eq!(branch.name, expected_name)
    }

    #[test]
    fn get_all_branches_should_return_all_branches() {
        let repo = Repository::open_from_env().unwrap();
        let mut branch_names: Vec<String> = Vec::new();
        let mut expected_branch_names = Vec::new();

        expected_branch_names.push("master".to_string());
        expected_branch_names.push("develop".to_string());
        expected_branch_names.push("origin/HEAD".to_string());
        expected_branch_names.push("origin/master".to_string());
        expected_branch_names.push("origin/develop".to_string());


        let branches_local = git::get_branches_by_type(&repo, BranchType::Local);
        let branches_remote = git::get_branches_by_type(&repo, BranchType::Remote);

        for item in branches_local {
            branch_names.push(item.name)
        }
        for item in branches_remote {
            branch_names.push(item.name)
        }

        for (index, item) in expected_branch_names.iter().enumerate() {
            assert_eq!(branch_names[index], item.clone());
        }
    }
}