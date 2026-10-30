use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::rc::Rc;

use crate::matra::larik::MatraArray;
use crate::value::Value;

/// Struktur data TabelData (DataFrame) columnar berbasis memori kontinu.
/// Menyimpan kumpulan kolom secara independen untuk performa agregasi dan slicing tinggi.
#[derive(Debug, Clone, PartialEq)]
pub struct MatraTabel {
    pub nama_kolom: Vec<String>,
    pub kolom: Vec<Vec<Value>>,
}

impl MatraTabel {
    /// Membuat tabel baru dari nama-nama kolom dan data kolomnya.
    pub fn baru(nama_kolom: Vec<String>, kolom: Vec<Vec<Value>>) -> Result<Self, String> {
        if nama_kolom.len() != kolom.len() {
            return Err(format!(
                "Jumlah nama kolom ({}) tidak sesuai dengan jumlah kolom data ({})",
                nama_kolom.len(),
                kolom.len()
            ));
        }

        // Pastikan nama kolom tidak ada duplikasi
        let mut unik = HashSet::new();
        for n in &nama_kolom {
            if !unik.insert(n) {
                return Err(format!("Nama kolom duplikat ditemukan: '{}'", n));
            }
        }

        // Pastikan semua kolom memiliki panjang baris yang sama
        if let Some(pertama) = kolom.first() {
            let n_baris = pertama.len();
            for (idx, c) in kolom.iter().enumerate() {
                if c.len() != n_baris {
                    return Err(format!(
                        "Kolom '{}' memiliki {} baris, diharapkan {} baris",
                        nama_kolom[idx],
                        c.len(),
                        n_baris
                    ));
                }
            }
        }

        Ok(Self { nama_kolom, kolom })
    }

    /// Membuat tabel kosong tanpa kolom
    pub fn kosong() -> Self {
        Self {
            nama_kolom: Vec::new(),
            kolom: Vec::new(),
        }
    }

    /// Membuat tabel dari kamus { "kolom": [data...] }
    pub fn dari_kamus(map: &HashMap<String, Value>) -> Result<Self, String> {
        if map.is_empty() {
            return Ok(Self::kosong());
        }

        let mut nama_kolom = Vec::with_capacity(map.len());
        let mut kolom = Vec::with_capacity(map.len());

        let mut n_baris: Option<usize> = None;

        // Ambil nama kolom secara terurut alfabet agar deterministik
        let mut keys: Vec<String> = map.keys().cloned().collect();
        keys.sort();

        for k in keys {
            let v = &map[&k];
            let data_kolom: Vec<Value> = match v {
                Value::Daftar(d) => d.borrow().clone(),
                Value::Larik(l) => l.borrow().data.iter().map(|n| Value::Angka(*n)).collect(),
                _ => {
                    return Err(format!(
                        "Data untuk kolom '{}' harus berupa daftar atau larik",
                        k
                    ))
                }
            };

            if let Some(expected) = n_baris {
                if data_kolom.len() != expected {
                    return Err(format!(
                        "Panjang data kolom '{}' ({}) tidak seragam (diharapkan {})",
                        k,
                        data_kolom.len(),
                        expected
                    ));
                }
            } else {
                n_baris = Some(data_kolom.len());
            }

            nama_kolom.push(k);
            kolom.push(data_kolom);
        }

        Ok(Self { nama_kolom, kolom })
    }

    /// Membuat tabel dari daftar kamus baris (records): [ {"nama": "A", "umur": 20}, ... ]
    pub fn dari_daftar_rekaman(list: &[Value]) -> Result<Self, String> {
        if list.is_empty() {
            return Ok(Self::kosong());
        }

        // Kumpulkan semua nama kolom dari seluruh rekaman
        let mut nama_kolom = Vec::new();
        let mut kunci_dilihat = HashSet::new();

        for item in list {
            if let Value::Kamus(k) = item {
                for key in k.borrow().keys() {
                    if kunci_dilihat.insert(key.clone()) {
                        nama_kolom.push(key.clone());
                    }
                }
            } else {
                return Err("Setiap elemen dalam daftar rekaman harus berupa kamus".to_string());
            }
        }

        nama_kolom.sort();

        let n_baris = list.len();
        let mut kolom = vec![Vec::with_capacity(n_baris); nama_kolom.len()];

        for item in list {
            if let Value::Kamus(k) = item {
                let map = k.borrow();
                for (c_idx, nama) in nama_kolom.iter().enumerate() {
                    let val = map.get(nama).cloned().unwrap_or(Value::Kosong);
                    kolom[c_idx].push(val);
                }
            }
        }

        Ok(Self { nama_kolom, kolom })
    }

