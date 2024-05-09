#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

use std::{fs::File, io::Read, path::Path, process::exit};

pub mod lexer;
pub mod parser;

pub struct State {
    pub debug: bool,
    pub line: u32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            debug: false,
            line: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Ident(String),
    Number(i64),
    Const,
    Var,
    Procedure,
    Call,
    Begin,
    End,
    If,
    Then,
    While,
    Do,
    Odd,
    Dot,
    Equal,
    Comma,
    Semicolon,
    Assign,
    Hash,
    LessThan,
    GreaterThan,
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}

impl std::fmt::Display for Token {
    // We need a better printer for these, I think, but this is good enough.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Can cause program termination with an error code.
pub fn read_file(filename: &Path) -> Result<String, String> {
    let path = Path::new(filename);

    if !path.extension().unwrap_or_default().eq("pl0") {
        eprintln!("Error: File must have a .pl0 extension.");
        exit(1);
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Couldn't open file: {e}")),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(e) => Err(format!("Couldn't read file: {e}")),
    }
}
