use core::fmt;

#[derive(Debug, Clone)]
pub struct NumberParseError(pub String, pub usize);
#[derive(Debug, Clone)]
pub struct UnterminatedStringError;
#[derive(Debug)]
pub struct EmptyStackError;

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Failed to parse number:",).unwrap();
        write!(f, "\t").unwrap();
        for _ in 0..self.1 {
            write!(f, " ").unwrap();
        }

        writeln!(f, "↓").unwrap();
        write!(f, "\t{}", self.0)
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
