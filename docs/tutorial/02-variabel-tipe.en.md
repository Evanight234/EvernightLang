---
judul: "Stage 2: Variables & Data Types"
grup: "Tutorial"
urutan: 2
---

# Stage 2: Variables & Data Types

## Goal

Store data in variables, recognize the basic data types, and convert types on
purpose with conversion functions.

## Concepts

### Declaring data

```eve
variabel skor = 90          # boleh diubah
tetap NEGARA = "Indonesia"   # konstanta, tidak boleh diubah
```

### Main data types

| Type | Literal example |
|------|-----------------|
| `angka` | `42`, `3.14`, `-10` |
| `teks` | `"Halo"`, `'Halo'` |
| `bolean` | `benar`, `salah` |
| `kosong` | `kosong` |

### `cetak` can take many values

```eve
cetak("Umur: ", 20, " tahun")
```

### Type conversion must be explicit

EvernightLanguage does **not** convert automatically. `10 + "5"` will fail. Use
`teks(nilai)` and `angka(nilai)`:

```eve
cetak("Tahun: " + teks(1945))   # teks(1945) -> "1945"
cetak(angka("42") + 8)          # 50
```

### Text length

The `.panjang` property counts the number of characters:

```eve
cetak("Indonesia".panjang)   # 9
```

## Complete Example

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

`baca(...)` reads text, `baca_angka(...)` reads a number. Both show the question
first.

## Run

Save it as `belajar.eve`, then run it from the same folder:

```powershell
evernight belajar.eve
```

When prompted, type `Budi` and press Enter, then type `20`:

```
Siapa namamu? Umurmu berapa? Halo Budi, tahun depan umurmu 21
Negara: Indonesia (panjang teks: 9)
Status aktif: benar
Angka jadi teks: 1945
Teks jadi angka: 50
Umur berupa angka: benar
```

## Exercises

1. Add a `bolean`-typed `nilai` variable, then print its value.
2. Print the number of characters in the name the user typed (`nama.panjang`).
3. Try running `cetak("Umur: " + 20)`. What happens? Why, and how do you fix it
   without removing `+`?
