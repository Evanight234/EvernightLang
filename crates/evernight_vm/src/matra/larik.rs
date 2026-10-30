use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::value::Value;

/// Struktur data array / tensor n-dimensi berkecepatan tinggi di level Rust.
/// Menggunakan buffer kontinu 1D `Vec<f64>` dengan metadata `bentuk` (shape).
#[derive(Debug, Clone, PartialEq)]
pub struct MatraArray {
    pub bentuk: Vec<usize>,
    pub data: Vec<f64>,
}

impl MatraArray {
    /// Membuat MatraArray dari data kontinu dan bentuk (shape).
    pub fn baru(data: Vec<f64>, bentuk: Vec<usize>) -> Result<Self, String> {
        let expected: usize = if bentuk.is_empty() {
            0
        } else {
            bentuk.iter().product()
        };
        if data.len() != expected {
            return Err(format!(
                "Jumlah elemen ({}) tidak cocok dengan bentuk {:?} (total {})",
                data.len(),
                bentuk,
                expected
            ));
        }
        Ok(Self { bentuk, data })
    }

    /// Membuat MatraArray dari vektor data dan bentuk langsung tanpa validasi gagal.
    pub fn dari_vektor(data: Vec<f64>, bentuk: Vec<usize>) -> Self {
        Self { bentuk, data }
    }

    /// Membuat MatraArray matriks 2D dari vektor data dengan ukuran baris dan kolom.
    pub fn dari_vektor_2d(data: Vec<f64>, baris: usize, kolom: usize) -> Self {
        Self {
            bentuk: vec![baris, kolom],
            data,
        }
    }

    /// Membuat array berisi angka 0 dengan bentuk tertentu.
    pub fn nol(bentuk: &[usize]) -> Self {
        let total: usize = if bentuk.is_empty() {
            0
        } else {
            bentuk.iter().product()
        };
        Self {
            bentuk: bentuk.to_vec(),
            data: vec![0.0; total],
        }
    }

    /// Membuat array berisi angka 1 dengan bentuk tertentu.
    pub fn satu(bentuk: &[usize]) -> Self {
        let total: usize = if bentuk.is_empty() {
            0
        } else {
            bentuk.iter().product()
        };
        Self {
            bentuk: bentuk.to_vec(),
            data: vec![1.0; total],
        }
    }

    /// Membuat array terisi nilai tertentu dengan bentuk tertentu.
    pub fn penuh(bentuk: &[usize], val: f64) -> Self {
        let total: usize = if bentuk.is_empty() {
            0
        } else {
            bentuk.iter().product()
        };
        Self {
            bentuk: bentuk.to_vec(),
            data: vec![val; total],
        }
    }

    /// Membuat deret angka dari `mulai` sampai sebelum `selesai` dengan `langkah`.
    pub fn rentang(mulai: f64, selesai: f64, langkah: f64) -> Result<Self, String> {
        if langkah == 0.0 {
            return Err("Langkah rentang tidak boleh 0".to_string());
        }
        if (langkah > 0.0 && mulai >= selesai) || (langkah < 0.0 && mulai <= selesai) {
            return Ok(Self {
                bentuk: vec![0],
                data: Vec::new(),
            });
        }
        let mut data = Vec::new();
        let mut curr = mulai;
        if langkah > 0.0 {
            while curr < selesai - 1e-12 {
                data.push(curr);
                curr += langkah;
            }
        } else {
            while curr > selesai + 1e-12 {
                data.push(curr);
                curr += langkah;
            }
        }
        let n = data.len();
        Ok(Self {
            bentuk: vec![n],
            data,
        })
    }

    /// Membuat `jumlah` titik berjarak seragam antara `mulai` dan `selesai` (inklusif).
    pub fn spasi_linear(mulai: f64, selesai: f64, jumlah: usize) -> Result<Self, String> {
        if jumlah == 0 {
            return Ok(Self {
                bentuk: vec![0],
                data: Vec::new(),
            });
        }
        if jumlah == 1 {
            return Ok(Self {
                bentuk: vec![1],
                data: vec![mulai],
            });
        }
        let mut data = Vec::with_capacity(jumlah);
        let step = (selesai - mulai) / (jumlah - 1) as f64;
        for i in 0..jumlah {
            data.push(mulai + i as f64 * step);
        }
        Ok(Self {
            bentuk: vec![jumlah],
            data,
        })
    }

