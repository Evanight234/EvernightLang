# TIPE.md - Data Type System & Design EvernightLanguage

> Source of truth for the EvernightLanguage data type system and specification.
> The language uses a **dynamic type** system with **ARC** memory management.

---

## 1. Design Decision Summary

| Aspect | Decision |
|--------|----------|
| **Typing System** | Dynamic (runtime type checking) |
| **Number Model** | One unified `angka` type (covers integers & decimals/floats) |
| **Coercion** | **Strict**: no automatic conversion between different types (raises a `BAHAYA` error) |
| **Text Length Access** | `.panjang` property (example: `teks.panjang`) |
| **List Length Access** | Built-in `panjang()` function (example: `panjang(daftar)`) |
| **Memory Management** | ARC (Automatic Reference Counting) |

---

## 2. Built-in Data Types

| Data Type | Type Name | Literal Format | Syntax Example |
|-----------|-----------|----------------|----------------|
| **Number** | `angka` | `42`, `-10`, `3.14`, `.5` | `variabel x = 42` |
| **Text** | `teks` | `"..."` or `'...'` | `variabel pesan = "Halo Dunia"` |
| **Boolean** | `bolean` | `benar` or `salah` | `variabel aktif = benar` |
| **Empty** | `kosong` | `kosong` | `variabel nilai = kosong` |
| **List** | `daftar` | `[item1, item2, ...]` | `variabel list = [1, "dua", benar]` |
| **Dict** | `kamus` | `{kunci: nilai, ...}` | `variabel data = {"nama": "Budi", "umur": 20}` |
| **Function** | `fungsi` | `fungsi(param) { ... }` | `variabel f = fungsi(x) { kembali x * 2 }` |
| **Object / Class** | `objek` | Based on class instantiation | `variabel mhs = Mahasiswa("Andi")` |

---

## 3. Type Characteristics in Detail

### A. Number (`angka`)
- Combines integer (64-bit) and fractional (64-bit float) representations.
- Supported operators:
  - Addition: `+`
  - Subtraction: `-`
  - Multiplication: `*`
  - Division: `/` (dividing by `0` produces `BAHAYA: Pembagian dengan nol!`)
  - Remainder (Modulo): `%`
  - Power: `**`
- Comparison operations: `==`, `!=`, `<`, `<=`, `>`, `>=`

### B. Text (`teks`)
- A UTF-8 character sequence wrapped in double quotes (`"..."`) or single
  quotes (`'...'`).
- Supported escape sequences: `\n` (newline), `\t` (tab), `\"`, `\'`, `\\`.
- Text concatenation: `teks1 + teks2` (only between `teks` values).
- Length access: `.panjang` property (example: `"Halo".panjang` is `4`).
- Substring check: `dalam` operator (example: `"al" dalam "Halo"` is `benar`).

### C. Boolean (`bolean`)
- Literal value `benar` (true) or `salah` (false).
- Logical operators:
  - `dan` (AND)
  - `atau` (OR)
  - `bukan` (NOT)

### D. Empty (`kosong`)
- Marks the absence of a value (null/nil/None).
- Single literal value: `kosong`.

### E. List (`daftar`)
- An ordered, dynamic data collection (flexible length, mixed element types).
- Refers fully to the `ARRAY.md` specification.
- Length access: the `panjang(daftar)` function.
- Built-in methods: `.tambah()`, `.sisip()`, `.hapus()`, `.ambil()`, `.bersihkan()`.

### F. Dict (`kamus`)
- A key-value pair collection (key-value store / map).
- Keys may be `teks`, `angka`, or `bolean`.
- Element access: `kamus[kunci]` or `kamus.kunci` (when the key is a valid text
  identifier).
- Key check: `kunci dalam kamus`.

---

## 4. Type Conversion Rules (Type Casting)

### No Implicit Coercion
EvernightLanguage rejects implicit conversion between different types to prevent
hidden bugs:
```eve
variabel hasil = 10 + "5"
# BAHAYA: Operasi '+' tidak didukung antara tipe 'angka' dan 'teks'!
```

### Explicit Conversion Functions
Conversion must be done deliberately with built-in functions:

| Conversion Function | Target Type | Example Input | Result |
|---------------------|-------------|----------------|--------|
| `angka(val)` | `angka` | `"123"`, `"3.14"`, `benar` | `123`, `3.14`, `1` |
| `teks(val)` | `teks` | `42`, `benar`, `[1, 2]` | `"42"`, `"benar"`, `"[1, 2]"` |
| `bolean(val)` | `bolean` | `0`, `""`, `[]`, `kosong` | `salah` (falsy) |
| `bolean(val)` | `bolean` | `1`, `"teks"`, `[1]` | `benar` (truthy) |
| `daftar(val)` | `daftar` | `"Halo"` | `["H", "a", "l", "o"]` |

---

## 5. Finalization Status

- [x] Primitive & composite types defined
- [x] Per-type operators documented
- [x] (Explicit) type conversion rules agreed on
- [x] Semantics synced with `ARRAY.md` and `KONSEP.md`
