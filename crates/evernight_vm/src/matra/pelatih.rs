use crate::matra::larik::MatraArray;
use crate::matra::syaraf::{MatraJaringan, MatraLoss, MatraOptimizer};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;

/// Generator acak sederhana untuk shuffling indeks mini-batch
struct BatchRng {
    state: u64,
}

impl BatchRng {
    fn new(seed: u64) -> Self {
        Self {
            state: if seed == 0 { 0x4d595df4d0f33173 } else { seed },
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        self.state
    }

    fn shuffle<T>(&mut self, slice: &mut [T]) {
        let n = slice.len();
        for i in (1..n).rev() {
            let j = (self.next_u64() as usize) % (i + 1);
            slice.swap(i, j);
        }
    }
}

/// Jenis-jenis Callback dalam siklus pelatihan
#[derive(Debug, Clone, PartialEq)]
pub enum JenisCallback {
    BerhentiAwal {
        pantau: String,
        kesabaran: usize,
        min_delta: f64,
        mode: String,
        terbaik: f64,
        hitung: usize,
        berhenti: bool,
    },
    SimpanTerbaik {
        berkas: String,
        pantau: String,
        mode: String,
        terbaik: f64,
    },
    PencatatRiwayat {
        berkas: Option<String>,
        log: Vec<HashMap<String, f64>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MatraCallback {
    pub jenis: JenisCallback,
}

impl MatraCallback {
    pub fn berhenti_awal(pantau: Option<&str>, kesabaran: Option<usize>, min_delta: Option<f64>, mode: Option<&str>) -> Self {
        let p = pantau.unwrap_or("rugi_val").to_string();
        let m = mode.unwrap_or_else(|| {
            if p.contains("akurasi") || p.contains("f1") || p.contains("skor") {
                "maks"
            } else {
                "min"
            }
        }).to_string();

        let initial_terbaik = if m == "maks" { f64::NEG_INFINITY } else { f64::INFINITY };

        Self {
            jenis: JenisCallback::BerhentiAwal {
                pantau: p,
                kesabaran: kesabaran.unwrap_or(5),
                min_delta: min_delta.unwrap_or(0.0001),
                mode: m,
                terbaik: initial_terbaik,
                hitung: 0,
                berhenti: false,
            },
        }
    }

    pub fn simpan_terbaik(berkas: &str, pantau: Option<&str>, mode: Option<&str>) -> Self {
        let p = pantau.unwrap_or("akurasi_val").to_string();
        let m = mode.unwrap_or_else(|| {
            if p.contains("akurasi") || p.contains("f1") || p.contains("skor") {
                "maks"
            } else {
                "min"
            }
        }).to_string();

        let initial_terbaik = if m == "maks" { f64::NEG_INFINITY } else { f64::INFINITY };

        Self {
            jenis: JenisCallback::SimpanTerbaik {
                berkas: berkas.to_string(),
                pantau: p,
                mode: m,
                terbaik: initial_terbaik,
            },
        }
    }

    pub fn pencatat_riwayat(berkas: Option<&str>) -> Self {
        Self {
            jenis: JenisCallback::PencatatRiwayat {
                berkas: berkas.map(|s| s.to_string()),
                log: Vec::new(),
            },
        }
    }

    /// Dieksekusi di akhir setiap epoch. Mengembalikan Ok(true) jika perlu menghentikan pelatihan.
    pub fn pada_akhir_putaran(
        &mut self,
        _putaran: usize,
        metrik: &HashMap<String, f64>,
        model: &mut MatraJaringan,
    ) -> Result<bool, String> {
        match &mut self.jenis {
            JenisCallback::BerhentiAwal { pantau, kesabaran, min_delta, mode, terbaik, hitung, berhenti } => {
                let nilai_sekarang = match metrik.get(pantau) {
                    Some(v) => *v,
                    None => return Ok(false),
                };

                let membaik = if mode == "maks" {
                    nilai_sekarang > (*terbaik + *min_delta)
                } else {
                    nilai_sekarang < (*terbaik - *min_delta)
                };

                if membaik {
                    *terbaik = nilai_sekarang;
                    *hitung = 0;
                } else {
                    *hitung += 1;
                    if *hitung >= *kesabaran {
                        *berhenti = true;
                        println!(
                            "   [BerhentiAwal] Pelatihan dihentikan karena metrik '{}' tidak membaik selama {} putaran berturut-turut.",
                            pantau, kesabaran
                        );
                        return Ok(true);
                    }
                }
                Ok(false)
            }

            JenisCallback::SimpanTerbaik { berkas, pantau, mode, terbaik } => {
                let nilai_sekarang = match metrik.get(pantau) {
                    Some(v) => *v,
                    None => return Ok(false),
                };

                let membaik = if mode == "maks" {
                    nilai_sekarang > *terbaik
                } else {
                    nilai_sekarang < *terbaik
                };

                if membaik {
                    *terbaik = nilai_sekarang;
                    model.simpan(berkas)?;
                }
                Ok(false)
            }

            JenisCallback::PencatatRiwayat { berkas, log } => {
                log.push(metrik.clone());
                if let Some(path) = berkas {
                    if let Ok(mut f) = File::create(path) {
                        let _ = writeln!(f, "{:?}", log);
                    }
                }
                Ok(false)
            }
        }
    }
}

/// Orkestrator Pelatihan Otomatis (Deep Learning Training Suite)
#[derive(Debug, Clone, PartialEq)]
pub struct MatraPelatih {
    pub model: Rc<RefCell<MatraJaringan>>,
    pub pengoptimal: Rc<RefCell<MatraOptimizer>>,
    pub kerugian: MatraLoss,
    pub metrik_nama: Vec<String>,
    pub callbacks: Vec<Rc<RefCell<MatraCallback>>>,
    pub riwayat: HashMap<String, Vec<f64>>,
}

impl MatraPelatih {
    pub fn new(
        model: Rc<RefCell<MatraJaringan>>,
        pengoptimal: Rc<RefCell<MatraOptimizer>>,
        kerugian: MatraLoss,
        metrik: Option<Vec<String>>,
    ) -> Self {
        Self {
            model,
            pengoptimal,
            kerugian,
            metrik_nama: metrik.unwrap_or_else(|| vec!["akurasi".to_string()]),
            callbacks: Vec::new(),
            riwayat: HashMap::new(),
        }
    }

    pub fn pasang_callback(&mut self, cb: Rc<RefCell<MatraCallback>>) {
        self.callbacks.push(cb);
    }

    /// Melatih model syaraf dengan batching dan pemantauan metrik otomatis
    pub fn latih(
        &mut self,
        x_latih: &MatraArray,
        y_latih: &MatraArray,
        putaran: usize,
        ukuran_batch: usize,
        x_val: Option<&MatraArray>,
        y_val: Option<&MatraArray>,
    ) -> Result<HashMap<String, Vec<f64>>, String> {
        let (n_sampel, n_fitur) = match x_latih.bentuk.as_slice() {
            [b, k] => (*b, *k),
            [k] => (1, *k),
            _ => return Err("Data latih X harus 1D atau 2D".to_string()),
        };

        let y_kolom = match y_latih.bentuk.as_slice() {
            [_, k] => *k,
            _ => 1,
        };

        let batch_size = if ukuran_batch == 0 || ukuran_batch > n_sampel {
            n_sampel
        } else {
            ukuran_batch
        };

        let mut rng = BatchRng::new(42);
        let mut indeks: Vec<usize> = (0..n_sampel).collect();

        // Inisialisasi riwayat
        self.riwayat.clear();
        self.riwayat.insert("putaran".to_string(), Vec::new());
        self.riwayat.insert("rugi".to_string(), Vec::new());
        if x_val.is_some() {
            self.riwayat.insert("rugi_val".to_string(), Vec::new());
        }
        for m in &self.metrik_nama {
            self.riwayat.insert(m.clone(), Vec::new());
            if x_val.is_some() {
                self.riwayat.insert(format!("{}_val", m), Vec::new());
            }
        }

        for epoch in 1..=putaran {
            // 1. Acak sampel per epoch
            rng.shuffle(&mut indeks);

            let mut total_loss = 0.0;
            let mut total_batches = 0;

            self.model.borrow_mut().mode_latih();

            // 2. Loop mini-batch
            let mut start_idx = 0;
            while start_idx < n_sampel {
                let end_idx = (start_idx + batch_size).min(n_sampel);
                let current_batch_size = end_idx - start_idx;

                // Ambil slice batch
                let mut batch_x_data = Vec::with_capacity(current_batch_size * n_fitur);
                let mut batch_y_data = Vec::with_capacity(current_batch_size * y_kolom);

                for i in start_idx..end_idx {
                    let sample_idx = indeks[i];
                    for f in 0..n_fitur {
                        batch_x_data.push(x_latih.data[sample_idx * n_fitur + f]);
                    }
                    for y_f in 0..y_kolom {
                        batch_y_data.push(y_latih.data[sample_idx * y_kolom + y_f]);
                    }
                }

                let batch_x = MatraArray::dari_vektor_2d(batch_x_data, current_batch_size, n_fitur);
                let batch_y = MatraArray::dari_vektor_2d(batch_y_data, current_batch_size, y_kolom);

                // Forward pass & loss
                self.pengoptimal.borrow_mut().nol_gradien()?;
                let pred = self.model.borrow_mut().maju(&batch_x)?;
                let loss = self.kerugian.hitung(&pred, &batch_y)?;
                total_loss += loss;
                total_batches += 1;

                // Backward pass & optimizer step
                let grad_loss = self.kerugian.gradien(&pred, &batch_y)?;
                self.model.borrow_mut().mundur(&grad_loss)?;
                self.pengoptimal.borrow_mut().langkah()?;

                start_idx = end_idx;
            }

            let avg_train_loss = if total_batches > 0 { total_loss / (total_batches as f64) } else { 0.0 };

            // 3. Evaluasi metrik latih & validasi
            let mut metrik_epoch = HashMap::new();
            metrik_epoch.insert("putaran".to_string(), epoch as f64);
            metrik_epoch.insert("rugi".to_string(), avg_train_loss);

            let train_acc = self.hitung_metrik_akurasi(x_latih, y_latih)?;
            metrik_epoch.insert("akurasi".to_string(), train_acc);

            let mut val_loss_opt = None;
            let mut val_acc_opt = None;

            if let (Some(xv), Some(yv)) = (x_val, y_val) {
                let res_val = self.evaluasi(xv, yv)?;
                if let Some(vl) = res_val.get("rugi") {
                    metrik_epoch.insert("rugi_val".to_string(), *vl);
                    val_loss_opt = Some(*vl);
                }
                if let Some(va) = res_val.get("akurasi") {
                    metrik_epoch.insert("akurasi_val".to_string(), *va);
                    val_acc_opt = Some(*va);
                }
            }

            // Catat ke riwayat
            for (k, v) in &metrik_epoch {
                if let Some(vec) = self.riwayat.get_mut(k) {
                    vec.push(*v);
                }
            }

            // Tampilkan kemajuan konsol setiap 5 putaran atau putaran akhir
            if epoch % 5 == 0 || epoch == 1 || epoch == putaran {
                let mut status = format!("Putaran [{}/{}] | Rugi: {:.4} | Akurasi: {:.2}%", epoch, putaran, avg_train_loss, train_acc * 100.0);
                if let (Some(vl), Some(va)) = (val_loss_opt, val_acc_opt) {
                    status.push_str(&format!(" | Rugi Val: {:.4} | Akurasi Val: {:.2}%", vl, va * 100.0));
                }
                println!("{}", status);
            }

            // 4. Panggil callbacks
            let mut harus_berhenti = false;
            let mut model_borrow = self.model.borrow_mut();
            for cb_rc in &self.callbacks {
                let mut cb = cb_rc.borrow_mut();
                if cb.pada_akhir_putaran(epoch, &metrik_epoch, &mut model_borrow)? {
                    harus_berhenti = true;
                }
            }

            if harus_berhenti {
                break;
            }
        }

        Ok(self.riwayat.clone())
    }

    /// Evaluasi model pada dataset terpisah
    pub fn evaluasi(&self, x: &MatraArray, y: &MatraArray) -> Result<HashMap<String, f64>, String> {
        let mut model_borrow = self.model.borrow_mut();
        let prev_mode = model_borrow.training;
        model_borrow.mode_evaluasi();

        let pred = model_borrow.maju(x)?;
        let loss = self.kerugian.hitung(&pred, y)?;
        model_borrow.training = prev_mode;

        let mut res = HashMap::new();
        res.insert("rugi".to_string(), loss);

        // Akurasi jika target diskrit / biner
        let acc = self.hitung_akurasi_dari_pred(&pred, y);
        res.insert("akurasi".to_string(), acc);

        Ok(res)
    }

    fn hitung_metrik_akurasi(&self, x: &MatraArray, y: &MatraArray) -> Result<f64, String> {
        let mut model_borrow = self.model.borrow_mut();
        let prev_mode = model_borrow.training;
        model_borrow.mode_evaluasi();
        let pred = model_borrow.maju(x)?;
        model_borrow.training = prev_mode;
        Ok(self.hitung_akurasi_dari_pred(&pred, y))
    }

    fn hitung_akurasi_dari_pred(&self, pred: &MatraArray, target: &MatraArray) -> f64 {
        let (baris, kolom) = match pred.bentuk.as_slice() {
            [b, k] => (*b, *k),
            [k] => (1, *k),
            _ => (1, pred.data.len()),
        };

        if kolom == 1 {
            // Binary classification (threshold 0.5)
            let mut benar = 0;
            for i in 0..baris {
                let p = if pred.data[i] >= 0.5 { 1.0 } else { 0.0 };
                let t = if target.data[i] >= 0.5 { 1.0 } else { 0.0 };
                if p == t {
                    benar += 1;
                }
            }
            (benar as f64) / (baris as f64)
        } else {
            // Multi-class classification (argmax)
            let mut benar = 0;
            for i in 0..baris {
                let mut best_pred_idx = 0;
                let mut max_pred = f64::NEG_INFINITY;
                for j in 0..kolom {
                    let v = pred.data[i * kolom + j];
                    if v > max_pred {
                        max_pred = v;
                        best_pred_idx = j;
                    }
                }

                let mut best_target_idx = 0;
                let mut max_target = f64::NEG_INFINITY;
                for j in 0..kolom {
                    let v = target.data[i * kolom + j];
                    if v > max_target {
                        max_target = v;
                        best_target_idx = j;
                    }
                }

                if best_pred_idx == best_target_idx {
                    benar += 1;
                }
            }
            (benar as f64) / (baris as f64)
        }
    }
}
