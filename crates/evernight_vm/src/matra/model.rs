use crate::matra::larik::MatraArray;
use crate::matra::latih::AcakLcg;
use std::collections::{HashMap, BTreeSet};

/// Node pohon keputusan untuk klasifikasi atau regresi
#[derive(Debug, Clone, PartialEq)]
pub struct NodePohon {
    pub indeks_fitur: usize,
    pub ambang: f64,
    pub kiri: Option<Box<NodePohon>>,
    pub kanan: Option<Box<NodePohon>>,
    pub nilai_daun: Option<f64>,
}

impl NodePohon {
    pub fn daun(nilai: f64) -> Self {
        Self {
            indeks_fitur: 0,
            ambang: 0.0,
            kiri: None,
            kanan: None,
            nilai_daun: Some(nilai),
        }
    }

    pub fn cabang(fitur: usize, ambang: f64, kiri: NodePohon, kanan: NodePohon) -> Self {
        Self {
            indeks_fitur: fitur,
            ambang,
            kiri: Some(Box::new(kiri)),
            kanan: Some(Box::new(kanan)),
            nilai_daun: None,
        }
    }

    pub fn prediksi_sampel(&self, sampel: &[f64]) -> f64 {
        if let Some(val) = self.nilai_daun {
            return val;
        }
        if sampel[self.indeks_fitur] <= self.ambang {
            if let Some(ref k) = self.kiri {
                return k.prediksi_sampel(sampel);
            }
        } else {
            if let Some(ref k) = self.kanan {
                return k.prediksi_sampel(sampel);
            }
        }
        0.0
    }
}

/// Ragam algoritma Machine Learning klasik
#[derive(Debug, Clone, PartialEq)]
pub enum JenisModel {
    RegresiLinear {
        bobot: Vec<f64>,
        intersep: f64,
        laju_belajar: f64,
        iterasi: usize,
    },
    RegresiLogistik {
        bobot_per_kelas: Vec<Vec<f64>>, // Bobot untuk masing-masing kelas (One-vs-Rest)
        intersep_per_kelas: Vec<f64>,
        kelas: Vec<f64>,
        laju_belajar: f64,
        iterasi: usize,
    },
    KNN {
        k: usize,
        metrik_jarak: String,
        data_x: Vec<f64>,
        data_y: Vec<f64>,
        rows: usize,
        cols: usize,
        is_klasifikasi: bool,
    },
    PohonKeputusan {
        root: Option<Box<NodePohon>>,
        kedalaman_maks: usize,
        min_sampel_pisah: usize,
        is_klasifikasi: bool,
    },
    HutanAcak {
        pohon_list: Vec<NodePohon>,
        jumlah_pohon: usize,
        kedalaman_maks: usize,
        min_sampel_pisah: usize,
        seed: u64,
        is_klasifikasi: bool,
    },
    KMeans {
        k: usize,
        iterasi_maks: usize,
        pusat_klaster: Vec<Vec<f64>>,
        inersia: f64,
        seed: u64,
    },
}

/// Struktur utama model Machine Learning Matra
#[derive(Debug, Clone, PartialEq)]
pub struct MatraModel {
    pub nama: String,
    pub jenis: JenisModel,
    pub fitted: bool,
}

impl MatraModel {
    /// Inisialisasi Regresi Linear
    pub fn regresi_linear(laju_belajar: Option<f64>, iterasi: Option<usize>) -> Self {
        Self {
            nama: "RegresiLinear".to_string(),
            jenis: JenisModel::RegresiLinear {
                bobot: Vec::new(),
                intersep: 0.0,
                laju_belajar: laju_belajar.unwrap_or(0.01),
                iterasi: iterasi.unwrap_or(1000),
            },
            fitted: false,
        }
    }

    /// Inisialisasi Regresi Logistik
    pub fn regresi_logistik(laju_belajar: Option<f64>, iterasi: Option<usize>) -> Self {
        Self {
            nama: "RegresiLogistik".to_string(),
            jenis: JenisModel::RegresiLogistik {
                bobot_per_kelas: Vec::new(),
                intersep_per_kelas: Vec::new(),
                kelas: Vec::new(),
                laju_belajar: laju_belajar.unwrap_or(0.05),
                iterasi: iterasi.unwrap_or(1000),
            },
            fitted: false,
        }
    }

