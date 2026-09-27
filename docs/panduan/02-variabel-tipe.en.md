---
judul: "Variables and Data Types"
grup: "Panduan"
urutan: 2
---

# Variables and Data Types

This page explains how to store data with `variabel` and `tetap`, the eight
EvernightLanguage data types, and how to convert between types.

## Declaring variables

```eve
variabel skor = 100
skor = skor + 25          # boleh diubah
```

- `variabel`: the value **can change** (mutable).
- The initial value may be omitted: `variabel data` (value `kosong`).
- Assigning to a name that was never declared (`x = 1`) also creates a new
  variable, but the recommended default style is still `variabel x = 1` so the
  code stays readable. Reading a name that was never assigned → `BAHAYA [VARIABLE]`.

## Constants

```eve
tetap NAMA_BAHASA = "EvernightLanguage"
```

- `tetap`: the value **must not change**.
- The initial value is **required**: `tanpa nilai` →
  `BAHAYA [SYNTAX]: Konstanta 'tetap x' wajib memiliki nilai awal!`.
- Changing a constant → `BAHAYA [VARIABLE]: Tidak dapat mengubah nilai konstanta ...`.

## The eight data types

| Type | Name | Literal | Example |
|------|------|---------|--------|
| Number | `angka` | `42`, `-10`, `3.14` | `variabel n = 3.14` |
| Text | `teks` | `"..."` or `'...'` | `variabel s = "halo"` |
| Boolean | `bolean` | `benar` / `salah` | `variabel ok = benar` |
| Empty | `kosong` | `kosong` | `variabel x = kosong` |
| List | `daftar` | `[1, 2, 3]` | `variabel d = [1, 2, 3]` |
| Dict | `kamus` | `{"kunci": nilai}` | `variabel k = {"kota": "Aceh"}` |
| Function | `fungsi` | `fungsi(x) { ... }` | `variabel f = fungsi(x) { kembali x * 2 }` |
| Object | `objek` | class instantiation | OOP model - still Phase 2 (reserved) |

Note: every number (integer or decimal) uses **a single `angka` type**.

## Length & index access

- Text: property `.panjang` → `"Halo".panjang` = `4`.
- List: property `.panjang` → `[1,2,3].panjang` = `3`.
- Dict: property `.panjang` = number of keys.
- List/text indices start at `0` and **support negative numbers**: `daftar[-1]`
  = the last element.

## List & text operators

```eve
cetak(gabung_larik([1, 2], [3]))  # gabung daftar -> [1, 2, 3]
cetak("Ever" + "night")           # sambung teks -> Evernight
```

> Lists cannot be added together directly with `+` (error `[TYPE]`); use
> `gabung_larik(a, b)`.

## Type conversion (must be explicit)

EvernightLanguage uses **strict coercion**: mixing different types is an error
right away.

```eve
cetak(10 + "5")
# BAHAYA [TYPE]: Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!
```

Conversions are done on purpose with built-in functions:

| Function | Result | Example |
|----------|--------|---------|
| `angka(x)` | `angka` | `angka("12.5")` → `12.5`, `angka(benar)` → `1` |
| `teks(x)` | `teks` | `teks(42)` → `"42"` |
| `bolean(x)` | `bolean` | `bolean(1)` → `benar`, `bolean(0)` → `salah` |
| `daftar(x)` | `daftar` | `daftar("eve")` → `["e", "v", "e"]` |
| `kamus(x)` | `kamus` | `kamus([["a", 1]])` → `{"a": 1}` |
| `salin(x)` | a copy | `salin(daftar)` - deep copy |

## Full program example

Save it as `tipe.eve`, then run: `evernight tipe.eve`

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

## Common mistakes

- **`10 + "5"`** → `BAHAYA [TYPE]`. Always convert first: `teks(10) + "5"` or
  `angka("5")`.
- **A constant without an initial value** → `BAHAYA [SYNTAX]`.
- **Changing a `tetap`** → `BAHAYA [VARIABLE]`.
- **Index out of range** → `BAHAYA [INDEX]` (a negative index still inside the
  range is safe; e.g. `[1,2][-5]` is still an error).
- **Dict key does not exist** → `BAHAYA [KEY]`; use `dapatkan(k, "kunci", bawaan)`
  for a fallback value.
