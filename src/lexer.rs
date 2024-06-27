use crate::{
    errors::{NumberParseError, UnterminatedStringError},
    interpreter::StackValue,
};

use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    content: Peekable<Chars<'a>>,
    source: &'a str,
    pos: usize,
}

#[derive(Debug)]
enum Token<'a> {
    Symbol(&'a str),
    NumericLiteral(&'a str),
    StringLiteral(&'a str),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Divide,
    Multiply,
}

impl Operator {
    // NOTE: Currently all math operations coerce to f64
    pub fn evaluate(&self, a: StackValue, b: StackValue) -> f64 {
        let a: f64 = a.into();
        let b: f64 = b.into();
        match &self {
            Self::Add => b + a,
            Self::Subtract => b - a,
            Self::Divide => b / a,
            Self::Multiply => b * a,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum ILToken {
    PushString(String),
    PushUnsignedInteger(u64),
    PushSignedInteger(i64),
    PushFloat(f64),
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
            source: string,
            pos: 0,
        }
    }

    fn trim_whitespace(&mut self) {
        while self.content.peek().is_some_and(|x| x.is_whitespace()) {
            self.content.next();
            self.pos += 1;
        }
    }

    fn parse_number(&mut self) -> Result<Token<'a>, NumberParseError> {
        // TODO: Floats, and negative numbers, and maybe hexadecimal values
        let saved_pos = self.pos;
        let negative = self.content.peek().is_some_and(|&x| x == '-');
        let mut points = vec![];

        if negative {
            self.content.next();
            self.pos += 1;
        }

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            if self.content.next().is_some_and(|x| x == '.') {
                points.push(self.pos - saved_pos);
            }

            self.pos += 1;
        }

        let buffer = &self.source[saved_pos..self.pos];

        if negative && buffer.len() == 1 {
            return Err(NumberParseError(buffer.to_string(), 0));
        }

        if points.len() > 1 {
            return Err(NumberParseError(buffer.to_string(), points[2]));
        }

        for i in negative as usize..buffer.len() {
            let c = buffer.chars().nth(i).unwrap();
            if c == '.' {
                continue;
            }
            if c.is_alphabetic() || !c.is_ascii_digit() {
                return Err(NumberParseError(buffer.to_string(), i));
            }
        }

        Ok(Token::NumericLiteral(buffer))
    }

    // TODO: Escaped strings
    fn parse_string(&mut self) -> Result<Token<'a>, UnterminatedStringError> {
        let saved_pos = self.pos;

        // Skip opening quote
        let opening_quote = self.content.next().unwrap();
        self.pos += 1;

        while let Some(&current_char) = self.content.peek() {
            if current_char == opening_quote {
                break;
            }
            self.content.next();
            self.pos += 1;
        }

        // Skip closing quote
        self.pos += 1;

        let buffer = &self.source[saved_pos + 1..self.pos - 1];
        if self.content.next().is_none() {
            return Err(UnterminatedStringError);
        }

        Ok(Token::StringLiteral(buffer))
    }

    fn parse_symbol(&mut self) -> Token<'a> {
        let saved_pos = self.pos;

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            self.content.next();
            self.pos += 1;
        }

        let buffer = &self.source[saved_pos..self.pos];

        Token::Symbol(buffer)
    }

    fn next_raw(&mut self) -> Option<Token<'a>> {
        self.trim_whitespace();

        let current_char = *self.content.peek()?;

        // HACK: Hack to correctly differenciate `-` operator from negative sign of signed integer
        let mut cloned = self.content.clone();
        cloned.next();
        let next_char = cloned.next();

        if current_char == '"' || current_char == '\'' {
            match self.parse_string() {
                Ok(str) => return Some(str),
                Err(e) => {
                    eprintln!("{}", e);
                    return None;
                }
            }
        }
        if current_char.is_ascii_digit()
            || (current_char == '-' && next_char.is_some_and(|x| x.is_ascii_digit()))
        {
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

impl<'a> Iterator for Lexer<'a> {
    type Item = ILToken;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_raw()?;

        match token {
            Token::StringLiteral(str) => Some(ILToken::PushString(str.to_string())),
            Token::NumericLiteral(num) => {
                if num.contains('.') {
                    return Some(ILToken::PushFloat(num.parse::<f64>().unwrap()));
                }
                if num.chars().nth(0).is_some_and(|x| x == '-') {
                    return Some(ILToken::PushSignedInteger(num.parse::<i64>().unwrap()));
                }
                Some(ILToken::PushUnsignedInteger(num.parse::<u64>().unwrap()))
            }
            Token::Symbol(name) => {
                if let Ok(op) = Operator::from_str(name) {
                    return Some(ILToken::Operator(op));
                }

                Some(ILToken::Symbol(name.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program() {
        let lexer = Lexer::new("");
        let empty: Vec<ILToken> = vec![];

        assert_eq!(empty, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn parse_u64() {
        let lexer = Lexer::new("123");
        let program = vec![ILToken::PushUnsignedInteger(123)];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn parse_i64() {
        let lexer = Lexer::new("-123");
        let program = vec![ILToken::PushSignedInteger(-123)];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn parse_f64() {
        let lexer = Lexer::new("-420.69");
        let program = vec![ILToken::PushFloat(-420.69)];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn parse_string() {
        let lexer = Lexer::new("\"Lotus\"");
        let program = vec![ILToken::PushString("Lotus".to_string())];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn parse_symbol() {
        let lexer = Lexer::new("Lotus");
        let program = vec![ILToken::Symbol("Lotus".to_string())];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn invalid_number() {
        let lexer = Lexer::new("6942O"); // Look at it closely
        let program: Vec<ILToken> = vec![];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }

    #[test]
    fn unterminated_string() {
        let lexer = Lexer::new("\"Lotus");
        let program: Vec<ILToken> = vec![];

        assert_eq!(program, lexer.collect::<Vec<ILToken>>());
    }
}