    /// Inisialisasi K-Nearest Neighbors Klasifikasi
    pub fn knn_klasifikasi(k: usize, metrik_jarak: Option<String>) -> Self {
        Self {
            nama: "KNNKlasifikasi".to_string(),
            jenis: JenisModel::KNN {
                k: k.max(1),
                metrik_jarak: metrik_jarak.unwrap_or_else(|| "euclidean".to_string()),
                data_x: Vec::new(),
                data_y: Vec::new(),
                rows: 0,
                cols: 0,
                is_klasifikasi: true,
            },
            fitted: false,
        }
    }

    /// Inisialisasi K-Nearest Neighbors Regresi
    pub fn knn_regresi(k: usize, metrik_jarak: Option<String>) -> Self {
        Self {
            nama: "KNNRegresi".to_string(),
            jenis: JenisModel::KNN {
                k: k.max(1),
                metrik_jarak: metrik_jarak.unwrap_or_else(|| "euclidean".to_string()),
                data_x: Vec::new(),
                data_y: Vec::new(),
                rows: 0,
                cols: 0,
                is_klasifikasi: false,
            },
            fitted: false,
        }
    }

    /// Inisialisasi Pohon Keputusan
    pub fn pohon_keputusan(
        kedalaman_maks: Option<usize>,
        min_sampel: Option<usize>,
        klasifikasi: bool,
    ) -> Self {
        Self {
            nama: if klasifikasi {
                "PohonKeputusanKlasifikasi".to_string()
            } else {
                "PohonKeputusanRegresi".to_string()
            },
            jenis: JenisModel::PohonKeputusan {
                root: None,
                kedalaman_maks: kedalaman_maks.unwrap_or(10),
                min_sampel_pisah: min_sampel.unwrap_or(2),
                is_klasifikasi: klasifikasi,
            },
            fitted: false,
        }
    }

    /// Inisialisasi Hutan Acak (Random Forest)
    pub fn hutan_acak(
        jumlah_pohon: usize,
        kedalaman_maks: Option<usize>,
        seed: Option<u64>,
        klasifikasi: bool,
    ) -> Self {
        Self {
            nama: if klasifikasi {
                "HutanAcakKlasifikasi".to_string()
            } else {
                "HutanAcakRegresi".to_string()
            },
            jenis: JenisModel::HutanAcak {
                pohon_list: Vec::new(),
                jumlah_pohon: jumlah_pohon.max(1),
                kedalaman_maks: kedalaman_maks.unwrap_or(8),
                min_sampel_pisah: 2,
                seed: seed.unwrap_or(42),
                is_klasifikasi: klasifikasi,
            },
            fitted: false,
        }
    }

    /// Inisialisasi K-Means Clustering
    pub fn kmeans(k: usize, iterasi_maks: Option<usize>, seed: Option<u64>) -> Self {
        Self {
            nama: "KMeans".to_string(),
            jenis: JenisModel::KMeans {
                k: k.max(1),
                iterasi_maks: iterasi_maks.unwrap_or(300),
                pusat_klaster: Vec::new(),
                inersia: 0.0,
                seed: seed.unwrap_or(42),
            },
            fitted: false,
        }
    }

    // =========================================================================
    // PELATIHAN (FIT / LATIH)
    // =========================================================================

