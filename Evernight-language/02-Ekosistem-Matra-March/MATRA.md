# MATRA.md — Spesifikasi Pustaka Sains Data & Machine Learning Terpadu

> **Filosofi**: *"Satu pustaka terpadu untuk komputasi numerik, olah data tabular, visualisasi grafik, pelatihan, pengujian, dan kecerdasan buatan di EvernightLanguage."*

---

## 1. Latar Belakang & Visi

Di ekosistem bahasa lain (khususnya Python), alur kerja sains data dan *Machine Learning* (ML) terpecah menjadi banyak pustaka independen yang harus dipasang dan dikonfigurasi satu per satu:
* **NumPy** untuk array multi-dimensi dan matematika matriks.
* **Pandas** untuk membaca CSV dan memanipulasi tabel (*DataFrame*).
* **Matplotlib / Seaborn** untuk menggambar grafik dan visualisasi data.
* **Scikit-learn** untuk *data splitting*, algoritma ML klasik, pipeline, dan metrik pengujian.
* **Optuna / Hyperopt** untuk pencarian hyperparameter otomatis (*hyperparameter tuning*).
* **Imbalanced-learn** untuk menangani data latih yang tidak seimbang (*SMOTE*).
* **Yellowbrick / Scikit-plot** untuk visualisasi diagnostik pengujian (*Learning Curves*, *ROC Curve*).
* **PyTorch Lightning / Keras** untuk orkestrasi siklus *train-validate-test*, *Early Stopping*, dan *Model Checkpoint*.
* **Deepchecks / Evidently AI** untuk pengujian integritas data, kebocoran data (*data leakage*), dan deteksi pergeseran data (*data drift*).
* **MLflow / TensorBoard** untuk pelacakan metrik pelatihan dan pengujian per putaran (*epoch history*).

Masalah yang sering muncul dari fragmentasi tersebut adalah inkonsistensi tipe data, bentrok versi dependensi (*dependency hell*), ukuran instalasi yang membengkak, serta sintaksis yang berbeda-beda.

**Matra** hadir di EvernightLanguage sebagai ekosistem *all-in-one* bawaan yang modular dan terintegrasi penuh. Semua komponen dirancang menggunakan satu konvensi tipe data yang serasi, memanfaatkan performa memori kontinu Rust di balik layar, dan diakses menggunakan sintaks Bahasa Indonesia yang ekspresif.

---

## 2. Pemetaan Lengkap Pustaka Python ke Modul Matra

Berikut pemetaan lengkap pustaka ekosistem Python ke dalam sub-modul terpadu **Matra**:

| Kategori Alur Kerja | Pustaka Python Populer | Modul di Matra | Deskripsi & Peran Utama |
|---|---|---|---|
| **Komputasi Vektor & Matriks** | NumPy, SciPy | `matra.larik` / `matra.tensor` | Struktur data array homogen n-dimensi kontinu, aljabar linier, *broadcasting*, operasi vektor, kalkulasi statistik. |
| **Manipulasi Data Tabular** | Pandas, Polars | `matra.tabel` | Struktur `TabelData` (DataFrame), pembacaan/penulisan CSV dan JSON, filter baris/kolom, agregasi `kelompokkan`, *missing values*. |
| **Visualisasi Data & Diagnostik** | Matplotlib, Seaborn, Yellowbrick | `matra.grafik` | Grafik garis, sebar (*scatter*), batang, histogram, *heatmap*, kurva belajar (*learning curves*), kurva ROC. Ekspor ke SVG vektor atau penampil terminal ASCII. |
| **Pembagian Data & Validasi** | Scikit-learn (`model_selection`) | `matra.latih` | Pembagian data latih/validasi/uji (`pisah_data`), *K-Fold Cross Validation*, *Stratified K-Fold*, *Time Series Split*. |
| **Penyetelan Hyperparameter** | Optuna, Hyperopt, GridSearchCV | `matra.tala` / `matra.latih` | Optimasi hyperparameter otomatis (Pencarian Kisi, Pencarian Acak, dan Optimasi Bayesian / *TalaBayes*). |
| **Penyeimbangan Data Latih** | Imbalanced-learn (`imblearn`) | `matra.seimbang` | Penanganan data tidak seimbang untuk pelatihan: `SMOTE`, `KurangSampelAcak`, `LebihSampelAcak`. |
| **Pipeline & Anti-Kebocoran** | Scikit-learn (`Pipeline`) | `matra.alur` | Merangkai transformasi dan model ke dalam satu pipa kerja (*pipeline*) untuk mencegah kebocoran data (*data leakage*) dari data uji ke data latih. |
| **Pengujian Integritas & Drift** | Deepchecks, Evidently AI | `matra.uji` | Pengujian kualitas data: deteksi duplikasi latih-uji, kebocoran target, dan uji perbedaan distribusi (*Data Drift*). |
| **Algoritma ML Klasik** | Scikit-learn | `matra.model` | Regresi Linear/Logistik, KNN (*K-Nearest Neighbors*), Pohon Keputusan, Hutan Acak, K-Means, SVM, Naive Bayes. |
| **Metrik Evaluasi & Uji** | Scikit-learn (`metrics`) | `matra.metrik` | Laporan klasifikasi (Akurasi, Presisi, Recall, F1, Matriks Konfusi), evaluasi regresi (MSE, RMSE, MAE, R2, MAPE). |
| **Deep Learning & Autograd** | PyTorch, TensorFlow | `matra.syaraf` | Diferensiasi otomatis (*Autograd tape*), lapisan neural network (`Linear`, `Konvolusi`), aktivasi, dan pengoptimal (`SGD`, `Adam`). |
| **Orkestrasi Latih & Callbacks** | PyTorch Lightning, Keras | `matra.pelatih` | Pengelola siklus latih-validasi-uji otomatis: *Early Stopping*, *Model Checkpoint* (simpan bobot terbaik), dan pelacakan metrik per putaran. |

