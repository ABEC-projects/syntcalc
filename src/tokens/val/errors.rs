use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error for math value
/// E. g. trying to sum numbers with different units
#[derive(Debug)]
pub struct ValComputeError{
    desc: String,
    err_type: ErrorType,
}

impl ValComputeError{
    pub fn new(desc: String, err_type: ErrorType) -> Self{
        ValComputeError{desc, err_type}
    }
}

impl Display for ValComputeError{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error type: {} , description: {}", self.err_type, self.desc)
    }
}
impl Error for ValComputeError{}

#[derive(Debug)]
pub enum ErrorType{
    DivisionByZero,
    IncompatibleUnits,
    Other,
}
impl Display for ErrorType{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
