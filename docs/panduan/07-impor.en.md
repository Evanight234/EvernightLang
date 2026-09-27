---
judul: "Importing Between Files and Library Modules"
grup: "Panduan"
urutan: 7
---

# Importing Between Files and Library Modules

This page explains how your `.eve` program splits itself across multiple files,
and how to import standard library modules (`matematika`, `string`, `konsol`,
etc.).

## Two forms of `impor`

```eve
# 1. Berkas .eve sendiri, dengan alias -> akses lewat alias
impor "helper" sebagai h
cetak(h.jumlah_dua(2, 3))

# 2. Modul pustaka standar, tanpa tanda petik -> fungsinya sudah global
impor matematika
cetak(akar(144))
```

| Form | Written as | How to use |
|------|------------|------------|
| Your own file | `impor "berkas" sebagai h` | `h.nama_fungsi(...)` |
| Your own file (no alias) | `impor "berkas"` | `berkas.nama_fungsi(...)` : the file name becomes a dict variable |
| Standard library | `impor matematika` | direct: `akar(144)`, `besar(...)` |

`sebagai` is a keyword (the `as` equivalent in other languages) and is also
supported by Go (`var x T = impor "..."`).

## File path rules

- **Relative paths are resolved from the folder of the main program file** (not
  the folder where you run the command).
- The `.eve` extension **may be written or omitted**: `impor "helper"` and
  `impor "helper.eve"` are the same.
- `/` and `\` separators are supported for subfolders: `impor "lib/util"`.
- Absolute paths or paths that leave the program folder →
  `BAHAYA [FILE]: Akses ditolak: impor '...' di luar direktori program`.

## What gets exported

When a file is imported, every **top-level variable and function** it defines
becomes the contents of one dict:

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

- Modules are **lazy-loaded + cached**: imported only once, their exported
  values are stored.
- **Circular imports** (A → B → A) →
  `BAHAYA [FILE]: Siklus impor terdeteksi saat memuat '...'`.
- Importing a file that does not exist →
  `BAHAYA [FILE]: Berkas '...' tidak ditemukan atau tidak dapat dibaca`.

## Standard library modules

Seven built-in modules: `konsol`, `string`, `matematika`, `daftar`, `kamus`,
`sistem`, `utilitas`.

- All of their functions are **already global with no import needed**: an
  `impor string` line is only a marker/readability aid.
- Because everything is global, **do not name your functions like built-in
  functions** (e.g. `tambah`, `daftar`, `format`, `teks`): the built-in name
  wins at compile time.
- Safe names for your functions: `jumlah_dua`, `sapa`, `proses_harga`.

## Full program example

Create two files, put them in the same folder, then run from that folder:
`evernight modul.eve`

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

## Common mistakes

- **`import` / `use` / `require`**: this language uses `impor`.
- **`impor "matematika"` with quotes**: still works, but the default style for
  built-in modules is without quotes: `impor matematika`.
- **`impor modul` as an identifier** (e.g. `impor helper` without quotes) is
  also valid: the result matches `impor "helper"`.
- **Calling module contents without a namespace** (`jumlah_dua(2,3)` without
  `u.`/`utils.`) → `BAHAYA [VARIABLE]`: an import's contents are one dict, not
  separate global names.
- **Importing from outside the program folder** → `BAHAYA [FILE] Akses ditolak`.
- **An imported file lives in another folder while the main program is
  elsewhere**: the path is resolved from the main program's folder, not the
  calling file's folder.
