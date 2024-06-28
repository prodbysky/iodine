use crate::interpreter::Interpreter;

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

pub fn word_dup(interpreter: &mut Interpreter) {
    let t = interpreter.pop_value().unwrap();
    interpreter.push_value(t.clone());
    interpreter.push_value(t.clone());
}

pub fn word_print(interpreter: &mut Interpreter) {
    let t = interpreter.pop_value().unwrap();
    println!("{}", t)
}

pub fn word_get_line(interpreter: &mut Interpreter) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    interpreter.push_value(buf.into())
}

pub fn word_get_int(interpreter: &mut Interpreter) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    interpreter.push_value(buf.trim_end().parse::<i64>().unwrap().into())
}

pub fn word_get_uint(interpreter: &mut Interpreter) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    interpreter.push_value(buf.trim_end().parse::<u64>().unwrap().into())
}

pub fn word_get_float(interpreter: &mut Interpreter) {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    interpreter.push_value(buf.trim_end().parse::<f64>().unwrap().into())
}
