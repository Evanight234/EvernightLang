use crate::matra::larik::MatraArray;
use crate::matra::latih::AcakLcg;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Algoritma penyeimbang data SMOTE (Synthetic Minority Over-sampling Technique)
pub struct Smote {
    pub k_tetangga: usize,
    pub rasio: f64,
    pub seed: Option<u64>,
}

impl Smote {
    pub fn new(k_tetangga: usize, rasio: f64, seed: Option<u64>) -> Result<Self, String> {
        if k_tetangga == 0 {
            return Err("k_tetangga pada SMOTE minimal harus 1".to_string());
        }
        if rasio <= 0.0 || rasio > 1.0 {
            return Err("Rasio penyeimbangan SMOTE harus berada di rentang (0.0, 1.0]".to_string());
        }
        Ok(Self {
            k_tetangga,
            rasio,
            seed,
        })
    }

    /// Melakukan oversampling sintetis pada kelas minoritas
    pub fn sesuaikan(
        &self,
        x_val: &Value,
        y_val: &Value,
    ) -> Result<(Value, Value), String> {
        let (x_data, rows, cols) = match x_val {
            Value::Larik(arr) => {
                let a = arr.borrow();
                if a.bentuk.len() != 2 {
                    return Err("Fitur X untuk SMOTE harus berupa matriks 2D".to_string());
                }
                (a.data.clone(), a.bentuk[0], a.bentuk[1])
            }
            _ => return Err("Fitur X harus berupa MatraArray".to_string()),
        };

        let y_data = match y_val {
            Value::Larik(arr) => arr.borrow().data.clone(),
            Value::Daftar(list) => {
                let mut out = Vec::new();
                for item in list.borrow().iter() {
                    match item {
                        Value::Angka(n) => out.push(*n),
                        _ => return Err("Label target y untuk SMOTE harus berupa angka".to_string()),
                    }
                }
                out
            }
            _ => return Err("Target y harus berupa Larik atau Daftar angka".to_string()),
        };

        if rows != y_data.len() {
            return Err(format!("Ukuran X ({}) tidak sesuai dengan target y ({})", rows, y_data.len()));
        }

        // Hitung frekuensi tiap kelas
        let mut kelas_indices: HashMap<i64, Vec<usize>> = HashMap::new();
        for (i, &y) in y_data.iter().enumerate() {
            kelas_indices.entry(y as i64).or_default().push(i);
        }

        if kelas_indices.len() < 2 {
            return Err("SMOTE membutuhkan minimal 2 kelas yang berbeda".to_string());
        }

        let max_count = kelas_indices.values().map(|v| v.len()).max().unwrap_or(0);
        let mut rng = AcakLcg::new(self.seed.unwrap_or(42));

        let mut out_x = x_data.clone();
        let mut out_y = y_data.clone();

        for (&kelas, indices) in &kelas_indices {
            let count = indices.len();
            let target_count = (max_count as f64 * self.rasio).round() as usize;

            if count < target_count {
                let n_sintetis = target_count - count;
                let k = self.k_tetangga.min(count.saturating_sub(1));

                if k == 0 {
                    // Jika hanya ada 1 sampel minoritas, duplikasi langsung
                    for _ in 0..n_sintetis {
                        let idx = indices[0];
                        let row_start = idx * cols;
                        out_x.extend_from_slice(&x_data[row_start..row_start + cols]);
                        out_y.push(kelas as f64);
                    }
                    continue;
                }

                // Pra-hitung k-tetangga terdekat untuk setiap titik minoritas
                let mut k_neighbors: Vec<Vec<usize>> = Vec::with_capacity(count);
                for i in 0..count {
                    let idx_i = indices[i];
                    let mut distances: Vec<(f64, usize)> = Vec::with_capacity(count - 1);
                    for j in 0..count {
                        if i == j {
                            continue;
                        }
                        let idx_j = indices[j];
                        let mut dist_sq = 0.0;
                        for c in 0..cols {
                            let diff = x_data[idx_i * cols + c] - x_data[idx_j * cols + c];
                            dist_sq += diff * diff;
                        }
                        distances.push((dist_sq, idx_j));
                    }
                    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    let nn: Vec<usize> = distances.iter().take(k).map(|item| item.1).collect();
                    k_neighbors.push(nn);
                }

                // Buat titik-titik sintetis
                for _ in 0..n_sintetis {
                    let rand_i = (rng.next_u64() as usize) % count;
                    let idx_base = indices[rand_i];
                    let neighbors = &k_neighbors[rand_i];
                    let rand_nn_idx = (rng.next_u64() as usize) % neighbors.len();
                    let idx_neighbor = neighbors[rand_nn_idx];

                    let lambda = rng.next_f64(); // faktor interpolasi [0, 1]
                    for c in 0..cols {
                        let val_base = x_data[idx_base * cols + c];
                        let val_nn = x_data[idx_neighbor * cols + c];
                        let synthetic_val = val_base + lambda * (val_nn - val_base);
                        out_x.push(synthetic_val);
                    }
                    out_y.push(kelas as f64);
                }
            }
        }

        let new_rows = out_y.len();
        let arr_x = MatraArray::dari_vektor_2d(out_x, new_rows, cols);
        let arr_y = MatraArray::dari_vektor(out_y, vec![new_rows]);

        Ok((
            Value::Larik(Rc::new(RefCell::new(arr_x))),
            Value::Larik(Rc::new(RefCell::new(arr_y))),
        ))
    }
}