    /// Melatih model menggunakan matriks fitur X dan target y
    pub fn latih(&mut self, x: &MatraArray, y: Option<&[f64]>) -> Result<(), String> {
        if x.bentuk.len() != 2 {
            return Err("Matriks fitur X harus berupa 2 dimensi [sampel, fitur]".to_string());
        }
        let rows = x.bentuk[0];
        let cols = x.bentuk[1];
        if rows == 0 || cols == 0 {
            return Err("Data fitur X tidak boleh kosong".to_string());
        }

        match &mut self.jenis {
            JenisModel::RegresiLinear {
                bobot,
                intersep,
                laju_belajar,
                iterasi,
            } => {
                let targets = y.ok_or_else(|| "RegresiLinear membutuhkan label target y".to_string())?;
                if targets.len() != rows {
                    return Err(format!("Ukuran X ({}) dan y ({}) tidak cocok", rows, targets.len()));
                }

                // Inisialisasi bobot dan intersep nol
                *bobot = vec![0.0; cols];
                *intersep = targets.iter().sum::<f64>() / rows as f64;

                let alpha = *laju_belajar;
                let max_iter = *iterasi;

                // Gradient Descent optimasi OLS
                for _ in 0..max_iter {
                    let mut grad_w = vec![0.0; cols];
                    let mut grad_b = 0.0;

                    for r in 0..rows {
                        let row_start = r * cols;
                        let mut y_pred = *intersep;
                        for c in 0..cols {
                            y_pred += bobot[c] * x.data[row_start + c];
                        }
                        let diff = y_pred - targets[r];
                        grad_b += diff;
                        for c in 0..cols {
                            grad_w[c] += diff * x.data[row_start + c];
                        }
                    }

                    *intersep -= (alpha * grad_b) / rows as f64;
                    for c in 0..cols {
                        bobot[c] -= (alpha * grad_w[c]) / rows as f64;
                    }
                }
            }

            JenisModel::RegresiLogistik {
                bobot_per_kelas,
                intersep_per_kelas,
                kelas,
                laju_belajar,
                iterasi,
            } => {
                let targets = y.ok_or_else(|| "RegresiLogistik membutuhkan label target y".to_string())?;
                if targets.len() != rows {
                    return Err(format!("Ukuran X ({}) dan y ({}) tidak cocok", rows, targets.len()));
                }

                let mut set_kelas = BTreeSet::new();
                for &v in targets {
                    set_kelas.insert(v.round() as i64);
                }
                *kelas = set_kelas.into_iter().map(|c| c as f64).collect();

                bobot_per_kelas.clear();
                intersep_per_kelas.clear();

                let n_kelas = kelas.len();
                let alpha = *laju_belajar;
                let max_iter = *iterasi;

                // Jika biner (2 kelas), cukup 1 model; jika multi-kelas, latih One-vs-Rest
                let models_to_train = if n_kelas <= 2 { 1 } else { n_kelas };

                for k_idx in 0..models_to_train {
                    let target_k = kelas[k_idx];
                    let mut w = vec![0.0; cols];
                    let mut b = 0.0;

                    for _ in 0..max_iter {
                        let mut grad_w = vec![0.0; cols];
                        let mut grad_b = 0.0;

                        for r in 0..rows {
                            let row_start = r * cols;
                            let mut z = b;
                            for c in 0..cols {
                                z += w[c] * x.data[row_start + c];
                            }
                            // Sigmoid
                            let p = 1.0 / (1.0 + (-z).exp());
                            let y_true = if (targets[r] - target_k).abs() < 1e-6 { 1.0 } else { 0.0 };
                            let err = p - y_true;

                            grad_b += err;
                            for c in 0..cols {
                                grad_w[c] += err * x.data[row_start + c];
                            }
                        }

                        b -= (alpha * grad_b) / rows as f64;
                        for c in 0..cols {
                            w[c] -= (alpha * grad_w[c]) / rows as f64;
                        }
                    }

                    bobot_per_kelas.push(w);
                    intersep_per_kelas.push(b);
                }
            }

            JenisModel::KNN {
                data_x,
                data_y,
                rows: stored_rows,
                cols: stored_cols,
                ..
            } => {
                let targets = y.ok_or_else(|| "KNN membutuhkan label target y".to_string())?;
                if targets.len() != rows {
                    return Err(format!("Ukuran X ({}) dan y ({}) tidak cocok", rows, targets.len()));
                }

                *data_x = x.data.clone();
                *data_y = targets.to_vec();
                *stored_rows = rows;
                *stored_cols = cols;
            }

            JenisModel::PohonKeputusan {
                root,
                kedalaman_maks,
                min_sampel_pisah,
                is_klasifikasi,
            } => {
                let targets = y.ok_or_else(|| "PohonKeputusan membutuhkan label target y".to_string())?;
                if targets.len() != rows {
                    return Err(format!("Ukuran X ({}) dan y ({}) tidak cocok", rows, targets.len()));
                }

                let indices: Vec<usize> = (0..rows).collect();
                *root = Some(Box::new(Self::bangun_pohon(
                    x,
                    targets,
                    &indices,
                    0,
                    *kedalaman_maks,
                    *min_sampel_pisah,
                    *is_klasifikasi,
                )));
            }

            JenisModel::HutanAcak {
                pohon_list,
                jumlah_pohon,
                kedalaman_maks,
                min_sampel_pisah,
                seed,
                is_klasifikasi,
            } => {
                let targets = y.ok_or_else(|| "HutanAcak membutuhkan label target y".to_string())?;
                if targets.len() != rows {
                    return Err(format!("Ukuran X ({}) dan y ({}) tidak cocok", rows, targets.len()));
                }

                let mut rng = AcakLcg::new(*seed);
                pohon_list.clear();

                for _ in 0..*jumlah_pohon {
                    // Bootstrap sampling (sampel baris dengan pengembalian)
                    let mut bootstrap_idx = Vec::with_capacity(rows);
                    for _ in 0..rows {
                        let idx = (rng.next_u64() as usize) % rows;
                        bootstrap_idx.push(idx);
                    }

                    let pohon = Self::bangun_pohon(
                        x,
                        targets,
                        &bootstrap_idx,
                        0,
                        *kedalaman_maks,
                        *min_sampel_pisah,
                        *is_klasifikasi,
                    );
                    pohon_list.push(pohon);
                }
            }

            JenisModel::KMeans {
                k,
                iterasi_maks,
                pusat_klaster,
                inersia,
                seed,
            } => {
                if *k > rows {
                    return Err(format!("Jumlah klaster k ({}) lebih besar dari jumlah sampel ({})", k, rows));
                }

                let mut rng = AcakLcg::new(*seed);
                pusat_klaster.clear();

                // Inisialisasi centroid secara acak dari sampel
                let mut initial_indices = BTreeSet::new();
                while initial_indices.len() < *k {
                    let idx = (rng.next_u64() as usize) % rows;
                    initial_indices.insert(idx);
                }

                for &idx in &initial_indices {
                    let start = idx * cols;
                    pusat_klaster.push(x.data[start..start + cols].to_vec());
                }

                let mut assignments = vec![0usize; rows];

                for _ in 0..*iterasi_maks {
                    let mut changed = false;

                    // Langkah 1: penetapan titik ke centroid terdekat
                    for r in 0..rows {
                        let start = r * cols;
                        let point = &x.data[start..start + cols];

                        let mut min_dist = f64::INFINITY;
                        let mut best_k = 0;

                        for (k_idx, center) in pusat_klaster.iter().enumerate() {
                            let mut d = 0.0;
                            for c in 0..cols {
                                d += (point[c] - center[c]).powi(2);
                            }
                            if d < min_dist {
                                min_dist = d;
                                best_k = k_idx;
                            }
                        }

                        if assignments[r] != best_k {
                            assignments[r] = best_k;
                            changed = true;
                        }
                    }

                    if !changed {
                        break;
                    }

                    // Langkah 2: perbarui pusat klaster ke mean anggota
                    for k_idx in 0..*k {
                        let mut new_center = vec![0.0; cols];
                        let mut count = 0;

                        for r in 0..rows {
                            if assignments[r] == k_idx {
                                let start = r * cols;
                                for c in 0..cols {
                                    new_center[c] += x.data[start + c];
                                }
                                count += 1;
                            }
                        }

                        if count > 0 {
                            for c in 0..cols {
                                new_center[c] /= count as f64;
                            }
                            pusat_klaster[k_idx] = new_center;
                        }
                    }
                }

                // Hitung total inersia akhir
                let mut total_inertia = 0.0;
                for r in 0..rows {
                    let k_idx = assignments[r];
                    let center = &pusat_klaster[k_idx];
                    let start = r * cols;
                    for c in 0..cols {
                        total_inertia += (x.data[start + c] - center[c]).powi(2);
                    }
                }
                *inersia = total_inertia;
            }
        }

        self.fitted = true;
        Ok(())
    }

