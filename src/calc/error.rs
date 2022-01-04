use std::{error, fmt};

#[derive(Debug)]
pub struct UnsupportedOperatorError {
    operator: String,
}

pub fn new(operator: String) -> UnsupportedOperatorError {
    UnsupportedOperatorError { operator }
}

impl fmt::Display for UnsupportedOperatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "unsupported operator {}", self.operator)
    }
}

impl error::Error for UnsupportedOperatorError {}

#[derive(Debug)]
pub struct FailedOperationError {
    pub reason: String,
}

impl fmt::Display for FailedOperationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to perform operation: {}", self.reason)
    }
}

impl error::Error for FailedOperationError {}
