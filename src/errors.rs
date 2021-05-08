pub mod term_errors {
    use std::io;

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum Error {
        #[error(transparent)]
        CrosstermError(#[from] crossterm::ErrorKind),

        #[error(transparent)]
        IoError(#[from] io::Error),

        #[error(transparent)]
        GitError(#[from] git2::Error),

    }
}