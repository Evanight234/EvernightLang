# STDLIB.md - Standard Library EvernightLanguage

> Source of truth for the design and specification of the EvernightLanguage
> standard library modules.
> Standard modules are imported explicitly using the keyword `impor <nama_modul>`.

---

## 1. Usage & Design Rules

| Aspect | Decision |
|--------|----------|
| **Import System** | Explicit via `impor <nama_modul>` (example: `impor matematika`); for other `.eve` files: `impor "path/modul" [sebagai alias]` |
| **Built-in Functions** | `cetak(...)`, `baca(...)`, `baca_angka(...)`, `bersihkan()` and the basic type conversion functions (`angka()`, `teks()`, `bolean()`, `daftar()`, `kamus()`) are available globally without import |
| **Naming Convention** | Indonesian (lowercase, underscore `_` separator) |
| **File Control** | Manual handles (`buka`, `tutup`) and convenience functions (`baca_file`, `tulis_file`) |

> **Cross-file import (2026-09-10):** `impor "nama_modul" [sebagai alias]` loads
> other `.eve` files relative to the main file's directory (the `.eve` extension
> is added automatically). A module runs **once** (lazy-load + cache), executes
> in its own scope, and all of its top-level `fungsi`/`variabel` are exposed as
> a **dict** (`Value::Kamus`). Access exports through the namespace:
> `impor "helper" sebagai h` → call `h.tambah(2, 3)`; without an alias the
> global name = the file's base name (`impor "sub/helper"` → `helper`). Import
> cycles (A→B→A) and missing files → `BAHAYA [FILE]`. Standard modules
> (`konsol`, `string`, etc.) remain no-op. The `impor X dari ...` form is **not
> yet supported** (deferred to v2).


---

## 2. Complete Standard Module List

### A. Console Module (`impor konsol`)
Handles user input interaction through the terminal/console interface.
> **Note (2026-09-10):** console module functions are exposed as **global
> builtins**: `baca`, `baca_angka`, `bersihkan` can be used **without**
> `impor`. `impor konsol` is accepted by the compiler as a no-op (optional,
> no warning) so anyone who already wrote it keeps running.

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `baca(pesan?)` | `teks?` | `teks` | Shows a prompt then reads a line of input from the user | `variabel nama = baca("Nama: ")` |
| `baca_angka(pesan?)` | `teks?` | `angka` | Reads input and converts it to a number right away | `variabel umur = baca_angka("Umur: ")` |
| `bersihkan()` | *no parameters* | `kosong` | Clears the terminal screen | `bersihkan()` |

---

### B. String Module (`impor string`)
Text manipulation and inspection functions.

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `besar(s)` | `teks` | `teks` | Converts text to uppercase | `besar("halo")` → `"HALO"` |
| `kecil(s)` | `teks` | `teks` | Converts text to lowercase | `kecil("DUNIA")` → `"dunia"` |
| `bersih(s)` | `teks` | `teks` | Trims leading and trailing whitespace | `bersih("  teks  ")` → `"teks"` |
| `potong(s, awal, akhir?)` | `teks`, `angka`, `angka?` | `teks` | Takes a text slice by index range | `potong("Nusantara", 0, 4)` → `"Nusa"` |
| `pecah(s, pemisah)` | `teks`, `teks` | `daftar` | Splits text into a list by a separator | `pecah("a,b,c", ",")` → `["a", "b", "c"]` |
| `gabung(lst, pemisah)` | `daftar`, `teks` | `teks` | Joins a list of text into one string | `gabung(["a", "b"], "-")` → `"a-b"` |
| `ganti(s, lama, baru)` | `teks`, `teks`, `teks` | `teks` | Replaces old text occurrences with new text | `ganti("Budi", "B", "R")` → `"Rudi"` |
| `mengandung(s, target)` | `teks`, `teks` | `bolean` | Checks whether text contains a substring | `mengandung("Evernight", "night")` → `benar` |
| `mulai_dengan(s, awalan)`| `teks`, `teks` | `bolean` | Checks whether text starts with a prefix | `mulai_dengan("index.eve", "index")` → `benar` |
| `akhir_dengan(s, akhiran)`| `teks`, `teks` | `bolean` | Checks whether text ends with a suffix | `akhir_dengan("main.eve", ".eve")` → `benar` |
| `ulang_teks(s, n)` | `teks`, `angka` | `teks` | Repeats a string `n` times | `ulang_teks("ha", 3)` → `"hahaha"` |
| `format(pola, args...)` | `teks`, `variadic` | `teks` | Formats a string with variable interpolation | `format("Halo {0}, umur {1}", "Budi", 20)` |

