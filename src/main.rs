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
//  *		  | "call" ident
//  *		  | "begin" statement { ";" statement } "end"
//  *		  | "if" condition "then" statement
//  *		  | "while" condition "do" statement ] .
//  * condition	= "odd" expression
//  *		| expression ( "=" | "#" | "<" | ">" ) expression .
//  * expression	= [ "+" | "-" ] term { ( "+" | "-" ) term } .
//  * term		= factor { ( "*" | "/" ) factor } .
//  * factor	= ident
//  *		| number
//  *		| "(" expression ")" .

use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Peekable;
use std::path::Path;
use std::path::PathBuf;
use std::str::Chars;

use clap::{Parser, Subcommand};

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

#[derive(Debug)]
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

/// Must only be called for characters which can begin an identifier.
fn identifier(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
    let mut idt = String::new();
    loop {
        if let Some(c) = chars.peek() {
            if (*c).is_alphanumeric() || (*c).eq(&'_') {
                idt.push(*c);
                chars.next();
            } else {
                break;
            }
        } else {
            return Err(format!("Unterminated identifier on line {}", state.line));
        }
    }

    let token = match idt.to_lowercase().as_str() {
        "const" => Token::CONST,
        "var" => Token::VAR,
        "procedure" => Token::PROCEDURE,
        "call" => Token::CALL,
        "begin" => Token::BEGIN,
        "end" => Token::END,
        "if" => Token::IF,
        "then" => Token::THEN,
        "while" => Token::WHILE,
        "do" => Token::DO,
        "odd" => Token::ODD,
        _ => Token::IDENT(idt),
    };

    Ok(token)
}

fn number(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
    let mut num = String::new();
    loop {
        if let Some(c) = chars.peek() {
            if (*c).is_numeric() || (*c).eq(&'_') {
                num.push(*c);
                chars.next();
            } else {
                break;
            }
        } else {
            return Err(format!("Unterminated number on line {}", state.line));
        }
    }

    if let Ok(res) = num.parse::<i64>() {
        println!("Returning {}", &res);
        return Ok(Token::NUMBER(res));
    } else {
        return Err(format!("Invalid number at line {}", state.line));
    }
}

fn assignment(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
    chars.next();
    if let Some(c) = chars.peek() {
        if (*c).eq(&'=') {
            return Ok(Token::ASSIGN);
        } else {
            return Err(format!("Unknown token on line {}", state.line));
        }
    } else {
        return Err(format!("Unterminated assignment on line {}", state.line));
    }
}

fn lex(state: &mut State, file: &str) -> Result<String, String> {
    let mut chars = file.chars().peekable();
    let mut comment = String::new();
    let mut tokens: Vec<Token> = vec![];

    'lexer: loop {
        if let Some(c) = chars.peek() {
            if (*c).eq(&'{') {
                chars.next(); // Consume the opening brace
                'comment: while let Some(c) = chars.next() {
                    if c == '}' {
                        break 'comment;
                    }
                    comment.push(c);
                }
            } else if (*c).is_whitespace() {
                if c.eq(&'\n') {
                    state.line += 1;
                }
                chars.next(); // Consume the whitespace.
            } else if (*c).is_alphabetic() || (*c).eq(&'_') {
                let token = identifier(&mut chars, state)?;
                tokens.push(token);
            } else if (*c).is_numeric() {
                let token = number(&mut chars, state)?;
                tokens.push(token);
            } else if (*c).eq(&':') {
                let token = assignment(&mut chars, state)?;
                tokens.push(token);
            } else {
                let token = match *c {
                    '.' => Token::DOT,
                    '=' => Token::EQUAL,
                    ',' => Token::COMMA,
                    ';' => Token::SEMICOLON,
                    '#' => Token::HASH,
                    '<' => Token::LESSTHAN,
                    '>' => Token::GREATERTHAN,
                    '+' => Token::PLUS,
                    '-' => Token::MINUS,
                    '*' => Token::MULTIPLY,
                    '/' => Token::DIVIDE,
                    '(' => Token::LPAREN,
                    ')' => Token::RPAREN,
                    _ => {
                        return Err(format!("Unknown token on line {}", state.line));
                    }
                };
                tokens.push(token);
                chars.next();
            }
        } else {
            break 'lexer; // No more characters
        }
    }

    dbg!(tokens);

    // Do something with the comment string (if needed)
    Ok(comment)
}

fn main() {
    let cli = Cli::parse();
    let mut file = String::new();

    if let Some(file_path) = cli.file.as_deref() {
        if let Ok(file_string) = read_file(file_path) {
            file = file_string;
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

    let res = lex(&mut state, &file);
    if let Err(s) = res {
        println!("{s}");
    }
    // Continued program logic goes here...
}
