use std::{iter::Peekable, vec::IntoIter};

use crate::{Token, Token::*};

const NESTING_DEPTH: i32 = 0;

pub fn parse(tokens: &mut Peekable<IntoIter<Token>>) -> Result<(), String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();

        t = block(tokens, NESTING_DEPTH)?; // t should be the last token in the file.

        if t == Dot {
            tokens.next();
        } else {
            return Err("Expected DOT but found end of file.".to_string());
        }

        // Now if tokens follow, we have an error.
        if let Some(tok) = tokens.peek() {
            Err("Error: expected end of file but found remaining token {tok} after `.`".to_string())
        } else {
            Ok(())
        }
    } else {
        Err("Error: No tokens found - empty file.".to_string())
    }
}

fn block(tokens: &mut Peekable<IntoIter<Token>>, mut depth: i32) -> Result<Token, String> {
    depth += 1;

    // Variable nesting depth should be set globally.
    if depth > 2 {
        return Err("Error: nesting depth exceeded.".to_string());
    }

    return if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        if t == Const {
            t = expect(Const, tokens)?;
            t = expect(Ident(String::new()), tokens)?;
            t = expect(Equal, tokens)?;
            t = expect(Number(0_i64), tokens)?;
            while t == Comma {
                t = expect(Comma, tokens)?;
                t = expect(Ident(String::new()), tokens)?;
                t = expect(Equal, tokens)?;
                t = expect(Number(0_i64), tokens)?;
            }
            t = expect(Semicolon, tokens)?;
        }

        if t == Var {
            t = expect(Var, tokens)?;
            t = expect(Ident(String::new()), tokens)?;
            while t == Comma {
                t = expect(Comma, tokens)?;
                t = expect(Ident(String::new()), tokens)?;
            }
            t = expect(Semicolon, tokens)?;
        }

        while t == Procedure {
            t = expect(Procedure, tokens)?;
            t = expect(Ident(String::new()), tokens)?;
            t = expect(Semicolon, tokens)?;

            t = block(tokens, depth)?;

            t = expect(Semicolon, tokens)?;
        }

        t = statement(tokens)?;

        depth -= 1;
        if depth < 0 {
            Err(format!("Nesting depth {depth} fell below zero"))
        } else {
            Ok(t)
        }
    } else {
        Err("Expected more tokens and found end of file.".to_string())
    };
}

fn statement(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    match tokens.peek() {
        Some(t) => {
            let mut t = t.to_owned();
            match t {
                Ident(_) => {
                    t = expect(Ident(String::new()), tokens)?;
                    t = expect(Assign, tokens)?;
                    t = expression(tokens)?;
                }
                Call => {
                    t = expect(Call, tokens)?;
                    t = expect(Ident(String::new()), tokens)?;
                }
                Begin => {
                    t = expect(Begin, tokens)?;
                    t = statement(tokens)?;
                    while t == Semicolon {
                        t = expect(Semicolon, tokens)?;
                        t = statement(tokens)?;
                    }
                    t = expect(End, tokens)?;
                }
                If => {
                    t = expect(If, tokens)?;
                    t = condition(tokens)?;
                    t = expect(Then, tokens)?;
                    t = statement(tokens)?;
                }
                While => {
                    t = expect(While, tokens)?;
                    t = condition(tokens)?;
                    t = expect(Do, tokens)?;
                    t = statement(tokens)?;
                }
                _ => {}
            };

            Ok(t)
        }
        None => Err("Unterminated statement.".to_string()),
    }
}

fn condition(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        if t == Odd {
            t = expect(Odd, tokens)?;
            t = expression(tokens)?;
        } else {
            t = expression(tokens)?;
            match t {
                Equal | Hash | LessThan | GreaterThan => {
                    // TODO: Isn't this just expect()?
                    t = if let Some(tok) = tokens.next() {
                        tok
                    } else {
                        return Err("Expected condition but found end of file.".to_string());
                    }
                }
                _ => return Err("Syntax error: invalid conditional expression.".to_string()),
            }

            t = expression(tokens)?;
        }

        Ok(t)
    } else {
        Err("Expected condition, but found end of file.".to_string())
    }
}

fn expression(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();

        if t == Plus || t == Minus {
            t = expect(t, tokens)?;
        }

        t = term(tokens)?;

        while t == Plus || t == Minus {
            t = expect(t, tokens)?;
            t = term(tokens)?;
        }

        Ok(t)
    } else {
        Err("Expected expression but found end of file.".to_string())
    }
}

fn factor(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        match t {
            Ident(_) => {
                t = expect(Ident(String::new()), tokens)?;
                Ok(t)
            }
            Number(_) => {
                t = expect(Number(0_i64), tokens)?;
                Ok(t)
            }
            LParen => {
                t = expect(LParen, tokens)?;
                t = expression(tokens)?;
                t = expect(RParen, tokens)?;
                Ok(t)
            }
            _ => Err("Syntax error: expected factor, found {t}".to_string()),
        }
    } else {
        Err("Expected factor but found end of file.".to_string())
    }
}

// This is a model function that we should use as a template.
fn term(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();

        t = factor(tokens)?;

        while t == Multiply || t == Divide {
            t = expect(t, tokens)?;
            t = factor(tokens)?;
        }
        Ok(t)
    } else {
        Err("Syntax error: Expected term but found end of file.".to_string())
    }
}

fn expect(val: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.next() {
        if val != t {
            Err(format!("Syntax error: expected {val} but found {t}"))
        } else {
            return if let Some(t) = tokens.peek() {
                Ok(t.clone())
            } else {
                Err(format!(
                    "Syntax error: expected {val} but found end of file."
                ))
            };
        }
    } else {
        Err(format!(
            "Syntax error: expected {val} but found end of file."
        ))
    }
}

/*
fn expect_with_ret(val: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<Token, String> {
    if let Some(t) = tokens.next() {
        let t = t.to_owned();
        if val == t {
            if let Some(tok) = tokens.peek() {
                return Ok(tok.clone());
            } else {
                // Handle the end of a file better, by not printing what we already found, if it was the last character of a file.
                return Err(format!(
                    "Syntax error: expected {val} but found end of file."
                ));
            }
        } else {
            return Err(format!("Syntax error: expected {val} but found {t}"));
        }
    } else {
        return Err(format!(
            "Syntax error: expected {val} but found end of file."
        ));
    }
}*/
