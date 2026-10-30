---
judul: "Stage 13: Machine Learning & Neural Networks"
grup: "Tutorial"
urutan: 13
---

# Stage 13: Machine Learning & Neural Networks

## Goal

Construct an end-to-end machine learning pipeline: partition training and testing data,
train classical classification and regression models (`matra.model`), assess performance
via evaluation metrics (`matra.metrik`), and explore deep learning foundations (`matra.syaraf`).

## Concepts

### Train-Test Splitting

Use `mt.latih.pisah_data` to partition features and target labels into distinct train and test splits:

```eve
impor "matra" sebagai mt

variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, 0.2)
```

The parameter `0.2` reserves 20% of the observations for the test set.

### Training a KNN Classifier

Construct and fit a K-Nearest Neighbors classification model:

```eve
variabel model = mt.model.KNNKlasifikasi(3)
model.latih(X_latih, y_latih)

variabel prediksi = model.prediksi(X_uji)
```

### Evaluation Metrics & Classification Report

Measure model performance accurately with `matra.metrik`:

```eve
variabel skor_akurasi = mt.metrik.akurasi(y_uji, prediksi)
cetak("Akurasi Model: ", skor_akurasi)

variabel laporan = mt.metrik.laporan_klasifikasi(y_uji, prediksi)
cetak(laporan)
```

### Linear Regression

For continuous value forecasting (real numbers):

```eve
variabel regresi = mt.model.RegresiLinear()
regresi.latih(X_latih, y_latih)

cetak("Koefisien bobot: ", regresi.koefisien)
cetak("Intersep bias: ", regresi.intersep)
```

### Neural Networks (Deep Learning)

Assemble multi-layer neural network architectures with `mt.syaraf`:

```eve
variabel jst = mt.syaraf.ModelSyaraf([
    mt.syaraf.Linear(2, 4),
    mt.syaraf.Relu(),
    mt.syaraf.Linear(4, 1),
    mt.syaraf.Sigmoid()
])

variabel pengoptimal = mt.syaraf.Adam(jst, 0.01)
variabel pelatih = mt.pelatih.Pelatih(jst, pengoptimal, mt.syaraf.KerugianMSE())
pelatih.latih(X_latih, y_latih, 100, 4)
```

## Complete example

```eve
impor "matra" sebagai mt

# 1. Menyiapkan data fitur (X) dan label kelas (y)
variabel X = mt.larik([
    [1.0, 1.1],
    [1.2, 0.9],
    [0.8, 1.0],
    [5.0, 5.2],
    [5.1, 4.8],
    [4.9, 5.0]
])
variabel y = mt.larik([0, 0, 0, 1, 1, 1])

# 2. Membagi data menjadi 70% latih dan 30% uji
variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, 0.3)

# 3. Melatih model KNN dengan 3 tetangga
variabel model = mt.model.KNNKlasifikasi(3)
model.latih(X_latih, y_latih)

# 4. Melakukan prediksi pada data uji
variabel hasil_prediksi = model.prediksi(X_uji)
cetak("Data Uji Sebenarnya: ", y_uji)
cetak("Hasil Prediksi Model: ", hasil_prediksi)

# 5. Menghitung akurasi prediksi
variabel nilai_akurasi = mt.metrik.akurasi(y_uji, hasil_prediksi)
cetak("Tingkat Akurasi: ", nilai_akurasi * 100, "%")
```

## Run

```powershell
evernight latih_model.eve
```

```
Data Uji Sebenarnya: Larik([0, 1])
Hasil Prediksi Model: Larik([0, 1])
Tingkat Akurasi: 100%
```

The model successfully identifies the spatial separation between clusters and accurately labels new observations.

## Exercises

1. Introduce new observations `[1.1, 0.9]` and `[5.2, 5.1]` and evaluate their predicted classes.
2. Adjust the number of neighbors `K` from 3 to 1 and observe the differences.
3. Build a simple `mt.model.RegresiLinear()` model to predict house prices from floor area.
