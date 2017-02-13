extern crate reqwest;
extern crate env_logger;

mod cmd_option;
mod error;

use cmd_option::CmdOpt;
use error::Error;

use std::env;
use std::str::FromStr;
use reqwest::Method;

struct Opt {
    url: Option<String>,
    method: Method,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            url: None,
            method: Method::Get,
        }
    }

    pub fn url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn is_valid(&self) -> bool {
        return self.url.is_some();
    }

    pub fn method(&mut self, method: Method) {
        self.method = method;
    }
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    env_logger::init().unwrap();

    let mut arguments = env::args();
    if arguments.len() < 2 {
        println!("creq: try 'creq --help' for more information");
        return;
    }
    arguments.next(); // execute name

    let mut opt = Opt::new();
    while let Some(arg) = arguments.next() {
        if arg.starts_with("--") {
            let option: Result<CmdOpt, Error> = FromStr::from_str(arg.as_str());
            match option {
                Ok(value) => {
                    match value {
                        CmdOpt::URL => {
                            opt.url(arguments.next().unwrap());
                        },
                        CmdOpt::REQUEST => {
                            let method = arguments.next().unwrap().to_uppercase();
                            opt.method(FromStr::from_str(method.as_str()).unwrap());
                        },
                        CmdOpt::HELP => {
                            help();
                            return;
                        },
                        CmdOpt::VERSION => {
                            println!("creq v{}", VERSION);
                            println!("{}", DESCRIPTION);
                            return;
                        } ,
                    }
                },
                Err(err) => {
                    println!("Invalid option: {}", err);
                    println!("creq: try 'creq --help' for more information");
                    return;
                }
            }
        } else {
            opt.url(arg);
        }

    }

    if !opt.is_valid() {
        help();
        return;
    }

    let client = reqwest::Client::new().unwrap();
    let res = client.request(opt.method, &opt.url.unwrap()).send().unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
}

fn help() {
    println!("Usage:");
    println!("  creq [options...] <url>");
    println!();
    println!("Options:");
    println!("  --help             Display this message");
    println!("  --request COMMAND  Specify HTTP method to use");
    println!("  --url URL          URL to work with");
    println!("  --version          Print version info and exit")
}
