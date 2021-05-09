pub mod data {
    use std::fmt;
    use std::fmt::Formatter;

    use chrono::NaiveDateTime;
    use git2::Oid;

    pub const HELP: &str = "quit or q - Exit from app\nlocal or l - Get local branches\n\
                remote or r - Get remote branches";

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
}