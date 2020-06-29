use serde::{export::Formatter, Deserialize, Serialize};
use warp::reject::Reject;

pub type Result<T> = std::result::Result<T, Error>;
// pub type AsyncResult<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Debug)]
pub enum Error {
    // system
    EnvVarError,
    ParseListeningAddressFailed,
    NotAuthed,
    SledGenIdFailed,
    SledSaveFailed,
    SledDbError,
    SqliteDbError,
    SerdeError,

    // business
    LoginFailed,
    SaveBlogFailed,
    CannotFoundBlog,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub message: String,
}

// impl std::fmt::Display for ErrResponse {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         unimplemented!()
//     }
// }

// impl std::error::Error for ErrResponse {}

impl warp::reject::Reject for Error {}

// impl From<std::io::Error> for ErrResponse {
//     fn from(e: std::io::Error) -> Self {
//         unimplemented!()
//     }
// }

impl From<std::env::VarError> for Error {
    fn from(e: std::env::VarError) -> Self {
        eprintln!("{}", e);
        Error::EnvVarError
    }
}

impl From<std::net::AddrParseError> for Error {
    fn from(e: std::net::AddrParseError) -> Self {
        eprintln!("{}", e);
        Error::ParseListeningAddressFailed
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        eprintln!("{}", e);
        Error::SerdeError
    }
}

impl From<sled::Error> for Error {
    fn from(e: sled::Error) -> Self {
        eprintln!("{}", e);
        Error::SledDbError
    }
}

impl From<sqlx::Error> for Error {
    fn from(e: sqlx::Error) -> Self {
        eprintln!("{}", e);
        Error::SqliteDbError
    }
}

// impl ErrResponse {
//     pub fn new(message: &str) -> Self {
//         ErrResponse {
//             message: String::from(message)
//         }
//     }
// }
