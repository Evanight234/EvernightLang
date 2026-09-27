# ERROR.md - EvernightLanguage Error System

> Source of truth for the specification, categories, and message format of
> errors (error handling) in EvernightLanguage.
> Error messages are presented in **Bahasa Indonesia** with a standardized format.

---

## 1. Standard Message Format

Every error or warning message follows this pattern:

```
LEVEL [KODE]: Baris <nomor_baris> (Kolom <nomor_kolom>) - <Deskripsi pesan>
   <baris> | <cuplikan_kode>
           | <penunjuk_caret ^>
```

- **`LEVEL`**: 
  - `BAHAYA` - Fatal error (red `\x1b[1;31m`, the program stops immediately / exit code 1).
  - `PERINGATAN` - Non-fatal warning (yellow `\x1b[1;33m`, the program keeps running).
- **`[KODE]`**: The error category code, for easier tracking.
- **`Baris (Kolom)`**: The line and column position of the offending code.
- **Line Snippet**: On the CLI (`printer.rs`), the offending source line is shown
  with its line number (cyan `\x1b[36m`) and a caret `^` right at the problem
  column. The `--tanpa-warna` option or the `NO_COLOR` environment variable
  disables ANSI colors.

---

## 2. WARNING Category (Non-Fatal)

Warnings do not stop program execution; instead they notify the programmer
about suboptimal or potentially problematic code constructs.

| Code | Warning Name | When It Happens | Syntax Example | Example Message |
|------|--------------|-----------------|----------------|-----------------|
| `WKFUNG` | Empty Function | The function body block has no instructions | `fungsi hitung() { }` | `PERINGATAN [WKFUNG]: Baris 1 - Fungsi 'hitung' tidak memiliki isi!` |
| `WKCLASS` | Empty Class | The class body block has no methods/properties | `class Model { }` | `PERINGATAN [WKCLASS]: Baris 4 - Kelas 'Model' tidak memiliki isi!` |
| `WKVAR` | Unused Variable | A variable is declared but never read | `variabel x = 10` | `PERINGATAN [WKVAR]: Baris 2 - Variabel 'x' dideklarasikan tetapi tidak pernah digunakan!` |
| `WKREACH` | Unreachable Code | There are instructions after a `kembali` statement | `kembali a; cetak(a)` | `PERINGATAN [WKREACH]: Baris 6 - Kode setelah pernyataan 'kembali' tidak akan pernah dieksekusi!` |

---

## 3. BAHAYA Category (Fatal / Stops the Program)

Fatal errors stop runtime execution or fail the compilation/parsing stage.

### A. Parsing & Syntax Errors (`SYNTAX`)
Occur while reading the source code (Lexer & Parser).

| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `SYNTAX` | Unclosed string | `variabel s = "halo dunia` | `BAHAYA [SYNTAX]: Baris 1 - Penutup tanda petik tidak ditemukan!` |
| `SYNTAX` | Unbalanced parentheses | `cetak((10 + 5)` | `BAHAYA [SYNTAX]: Baris 3 - Tanda kurung penutup ')' tidak cocok atau kurang!` |
| `SYNTAX` | Unexpected token | `variabel = 20` | `BAHAYA [SYNTAX]: Baris 2 - Diharapkan nama variabel sebelum '='!` |
| `SYNTAX` | Wrong indentation / block | `jika benar` (without `{`) | `BAHAYA [SYNTAX]: Baris 5 - Diharapkan pembuka blok '{'!` |

### B. Type Errors (`TYPE`)
Occur while evaluating an expression with mismatched data types.

| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `TYPE` | Mismatched type operation | `10 + "5"` | `BAHAYA [TYPE]: Baris 4 - Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!` |
| `TYPE` | Property access on the wrong type / `kosong` | `variabel x = kosong; x.nama` | `BAHAYA [TYPE]: Baris 11 - Tidak dapat membaca properti dari nilai 'kosong'!` |

### B.5. Function Call Errors (`FUNGSI`)
Occur when calling functions (wrong arity, not a function, callback).

| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `FUNGSI` | Not a function | `variabel x = 10; x()` | `BAHAYA [FUNGSI]: Baris 3 - '10' bukan fungsi` |
| `FUNGSI` | Wrong argument count | `fungsi kali(a,b) {}; kali(5)` | `BAHAYA [FUNGSI]: Baris 5 - Fungsi 'kali' butuh 2 argumen, diberikan 1` |
| `FUNGSI` | Callback arity mismatch | `peta([1], fungsi(a,b){})` | `BAHAYA [FUNGSI]: Baris 1 - Fungsi callback harus menerima 1 argumen` |

### C. Not-a-Number Errors (`NaN`)
Invalid mathematical conversion operations immediately produce a fatal error
(strict mode).

| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `NaN` | Conversion to number failed | `angka("bukan_angka")` | `BAHAYA [NaN]: Baris 2 - Gagal mengonversi teks 'bukan_angka' menjadi angka!` |
| `NaN` | Undefined mathematical operation | `0 / 0` | `BAHAYA [NaN]: Baris 5 - Hasil operasi matematika tidak valid (Bukan Angka)!` |

