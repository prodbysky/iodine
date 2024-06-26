#[derive(Debug, PartialEq, Eq, Default)]
pub struct Stack<T> {
    vec: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self { vec: vec![] }
    }

    pub fn push(&mut self, element: T) {
        self.vec.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
}
