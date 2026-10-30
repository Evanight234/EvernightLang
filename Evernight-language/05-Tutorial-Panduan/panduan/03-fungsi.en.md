---
judul: "Functions, Parameters, and Return"
grup: "Panduan"
urutan: 3
---

# Functions, Parameters, and Return

This page explains how to declare functions with `fungsi`, pass values through
parameters, return values with `kembali`, and use functions as values
(lambda/closure).

## Function declaration

```eve
fungsi nama_fungsi(a, b) {
    kembali a + b
}
```

- The keyword `fungsi` (not `fn`/`def`).
- Function names should use `snake_case` (linter rule `WKHURUF`).
- The parameter list is comma-separated; with no parameters still write `()`.
- The body must use `{ ... }`.

## Returning a value

```eve
fungsi tambah(a, b) {
    kembali a + b
}
```

- `kembali` is the keyword for returning a value (not `return`).
- A function **without `kembali`** returns `kosong`.
- `kembali` without a value (in the last `{ }`) also produces `kosong`.
- Code after `kembali` never runs → warning `PERINGATAN [WKREACH]`.

## Parameters

```eve
fungsi sapa(nama, salam = "Halo") {
    kembali salam + ", " + nama + "!"
}
```

- Default parameter values **are supported by the syntax** (`salam = "Halo"`).
- **v0.1.0 note:** the call must still provide **all arguments**.
  `sapa("Budi")` raises `BAHAYA [FUNGSI]: Fungsi 'sapa' butuh 2 argumen, diberikan 1`.
  Write `sapa("Budi", "Halo")`.
- The argument count must match exactly (too many is also an error). See
  `BAHAYA [FUNGSI]`.

## Recursion

```eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
```

## Functions as values

Functions are ordinary values: they can be stored in a variable, put into a
list, and passed as arguments (*first-class functions*).

```eve
variabel kali_dua = fungsi(x) { kembali x * 2 }   # fungsi anonim / lambda
cetak(kali_dua(21))                                # 42
cetak(peta([1, 2, 3], kali_dua))                   # [2, 4, 6]
```

Higher-order functions available: `peta`, `saring`, `lipat`, `setiap` (see the
[Branching Guide](04-percabangan.md) and STDLIB).

## Nested functions (closure)

```eve
variabel pengali = 3
fungsi buat_kali() {
    kembali fungsi(x) {
        kembali x * pengali
    }
}
cetak(buat_kali()(7))   # 21
```

A function inside a function can read **global variables**. In v0.1.0,
capturing *local* variables of the outer function (parameters/variables in its
body) is **not supported yet**. Do not rely on it for now.

## Full program example

Save it as `fungsi.eve`, then run: `evernight fungsi.eve`

```eve
# 03 - Fungsi, parameter, dan fungsi sebagai nilai

# Fungsi bernama dengan deklarasi 'fungsi'
fungsi tambah(a, b) {
    kembali a + b
}
cetak("tambah(4, 6) = ", tambah(4, 6))

# Deklarasi parameter dengan nilai awal (sintaksis didukung).
# Catatan v0.1.0: pemanggilan tetap wajib menyediakan SEMUA argumen.
fungsi sapa(nama, salam = "Halo") {
    kembali salam + ", " + nama + "!"
}
cetak(sapa("Budi", "Halo"))
cetak(sapa("Ani", "Selamat malam"))

# Rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
cetak("faktorial(6) = ", faktorial(6))

# Fungsi anonim (lambda) disimpan ke variabel
variabel kali_dua = fungsi(x) { kembali x * 2 }
cetak("kali_dua(21) = ", kali_dua(21))

# Fungsi sebagai nilai (first-class): dikirim sebagai argumen
cetak("peta: ", peta([1, 2, 3], kali_dua))
cetak("saring: ", saring([1, 2, 3, 4], fungsi(x) { kembali x % 2 == 0 }))
cetak("lipat: ", lipat([1, 2, 3, 4], 0, fungsi(acc, x) { kembali acc + x }))

# Fungsi tanpa 'kembali' mengembalikan kosong
fungsi sapa_langsung(nama) {
    cetak("Langsung: ", nama)
}
variabel hasil_kosong = sapa_langsung("Dunia")
cetak("Hasil fungsi tanpa kembali: ", hasil_kosong)

# Fungsi bersarang dapat membaca variabel GLOBAL
variabel pengali = 3
fungsi buat_kali() {
    kembali fungsi(x) {
        kembali x * pengali
    }
}
cetak("fungsi bersarang: ", buat_kali()(7))
```

Output:

```text
tambah(4, 6) = 10
Halo, Budi!
Selamat malam, Ani!
faktorial(6) = 720
kali_dua(21) = 42
peta: [2, 4, 6]
saring: [2, 4]
lipat: 10
Langsung: Dunia
Hasil fungsi tanpa kembali: kosong
fungsi bersarang: 21
```

## Common mistakes

- **Argument count mismatch** →
  `BAHAYA [FUNGSI]: Fungsi 'x' butuh 2 argumen, diberikan 1`.
- **Calling something that is not a function** (e.g. `variabel x = 10` then `x()`)
  → `BAHAYA [FUNGSI]: '10' bukan fungsi`.
- **Forgetting `kembali`** → the function returns `kosong`, not an error. This
  is a common cause of "empty" results.
- **Reading another function's local variables from inside a lambda** →
  `BAHAYA [VARIABLE] ... tidak didefinisikan`. Use global variables in v0.1.0.
- **Empty function** `fungsi f() { }` → `PERINGATAN [WKFUNG]`.
