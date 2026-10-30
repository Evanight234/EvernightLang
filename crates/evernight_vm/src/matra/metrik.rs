use std::collections::{BTreeSet, HashMap};

/// Menghitung akurasi prediksi: jumlah prediksi benar dibagi total sampel
pub fn akurasi(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err(format!(
            "Panjang y_asli ({}) tidak sama dengan y_pred ({})",
            y_asli.len(),
            y_pred.len()
        ));
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }
    let benar = y_asli
        .iter()
        .zip(y_pred.iter())
        .filter(|(&a, &p)| (a - p).abs() < 1e-6)
        .count();
    Ok(benar as f64 / y_asli.len() as f64)
}

/// Menghitung presisi untuk kelas target atau rata-rata makro
pub fn presisi(y_asli: &[f64], y_pred: &[f64], target_kelas: Option<f64>) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }

    if let Some(target) = target_kelas {
        let mut tp = 0.0;
        let mut fp = 0.0;
        for (&a, &p) in y_asli.iter().zip(y_pred.iter()) {
            let pred_target = (p - target).abs() < 1e-6;
            let asli_target = (a - target).abs() < 1e-6;
            if pred_target && asli_target {
                tp += 1.0;
            } else if pred_target && !asli_target {
                fp += 1.0;
            }
        }
        if tp + fp == 0.0 {
            Ok(0.0)
        } else {
            Ok(tp / (tp + fp))
        }
    } else {
        // Rata-rata makro (Macro Average)
        let unique_classes: BTreeSet<i64> = y_asli.iter().map(|&v| v.round() as i64).collect();
        if unique_classes.is_empty() {
            return Ok(0.0);
        }
        let mut total_presisi = 0.0;
        for &cls in &unique_classes {
            total_presisi += presisi(y_asli, y_pred, Some(cls as f64))?;
        }
        Ok(total_presisi / unique_classes.len() as f64)
    }
}

/// Menghitung recall (sensitivitas) untuk kelas target atau rata-rata makro
pub fn recall(y_asli: &[f64], y_pred: &[f64], target_kelas: Option<f64>) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }

    if let Some(target) = target_kelas {
        let mut tp = 0.0;
        let mut fn_count = 0.0;
        for (&a, &p) in y_asli.iter().zip(y_pred.iter()) {
            let pred_target = (p - target).abs() < 1e-6;
            let asli_target = (a - target).abs() < 1e-6;
            if pred_target && asli_target {
                tp += 1.0;
            } else if !pred_target && asli_target {
                fn_count += 1.0;
            }
        }
        if tp + fn_count == 0.0 {
            Ok(0.0)
        } else {
            Ok(tp / (tp + fn_count))
        }
    } else {
        // Rata-rata makro (Macro Average)
        let unique_classes: BTreeSet<i64> = y_asli.iter().map(|&v| v.round() as i64).collect();
        if unique_classes.is_empty() {
            return Ok(0.0);
        }
        let mut total_recall = 0.0;
        for &cls in &unique_classes {
            total_recall += recall(y_asli, y_pred, Some(cls as f64))?;
        }
        Ok(total_recall / unique_classes.len() as f64)
    }
}

/// Menghitung F1-score (rata-rata harmonik presisi dan recall)
pub fn f1_skor(y_asli: &[f64], y_pred: &[f64], target_kelas: Option<f64>) -> Result<f64, String> {
    let p = presisi(y_asli, y_pred, target_kelas)?;
    let r = recall(y_asli, y_pred, target_kelas)?;
    if p + r == 0.0 {
        Ok(0.0)
    } else {
        Ok(2.0 * (p * r) / (p + r))
    }
}

/// Menghasilkan matriks kontingensi (matriks konfusi) dan label kelasnya
pub fn matriks_konfusi(
    y_asli: &[f64],
    y_pred: &[f64],
) -> Result<(Vec<Vec<f64>>, Vec<String>), String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }

    let mut classes_set = BTreeSet::new();
    for &y in y_asli {
        classes_set.insert(y.round() as i64);
    }
    for &y in y_pred {
        classes_set.insert(y.round() as i64);
    }

    let classes: Vec<i64> = classes_set.into_iter().collect();
    let n = classes.len();
    let class_to_idx: HashMap<i64, usize> =
        classes.iter().enumerate().map(|(i, &c)| (c, i)).collect();

    let mut matrix = vec![vec![0.0; n]; n];

    for (&a, &p) in y_asli.iter().zip(y_pred.iter()) {
        let actual_idx = class_to_idx[&(a.round() as i64)];
        let pred_idx = class_to_idx[&(p.round() as i64)];
        matrix[actual_idx][pred_idx] += 1.0;
    }

    let labels: Vec<String> = classes.iter().map(|c| c.to_string()).collect();
    Ok((matrix, labels))
}

