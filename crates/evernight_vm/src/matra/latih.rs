use crate::matra::larik::MatraArray;
use crate::matra::tabel::MatraTabel;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Generator bilangan acak deterministik berbasis Linear Congruential Generator (LCG)
pub struct AcakLcg {
    state: u64,
}

impl AcakLcg {
    pub fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x853c49e6748fea9b } else { seed },
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.state
    }

    pub fn next_f64(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / ((1u64 << 53) as f64)
    }

    /// Melakukan acak susunan urutan indeks (Fisher-Yates Shuffle)
    pub fn acak_indeks(&mut self, list: &mut [usize]) {
        let n = list.len();
        for i in (1..n).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            list.swap(i, j);
        }
    }
}

/// Memisahkan dataset X dan label target y menjadi himpunan latih dan uji.
/// Mendukung opsi rasio uji, pengacakan berbasis seed, dan stratifikasi proporsional.
pub fn pisah_data(
    x_val: &Value,
    y_val: &Value,
    rasio_uji: f64,
    acak: Option<u64>,
    stratifikasi: bool,
) -> Result<(Value, Value, Value, Value), String> {
    if rasio_uji <= 0.0 || rasio_uji >= 1.0 {
        return Err(
            "Rasio uji harus berada di antara 0.0 dan 1.0 (misal 0.2 untuk 20%)".to_string(),
        );
    }

    let n_samples = match x_val {
        Value::Larik(arr) => {
            let a = arr.borrow();
            if a.bentuk.is_empty() {
                0
            } else {
                a.bentuk[0]
            }
        }
        Value::Tabel(tbl) => tbl.borrow().jumlah_baris(),
        Value::Daftar(list) => list.borrow().len(),
        _ => return Err("Fitur X harus berupa Larik, Tabel, atau Daftar".to_string()),
    };

    if n_samples == 0 {
        return Err("Data X tidak boleh kosong".to_string());
    }

    let y_len = match y_val {
        Value::Larik(arr) => arr.borrow().data.len(),
        Value::Daftar(list) => list.borrow().len(),
        _ => return Err("Target y harus berupa Larik atau Daftar".to_string()),
    };

    if n_samples != y_len {
        return Err(format!(
            "Jumlah baris fitur X ({}) tidak sama dengan jumlah label target y ({})",
            n_samples, y_len
        ));
    }

    let seed = acak.unwrap_or(42);
    let mut rng = AcakLcg::new(seed);

    let (train_indices, test_indices) = if stratifikasi {
        // Ekstrak label target sebagai string untuk pengelompokan kelas
        let mut target_labels = Vec::with_capacity(n_samples);
        match y_val {
            Value::Larik(arr) => {
                for &val in &arr.borrow().data {
                    target_labels.push(format!("{:.0}", val));
                }
            }
            Value::Daftar(list) => {
                for item in list.borrow().iter() {
                    target_labels.push(item.to_string());
                }
            }
            _ => unreachable!(),
        }

        // Kelompokkan indeks berdasarkan kelas target
        let mut grup_kelas: HashMap<String, Vec<usize>> = HashMap::new();
        for (i, label) in target_labels.into_iter().enumerate() {
            grup_kelas.entry(label).or_default().push(i);
        }

        let mut train_idx = Vec::new();
        let mut test_idx = Vec::new();

        // Ambil sampel secara proporsional dari masing-masing kelas
        for (_kelas, mut indices) in grup_kelas {
            rng.acak_indeks(&mut indices);
            let n_kelas = indices.len();
            let n_test_kelas = ((n_kelas as f64 * rasio_uji).round() as usize)
                .max(1)
                .min(n_kelas.saturating_sub(1));

            test_idx.extend_from_slice(&indices[..n_test_kelas]);
            train_idx.extend_from_slice(&indices[n_test_kelas..]);
        }

        rng.acak_indeks(&mut train_idx);
        rng.acak_indeks(&mut test_idx);
        (train_idx, test_idx)
    } else {
        let mut all_indices: Vec<usize> = (0..n_samples).collect();
        rng.acak_indeks(&mut all_indices);

        let n_test = ((n_samples as f64 * rasio_uji).round() as usize)
            .max(1)
            .min(n_samples - 1);
        let test_idx = all_indices[..n_test].to_vec();
        let train_idx = all_indices[n_test..].to_vec();
        (train_idx, test_idx)
    };

    let x_latih = ambil_sub_x(x_val, &train_indices)?;
    let x_uji = ambil_sub_x(x_val, &test_indices)?;
    let y_latih = ambil_sub_y(y_val, &train_indices)?;
    let y_uji = ambil_sub_y(y_val, &test_indices)?;

    Ok((x_latih, x_uji, y_latih, y_uji))
}

