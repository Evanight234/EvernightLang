use crate::bytecode::Chunk;
use crate::matra::{
    MatraArray, MatraCallback, MatraJaringan, MatraLapisan, MatraLoss, MatraModel, MatraOptimizer,
    MatraPelatih, MatraPlot, MatraTabel, MatraTransformer, PipaKerja,
};
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
    Larik(Rc<RefCell<MatraArray>>),
    Tabel(Rc<RefCell<MatraTabel>>),
    Grafik(Rc<RefCell<MatraPlot>>),
    Transformer(Rc<RefCell<MatraTransformer>>),
    Pipa(Rc<RefCell<PipaKerja>>),
    Model(Rc<RefCell<MatraModel>>),
    ModelSyaraf(Rc<RefCell<MatraJaringan>>),
    Lapisan(Rc<RefCell<MatraLapisan>>),
    FungsiKerugian(Rc<RefCell<MatraLoss>>),
    Pengoptimal(Rc<RefCell<MatraOptimizer>>),
    Pelatih(Rc<RefCell<MatraPelatih>>),
    Callback(Rc<RefCell<MatraCallback>>),
    FungsiAsli {
        name: String,
        id: u16,
    },
    MetodeAsli {
        name: String,
        id: u16,
        target: Box<Value>,
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
            Value::Larik(l) => !l.borrow().data.is_empty(),
            Value::Tabel(t) => t.borrow().jumlah_baris() > 0,
            Value::Grafik(_) => true,
            Value::Transformer(_) => true,
            Value::Pipa(_) => true,
            Value::Model(_) => true,
            Value::ModelSyaraf(_)
            | Value::Lapisan(_)
            | Value::FungsiKerugian(_)
            | Value::Pengoptimal(_)
            | Value::Pelatih(_)
            | Value::Callback(_) => true,
            Value::FungsiAsli { .. } | Value::MetodeAsli { .. } => true,
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
            Value::Larik(l) => write!(f, "{}", l.borrow()),
            Value::Tabel(t) => write!(f, "{}", t.borrow()),
            Value::Grafik(g) => write!(f, "{}", g.borrow()),
            Value::Transformer(_) => write!(f, "<transformer pra>"),
            Value::Pipa(_) => write!(f, "<pipa alur kerja>"),
            Value::Model(m) => write!(f, "<model {}>", m.borrow().nama),
            Value::ModelSyaraf(_) => write!(f, "<model syaraf>"),
            Value::Lapisan(_) => write!(f, "<lapisan syaraf>"),
            Value::FungsiKerugian(_) => write!(f, "<fungsi kerugian>"),
            Value::Pengoptimal(_) => write!(f, "<pengoptimal>"),
            Value::Pelatih(_) => write!(f, "<pelatih model syaraf>"),
            Value::Callback(_) => write!(f, "<callback pelatih>"),
            Value::FungsiAsli { name, .. } => write!(f, "<fungsi asli {}>", name),
            Value::MetodeAsli { name, .. } => write!(f, "<metode {}>", name),
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
            (Value::Larik(a), Value::Larik(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Tabel(a), Value::Tabel(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Grafik(a), Value::Grafik(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Transformer(a), Value::Transformer(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::Pipa(a), Value::Pipa(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::Model(a), Value::Model(b)) => Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow(),
            (Value::ModelSyaraf(a), Value::ModelSyaraf(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::Lapisan(a), Value::Lapisan(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::FungsiKerugian(a), Value::FungsiKerugian(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::Pengoptimal(a), Value::Pengoptimal(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::Pelatih(a), Value::Pelatih(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::Callback(a), Value::Callback(b)) => {
                Rc::ptr_eq(a, b) || *a.borrow() == *b.borrow()
            }
            (Value::FungsiAsli { id: a, .. }, Value::FungsiAsli { id: b, .. }) => a == b,
            (
                Value::MetodeAsli {
                    id: a, target: t1, ..
                },
                Value::MetodeAsli {
                    id: b, target: t2, ..
                },
            ) => a == b && t1 == t2,
            _ => false,
        }
    }
}
