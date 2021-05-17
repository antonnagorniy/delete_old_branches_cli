#[cfg(test)]
pub mod handlers_test {
    use git2::{BranchType, Repository};

    use crate::handlers::git;

    #[test]
    fn get_branches_by_type_should_return_local_branches() {
        let repo = Repository::open_from_env().unwrap();
        let branches = git::get_branches_by_type(
            &repo, BranchType::Local);
        assert!(branches.is_ok())
    }

    #[test]
    fn get_branches_by_type_should_return_remote_branches() {
        let repo = Repository::open_from_env().unwrap();
        let branches = git::get_branches_by_type(
            &repo, BranchType::Remote);
        assert!(branches.is_ok())
    }

    #[test]
    fn get_branch_by_name_should_return_branch() {
        let repo = Repository::open_from_env().unwrap();
        let expected_name = String::from("develop");
        let branch = git::get_branch_by_name(
            &repo, expected_name.clone()).unwrap();
        assert_eq!(branch.name, expected_name)
    }

    #[test]
    fn get_branch_by_name_should_return_error() {
        let repo = Repository::open_from_env().unwrap();
        let expected_name = String::from("tests_");
        let result = git::get_branch_by_name(
            &repo, expected_name.clone());
        assert!(!result.is_ok())
    }

    #[test]
    fn get_all_branches_should_return_all_branches() {
        let repo = Repository::open_from_env().unwrap();
        let result = git::get_all_branches(&repo);
        assert!(result.is_ok())
    }
}