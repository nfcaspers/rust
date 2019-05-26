//implement own error-type

use std::error;
use std::fmt;

#[derive(Debug)]
enum BarError {
    Err1,
    Err2,
    Err3(BarErrorInfo),
}

impl fmt::Display for BarError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Debug)]
struct BarErrorInfo {
    info: String,
}

impl BarErrorInfo {
    fn new(info: String) -> BarErrorInfo {
        BarErrorInfo { info: info }
    }
}

impl error::Error for BarError {}
