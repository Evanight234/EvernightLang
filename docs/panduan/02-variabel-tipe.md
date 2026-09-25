---
judul: "Variabel dan Tipe Data"
grup: "Panduan"
urutan: 2
---

# Variabel dan Tipe Data

Halaman ini menjelaskan cara menyimpan data dengan `variabel` dan `tetap`, delapan tipe data EvernightLanguage, serta cara mengonversi antar tipe.

## Mendeklarasikan variabel

```eve
variabel skor = 100
skor = skor + 25          # boleh diubah
```

- `variabel` — nilai **bisa diubah** (mutable).
- Nilai awal boleh dihilangkan: `variabel data` (bernilai `kosong`).
- Penugasan ke nama yang belum pernah dideklarasikan (`x = 1`) ikut membuat variabel baru, tetapi gaya baku yang disarankan tetap `variabel x = 1` agar kode mudah dibaca. Membaca nama yang belum pernah diisi → `BAHAYA [VARIABLE]`.

## Konstanta

```eve
tetap NAMA_BAHASA = "EvernightLanguage"
```

- `tetap` — nilai **tidak boleh diubah**.
- Nilai awal **wajib**: `tanpa nilai` → `BAHAYA [SYNTAX]: Konstanta 'tetap x' wajib memiliki nilai awal!`.
- Mengubah nilai konstanta → `BAHAYA [VARIABLE]: Tidak dapat mengubah nilai konstanta ...`.

## Delapan tipe data

| Tipe | Nama | Literal | Contoh |
|------|------|---------|--------|
| Angka | `angka` | `42`, `-10`, `3.14` | `variabel n = 3.14` |
| Teks | `teks` | `"..."` atau `'...'` | `variabel s = "halo"` |
| Bolean | `bolean` | `benar` / `salah` | `variabel ok = benar` |
| Kosong | `kosong` | `kosong` | `variabel x = kosong` |
| Daftar | `daftar` | `[1, 2, 3]` | `variabel d = [1, 2, 3]` |
| Kamus | `kamus` | `{"kunci": nilai}` | `variabel k = {"kota": "Aceh"}` |
| Fungsi | `fungsi` | `fungsi(x) { ... }` | `variabel f = fungsi(x) { kembali x * 2 }` |
| Objek | `objek` | instansiasi kelas | model OOP — masih Fase 2 (cadangan) |

Catatan: seluruh bilangan (bulat maupun desimal) memakai **satu tipe `angka`**.

## Akses panjang & indeks

- Teks: properti `.panjang` → `"Halo".panjang` = `4`.
- Daftar: properti `.panjang` → `[1,2,3].panjang` = `3`.
- Kamus: properti `.panjang` = jumlah kunci.
- Indeks daftar/teks dimulai dari `0` dan **mendukung angka negatif**: `daftar[-1]` = elemen terakhir.

## Operator daftar & teks

```eve
cetak(gabung_larik([1, 2], [3]))  # gabung daftar -> [1, 2, 3]
cetak("Ever" + "night")           # sambung teks -> Evernight
```

> Daftar tidak bisa langsung dijumlahkan dengan `+` (error `[TYPE]`); gunakan `gabung_larik(a, b)`.

## Konversi tipe (harus eksplisit)

EvernightLanguage memakai **coercion ketat**: campuran tipe berbeda langsung error.

```eve
cetak(10 + "5")
# BAHAYA [TYPE]: Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!
```

Konversi dilakukan sadar dengan fungsi bawaan:

| Fungsi | Hasil | Contoh |
|--------|-------|--------|
| `angka(x)` | `angka` | `angka("12.5")` → `12.5`, `angka(benar)` → `1` |
| `teks(x)` | `teks` | `teks(42)` → `"42"` |
| `bolean(x)` | `bolean` | `bolean(1)` → `benar`, `bolean(0)` → `salah` |
| `daftar(x)` | `daftar` | `daftar("eve")` → `["e", "v", "e"]` |
| `kamus(x)` | `kamus` | `kamus([["a", 1]])` → `{"a": 1}` |
| `salin(x)` | salinan | `salin(daftar)` — salinan mendalam (deep copy) |

## Contoh program lengkap

Simpan sebagai `tipe.eve`, lalu jalankan: `evernight tipe.eve`

```eve
# 02 - Variabel, konstanta, dan tipe data

# Deklarasi variabel (bisa diubah)
variabel skor = 100
skor = skor + 25
cetak("Skor: ", skor)

# Konstanta (wajib punya nilai awal, tidak boleh diubah)
tetap NAMA_BAHASA = "EvernightLanguage"
cetak("Nama bahasa: ", NAMA_BAHASA)

# Delapan tipe data
variabel angka_nilai = 3.14          # angka
variabel teks_nilai = "halo"         # teks
variabel bolean_nilai = benar        # bolean
variabel kosong_nilai = kosong       # kosong
variabel daftar_nilai = [1, 2, 3]    # daftar
variabel kamus_nilai = {"kota": "Aceh"}  # kamus
variabel fungsi_nilai = fungsi(x) { kembali x * 2 }  # fungsi

cetak("angka: ", angka_nilai, " teks: ", teks_nilai)
cetak("bolean: ", bolean_nilai, " kosong: ", kosong_nilai)
cetak("daftar: ", daftar_nilai, " panjang: ", daftar_nilai.panjang)
cetak("kamus: ", kamus_nilai["kota"], " panjang: ", kamus_nilai.panjang)
cetak("fungsi: ", fungsi_nilai(21))

# Indeks daftar mendukung angka negatif
variabel huruf = ["e", "v", "e", "n"]
cetak("Indeks terakhir: ", huruf[-1])

# Konversi tipe harus eksplisit
cetak("angka -> teks: ", teks(42))
cetak("teks -> angka: ", angka("12.5"))
cetak("angka -> bolean: ", bolean(1))
cetak("teks -> daftar: ", daftar("eve"))
```

Output:

```text
Skor: 125
Nama bahasa: EvernightLanguage
angka: 3.14 teks: halo
bolean: benar kosong: kosong
daftar: [1, 2, 3] panjang: 3
kamus: Aceh panjang: 1
fungsi: 42
Indeks terakhir: n
angka -> teks: 42
teks -> angka: 12.5
angka -> bolean: benar
teks -> daftar: [e, v, e]
```

## Kesalahan umum

- **`10 + "5"`** → `BAHAYA [TYPE]`. Selalu konversi dulu: `teks(10) + "5"` atau `angka("5")`.
- **Konstanta tanpa nilai awal** → `BAHAYA [SYNTAX]`.
- **Mengubah `tetap`** → `BAHAYA [VARIABLE]`.
- **Indeks di luar jangkauan** → `BAHAYA [INDEX]` (negatif yang masih dalam jangkauan aman, misal `[1,2][-5]` tetap error).
- **Kunci kamus tidak ada** → `BAHAYA [KEY]`; pakai `dapatkan(k, "kunci", bawaan)` untuk nilai cadangan.