    /// Membuat tabel dari MatraArray (vektor 1D atau matriks 2D)
    pub fn dari_larik(larik: &MatraArray, nama_kolom: Option<Vec<String>>) -> Result<Self, String> {
        if larik.bentuk.is_empty() {
            return Ok(Self::kosong());
        }

        if larik.bentuk.len() == 1 {
            let _n_baris = larik.data.len();
            let col_name = nama_kolom
                .and_then(|mut nk| {
                    if !nk.is_empty() {
                        Some(nk.remove(0))
                    } else {
                        None
                    }
                })
                .unwrap_or_else(|| "k0".to_string());

            let col_data: Vec<Value> = larik.data.iter().map(|&n| Value::Angka(n)).collect();
            Ok(Self {
                nama_kolom: vec![col_name],
                kolom: vec![col_data],
            })
        } else if larik.bentuk.len() == 2 {
            let rows = larik.bentuk[0];
            let cols = larik.bentuk[1];

            let names = match nama_kolom {
                Some(nk) => {
                    if nk.len() != cols {
                        return Err(format!(
                            "Jumlah nama kolom ({}) tidak cocok dengan jumlah kolom matriks ({})",
                            nk.len(),
                            cols
                        ));
                    }
                    nk
                }
                None => (0..cols).map(|i| format!("k{}", i)).collect(),
            };

            let mut kolom = vec![Vec::with_capacity(rows); cols];
            for r in 0..rows {
                for c in 0..cols {
                    kolom[c].push(Value::Angka(larik.data[r * cols + c]));
                }
            }

            Ok(Self {
                nama_kolom: names,
                kolom,
            })
        } else {
            Err("Hanya array 1D atau 2D yang dapat dikonversi ke TabelData".to_string())
        }
    }

    /// Jumlah total baris dalam tabel.
    pub fn jumlah_baris(&self) -> usize {
        self.kolom.first().map(|c| c.len()).unwrap_or(0)
    }

    /// Jumlah total kolom dalam tabel.
    pub fn jumlah_kolom(&self) -> usize {
        self.nama_kolom.len()
    }

    /// Bentuk tabel dalam format `[jumlah_baris, jumlah_kolom]`.
    pub fn bentuk(&self) -> Vec<usize> {
        vec![self.jumlah_baris(), self.jumlah_kolom()]
    }

    /// Mencari indeks kolom berdasarkan namanya.
    pub fn indeks_kolom(&self, nama: &str) -> Option<usize> {
        self.nama_kolom.iter().position(|n| n == nama)
    }

    /// Mengambil data satu kolom.
    /// Jika seluruh nilai dalam kolom bertipe angka, dikonversi otomatis menjadi `Value::Larik`
    /// sehingga langsung mendukung operasi aljabar linier / vektorisasi.
    pub fn ambil_kolom(&self, nama: &str) -> Result<Value, String> {
        let idx = self
            .indeks_kolom(nama)
            .ok_or_else(|| format!("Kolom '{}' tidak ditemukan dalam tabel", nama))?;

        let col_data = &self.kolom[idx];

        // Cek apakah seluruh elemen kolom adalah numerik
        let mut semua_angka = true;
        let mut float_vec = Vec::with_capacity(col_data.len());
        for val in col_data {
            if let Value::Angka(n) = val {
                float_vec.push(*n);
            } else {
                semua_angka = false;
                break;
            }
        }

        if semua_angka && !col_data.is_empty() {
            let arr = MatraArray {
                bentuk: vec![col_data.len()],
                data: float_vec,
            };
            Ok(Value::Larik(Rc::new(RefCell::new(arr))))
        } else {
            Ok(Value::Daftar(Rc::new(RefCell::new(col_data.clone()))))
        }
    }

