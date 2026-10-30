---
judul: "Tahap 2: Variabel & Tipe Data"
grup: "Tutorial"
urutan: 2
---

# Tahap 2: Variabel & Tipe Data

## Tujuan

Menyimpan data di variabel, mengenali tipe data dasar, dan mengubah tipe secara sadar
dengan fungsi konversi.

## Konsep

### Mendeklarasikan data

```eve
variabel skor = 90          # boleh diubah
tetap NEGARA = "Indonesia"   # konstanta, tidak boleh diubah
```

### Tipe data utama

| Tipe | Contoh literal |
|------|----------------|
| `angka` | `42`, `3.14`, `-10` |
| `teks` | `"Halo"`, `'Halo'` |
| `bolean` | `benar`, `salah` |
| `kosong` | `kosong` |

### `cetak` bisa menerima banyak nilai

```eve
cetak("Umur: ", 20, " tahun")
```

### Konversi tipe harus eksplisit

EvernightLanguage **tidak** mengonversi otomatis. `10 + "5"` akan gagal. Gunakan
`teks(nilai)` dan `angka(nilai)`:

```eve
cetak("Tahun: " + teks(1945))   # teks(1945) -> "1945"
cetak(angka("42") + 8)          # 50
```

### Panjang teks

Properti `.panjang` menghitung jumlah karakter:

```eve
cetak("Indonesia".panjang)   # 9
```

## Contoh lengkap

```eve
impor konsol

variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Umurmu berapa? ")

tetap NEGARA = "Indonesia"
variabel aktif = benar

cetak("Halo ", nama, ", tahun depan umurmu ", umur + 1)
cetak("Negara: ", NEGARA, " (panjang teks: ", NEGARA.panjang, ")")
cetak("Status aktif: ", aktif)
cetak("Angka jadi teks: ", teks(1945))
cetak("Teks jadi angka: ", angka("42") + 8)
cetak("Umur berupa angka: ", adalah_angka(umur))
```

`baca(...)` membaca teks, `baca_angka(...)` membaca angka. Keduanya menampilkan
pertanyaan lebih dulu.

## Jalankan

Simpan sebagai `belajar.eve`, lalu jalankan dari folder yang sama:

```powershell
evernight belajar.eve
```

Saat ditanya, ketik `Budi` lalu tekan Enter, kemudian ketik `20`:

```
Siapa namamu? Umurmu berapa? Halo Budi, tahun depan umurmu 21
Negara: Indonesia (panjang teks: 9)
Status aktif: benar
Angka jadi teks: 1945
Teks jadi angka: 50
Umur berupa angka: benar
```

## Latihan

1. Tambahkan variabel `nilai` bertipe `bolean` lalu cetak nilainya.
2. Cetak jumlah karakter nama yang diketik pengguna (`nama.panjang`).
3. Coba jalankan `cetak("Umur: " + 20)` — apa yang terjadi? Mengapa, dan bagaimana
   cara memperbaikinya tanpa menghapus `+`?
