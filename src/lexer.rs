use std::{iter::Peekable, str::Chars};

use crate::{State, Token};

/// Must only be called for characters which can begin an identifier.
pub fn identifier(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
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

pub fn number(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
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

pub fn assignment(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
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

pub fn lex(state: &mut State, file: &str) -> Result<String, String> {
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
