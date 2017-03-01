
use std::error::Error as StdError;
use std::fmt;

use self::Error::{
    Command,
};

#[derive(Debug)]
pub enum Error {
    /// An invalid `option`, such as `--qwer`
    Command(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ref e => f.write_str(e.description()),
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Command(ref e) => e.as_str(),
        }
    }
}

#[cfg(test)]
mod tests {

}