    /// Membuat matriks identitas berukuran N x N.
    pub fn identitas(n: usize) -> Self {
        let mut data = vec![0.0; n * n];
        for i in 0..n {
            data[i * n + i] = 1.0;
        }
        Self {
            bentuk: vec![n, n],
            data,
        }
    }

    /// Membuat array dengan nilai pseudo-acak uniform [0.0, 1.0).
    pub fn acak(bentuk: &[usize], seed: u64) -> Self {
        let total: usize = if bentuk.is_empty() {
            0
        } else {
            bentuk.iter().product()
        };
        let mut state = if seed == 0 { 0x853c49e6748fea9b } else { seed };
        let mut data = Vec::with_capacity(total);
        for _ in 0..total {
            // Xorshift64
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            let val = (state as f64) / (u64::MAX as f64);
            data.push(val);
        }
        Self {
            bentuk: bentuk.to_vec(),
            data,
        }
    }

    /// Mengonversi `Value` (Daftar atau nested Daftar) ke `MatraArray`.
    pub fn dari_value(val: &Value) -> Result<Self, String> {
        match val {
            Value::Larik(l) => Ok(l.borrow().clone()),
            Value::Angka(n) => Ok(Self {
                bentuk: vec![1],
                data: vec![*n],
            }),
            Value::Daftar(list_rc) => {
                let list = list_rc.borrow();
                if list.is_empty() {
                    return Ok(Self {
                        bentuk: vec![0],
                        data: Vec::new(),
                    });
                }
                // Cek apakah 1D atau 2D
                if let Some(Value::Daftar(_)) = list.first() {
                    // Kemungkinan 2D
                    let rows = list.len();
                    let mut cols = 0;
                    let mut data = Vec::new();
                    for (i, row_val) in list.iter().enumerate() {
                        match row_val {
                            Value::Daftar(r_rc) => {
                                let r = r_rc.borrow();
                                if i == 0 {
                                    cols = r.len();
                                } else if r.len() != cols {
                                    return Err(format!(
                                        "Panjang baris tidak seragam pada baris {} (diharapkan {}, didapat {})",
                                        i,
                                        cols,
                                        r.len()
                                    ));
                                }
                                for item in r.iter() {
                                    match item {
                                        Value::Angka(n) => data.push(*n),
                                        Value::Bolean(b) => data.push(if *b { 1.0 } else { 0.0 }),
                                        _ => return Err("Elemen larik harus bertipe angka".to_string()),
                                    }
                                }
                            }
                            _ => return Err("Format daftar 2D tidak konsisten".to_string()),
                        }
                    }
                    Ok(Self {
                        bentuk: vec![rows, cols],
                        data,
                    })
                } else {
                    // 1D
                    let mut data = Vec::with_capacity(list.len());
                    for item in list.iter() {
                        match item {
                            Value::Angka(n) => data.push(*n),
                            Value::Bolean(b) => data.push(if *b { 1.0 } else { 0.0 }),
                            _ => return Err("Elemen larik harus bertipe angka".to_string()),
                        }
                    }
                    let n = data.len();
                    Ok(Self {
                        bentuk: vec![n],
                        data,
                    })
                }
            }
            _ => Err("Nilai tidak dapat dikonversi ke Larik".to_string()),
        }
    }

