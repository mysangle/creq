extern crate futures;
extern crate hyper;
extern crate tokio_core;

extern crate env_logger;

mod cmd_option;
mod error;

use cmd_option::CmdOpt;
use error::Error;

use std::env;
use std::io::{self, Write};
use std::str::FromStr;

use futures::Future;
use futures::stream::Stream;

use hyper::Client;
use hyper::Method;
use hyper::client::Request;
use hyper::Uri;

struct Opt {
    uri: Option<Uri>,
    method: Method,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            uri: None,
            method: Method::Get,
        }
    }

    pub fn url(&mut self, url: String) {
        self.uri = url.parse::<hyper::Uri>().ok();
    }

    pub fn is_valid(&self) -> bool {
        self.uri.is_some()
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

    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);

    let req = Request::new(opt.method, opt.uri.unwrap());

    let work = client.request(req).and_then(|res| {
        println!("Status: {}", res.status());
        println!("Headers:\n{}", res.headers());

        res.body().for_each(|chunk| {
            io::stdout().write_all(&chunk).map_err(From::from)
        })
    }).map(|_| {
        println!("\n\nDone.");
    });

    core.run(work).unwrap();
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
