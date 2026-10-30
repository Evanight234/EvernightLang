---
judul: "Tahap 11: Analisis Data Tabular & CSV"
grup: "Tutorial"
urutan: 11
---

# Tahap 11: Analisis Data Tabular & CSV

## Tujuan

Mengolah data berbentuk tabel (*DataFrame*) menggunakan pustaka `matra.tabel`:
membaca berkas CSV lokal, memilih kolom dan baris, membersihkan nilai kosong, serta
melakukan agregasi data (*groupby*).

## Konsep

### Membuat `Tabel` (DataFrame)

Objek `Tabel` dapat dibuat langsung dari kamus kolom:

```eve
impor "matra" sebagai mt

variabel data = mt.tabel({
    "nama": ["Andi", "Budi", "Citra", "Dewi"],
    "usia": [17, 18, 17, 19],
    "nilai": [85.5, 92.0, 78.0, 95.5]
})
cetak(data)
```

### Membaca & Menyimpan Berkas CSV

Matra dilengkapi parser CSV berkecepatan tinggi yang membaca langsung dari disk laptop lokal:

```eve
# Membaca berkas CSV
variabel df = mt.baca_csv("data_siswa.csv")

# Menyimpan kembali ke berkas CSV
df.simpan_csv("siswa_bersih.csv")
```

### Menginspeksi Data

Gunakan method inspeksi untuk memahami struktur tabel:

```eve
# 5 baris pertama
df.kepala(5)

# Ringkasan statistik kolom angka
df.ringkasan()

# Dimensi tabel [jumlah_baris, jumlah_kolom]
cetak("Bentuk tabel: ", df.bentuk)
```

### Memilih & Memanipulasi Kolom

Akses dan manipulasi kolom dilakukan dengan method terstruktur:

```eve
# Mengambil kolom tertentu
variabel usia_siswa = df.kolom("usia")

# Memilih subset kolom
variabel sub = df.pilih_kolom(["nama", "nilai"])

# Menambah kolom baru
df.tambah_kolom("status", ["Lulus", "Lulus", "Lulus", "Lulus"])
```

### Pembersihan Nilai Kosong (*Missing Values*)

Data mentah seringkali memiliki nilai kosong (`kosong`):

```eve
# Membuang baris yang memiliki nilai kosong
variabel bersih = df.buang_kosong()

# Mengisi nilai kosong dengan nilai pengganti
variabel terisi = df.isi_kosong("usia", 17)
```

### Agregasi & Pengelompokan (*Groupby*)

Mengelompokkan data berdasarkan kategori tertentu dan menghitung agregat:

```eve
variabel rekap = df.kelompokkan("jurusan").rata_rata()
cetak(rekap)
```

## Contoh lengkap

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

## Jalankan

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

## Latihan

1. Buat tabel berisi 4 teman sekelas Anda dengan kolom `nama`, `hobi`, dan `jam_tidur`.
2. Filter baris tabel untuk hanya menampilkan data dengan `jam_tidur >= 7`.
3. Hitung rata-rata `jam_tidur` seluruh siswa menggunakan method `.rata_rata()` pada kolom tersebut.
