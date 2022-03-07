use std::fmt;

// An error that occurred while trying to authorize the user
#[derive(Debug)]
pub enum AuthorizationError {
    JoinError(tokio::task::JoinError),
    ReqwestError(reqwest::Error),
    SerdeJsonError(serde_json::Error),
    SerdeQsError(serde_qs::Error),
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::JoinError(e) => e.to_string(),
                Self::ReqwestError(e) => e.to_string(),
                Self::SerdeJsonError(e) => e.to_string(),
                Self::SerdeQsError(e) => e.to_string(),
            }
        )
    }
}

impl std::error::Error for AuthorizationError {}

impl From<tokio::task::JoinError> for AuthorizationError {
    fn from(e: tokio::task::JoinError) -> Self {
        Self::JoinError(e)
    }
}

impl From<reqwest::Error> for AuthorizationError {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<serde_json::Error> for AuthorizationError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<serde_qs::Error> for AuthorizationError {
    fn from(e: serde_qs::Error) -> Self {
        Self::SerdeQsError(e)
    }
}

#[derive(Debug)]
pub struct InvalidArgumentError(pub String);

impl fmt::Display for InvalidArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid argument error: {}", self.0)
    }
}

impl std::error::Error for InvalidArgumentError {}

impl InvalidArgumentError {}

#[derive(Debug)]
pub struct BadArgumentError(pub String);

impl fmt::Display for BadArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad argument error: {}", self.0)
    }
}

impl std::error::Error for BadArgumentError {}

impl BadArgumentError {}

#[derive(Debug, Clone)]
pub enum UserFileError {
    InvalidOrAbsent,
    Expired,
}

impl fmt::Display for UserFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "user file error: {}",
            match self {
                &UserFileError::Expired => "the user file has expired",
                &UserFileError::InvalidOrAbsent => "the user file is invalid or absent",
            }
        )
    }
}

impl std::error::Error for UserFileError {}