    // =========================================================================
    // INFERENSI (PREDICT / PREDIKSI)
    // =========================================================================

    /// Memprediksi target untuk data masukan baru X
    pub fn prediksi(&self, x: &MatraArray) -> Result<MatraArray, String> {
        if !self.fitted {
            return Err(format!("Model '{}' belum dilatih. Panggil .latih(X, y) terlebih dahulu.", self.nama));
        }
        if x.bentuk.len() != 2 {
            return Err("Matriks X untuk prediksi harus 2 dimensi".to_string());
        }
        let rows = x.bentuk[0];
        let cols = x.bentuk[1];

        match &self.jenis {
            JenisModel::RegresiLinear { bobot, intersep, .. } => {
                let mut out = Vec::with_capacity(rows);
                for r in 0..rows {
                    let start = r * cols;
                    let mut y_pred = *intersep;
                    for c in 0..cols {
                        y_pred += bobot[c] * x.data[start + c];
                    }
                    out.push(y_pred);
                }
                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }

            JenisModel::RegresiLogistik {
                bobot_per_kelas,
                intersep_per_kelas,
                kelas,
                ..
            } => {
                let mut out = Vec::with_capacity(rows);
                let n_models = bobot_per_kelas.len();

                for r in 0..rows {
                    let start = r * cols;
                    if n_models == 1 {
                        // Biner
                        let mut z = intersep_per_kelas[0];
                        for c in 0..cols {
                            z += bobot_per_kelas[0][c] * x.data[start + c];
                        }
                        let p = 1.0 / (1.0 + (-z).exp());
                        let pred = if p >= 0.5 { kelas[0] } else { kelas.get(1).cloned().unwrap_or(0.0) };
                        out.push(pred);
                    } else {
                        // Multi-kelas: pilih kelas dengan z (log-odds) tertinggi
                        let mut max_z = f64::NEG_INFINITY;
                        let mut best_cls = kelas[0];
                        for k in 0..n_models {
                            let mut z = intersep_per_kelas[k];
                            for c in 0..cols {
                                z += bobot_per_kelas[k][c] * x.data[start + c];
                            }
                            if z > max_z {
                                max_z = z;
                                best_cls = kelas[k];
                            }
                        }
                        out.push(best_cls);
                    }
                }
                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }

            JenisModel::KNN {
                k,
                data_x,
                data_y,
                rows: tr_rows,
                cols: tr_cols,
                is_klasifikasi,
                ..
            } => {
                if cols != *tr_cols {
                    return Err(format!("Jumlah fitur ({}) tidak sama dengan data latih ({})", cols, tr_cols));
                }

                let mut out = Vec::with_capacity(rows);

                for r in 0..rows {
                    let start = r * cols;
                    let sample = &x.data[start..start + cols];

                    // Hitung jarak ke seluruh sampel latih
                    let mut jarak_list = Vec::with_capacity(*tr_rows);
                    for tr_r in 0..*tr_rows {
                        let tr_start = tr_r * cols;
                        let mut dist = 0.0;
                        for c in 0..cols {
                            dist += (sample[c] - data_x[tr_start + c]).powi(2);
                        }
                        jarak_list.push((dist, data_y[tr_r]));
                    }

                    // Urutkan jarak terkecil
                    jarak_list.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
                    let top_k = &jarak_list[..(*k).min(jarak_list.len())];

                    if *is_klasifikasi {
                        // Majority voting
                        let mut counts: HashMap<i64, usize> = HashMap::new();
                        for &(_, label) in top_k {
                            *counts.entry(label.round() as i64).or_default() += 1;
                        }
                        let mut best_label = 0;
                        let mut max_count = 0;
                        for (lbl, cnt) in counts {
                            if cnt > max_count {
                                max_count = cnt;
                                best_label = lbl;
                            }
                        }
                        out.push(best_label as f64);
                    } else {
                        // Rata-rata target
                        let avg: f64 = top_k.iter().map(|&(_, y)| y).sum::<f64>() / top_k.len() as f64;
                        out.push(avg);
                    }
                }

                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }

            JenisModel::PohonKeputusan { root, .. } => {
                let r_node = root.as_ref().ok_or_else(|| "Pohon keputusan kosong".to_string())?;
                let mut out = Vec::with_capacity(rows);
                for r in 0..rows {
                    let start = r * cols;
                    let sample = &x.data[start..start + cols];
                    out.push(r_node.prediksi_sampel(sample));
                }
                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }

            JenisModel::HutanAcak {
                pohon_list,
                is_klasifikasi,
                ..
            } => {
                let mut out = Vec::with_capacity(rows);
                for r in 0..rows {
                    let start = r * cols;
                    let sample = &x.data[start..start + cols];

                    let mut tree_preds = Vec::with_capacity(pohon_list.len());
                    for pohon in pohon_list {
                        tree_preds.push(pohon.prediksi_sampel(sample));
                    }

                    if *is_klasifikasi {
                        // Majority voting
                        let mut counts: HashMap<i64, usize> = HashMap::new();
                        for &pred in &tree_preds {
                            *counts.entry(pred.round() as i64).or_default() += 1;
                        }
                        let mut best_lbl = 0;
                        let mut max_c = 0;
                        for (lbl, c) in counts {
                            if c > max_c {
                                max_c = c;
                                best_lbl = lbl;
                            }
                        }
                        out.push(best_lbl as f64);
                    } else {
                        let avg: f64 = tree_preds.iter().sum::<f64>() / tree_preds.len() as f64;
                        out.push(avg);
                    }
                }
                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }

            JenisModel::KMeans { pusat_klaster, .. } => {
                let mut out = Vec::with_capacity(rows);
                for r in 0..rows {
                    let start = r * cols;
                    let sample = &x.data[start..start + cols];

                    let mut min_dist = f64::INFINITY;
                    let mut best_k = 0;

                    for (k_idx, center) in pusat_klaster.iter().enumerate() {
                        let mut d = 0.0;
                        for c in 0..cols {
                            d += (sample[c] - center[c]).powi(2);
                        }
                        if d < min_dist {
                            min_dist = d;
                            best_k = k_idx;
                        }
                    }
                    out.push(best_k as f64);
                }
                Ok(MatraArray::dari_vektor(out, vec![rows]))
            }
        }
    }

