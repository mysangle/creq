extern crate reqwest;
extern crate env_logger;

use std::env;

pub struct Opt {
    url: Option<String>,
}

impl Opt {
    pub fn new() -> Opt {
        Opt {
            url: None,
        }
    }

    pub fn url(&mut self, url: String) {
        self.url = Some(url);
    }

    pub fn is_valid(&self) -> bool {
        return self.url.is_some();
    }
}

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
        match arg.as_str() {
            "--url" => {
                match arguments.next() {
                    Some(url) => {
                        opt.url(url);
                    },
                    None => {
                        println!("creq: option --url: requires parameter");
                        println!("creq: try 'creq --help' for more information");
                        return;
                    }
                }
            },
            "--help" => {
                help();
                return;
            },
            url => {
                opt.url(url.to_string());
            }
        }
    }

    if !opt.is_valid() {
        help();
        return;
    }
    
    let res = reqwest::get(&opt.url.unwrap()).unwrap();

    println!("Status: {}", res.status());
    println!("Headers:\n{}", res.headers());
}

fn help() {
    println!("Usage: creq [options...] <url>");
    println!("  --url <url>");
}