    /// Mengonversi `MatraArray` kembali ke `Value::Daftar` (nested bila 2D).
    pub fn ke_value(&self) -> Value {
        if self.bentuk.is_empty() {
            return Value::Daftar(Rc::new(RefCell::new(Vec::new())));
        }
        if self.bentuk.len() == 1 {
            let list: Vec<Value> = self.data.iter().map(|n| Value::Angka(*n)).collect();
            Value::Daftar(Rc::new(RefCell::new(list)))
        } else if self.bentuk.len() == 2 {
            let rows = self.bentuk[0];
            let cols = self.bentuk[1];
            let mut outer = Vec::with_capacity(rows);
            for r in 0..rows {
                let mut inner = Vec::with_capacity(cols);
                for c in 0..cols {
                    inner.push(Value::Angka(self.data[r * cols + c]));
                }
                outer.push(Value::Daftar(Rc::new(RefCell::new(inner))));
            }
            Value::Daftar(Rc::new(RefCell::new(outer)))
        } else {
            // General fallback
            let list: Vec<Value> = self.data.iter().map(|n| Value::Angka(*n)).collect();
            Value::Daftar(Rc::new(RefCell::new(list)))
        }
    }

    /// Total jumlah elemen dalam array.
    pub fn panjang(&self) -> usize {
        self.data.len()
    }

    /// Jumlah dimensi (1 = vektor, 2 = matriks).
    pub fn dimensi(&self) -> usize {
        self.bentuk.len()
    }

    /// Mengubah bentuk array ke bentuk baru tanpa mengubah datanya.
    pub fn ubah_bentuk(&self, bentuk_baru: &[usize]) -> Result<Self, String> {
        let total_baru: usize = if bentuk_baru.is_empty() {
            0
        } else {
            bentuk_baru.iter().product()
        };
        if self.data.len() != total_baru {
            return Err(format!(
                "Tidak dapat mengubah bentuk dari {:?} (total {}) ke {:?} (total {})",
                self.bentuk,
                self.data.len(),
                bentuk_baru,
                total_baru
            ));
        }
        Ok(Self {
            bentuk: bentuk_baru.to_vec(),
            data: self.data.clone(),
        })
    }

    /// Meratakan array menjadi vektor 1D.
    pub fn ratakan(&self) -> Self {
        Self {
            bentuk: vec![self.data.len()],
            data: self.data.clone(),
        }
    }

    /// Transposisi matriks (menukar baris dan kolom).
    pub fn transposisi(&self) -> Result<Self, String> {
        if self.bentuk.len() == 1 {
            Ok(self.clone())
        } else if self.bentuk.len() == 2 {
            let rows = self.bentuk[0];
            let cols = self.bentuk[1];
            let mut new_data = vec![0.0; rows * cols];
            for r in 0..rows {
                for c in 0..cols {
                    new_data[c * rows + r] = self.data[r * cols + c];
                }
            }
            Ok(Self {
                bentuk: vec![cols, rows],
                data: new_data,
            })
        } else {
            Err("Transposisi saat ini hanya mendukung 1D dan 2D".to_string())
        }
    }

    /// Mengambil baris tertentu sebagai vektor 1D.
    pub fn baris(&self, r: usize) -> Result<Self, String> {
        if self.bentuk.len() != 2 {
            return Err("Operasi baris hanya berlaku untuk matriks 2D".to_string());
        }
        let rows = self.bentuk[0];
        let cols = self.bentuk[1];
        if r >= rows {
            return Err(format!("Indeks baris {} di luar batas [0..{}]", r, rows - 1));
        }
        let start = r * cols;
        let end = start + cols;
        Ok(Self {
            bentuk: vec![cols],
            data: self.data[start..end].to_vec(),
        })
    }

    // --- OPERASI ARITMATIKA ELEMENT-WISE ---

    pub fn tambah(&self, other: &Self) -> Result<Self, String> {
        if self.bentuk == other.bentuk {
            let data: Vec<f64> = self.data.iter().zip(&other.data).map(|(a, b)| a + b).collect();
            Ok(Self {
                bentuk: self.bentuk.clone(),
                data,
            })
        } else if other.data.len() == 1 {
            Ok(self.tambah_skalar(other.data[0]))
        } else if self.data.len() == 1 {
            Ok(other.tambah_skalar(self.data[0]))
        } else {
            Err(format!(
                "Dimensi tidak cocok untuk penjumlahan: {:?} vs {:?}",
                self.bentuk, other.bentuk
            ))
        }
    }

