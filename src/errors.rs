pub mod term_errors {
    use std::io;

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum Errors {
        #[error(transparent)]
        CrosstermError(#[from] crossterm::ErrorKind),

        #[error(transparent)]
        IoError(#[from] io::Error),

        #[error(transparent)]
        GitError(#[from] git2::Error),

    }
}