    /// Menambahkan kolom baru atau menimpa kolom lama dengan nama yang sama.
    pub fn tambah_atau_ganti_kolom(
        &mut self,
        nama: String,
        data: Vec<Value>,
    ) -> Result<(), String> {
        let n_baris = self.jumlah_baris();
        if self.kolom.is_empty() {
            self.nama_kolom.push(nama);
            self.kolom.push(data);
            return Ok(());
        }

        if data.len() != n_baris {
            return Err(format!(
                "Panjang kolom baru ({}) tidak cocok dengan jumlah baris tabel ({})",
                data.len(),
                n_baris
            ));
        }

        if let Some(idx) = self.indeks_kolom(&nama) {
            self.kolom[idx] = data;
        } else {
            self.nama_kolom.push(nama);
            self.kolom.push(data);
        }
        Ok(())
    }

    /// Menghapus kolom berdasarkan namanya dan mengembalikan tabel baru.
    pub fn hapus_kolom(&self, nama: &str) -> Result<Self, String> {
        let idx = self
            .indeks_kolom(nama)
            .ok_or_else(|| format!("Kolom '{}' tidak ditemukan", nama))?;

        let mut nama_baru = self.nama_kolom.clone();
        nama_baru.remove(idx);

        let mut kolom_baru = self.kolom.clone();
        kolom_baru.remove(idx);

        Ok(Self {
            nama_kolom: nama_baru,
            kolom: kolom_baru,
        })
    }

    /// Membuang satu atau beberapa kolom (kebalikan dari pilih_kolom) dan mengembalikan tabel baru.
    pub fn kecuali(&self, nama_dibuang: &[String]) -> Result<Self, String> {
        let mut target_set = HashSet::new();
        for n in nama_dibuang {
            target_set.insert(n.as_str());
        }
        let mut nama_baru = Vec::new();
        let mut kolom_baru = Vec::new();
        for (i, n) in self.nama_kolom.iter().enumerate() {
            if !target_set.contains(n.as_str()) {
                nama_baru.push(n.clone());
                kolom_baru.push(self.kolom[i].clone());
            }
        }
        Ok(Self {
            nama_kolom: nama_baru,
            kolom: kolom_baru,
        })
    }

    /// Mengambil satu baris data pada indeks baris `r` sebagai kamus `{"kolom": nilai}`.
    pub fn ambil_baris(&self, r: usize) -> Result<HashMap<String, Value>, String> {
        let n_baris = self.jumlah_baris();
        if r >= n_baris {
            return Err(format!(
                "Indeks baris {} di luar batas [0..{}]",
                r,
                n_baris.saturating_sub(1)
            ));
        }

        let mut row_map = HashMap::with_capacity(self.nama_kolom.len());
        for (c_idx, nama) in self.nama_kolom.iter().enumerate() {
            row_map.insert(nama.clone(), self.kolom[c_idx][r].clone());
        }
        Ok(row_map)
    }

