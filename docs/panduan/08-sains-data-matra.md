---
judul: "Ekosistem Sains Data & ML (Matra)"
grup: "Panduan"
urutan: 8
---

# Ekosistem Sains Data & ML (Matra)

## Gambaran Umum

**Matra** adalah pustaka ekosistem resmi terpadu di EvernightLanguage untuk komputasi numerik,
analisis data tabular, visualisasi grafik, machine learning klasik, dan deep learning.

Filosofi utama Matra adalah **"Satu Pustaka untuk Seluruh Siklus Sains Data"**:
menghilangkan fragmentasi dependensi (tidak perlu memasang NumPy, Pandas, Matplotlib,
Scikit-learn, dan PyTorch secara terpisah). Semua komponen terintegrasi langsung
di dalam compiler dan runtime EvernightLanguage.

## Arsitektur Dua Lapis (*Two-Tier Architecture*)

1. **Lapisan Bawah (Rust Core Engine)**:
   - Data tensor disimpan dalam blok memori kontinu datar (*flat contiguous buffer* `Vec<f64>`).
   - Eksekusi perkalian matriks, pencarian spasial KD-Tree (KNN), dan turunan diferensial otomatis (*autograd*) berjalan pada kecepatan native C++/Rust.
   - Tanpa *garbage collector pauses* karena dikelola oleh model Automatic Reference Counting (ARC).

2. **Lapisan Atas (Sintaksis Evernight `.eve`)**:
   - Panggilan ekspresif dalam Bahasa Indonesia.
   - Format error informatif berbahasa Indonesia (contoh: `BAHAYA [MATRA_SHAPE]: Dimensi matriks tidak cocok untuk perkalian!`).

## Struktur Sub-Modul

| Sub-Modul | Peran Utama | Padanan di Python |
|---|---|---|
| `matra.larik` | Tensor n-dimensi, aljabar linier, operasi statistik | NumPy, SciPy |
| `matra.tabel` | Struktur `Tabel` (DataFrame), I/O CSV lokal, filter & groupby | Pandas, Polars |
| `matra.grafik` | Pembuat visualisasi vektor SVG mandiri | Matplotlib, Seaborn |
| `matra.latih` & `matra.alur` | Pemisahan data, validasi silang K-Fold, Pipeline anti-bocor | Scikit-learn (`model_selection`, `Pipeline`) |
| `matra.seimbang` | Penyeimbangan kelas sampel data minoritas (`SMOTE`) | Imbalanced-learn (`imblearn`) |
| `matra.model` | Model regresi, klasifikasi KNN/Tree/Forest, clustering KMeans | Scikit-learn (`linear_model`, `ensemble`, dll) |
| `matra.metrik` | Metrik evaluasi akurasi, F1, MSE, dan laporan klasifikasi | Scikit-learn (`metrics`) |
| `matra.syaraf` | Autograd engine, lapisan Neural Network, aktivasi & pengoptimal | PyTorch, TensorFlow |
| `matra.pelatih` | Orkestrator siklus latih otomatis, Early Stopping, Checkpointing | PyTorch Lightning, Keras |

## Contoh Alur Kerja Sains Data Terpadu

```eve
impor "matra" sebagai mt

# 1. Membaca data tabular CSV
variabel df = mt.baca_csv("dataset.csv")

# 2. Pembersihan & persiapan data
variabel bersih = df.buang_kosong()
variabel X = bersih.ke_larik(["fitur1", "fitur2"])
variabel y = bersih.ke_larik(["target"])

# 3. Pemisahan data latih & uji
variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, 0.2)

# 4. Melatih model
variabel model = mt.model.KNNKlasifikasi(5)
model.latih(X_latih, y_latih)

# 5. Evaluasi performa
variabel y_pred = model.prediksi(X_uji)
cetak("Akurasi Uji: ", mt.metrik.akurasi(y_uji, y_pred))
```
