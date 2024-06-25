use core::fmt;

#[derive(Debug, Clone)]
pub struct NumberParseError(pub String, pub usize, pub Location);

impl fmt::Display for NumberParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}, Failed to parse number:",
            Location {
                row: self.2.row + self.1 as u32,
                column: self.2.column
            }
        )
        .unwrap();
        write!(f, "\t").unwrap();
        for _ in 0..self.1 {
            write!(f, " ").unwrap();
        }

        writeln!(f, "↓").unwrap();
        write!(f, "\t{}", self.0)
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Location {
    pub column: u32,
    pub row: u32,
}

impl Location {
    pub fn new(column: u32, row: u32) -> Self {
        Self { column, row }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.row, self.column)
    }
}
