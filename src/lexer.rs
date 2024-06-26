use crate::{errors, stack::Stack};

use std::{
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    content: Peekable<Chars<'a>>,
    source: &'a str,
    pos: usize,
    time: bool,
}

#[derive(Debug)]
enum Token<'a> {
    Symbol(&'a str),
    NumericLiteral(&'a str),
    StringLiteral(&'a str),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ArgumentType {
    Nothing,
    Bool,
    String,
    Number,
}

impl FromStr for ArgumentType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "!" => Ok(Self::Nothing),
            "bool" => Ok(Self::Bool),
            "string" => Ok(Self::String),
            "number" => Ok(Self::Number),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionInfo {
    pub name: String,
    return_type: ArgumentType,
    pub pos: usize,
}

impl FunctionInfo {
    fn new(name: String, return_type: ArgumentType) -> Self {
        FunctionInfo {
            name,
            return_type,
            pos: 0,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ILToken {
    PushString(String),
    PushUnsignedInteger(u64),
    PushSignedInteger(i64),
    PushFloat(f64),
    PushBoolean(u64),
    Symbol(String),
    If(usize),
    FuncDef(FunctionInfo),
    FuncEnd,
    End,
    CommentMarker,
}

impl<'a> Lexer<'a> {
    pub fn new(string: &'a str, time: bool) -> Self {
        let string = string.trim();
        Self {
            content: string.chars().peekable(),
            source: string,
            pos: 0,
            time,
        }
    }

    fn trim_whitespace(&mut self) {
        while self.content.peek().is_some_and(|x| x.is_whitespace()) {
            self.next();
        }
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.content.next()
    }

    fn parse_number(&mut self) -> Result<Token<'a>, errors::NumberParseError> {
        let saved_pos = self.pos;
        let negative = self.content.peek().is_some_and(|&x| x == '-');
        let mut points = vec![];

        if negative {
            self.next();
        }

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            if self.next().is_some_and(|x| x == '.') {
                points.push(self.pos - saved_pos);
            }
        }

        let buffer = &self.source[saved_pos..self.pos];

        // NOTE: If the only char in number is `-`
        if negative && buffer.len() == 1 {
            return Err(errors::NumberParseError::new(
                errors::NumberParseErrorType::OnlyNegativeSign,
                buffer.to_string(),
                0,
            ));
        }

        if points.len() > 1 {
            return Err(errors::NumberParseError::new(
                errors::NumberParseErrorType::WrongFloat,
                buffer.to_string(),
                points[2],
            ));
        }

        for i in negative as usize..buffer.len() {
            let c = buffer.chars().nth(i).unwrap();
            if c == '.' {
                continue;
            }
            if c.is_alphabetic() || !c.is_ascii_digit() {
                return Err(errors::NumberParseError::new(
                    errors::NumberParseErrorType::NonNumericChar,
                    buffer.to_string(),
                    i,
                ));
            }
        }

        Ok(Token::NumericLiteral(buffer))
    }

    // TODO: Escaped strings
    fn parse_string(&mut self) -> Result<Token<'a>, errors::UnterminatedStringError> {
        let saved_pos = self.pos;

        // Skip opening quote
        let opening_quote = self.next().unwrap();

        while let Some(&current_char) = self.content.peek() {
            if current_char == opening_quote {
                break;
            }
            self.next();
        }

        // Skip closing quote
        self.pos += 1;
        let buffer = &self.source[saved_pos + 1..self.pos - 1];
        // NOTE: Can't use self.next() here (things go out of bounds in tests)
        if self.content.next().is_none() {
            return Err(errors::UnterminatedStringError);
        }

        Ok(Token::StringLiteral(buffer))
    }

    fn parse_symbol(&mut self) -> Token<'a> {
        let saved_pos = self.pos;

        while self.content.peek().is_some_and(|x| !x.is_whitespace()) {
            self.next();
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

    fn next_processed(&mut self) -> Option<ILToken> {
        let token = self.next_raw()?;

        match token {
            Token::StringLiteral(str) => Some(ILToken::PushString(str.to_string())),
            Token::NumericLiteral(num) => {
                if num.contains('.') {
                    match num.parse::<f64>() {
                        Ok(num) => return Some(ILToken::PushFloat(num)),
                        Err(e) => {
                            eprintln!("{}", e);
                            return None;
                        }
                    }
                }
                if num.chars().nth(0).is_some_and(|x| x == '-') {
                    match num.parse::<i64>() {
                        Ok(num) => return Some(ILToken::PushSignedInteger(num)),
                        Err(e) => {
                            eprintln!("{}", e);
                            return None;
                        }
                    }
                }
                match num.parse::<u64>() {
                    Ok(num) => Some(ILToken::PushUnsignedInteger(num)),
                    Err(e) => {
                        eprintln!("{}", e);
                        None
                    }
                }
            }
            Token::Symbol(name) => match name {
                "if" => Some(ILToken::If(0)),
                "end" => Some(ILToken::End),
                "fdef" => match self.next_raw() {
                    Some(Token::Symbol(name)) => {
                        self.next_raw();
                        match self.next_raw().unwrap() {
                            Token::Symbol(ret_type) => Some(ILToken::FuncDef(FunctionInfo::new(
                                name.to_string(),
                                ArgumentType::from_str(ret_type).unwrap(),
                            ))),
                            _ => None,
                        }
                    }
                    _ => None,
                },
                "fend" => Some(ILToken::FuncEnd),
                "#" => Some(ILToken::CommentMarker),
                "false" => Some(ILToken::PushBoolean(0)),
                "true" => Some(ILToken::PushBoolean(1)),
                _ => Some(ILToken::Symbol(name.to_string())),
            },
        }
    }

    fn cross_reference_blocks(program: Vec<ILToken>) -> Vec<ILToken> {
        let mut result = program.clone();
        let mut stack: Stack<usize> = Stack::new();
        for (i, token) in program.iter().enumerate() {
            match &token {
                ILToken::End => {
                    let if_ip = stack.pop().unwrap();
                    result[if_ip] = ILToken::If(i);
                }
                ILToken::If(_) => {
                    stack.push(i);
                }

                _ => {}
            }
        }

        result
    }

    pub fn parse(mut self) -> Vec<ILToken> {
        let mut program = vec![];
        if self.time {
            let mut now = std::time::Instant::now();

            while let Some(token) = self.next_processed() {
                program.push(token);
            }

            let mut elapsed = now.elapsed();
            eprintln!("Parsing program took: {:?}", elapsed);

            now = std::time::Instant::now();
            let tokens = Self::cross_reference_blocks(program);
            elapsed = now.elapsed();
            eprintln!("Cross referencing blocks took: {:?}", elapsed);
            return tokens;
        }

        while let Some(token) = self.next_processed() {
            program.push(token);
        }
        Self::cross_reference_blocks(program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_program() {
        let lexer = Lexer::new("", false);
        let empty: Vec<ILToken> = vec![];

        assert_eq!(empty, lexer.parse());
    }

    #[test]
    fn parse_u64() {
        let lexer = Lexer::new("123", false);
        let program = vec![ILToken::PushUnsignedInteger(123)];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn parse_i64() {
        let lexer = Lexer::new("-123", false);
        let program = vec![ILToken::PushSignedInteger(-123)];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn parse_f64() {
        let lexer = Lexer::new("-420.69", false);
        let program = vec![ILToken::PushFloat(-420.69)];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn parse_string() {
        let lexer = Lexer::new("\"Lotus\"", false);
        let program = vec![ILToken::PushString("Lotus".to_string())];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn parse_symbol() {
        let lexer = Lexer::new("Lotus", false);
        let program = vec![ILToken::Symbol("Lotus".to_string())];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn invalid_number() {
        let lexer = Lexer::new("6942O", false); // Look at it closely
        let program: Vec<ILToken> = vec![];

        assert_eq!(program, lexer.parse());
    }

    #[test]
    fn unterminated_string() {
        let lexer = Lexer::new("\"Lotus", false);
        let program: Vec<ILToken> = vec![];

        assert_eq!(program, lexer.parse());
    }
}
