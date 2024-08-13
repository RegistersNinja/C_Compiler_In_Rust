/*
1. read in the file
2. start tokenization:
    identifier
    constant
    keyword
    all kinds of parenthesis and semicolons

    sample output:
        For Expression 1:
        Token: Keyword, Value: int
        Token: Identifier, Value: a
        Token: Operator, Value: =
        Token: Identifier, Value: b
        Token: Operator, Value: +
        Token: Identifier, Value: c
*/


use std::{fmt::Error, fs};
use crate::args::ReturnStruct;
use regex::Regex;



fn read_input(src: String) -> Result<String,Error> {
    match fs::read_to_string(&src) {
        Ok(content) => {
            Ok(content) 
        }
        Err(e) => {
            eprintln!("An error reading the file:\n{}", e);
            Err(Error)
        }
    }
}

fn tokenizer(mut input: &str) -> Vec<(String, String)> { //list
    let regex_keys:Vec<(&str, &str)> = vec![
        ("constant", r"^([0-9]+\b)"),
        ("identifier", r"^([a-zA-Z_]\w*\b)"),
        ("int_keyword", r"^(int\b)"),
        ("main_keyword", r"^(main\b)"),
        ("void_keyword", r"^(void\b)"),
        ("return_keyword", r"^(return\b)"),
        ("open_parenthesis", r"^(\()"),
        ("close_parenthesis", r"^(\))"),
        ("open_brace", r"^(\{)"),
        ("close_brace", r"^(\})"),
        ("semicolon", r"^(;)"),
    ];
    
    let mut tokenized: Vec<(String, String)> = Vec::new(); //list
    while !input.is_empty(){
        input = input.trim();
        let (mut match_prev_len,mut match_token, mut match_value):(usize, String, String) = (0,"".to_string(),"".to_string());
        for (token,reg) in regex_keys.iter() {
            let regex = Regex::new(reg).unwrap(); 
            let capture = regex.find(input);
            match capture {
                None => continue,
                Some(m) => {
                    let capture_length = m.end() - m.start();
                    if capture_length > match_prev_len {
                        match_prev_len = capture_length;
                        match_token = token.to_string();
                        match_value = m.as_str().to_string();
                    }
                }
            }  
        }
        tokenized.push((match_token.to_string(),match_value.to_string()));
        input = &input[match_prev_len..];
    }
    tokenized
}


pub fn lexer(input: &ReturnStruct) -> Result<Vec<(String, String)>,Error> { 
    let read_file = read_input(input.src_path.clone());
    match read_file{
        Ok(s) => return Ok(tokenizer(&s)),
        Err(e) => {
            Err(e)
        }
    }
}
