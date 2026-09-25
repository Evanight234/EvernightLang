---
judul: "Tahap 7: String & Matematika"
grup: "Tutorial"
urutan: 7
---

# Tahap 7: String & Matematika

## Tujuan

Memanipulasi teks (potong, ganti, gabung, format) dan memakai fungsi matematika
bawaan tanpa perlu menghitung sendiri.

## Konsep

### Modul string

```eve
impor string
```

`impor` bersifat opsional — semua fungsi di bawah ini sudah tersedia global.

| Fungsi | Contoh hasil |
|--------|--------------|
| `besar(s)` / `kecil(s)` | `"HALO"` / `"halo"` |
| `bersih(s)` | buang spasi awal–akhir |
| `potong(s, 0, 4)` | 4 karakter pertama |
| `pecah(s, ",")` | teks → daftar |
| `gabung(daftar, "-")` | daftar → teks |
| `ganti(s, lama, baru)` | ganti kemunculan teks |
| `mengandung(s, target)` | `benar` / `salah` |
| `ulang_teks(s, n)` | ulang `n` kali |
| `format(pola, ...)` | `format("{0} + {1} = {2}", 2, 3, 5)` |
| `s.panjang` | jumlah karakter |

Gabung dua teks memakai `+` (kedua sisi harus teks — pakai `teks()` untuk angka).

### Modul matematika

```eve
impor matematika
```

| Fungsi | Fungsi |
|--------|--------|
| `akar(x)` | akar kuadrat, `akar(144)` → 12 |
| `pangkat(a, b)` | `pangkat(2, 10)` → 1024 |
| `mutlak(x)` | nilai absolut |
| `pembulatan(x)` | bulat terdekat |
| `bulat_bawah(x)` / `bulat_atas(x)` | floor / ceil |
| `min(a, b)` / `max(a, b)` | terkecil / terbesar |
| `faktorial(x)` | faktorial |
| `sin(x)` / `cos(x)` / `tan(x)` | trigonometri (radian) |
| `pi`, `e` | konstanta |

## Contoh lengkap

```eve
impor string
impor matematika

variabel kalimat = "  Evernight Language  "
cetak("asli   : [", kalimat, "]")
cetak("bersih : ", bersih(kalimat))
cetak("besar  : ", besar(kalimat))
cetak("kecil  : ", kecil(kalimat))
cetak("potong : ", potong("Nusantara", 0, 4))

variabel buah = pecah("apel,mangga,jeruk", ",")
cetak("pecah  : ", buah)
cetak("gabung : ", gabung(buah, " - "))
cetak("ganti  : ", ganti("halo dunia", "dunia", "evernight"))
cetak("ada    : ", mengandung("Evernight", "night"))
cetak("ulang  : ", ulang_teks("= ", 8))
cetak("format : ", format("{0} + {1} = {2}", 2, 3, 5))
cetak("panjang: ", kalimat.panjang)

cetak("akar(144)         = ", akar(144))
cetak("pangkat(2, 10)    = ", pangkat(2, 10))
cetak("mutlak(-42)       = ", mutlak(-42))
cetak("pembulatan(4.5)   = ", pembulatan(4.5))
cetak("bulat_bawah(7.9)  = ", bulat_bawah(7.9))
cetak("bulat_atas(7.1)   = ", bulat_atas(7.1))
cetak("min(3, 9)         = ", min(3, 9))
cetak("max(3, 9)         = ", max(3, 9))
cetak("faktorial(6)      = ", faktorial(6))
cetak("2 * pi            = ", 2 * pi)
cetak("sin(pi / 2)       = ", sin(pi / 2))
```

## Jalankan

```powershell
evernight teks_angka.eve
```

```
asli   : [  Evernight Language  ]
bersih : Evernight Language
besar  :   EVERNIGHT LANGUAGE  
kecil  :   evernight language  
potong : Nusa
pecah  : [apel, mangga, jeruk]
gabung : apel - mangga - jeruk
ganti  : halo evernight
ada    : benar
ulang  : = = = = = = = = 
format : 2 + 3 = 5
panjang: 22
akar(144)         = 12
pangkat(2, 10)    = 1024
mutlak(-42)       = 42
pembulatan(4.5)   = 5
bulat_bawah(7.9)  = 7
bulat_atas(7.1)   = 8
min(3, 9)         = 3
max(3, 9)         = 9
faktorial(6)      = 720
2 * pi            = 6.283185307179586
sin(pi / 2)       = 1
```

Contoh bawaan: `evernight examples/string.eve` dan `evernight examples/matematika.eve`.

## Latihan

1. Ubah `format` menjadi format nama dan umur Anda sendiri.
2. Buat fungsi `celcius_ke_fahrenheit(c)` memakai `pangkat`/`*`/`+`, lalu cetak hasil
   untuk `36.5`.
3. Cetak segitiga bintang 5 baris memakai `ulang_teks` dan perulangan Tahap 4.
4. (Eksperimen) `acak_antara(1, 10)` menghasilkan angka acak — cocok untuk dadu.