---

### C. List Module (`impor daftar`)
Operations and manipulation for array/list collections.

> **Implementation (2026-09-10):** global functions without import
> (`impor daftar` allowed, no-op). `tambah/sisip/hapus`(functions) mutate the
> original list in-place and return `kosong`; `urutkan/balik/unik/gabung_larik/iris`
> return a new list (the original is unchanged). `urutkan` only works on
> **homogeneous** lists of numbers or text: mixed → `BAHAYA [TYPE]`.
> `jumlah`/`rata_rata` require number elements → `[TYPE]`; `rata_rata([])` →
> `[JUMLAH]`. `sisip` index out-of-range → `[INDEX]`; `iris` clamps
> automatically like `potong`. `cari` → first index or `-1`; `ada` →
> `benar/salah`. Higher-order `peta/saring/lipat/setiap` need a 1-argument
> callback (`lipat` callback takes 2 arguments: accumulator + element: that
> ORDER, not element first), wrong arity → `[FUNGSI]`; `lipat([], awal)` →
> `awal`; `setiap` → `kosong`; errors inside a callback propagate to the
> caller's `coba/tangkap`. Method syntax `a.tambah(x)` is **not yet
> supported**: use the global functions (`ARRAY.md`). The statement
> `hapus a[i]` removes an element (negative wrap, out-of-range `[INDEX]`);
> see the `hapus` keyword.

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `tambah(lst, item)` | `daftar`, `apapun` | `kosong` | Appends an element to the end of the list | `tambah(a, 10)` |
| `sisip(lst, idx, item)` | `daftar`, `angka`, `apapun`| `kosong` | Inserts an element at a given index | `sisip(a, 0, "awal")` |
| `hapus(lst, item)` | `daftar`, `apapun` | `kosong` | Removes the first matching element | `hapus(a, "awal")` |
| `urutkan(lst)` | `daftar` | `daftar` | Sorts the list elements | `urutkan([3, 1, 2])` → `[1, 2, 3]` |
| `balik(lst)` | `daftar` | `daftar` | Reverses the element order | `balik([1, 2, 3])` → `[3, 2, 1]` |
| `unik(lst)` | `daftar` | `daftar` | Keeps unique elements (no duplicates) | `unik([1, 2, 2, 3])` → `[1, 2, 3]` |
| `jumlah(lst)` | `daftar` | `angka` | Sums all number elements | `jumlah([10, 20, 30])` → `60` |
| `rata_rata(lst)` | `daftar` | `angka` | Computes the average of the elements | `rata_rata([10, 20, 30])` → `20` |
| `gabung_larik(a, b)` | `daftar`, `daftar` | `daftar` | Joins two lists into one | `gabung_larik([1], [2])` → `[1, 2]` |
| `iris(lst, awal, akhir)` | `daftar`, `angka`, `angka` | `daftar` | Takes a sublist (slicing) | `iris([10, 20, 30, 40], 1, 3)` → `[20, 30]` |
| `cari(lst, target)` | `daftar`, `apapun` | `angka` | Finds the first index of an element (`-1` if none) | `cari([10, 20], 20)` → `1` |
| `ada(lst, target)` | `daftar`, `apapun` | `bolean` | Checks whether an element exists in the list | `ada([1, 2], 2)` → `benar` |
| `peta(lst, fn)` | `daftar`, `fungsi` | `daftar` | Applies a transformation function per element (`map`) | `peta([1, 2], fungsi(x) { kembali x * 2 })` → `[2, 4]` |
| `saring(lst, fn)` | `daftar`, `fungsi` | `daftar` | Filters elements by a predicate (`filter`) | `saring([1, 6], fungsi(x) { kembali x > 5 })` → `[6]` |
| `lipat(lst, awal, fn)` | `daftar`, `apapun`, `fungsi` | `apapun` | Accumulates the whole list (`reduce`) | `lipat([1, 2], 0, fungsi(acc, x) { kembali acc + x })` → `3` |
| `setiap(lst, fn)` | `daftar`, `fungsi` | `kosong` | Runs an iteration per element (`forEach`) | `setiap(data, fungsi(x) { cetak(x) })` |

