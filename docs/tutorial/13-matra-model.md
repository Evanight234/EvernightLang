---
judul: "Tahap 13: Machine Learning & Jaringan Syaraf"
grup: "Tutorial"
urutan: 13
---

# Tahap 13: Machine Learning & Jaringan Syaraf

## Tujuan

Membangun alur machine learning terintegrasi penuh: membagi dataset latih dan uji,
melatih model klasifikasi dan regresi klasik (`matra.model`), mengukur akurasi dengan
metrik evaluasi (`matra.metrik`), serta mengenal dasar deep learning (`matra.syaraf`).

## Konsep

### Pembagian Data Latih & Uji (*Train-Test Split*)

Gunakan `mt.latih.pisah_data` untuk membagi fitur dan label menjadi set latih dan set uji:

```eve
impor "matra" sebagai mt

variabel [X_latih, X_uji, y_latih, y_uji] = mt.latih.pisah_data(X, y, 0.2)
```

Parameter `0.2` menandakan 20% data dialokasikan untuk pengujian (*test set*).

### Melatih Model Klasifikasi KNN (*K-Nearest Neighbors*)

Membuat dan melatih model klasifikasi berbasis tetangga terdekat:

```eve
variabel model = mt.model.KNNKlasifikasi(3)
model.latih(X_latih, y_latih)

variabel prediksi = model.prediksi(X_uji)
```

### Evaluasi Metrik & Laporan Klasifikasi

Ukur performa model secara komprehensif menggunakan modul `matra.metrik`:

```eve
variabel skor_akurasi = mt.metrik.akurasi(y_uji, prediksi)
cetak("Akurasi Model: ", skor_akurasi)

variabel laporan = mt.metrik.laporan_klasifikasi(y_uji, prediksi)
cetak(laporan)
```

### Model Regresi Linear

Untuk memprediksi nilai kontinu (angka riil):

```eve
variabel regresi = mt.model.RegresiLinear()
regresi.latih(X_latih, y_latih)

cetak("Koefisien bobot: ", regresi.koefisien)
cetak("Intersep bias: ", regresi.intersep)
```

### Jaringan Syaraf Tiruan (*Deep Learning*)

Membangun arsitektur multi-layer neural network dengan `mt.syaraf`:

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

## Contoh lengkap

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

## Jalankan

```powershell
evernight latih_model.eve
```

```
Data Uji Sebenarnya: Larik([0, 1])
Hasil Prediksi Model: Larik([0, 1])
Tingkat Akurasi: 100%
```

Model berhasil mempelajari pola pemisahan spasial dua klaster data dan memprediksi sampel baru dengan tepat.

## Latihan

1. Tambahkan titik data baru `[1.1, 0.9]` dan `[5.2, 5.1]` lalu uji hasil prediksi kelasnya.
2. Coba ganti parameter jumlah tetangga `K` pada model KNN dari 3 menjadi 1, dan amati perbedaannya.
3. Buat model `mt.model.RegresiLinear()` untuk memprediksi harga rumah sederhana berdasarkan luas tanah.
