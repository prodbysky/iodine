use crate::errors::NumberParseError;
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    pub content: Peekable<Chars<'a>>,
}

#[derive(Debug)]
pub enum Token {
    Operator(String),
    Symbol(String),
    NumericLiteral(String),
    StringLiteral(String),
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Self {
        let string = string.trim();
        Self {
            content: string.chars().peekable(),
        }
    }

    fn trim_whitespace(&mut self) {
        while self.content.peek().is_some_and(|x| x.is_whitespace()) {
            self.content.next();
        }
    }

    fn parse_number(&mut self) -> Result<Token, NumberParseError> {
        // TODO: Floats, and negative numbers, and maybe hexadecimal values
        let mut buffer = String::new();

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            buffer.push(self.content.next().unwrap());
        }

        for i in 0..buffer.len() {
            let c = buffer.chars().nth(i).unwrap();
            if c.is_alphabetic() || !c.is_ascii_digit() {
                return Err(NumberParseError(buffer, i));
            }
        }

        Ok(Token::NumericLiteral(buffer))
    }

    // TODO: Escaped strings
    fn parse_string(&mut self) -> Token {
        let mut buffer = String::new();

        while self.content.peek().is_some_and(|&x| x != '"' || x != '\'') {
            buffer.push(self.content.next().unwrap());
        }

        let buffer = buffer.trim_matches('\'').trim_matches('"');

        Token::NumericLiteral(buffer.to_string())
    }

    fn parse_symbol(&mut self) -> Token {
        let mut buffer = String::new();

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            buffer.push(self.content.next().unwrap());
        }

        Token::Symbol(buffer)
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_whitespace();

        let current_char = self.content.peek();

        if current_char == None {
            return None;
        }

        let current_char = *current_char.unwrap();

        if current_char == '"' || current_char == '\'' {
            return Some(self.parse_string());
        }

        if current_char.is_ascii_digit() {
            match self.parse_number() {
                Ok(num) => return Some(num),
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        }
        return Some(self.parse_symbol());
    }
}
