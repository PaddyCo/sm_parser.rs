use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ChartParseError;

impl fmt::Display for ChartParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "something went wrong during chart parsing")
    }
}

impl error::Error for ChartParseError {
    fn description(&self) -> &str {
        "something went wrong during chart parsing"
    }

    fn cause(&self) -> Option<&error::Error> {
        // Generic error, underlying cause isn't tracked.
        None
    }
}
