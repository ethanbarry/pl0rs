#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
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
    current_char: char,
    debug: bool,
    line: u32,
}

#[derive(Debug, Clone)]
enum Token {
    IDENT(String),
    NUMBER(i64),
    CONST,
    VAR,
    PROCEDURE,
    CALL,
    BEGIN,
    END,
    IF,
    THEN,
    WHILE,
    DO,
    ODD,
    DOT,
    EQUAL,
    COMMA,
    SEMICOLON,
    ASSIGN,
    HASH,
    LESSTHAN,
    GREATERTHAN,
    PLUS,
    MINUS,
    MULTIPLY,
    DIVIDE,
    LPAREN,
    RPAREN,
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

fn read_file(filename: &Path) -> Result<String, String> {
    let path = Path::new(filename);

    if !path.extension().unwrap_or_default().eq("pl0") {
        return Err(String::from("Error: File must have a .pl0 extension"));
    }

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => return Err(format!("Couldn't open file: {}", why)),
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Ok(contents),
        Err(why) => Err(format!("Couldn't read file: {}", why)),
    }
}

fn main() {
    let cli = Cli::parse();
    let mut file = String::new();

    if let Some(file_path) = cli.file.as_deref() {
        if let Ok(file_string) = read_file(file_path) {
            file = file_string;
        } else {
            eprintln!("Error: No such file found.");
            exit(1);
        }
    }

    let mut state = State {
        current_char: ' ',
        debug: false,
        line: 1,
    };

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    match cli.debug {
        0 => {
            println!("Debug mode is off");
        }
        1 => {
            println!("Debug mode is on");
            state = State {
                current_char: ' ',
                debug: true,
                line: 1,
            }
        }
        _ => {
            println!("Don't be crazy. Defaulting to debug mode on");
            state = State {
                current_char: ' ',
                debug: true,
                line: 1,
            }
        }
    }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
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

    match lexer::lex(&mut state, &file) {
        Ok(res) => match parser::parse(res) {
            Ok(s) => println!("{s}\nProgram complete."),
            Err(e) => eprintln!("{e}"),
        },
        Err(e) => {
            eprintln!("{e}");
        }
    }
}