---

### D. Math Module
> **Implementation (2026-09-10):** global functions without import
> (`impor matematika` allowed, no-op). `max` is used (not `maks`); both aliases
> `pembulatan`/`bundar` and `mutlak`/`abs` apply; constants `pi` & `e` are
> available directly. Domain errors: `akar(-4)`/`log(0)`/`log(-1)` →
> `BAHAYA [MATH]`; negative/`> 170` `faktorial` → `BAHAYA [JUMLAH]`.
> `acak_antara(min, max)` is inclusive, and swaps automatically when
> `min > max`.

| Item | Type | Description | Usage Example |
|------|------|-------------|-------------------|
| `akar(x)` | Function | Computes the square root | `akar(25)` → `5` |
| `pangkat(a, b)` | Function | Computes `a` to the power `b` | `pangkat(2, 3)` → `8` |
| `bulat_bawah(x)` | Function | Rounds a decimal down (floor) | `bulat_bawah(4.9)` → `4` |
| `bulat_atas(x)` | Function | Rounds a decimal up (ceil) | `bulat_atas(4.1)` → `5` |
| `pembulatan(x)` / `bundar(x)` | Function | Rounds to the nearest integer (round) | `pembulatan(4.5)` → `5` |
| `mutlak(x)` / `abs(x)` | Function | Computes the absolute value | `mutlak(-10)` → `10` |
| `acak_antara(min, max)` | Function | Produces an inclusive random integer between min and max | `acak_antara(1, 10)` |
| `log(x)` | Function | Computes the natural logarithm | `log(10)` |
| `faktorial(x)` | Function | Computes the mathematical factorial (integer ≤ 170) | `faktorial(5)` → `120` |
| `min(a, b)` | Function | Returns the smaller value | `min(10, 5)` → `5` |
| `max(a, b)` | Function | Returns the larger value | `max(10, 5)` → `10` |
| `sin(x)` / `cos(x)` / `tan(x)` | Function | Basic trigonometry functions (radians) | `sin(0)` → `0` |
| `pi` | Constant | Value of Pi ($\approx 3.141592653589793$) | `variabel k = 2 * pi * r` |
| `e` | Constant | Euler's number ($\approx 2.718281828459045$) | `variabel log_e = e` |

---

### E. Dict Module (`impor kamus`)
Utilities for processing associative key-value data structures.

