---
judul: "Impor antar Berkas dan Modul Pustaka"
grup: "Panduan"
urutan: 7
---

# Impor antar Berkas dan Modul Pustaka

Halaman ini menjelaskan bagaimana program `.eve` Anda memecah diri menjadi beberapa berkas, dan bagaimana mengimpor modul pustaka standar (`matematika`, `string`, `konsol`, dll.).

## Dua bentuk `impor`

```eve
# 1. Berkas .eve sendiri, dengan alias -> akses lewat alias
impor "helper" sebagai h
cetak(h.jumlah_dua(2, 3))

# 2. Modul pustaka standar, tanpa tanda petik -> fungsinya sudah global
impor matematika
cetak(akar(144))
```

| Bentuk | Ditulis dengan | Cara memakai |
|--------|----------------|--------------|
| Berkas Anda sendiri | `impor "berkas" sebagai h` | `h.nama_fungsi(...)` |
| Berkas Anda sendiri (tanpa alias) | `impor "berkas"` | `berkas.nama_fungsi(...)` — nama berkas jadi variabel kamus |
| Pustaka standar | `impor matematika` | langsung: `akar(144)`, `besar(...)` |

`sebagai` adalah kata kunci (padanan `as` di bahasa lain) dan didukung juga oleh Go (`var x T = impor "..."`).

## Aturan jalur berkas

- Jalur **relatif dihitung dari folder berkas program utama** (bukan dari folder tempat Anda menjalankan perintah).
- Ekstensi `.eve` **boleh ditulis maupun tidak**: `impor "helper"` dan `impor "helper.eve"` sama saja.
- Dukungan pemisah `/` dan `\` untuk sub-folder: `impor "lib/util"`.
- Jalur absolut atau jalur yang keluar dari folder program → `BAHAYA [FILE]: Akses ditolak: impor '...' di luar direktori program`.

## Apa yang ikut diekspor

Saat sebuah berkas diimpor, seluruh **variabel & fungsi tingkat atas** yang didefinisikannya menjadi isi satu kamus:

```eve
# helper.eve
fungsi jumlah_dua(a, b) { kembali a + b }
fungsi sapa(nama) { kembali "Halo dari modul, " + nama + "!" }
```

```eve
# berkas utama
impor "helper" sebagai h
cetak(h.jumlah_dua(2, 3))      # 5
cetak(h.sapa("Evernight"))     # Halo dari modul, Evernight!

impor "helper"                 # tanpa alias
cetak(helper.jumlah_dua(10, 20))   # 30
```

- Modul bersifat **lazy-load + cache**: diimpor sekali saja, nilai ekspornya disimpan.
- **Impor sirkular** (A → B → A) → `BAHAYA [FILE]: Siklus impor terdeteksi saat memuat '...'`.
- Impor berkas yang tidak ada → `BAHAYA [FILE]: Berkas '...' tidak ditemukan atau tidak dapat dibaca`.

## Modul pustaka standar

Tujuh modul bawaan: `konsol`, `string`, `matematika`, `daftar`, `kamus`, `sistem`, `utilitas`.

- Seluruh fungsinya **sudah global tanpa perlu diimpor** — baris `impor string` hanya penanda/keterbacaan.
- Karena semuanya global, **jangan menamai fungsi Anda seperti fungsi bawaan** (mis. `tambah`, `daftar`, `format`, `teks`) — nama bawaan menang saat kompilasi.
- Contoh nama yang aman untuk fungsi Anda: `jumlah_dua`, `sapa`, `proses_harga`.

## Contoh program lengkap

Buat dua berkas, simpan di folder yang sama, lalu jalankan dari folder itu: `evernight modul.eve`

```eve
# --- berkas: utils.eve ---
fungsi jumlah_dua(a, b) {
    kembali a + b
}

fungsi sapa(nama) {
    kembali "Halo dari modul, " + nama + "!"
}

tetap PENGGUNA = "tim evernight"
```

```eve
# --- berkas: modul.eve (program utama) ---

# Impor berkas .eve lain DENGAN alias -> akses lewat alias
impor "utils" sebagai u
cetak(u.jumlah_dua(2, 3))
cetak(u.sapa("Evernight"))
cetak("Ekspor konstanta: ", u.PENGGUNA)

# Impor tanpa alias -> seluruh isi modul jadi satu kamus bernama isi berkas
impor "utils"
cetak(utils.jumlah_dua(10, 20))

# Modul pustaka standar ditulis tanpa tanda petik.
# Seluruh fungsinya sudah global, jadi 'impor' di sini hanya penanda.
impor matematika
impor string
cetak("akar(144) = ", akar(144))
cetak("besar = ", besar("nusantara"))
cetak("2 * pi = ", 2 * pi)
```

Output:

```text
5
Halo dari modul, Evernight!
Ekspor konstanta: tim evernight
30
akar(144) = 12
besar = NUSANTARA
2 * pi = 6.283185307179586
```

## Kesalahan umum

- **`import` / `use` / `require`** — bahasa ini memakai `impor`.
- **`impor "matematika"` dengan tanda petik** — tetap bekerja, tetapi gaya baku modul bawaan adalah tanpa petik: `impor matematika`.
- **`impor modul` sebagai identifier** (mis. `impor helper` tanpa petik) juga valid — hasilnya sama dengan `impor "helper"`.
- **Memanggil isi modul tanpa namespace** (`jumlah_dua(2,3)` tanpa `u.`/`utils.`) → `BAHAYA [VARIABLE]` — isi impor adalah satu kamus, bukan nama global terpisah.
- **Mengimpor dari folder luar program** → `BAHAYA [FILE] Akses ditolak`.
- **File impor di folder lain saat program utama ada di folder lain** → jalur dihitung dari folder program utama, bukan dari file pemanggil.