    /// Memprediksi probabilitas kelas untuk model klasifikasi
    pub fn prediksi_probabilitas(&self, x: &MatraArray) -> Result<MatraArray, String> {
        if !self.fitted {
            return Err("Model belum dilatih".to_string());
        }
        let rows = x.bentuk[0];
        let cols = x.bentuk[1];

        match &self.jenis {
            JenisModel::RegresiLogistik {
                bobot_per_kelas,
                intersep_per_kelas,
                kelas,
                ..
            } => {
                let n_models = bobot_per_kelas.len();
                if n_models == 1 {
                    // 2 kelas (prob_0, prob_1)
                    let mut out = Vec::with_capacity(rows * 2);
                    for r in 0..rows {
                        let start = r * cols;
                        let mut z = intersep_per_kelas[0];
                        for c in 0..cols {
                            z += bobot_per_kelas[0][c] * x.data[start + c];
                        }
                        let p1 = 1.0 / (1.0 + (-z).exp());
                        let p0 = 1.0 - p1;
                        out.push(p0);
                        out.push(p1);
                    }
                    Ok(MatraArray::dari_vektor_2d(out, rows, 2))
                } else {
                    let mut out = Vec::with_capacity(rows * kelas.len());
                    for r in 0..rows {
                        let start = r * cols;
                        let mut exps = Vec::with_capacity(n_models);
                        let mut sum_exp = 0.0;
                        for k in 0..n_models {
                            let mut z = intersep_per_kelas[k];
                            for c in 0..cols {
                                z += bobot_per_kelas[k][c] * x.data[start + c];
                            }
                            let e = z.exp();
                            exps.push(e);
                            sum_exp += e;
                        }
                        for e in exps {
                            out.push(if sum_exp > 0.0 { e / sum_exp } else { 0.0 });
                        }
                    }
                    Ok(MatraArray::dari_vektor_2d(out, rows, kelas.len()))
                }
            }
            _ => Err("Prediksi probabilitas hanya didukung untuk model klasifikasi probabilistik (RegresiLogistik)".to_string()),
        }
    }

