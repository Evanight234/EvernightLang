use crate::matra::MatraArray;
use std::collections::HashMap;

/// Jenis transformer prapemrosesan data
#[derive(Debug, Clone, PartialEq)]
pub enum JenisTransformer {
    Standarisasi,
    PenskalaMinMaks,
    PengekodLabel,
}

/// Struktur transformer prapemrosesan fitur numerik & label
#[derive(Debug, Clone, PartialEq)]
pub struct MatraTransformer {
    pub jenis: JenisTransformer,
    pub mean: Vec<f64>,
    pub std: Vec<f64>,
    pub min: Vec<f64>,
    pub max: Vec<f64>,
    pub rentang_bawah: f64,
    pub rentang_atas: f64,
    pub label_ke_id: HashMap<String, f64>,
    pub id_ke_label: HashMap<i64, String>,
    pub fitted: bool,
}

impl MatraTransformer {
    /// Membuat objek Standarisasi (Z-Score Scaler: mean = 0, std = 1)
    pub fn standarisasi() -> Self {
        Self {
            jenis: JenisTransformer::Standarisasi,
            mean: Vec::new(),
            std: Vec::new(),
            min: Vec::new(),
            max: Vec::new(),
            rentang_bawah: 0.0,
            rentang_atas: 1.0,
            label_ke_id: HashMap::new(),
            id_ke_label: HashMap::new(),
            fitted: false,
        }
    }

    /// Membuat objek PenskalaMinMaks (skala fitur ke rentang [min, max], bawaan [0, 1])
    pub fn min_maks(bawah: f64, atas: f64) -> Self {
        Self {
            jenis: JenisTransformer::PenskalaMinMaks,
            mean: Vec::new(),
            std: Vec::new(),
            min: Vec::new(),
            max: Vec::new(),
            rentang_bawah: bawah,
            rentang_atas: atas,
            label_ke_id: HashMap::new(),
            id_ke_label: HashMap::new(),
            fitted: false,
        }
    }

    /// Membuat objek PengekodLabel (mengubah teks kategori ke id numerik 0, 1, 2..)
    pub fn pengekod_label() -> Self {
        Self {
            jenis: JenisTransformer::PengekodLabel,
            mean: Vec::new(),
            std: Vec::new(),
            min: Vec::new(),
            max: Vec::new(),
            rentang_bawah: 0.0,
            rentang_atas: 1.0,
            label_ke_id: HashMap::new(),
            id_ke_label: HashMap::new(),
            fitted: false,
        }
    }

    /// Melatih parameter prapemrosesan (menghitung mean, std, atau min, max) dari data X
    pub fn sesuaikan(&mut self, data: &MatraArray) -> Result<(), String> {
        if data.bentuk.len() != 2 {
            return Err("Transformer prapemrosesan membutuhkan data matriks 2D".to_string());
        }
        let rows = data.bentuk[0];
        let cols = data.bentuk[1];
        if rows == 0 || cols == 0 {
            return Err("Data matriks tidak boleh kosong".to_string());
        }

        match self.jenis {
            JenisTransformer::Standarisasi => {
                let mut means = vec![0.0_f64; cols];
                let mut stds = vec![0.0_f64; cols];

                // Hitung rata-rata per kolom
                for c in 0..cols {
                    let mut sum = 0.0;
                    for r in 0..rows {
                        sum += data.data[r * cols + c];
                    }
                    means[c] = sum / rows as f64;
                }

                // Hitung standar deviasi per kolom
                for c in 0..cols {
                    let m = means[c];
                    let mut var_sum = 0.0;
                    for r in 0..rows {
                        let diff = data.data[r * cols + c] - m;
                        var_sum += diff * diff;
                    }
                    let variance = var_sum / rows as f64;
                    stds[c] = if variance <= 1e-12 { 1.0 } else { variance.sqrt() };
                }

                self.mean = means;
                self.std = stds;
                self.fitted = true;
                Ok(())
            }
            JenisTransformer::PenskalaMinMaks => {
                let mut mins = vec![f64::INFINITY; cols];
                let mut maxs = vec![f64::NEG_INFINITY; cols];

                for r in 0..rows {
                    for c in 0..cols {
                        let val = data.data[r * cols + c];
                        if val < mins[c] {
                            mins[c] = val;
                        }
                        if val > maxs[c] {
                            maxs[c] = val;
                        }
                    }
                }

                // Hindari pembagian nol bila kolom bernilai konstan
                for c in 0..cols {
                    if mins[c] == maxs[c] {
                        maxs[c] += 1.0;
                    }
                }

                self.min = mins;
                self.max = maxs;
                self.fitted = true;
                Ok(())
            }
            JenisTransformer::PengekodLabel => {
                Err("PengekodLabel harus dilatih menggunakan daftar teks kategori".to_string())
            }
        }
    }

    /// Melatih PengekodLabel dari daftar teks kategori
    pub fn sesuaikan_label(&mut self, label_list: &[String]) -> Result<(), String> {
        let mut map = HashMap::new();
        let mut inv_map = HashMap::new();
        let mut current_id = 0.0_f64;

        for item in label_list {
            if !map.contains_key(item) {
                map.insert(item.clone(), current_id);
                inv_map.insert(current_id as i64, item.clone());
                current_id += 1.0;
            }
        }

        self.label_ke_id = map;
        self.id_ke_label = inv_map;
        self.fitted = true;
        Ok(())
    }

