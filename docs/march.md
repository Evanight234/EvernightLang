---
judul: "Web Notebook March (.emlbn)"
grup: "Mulai"
urutan: 5
---

# Web Notebook March (.emlbn)

## Apa itu March?

**March** adalah lingkungan kerja *notebook* web interaktif resmi untuk EvernightLanguage,
terinspirasi dari antarmuka modern **Google Colab** (Dark Mode).

March dirancang khusus untuk eksplorasi data, eksperimen Machine Learning, dan pembuatan
prototipe cepat dengan pustaka **`Matra`**, langsung dari peramban web Anda tanpa ketergantungan
layanan cloud pihak ketiga.

![Antarmuka Web Notebook March](https://raw.githubusercontent.com/Evanight234/EvernightLang/main/assets/march_preview.png)

## Keunggulan Utama

1. **100% Akses Berkas Lokal**:
   - Berbeda dari notebook berbasis cloud yang mengharuskan pengunggahan berkas ke server terisolasi, March membaca dan menulis berkas nyata langsung di harddisk laptop Anda.
   - Perintah `mt.baca_csv("penjualan.csv")` membaca langsung dari direktori kerja Anda.

2. **Format Berkas `.emlbn`**:
   - Berkas notebook disimpan dengan format `.emlbn` (*Evernight Machine Learning Notebook*), berbasis JSON terstruktur yang menyimpan sel kode, teks penjelasan Markdown, dan metadata eksekusi.

3. **Editor Cerdas Dual-Layer**:
   - **Syntax Highlighting Real-Time**: Kode diwarnai otomatis dengan palet tema Nusantara / Colab.
   - **IntelliSense Auto-Complete**: Mengetik awalan kata kunci (misalnya `j`) memunculkan popup saran cerdas (`jika`, `jk`, `jkl`, dll) yang dapat dipilih dengan tombol panah dan `Tab`/`Enter`.

4. **Visualisasi Interaktif (Rich Media)**:
   - Objek `Tabel` (*DataFrame*) dirender menjadi tabel HTML interaktif dengan pagination dan zebra-striping.
   - Objek `Grafik` dirender menjadi visualisasi vektor SVG tajam.

5. **Server Mandiri Zero-Dependency**:
   - Dibangun murni di dalam biner compiler `evernight.exe` menggunakan Rust murni tanpa memerlukan Python, Node.js, atau server eksternal.

## Cara Menjalankan March

Cukup jalankan perintah berikut di terminal:

```powershell
# Membuka Web Notebook March di peramban web default
evernight march

# Membuka atau membuat berkas notebook tertentu
evernight march eksperimen.emlbn

# Menentukan port kustom
evernight march --port 8888
```

## Ekspor Notebook ke Skrip `.eve`

Jika Anda ingin mengonversi seluruh sel kode di dalam berkas `.emlbn` menjadi skrip program Evernight biasa:

```powershell
evernight march ekspor eksperimen.emlbn
```

Perintah di atas akan menghasilkan berkas `eksperimen.eve` yang siap dieksekusi langsung dengan `evernight run eksperimen.eve`.