    /// Evaluasi performa model: R2 untuk regresi, akurasi untuk klasifikasi
    pub fn skor(&self, x: &MatraArray, y: &[f64]) -> Result<f64, String> {
        let preds = self.prediksi(x)?;
        match &self.jenis {
            JenisModel::RegresiLinear { .. } | JenisModel::KNN { is_klasifikasi: false, .. } => {
                crate::matra::metrik::r2_skor(y, &preds.data)
            }
            _ => crate::matra::metrik::akurasi(y, &preds.data),
        }
    }

    // =========================================================================
    // HELPER: PEMBANGUN POHON KEPUTUSAN CART
    // =========================================================================

    fn bangun_pohon(
        x: &MatraArray,
        y: &[f64],
        indices: &[usize],
        kedalaman: usize,
        kedalaman_maks: usize,
        min_sampel: usize,
        is_klasifikasi: bool,
    ) -> NodePohon {
        let n_samples = indices.len();
        if n_samples == 0 {
            return NodePohon::daun(0.0);
        }

        // Hitung nilai daun bawaan
        let default_val = if is_klasifikasi {
            let mut counts: HashMap<i64, usize> = HashMap::new();
            for &idx in indices {
                *counts.entry(y[idx].round() as i64).or_default() += 1;
            }
            counts.into_iter().max_by_key(|&(_, c)| c).map(|(k, _)| k as f64).unwrap_or(0.0)
        } else {
            indices.iter().map(|&idx| y[idx]).sum::<f64>() / n_samples as f64
        };

        if kedalaman >= kedalaman_maks || n_samples < min_sampel {
            return NodePohon::daun(default_val);
        }

        // Cek kemurnian (purity)
        let first_val = y[indices[0]];
        let pure = indices.iter().all(|&idx| (y[idx] - first_val).abs() < 1e-6);
        if pure {
            return NodePohon::daun(first_val);
        }

        let cols = x.bentuk[1];
        let mut best_gain = -1.0;
        let mut best_feature = 0;
        let mut best_threshold = 0.0;
        let mut best_left_idx = Vec::new();
        let mut best_right_idx = Vec::new();

        let base_impurity = if is_klasifikasi {
            Self::gini(y, indices)
        } else {
            Self::varians(y, indices)
        };

        for f in 0..cols {
            // Ambil kandidat ambang batas unik
            let mut values: Vec<f64> = indices.iter().map(|&idx| x.data[idx * cols + f]).collect();
            values.sort_by(|a, b| a.partial_cmp(b).unwrap());
            values.dedup();

            for w in values.windows(2) {
                let threshold = (w[0] + w[1]) / 2.0;
                let mut left = Vec::new();
                let mut right = Vec::new();

                for &idx in indices {
                    if x.data[idx * cols + f] <= threshold {
                        left.push(idx);
                    } else {
                        right.push(idx);
                    }
                }

                if left.is_empty() || right.is_empty() {
                    continue;
                }

                let impurity_left = if is_klasifikasi { Self::gini(y, &left) } else { Self::varians(y, &left) };
                let impurity_right = if is_klasifikasi { Self::gini(y, &right) } else { Self::varians(y, &right) };

                let weighted = (left.len() as f64 * impurity_left + right.len() as f64 * impurity_right) / n_samples as f64;
                let gain = base_impurity - weighted;

                if gain > best_gain {
                    best_gain = gain;
                    best_feature = f;
                    best_threshold = threshold;
                    best_left_idx = left;
                    best_right_idx = right;
                }
            }
        }

        if best_gain <= 1e-7 || best_left_idx.is_empty() || best_right_idx.is_empty() {
            return NodePohon::daun(default_val);
        }

        let left_child = Self::bangun_pohon(
            x,
            y,
            &best_left_idx,
            kedalaman + 1,
            kedalaman_maks,
            min_sampel,
            is_klasifikasi,
        );
        let right_child = Self::bangun_pohon(
            x,
            y,
            &best_right_idx,
            kedalaman + 1,
            kedalaman_maks,
            min_sampel,
            is_klasifikasi,
        );

        NodePohon::cabang(best_feature, best_threshold, left_child, right_child)
    }

    fn gini(y: &[f64], indices: &[usize]) -> f64 {
        let total = indices.len() as f64;
        if total == 0.0 {
            return 0.0;
        }
        let mut counts: HashMap<i64, usize> = HashMap::new();
        for &idx in indices {
            *counts.entry(y[idx].round() as i64).or_default() += 1;
        }
        let mut sum_sq = 0.0;
        for (_, count) in counts {
            let p = count as f64 / total;
            sum_sq += p * p;
        }
        1.0 - sum_sq
    }

    fn varians(y: &[f64], indices: &[usize]) -> f64 {
        let total = indices.len() as f64;
        if total <= 1.0 {
            return 0.0;
        }
        let mean = indices.iter().map(|&idx| y[idx]).sum::<f64>() / total;
        let sum_sq: f64 = indices.iter().map(|&idx| (y[idx] - mean).powi(2)).sum();
        sum_sq / total
    }
}
