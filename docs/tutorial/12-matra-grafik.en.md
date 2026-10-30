---
judul: "Stage 12: Vector SVG Data Visualization"
grup: "Tutorial"
urutan: 12
---

# Stage 12: Vector SVG Data Visualization

## Goal

Create standalone vector graphics (*Scalable Vector Graphics* - SVG) using `matra.grafik`:
render line plots, scatter plots, bar charts, and histograms without external C dependencies.

## Concepts

### Creating a Plot Canvas

The `matra.grafik` module uses a fluent method chaining syntax:

```eve
impor "matra" sebagai mt

variabel plot = mt.grafik()
    .judul("Tren Penjualan Bulanan")
    .label_x("Bulan Ke-")
    .label_y("Total Pendapatan (Juta Rp)")
```

### Line Charts

Ideal for demonstrating time-series data or performance progression:

```eve
variabel x = [1, 2, 3, 4, 5]
variabel y = [10, 15, 13, 18, 22]

variabel plot = mt.grafik()
    .judul("Pertumbuhan Pengguna")
    .garis(x, y, "label: Pengguna Aktif")
```

### Scatter Plots

Helpful for observing correlations across two independent numeric variables:

```eve
variabel tinggi = [160, 165, 170, 175, 180]
variabel berat = [55, 62, 68, 74, 82]

variabel plot = mt.grafik()
    .judul("Korelasi Tinggi vs Berat")
    .sebar(tinggi, berat)
```

### Bar Charts

Visually compares categorical metrics:

```eve
variabel kategori = ["Sains", "Teknologi", "Seni", "Bisnis"]
variabel peminat = [120, 310, 95, 240]

variabel plot = mt.grafik()
    .judul("Peminatan Jurusan")
    .batang(kategori, peminat)
```

### Histograms

Shows frequency distributions for continuous numeric data:

```eve
variabel nilai_ujian = [65, 70, 72, 75, 80, 82, 85, 90, 95]

variabel plot = mt.grafik()
    .judul("Distribusi Nilai")
    .histogram(nilai_ujian, 5)
```

### Exporting Plots to SVG

All configured graphs can be rendered directly into clean vector SVG files:

```eve
plot.simpan_svg("grafik_hasil.svg")
```

## Complete example

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

## Run

```powershell
evernight gambar_grafik.eve
```

```
Visualisasi grafik berhasil disimpan ke 'kurva_belajar.svg'
```

Open `kurva_belajar.svg` in your browser (Chrome or Edge) or an image viewer to inspect the sharp vector chart.

## Exercises

1. Create a bar chart using `mt.grafik().batang(...)` comparing your calorie intake from Monday to Sunday.
2. Build a scatter plot showing study hours against test scores across 6 data pairs.
3. Export your plot to `latihan_grafik.svg` and verify the output in your browser.
