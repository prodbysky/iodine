use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    iter::Peekable,
    process::exit,
};

use crate::{
    built_in_words::*,
    errors,
    lexer::{self},
    stack::{self, Stack},
};

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    String(String),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
}

impl Default for StackValue {
    fn default() -> Self {
        Self::SignedInt(0)
    }
}

impl From<String> for StackValue {
    fn from(value: String) -> Self {
        Self::String(value.to_string())
    }
}

impl From<u64> for StackValue {
    fn from(value: u64) -> Self {
        Self::UnsignedInt(value)
    }
}

impl From<i64> for StackValue {
    fn from(value: i64) -> Self {
        Self::SignedInt(value)
    }
}

impl From<f64> for StackValue {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<StackValue> for f64 {
    fn from(val: StackValue) -> Self {
        match val {
            StackValue::Float(f) => f,
            StackValue::SignedInt(f) => f as f64,
            StackValue::UnsignedInt(f) => f as f64,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::String(str) => write!(f, "{}", str),
            Self::UnsignedInt(num) => write!(f, "{}", num),
            Self::SignedInt(num) => write!(f, "{}", num),
            Self::Float(num) => write!(f, "{}", num),
        }
    }
}

pub struct Interpreter<'a> {
    pub lexer: Peekable<lexer::Lexer<'a>>,
    stack: stack::Stack<StackValue>,
    builtins: HashMap<String, BuiltInAction>,
    pub output: Box<dyn Write>,
    pub input: Box<dyn BufRead>,
}

type BuiltInAction = fn(&mut Interpreter<'_>);

impl<'a> Interpreter<'a> {
    pub fn new(
        lexer: lexer::Lexer<'a>,
        output: Option<Box<dyn Write>>,
        input: Option<Box<dyn BufRead>>,
    ) -> Self {
        Self {
            lexer: lexer.peekable(),
            stack: stack::Stack::new(),
            builtins: HashMap::new(),
            output: output.unwrap_or_else(|| Box::new(std::io::stdout())),
            input: input.unwrap_or_else(|| Box::new(BufReader::new(std::io::stdin()))),
        }
    }

    fn add_word(&mut self, name: String, func: BuiltInAction) {
        self.builtins.insert(name, func);
    }

    pub fn run(&mut self) -> Result<(), errors::EmptyStackError> {
        self.add_word("drop".to_string(), word_drop);
        self.add_word("dup".to_string(), word_dup);
        self.add_word("print".to_string(), word_print);
        self.add_word("get_line".to_string(), word_get_line);

        self.add_word("+".to_string(), word_add);
        self.add_word("-".to_string(), word_subtract);
        self.add_word("/".to_string(), word_divide);
        self.add_word("*".to_string(), word_multiply);

        self.add_word("get_int".to_string(), word_get_int);
        self.add_word("get_uint".to_string(), word_get_uint);
        self.add_word("get_float".to_string(), word_get_float);
        use lexer::ILToken;
        while let Some(token) = self.lexer.next() {
            match token {
                ILToken::PushString(value) => self.push_value(value.into()),
                ILToken::PushUnsignedInteger(value) => self.push_value(value.into()),
                ILToken::PushSignedInteger(value) => self.push_value(value.into()),
                ILToken::PushFloat(value) => self.push_value(value.into()),
                ILToken::Symbol(name) => match self.builtins.get(&name) {
                    Some(t) => (t)(self),
                    None => {
                        writeln!(self.output, "Unknown word: {}", name).unwrap();
                        exit(1)
                    }
                },
            }
        }
        Ok(())
    }

    pub fn push_value(&mut self, value: StackValue) {
        match value {
            StackValue::String(str) => self.stack.push(StackValue::String(str.to_string())),
            _ => self.stack.push(value),
        }
    }

    pub fn pop_value(&mut self) -> Option<StackValue> {
        self.stack.pop()
    }

    pub fn get_stack(&self) -> &Stack<StackValue> {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    use crate::{interpreter::Interpreter, lexer::Lexer, stack::Stack};

    use super::StackValue;

    #[test]
    fn empty_program() {
        let lexer = Lexer::new("");
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let expected_stack: Stack<StackValue> = Stack::new();
        let stack = interpreter.get_stack();
        assert_eq!(&expected_stack, stack);
    }

    #[test]
    fn pushing_numbers() {
        let src = "0 1 2 3 4 5 6 7 8 9";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        for i in 0..=9 {
            expected_stack.push(StackValue::UnsignedInt(i));
        }

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn pushing_strings() {
        let src = "\"Hello :D\"";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(String::from("Hello :D").into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn mathematics() {
        let src = "1 1 + 3 1 - 1 2 * 8 4 /";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        for _ in 0..4 {
            expected_stack.push(StackValue::Float(2.0));
        }

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn drop() {
        let src = "6 9 9 drop";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(StackValue::UnsignedInt(6));
        expected_stack.push(StackValue::UnsignedInt(9));

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn dup() {
        let src = "6 9 9 dup";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer, None, None);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(StackValue::UnsignedInt(6));
        expected_stack.push(StackValue::UnsignedInt(9));
        expected_stack.push(StackValue::UnsignedInt(9));
        expected_stack.push(StackValue::UnsignedInt(9));

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn input() {
        let src = "get_line get_int get_uint get_float";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(
            lexer,
            None,
            Some(Box::new("Hello -123\r\n 246\r\n 2.0\r\n".as_bytes())),
        );
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push("Hello ".to_owned().into()); // NOTE: This is a weird space
        expected_stack.push((-123_i64).into());
        expected_stack.push(246_u64.into());
        expected_stack.push(2.0.into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }
}
