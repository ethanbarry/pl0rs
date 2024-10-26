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

use std::path::PathBuf;
use std::process::exit;

use clap::{Parser, Subcommand};

use pl0rs::{self, lexer::lex, parser::parse, read_file, COMPILER_VERSION};

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

fn main() {
    let cli = Cli::parse();
    let mut file = String::new();
    let mut state = pl0rs::State::default();

    // Print compiler version, &c.
    println!("pl0rs -- PL/0 Compiler version {}", COMPILER_VERSION);
    println!("(c) Ethan Barry, 2024. GPLv3 licensed.");

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
            println!("Debug mode is off.");
        }
        1 => {
            println!("Debug mode is on.");
            state.debug = true;
        }
        _ => {
            println!("Don't be crazy. Defaulting to debug mode on.");
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
    match lex(&mut state, &file) {
        Ok(res) => {
            println!("Lexer succeeded.");
            match parse(&mut res.into_iter().peekable()) {
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
