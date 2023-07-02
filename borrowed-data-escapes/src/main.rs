// Code this may have been fixed by ChatGPT
use std::error::Error;
use std::fmt;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct MyError {
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

// There are two versions of the same line below. The first one seems to
// exhibit the borrows data escapes from function problem, the second does not
//    pub fn parse(&self, input: &'static str) -> Result<(), MyError> {
    pub fn parse(&self, input: &str) -> Result<(), MyError> {
        println!("parse: {}", input);
        return Ok(())
    }
}

fn main() {
    println!("Hello, world!");

    let input = read_to_string("infile".to_owned()).unwrap();
    let _res = fn1(&input);
}

pub fn fn1(input: &str)->Result<(), MyError> {
    let p = Parser::new();
    let _res = p.parse(input);
    return Ok(());
}
