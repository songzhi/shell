use serde::{Deserialize, Serialize};

/// A `ShellError` is a proximate error and a possible cause, which could have its own cause,
/// creating a cause chain.
#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Serialize, Deserialize, Hash)]
pub struct ShellError {
    error: ProximateShellError,
    cause: Option<Box<ShellError>>,
}

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Serialize, Deserialize, Hash)]
pub enum ProximateShellError {
    ParseError,
}

impl ProximateShellError {
    pub fn start(self) -> ShellError {
        ShellError {
            cause: None,
            error: self,
        }
    }
}
