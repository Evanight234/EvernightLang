use crate::bytecode::Chunk;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Angka(f64),
    Teks(String),
    Bolean(bool),
    Kosong,
    Daftar(Rc<RefCell<Vec<Value>>>),
    Kamus(Rc<RefCell<HashMap<String, Value>>>),
    Fungsi {
        name: String,
        arity: usize,
        chunk: Rc<Chunk>,
    },
}

impl Value {
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bolean(b) => *b,
            Value::Kosong => false,
            Value::Angka(n) => *n != 0.0,
            Value::Teks(s) => !s.is_empty(),
            Value::Daftar(d) => !d.borrow().is_empty(),
            Value::Kamus(k) => !k.borrow().is_empty(),
            Value::Fungsi { .. } => true,
        }
    }

    pub fn as_angka(&self) -> Option<f64> {
        match self {
            Value::Angka(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bolean(b) => Some(*b),
            _ => None,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Angka(n) => {
                if n.fract() == 0.0 && *n >= -9007199254740992.0 && *n <= 9007199254740992.0 {
                    write!(f, "{:.0}", n)
                } else {
                    write!(f, "{}", n)
                }
            }
            Value::Teks(s) => write!(f, "{}", s),
            Value::Bolean(b) => write!(f, "{}", if *b { "benar" } else { "salah" }),
            Value::Kosong => write!(f, "kosong"),
            Value::Daftar(d) => {
                let v = d.borrow();
                write!(f, "[")?;
                for (i, val) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", val)?;
                }
                write!(f, "]")
            }
            Value::Kamus(k) => {
                let m = k.borrow();
                write!(f, "{{")?;
                for (i, (key, val)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", key, val)?;
                }
                write!(f, "}}")
            }
            Value::Fungsi { name, .. } => write!(f, "<fungsi {}>", name),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Angka(a), Value::Angka(b)) => a == b,
            (Value::Teks(a), Value::Teks(b)) => a == b,
            (Value::Bolean(a), Value::Bolean(b)) => a == b,
            (Value::Kosong, Value::Kosong) => true,
            (Value::Daftar(a), Value::Daftar(b)) => Rc::ptr_eq(a, b),
            (Value::Kamus(a), Value::Kamus(b)) => Rc::ptr_eq(a, b),
            (Value::Fungsi { chunk: a, .. }, Value::Fungsi { chunk: b, .. }) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}
