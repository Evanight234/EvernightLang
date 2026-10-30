use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

/// Struktur PipaKerja (Pipeline) untuk merangkai transformasi data & pemodelan
#[derive(Debug, Clone, PartialEq)]
pub struct PipaKerja {
    pub tahapan: Vec<(String, Value)>,
}

impl PipaKerja {
    pub fn new(langkah_awal: Vec<(String, Value)>) -> Self {
        Self {
            tahapan: langkah_awal,
        }
    }

    /// Menambahkan satu tahap prapemrosesan atau model baru ke dalam pipa
    pub fn tambah(&mut self, nama: String, langkah: Value) {
        self.tahapan.push((nama, langkah));
    }

    /// Melatih (fit) semua tahapan secara berurutan pada data latih
    pub fn latih(&mut self, x_val: &Value, y_val: &Value) -> Result<(), String> {
        let mut current_x = match x_val {
            Value::Larik(arr) => arr.borrow().clone(),
            _ => return Err("Input X untuk PipaKerja harus berupa MatraArray".to_string()),
        };

        let targets: Option<Vec<f64>> = match y_val {
            Value::Larik(l) => Some(l.borrow().data.clone()),
            Value::Daftar(d) => {
                let mut v = Vec::new();
                for item in d.borrow().iter() {
                    if let Value::Angka(n) = item {
                        v.push(*n);
                    }
                }
                Some(v)
            }
            _ => None,
        };

        let n = self.tahapan.len();
        for (i, (_nama, step)) in self.tahapan.iter_mut().enumerate() {
            match step {
                Value::Transformer(t_rc) => {
                    let mut trans = t_rc.borrow_mut();
                    trans.sesuaikan(&current_x)?;
                    if i < n - 1 {
                        current_x = trans.transformasi(&current_x)?;
                    }
                }
                Value::Model(m_rc) => {
                    let mut model = m_rc.borrow_mut();
                    model.latih(&current_x, targets.as_deref())?;
                }
                _ => {}
            }
        }
        Ok(())
    }

    /// Mentransformasi data baru (misal data uji) menggunakan parameter latih tanpa kebocoran data
    pub fn transformasi(&self, x_val: &Value) -> Result<Value, String> {
        let mut current_x = match x_val {
            Value::Larik(arr) => arr.borrow().clone(),
            _ => return Err("Input X harus berupa MatraArray".to_string()),
        };

        for (_nama, step) in &self.tahapan {
            match step {
                Value::Transformer(t_rc) => {
                    let trans = t_rc.borrow();
                    current_x = trans.transformasi(&current_x)?;
                }
                _ => {}
            }
        }

        Ok(Value::Larik(Rc::new(RefCell::new(current_x))))
    }

    /// Melakukan prediksi akhir melalui tahapan pipeline (prapemrosesan + model)
    pub fn prediksi(&self, x_val: &Value) -> Result<Value, String> {
        let mut current_x = match x_val {
            Value::Larik(arr) => arr.borrow().clone(),
            _ => return Err("Input X harus berupa MatraArray".to_string()),
        };

        let n = self.tahapan.len();
        for (i, (_nama, step)) in self.tahapan.iter().enumerate() {
            match step {
                Value::Transformer(t_rc) => {
                    let trans = t_rc.borrow();
                    current_x = trans.transformasi(&current_x)?;
                }
                Value::Model(m_rc) => {
                    if i == n - 1 {
                        let res = m_rc.borrow().prediksi(&current_x)?;
                        return Ok(Value::Larik(Rc::new(RefCell::new(res))));
                    }
                }
                _ => {}
            }
        }

        Ok(Value::Larik(Rc::new(RefCell::new(current_x))))
    }
}