> **Implementation (2026-09-10):** global functions without import
> (`impor kamus` allowed, no-op). `setel/hapus_kunci` mutate the dict in-place
> → `kosong`; `kunci/nilai/pasangan/gabung_objek` return new ones. Results of
> `kunci/nilai/pasangan` are **sorted alphabetically by key** (consistent, not
> random order). `dapatkan(k, kunci, bawaan?)`: argc 2 or 3; missing key
> without `bawaan` → `BAHAYA [KEY]`. `gabung_objek(a, b)`: for the same key,
> `b` wins. Non-text keys in comparisons are converted to text. The statements
> `hapus k["x"]` and `hapus k.nama` delete entries; a missing key → silently
> `kosong` (see the `hapus` keyword).

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `kunci(k)` | `kamus` | `daftar` | Gets the full list of keys | `kunci({a: 1, b: 2})` → `["a", "b"]` |
| `nilai(k)` | `kamus` | `daftar` | Gets the full list of values | `nilai({a: 1, b: 2})` → `[1, 2]` |
| `pasangan(k)` | `kamus` | `daftar` | Gets the list of `[kunci, nilai]` pairs | `pasangan({a: 1})` → `[["a", 1]]` |
| `ada_kunci(k, target)` | `kamus`, `apapun` | `bolean` | Checks whether a key exists in the dict | `ada_kunci(k, "nama")` → `benar` |
| `hapus_kunci(k, target)` | `kamus`, `apapun` | `kosong` | Removes a key entry from the dict | `hapus_kunci(k, "umur")` |
| `dapatkan(k, kunci, bawaan?)` | `kamus`, `apapun`, `apapun?` | `apapun` | Gets a value with a fallback | `dapatkan(k, "skor", 0)` |
| `setel(k, kunci, nilai)` | `kamus`, `apapun`, `apapun` | `kosong` | Updates or adds a key value | `setel(k, "skor", 100)` |
| `gabung_objek(a, b)` | `kamus`, `kamus` | `kamus` | Joins two dicts into a new dict | `gabung_objek(d1, d2)` |

---

### F. Type & Utility Module (`impor utilitas` / global)
Type checks, data reflection, and memory copying.

> **Implementation (2026-09-10):** global functions without import
> (`impor utilitas` allowed, no-op). `adalah_*` → `benar/salah` (missing
> argument → `salah`). `ke_boolean`/`e_boolean`/`bolean` → `is_truthy` (number
> ≠ 0, non-empty text, non-empty list/dict). `ke_larik`/`daftar` → text→list of
> characters, list→shallow copy, dict→alphabetically sorted key list; anything
> else → `BAHAYA [TYPE]`. `salin(x)` is a **deep copy** (recursive
> list/dict; cyclic structures are safe; functions are shared, not copied).
> Basic conversions: `angka()` (Number→identity, Boolean→1/0, Text→parse fails
> → `[NaN]`), `teks()` (all types → `to_string`), `bolean()` (≡ `ke_boolean`),
> `daftar()` (≡ `ke_larik`), `kamus()` (Dict→clone; list of `[kunci, nilai]`
> pairs → dict, wrong-size/wrong-type element → `[TYPE]`). Aliases
> `e_boolean`≡`bolean`, `daftar`≡`ke_larik` (STDLIB `bolean()`/`daftar()`/
> `kamus()` constructors merged).

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `adalah_angka(x)` | `apapun` | `bolean` | Checks whether the value is of type `angka` | `adalah_angka(42)` → `benar` |
| `adalah_teks(x)` | `apapun` | `bolean` | Checks whether the value is of type `teks` | `adalah_teks("a")` → `benar` |
| `adalah_daftar(x)`| `apapun` | `bolean` | Checks whether the value is of type `daftar` | `adalah_daftar([])` → `benar` |
| `adalah_kamus(x)` | `apapun` | `bolean` | Checks whether the value is of type `kamus` | `adalah_kamus({})` → `benar` |
| `ke_boolean(x)` / `e_boolean(x)` | `apapun` | `bolean` | Strictly converts a value to `bolean` | `e_boolean(1)` → `benar` |
| `ke_larik(x)` / `daftar(x)` | `apapun` | `daftar` | Converts text to a character list / list to a copy / dict to a key list | `ke_larik("abc")` → `["a", "b", "c"]` |
| `salin(x)` | `apapun` | `apapun` | Makes a deep copy of an object/list | `variabel klon = salin(objek_asli)` |
| `angka(x)` | `apapun` | `angka` | Basic conversion to number (text, boolean) | `angka("12.5")` → `12.5` |
| `teks(x)` | `apapun` | `teks` | Basic conversion to text | `teks(42)` → `"42"` |

