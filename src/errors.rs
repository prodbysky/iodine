use core::fmt;

#[derive(Debug, Clone)]
pub enum NumberParseErrorType {
    WrongFloat,
    NonNumericChar,
    OnlyNegativeSign,
}

#[derive(Debug, Clone)]
pub struct NumberParseError {
    pub error_type: NumberParseErrorType,
    pub literal: String,
    pub pos: usize,
}

impl NumberParseError {
    pub fn new(error_type: NumberParseErrorType, literal: String, pos: usize) -> Self {
        Self {
            error_type,
            literal,
            pos,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnterminatedStringError;
#[derive(Debug)]
pub struct EmptyStackError;

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self.error_type {
            NumberParseErrorType::WrongFloat => "Too many decimal points found in number: ",
            NumberParseErrorType::NonNumericChar => "Non-numeric char found in number: ",
            NumberParseErrorType::OnlyNegativeSign => "Only negative sign found in number",
        };
        writeln!(f, "{}", message).unwrap();
        write!(f, "\t").unwrap();
        for _ in 0..self.pos {
            write!(f, " ").unwrap();
        }

        writeln!(f, "↓").unwrap();
        write!(f, "\t{}", self.literal)
    }
}

impl fmt::Display for UnterminatedStringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Unterminated string")
    }
}

impl fmt::Display for EmptyStackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Empty stack during execution")
    }
}