    pub fn tambah_skalar(&self, s: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|a| a + s).collect();
        Self {
            bentuk: self.bentuk.clone(),
            data,
        }
    }

    pub fn kurang(&self, other: &Self) -> Result<Self, String> {
        if self.bentuk == other.bentuk {
            let data: Vec<f64> = self.data.iter().zip(&other.data).map(|(a, b)| a - b).collect();
            Ok(Self {
                bentuk: self.bentuk.clone(),
                data,
            })
        } else if other.data.len() == 1 {
            Ok(self.kurang_skalar(other.data[0]))
        } else {
            Err(format!(
                "Dimensi tidak cocok untuk pengurangan: {:?} vs {:?}",
                self.bentuk, other.bentuk
            ))
        }
    }

    pub fn kurang_skalar(&self, s: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|a| a - s).collect();
        Self {
            bentuk: self.bentuk.clone(),
            data,
        }
    }

    pub fn kali(&self, other: &Self) -> Result<Self, String> {
        if self.bentuk == other.bentuk {
            let data: Vec<f64> = self.data.iter().zip(&other.data).map(|(a, b)| a * b).collect();
            Ok(Self {
                bentuk: self.bentuk.clone(),
                data,
            })
        } else if other.data.len() == 1 {
            Ok(self.kali_skalar(other.data[0]))
        } else if self.data.len() == 1 {
            Ok(other.kali_skalar(self.data[0]))
        } else {
            Err(format!(
                "Dimensi tidak cocok untuk perkalian elemen: {:?} vs {:?}",
                self.bentuk, other.bentuk
            ))
        }
    }

    pub fn kali_skalar(&self, s: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|a| a * s).collect();
        Self {
            bentuk: self.bentuk.clone(),
            data,
        }
    }

    pub fn bagi(&self, other: &Self) -> Result<Self, String> {
        if self.bentuk == other.bentuk {
            let mut data = Vec::with_capacity(self.data.len());
            for (a, b) in self.data.iter().zip(&other.data) {
                if *b == 0.0 {
                    return Err("Pembagian dengan nol pada elemen larik".to_string());
                }
                data.push(a / b);
            }
            Ok(Self {
                bentuk: self.bentuk.clone(),
                data,
            })
        } else if other.data.len() == 1 {
            self.bagi_skalar(other.data[0])
        } else {
            Err(format!(
                "Dimensi tidak cocok untuk pembagian: {:?} vs {:?}",
                self.bentuk, other.bentuk
            ))
        }
    }

    pub fn bagi_skalar(&self, s: f64) -> Result<Self, String> {
        if s == 0.0 {
            return Err("Pembagian dengan nol".to_string());
        }
        let data: Vec<f64> = self.data.iter().map(|a| a / s).collect();
        Ok(Self {
            bentuk: self.bentuk.clone(),
            data,
        })
    }

    pub fn negatif(&self) -> Self {
        let data: Vec<f64> = self.data.iter().map(|a| -a).collect();
        Self {
            bentuk: self.bentuk.clone(),
            data,
        }
    }

    pub fn pangkat_skalar(&self, exp: f64) -> Self {
        let data: Vec<f64> = self.data.iter().map(|a| a.powf(exp)).collect();
        Self {
            bentuk: self.bentuk.clone(),
            data,
        }
    }

    // --- ALJABAR LINIER: TITIK (DOT PRODUCT), INVERS, DETERMINAN ---

    /// Perkalian matriks atau perkalian titik (Dot Product).
    /// Mendukung 1D dot 1D, 2D dot 1D, dan 2D dot 2D.
    pub fn titik(&self, other: &Self) -> Result<Self, String> {
        // Kasus 1: 1D dot 1D -> Skalar (1D berisi 1 elemen)
        if self.bentuk.len() == 1 && other.bentuk.len() == 1 {
            if self.data.len() != other.data.len() {
                return Err(format!(
                    "Panjang vektor tidak sama untuk perkalian titik: {} vs {}",
                    self.data.len(),
                    other.data.len()
                ));
            }
            let sum: f64 = self.data.iter().zip(&other.data).map(|(a, b)| a * b).sum();
            return Ok(Self {
                bentuk: vec![1],
                data: vec![sum],
            });
        }

        // Kasus 2: 2D dot 1D (Matriks x Vektor) -> Vektor [M]
        if self.bentuk.len() == 2 && other.bentuk.len() == 1 {
            let m = self.bentuk[0];
            let k = self.bentuk[1];
            if k != other.data.len() {
                return Err(format!(
                    "Dimensi tidak cocok untuk matriks x vektor: ({}, {}) vs ({})",
                    m,
                    k,
                    other.data.len()
                ));
            }
            let mut out = Vec::with_capacity(m);
            for r in 0..m {
                let mut sum = 0.0;
                let offset = r * k;
                for c in 0..k {
                    sum += self.data[offset + c] * other.data[c];
                }
                out.push(sum);
            }
            return Ok(Self {
                bentuk: vec![m],
                data: out,
            });
        }

        // Kasus 3: 2D dot 2D (Matriks x Matriks): [M, K] x [K, N] -> [M, N]
        if self.bentuk.len() == 2 && other.bentuk.len() == 2 {
            let m = self.bentuk[0];
            let k1 = self.bentuk[1];
            let k2 = other.bentuk[0];
            let n = other.bentuk[1];
            if k1 != k2 {
                return Err(format!(
                    "Dimensi tidak cocok untuk perkalian matriks: [{}, {}] vs [{}, {}]",
                    m, k1, k2, n
                ));
            }
            let mut out = vec![0.0; m * n];
            for i in 0..m {
                for k in 0..k1 {
                    let a_val = self.data[i * k1 + k];
                    for j in 0..n {
                        out[i * n + j] += a_val * other.data[k * n + j];
                    }
                }
            }
            return Ok(Self {
                bentuk: vec![m, n],
                data: out,
            });
        }

        Err(format!(
            "Perkalian titik belum didukung untuk dimensi {:?} vs {:?}",
            self.bentuk, other.bentuk
        ))
    }

    /// Menghitung determinan matriks persegi 2D.
    pub fn determinan(&self) -> Result<f64, String> {
        if self.bentuk.len() != 2 || self.bentuk[0] != self.bentuk[1] {
            return Err("Determinan hanya berlaku untuk matriks persegi 2D".to_string());
        }
        let n = self.bentuk[0];
        if n == 1 {
            return Ok(self.data[0]);
        }
        if n == 2 {
            // ad - bc
            return Ok(self.data[0] * self.data[3] - self.data[1] * self.data[2]);
        }

        // Eliminasi Gauss untuk n x n
        let mut a = self.data.clone();
        let mut det = 1.0;
        for i in 0..n {
            // Pivot parsial
            let mut pivot = i;
            for j in (i + 1)..n {
                if a[j * n + i].abs() > a[pivot * n + i].abs() {
                    pivot = j;
                }
            }
            if a[pivot * n + i].abs() < 1e-12 {
                return Ok(0.0);
            }
            if pivot != i {
                for col in 0..n {
                    a.swap(i * n + col, pivot * n + col);
                }
                det = -det;
            }
            det *= a[i * n + i];
            let diag = a[i * n + i];
            for j in (i + 1)..n {
                let factor = a[j * n + i] / diag;
                for col in i..n {
                    a[j * n + col] -= factor * a[i * n + col];
                }
            }
        }
        Ok(det)
    }

    /// Menghitung invers matriks persegi 2D menggunakan eliminasi Gauss-Jordan.
    pub fn invers(&self) -> Result<Self, String> {
        if self.bentuk.len() != 2 || self.bentuk[0] != self.bentuk[1] {
            return Err("Invers hanya berlaku untuk matriks persegi 2D".to_string());
        }
        let n = self.bentuk[0];
        let mut a = self.data.clone();
        let mut inv = vec![0.0; n * n];
        for i in 0..n {
            inv[i * n + i] = 1.0;
        }

        for i in 0..n {
            // Cari pivot terbesar
            let mut pivot = i;
            for j in (i + 1)..n {
                if a[j * n + i].abs() > a[pivot * n + i].abs() {
                    pivot = j;
                }
            }
            if a[pivot * n + i].abs() < 1e-12 {
                return Err("Matriks singular (determinan 0), tidak memiliki invers".to_string());
            }
            if pivot != i {
                for col in 0..n {
                    a.swap(i * n + col, pivot * n + col);
                    inv.swap(i * n + col, pivot * n + col);
                }
            }

            let div = a[i * n + i];
            for col in 0..n {
                a[i * n + col] /= div;
                inv[i * n + col] /= div;
            }

            for row in 0..n {
                if row != i {
                    let factor = a[row * n + i];
                    for col in 0..n {
                        a[row * n + col] -= factor * a[i * n + col];
                        inv[row * n + col] -= factor * inv[i * n + col];
                    }
                }
            }
        }

        Ok(Self {
            bentuk: vec![n, n],
            data: inv,
        })
    }

    // --- OPERASI STATISTIK ---

    /// Jumlah semua elemen atau jumlah sepanjang sumbu (0 = kolom, 1 = baris).
    pub fn jumlah(&self, sumbu: Option<usize>) -> Result<Self, String> {
        match sumbu {
            None => {
                let s: f64 = self.data.iter().sum();
                Ok(Self {
                    bentuk: vec![1],
                    data: vec![s],
                })
            }
            Some(0) => {
                // Runtuhkan baris (jumlah per kolom)
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 0 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![0.0; cols];
                for r in 0..rows {
                    for c in 0..cols {
                        out[c] += self.data[r * cols + c];
                    }
                }
                Ok(Self {
                    bentuk: vec![cols],
                    data: out,
                })
            }
            Some(1) => {
                // Runtuhkan kolom (jumlah per baris)
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 1 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![0.0; rows];
                for r in 0..rows {
                    for c in 0..cols {
                        out[r] += self.data[r * cols + c];
                    }
                }
                Ok(Self {
                    bentuk: vec![rows],
                    data: out,
                })
            }
            _ => Err("Sumbu yang didukung: 0 (kolom) atau 1 (baris)".to_string()),
        }
    }

    /// Rata-rata elemen atau rata-rata sepanjang sumbu.
    pub fn rata_rata(&self, sumbu: Option<usize>) -> Result<Self, String> {
        if self.data.is_empty() {
            return Err("Tidak dapat menghitung rata-rata dari larik kosong".to_string());
        }
        match sumbu {
            None => {
                let sum: f64 = self.data.iter().sum();
                let avg = sum / self.data.len() as f64;
                Ok(Self {
                    bentuk: vec![1],
                    data: vec![avg],
                })
            }
            Some(0) => {
                let j = self.jumlah(Some(0))?;
                let rows = self.bentuk[0] as f64;
                Ok(j.bagi_skalar(rows)?)
            }
            Some(1) => {
                let j = self.jumlah(Some(1))?;
                let cols = self.bentuk[1] as f64;
                Ok(j.bagi_skalar(cols)?)
            }
            _ => Err("Sumbu yang didukung: 0 atau 1".to_string()),
        }
    }

    pub fn min(&self, sumbu: Option<usize>) -> Result<Self, String> {
        if self.data.is_empty() {
            return Err("Larik kosong".to_string());
        }
        match sumbu {
            None => {
                let m = self.data.iter().cloned().fold(f64::INFINITY, f64::min);
                Ok(Self {
                    bentuk: vec![1],
                    data: vec![m],
                })
            }
            Some(0) => {
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 0 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![f64::INFINITY; cols];
                for r in 0..rows {
                    for c in 0..cols {
                        out[c] = out[c].min(self.data[r * cols + c]);
                    }
                }
                Ok(Self {
                    bentuk: vec![cols],
                    data: out,
                })
            }
            Some(1) => {
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 1 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![f64::INFINITY; rows];
                for r in 0..rows {
                    for c in 0..cols {
                        out[r] = out[r].min(self.data[r * cols + c]);
                    }
                }
                Ok(Self {
                    bentuk: vec![rows],
                    data: out,
                })
            }
            _ => Err("Sumbu yang didukung: 0 atau 1".to_string()),
        }
    }

    pub fn max(&self, sumbu: Option<usize>) -> Result<Self, String> {
        if self.data.is_empty() {
            return Err("Larik kosong".to_string());
        }
        match sumbu {
            None => {
                let m = self.data.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                Ok(Self {
                    bentuk: vec![1],
                    data: vec![m],
                })
            }
            Some(0) => {
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 0 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![f64::NEG_INFINITY; cols];
                for r in 0..rows {
                    for c in 0..cols {
                        out[c] = out[c].max(self.data[r * cols + c]);
                    }
                }
                Ok(Self {
                    bentuk: vec![cols],
                    data: out,
                })
            }
            Some(1) => {
                if self.bentuk.len() != 2 {
                    return Err("Sumbu 1 hanya didukung untuk matriks 2D".to_string());
                }
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![f64::NEG_INFINITY; rows];
                for r in 0..rows {
                    for c in 0..cols {
                        out[r] = out[r].max(self.data[r * cols + c]);
                    }
                }
                Ok(Self {
                    bentuk: vec![rows],
                    data: out,
                })
            }
            _ => Err("Sumbu yang didukung: 0 atau 1".to_string()),
        }
    }

    pub fn varian(&self, sumbu: Option<usize>) -> Result<Self, String> {
        if self.data.len() < 2 {
            return Ok(Self {
                bentuk: vec![1],
                data: vec![0.0],
            });
        }
        let avg = self.rata_rata(sumbu)?;
        match sumbu {
            None => {
                let mu = avg.data[0];
                let v: f64 = self.data.iter().map(|x| (x - mu).powi(2)).sum::<f64>() / self.data.len() as f64;
                Ok(Self {
                    bentuk: vec![1],
                    data: vec![v],
                })
            }
            Some(0) => {
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![0.0; cols];
                for r in 0..rows {
                    for c in 0..cols {
                        let diff = self.data[r * cols + c] - avg.data[c];
                        out[c] += diff * diff;
                    }
                }
                for c in 0..cols {
                    out[c] /= rows as f64;
                }
                Ok(Self {
                    bentuk: vec![cols],
                    data: out,
                })
            }
            Some(1) => {
                let rows = self.bentuk[0];
                let cols = self.bentuk[1];
                let mut out = vec![0.0; rows];
                for r in 0..rows {
                    for c in 0..cols {
                        let diff = self.data[r * cols + c] - avg.data[r];
                        out[r] += diff * diff;
                    }
                }
                for r in 0..rows {
                    out[r] /= cols as f64;
                }
                Ok(Self {
                    bentuk: vec![rows],
                    data: out,
                })
            }
            _ => Err("Sumbu yang didukung: 0 atau 1".to_string()),
        }
    }

    pub fn deviasi_standar(&self, sumbu: Option<usize>) -> Result<Self, String> {
        let v = self.varian(sumbu)?;
        let data: Vec<f64> = v.data.iter().map(|x| x.sqrt()).collect();
        Ok(Self {
            bentuk: v.bentuk,
            data,
        })
    }

    pub fn median(&self) -> Result<f64, String> {
        if self.data.is_empty() {
            return Err("Larik kosong".to_string());
        }
        let mut sorted = self.data.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            Ok((sorted[mid - 1] + sorted[mid]) / 2.0)
        } else {
            Ok(sorted[mid])
        }
    }

    // --- OPERASI MATEMATIKA ELEMEN ---

    pub fn akar(&self) -> Result<Self, String> {
        for n in &self.data {
            if *n < 0.0 {
                return Err("Akar dari bilangan negatif tidak terdefinisi".to_string());
            }
        }
        Ok(Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.sqrt()).collect(),
        })
    }

    pub fn mutlak(&self) -> Self {
        Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.abs()).collect(),
        }
    }

    pub fn sin(&self) -> Self {
        Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.sin()).collect(),
        }
    }

    pub fn cos(&self) -> Self {
        Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.cos()).collect(),
        }
    }

    pub fn tan(&self) -> Self {
        Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.tan()).collect(),
        }
    }

    pub fn exp(&self) -> Self {
        Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.exp()).collect(),
        }
    }

    pub fn log(&self) -> Result<Self, String> {
        for n in &self.data {
            if *n <= 0.0 {
                return Err("Logaritma dari angka <= 0 tidak terdefinisi".to_string());
            }
        }
        Ok(Self {
            bentuk: self.bentuk.clone(),
            data: self.data.iter().map(|x| x.ln()).collect(),
        })
    }

    /// Menggabungkan dua array sepanjang sumbu tertentu.
    pub fn gabung(&self, other: &Self, sumbu: usize) -> Result<Self, String> {
        if self.bentuk.len() != other.bentuk.len() {
            return Err("Jumlah dimensi harus sama untuk penggabungan".to_string());
        }
        if self.bentuk.len() == 1 {
            let mut data = self.data.clone();
            data.extend_from_slice(&other.data);
            let n = data.len();
            Ok(Self {
                bentuk: vec![n],
                data,
            })
        } else if self.bentuk.len() == 2 {
            let r1 = self.bentuk[0];
            let c1 = self.bentuk[1];
            let r2 = other.bentuk[0];
            let c2 = other.bentuk[1];

            if sumbu == 0 {
                // Tumpuk vertikal (jumlah kolom harus sama)
                if c1 != c2 {
                    return Err(format!(
                        "Jumlah kolom tidak sama untuk tumpuk vertikal: {} vs {}",
                        c1, c2
                    ));
                }
                let mut data = self.data.clone();
                data.extend_from_slice(&other.data);
                Ok(Self {
                    bentuk: vec![r1 + r2, c1],
                    data,
                })
            } else if sumbu == 1 {
                // Tumpuk horizontal (jumlah baris harus sama)
                if r1 != r2 {
                    return Err(format!(
                        "Jumlah baris tidak sama untuk tumpuk horizontal: {} vs {}",
                        r1, r2
                    ));
                }
                let mut data = Vec::with_capacity((c1 + c2) * r1);
                for r in 0..r1 {
                    data.extend_from_slice(&self.data[r * c1..(r + 1) * c1]);
                    data.extend_from_slice(&other.data[r * c2..(r + 1) * c2]);
                }
                Ok(Self {
                    bentuk: vec![r1, c1 + c2],
                    data,
                })
            } else {
                Err("Sumbu yang didukung: 0 atau 1".to_string())
            }
        } else {
            Err("Penggabungan hanya didukung untuk 1D dan 2D".to_string())
        }
    }
}