### D. Division Errors (`DIVISION`)
| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `DIVISION` | Division by zero | `100 / 0` | `BAHAYA [DIVISION]: Baris 3 - Pembagian dengan nol tidak diizinkan!` |

### E. Collection Index & Key Errors (`INDEX` & `KEY`)
| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `INDEX` | Index out of range | `[1, 2][5]` | `BAHAYA [INDEX]: Baris 6 - Indeks 5 di luar jangkauan daftar (panjang: 2)!` |
| `KEY` | Key missing from dict | `kamus["kunci_asing"]` | `BAHAYA [KEY]: Baris 8 - Kunci 'kunci_asing' tidak ditemukan dalam kamus!` |

### F. Variable Scope Errors (`VARIABLE`)
| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `VARIABLE` | Undefined variable | `cetak(nilai_rahasia)` | `BAHAYA [VARIABLE]: Baris 4 - Variabel 'nilai_rahasia' belum dideklarasikan!` |
| `VARIABLE` | Assignment to a `tetap` variable | `tetap PI = 3.14; PI = 3.15` | `BAHAYA [VARIABLE]: Baris 2 - Tidak dapat mengubah nilai konstanta 'tetap' PI!` |

### G. Runtime & System Errors (`RUNTIME`, `MATH`, `JUMLAH`, `ASSERT`, `FILE`, `ENV`, `WAKTU`, `STACK`, `MEMORY`)
| Code | Error Case | Example Trigger | Example Message |
|------|------------|-----------------|-----------------|
| `RUNTIME` | Program passed the bytecode boundary | Internal VM error | `BAHAYA [RUNTIME]: Baris 0 - Program melewati batas bytecode` |
| `RUNTIME` | Custom thrown error | `lempar "gagal!"` | `BAHAYA [RUNTIME]: Baris 1 - gagal!` |
| `RUNTIME` | Unknown builtin | Internal error | `BAHAYA [RUNTIME]: Baris 0 - Builtin tak dikenal` |
| `MATH` | Negative square root | `akar(-4)` | `BAHAYA [MATH]: Baris 1 - Akar kuadrat tidak terdefinisi untuk angka negatif` |
| `MATH` | Undefined logarithm | `log(0)` | `BAHAYA [MATH]: Baris 1 - Logaritma tidak terdefinisi untuk angka nol atau negatif` |
| `JUMLAH` | Factorial of negative number/fraction | `faktorial(-1)` | `BAHAYA [JUMLAH]: Baris 1 - Faktorial hanya untuk bilangan bulat tak-negatif` |
| `JUMLAH` | Factorial exceeds limit | `faktorial(171)` | `BAHAYA [JUMLAH]: Baris 1 - Faktorial melebihi batas angka (maksimal 170)` |
| `JUMLAH` | Average of empty list | `rata_rata([])` | `BAHAYA [JUMLAH]: Baris 1 - rata_rata dari daftar kosong tidak terdefinisi` |
| `ASSERT` | Failed `pastikan` statement | `pastikan 1 == 2` | `BAHAYA [ASSERT]: Baris 1 - Pernyataan tidak benar!` |
| `FILE` | File not found | `impor modul_hilang` | `BAHAYA [FILE]: Baris 1 - Berkas 'modul_hilang.eve' tidak ditemukan!` |
| `FILE` | Import cycle detected | `a.eve` imports `b.eve`, `b.eve` imports `a.eve` | `BAHAYA [FILE]: Baris 1 - Siklus impor terdeteksi saat memuat 'a'!` |
| `FILE` | Access permission denied | Reading a file without permission | `BAHAYA [FILE]: Baris 3 - Akses ke berkas ditolak (Permission Denied)!` |
| `FILE` | File read/write failed | `baca_file("data.txt")` (missing) | `BAHAYA [FILE]: Baris 5 - Gagal membaca berkas 'data.txt': No such file or directory (os error 2)!` |
| `ENV` | Environment variable missing | `env("DB_HOST")` without a default | `BAHAYA [ENV]: Baris 2 - Variabel lingkungan 'DB_HOST' tidak ditemukan!` |
| `WAKTU` | Time function received an invalid value | `tunda(-1)` | `BAHAYA [WAKTU]: Baris 4 - tunda tidak menerima nilai negatif!` |
| `STACK` | Recursion depth over the limit *(planned - not yet raised)* | Infinite recursion | `BAHAYA [STACK]: Batas kedalaman rekursi tercapai (Stack Overflow)!` |
| `MEMORY` | Memory allocation failed *(planned - not yet raised)* | List allocation over the limit | `BAHAYA [MEMORY]: Kehabisan memori (Out of Memory)!` |

---

## 4. Finalization Status

- [x] Message format standard (`BAHAYA [KODE]` and `PERINGATAN [KODE]`)
- [x] Warning categories (empty function, empty class, idle variable)
- [x] Fatal runtime & parsing error categories
- [x] Handling of `NaN` as an immediate fatal error agreed on
- [x] Error codes synced with stdlib usage: `FUNGSI` (function calls), `MATH` (sqrt/log domain), `JUMLAH` (factorial/average), `RUNTIME` (throw/bytecode), `ASSERT` (assert) - 2026-09-10
- [ ] `STACK`/`MEMORY` will be raised when recursion & allocation limits are implemented