---

## 3. Arsitektur Dua Lapis (*Two-Tier Architecture*)

Agar komputasi numerik dan pelatihan model di Evernight secepat C++ dan seringan Rust:

```
┌────────────────────────────────────────────────────────────────────────┐
│                      SINTAKSIS EVERNIGHT (.eve)                       │
│      impor "matra" sebagai mt                                         │
│      variabel [latih, uji] = mt.latih.pisah_data(data, rasio: 0.2)   │
│      variabel pelatih = mt.pelatih.Pelatih(model)                     │
│      pelatih.latih(latih, uji, callbacks=[mt.pelatih.BerhentiAwal()]) │
└───────────────────────────────────┬────────────────────────────────────┘
                                    │ Panggilan Native / Opcode Cepat
┌───────────────────────────────────▼────────────────────────────────────┐
│                       ENGINE MATRA (RUST CORE)                         │
│  - Memori Flat Kontinu: Buffer `Vec<f64>` tanpa overhead boxing        │
│  - Akselerasi Spasial: Struktur KD-Tree & Ball-Tree (KNN Cepat)        │
│  - Vektorisasi SIMD & Multithreading (Rayon/std::thread)               │
│  - Parser CSV Zero-Allocation                                          │
│  - Generator Grafik SVG & Terminal ASCII Vektor Zero-Dep              │
│  - Engine Autograd & Graph Tape (Backpropagation Cepat)                │
│  - Mesin Matriks: Perkalian Titik (Dot Product) & Transposisi SIMD     │
└────────────────────────────────────────────────────────────────────────┘
```

1. **Lapisan Bawah (Rust Engine)**:
   * Menyimpan matriks/tensor sebagai array kontinu datar berkecepatan native.
   * Eksekusi perkalian matriks, normalisasi, pencarian tetangga terdekat (KD-Tree), dan perhitungan gradien berjalan pada level Rust murni (multithreaded & SIMD-ready).
   * Meniadakan overhead alokasi dinamis per instruksi VM.
2. **Lapisan Atas (Evernight API)**:
   * Menyediakan API seragam dengan metode berantai (*method chaining*).
   * Error terstandarisasi dengan Bahasa Indonesia (misal: `BAHAYA [MATRA_LEAKAGE]: Terdeteksi sampel data uji yang bocor ke data latih!`).

---

## 4. Alur Kerja Khusus: Data Training & Data Testing

### 4.1. Pembagian Data Latih, Validasi, dan Uji (*Splitting & Stratification*)
```eve
impor "matra" sebagai mt

variabel data = mt.baca_csv("nasabah_kredit.csv")
variabel X = data.kecuali(["gagal_bayar"])
variabel y = data["gagal_bayar"]

# Pembagian 3 arah: 70% Latih, 15% Validasi, 15% Uji dengan stratifikasi kelas
variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(
    X, y, 
    rasio_uji: 0.15, 
    acak: 42, 
    stratifikasi: benar
)

variabel [X_latih, X_val, y_latih, y_val] = mt.latih.pisah_data(
    X_latih, y_latih, 
    rasio_uji: 0.176, # 0.176 dari 85% = ~15% total
    acak: 42
)
```

