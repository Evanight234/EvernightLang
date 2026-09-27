---
judul: "Stage 7: Strings & Math"
grup: "Tutorial"
urutan: 7
---

# Stage 7: Strings & Math

## Goal

Manipulate text (cut, replace, join, format) and use the built-in math functions
without having to calculate everything yourself.

## Concepts

### String module

```eve
impor string
```

`impor` is optional: all the functions below are already available globally.

| Function | Example result |
|----------|----------------|
| `besar(s)` / `kecil(s)` | `"HALO"` / `"halo"` |
| `bersih(s)` | trims leading-trailing spaces |
| `potong(s, 0, 4)` | the first 4 characters |
| `pecah(s, ",")` | text → list |
| `gabung(daftar, "-")` | list → text |
| `ganti(s, lama, baru)` | replace occurrences of text |
| `mengandung(s, target)` | `benar` / `salah` |
| `ulang_teks(s, n)` | repeat `n` times |
| `format(pola, ...)` | `format("{0} + {1} = {2}", 2, 3, 5)` |
| `s.panjang` | number of characters |

Join two texts using `+` (both sides must be text; use `teks()` for numbers).

### Math module

```eve
impor matematika
```

| Function | Description |
|----------|-------------|
| `akar(x)` | square root, `akar(144)` → 12 |
| `pangkat(a, b)` | `pangkat(2, 10)` → 1024 |
| `mutlak(x)` | absolute value |
| `pembulatan(x)` | nearest integer |
| `bulat_bawah(x)` / `bulat_atas(x)` | floor / ceil |
| `min(a, b)` / `max(a, b)` | smallest / largest |
| `faktorial(x)` | factorial |
| `sin(x)` / `cos(x)` / `tan(x)` | trigonometry (radians) |
| `pi`, `e` | constants |

## Complete Example

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

## Run

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

Built-in examples: `evernight examples/string.eve` and `evernight examples/matematika.eve`.

## Exercises

1. Change the `format` to format your own name and age.
2. Create a function `celcius_ke_fahrenheit(c)` using `pangkat`/`*`/`+`, then print
   the result for `36.5`.
3. Print a 5-line star triangle using `ulang_teks` and the loops from Stage 4.
4. (Experiment) `acak_antara(1, 10)` produces a random number, perfect for dice.