fn ambil_sub_x(x: &Value, indices: &[usize]) -> Result<Value, String> {
    match x {
        Value::Larik(arr) => {
            let a = arr.borrow();
            if a.bentuk.len() != 2 {
                return Err("MatraArray untuk fitur X harus berbentuk matriks 2D".to_string());
            }
            let cols = a.bentuk[1];
            let rows_out = indices.len();
            let mut out = Vec::with_capacity(rows_out * cols);
            for &r in indices {
                let start = r * cols;
                out.extend_from_slice(&a.data[start..start + cols]);
            }
            Ok(Value::Larik(Rc::new(RefCell::new(
                MatraArray::dari_vektor_2d(out, rows_out, cols),
            ))))
        }
        Value::Tabel(tbl) => {
            let t = tbl.borrow();
            let mut kolom_baru = Vec::with_capacity(t.kolom.len());
            for col in &t.kolom {
                let mut c_out = Vec::with_capacity(indices.len());
                for &r in indices {
                    c_out.push(col[r].clone());
                }
                kolom_baru.push(c_out);
            }
            let res = MatraTabel {
                nama_kolom: t.nama_kolom.clone(),
                kolom: kolom_baru,
            };
            Ok(Value::Tabel(Rc::new(RefCell::new(res))))
        }
        Value::Daftar(list) => {
            let l = list.borrow();
            let mut out = Vec::with_capacity(indices.len());
            for &r in indices {
                out.push(l[r].clone());
            }
            Ok(Value::Daftar(Rc::new(RefCell::new(out))))
        }
        _ => unreachable!(),
    }
}

fn ambil_sub_y(y: &Value, indices: &[usize]) -> Result<Value, String> {
    match y {
        Value::Larik(arr) => {
            let a = arr.borrow();
            let mut out = Vec::with_capacity(indices.len());
            for &r in indices {
                out.push(a.data[r]);
            }
            let len = out.len();
            Ok(Value::Larik(Rc::new(RefCell::new(
                MatraArray::dari_vektor(out, vec![len]),
            ))))
        }
        Value::Daftar(list) => {
            let l = list.borrow();
            let mut out = Vec::with_capacity(indices.len());
            for &r in indices {
                out.push(l[r].clone());
            }
            Ok(Value::Daftar(Rc::new(RefCell::new(out))))
        }
        _ => unreachable!(),
    }
}

/// Struktur pembuat K-Fold Cross Validation
#[derive(Debug, Clone)]
pub struct KLipat {
    pub k: usize,
    pub acak_baris: bool,
    pub seed: Option<u64>,
}

impl KLipat {
    pub fn new(k: usize, acak_baris: bool, seed: Option<u64>) -> Result<Self, String> {
        if k < 2 {
            return Err("Nilai K pada K-Fold minimal harus 2".to_string());
        }
        Ok(Self {
            k,
            acak_baris,
            seed,
        })
    }

    /// Menghasilkan pasangan indeks (train_indices, val_indices) untuk setiap fold
    pub fn split(&self, n_samples: usize) -> Vec<(Vec<usize>, Vec<usize>)> {
        let mut indices: Vec<usize> = (0..n_samples).collect();
        if self.acak_baris {
            let mut rng = AcakLcg::new(self.seed.unwrap_or(42));
            rng.acak_indeks(&mut indices);
        }

        let fold_size = n_samples / self.k;
        let sisa = n_samples % self.k;

        let mut out = Vec::with_capacity(self.k);
        let mut current_start = 0;

        for i in 0..self.k {
            let current_fold_len = fold_size + if i < sisa { 1 } else { 0 };
            let current_end = current_start + current_fold_len;

            let val_idx = indices[current_start..current_end].to_vec();
            let mut train_idx = Vec::with_capacity(n_samples - current_fold_len);
            train_idx.extend_from_slice(&indices[..current_start]);
            train_idx.extend_from_slice(&indices[current_end..]);

            out.push((train_idx, val_idx));
            current_start = current_end;
        }

        out
    }
}
