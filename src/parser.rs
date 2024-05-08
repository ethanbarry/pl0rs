use std::{iter::Peekable, vec::IntoIter};

use crate::{Token, Token::*};

pub fn parse(tokens: Vec<Token>) -> Result<String, String> {
    // Parser state.
    let mut depth = 0;

    let mut tokens = tokens.into_iter().peekable();
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        t = block(&mut tokens, depth)?;
        expect(DOT, &mut tokens)?;
        if let Some(tok) = tokens.peek() {
            eprintln!("Error: expected end of file but found remaining token {tok} after `.`");
        } else {
            println!("Parse successful.");
        }
    }
    Ok(format!("String return."))
}

fn block(tokens: &mut Peekable<IntoIter<Token>>, mut depth: i32) -> Result<Token, String> {
    depth += 1;

    // Variable nesting depth should be set globally.
    if depth > 2 {
        return Err(format!("Error: nesting depth exceeded."));
    }

    while let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        if t == CONST {
            expect(CONST, tokens)?;
            expect(IDENT(String::new()), tokens)?;
            expect(EQUAL, tokens)?;
            t = expect_with_ret(NUMBER(0_i64), tokens)?;
            while t == COMMA {
                expect(COMMA, tokens)?;
                expect(IDENT(String::new()), tokens)?;
                expect(EQUAL, tokens)?;
                t = expect_with_ret(NUMBER(0_i64), tokens)?;
            }
            t = expect_with_ret(SEMICOLON, tokens)?;
        }

        if t == VAR {
            expect(VAR, tokens)?;
            t = expect_with_ret(IDENT(String::new()), tokens)?;
            while t == COMMA {
                expect(COMMA, tokens)?;
                t = expect_with_ret(IDENT(String::new()), tokens)?;
            }
            t = expect_with_ret(SEMICOLON, tokens)?;
        }

        while t == PROCEDURE {
            expect(PROCEDURE, tokens)?;
            expect(IDENT(String::new()), tokens)?;
            expect(SEMICOLON, tokens)?;

            block(tokens, depth)?;

            t = expect_with_ret(SEMICOLON, tokens)?;
        }

        (_, t) = statement(tokens, depth)?;

        depth -= 1;
        if depth < 0 {
            return Err(format!("Nesting depth {depth} fell below zero"));
        } else {
            return Ok(t);
        }
    }

    Err(format!("Expected more tokens and found end of file."))
}

fn statement(
    tokens: &mut Peekable<IntoIter<Token>>,
    mut depth: i32,
) -> Result<(String, Token), String> {
    match tokens.peek() {
        Some(t) => {
            let mut t = t.to_owned();
            match t {
                IDENT(_) => {
                    t = expect_with_ret(IDENT(String::new()), tokens)?;
                    t = expect_with_ret(ASSIGN, tokens)?;
                    (_, t) = expression(tokens, depth)?;
                }
                CALL => {
                    t = expect_with_ret(CALL, tokens)?;
                    t = expect_with_ret(IDENT(String::new()), tokens)?;
                }
                BEGIN => {
                    t = expect_with_ret(BEGIN, tokens)?;
                    (_, t) = statement(tokens, depth)?;
                    while t == SEMICOLON {
                        t = expect_with_ret(SEMICOLON, tokens)?;
                        (_, t) = statement(tokens, depth)?;
                    }
                    t = expect_with_ret(END, tokens)?;
                }
                IF => {
                    t = expect_with_ret(IF, tokens)?;
                    (_, t) = condition(tokens, depth)?;
                    t = expect_with_ret(THEN, tokens)?;
                    (_, t) = statement(tokens, depth)?;
                }
                WHILE => {
                    t = expect_with_ret(WHILE, tokens)?;
                    (_, t) = condition(tokens, depth)?;
                    t = expect_with_ret(DO, tokens)?;
                    (_, t) = statement(tokens, depth)?;
                }
                _ => {}
            };

            return Ok((format!("String return."), t));
        }
        None => return Err(format!("Unterminated statement.")),
    }
}

fn condition(
    tokens: &mut Peekable<IntoIter<Token>>,
    mut depth: i32,
) -> Result<(String, Token), String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        if t == ODD {
            expect(ODD, tokens)?;
            (_, t) = expression(tokens, depth)?;
        } else {
            (_, t) = expression(tokens, depth)?;
            match t {
                EQUAL | HASH | LESSTHAN | GREATERTHAN => {
                    t = if let Some(tok) = tokens.next() {
                        tok
                    } else {
                        return Err(format!("Expected condition but found end of file."));
                    }
                }
                _ => return Err(format!("Syntax error: invalid conditional expression.")),
            }

            (_, t) = expression(tokens, depth)?;
        }

        return Ok((format!("String return"), t));
    } else {
        return Err(format!("Expected condition, but found end of file."));
    }
}

fn expression(
    tokens: &mut Peekable<IntoIter<Token>>,
    mut depth: i32,
) -> Result<(String, Token), String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();

        if t == PLUS || t == MINUS {
            t = expect_with_ret(t, tokens)?;
        }

        t = term(tokens, depth)?;

        while t == PLUS || t == MINUS {
            t = expect_with_ret(t, tokens)?;
            t = term(tokens, depth)?;
        }

        Ok((format!("String return"), t))
    } else {
        return Err(format!("Expected expression but found end of file."));
    }
}

fn factor(tokens: &mut Peekable<IntoIter<Token>>, mut depth: i32) -> Result<Token, String> {
    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        match t {
            IDENT(_) => {
                t = expect_with_ret(IDENT(String::new()), tokens)?;
                return Ok(t);
            }
            NUMBER(_) => {
                t = expect_with_ret(NUMBER(0_i64), tokens)?;
                return Ok(t);
            }
            LPAREN => {
                t = expect_with_ret(LPAREN, tokens)?;
                expression(tokens, depth)?;
                t = expect_with_ret(RPAREN, tokens)?;
                return Ok(t);
            }
            _ => return Err(format!("Syntax error: expected factor, found {}", t)),
        }
    } else {
        return Err(format!("Expected factor but found end of file."));
    }
}

// This is a model function that we should use as a template.
fn term(tokens: &mut Peekable<IntoIter<Token>>, mut depth: i32) -> Result<Token, String> {
    factor(tokens, depth)?;

    if let Some(t) = tokens.peek() {
        let mut t = t.to_owned();
        while t == MULTIPLY || t == DIVIDE {
            t = expect_with_ret(t, tokens)?;
            factor(tokens, depth)?;
        }
        Ok(t)
    } else {
        return Err(format!(
            "Syntax error: Expected term but found end of file."
        ));
    }
}

fn expect(val: Token, tokens: &mut Peekable<IntoIter<Token>>) -> Result<String, String> {
    if let Some(t) = tokens.next() {
        if val != t {
            return Err(format!("Syntax error: expected {val} but found {t}"));
        }
        Ok(format!(""))
    } else {
        return Err(format!(
            "Syntax error: expected {val} but found end of file."
        ));
    }
}

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
}
