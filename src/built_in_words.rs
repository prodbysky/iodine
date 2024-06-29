use crate::{interpreter::Interpreter, lexer::ILToken};

pub fn word_drop(interpreter: &mut Interpreter) {
    interpreter.pop_value();
}

pub fn word_add(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();
    let result = a + b;

    interpreter.push_value(result.into())
}

pub fn word_subtract(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();
    let result = b - a;

    interpreter.push_value(result.into())
}

pub fn word_multiply(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();
    let result = b * a;

    interpreter.push_value(result.into())
}

pub fn word_divide(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();
    let result = b / a;

    interpreter.push_value(result.into())
}

pub fn word_less(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b < a).into())
}

pub fn word_more(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b > a).into())
}

pub fn word_equal(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b == a).into())
}

pub fn word_less_or_equal(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b <= a).into())
}

pub fn word_more_or_equal(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b <= a).into())
}

pub fn word_not_equal(interpreter: &mut Interpreter) {
    let a: f64 = interpreter.pop_value().unwrap().into();
    let b: f64 = interpreter.pop_value().unwrap().into();

    interpreter.push_value((b != a).into())
}

pub fn word_dup(interpreter: &mut Interpreter) {
    let t = interpreter.pop_value().unwrap();
    interpreter.push_value(t.clone());
    interpreter.push_value(t.clone());
}

pub fn word_print(interpreter: &mut Interpreter) {
    let t = interpreter.pop_value().unwrap();
    writeln!(interpreter.output, "{}", t).unwrap();
}

pub fn word_get_line(interpreter: &mut Interpreter) {
    let mut buf = vec![];
    let _ = interpreter.input.read_until(b'\n', &mut buf).unwrap();
    let buf: String = String::from_utf8(buf).unwrap().trim().to_string();
    interpreter.push_value(buf.into())
}

pub fn word_get_int(interpreter: &mut Interpreter) {
    let mut buf = vec![];
    let _ = interpreter.input.read_until(b'\n', &mut buf).unwrap();
    let buf: String = String::from_utf8(buf).unwrap().trim().to_string();

    interpreter.push_value(buf.parse::<i64>().unwrap().into())
}

pub fn word_get_uint(interpreter: &mut Interpreter) {
    let mut buf = vec![];
    let _ = interpreter.input.read_until(b'\n', &mut buf).unwrap();
    let buf: String = String::from_utf8(buf).unwrap().trim().to_string();
    eprintln!("{:?}", buf.trim());
    interpreter.push_value(buf.parse::<u64>().unwrap().into())
}

pub fn word_get_float(interpreter: &mut Interpreter) {
    let mut buf = vec![];
    let _ = interpreter.input.read(&mut buf).unwrap();
    let _ = interpreter.input.read_until(b'\n', &mut buf).unwrap();
    let buf: String = String::from_utf8(buf).unwrap().trim().to_string();
    interpreter.push_value(buf.trim_end().parse::<f64>().unwrap().into())
}

pub fn word_if(interpreter: &mut Interpreter) {
    let condition: bool = interpreter.pop_value().unwrap().into();

    if !condition {
        if let ILToken::If(ip) = interpreter.tokens[interpreter.position] {
            interpreter.position = ip;
        }
    }
}
