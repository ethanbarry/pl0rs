use std::path::PathBuf;

use pl0rs::{lexer::lex, parser::parse, *};

#[test]
// Generates strings like 0001, and passes them to parse_file() to get either an Ok(()) or Err(()).
fn test_parse() {
    for i in 0..11 {
        println!("{:04}", i);
        assert_eq!(parse_file(format!("{:04}", i).as_str()).unwrap(), ())
    }
}

fn parse_file(name: &str) -> Result<(), String> {
    let file_path: PathBuf = std::env::current_dir()
        .map_err(|e| eprintln!("{}", e))
        .unwrap()
        .join(format!("tests/{}.pl0", name))
        .into();
    let file = read_file(&file_path)?;
    let mut state = State::default();

    match lex(&mut state, &file) {
        Ok(res) => match parse(&mut res.into_iter().peekable()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}
