pub mod term_errors {
    use std::io;

    #[derive(Debug, thiserror::Error)]
    pub enum Errors {
        #[error(transparent)]
        CrosstermError(#[from] crossterm::ErrorKind),

        #[error(transparent)]
        IoError(#[from] io::Error),

        #[error(transparent)]
        GitError(#[from] git2::Error),

        #[error("Invalid command: {0}")]
        InvalidInput(String),

        #[error("Empty argument for '{0}'")]
        EmptyCommandArg(String),

        #[error("Branch not found: {0}")]
        BranchNotFound(String),
    }
}