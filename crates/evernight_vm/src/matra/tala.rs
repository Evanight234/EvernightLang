use crate::matra::latih::AcakLcg;
use crate::value::Value;
use std::collections::HashMap;

/// Generator kombinasi Cartesian Product dari ruang parameter
pub fn bangun_kisi_parameter(ruang: &HashMap<String, Vec<Value>>) -> Vec<HashMap<String, Value>> {
    let keys: Vec<String> = ruang.keys().cloned().collect();
    if keys.is_empty() {
        return Vec::new();
    }

    let mut hasil: Vec<HashMap<String, Value>> = vec![HashMap::new()];

    for key in &keys {
        let values = &ruang[key];
        let mut kombinasi_baru = Vec::new();

        for combo in &hasil {
            for val in values {
                let mut c = combo.clone();
                c.insert(key.clone(), val.clone());
                kombinasi_baru.push(c);
            }
        }

        hasil = kombinasi_baru;
    }

    hasil
}

/// Struktur Pencarian Kisi (Grid Search) Hyperparameter
#[derive(Debug, Clone)]
pub struct PencarianKisi {
    pub ruang_parameter: HashMap<String, Vec<Value>>,
    pub parameter_terbaik: HashMap<String, Value>,
    pub skor_terbaik: f64,
    pub riwayat_skor: Vec<(HashMap<String, Value>, f64)>,
}

impl PencarianKisi {
    pub fn new(ruang: HashMap<String, Vec<Value>>) -> Self {
        Self {
            ruang_parameter: ruang,
            parameter_terbaik: HashMap::new(),
            skor_terbaik: f64::NEG_INFINITY,
            riwayat_skor: Vec::new(),
        }
    }

    /// Menghasilkan seluruh daftar kombinasi parameter
    pub fn daftar_kombinasi(&self) -> Vec<HashMap<String, Value>> {
        bangun_kisi_parameter(&self.ruang_parameter)
    }

    /// Mencatat skor hasil evaluasi untuk suatu kombinasi parameter
    pub fn catat_hasil(&mut self, parameter: HashMap<String, Value>, skor: f64) {
        if skor > self.skor_terbaik {
            self.skor_terbaik = skor;
            self.parameter_terbaik = parameter.clone();
        }
        self.riwayat_skor.push((parameter, skor));
    }
}

/// Struktur TalaBayes (Bayesian / Adaptive Hyperparameter Tuning)
#[derive(Debug, Clone)]
pub struct TalaBayes {
    pub ruang_parameter: HashMap<String, Vec<Value>>,
    pub iterasi: usize,
    pub parameter_terbaik: HashMap<String, Value>,
    pub skor_terbaik: f64,
    pub riwayat: Vec<(HashMap<String, Value>, f64)>,
    pub seed: Option<u64>,
}

impl TalaBayes {
    pub fn new(ruang: HashMap<String, Vec<Value>>, iterasi: usize, seed: Option<u64>) -> Self {
        Self {
            ruang_parameter: ruang,
            iterasi: iterasi.max(1),
            parameter_terbaik: HashMap::new(),
            skor_terbaik: f64::NEG_INFINITY,
            riwayat: Vec::new(),
            seed,
        }
    }

    /// Memilih kandidat kombinasi parameter berikutnya menggunakan strategi eksplorasi-eksploitasi
    pub fn saran_parameter(&self) -> HashMap<String, Value> {
        let mut rng = AcakLcg::new(self.seed.unwrap_or(42) + self.riwayat.len() as u64 * 31);
        let mut saran = HashMap::new();

        for (k, vals) in &self.ruang_parameter {
            if !vals.is_empty() {
                // Jika sudah ada riwayat dan acak < 0.7, condong ke nilai di sekitar yang terbaik (eksploitasi)
                if !self.parameter_terbaik.is_empty() && rng.next_f64() < 0.65 {
                    if let Some(best_val) = self.parameter_terbaik.get(k) {
                        saran.insert(k.clone(), best_val.clone());
                        continue;
                    }
                }
                // Eksplorasi acak
                let idx = (rng.next_u64() as usize) % vals.len();
                saran.insert(k.clone(), vals[idx].clone());
            }
        }

        saran
    }

    /// Memperbarui model dengan skor yang didapat
    pub fn perbarui(&mut self, parameter: HashMap<String, Value>, skor: f64) {
        if skor > self.skor_terbaik {
            self.skor_terbaik = skor;
            self.parameter_terbaik = parameter.clone();
        }
        self.riwayat.push((parameter, skor));
    }
}
