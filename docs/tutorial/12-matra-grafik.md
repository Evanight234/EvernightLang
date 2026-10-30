---
judul: "Tahap 12: Visualisasi Data Grafik SVG"
grup: "Tutorial"
urutan: 12
---

# Tahap 12: Visualisasi Data Grafik SVG

## Tujuan

Membuat visualisasi data grafik vektor (*Scalable Vector Graphics* - SVG) mandiri
menggunakan `matra.grafik`: membuat plot garis, sebar (*scatter*), diagram batang,
dan histogram tanpa ketergantungan pustaka C eksternal.

## Konsep

### Membuat Kanvas Grafik

Pustaka `matra.grafik` menggunakan antarmuka pemanggilan berantai (*method chaining*):

```eve
impor "matra" sebagai mt

variabel plot = mt.grafik()
    .judul("Tren Penjualan Bulanan")
    .label_x("Bulan Ke-")
    .label_y("Total Pendapatan (Juta Rp)")
```

### Grafik Garis (*Line Chart*)

Ideal untuk menunjukkan data deret waktu (*time series*) atau tren:

```eve
variabel x = [1, 2, 3, 4, 5]
variabel y = [10, 15, 13, 18, 22]

variabel plot = mt.grafik()
    .judul("Pertumbuhan Pengguna")
    .garis(x, y, "label: Pengguna Aktif")
```

### Grafik Titik Sebar (*Scatter Plot*)

Berguna untuk melihat korelasi antara dua variabel independen:

```eve
variabel tinggi = [160, 165, 170, 175, 180]
variabel berat = [55, 62, 68, 74, 82]

variabel plot = mt.grafik()
    .judul("Korelasi Tinggi vs Berat")
    .sebar(tinggi, berat)
```

### Diagram Batang (*Bar Chart*)

Menampilkan perbandingan antar-kategori secara visual:

```eve
variabel kategori = ["Sains", "Teknologi", "Seni", "Bisnis"]
variabel peminat = [120, 310, 95, 240]

variabel plot = mt.grafik()
    .judul("Peminatan Jurusan")
    .batang(kategori, peminat)
```

### Histogram

Menampilkan sebaran distribusi frekuensi data numerik:

```eve
variabel nilai_ujian = [65, 70, 72, 75, 80, 82, 85, 90, 95]

variabel plot = mt.grafik()
    .judul("Distribusi Nilai")
    .histogram(nilai_ujian, 5)
```

### Menyimpan Grafik ke Berkas SVG

Semua konfigurasi grafik dapat langsung diekspor menjadi berkas SVG beresolusi tajam:

```eve
plot.simpan_svg("grafik_hasil.svg")
```

## Contoh lengkap

```eve
impor "matra" sebagai mt

# 1. Menyiapkan data performa model machine learning per putaran (epoch)
variabel putaran = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
variabel akurasi_latih = [0.55, 0.68, 0.76, 0.82, 0.87, 0.91, 0.93, 0.95, 0.96, 0.98]
variabel akurasi_uji = [0.52, 0.64, 0.72, 0.79, 0.83, 0.85, 0.87, 0.88, 0.88, 0.89]

# 2. Menggambar kurva belajar (Learning Curves)
variabel g = mt.grafik()
    .judul("Kurva Belajar Model Klasifikasi")
    .label_x("Putaran (Epoch)")
    .label_y("Akurasi")
    .garis(putaran, akurasi_latih, "label: Data Latih")
    .garis(putaran, akurasi_uji, "label: Data Uji")

# 3. Menyimpan visualisasi ke berkas SVG lokal
g.simpan_svg("kurva_belajar.svg")
cetak("Visualisasi grafik berhasil disimpan ke 'kurva_belajar.svg'")
```

## Jalankan

```powershell
evernight gambar_grafik.eve
```

```
Visualisasi grafik berhasil disimpan ke 'kurva_belajar.svg'
```

Buka berkas `kurva_belajar.svg` di peramban web (Chrome / Edge) atau peninjau gambar untuk melihat grafik garis vektor tajam yang terbentuk.

## Latihan

1. Buat diagram batang menggunakan `mt.grafik().batang(...)` yang membandingkan konsumsi kalori Anda dari hari Senin sampai Minggu.
2. Buat scatter plot hubungan antara jumlah jam belajar dengan skor tes menggunakan 6 pasangan data.
3. Ekspor grafik tersebut ke berkas `latihan_grafik.svg` dan periksa di browser.
