use std::fmt;

#[derive(Debug)]
pub enum LshError {
    ExecutionError,
    CommandError(String),
    ForkError(String),
    ParentError(String),
    ChildError(String),
}

impl fmt::Display for LshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LshError::ExecutionError      => write!(f, "ExecutionError"),
            LshError::CommandError(ref e) => write!(f, "CommandError: {}", e),
            LshError::ForkError(ref e)    => write!(f, "ForkError: {}", e),
            LshError::ParentError(ref e)  => write!(f, "ParentError: {}", e),
            LshError::ChildError(ref e)   => write!(f, "ChildError: {}", e),
        }
    }
}