### 4.2. Penyeimbangan Data Latih (Pengganti Imbalanced-Learn)
Mencegah model hanya condong ke kelas mayoritas:
```eve
# SMOTE hanya diterapkan pada data latih (bukan data uji!)
variabel penyeimbang = mt.seimbang.SMOTE(rasio: 1.0, acak: 42)
variabel [X_latih_seimbang, y_latih_seimbang] = penyeimbang.sesuaikan(X_latih, y_latih)
```

### 4.3. Pipeline Kerja Terpadu (Pengganti Scikit-Learn Pipeline)
Mencegah kebocoran data (*data leakage*) di mana statistik data uji tidak sengaja terhitung saat normalisasi data latih:
```eve
variabel pipa = mt.alur.PipaKerja([
    ["skala", mt.pra.Standarisasi()],
    ["model", mt.model.PohonKeputusan(kedalaman_maks: 8)]
])

# Fit otomatis hanya pada data latih, transformasi pada data uji
pipa.latih(X_latih_seimbang, y_latih_seimbang)
variabel prediksi_uji = pipa.prediksi(X_uji)
```

### 4.4. Validasi Silang (*Cross Validation*) & Penyetelan Hyperparameter (Pengganti Optuna / GridSearch)
```eve
# Validasi Silang 5-Lipatan
variabel k_lipat = mt.latih.LipatanBerlapis(k: 5, acak: 42)
variabel skor_cv = mt.latih.validasi_silang(pipa, X_latih, y_latih, cv: k_lipat, metrik: "f1")
cetak("Rata-rata Skor F1 CV: ", skor_cv.rata_rata())

# Penyetelan Hyperparameter Bayesian Otomatis (TalaBayes)
variabel tala = mt.tala.TalaBayes(
    model: mt.model.HutanAcak(),
    ruang_parameter: {
        "jumlah_pohon": [50, 100, 200],
        "kedalaman_maks": [5, 10, 15, 20],
        "laju_belajar": [0.01, 0.05, 0.1]
    },
    iterasi: 30,
    metrik: "f1"
)
tala.optimasi(X_latih, y_latih)
cetak("Parameter Terbaik: ", tala.parameter_terbaik)
```

### 4.5. Pengujian Model & Diagnostik Visual (Pengganti Yellowbrick & Deepchecks)
```eve
# 1. Laporan Pengujian Lengkap
variabel laporan = mt.metrik.laporan_klasifikasi(y_uji, prediksi_uji)
cetak(laporan)
# Output:
#               presisi    recall    f1-skor    jumlah
# Kelas 0          0.92      0.95       0.93       850
# Kelas 1          0.84      0.76       0.80       150
# Akurasi                               0.91      1000

# 2. Kurva Belajar (Deteksi Overfitting vs Underfitting)
variabel kurva = mt.grafik.kurva_belajar(pipa, X_latih, y_latih, cv: 5)
kurva.simpan_svg("kurva_belajar.svg")
kurva.tampil_terminal() # Grafik ASCII di konsol

# 3. Uji Kebocoran & Pergeseran Data (Data Drift)
variabel hasil_drift = mt.uji.cek_pergeseran(X_latih, X_uji)
jika hasil_drift.ada_pergeseran {
    cetak("PERINGATAN: Distribusi fitur ", hasil_drift.kolom_geser, " berbeda signifikan!")
}
```

### 4.6. Pelatihan Jaringan Syaraf dengan Callbacks (Pengganti PyTorch Lightning / Keras)
```eve
variabel model_jst = mt.syaraf.Model([
    mt.syaraf.Linear(10, 32),
    mt.syaraf.Relu(),
    mt.syaraf.Dropout(0.2),
    mt.syaraf.Linear(32, 1),
    mt.syaraf.Sigmoid()
])

# Pelatih otomatis dengan Early Stopping dan Checkpointing
variabel pelatih = mt.pelatih.Pelatih(
    model: model_jst,
    pengoptimal: mt.syaraf.Adam(model_jst.parameter(), laju_belajar: 0.001),
    kerugian: mt.syaraf.KerugianBCE(),
    metrik: ["akurasi", "f1"]
)

pelatih.pasang_callback([
    # Berhenti jika kerugian validasi tidak turun selama 5 putaran berturut-turut
    mt.pelatih.BerhentiAwal(pantau: "rugi_val", kesabaran: 5),
    # Simpan bobot model terbaik
    mt.pelatih.SimpanTerbaik("model_terbaik.matra", pantau: "f1_val", mode: "maks"),
    # Catat riwayat pelatihan
    mt.pelatih.PencatatRiwayat("pelatihan_log.json")
])

# Jalankan pelatihan & evaluasi data uji secara otomatis
variabel riwayat = pelatih.latih(
    data_latih: [X_latih, y_latih],
    data_validasi: [X_val, y_val],
    putaran: 100,
    ukuran_batch: 32
)

# Pengujian final pada test set murni
variabel hasil_uji = pelatih.evaluasi(X_uji, y_uji)
cetak("Akurasi Data Uji: ", hasil_uji["akurasi"])
```

