use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

#[derive(Debug)]
struct MyError {
    msg:    String,
}

impl MyError {
    pub fn new(msg: String)->MyError {
        MyError {msg: msg}
    }
}

impl Error for MyError {
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result {
        write!(f, "Error: {}", self.msg)
    }
}

struct Parser {
}

impl Parser {
    pub fn new()->Parser {
        Parser {}
    }

    pub fn parse(&self, input: &'static str)->Result<(), MyError> {
        println!("parse: {}", input);
        return Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let input = read_to_string("infile".to_owned()).unwrap();
    fn1(&input);
}

pub fn fn1(input: &str)->Result<(), MyError> {
    let p = Parser::new();
    let res = p.parse(input);
    return Ok(());
}
