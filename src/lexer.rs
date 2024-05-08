use std::{iter::Peekable, str::Chars};

use crate::{State, Token};

/// ## Preconditions
///
/// - Must only be called for characters which can begin an identifier.
pub fn identifier(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
    let mut idt = String::new();

    loop {
        // Push characters onto idt one by one.
        if let Some(c) = chars.peek() {
            if (*c).is_alphanumeric() || (*c).eq(&'_') {
                idt.push(*c);
                chars.next();
            } else {
                break;
            }
        } else {
            // Was a None.
            return Err(format!("Unterminated identifier on line {}", state.line));
        }
    }

    let token = match idt.to_lowercase().as_str() {
        "const" => Token::Const,
        "var" => Token::Var,
        "procedure" => Token::Procedure,
        "call" => Token::Call,
        "begin" => Token::Begin,
        "end" => Token::End,
        "if" => Token::If,
        "then" => Token::Then,
        "while" => Token::While,
        "do" => Token::Do,
        "odd" => Token::Odd,
        _ => Token::Ident(idt),
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
            // Was a None.
            return Err(format!("Unterminated number on line {}", state.line));
        }
    }

    if let Ok(res) = num.parse::<i64>() {
        Ok(Token::Number(res))
    } else {
        Err(format!("Invalid number at line {}", state.line))
    }
}

pub fn assignment(chars: &mut Peekable<Chars<'_>>, state: &mut State) -> Result<Token, String> {
    chars.next(); // Consume the ':' character.

    if let Some(c) = chars.peek() {
        if (*c).eq(&'=') {
            chars.next();
            Ok(Token::Assign)
        } else {
            Err(format!("Unknown token ':' on line {}", state.line))
        }
    } else {
        Err(format!("Unterminated assignment on line {}", state.line))
    }
}

pub fn lex(state: &mut State, file: &str) -> Result<Vec<Token>, String> {
    let mut chars = file.chars().peekable();
    let mut comment = String::new();
    let mut tokens: Vec<Token> = vec![];

    'lexer: loop {
        if let Some(c) = chars.peek() {
            if (*c).eq(&'{') {
                chars.next(); // Consume the opening brace
                'comment: for c in chars.by_ref() {
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
                    '.' => Token::Dot,
                    '=' => Token::Equal,
                    ',' => Token::Comma,
                    ';' => Token::Semicolon,
                    '#' => Token::Hash,
                    '<' => Token::LessThan,
                    '>' => Token::GreaterThan,
                    '+' => Token::Plus,
                    '-' => Token::Minus,
                    '*' => Token::Multiply,
                    '/' => Token::Divide,
                    '(' => Token::LParen,
                    ')' => Token::RParen,
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

    Ok(tokens)
}
