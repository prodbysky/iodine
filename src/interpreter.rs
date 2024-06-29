use std::{
    collections::HashMap,
    fmt::Result,
    io::{BufRead, BufReader, Write},
    process::exit,
};

use crate::{
    built_in_words::*,
    lexer::{self, ILToken},
    stack,
};

#[derive(Debug, PartialEq, Clone)]
pub enum StackValue {
    String(String),
    UnsignedInt(u64),
    SignedInt(i64),
    Float(f64),
    Bool(bool),
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

impl From<bool> for StackValue {
    fn from(value: bool) -> Self {
        Self::Bool(value)
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

impl From<StackValue> for bool {
    fn from(value: StackValue) -> Self {
        match value {
            StackValue::Bool(bool) => bool,
            StackValue::UnsignedInt(int) => int != 0,
            StackValue::SignedInt(int) => int != 0,
            StackValue::Float(flt) => flt != 0.0,
            StackValue::String(str) => !str.is_empty(),
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
            Self::Bool(bool) => write!(f, "{}", bool),
        }
    }
}

pub struct Interpreter {
    pub tokens: Vec<ILToken>,
    pub position: usize,
    pub output: Box<dyn Write>,
    pub input: Box<dyn BufRead>,
    stack: stack::Stack<StackValue>,
    return_stack: stack::Stack<usize>,
    functions: HashMap<String, usize>,
    builtins: HashMap<String, BuiltInAction>,
    time: bool,
}

type BuiltInAction = fn(&mut Interpreter);

impl<'a> Interpreter {
    pub fn new(
        lexer: lexer::Lexer<'a>,
        output: Option<Box<dyn Write>>,
        input: Option<Box<dyn BufRead>>,
        time: bool,
    ) -> Self {
        Self {
            tokens: lexer.parse(),
            position: 0,
            output: output.unwrap_or_else(|| Box::new(std::io::stdout())),
            input: input.unwrap_or_else(|| Box::new(BufReader::new(std::io::stdin()))),
            stack: stack::Stack::new(),
            return_stack: stack::Stack::new(),
            functions: HashMap::new(),
            builtins: HashMap::new(),
            time,
        }
    }

    fn add_word(&mut self, name: String, func: BuiltInAction) {
        self.builtins.insert(name, func);
    }

    fn skip_function_body(&mut self) {
        self.skip_until(ILToken::FuncEnd);
    }

    fn skip_until(&mut self, token: ILToken) {
        self.position += 1;
        while self.position < self.tokens.len() && self.tokens[self.position] != token {
            self.position += 1;
        }
    }

    fn interpret(&mut self) -> Result {
        use lexer::ILToken;
        while self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            match token {
                ILToken::PushString(value) => self.push_value(value.into()),
                ILToken::PushUnsignedInteger(value) => self.push_value(value.into()),
                ILToken::PushSignedInteger(value) => self.push_value(value.into()),
                ILToken::PushFloat(value) => self.push_value(value.into()),
                ILToken::If(_) => {
                    word_if(self);
                }
                ILToken::End => {}
                ILToken::Symbol(name) => match self.builtins.get(&name) {
                    Some(t) => (t)(self),
                    None => match self.functions.get(&name) {
                        Some(pos) => {
                            self.return_stack.push(self.position);
                            self.position = *pos;
                        }
                        None => {
                            writeln!(self.output, "Unknown word: {}", name).unwrap();
                            exit(1)
                        }
                    },
                },
                ILToken::FuncDef(name) => {
                    self.functions.insert(name, self.position);
                    self.skip_function_body();
                }
                ILToken::FuncEnd => {
                    self.position = self.return_stack.pop().unwrap();
                }
                ILToken::CommentMarker => {
                    self.skip_until(ILToken::CommentMarker);
                }
            }
            self.position += 1;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result {
        self.add_word("drop".to_string(), word_drop);
        self.add_word("dup".to_string(), word_dup);
        self.add_word("print".to_string(), word_print);
        self.add_word("get_line".to_string(), word_get_line);

        self.add_word("+".to_string(), word_add);
        self.add_word("-".to_string(), word_subtract);
        self.add_word("/".to_string(), word_divide);
        self.add_word("*".to_string(), word_multiply);

        self.add_word("<".to_string(), word_less);
        self.add_word(">".to_string(), word_more);
        self.add_word("<=".to_string(), word_less_or_equal);
        self.add_word(">=".to_string(), word_more_or_equal);
        self.add_word("==".to_string(), word_equal);
        self.add_word("!=".to_string(), word_not_equal);

        self.add_word("get_int".to_string(), word_get_int);
        self.add_word("get_uint".to_string(), word_get_uint);
        self.add_word("get_float".to_string(), word_get_float);

        if self.time {
            let now = std::time::Instant::now();
            let result = self.interpret();
            let elapsed = now.elapsed();
            eprintln!("Running program took: {:?}", elapsed);

            return result;
        }
        self.interpret()
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

    #[cfg(test)]
    pub fn get_stack(&self) -> &stack::Stack<StackValue> {
        &self.stack
    }
}

#[cfg(test)]
mod tests {
    // TODO: Commandline output tests
    use crate::{interpreter::Interpreter, lexer::Lexer, stack::Stack};

    use super::StackValue;

    #[test]
    fn empty_program() {
        let lexer = Lexer::new("", false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let expected_stack: Stack<StackValue> = Stack::new();
        let stack = interpreter.get_stack();
        assert_eq!(&expected_stack, stack);
    }

    #[test]
    fn pushing_numbers() {
        let src = "0 1 2 3 4 5 6 7 8 9";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
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
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(String::from("Hello :D").into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn mathematics() {
        let src = "1 1 + 3 1 - 1 2 * 8 4 /";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
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
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(StackValue::UnsignedInt(6));
        expected_stack.push(StackValue::UnsignedInt(9));

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn dup() {
        let src = "6 9 9 dup";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
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
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(
            lexer,
            None,
            Some(Box::new("Hello\n -123\n 246\n 2.0\n".as_bytes())),
            false,
        );
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push("Hello".to_owned().into());
        expected_stack.push((-123_i64).into());
        expected_stack.push(246_u64.into());
        expected_stack.push(2.0.into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn if_statement() {
        let src = "18 15 > if \"You are underage\" end";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push("You are underage".to_owned().into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn function() {
        let src = "fdef square dup * fend 2 square";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(4.0.into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn comments() {
        let src = "# Multiply # fdef mul * fend 4 2 mul # Output: 8 #";
        let lexer = Lexer::new(src, false);
        let mut interpreter = Interpreter::new(lexer, None, None, false);
        interpreter.run().unwrap();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(8.0.into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }
}