    /// Mengambil N baris pertama dari tabel.
    pub fn kepala(&self, n: usize) -> Self {
        let batas = n.min(self.jumlah_baris());
        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for c in &self.kolom {
            kolom_baru.push(c[..batas].to_vec());
        }
        Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        }
    }

    /// Mengambil N baris terakhir dari tabel.
    pub fn ekor(&self, n: usize) -> Self {
        let total = self.jumlah_baris();
        let mulai = total.saturating_sub(n);
        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for c in &self.kolom {
            kolom_baru.push(c[mulai..].to_vec());
        }
        Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        }
    }

    /// Memilih subset kolom tertentu dan mengembalikannya sebagai tabel baru.
    pub fn pilih_kolom(&self, nama_pilihan: &[String]) -> Result<Self, String> {
        let mut kolom_baru = Vec::with_capacity(nama_pilihan.len());
        for nama in nama_pilihan {
            let idx = self
                .indeks_kolom(nama)
                .ok_or_else(|| format!("Kolom '{}' tidak ditemukan", nama))?;
            kolom_baru.push(self.kolom[idx].clone());
        }
        Ok(Self {
            nama_kolom: nama_pilihan.to_vec(),
            kolom: kolom_baru,
        })
    }

    /// Mengganti nama kolom berdasarkan pemetaan kamus.
    pub fn ganti_nama_kolom(&self, mapping: &HashMap<String, String>) -> Self {
        let nama_baru: Vec<String> = self
            .nama_kolom
            .iter()
            .map(|n| mapping.get(n).cloned().unwrap_or_else(|| n.clone()))
            .collect();

        Self {
            nama_kolom: nama_baru,
            kolom: self.kolom.clone(),
        }
    }

    /// Mengurutkan tabel berdasarkan nilai pada kolom tertentu.
    pub fn urutkan(&self, nama_kolom: &str, menaik: bool) -> Result<Self, String> {
        let c_idx = self
            .indeks_kolom(nama_kolom)
            .ok_or_else(|| format!("Kolom '{}' tidak ditemukan untuk pengurutan", nama_kolom))?;

        let n_baris = self.jumlah_baris();
        let mut urutan_indeks: Vec<usize> = (0..n_baris).collect();

        let ref_col = &self.kolom[c_idx];
        urutan_indeks.sort_by(|&a, &b| {
            let va = &ref_col[a];
            let vb = &ref_col[b];
            let cmp = match (va, vb) {
                (Value::Angka(x), Value::Angka(y)) => {
                    x.partial_cmp(y).unwrap_or(std::cmp::Ordering::Equal)
                }
                (Value::Teks(x), Value::Teks(y)) => x.cmp(y),
                (Value::Bolean(x), Value::Bolean(y)) => x.cmp(y),
                (Value::Kosong, Value::Kosong) => std::cmp::Ordering::Equal,
                (Value::Kosong, _) => std::cmp::Ordering::Greater,
                (_, Value::Kosong) => std::cmp::Ordering::Less,
                _ => std::cmp::Ordering::Equal,
            };
            if menaik {
                cmp
            } else {
                cmp.reverse()
            }
        });

        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for c in &self.kolom {
            let reordered: Vec<Value> = urutan_indeks.iter().map(|&idx| c[idx].clone()).collect();
            kolom_baru.push(reordered);
        }

        Ok(Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        })
    }

    /// Menyaring baris berdasarkan vektor boolean mask.
    pub fn saring_dengan_mask(&self, mask: &[bool]) -> Result<Self, String> {
        let n_baris = self.jumlah_baris();
        if mask.len() != n_baris {
            return Err(format!(
                "Panjang mask penyaringan ({}) tidak sama dengan jumlah baris tabel ({})",
                mask.len(),
                n_baris
            ));
        }

        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for c in &self.kolom {
            let mut filtered = Vec::new();
            for (idx, &kondisi) in mask.iter().enumerate() {
                if kondisi {
                    filtered.push(c[idx].clone());
                }
            }
            kolom_baru.push(filtered);
        }

        Ok(Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        })
    }

    /// Memeriksa apakah ada sel kosong (Value::Kosong atau NaN).
    pub fn ada_kosong(&self) -> bool {
        for c in &self.kolom {
            for v in c {
                match v {
                    Value::Kosong => return true,
                    Value::Angka(n) if n.is_nan() => return true,
                    _ => {}
                }
            }
        }
        false
    }

    /// Menghapus seluruh baris yang memiliki nilai kosong.
    pub fn buang_kosong(&self) -> Self {
        let n_baris = self.jumlah_baris();
        let mut baris_valid = Vec::with_capacity(n_baris);

        for r in 0..n_baris {
            let mut valid = true;
            for c in &self.kolom {
                match &c[r] {
                    Value::Kosong => {
                        valid = false;
                        break;
                    }
                    Value::Angka(n) if n.is_nan() => {
                        valid = false;
                        break;
                    }
                    _ => {}
                }
            }
            baris_valid.push(valid);
        }

        self.saring_dengan_mask(&baris_valid)
            .unwrap_or_else(|_| self.clone())
    }

    /// Mengisi seluruh sel kosong dengan nilai pengganti skalar.
    pub fn isi_kosong(&self, pengganti: &Value) -> Self {
        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for c in &self.kolom {
            let mut filled = Vec::with_capacity(c.len());
            for v in c {
                match v {
                    Value::Kosong => filled.push(pengganti.clone()),
                    Value::Angka(n) if n.is_nan() => filled.push(pengganti.clone()),
                    other => filled.push(other.clone()),
                }
            }
            kolom_baru.push(filled);
        }
        Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        }
    }

    /// Mengisi nilai kosong per-kolom berdasarkan kamus pemetaan `{ "umur": 0, "nama": "Anonim" }`.
    pub fn isi_kosong_per_kolom(&self, pemetaan: &HashMap<String, Value>) -> Self {
        let mut kolom_baru = Vec::with_capacity(self.kolom.len());
        for (c_idx, nama) in self.nama_kolom.iter().enumerate() {
            let pengganti = pemetaan.get(nama);
            let mut filled = Vec::with_capacity(self.kolom[c_idx].len());
            for v in &self.kolom[c_idx] {
                let is_kosong =
                    matches!(v, Value::Kosong) || matches!(v, Value::Angka(n) if n.is_nan());
                if is_kosong {
                    if let Some(sub) = pengganti {
                        filled.push(sub.clone());
                    } else {
                        filled.push(v.clone());
                    }
                } else {
                    filled.push(v.clone());
                }
            }
            kolom_baru.push(filled);
        }
        Self {
            nama_kolom: self.nama_kolom.clone(),
            kolom: kolom_baru,
        }
    }

    /// Mengonversi kolom-kolom numerik ke `MatraArray` 2D (Tensor) siap pakai untuk ML.
    /// Bila `nama_pilihan` bernilai None, seluruh kolom numerik akan otomatis dipilih.
    pub fn ke_larik(&self, nama_pilihan: Option<&[String]>) -> Result<MatraArray, String> {
        let target_names: Vec<String> = match nama_pilihan {
            Some(names) => names.to_vec(),
            None => {
                // Pilih semua kolom numerik
                let mut num_cols = Vec::new();
                for (c_idx, name) in self.nama_kolom.iter().enumerate() {
                    let col_data = &self.kolom[c_idx];
                    let is_num = !col_data.is_empty()
                        && col_data.iter().all(|v| matches!(v, Value::Angka(_)));
                    if is_num {
                        num_cols.push(name.clone());
                    }
                }
                if num_cols.is_empty() {
                    return Err(
                        "Tidak ada kolom numerik yang ditemukan untuk dikonversi ke Larik"
                            .to_string(),
                    );
                }
                num_cols
            }
        };

        let rows = self.jumlah_baris();
        let cols = target_names.len();
        let mut data = Vec::with_capacity(rows * cols);

        let mut col_indices = Vec::with_capacity(cols);
        for name in &target_names {
            let idx = self
                .indeks_kolom(name)
                .ok_or_else(|| format!("Kolom '{}' tidak ditemukan", name))?;
            col_indices.push(idx);
        }

        for r in 0..rows {
            for &c_idx in &col_indices {
                match &self.kolom[c_idx][r] {
                    Value::Angka(n) => data.push(*n),
                    Value::Bolean(b) => data.push(if *b { 1.0 } else { 0.0 }),
                    _ => {
                        return Err(format!(
                            "Nilai pada kolom '{}' baris {} bukan angka/boolean dan tidak dapat diubah ke Larik",
                            self.nama_kolom[c_idx], r
                        ));
                    }
                }
            }
        }

        MatraArray::baru(data, vec![rows, cols])
    }

    /// Mengonversi tabel menjadi daftar kamus baris (List of Dictionaries).
    pub fn ke_daftar(&self) -> Value {
        let rows = self.jumlah_baris();
        let mut out = Vec::with_capacity(rows);
        for r in 0..rows {
            if let Ok(map) = self.ambil_baris(r) {
                out.push(Value::Kamus(Rc::new(RefCell::new(map))));
            }
        }
        Value::Daftar(Rc::new(RefCell::new(out)))
    }

    /// Menghasilkan tabel ringkasan statistik deskriptif untuk semua kolom numerik.
    pub fn deskripsi(&self) -> Self {
        let stats_names = vec![
            "hitung".to_string(),
            "rata_rata".to_string(),
            "std".to_string(),
            "min".to_string(),
            "max".to_string(),
        ];

        let mut out_cols_names = vec!["statistik".to_string()];
        let mut out_cols = vec![stats_names.iter().map(|s| Value::Teks(s.clone())).collect()];

        for (c_idx, name) in self.nama_kolom.iter().enumerate() {
            let mut numbers = Vec::new();
            for v in &self.kolom[c_idx] {
                if let Value::Angka(n) = v {
                    if !n.is_nan() {
                        numbers.push(*n);
                    }
                }
            }

            if numbers.is_empty() {
                continue;
            }

            let count = numbers.len() as f64;
            let sum: f64 = numbers.iter().sum();
            let avg = sum / count;
            let var: f64 = numbers.iter().map(|x| (x - avg).powi(2)).sum::<f64>() / count;
            let std = var.sqrt();
            let min = numbers.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = numbers.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

            out_cols_names.push(name.clone());
            out_cols.push(vec![
                Value::Angka(count),
                Value::Angka(avg),
                Value::Angka(std),
                Value::Angka(min),
                Value::Angka(max),
            ]);
        }

        Self {
            nama_kolom: out_cols_names,
            kolom: out_cols,
        }
    }

    /// Agregasi data dengan pengelompokan (GroupBy).
    pub fn kelompokkan_dan_agregasi(
        &self,
        kolom_grup: &str,
        operasi: &str,
    ) -> Result<Self, String> {
        let g_idx = self
            .indeks_kolom(kolom_grup)
            .ok_or_else(|| format!("Kolom pengelompokan '{}' tidak ditemukan", kolom_grup))?;

        let n_baris = self.jumlah_baris();
        let mut grup_map: HashMap<String, Vec<usize>> = HashMap::new();
        let mut grup_keys: Vec<String> = Vec::new();

        for r in 0..n_baris {
            let key_str = self.kolom[g_idx][r].to_string();
            if let Some(indices) = grup_map.get_mut(&key_str) {
                indices.push(r);
            } else {
                grup_map.insert(key_str.clone(), vec![r]);
                grup_keys.push(key_str);
            }
        }
        grup_keys.sort();

        let mut res_nama_kolom = vec![kolom_grup.to_string()];
        let mut res_kolom = vec![grup_keys
            .iter()
            .map(|k| Value::Teks(k.clone()))
            .collect::<Vec<Value>>()];

        for (c_idx, name) in self.nama_kolom.iter().enumerate() {
            if c_idx == g_idx {
                continue;
            }

            // Cek apakah numerik
            let is_num = self.kolom[c_idx]
                .iter()
                .any(|v| matches!(v, Value::Angka(_)));
            if !is_num && operasi != "hitung" && operasi != "count" {
                continue;
            }

            let mut col_vals = Vec::with_capacity(grup_keys.len());
            for g_key in &grup_keys {
                let row_indices = &grup_map[g_key];
                if operasi == "hitung" || operasi == "count" {
                    col_vals.push(Value::Angka(row_indices.len() as f64));
                    continue;
                }

                let mut nums = Vec::new();
                for &r in row_indices {
                    if let Value::Angka(n) = self.kolom[c_idx][r] {
                        if !n.is_nan() {
                            nums.push(n);
                        }
                    }
                }

                if nums.is_empty() {
                    col_vals.push(Value::Kosong);
                    continue;
                }

                let agg_val = match operasi {
                    "jumlah" | "sum" => nums.iter().sum::<f64>(),
                    "rata_rata" | "mean" | "avg" => nums.iter().sum::<f64>() / nums.len() as f64,
                    "min" => nums.iter().cloned().fold(f64::INFINITY, f64::min),
                    "max" => nums.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
                    _ => return Err(format!("Operasi agregasi '{}' tidak dikenal", operasi)),
                };
                col_vals.push(Value::Angka(agg_val));
            }

            res_nama_kolom.push(name.clone());
            res_kolom.push(col_vals);
        }

        Ok(Self {
            nama_kolom: res_nama_kolom,
            kolom: res_kolom,
        })
    }

    // --- PARSER DAN SERIALIZER CSV MANDIRI (ZERO-DEPENDENCY) ---

    /// Membaca dan mem-parsing teks CSV menjadi MatraTabel.
    pub fn parse_csv(teks: &str, pemisah_kustom: Option<char>) -> Result<Self, String> {
        let lines: Vec<&str> = teks.lines().collect();
        if lines.is_empty() {
            return Ok(Self::kosong());
        }

        // Tentukan pemisah delimiter
        let pemisah = pemisah_kustom.unwrap_or_else(|| {
            let first_line = lines[0];
            let comma_count = first_line.matches(',').count();
            let semi_count = first_line.matches(';').count();
            let tab_count = first_line.matches('\t').count();
            if semi_count > comma_count && semi_count > tab_count {
                ';'
            } else if tab_count > comma_count && tab_count > semi_count {
                '\t'
            } else {
                ','
            }
        });

        let mut baris_data: Vec<Vec<String>> = Vec::new();
        let mut baris_sekarang: Vec<String> = Vec::new();
        let mut field_sekarang = String::new();
        let mut dalam_tanda_petik = false;

        let chars: Vec<char> = teks.chars().collect();
        let mut i = 0;
        let len = chars.len();

        while i < len {
            let c = chars[i];
            if c == '"' {
                if dalam_tanda_petik && i + 1 < len && chars[i + 1] == '"' {
                    field_sekarang.push('"');
                    i += 1;
                } else {
                    dalam_tanda_petik = !dalam_tanda_petik;
                }
            } else if c == pemisah && !dalam_tanda_petik {
                baris_sekarang.push(field_sekarang.trim().to_string());
                field_sekarang.clear();
            } else if (c == '\n' || c == '\r') && !dalam_tanda_petik {
                if c == '\r' && i + 1 < len && chars[i + 1] == '\n' {
                    i += 1;
                }
                baris_sekarang.push(field_sekarang.trim().to_string());
                field_sekarang.clear();
                if !baris_sekarang.is_empty()
                    && !(baris_sekarang.len() == 1 && baris_sekarang[0].is_empty())
                {
                    baris_data.push(std::mem::take(&mut baris_sekarang));
                }
            } else {
                field_sekarang.push(c);
            }
            i += 1;
        }

        if !field_sekarang.is_empty() || !baris_sekarang.is_empty() {
            baris_sekarang.push(field_sekarang.trim().to_string());
            if !baris_sekarang.is_empty()
                && !(baris_sekarang.len() == 1 && baris_sekarang[0].is_empty())
            {
                baris_data.push(baris_sekarang);
            }
        }

        if baris_data.is_empty() {
            return Ok(Self::kosong());
        }

        let header = baris_data.remove(0);
        let n_kolom = header.len();
        let n_baris = baris_data.len();

        let mut kolom: Vec<Vec<Value>> = vec![Vec::with_capacity(n_baris); n_kolom];

        for row in baris_data {
            for c_idx in 0..n_kolom {
                let text_val = row.get(c_idx).map(|s| s.as_str()).unwrap_or("");
                let value = if text_val.is_empty()
                    || text_val == "NA"
                    || text_val == "null"
                    || text_val == "kosong"
                {
                    Value::Kosong
                } else if text_val.eq_ignore_ascii_case("benar")
                    || text_val.eq_ignore_ascii_case("true")
                {
                    Value::Bolean(true)
                } else if text_val.eq_ignore_ascii_case("salah")
                    || text_val.eq_ignore_ascii_case("false")
                {
                    Value::Bolean(false)
                } else if let Ok(num) = text_val.parse::<f64>() {
                    Value::Angka(num)
                } else {
                    Value::Teks(text_val.to_string())
                };
                kolom[c_idx].push(value);
            }
        }

        Self::baru(header, kolom)
    }

    /// Mengonversi tabel menjadi teks format CSV.
    pub fn ke_csv(&self, pemisah: Option<char>) -> String {
        let sep = pemisah.unwrap_or(',');
        let mut out = String::new();

        // Header
        for (i, nama) in self.nama_kolom.iter().enumerate() {
            if i > 0 {
                out.push(sep);
            }
            out.push_str(&Self::escape_csv_field(nama, sep));
        }
        out.push('\n');

        // Rows
        let n_baris = self.jumlah_baris();
        for r in 0..n_baris {
            for c in 0..self.kolom.len() {
                if c > 0 {
                    out.push(sep);
                }
                let val_str = match &self.kolom[c][r] {
                    Value::Kosong => String::new(),
                    Value::Bolean(b) => {
                        if *b {
                            "benar".to_string()
                        } else {
                            "salah".to_string()
                        }
                    }
                    Value::Angka(n) => n.to_string(),
                    Value::Teks(s) => s.clone(),
                    other => other.to_string(),
                };
                out.push_str(&Self::escape_csv_field(&val_str, sep));
            }
            out.push('\n');
        }

        out
    }

    fn escape_csv_field(field: &str, sep: char) -> String {
        if field.contains(sep)
            || field.contains('"')
            || field.contains('\n')
            || field.contains('\r')
        {
            let escaped = field.replace('"', "\"\"");
            format!("\"{}\"", escaped)
        } else {
            field.to_string()
        }
    }
}