impl fmt::Display for MatraArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.bentuk.is_empty() || self.data.is_empty() {
            return write!(f, "Larik([])");
        }
        if self.bentuk.len() == 1 {
            write!(f, "Larik([")?;
            for (i, val) in self.data.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }
                if val.fract() == 0.0 && *val >= -1e15 && *val <= 1e15 {
                    write!(f, "{:.0}", val)?;
                } else {
                    write!(f, "{:.4}", val)?;
                }
            }
            write!(f, "])")
        } else if self.bentuk.len() == 2 {
            let rows = self.bentuk[0];
            let cols = self.bentuk[1];
            writeln!(f, "Larik([")?;
            for r in 0..rows {
                write!(f, "  [")?;
                for c in 0..cols {
                    if c > 0 {
                        write!(f, ", ")?;
                    }
                    let val = self.data[r * cols + c];
                    if val.fract() == 0.0 && val >= -1e15 && val <= 1e15 {
                        write!(f, "{:.0}", val)?;
                    } else {
                        write!(f, "{:.4}", val)?;
                    }
                }
                if r + 1 < rows {
                    writeln!(f, "],")?;
                } else {
                    writeln!(f, "]")?;
                }
            }
            write!(f, "])")
        } else {
            write!(f, "Larik({:?}, {} elemen)", self.bentuk, self.data.len())
        }
    }
}
