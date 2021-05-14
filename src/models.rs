pub mod data {
    use std::convert::TryFrom;
    use std::fmt;
    use std::fmt::Formatter;

    use chrono::NaiveDateTime;

    use crate::errors::term_errors::Errors;

    pub const HELP: &str = r#"
        all or 'a' - Shows all branches
        quit or 'q' - Exits from app
        local or 'l' - Shows local branches
        remote or 'r' - Shows remote branches
        delete 'branch name' or 'd' 'branch name' - Deletes branch by specified name
"#;

    pub struct Branch<'repo> {
        pub name: String,
        pub time: NaiveDateTime,
        pub branch: git2::Branch<'repo>,
    }

    impl Branch<'_> {
        pub fn new(name: String, time: NaiveDateTime, branch: git2::Branch) -> Branch {
            Branch {
                name,
                time,
                branch,
            }
        }
    }

    impl fmt::Display for Branch<'_> {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{} - Last commit {}", self.name, self.time)
        }
    }

    pub enum Commands {
        All(),
        Quit(),
        Local(),
        Remote(),
        Help(),
        Delete(String),
    }

    impl TryFrom<Vec<&str>> for Commands {
        type Error = Errors;

        fn try_from(value: Vec<&str>) -> Result<Self, Self::Error> {
            match value[0].to_lowercase().as_str() {
                "all" | "a" => Ok(Commands::All()),
                "quit" | "q" => Ok(Commands::Quit()),
                "local" | "l" => Ok(Commands::Local()),
                "remote" | "r" => Ok(Commands::Remote()),
                "help" | "h" | "?" => Ok(Commands::Help()),
                "delete" | "d" => {
                    if value.len() == 1 {
                        Err(Errors::EmptyCommandArg(value[0].to_string()))
                    } else {
                        Ok(Commands::Delete(value[1].to_string()))
                    }
                }
                _ => {
                    Err(Errors::InvalidInput(value.join(" ")))
                }
            }
        }
    }

    pub type Result<T, E = Errors> = std::result::Result<T, E>;
}