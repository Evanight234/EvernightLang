# FUNCTION.md - Function & Execution Specification EvernightLanguage

> Source of truth for function declaration rules, execution, loop control, and
> the list of built-in functions in EvernightLanguage.

---

## A. Function Categories & Types

| Category | Description | Syntax Example |
|----------|-------------|----------------|
| **Built-in Functions** | Global functions always available without import (`cetak`, type conversion) | `cetak("Halo")` |
| **User-Defined Functions** | Defined with the `fungsi` keyword and an identification name | `fungsi tambah(a, b) { kembali a + b }` |
| **Anonymous Functions (Lambda)** | Nameless functions that can be stored in a variable or passed as an argument | `variabel kali_dua = fungsi(x) { kembali x * 2 }` |
| **Closure** | Anonymous or nested functions that capture and remember their lexical environment | `fungsi pembuat_tambah(n) { kembali fungsi(x) { kembali x + n } }` |
| **Higher-Order Functions (HOF)** | Functions that take another function as an argument or return a function | `peta(data, fungsi(x) { kembali x + 1 })` |

---

## B. Function Declaration Rules (Add Function)

| Rule | Status | Notes & Example |
|------|--------|-----------------|
| **Keyword** | Required | Uses `fungsi` at the start of the definition |
| **Function Name** | Required (except anonymous) | Written in lowercase / snake_case: `fungsi hitung_total(a, b)` |
| **Parameters** | Optional | Listed inside parentheses `()`, separated by commas `,` |
| **Default Parameters** | Supported | Default values go at the end of the parameter list: `fungsi sapa(nama, salam = "Halo")` |
| **Body Block** | Required | Wrapped in curly braces `{ ... }` |
| **Return (`kembali`)** | **Explicitly Required** | A function must call `kembali <nilai>` to produce a return value |
| **Nested Functions** | Supported | A function may be defined inside another function's body |
| **First-Class Citizenship** | Supported | Functions can be stored in variables, array elements, and passed as arguments |

### Complete Implementation Example:
```eve
# 1. Fungsi biasa dengan rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

# 2. Fungsi dengan parameter default
fungsi buat_pesan(nama, awalan = "Selamat pagi") {
    kembali awalan + ", " + nama + "!"
}

# 3. Fungsi closure
fungsi buat_penghitung(awal = 0) {
    variabel hitungan = awal
    kembali fungsi() {
        hitungan = hitungan + 1
        kembali hitungan
    }
}
```

---

## C. Loop Control (Loop & Iteration)

### C1. Basic Loop Keywords

| Keyword | Syntax Pattern | Description |
|---------|----------------|-------------|
| `selama` | `selama <kondisi> { ... }` | Repeats a code block while the condition is `benar` |
| `untuk` (Range) | `untuk i dari <awal> sampai <akhir> { ... }` | Repeats an inclusive number range |
| `untuk` (Iteration) | `untuk item dalam <daftar> { ... }` | Iterates over every element in a list collection |
| `berhenti` | `berhenti` | Forcefully stops and exits the loop block (`break`) |
| `lanjut` | `lanjut` | Skips the current iteration and moves to the next one (`continue`) |

Example:
```eve
# Loop range dengan henti/lanjut
untuk i dari 0 sampai 10 {
    jika i == 3 {
        lanjut
    }
    jika i == 8 {
        berhenti
    }
    cetak(i)
}
```

---

### C2. Functional Iteration (Module `daftar`)

| Function | Input | Callback | Output | Description |
|----------|-------|----------|--------|-------------|
| `peta(lst, fn)` | `daftar`, `fn(item)` | `fn(x) -> hasil` | new `daftar` | Transform elements (Map) |
| `saring(lst, fn)` | `daftar`, `fn(item)` | `fn(x) -> bolean` | filtered `daftar` | Filter elements by a predicate |
| `lipat(lst, awal, fn)` | `daftar`, `awal`, `fn(acc, item)` | `fn(acc, x) -> acc`| `nilai` | Accumulate a value (Reduce/Fold) |
| `setiap(lst, fn)` | `daftar`, `fn(item)` | `fn(x)` | `kosong` | Run a side effect per element |

---

## D. Error Control Flow

| Pattern | Notes & Example |
|---------|-----------------|
| `coba { ... } tangkap(err) { ... }` | Catch runtime errors and handle them without stopping the system |
| `lempar(pesan)` | Deliberately raise a fatal error |
| `pastikan(kondisi, pesan)` | Evaluate an assertion; raise a fatal error when the condition is `salah` |

---

## E. Finalization Status

- [x] Function categories fully defined (Built-in, User-defined, Anonymous, Closure, HOF)
- [x] Declaration rules finalized (required `kembali`, default parameters, nested)
- [x] Loops `selama`, `untuk`, `berhenti`, `lanjut` agreed on
- [x] Error flow structure (`coba`, `tangkap`, `lempar`, `pastikan`) integrated
