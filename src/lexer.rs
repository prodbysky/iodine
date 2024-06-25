use crate::errors::{Location, NumberParseError};
use std::{iter::Peekable, str::Chars};

pub struct Lexer<'a> {
    pub content: Peekable<Chars<'a>>,
    location: Location,
}

#[derive(Debug)]
pub enum Token {
    Operator(Location, String),
    NumericLiteral(Location, String),
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str) -> Self {
        let string = string.trim();
        Self {
            content: string.chars().peekable(),
            location: Location::new(1, 1),
        }
    }

    fn trim_whitespace(&mut self) {
        while self.content.peek().is_some_and(|x| x.is_whitespace()) {
            if self.content.peek().is_some_and(|&x| x == ' ') {
                self.location.column += 1;
            }
            if self.content.peek().is_some_and(|&x| x == '\n') {
                self.location.row += 1;
            }
            if self.content.peek().is_some_and(|&x| x == '\t') {
                self.location.column += 4;
            }
            self.content.next();
        }
    }

    // Currently only parses unsigned integers
    fn parse_number(&mut self) -> Result<Token, NumberParseError> {
        let mut number = String::new();
        let saved_loc = self.location;

        number.push(self.content.next().unwrap());
        self.location.column += 1;

        for c in self.content.by_ref() {
            if c == '_' {
                continue;
            }
            if c.is_whitespace() {
                self.location.column += 1;
                break;
            }
            number.push(c);
            self.location.column += 1;
        }

        for i in 0..number.len() {
            if number.chars().nth(i).unwrap().is_alphabetic() {
                return Err(NumberParseError(number, i, saved_loc));
            }
        }

        number = number.trim().to_string();
        self.trim_whitespace();
        Ok(Token::NumericLiteral(saved_loc, number))
    }

    fn parse_operator(&mut self) -> Option<Token> {
        let c = self.content.peek().cloned();
        let saved_loc = self.location;
        if c.is_none() || !"+-/*=".contains(c.unwrap()) {
            return None;
        }

        self.content.next();
        self.location.column += 1;
        match c.unwrap() {
            '+' => Some(Token::Operator(saved_loc, "+".to_string())),
            '-' => Some(Token::Operator(saved_loc, "-".to_string())),
            '/' => Some(Token::Operator(saved_loc, "/".to_string())),
            '*' => Some(Token::Operator(saved_loc, "*".to_string())),
            '=' => Some(Token::Operator(saved_loc, "=".to_string())),
            _ => None,
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.trim_whitespace();

        self.content.peek()?;

        if let Some(operator) = self.parse_operator() {
            Some(operator)
        } else {
            match self.parse_number() {
                Ok(num) => Some(num),
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
            }
        }
    }
}