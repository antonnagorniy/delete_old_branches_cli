pub mod data {
    use std::convert::TryFrom;
    use std::fmt;
    use std::fmt::Formatter;

    use chrono::NaiveDateTime;
    use git2::Oid;

    use crate::errors::term_errors::Errors;

    pub const HELP: &str = r#"quit or q - Exit from app
local or l - Get local branches
remote or r - Get remote branches"#;

    #[derive(Debug, Clone)]
    pub struct Branch {
        id: Oid,
        name: String,
        pub time: NaiveDateTime,
    }

    impl Branch {
        pub fn new(id: Oid, name: String, time: NaiveDateTime) -> Branch {
            Branch {
                id,
                name,
                time,
            }
        }
    }

    impl fmt::Display for Branch {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} - Last commit {}", self.name, self.time)
        }
    }

    pub enum Commands {
        Quit(),
        Local(),
        Remote(),
        Help(),
        Delete(),
    }

    impl TryFrom<String> for Commands {
        type Error = Errors;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.to_lowercase().as_str() {
                "quit" | "q" => Ok(Commands::Quit()),
                "local" | "l" => Ok(Commands::Local()),
                "remote" | "r" => Ok(Commands::Remote()),
                "help" | "h" | "?" => Ok(Commands::Help()),
                "delete" | "d" => Ok(Commands::Delete()),
                _ => { Err(Errors::InvalidInput(value)) }
            }
        }
    }

    pub type Result<T, E = Errors> = std::result::Result<T, E>;
}