### 4.7. Studi Kasus Klasifikasi: K-Nearest Neighbors (KNN)
Model klasifikasi non-parametrik berbasis tetangga terdekat dengan akselerasi pencarian pohon multidimensi (KD-Tree / Ball-Tree) di Rust engine:
```eve
impor "matra" sebagai mt

# 1. Dataset Bunga Iris (Klasifikasi Multi-Kelas)
variabel data = mt.baca_csv("iris.csv")
variabel X = data[["panjang_sepal", "lebar_sepal", "panjang_petal", "lebar_petal"]]
variabel y = data["spesies"] # "setosa", "versicolor", "virginica"

# 2. Pembagian Data Latih dan Uji
variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, rasio_uji: 0.25, acak: 42)

# 3. Pipeline KNN: Normalisasi Jarak + Model KNNKlasifikasi
# KNN sangat sensitif terhadap skala fitur, sehingga Standarisasi wajib digunakan
variabel model_knn = mt.alur.PipaKerja([
    ["skala", mt.pra.Standarisasi()],
    ["knn", mt.model.KNNKlasifikasi(
        k: 5,                      # Jumlah tetangga terdekat
        metrik_jarak: "euclidean", # "euclidean", "manhattan", atau "minkowski"
        bobot: "jarak",            # "seragam" (uniform) atau "jarak" (distance weighted: 1/d)
        algoritma: "kdtree"        # "otomatis", "brute", atau "kdtree" (akselerasi Rust O(log N))
    )]
])

# 4. Pelatihan Model (Membangun Struktur Spasial KD-Tree di Rust)
model_knn.latih(X_latih, y_latih)

# 5. Pengujian & Prediksi
variabel prediksi = model_knn.prediksi(X_uji)
cetak("Akurasi KNN: ", mt.metrik.akurasi(y_uji, prediksi))

# 6. Menampilkan Matriks Konfusi
variabel matriks = mt.metrik.matriks_konfusi(y_uji, prediksi)
cetak(matriks)
```

---

## 5. Rincian Sub-Fase Pelaksanaan (Fase 9 di Rencana)

Pengembangan **Matra** dibagi menjadi 7 sub-fase terstruktur:

### Sub-Fase 9A: `matra.larik` — Engine Tensor & Aljabar Linier
* [x] Tipe data primitif baru di Rust: `MatraArray` (buffer flat `f64` 1D/2D).
* [x] Operasi matematika vektor: `+`, `-`, `*`, `/`, `titik`, transposisi `.T`, *broadcasting*, mutasi `arr[i] = val`.
* [x] Fungsi pembuat array: `nol([b, k])`, `satu([b, k])`, `penuh(...)`, `rentang(...)`, `spasi_linear(...)`, `identitas(...)`, `acak([b, k])`.
* [x] Kalkulasi aljabar linier & statistik: `titik` (dot product 1D/2D), `invers` (Gauss-Jordan), `determinan`, `jumlah`, `rata_rata`, `min`, `max`, `median`, `varian`, `deviasi_standar` (dukungan sumbu 0 dan 1).
* [x] Manipulasi bentuk & matematika elemen: `ubah_bentuk`, `ratakan`, `gabung`, `akar`, `mutlak`, `sin`, `cos`, `tan`, `exp`, `log`, `ke_daftar()`.

### Sub-Fase 9B: `matra.tabel` — Pengolah Data Tabular (DataFrame)
* [ ] Parser CSV mandiri di Rust (zero-allocation, performa tinggi).
* [ ] Tipe `TabelData`: penyimpanan kolom (*columnar storage*).
* [ ] Operasi filter, pilih kolom, tambah/hapus kolom, pengurutan.
* [ ] Penanganan nilai kosong: `isi_kosong`, `buang_kosong`.
* [ ] Agregasi grup: `kelompokkan` (*groupby*).

