
use std::str::FromStr;

use error::Error;

use self::CmdOpt::{
    URL,
    REQUEST,
    HELP,
    VERSION,
};

#[derive(PartialEq, Eq, Debug)]
pub enum CmdOpt {
    /// --url
    URL,
    /// --request
    REQUEST,
    /// --help
    HELP,
    /// --version
    VERSION
}

impl FromStr for CmdOpt {
    type Err = Error;

    fn from_str(s: &str) -> Result<CmdOpt, Error> {
        match s {
            "--url" => Ok(URL),
            "--request" => Ok(REQUEST),
            "--help" => Ok(HELP),
            "--version" => Ok(VERSION),
            _ => Err(Error::CmdOpt(s.to_owned())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use error::Error;
    use super::CmdOpt;
    use super::CmdOpt::{URL};

    #[test]
    fn test_from_str() {
        assert_eq!(URL, FromStr::from_str("--url").unwrap());
        let x: Result<CmdOpt, _> = FromStr::from_str("--invalid");
        if let Err(Error::CmdOpt) = x {
        } else {
            panic!("An option is invalid!")
        }
    }
}
