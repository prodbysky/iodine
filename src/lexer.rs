use crate::errors::NumberParseError;

use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

pub struct Lexer<'a> {
    content: Peekable<Chars<'a>>,
}

#[derive(Debug)]
enum Token {
    Symbol(String),
    NumericLiteral(String),
    StringLiteral(String),
}

#[derive(Debug)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

#[derive(Debug)]
pub enum ILToken {
    String(String),
    Number(u64),
    Symbol(String),
    Operator(Operator),
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "/" => Ok(Self::Divide),
            "*" => Ok(Self::Multiply),
            _ => Err(()),
        }
    }
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

    fn next_raw(&mut self) -> Option<Token> {
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

impl Iterator for Lexer<'_> {
    type Item = ILToken;

    fn next(&mut self) -> Option<Self::Item> {
        let raw_token = self.next_raw();

        if raw_token.is_none() {
            return None;
        }

        let token = raw_token.unwrap();

        match token {
            Token::StringLiteral(str) => Some(ILToken::String(str)),
            Token::NumericLiteral(num) => Some(ILToken::Number(num.parse::<u64>().unwrap())),
            Token::Symbol(name) => {
                if let Ok(op) = Operator::from_str(name.as_str()) {
                    return Some(ILToken::Operator(op));
                }

                Some(ILToken::Symbol(name))
            }
        }
    }
}
