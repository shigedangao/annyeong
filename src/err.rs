use core::fmt;
use std::error::Error;


#[derive(Debug)]
pub enum AnnyeongError {
    KubeError(String),
    CommandNotFound
}

impl fmt::Display for AnnyeongError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnnyeongError::CommandNotFound => write!(f, "Command not found"),
            AnnyeongError::KubeError(value) => write!(f, "Error with kubernetes {}", value)
        }
    }
}

impl From<kube::Error> for AnnyeongError {
    fn from(err: kube::Error) -> Self {
        AnnyeongError::KubeError(err.to_string())
    }
}

impl Error for AnnyeongError {}