impl fmt::Display for MatraTabel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n_baris = self.jumlah_baris();
        let n_kolom = self.jumlah_kolom();

        if n_kolom == 0 {
            return write!(f, "TabelData(Kosong: 0 baris x 0 kolom)");
        }

        // Hitung lebar setiap kolom
        let mut lebar_kolom: Vec<usize> = self.nama_kolom.iter().map(|n| n.len()).collect();

        let baris_tampil: Vec<usize> = if n_baris <= 10 {
            (0..n_baris).collect()
        } else {
            let mut b = Vec::new();
            b.extend(0..5);
            b.extend((n_baris - 5)..n_baris);
            b
        };

        for &r in &baris_tampil {
            for c in 0..n_kolom {
                let cell_len = self.kolom[c][r].to_string().len();
                lebar_kolom[c] = lebar_kolom[c].max(cell_len);
            }
        }

        // Garis pemisah atas
        write!(f, "┌")?;
        for (i, &w) in lebar_kolom.iter().enumerate() {
            write!(f, "{}", "─".repeat(w + 2))?;
            if i + 1 < n_kolom {
                write!(f, "┬")?;
            }
        }
        writeln!(f, "┐")?;

        // Header
        write!(f, "│")?;
        for (i, name) in self.nama_kolom.iter().enumerate() {
            write!(f, " {:<width$} │", name, width = lebar_kolom[i])?;
        }
        writeln!(f)?;

        // Garis pemisah header
        write!(f, "├")?;
        for (i, &w) in lebar_kolom.iter().enumerate() {
            write!(f, "{}", "─".repeat(w + 2))?;
            if i + 1 < n_kolom {
                write!(f, "┼")?;
            }
        }
        writeln!(f, "┤")?;

        // Baris data
        let mut dipotong = false;
        for (idx_tampil, &r) in baris_tampil.iter().enumerate() {
            if n_baris > 10 && idx_tampil == 5 && !dipotong {
                write!(f, "│")?;
                for &w in &lebar_kolom {
                    write!(f, " {:^width$} │", "...", width = w)?;
                }
                writeln!(f)?;
                dipotong = true;
            }

            write!(f, "│")?;
            for c in 0..n_kolom {
                let text = self.kolom[c][r].to_string();
                write!(f, " {:<width$} │", text, width = lebar_kolom[c])?;
            }
            writeln!(f)?;
        }

        // Garis pemisah bawah
        write!(f, "└")?;
        for (i, &w) in lebar_kolom.iter().enumerate() {
            write!(f, "{}", "─".repeat(w + 2))?;
            if i + 1 < n_kolom {
                write!(f, "┴")?;
            }
        }
        writeln!(f, "┘")?;

        write!(f, "[{} baris x {} kolom]", n_baris, n_kolom)
    }
}
