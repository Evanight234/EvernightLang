---
judul: "Handling Errors"
grup: "Panduan"
urutan: 6
---

# Handling Errors

This page explains the `coba` / `tangkap` / `akhirnya` blocks, the `lempar` and
`pastikan` commands, the error message format, and the error codes that show up
most often.

## Error message format

```text
BAHAYA [KODE]: Baris X (Kolom Y) - Deskripsi masalah!
   X | baris kode sumber
     | ^
```

- **`BAHAYA`**: a fatal error; the program stops (exit code `1`).
- **`PERINGATAN`**: does not stop the program (yellow).
- In the terminal, the CLI adds a snippet of the source line with a `^` marker.
- Turn colors off with `--tanpa-warna` or the `NO_COLOR` environment variable.

## `coba` and `tangkap`

```eve
coba {
    variabel hasil = 100 / 0
} tangkap (pesan) {
    cetak("Tertangkap: ", pesan)
}
```

- The parentheses after `tangkap` are **required**: `tangkap (e) { ... }`.
- Variable `e` holds the **error message** (text), not an error object.
- The optional `akhirnya { ... }` block runs after `coba`/`tangkap` finishes.
- `coba` only catches errors in **the same scope**. Errors raised inside other
  functions are not caught by a `coba` block in the outer file: wrap `coba`
  inside that function.

## `lempar`: custom errors

```eve
coba {
    lempar("Data rusak!")
} tangkap (e) {
    cetak("Tertangkap: ", e)   # Tertangkap: Data rusak!
}
```

`lempar` can carry any expression; text is the most common.

## `pastikan`: assertions

```eve
pastikan(b != 0, "Pembagi tidak boleh nol")
```

- Parentheses are **required**, arguments comma-separated:
  `pastikan(kondisi, "pesan")`.
- When the condition is `salah` → `BAHAYA [ASSERT]: Pernyataan tidak benar!`.
- To catch it, wrap with `coba`/`tangkap` in the same scope.

## Most common error codes

| Code | Level | Trigger | Example message |
|------|-------|---------|-----------------|
| `SYNTAX` | BAHAYA | Syntax error (quotes/braces/blocks) | `Diharapkan pembuka blok '{'!` |
| `TYPE` | BAHAYA | Mismatched data types | `Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!` |
| `FUNGSI` | BAHAYA | Wrong argument count / not a function | `Fungsi 'kali' butuh 2 argumen, diberikan 1` |
| `DIVISION` | BAHAYA | Division by zero | `Pembagian dengan nol tidak diizinkan!` |
| `NaN` | BAHAYA | Number conversion failed / invalid result | `Gagal mengonversi teks 'bukan_angka' menjadi angka!` |
| `VARIABLE` | BAHAYA | Variable not declared / changing a constant | `Variabel 'x' belum dideklarasikan!` |
| `INDEX` | BAHAYA | List index out of range | `Indeks 5 di luar jangkauan daftar (panjang: 2)!` |
| `KEY` | BAHAYA | Dict key missing | `Kunci 'kunci_asing' tidak ditemukan dalam kamus!` |
| `ASSERT` | BAHAYA | `pastikan` failed | `Pernyataan tidak benar!` |
| `RUNTIME` | BAHAYA | `lempar` / runtime error | the message from `lempar` |
| `FILE` | BAHAYA | Missing file, import cycle, access denied | `Berkas 'modul_hilang.eve' tidak ditemukan!` |
| `MATH` / `JUMLAH` | BAHAYA | Invalid math domain | `Akar kuadrat tidak terdefinisi untuk angka negatif` |
| `WKVAR` / `WKREACH` / `WKFUNG` | PERINGATAN | Unused variable / dead code / empty function | `Variabel 'x' dideklarasikan tetapi tidak pernah digunakan!` |

The full list is in `ERROR.md`.

## Full program example

Save it as `kesalahan.eve`, then run **from this folder**:
`evernight kesalahan.eve`

```eve
# 06 - Menangani kesalahan: coba, tangkap, lempar, pastikan

# 'coba' menangkap kesalahan supaya program tetap jalan
coba {
    variabel hasil = 100 / 0
    cetak("Tidak akan tercetak: ", hasil)
} tangkap (pesan) {
    cetak("Tertangkap: ", pesan)
} akhirnya {
    cetak("Blok akhirnya tetap dijalankan")
}

# 'pastikan' memvalidasi syarat; jika salah -> BAHAYA [ASSERT]
coba {
    pastikan(10 > 20, "Sepuluh harus lebih besar dari dua puluh")
} tangkap (e) {
    cetak("Assertion ditangkap: ", e)
}

# 'lempar' memicu kesalahan dengan pesan sendiri.
# Blok coba/tangkap yang menangkapnya harus berada di berkas/scope yang sama.
fungsi setor(nilai) {
    coba {
        jika nilai <= 0 {
            lempar("Nilai setor harus positif!")
        }
        kembali "Setor " + teks(nilai) + " berhasil"
    } tangkap (e) {
        kembali "GAGAL: " + e
    }
}
cetak(setor(50000))
cetak(setor(-1))

# Kegagalan fungsi bawaan juga bisa ditangkap (pakai jalur './...')
coba {
    cetak(baca_file("./berkas_tidak_ada.txt"))
} tangkap (e) {
    cetak("Berkas hilang: ", e)
}
cetak("Program selesai dengan aman.")
```

Output:

```text
Tertangkap: Pembagian dengan nol
Blok akhirnya tetap dijalankan
Assertion ditangkap: Sepuluh harus lebih besar dari dua puluh
Setor 50000 berhasil
GAGAL: Nilai setor harus positif!
Berkas hilang: Gagal membaca berkas './berkas_tidak_ada.txt': The system cannot find the file specified. (os error 2)
Program selesai dengan aman.
```

## Common mistakes

- **`tangkap e { }` without parentheses** →
  `BAHAYA [SYNTAX]: Diharapkan '(' setelah kata kunci 'tangkap'!`.
- **`pastikan kondisi` without parentheses** →
  `BAHAYA [SYNTAX]: Diharapkan '(' setelah 'pastikan'!`.
- **`coba` without `tangkap`** →
  `BAHAYA [SYNTAX]: Diharapkan blok 'tangkap' setelah blok 'coba'!`.
- **Reading a file without a folder path** (`baca_file("data.txt")`) → an
  uncaught `BAHAYA [FILE]`. Write `./data.txt` and run from the program's folder.
- **Expecting a `coba` in another file to catch your function's error**: move
  `coba` inside that function.