/// Menghasilkan laporan klasifikasi lengkap berformat teks ASCII tabel
pub fn laporan_klasifikasi(y_asli: &[f64], y_pred: &[f64]) -> Result<String, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }

    let mut classes_set = BTreeSet::new();
    for &y in y_asli {
        classes_set.insert(y.round() as i64);
    }
    for &y in y_pred {
        classes_set.insert(y.round() as i64);
    }

    let total_samples = y_asli.len();
    let acc = akurasi(y_asli, y_pred)?;

    let mut s = String::new();
    s.push_str("              presisi     recall    f1-skor    dukungan\n\n");

    let mut total_p = 0.0;
    let mut total_r = 0.0;
    let mut total_f1 = 0.0;
    let mut count = 0;

    for &cls in &classes_set {
        let cls_f = cls as f64;
        let p = presisi(y_asli, y_pred, Some(cls_f))?;
        let r = recall(y_asli, y_pred, Some(cls_f))?;
        let f1 = f1_skor(y_asli, y_pred, Some(cls_f))?;
        let support = y_asli
            .iter()
            .filter(|&&a| (a.round() as i64) == cls)
            .count();

        total_p += p;
        total_r += r;
        total_f1 += f1;
        count += 1;

        let row = format!(
            "{:>12}      {:.2}       {:.2}       {:.2}        {:>4}\n",
            cls, p, r, f1, support
        );
        s.push_str(&row);
    }

    s.push_str("\n");
    s.push_str(&format!(
        "     akurasi                           {:.2}        {:>4}\n",
        acc, total_samples
    ));

    let macro_p = if count > 0 {
        total_p / count as f64
    } else {
        0.0
    };
    let macro_r = if count > 0 {
        total_r / count as f64
    } else {
        0.0
    };
    let macro_f1 = if count > 0 {
        total_f1 / count as f64
    } else {
        0.0
    };

    s.push_str(&format!(
        "   rata makro      {:.2}       {:.2}       {:.2}        {:>4}\n",
        macro_p, macro_r, macro_f1, total_samples
    ));

    Ok(s)
}

// =============================================================================
// METRIK EVALUASI REGRESI
// =============================================================================

/// Mean Squared Error (MSE)
pub fn mse(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }
    let sum_sq: f64 = y_asli
        .iter()
        .zip(y_pred.iter())
        .map(|(&a, &p)| (a - p).powi(2))
        .sum();
    Ok(sum_sq / y_asli.len() as f64)
}

/// Root Mean Squared Error (RMSE)
pub fn rmse(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    Ok(mse(y_asli, y_pred)?.sqrt())
}

/// Mean Absolute Error (MAE)
pub fn mae(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }
    let sum_abs: f64 = y_asli
        .iter()
        .zip(y_pred.iter())
        .map(|(&a, &p)| (a - p).abs())
        .sum();
    Ok(sum_abs / y_asli.len() as f64)
}

/// Koefisien Determinasi R-squared (R2 Score)
pub fn r2_skor(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }
    let n = y_asli.len() as f64;
    let mean_asli = y_asli.iter().sum::<f64>() / n;

    let mut ss_res = 0.0;
    let mut ss_tot = 0.0;

    for (&a, &p) in y_asli.iter().zip(y_pred.iter()) {
        ss_res += (a - p).powi(2);
        ss_tot += (a - mean_asli).powi(2);
    }

    if ss_tot == 0.0 {
        return Ok(1.0); // Variance nol sempurna
    }

    Ok(1.0 - (ss_res / ss_tot))
}

/// Mean Absolute Percentage Error (MAPE)
pub fn mape(y_asli: &[f64], y_pred: &[f64]) -> Result<f64, String> {
    if y_asli.len() != y_pred.len() {
        return Err("Panjang y_asli dan y_pred harus sama".to_string());
    }
    if y_asli.is_empty() {
        return Ok(0.0);
    }
    let mut sum_pct = 0.0;
    let mut valid_n = 0;
    for (&a, &p) in y_asli.iter().zip(y_pred.iter()) {
        if a.abs() > 1e-9 {
            sum_pct += ((a - p) / a).abs();
            valid_n += 1;
        }
    }
    if valid_n == 0 {
        Ok(0.0)
    } else {
        Ok((sum_pct / valid_n as f64) * 100.0)
    }
}
