pub mod data {
    use std::fmt;
    use std::fmt::Formatter;

    use chrono::NaiveDateTime;
    use git2::Oid;

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