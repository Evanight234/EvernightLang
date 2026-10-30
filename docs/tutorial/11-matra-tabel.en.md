---
judul: "Stage 11: Tabular Data Analysis & CSV"
grup: "Tutorial"
urutan: 11
---

# Stage 11: Tabular Data Analysis & CSV

## Goal

Manipulate tabular data (*DataFrames*) using the `matra.tabel` module:
read local CSV files, select columns and rows, clean missing values, and
perform data aggregation (*groupby*).

## Concepts

### Creating a `Tabel` (DataFrame)

A `Tabel` object can be constructed directly from a dictionary of columns:

```eve
impor "matra" sebagai mt

variabel data = mt.tabel({
    "nama": ["Andi", "Budi", "Citra", "Dewi"],
    "usia": [17, 18, 17, 19],
    "nilai": [85.5, 92.0, 78.0, 95.5]
})
cetak(data)
```

### Reading & Saving CSV Files

Matra includes a high-speed CSV parser that reads directly from the user local laptop storage:

```eve
# Membaca berkas CSV
variabel df = mt.baca_csv("data_siswa.csv")

# Menyimpan kembali ke berkas CSV
df.simpan_csv("siswa_bersih.csv")
```

### Inspecting Data

Use inspection methods to quickly understand the table layout:

```eve
# 5 baris pertama
df.kepala(5)

# Ringkasan statistik kolom angka
df.ringkasan()

# Dimensi tabel [jumlah_baris, jumlah_kolom]
cetak("Bentuk tabel: ", df.bentuk)
```

### Selecting & Manipulating Columns

Access and alter columns through structured methods:

```eve
# Mengambil kolom tertentu
variabel usia_siswa = df.kolom("usia")

# Memilih subset kolom
variabel sub = df.pilih_kolom(["nama", "nilai"])

# Menambah kolom baru
df.tambah_kolom("status", ["Lulus", "Lulus", "Lulus", "Lulus"])
```

### Handling Missing Values

Raw datasets often have missing entries (`kosong`):

```eve
# Membuang baris yang memiliki nilai kosong
variabel bersih = df.buang_kosong()

# Mengisi nilai kosong dengan nilai pengganti
variabel terisi = df.isi_kosong("usia", 17)
```

### Aggregation & Grouping (*Groupby*)

Group rows by categorical values and compute aggregate summaries:

```eve
variabel rekap = df.kelompokkan("jurusan").rata_rata()
cetak(rekap)
```

## Complete example

```eve
impor "matra" sebagai mt

# 1. Membuat DataFrame data penjualan toko
variabel penjualan = mt.tabel({
    "cabang": ["Jakarta", "Surabaya", "Jakarta", "Bandung", "Surabaya"],
    "produk": ["Buku", "Pensil", "Buku", "Penghapus", "Buku"],
    "jumlah": [12, 25, 18, 15, 30],
    "harga": [15000, 3000, 15000, 2500, 15000]
})

cetak("--- Data Penjualan Awal ---")
penjualan.kepala(5)

# 2. Menghitung omzet total (jumlah * harga)
variabel omzet = []
variabel jml = penjualan.kolom("jumlah")
variabel hrg = penjualan.kolom("harga")

untuk i dari 0 sampai jml.panjang - 1 {
    tambah(omzet, jml[i] * hrg[i])
}
penjualan.tambah_kolom("total_omzet", omzet)

# 3. Menampilkan ringkasan statistik
cetak("\n--- Ringkasan Numerik ---")
penjualan.ringkasan()

# 4. Agregasi penjualan berdasarkan cabang
cetak("\n--- Rekap per Cabang ---")
variabel rekap_cabang = penjualan.kelompokkan("cabang").jumlah()
cetak(rekap_cabang)
```

## Run

```powershell
evernight olah_tabel.eve
```

```
--- Data Penjualan Awal ---
[TabelData 5 baris x 4 kolom]
cabang    | produk    | jumlah | harga
Jakarta   | Buku      | 12     | 15000
Surabaya  | Pensil    | 25     | 3000
Jakarta   | Buku      | 18     | 15000
Bandung   | Penghapus | 15     | 2500
Surabaya  | Buku      | 30     | 15000

--- Ringkasan Numerik ---
Statistik kolom jumlah dan harga berhasil dihitung.

--- Rekap per Cabang ---
Rekap agregasi jumlah per cabang berhasil ditampilkan.
```

## Exercises

1. Create a table with 4 classmates containing `nama`, `hobi`, and `jam_tidur`.
2. Filter table rows to only keep records where `jam_tidur >= 7`.
3. Compute the average sleep hours across all students using `.rata_rata()` on the column.
