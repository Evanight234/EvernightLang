# KEYWORD.md - List of Keywords (Keywords) EvernightLanguage

> Source of truth for all official keywords in EvernightLanguage.
> The keywords below are reserved words and must not be used as identifiers (variable/function names).

---

## Keyword Rules

- Keywords are written **all lowercase** (case-sensitive).
- They cannot be used as identifiers for variables, functions, or classes.
- Keywords are grouped into **Phase 1 (Core / Active)** and **Phase 2 (Future Reserve)**.

---

## 1. Phase 1 Keywords (Core & Active)

### A. Types & Literals
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `benar` | `true` | Positive boolean truth value | `variabel status = benar` |
| `salah` | `false` | Negative boolean truth value | `variabel aktif = salah` |
| `kosong` | `null` / `nil` | Marks the absence of a value | `variabel data = kosong` |
| `angka` | `number` | Numeric type (integers & decimals) | `angka("123")` |
| `teks` | `string` | UTF-8 character string | `teks(42)` |
| `bolean` | `boolean` | Boolean truth-value type | `bolean(1)` |

### B. Variable & Constant Declaration
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `variabel` | `let` / `var` | Declares a variable whose value can change | `variabel skor = 100` |
| `tetap` | `const` | Declares an absolute constant | `tetap PI = 3.14159` |

### C. Branching
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `jika` | `if` | Evaluates the first condition block | `jika nilai > 80 { ... }` |
| `lainnya_jika` | `else if` | Evaluates a follow-up condition when the previous one was `salah` | `lainnya_jika nilai > 60 { ... }` |
| `lainnya` | `else` | Fallback block when every condition is `salah` | `lainnya { ... }` |
| `cocok` | `switch` / `match` | Value-matching structure against patterns/cases | `cocok opsi { ... }` |
| `kasus` | `case` | A specific case branch inside `cocok` | `kasus 1 { cetak("Satu") }` |
| `bawaan` | `default` | Default branch inside `cocok` | `bawaan { cetak("Lainnya") }` |
| `_` | `_` (wildcard) | Catch-all pattern (wildcard) inside `cocok` | `kasus _ { cetak("Lainnya") }` |

### D. Looping
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `selama` | `while` | Repeats a block while the condition is `benar` | `selama x < 10 { x = x + 1 }` |
| `untuk` | `for` | Repeats a number range or iterates elements | `untuk item dalam daftar { ... }` |
| `dalam` | `in` | Collection membership operator / `untuk` iteration | `untuk i dari 0 sampai 5` / `x dalam list` |
| `berhenti` | `break` | Stops and exits the loop forcibly | `jika i == 5 { berhenti }` |
| `lanjut` | `continue` | Skips the rest of the block and moves to the next iteration | `jika i == 2 { lanjut }` |

### E. Functions
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `fungsi` | `fn` / `func` | Declares a function or lambda | `fungsi tambah(a, b) { ... }` |
| `kembali` | `return` | Returns a value from inside a function | `kembali a + b` |

### F. Word-Based Logical & Comparison Operators
| Keyword | EN Equivalent | Equivalent Symbol | Syntax Example |
|---------|---------------|-------------------|----------------|
| `dan` | `and` | `&&` | `jika umur >= 18 dan punya_ktp { ... }` |
| `atau` | `or` | `\|\|` | `jika status == "admin" atau status == "root" { ... }` |
| `bukan` | `not` | `!` | `jika bukan aktif { ... }` |
| `sama_dengan` | `equal` | `==` | `jika x sama_dengan 10 { ... }` |
| `lebih_dari` | `greater than` | `>` | `jika skor lebih_dari 50 { ... }` |
| `kurang_dari` | `less than` | `<` | `jika suhu kurang_dari 0 { ... }` |

### G. Error Handling
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `coba` | `try` | Opens a block watching for potential errors | `coba { ... }` |
| `tangkap` | `catch` | Catches and isolates the error message | `tangkap(err) { ... }` |
| `akhirnya` | `finally` | Block that always runs at the end of `coba` | `akhirnya { tutup(f) }` |
| `lempar` | `throw` | Explicitly raises a fatal error | `lempar("Data rusak!")` |
| `pastikan` | `assert` | Validates an absolute condition as truth | `pastikan(x > 0, "x harus positif")` |

### H. Module & Import System
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `impor` | `import` | Loads an external library module / other `.eve` file | `impor matematika` |
| `dari` | `from` | Selectively imports specific items | `impor akar, pi dari "matematika"` |
| `sebagai` | `as` | Gives an alias name to an imported module/item | `impor matematika sebagai mtk` |

### I. Basic Input & Output (I/O)
| Keyword | EN Equivalent | Description & Role | Syntax Example |
|---------|---------------|--------------------|----------------|
| `cetak` | `print` | Prints output to the standard terminal | `cetak("Halo Dunia")` |
| `baca` | `read / input` | Reads a line of text input from the terminal | `variabel input = baca("Prompt: ")` |

---

## 2. Phase 2 Keywords (Reserved: OOP & Advanced)

These keywords are reserved for future object-oriented, asynchronous, and
advanced memory-management development:

| Group | Reserved Keywords |
|-------|-------------------|
| **OOP & Structure** | `objek`, `larik`, `ini`, `warisi`, `induk`, `baru`, `privat`, `publik`, `statis`, `abstrak`, `konstruktor`, `destruktor`, `antarmuka`, `implementasi`, `struktur`, `enum`, `operator` |
| **Asynchronous & Concurrency** | `async`, `tunggu`, `paralel`, `kunci`, `sinkron` |
| **Generators & Memory** | `hasil`, `panggil_ulang`, `lakukan`, `dengan`, `hapus`, `jenis`, `global`, `lokal`, `debug`, `peringatan` |

---

## 3. Finalization Status

- [x] All core Phase 1 keywords fully defined
- [x] Keyword names synced: `variabel`, `fungsi`, `kembali`, `impor`
- [x] Loop keywords `berhenti`, `lanjut`, `dalam` accommodated
- [x] Wildcard `_` added to `cocok` pattern matching
- [x] Error keywords `coba`, `tangkap`, `akhirnya`, `lempar`, `pastikan` agreed on
- [x] Future keyword reservation (Phase 2) documented
