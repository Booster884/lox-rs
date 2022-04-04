use std::fmt;

#[derive(Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    Nil,
    Boolean(bool),
}

impl Value {
    pub fn is_falsey(&self) -> bool {
        match self {
            Value::Boolean(value) => !value,
            Value::Nil => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{}", value),
            Value::Nil => write!(f, "nil"),
            Value::Boolean(value) => write!(f, "{}", value),
        }
    }
}