    /// Mentransformasi data X menggunakan parameter yang telah dipelajari
    pub fn transformasi(&self, data: &MatraArray) -> Result<MatraArray, String> {
        if !self.fitted {
            return Err("Transformer belum dilatih (panggil .sesuaikan(X) terlebih dahulu)".to_string());
        }
        if data.bentuk.len() != 2 {
            return Err("Data matriks harus 2D".to_string());
        }
        let rows = data.bentuk[0];
        let cols = data.bentuk[1];

        match self.jenis {
            JenisTransformer::Standarisasi => {
                if cols != self.mean.len() {
                    return Err(format!(
                        "Jumlah kolom data ({}) berbeda dengan parameter latih ({})",
                        cols,
                        self.mean.len()
                    ));
                }
                let mut out = Vec::with_capacity(rows * cols);
                for r in 0..rows {
                    for c in 0..cols {
                        let val = data.data[r * cols + c];
                        let z = (val - self.mean[c]) / self.std[c];
                        out.push(z);
                    }
                }
                Ok(MatraArray::dari_vektor_2d(out, rows, cols))
            }
            JenisTransformer::PenskalaMinMaks => {
                if cols != self.min.len() {
                    return Err(format!(
                        "Jumlah kolom data ({}) berbeda dengan parameter latih ({})",
                        cols,
                        self.min.len()
                    ));
                }
                let mut out = Vec::with_capacity(rows * cols);
                let target_range = self.rentang_atas - self.rentang_bawah;
                for r in 0..rows {
                    for c in 0..cols {
                        let val = data.data[r * cols + c];
                        let norm = (val - self.min[c]) / (self.max[c] - self.min[c]);
                        let scaled = self.rentang_bawah + norm * target_range;
                        out.push(scaled);
                    }
                }
                Ok(MatraArray::dari_vektor_2d(out, rows, cols))
            }
            JenisTransformer::PengekodLabel => {
                Err("Gunakan .transformasi_label untuk PengekodLabel".to_string())
            }
        }
    }

    /// Mentransformasi daftar label teks menjadi array float
    pub fn transformasi_label(&self, label_list: &[String]) -> Result<MatraArray, String> {
        if !self.fitted {
            return Err("PengekodLabel belum dilatih".to_string());
        }
        let mut ids = Vec::with_capacity(label_list.len());
        for l in label_list {
            if let Some(&id) = self.label_ke_id.get(l) {
                ids.push(id);
            } else {
                return Err(format!("Label kategori baru '{}' tidak ditemukan saat fitting", l));
            }
        }
        let len = ids.len();
        Ok(MatraArray::dari_vektor(ids, vec![len]))
    }

    /// Mengembalikan id numerik kembali ke teks label aslinya
    pub fn invers_transformasi_label(&self, ids: &[f64]) -> Result<Vec<String>, String> {
        if !self.fitted {
            return Err("PengekodLabel belum dilatih".to_string());
        }
        let mut out = Vec::with_capacity(ids.len());
        for &id in ids {
            let key = id.round() as i64;
            if let Some(lbl) = self.id_ke_label.get(&key) {
                out.push(lbl.clone());
            } else {
                return Err(format!("ID label '{}' tidak ditemukan pada kamus encoder", id));
            }
        }
        Ok(out)
    }

    /// Mengembalikan data yang telah ditransformasi kembali ke skala aslinya
    pub fn invers_transformasi(&self, data: &MatraArray) -> Result<MatraArray, String> {
        if !self.fitted {
            return Err("Transformer belum dilatih".to_string());
        }
        if data.bentuk.len() != 2 {
            return Err("Data matriks harus 2D".to_string());
        }
        let rows = data.bentuk[0];
        let cols = data.bentuk[1];

        match self.jenis {
            JenisTransformer::Standarisasi => {
                let mut out = Vec::with_capacity(rows * cols);
                for r in 0..rows {
                    for c in 0..cols {
                        let z = data.data[r * cols + c];
                        let orig = z * self.std[c] + self.mean[c];
                        out.push(orig);
                    }
                }
                Ok(MatraArray::dari_vektor_2d(out, rows, cols))
            }
            JenisTransformer::PenskalaMinMaks => {
                let mut out = Vec::with_capacity(rows * cols);
                let target_range = self.rentang_atas - self.rentang_bawah;
                for r in 0..rows {
                    for c in 0..cols {
                        let scaled = data.data[r * cols + c];
                        let norm = (scaled - self.rentang_bawah) / target_range;
                        let orig = self.min[c] + norm * (self.max[c] - self.min[c]);
                        out.push(orig);
                    }
                }
                Ok(MatraArray::dari_vektor_2d(out, rows, cols))
            }
            JenisTransformer::PengekodLabel => {
                Err("Gunakan invers_transformasi_label untuk PengekodLabel".to_string())
            }
        }
    }
}
