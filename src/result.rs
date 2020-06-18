use sqlx::Error;
use serde::export::Formatter;

pub type Result<T> = std::result::Result<T, Err>;
// pub type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub struct Err {
    pub message: String,
}

impl std::fmt::Display for Err {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

impl std::error::Error for Err {}

impl From<std::io::Error> for Err {
    fn from(e: std::io::Error) -> Self {
        unimplemented!()
    }
}

impl From<std::env::VarError> for Err {
    fn from(e: std::env::VarError) -> Self {
        unimplemented!()
    }
}

impl From<serde_json::error::Error> for Err {
    fn from(e: serde_json::error::Error) -> Self {
        unimplemented!()
    }
}

impl From<sled::Error> for Err {
    fn from(e: sled::Error) -> Self {
        let m = match e {
            sled::Error::CollectionNotFound(v) => format!("The underlying collection no longer exists."),
            sled::Error::Unsupported(s) => format!("The system has been used in an unsupported way {}.", s),
            sled::Error::ReportableBug(s) => format!("An unexpected bug has happened. Please open an issue on github! {}", s),
            sled::Error::Io(e) => format!("A read or write error has happened when interacting with the file system. {:?}", e),
            sled::Error::Corruption{at, bt} => format!("Corruption has been detected in the storage file."),
            // sled::Error::FailPoint => format!("a failpoint has been triggered for testing purposes"),
        };
        Err {
            message: m,
        }
    }
}

impl From<sqlx::Error> for Err {
    fn from(e: Error) -> Self {
        let m = match e {
            sqlx::Error::Io(e) => format!(""),
            _ => format!(""),
        };
        Err {
            message: m,
        }
    }
}

impl Err {
    pub fn new(message: &str) -> Self {
        Err {
            message: String::from(message)
        }
    }
}