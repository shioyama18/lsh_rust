use std::fmt;

#[derive(Debug)]
pub enum RshError {
    ExecutionError,
    CommandError(String),
    ForkError(String),
    ParentError(String),
    ChildError(String),
}

impl fmt::Display for RshError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RshError::ExecutionError      => write!(f, "ExecutionError"),
            RshError::CommandError(ref e) => write!(f, "CommandError: {}", e),
            RshError::ForkError(ref e)    => write!(f, "ForkError: {}", e),
            RshError::ParentError(ref e)  => write!(f, "ParentError: {}", e),
            RshError::ChildError(ref e)   => write!(f, "ChildError: {}", e),
        }
    }
}


