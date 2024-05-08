//  * PL/0 Syntax
//
//  * pl0rs -- PL/0 compiler.
//  *
//  * program	= block "." .
//  * block	= [ "const" ident "=" number { "," ident "=" number } ";" ]
//  *		  [ "var" ident { "," ident } ";" ]
//  *		  { "procedure" ident ";" block ";" } statement .
//  * statement	= [ ident ":=" expression
//  *		      | "call" ident
//  *		      | "begin" statement { ";" statement } "end"
//  *		      | "if" condition "then" statement
//  *		      | "while" condition "do" statement ] .
//  * condition	= "odd" expression
//  *		      | expression ( "=" | "#" | "<" | ">" ) expression .
//  * expression	= [ "+" | "-" ] term { ( "+" | "-" ) term } .
//  * term		= factor { ( "*" | "/" ) factor } .
//  * factor	=   ident
//  *		      | number
//  *		      | "(" expression ")" .

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, Subcommand};

mod lexer;
mod parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the input file
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

struct State {
    debug: bool,
    line: u32,
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
enum Token {
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
fn read_file(filename: &Path) -> Result<String, String> {
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

fn main() {
    let cli = Cli::parse();
    let mut file = String::new();
    let mut state = State::default();

    // Open the file and pass it into the file string.
    // Program can terminate here with an error code.
    if let Some(file_path) = &cli.file {
        if let Ok(file_string) = read_file(file_path) {
            file = file_string;
        } else {
            eprintln!("Error: No such file found.");
            exit(1);
        }
    }

    match cli.debug {
        0 => {
            println!("Debug mode is off");
        }
        1 => {
            println!("Debug mode is on");
            state.debug = true;
        }
        _ => {
            println!("Don't be crazy. Defaulting to debug mode on");
            state.debug = true;
        }
    }

    // Not actually used at the moment...
    match &cli.command {
        Some(Commands::Test { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Call the other modules.
    match lexer::lex(&mut state, &file) {
        Ok(res) => {
            println!("Lexer succeeded.");
            match parser::parse(&mut res.into_iter().peekable()) {
                Ok(_) => {
                    println!("Parser succeeded.");
                    exit(0);
                }
                Err(e) => {
                    eprintln!("{e}");
                    exit(1)
                } // A returned syntax error.
            }
        }
        Err(e) => {
            {
                eprintln!("{e}");
                exit(1);
            } // A returned lexer error.
        }
    }
}
