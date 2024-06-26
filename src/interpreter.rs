use crate::{
    lexer,
    stack::{self, Stack},
};

#[derive(Debug, PartialEq, Eq)]
pub enum StackValue {
    String(String),
    Number(u64),
}

impl Default for StackValue {
    fn default() -> Self {
        Self::Number(0)
    }
}

impl From<String> for StackValue {
    fn from(value: String) -> Self {
        Self::String(value.to_string())
    }
}

impl From<u64> for StackValue {
    fn from(value: u64) -> Self {
        Self::Number(value)
    }
}

#[derive(Debug)]
pub struct Interpreter<'a> {
    lexer: lexer::Lexer<'a>,
    stack: stack::Stack<StackValue>,
}

impl<'a> Interpreter<'a> {
    pub fn new(lexer: lexer::Lexer<'a>) -> Self {
        Self {
            lexer,
            stack: stack::Stack::new(),
        }
    }

    pub fn run(&mut self) {
        while let Some(token) = self.lexer.next() {
            match token {
                lexer::ILToken::PushString(value) => self.push_value(StackValue::from(value)),
                lexer::ILToken::PushNumber(value) => self.push_value(StackValue::from(value)),
                lexer::ILToken::Operator(op) => {
                    let a = self.pop_value().unwrap();
                    let b = self.pop_value().unwrap();
                    if let StackValue::Number(a) = a {
                        if let StackValue::Number(b) = b {
                            self.push_value(op.evaluate(a, b).into())
                        }
                    }
                }
                _ => {}
            }
        }
    }

    fn push_value(&mut self, value: StackValue) {
        match value {
            StackValue::String(str) => self.stack.push(StackValue::String(str.to_string())),
            StackValue::Number(num) => self.stack.push(StackValue::Number(num)),
        }
    }

    fn pop_value(&mut self) -> Option<StackValue> {
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
        let mut interpreter = Interpreter::new(lexer);
        interpreter.run();

        let expected_stack: Stack<StackValue> = Stack::new();
        let stack = interpreter.get_stack();
        assert_eq!(&expected_stack, stack);
    }

    #[test]
    fn pushing_numbers() {
        let src = "0 1 2 3 4 5 6 7 8 9";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer);
        interpreter.run();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        for i in 0..=9 {
            expected_stack.push(StackValue::Number(i));
        }

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn pushing_strings() {
        let src = "\"Hello :D\"";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer);
        interpreter.run();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        expected_stack.push(String::from("Hello :D").into());

        assert_eq!(&expected_stack, interpreter.get_stack());
    }

    #[test]
    fn mathematics() {
        let src = "1 1 + 3 1 - 1 2 * 8 4 /";
        let lexer = Lexer::new(src);
        let mut interpreter = Interpreter::new(lexer);
        interpreter.run();

        let mut expected_stack: Stack<StackValue> = Stack::new();
        for _ in 0..4 {
            expected_stack.push(StackValue::Number(2));
        }

        assert_eq!(&expected_stack, interpreter.get_stack());
    }
}