### Sub-Fase 9C: `matra.grafik` — Visualisasi Data & Diagnostik Pengujian
* [ ] Generator grafik vektor SVG mandiri (bebas dependensi eksternal C/GUI).
* [ ] Grafik analitis: Garis, Titik Sebar, Batang, Histogram, *Heatmap Matriks Konfusi*.
* [ ] Grafik diagnostik pelatihan: Kurva Belajar (*Learning Curve*), Kurva ROC-AUC.
* [ ] Penampil grafik ASCII langsung di terminal untuk pengujian cepat.

### Sub-Fase 9D: `matra.latih`, `matra.alur` & `matra.uji` — Suite Pelatihan & Pengujian Data
* [ ] Pembagian data: `pisah_data` (Latih/Uji/Validasi, acak, stratifikasi).
* [ ] Validasi silang: `LipatanK`, `LipatanBerlapis`, `LipatanWaktu`.
* [ ] Penyeimbang data: `SMOTE` dan *resampling*.
* [ ] Pipa kerja: `PipaKerja` anti kebocoran data (*data leakage prevention*).
* [ ] Diagnostik integritas: Uji kebocoran data dan deteksi pergeseran fitur (*Data Drift*).
* [ ] Penyetelan hyperparameter: `PencarianKisi`, `PencarianAcak`, `TalaBayes`.

### Sub-Fase 9E: `matra.model` & `matra.metrik` — Model ML Klasik & Evaluasi
* [ ] Pra-pemrosesan: `Standarisasi`, `MinMaksSkala`, `KategoriKeAngka` (*One-Hot*).
* [ ] Model Klasifikasi & Regresi: `RegresiLinear`, `RegresiLogistik`, `KNNKlasifikasi`, `KNNRegresi`, `PohonKeputusan`, `HutanAcak`, `KMeans`.
* [ ] Mesin spasial KNN: Implementasi pencarian tetangga cepat berbasis **KD-Tree** dan **Ball-Tree** di level Rust engine.
* [ ] Metrik evaluasi komprehensif: `laporan_klasifikasi`, `matriks_konfusi`, `akurasi`, `presisi`, `recall`, `f1_skor`, `mse`, `mae`, `r2`.

### Sub-Fase 9F: `matra.syaraf` & `matra.pelatih` — Deep Learning & Otomatisasi Training
* [ ] Engine *Autograd*: Pelacakan *computational graph* dan *backpropagation*.
* [ ] Lapisan Jaringan: `Linear`, `Dropout`, `NormalisasiBatch`.
* [ ] Aktivasi: `Relu`, `Sigmoid`, `Tanh`, `Gelu`, `Softmax`.
* [ ] Pengoptimal: `SGD`, `Adam`, `AdamW`.
* [ ] Orkestrasi `Pelatih`: Siklus latih-validasi otomatis, *Early Stopping*, *Model Checkpoint*, pencatat riwayat per putaran.
* [ ] Simpan dan muat bobot model serial (`.matra`).

### Sub-Fase 9G: Rilis & Distribusi Pembaruan Sistem via GitHub (`update evernight system`)
* [ ] Pemaketan biner rilis mandiri teroptimasi (`evernight.exe` + runtime Matra penuh).
* [ ] Sinkronisasi manifes online `version/version` dan `version/catatan-<versi>.txt` di repositori GitHub `Evanight234/EvernightLang`.
* [ ] Pengujian rilis auto-update: User di sembarang folder di Windows (misal di root `C:\` atau terminal CMD) dapat langsung memperbarui instalasi sistemnya hanya dengan mengetik perintah:
  ```cmd
  update evernight system
  ```
  Sistem akan otomatis mendeteksi rilis baru di GitHub, mengunduh file biner terbaru, menimpa `%LOCALAPPDATA%\Programs\Evernight\bin\evernight.exe`, dan langsung mengaktifkan fitur `matra` secara global.

---

## 6. Integrasi dengan Ekosistem Evernight
Matra didistribusikan sebagai modul pustaka tingkat pertama (*first-class library*) bawaan EvernightLanguage, dapat langsung digunakan via `impor "matra"` tanpa perlu instalasi rumit, serta siap dipaketkan via `evernight pkg`.
