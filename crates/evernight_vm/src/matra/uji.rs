use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Memeriksa apakah ada sampel data yang bocor (duplikat persis baris) antara data latih dan uji
pub fn cek_kebocoran(x_latih: &Value, x_uji: &Value) -> Result<Value, String> {
    let (latih_data, l_rows, cols) = ekstrak_matriks(x_latih)?;
    let (uji_data, u_rows, u_cols) = ekstrak_matriks(x_uji)?;

    if cols != u_cols {
        return Err(format!(
            "Jumlah kolom data latih ({}) berbeda dengan data uji ({})",
            cols, u_cols
        ));
    }

    let mut indeks_bocor = Vec::new();
    let eps = 1e-9;

    for u in 0..u_rows {
        let u_start = u * cols;
        let u_row = &uji_data[u_start..u_start + cols];

        let mut ditemukan = false;
        for l in 0..l_rows {
            let l_start = l * cols;
            let l_row = &latih_data[l_start..l_start + cols];

            let mut persis = true;
            for c in 0..cols {
                if (u_row[c] - l_row[c]).abs() > eps {
                    persis = false;
                    break;
                }
            }
            if persis {
                ditemukan = true;
                break;
            }
        }

        if ditemukan {
            indeks_bocor.push(Value::Angka(u as f64));
        }
    }

    let n_bocor = indeks_bocor.len();
    let mut hasil = HashMap::new();
    hasil.insert("ada_kebocoran".to_string(), Value::Bolean(n_bocor > 0));
    hasil.insert("jumlah_kebocoran".to_string(), Value::Angka(n_bocor as f64));
    hasil.insert(
        "persentase_bocor".to_string(),
        Value::Angka(if u_rows > 0 {
            (n_bocor as f64 / u_rows as f64) * 100.0
        } else {
            0.0
        }),
    );
    hasil.insert(
        "indeks_bocor".to_string(),
        Value::Daftar(Rc::new(RefCell::new(indeks_bocor))),
    );

    Ok(Value::Kamus(Rc::new(RefCell::new(hasil))))
}

/// Mendeteksi apakah terjadi pergeseran distribusi data (Data Drift) antar kolom fitur
pub fn cek_pergeseran(
    x_latih: &Value,
    x_uji: &Value,
    ambang: Option<f64>,
) -> Result<Value, String> {
    let (latih_data, l_rows, cols) = ekstrak_matriks(x_latih)?;
    let (uji_data, u_rows, u_cols) = ekstrak_matriks(x_uji)?;

    if cols != u_cols {
        return Err(format!(
            "Jumlah kolom data latih ({}) berbeda dengan data uji ({})",
            cols, u_cols
        ));
    }
    if l_rows == 0 || u_rows == 0 {
        return Err("Data latih atau uji tidak boleh kosong".to_string());
    }

    let threshold = ambang.unwrap_or(0.25);
    let mut kolom_geser = Vec::new();
    let mut skor_map = HashMap::new();

    for c in 0..cols {
        let mut sum_l = 0.0;
        for r in 0..l_rows {
            sum_l += latih_data[r * cols + c];
        }
        let mean_l = sum_l / l_rows as f64;

        let mut sum_u = 0.0;
        for r in 0..u_rows {
            sum_u += uji_data[r * cols + c];
        }
        let mean_u = sum_u / u_rows as f64;

        let mut var_l = 0.0;
        for r in 0..l_rows {
            let diff = latih_data[r * cols + c] - mean_l;
            var_l += diff * diff;
        }
        var_l /= l_rows as f64;

        let mut var_u = 0.0;
        for r in 0..u_rows {
            let diff = uji_data[r * cols + c] - mean_u;
            var_u += diff * diff;
        }
        var_u /= u_rows as f64;

        // Skor deviasi rata-rata terstandarisasi (Standardized Mean Difference / Drift Metric)
        let pooled_std = ((var_l + var_u) / 2.0).sqrt().max(1e-9);
        let skor_drift = (mean_l - mean_u).abs() / pooled_std;

        let col_name = format!("fitur_{}", c);
        skor_map.insert(col_name.clone(), Value::Angka(skor_drift));

        if skor_drift > threshold {
            kolom_geser.push(Value::Teks(col_name));
        }
    }

    let ada_geser = !kolom_geser.is_empty();
    let mut hasil = HashMap::new();
    hasil.insert("ada_pergeseran".to_string(), Value::Bolean(ada_geser));
    hasil.insert(
        "kolom_geser".to_string(),
        Value::Daftar(Rc::new(RefCell::new(kolom_geser.clone()))),
    );
    hasil.insert(
        "kolom_bergeser".to_string(),
        Value::Daftar(Rc::new(RefCell::new(kolom_geser))),
    );
    hasil.insert(
        "skor_pergeseran".to_string(),
        Value::Kamus(Rc::new(RefCell::new(skor_map))),
    );

    Ok(Value::Kamus(Rc::new(RefCell::new(hasil))))
}

fn ekstrak_matriks(val: &Value) -> Result<(Vec<f64>, usize, usize), String> {
    match val {
        Value::Larik(arr) => {
            let a = arr.borrow();
            if a.bentuk.len() != 2 {
                return Err("Data harus berbentuk matriks 2D".to_string());
            }
            Ok((a.data.clone(), a.bentuk[0], a.bentuk[1]))
        }
        Value::Tabel(tbl) => {
            let t = tbl.borrow();
            let arr = t.ke_larik(None)?;
            Ok((arr.data, arr.bentuk[0], arr.bentuk[1]))
        }
        _ => Err("Data harus berupa MatraArray atau MatraTabel".to_string()),
    }
}