---

### G. System & Time Module (`impor sistem`)
File, time, and environment variable operations.

> **Implementation (2026-09-10):** global functions without import
> (`impor sistem` allowed, no-op). `baca_file`/`tulis_file`/`ada_file` use
> instant operations (`std::fs`); IO failure → `BAHAYA [FILE]`.
> `env(nama, bawaan?)`: argc 1–2; no default and missing variable →
> `BAHAYA [ENV]`. `tanggal_sekarang(ts?)` & `format_tanggal(ts, pola)`: ts is
> a millisecond timestamp, displayed in **UTC**; `tanggal_sekarang()` with no
> argument = the current time. `format_tanggal` pattern: tokens
> `YYYY MM DD HH mm ss` (everything else is copied verbatim). `tunda(detik)`
> accepts decimals, negative → `BAHAYA [WAKTU]`. `selisih_waktu(t1, t2)` →
> absolute difference. New error codes: `[ENV]`, `[WAKTU]` (see `ERROR.md`).
> **`argumen()` was implemented in Phase 4A** (id 75: returns the `daftar` of
> program arguments given after `--` on the CLI). **File handles
> (`buka/tutup/baca_baris/tulis`) are DEFERRED**: not implemented; use the
> instant `baca_file`/`tulis_file` instead.

| Function | Parameters | Return Type | Description | Usage Example |
|----------|------------|-------------|-------------|-------------------|
| `argumen()` | *no parameters* | `daftar` | Returns the program's command-line argument list (after `--`) | `variabel args = argumen()` |
| `waktu_sekarang()` | *no parameters* | `angka` | Returns the epoch timestamp in milliseconds | `variabel t = waktu_sekarang()` |
| `tanggal_sekarang(ts?)` | `angka?` | `teks` | Current date/time in ISO (UTC); optional ts | `tanggal_sekarang()` / `tanggal_sekarang(0)` |
| `format_tanggal(ts, format)`| `angka`, `teks` | `teks` | Formats a timestamp by pattern (`YYYY MM DD HH mm ss`) | `format_tanggal(t, "YYYY-MM-DD")` |
| `tunda(detik)` | `angka` | `kosong` | Pauses execution for n seconds | `tunda(1.5)` |
| `selisih_waktu(t1, t2)` | `angka`, `angka` | `angka` | Computes the duration between two timestamps | `selisih_waktu(t2, t1)` |
| `baca_file(path)` | `teks` | `teks` | Reads a whole file instantly | `variabel isi = baca_file("data.txt")` |
| `tulis_file(path, teks)` | `teks`, `teks` | `kosong` | Writes/overwrites text to a file instantly | `tulis_file("data.txt", "Halo")` |
| `buka(path, mode)` | `teks`, `teks` | `objek` | DEFERRED: not yet implemented | `~` |
| `tutup(f)` | `objek` | `kosong` | DEFERRED: not yet implemented | `~` |
| `baca_baris(f)` | `objek` | `teks` | DEFERRED: not yet implemented | `~` |
| `tulis(f, teks)` | `objek`, `teks` | `kosong` | DEFERRED: not yet implemented | `~` |
| `ada_file(path)` | `teks` | `bolean` | Checks whether a file exists on the filesystem | `ada_file("config.eve")` → `benar` |
| `env(nama, bawaan?)` | `teks`, `apapun?` | `teks` | Gets an OS environment variable (default if missing) | `env("DB_HOST", "localhost")` |
| `atur_env(nama, nilai)` | `teks`, `teks` | `kosong` | Sets an OS environment variable | `atur_env("APP_ENV", "dev")` |

---

## 3. Finalization Status

- [x] 7 standard library module categories fully defined
- [x] Function names synced with language decisions (`besar`/`kecil`, `bersih`, `tambah`, `ada_kunci`)
- [x] Added time features, deep copy, type checks, and fast file utilities
