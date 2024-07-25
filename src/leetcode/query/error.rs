use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum QueryError {
    Sqlx(sqlx::Error),
    Reqwest(reqwest::Error),
    Misc(String)
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
