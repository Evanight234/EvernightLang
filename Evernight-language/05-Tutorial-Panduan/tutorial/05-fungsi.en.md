---
judul: "Stage 5: Functions"
grup: "Tutorial"
urutan: 5
---

# Stage 5: Functions

## Goal

Write your own functions with parameters and return values, then use recursion as
a way to calculate repeatedly.

## Concepts

### Defining a function

```eve
fungsi tambah(a, b) {
    kembali a + b
}

cetak(tambah(2, 3))   # 5
```

- `fungsi nama(parameter, ...) { }`: the declaration.
- `kembali nilai`: sends a value out. Without `kembali`, the result is `kosong`.
- Functions without parameters still use empty parentheses: `fungsi sapa() { }`.

### Functions without a return value

A function may just perform an action (for example printing):

```eve
fungsi sapa(nama) {
    cetak("Halo, ", nama, "!")
}
sapa("Budi")
```

### Recursion

A function that calls itself. It must have a **basi** (stopping condition):

```eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1            # basi
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
```

`faktorial(5)` → 5 × 4 × 3 × 2 × 1 = `120`.

### Functions as values

Anonymous functions can be stored in a variable (key to recursion and functional
programming):

```eve
variabel ganda = fungsi(x) {
    kembali x * 2
}
cetak(ganda(21))   # 42
```

## Complete Example

```eve
# Fungsi: parameter, kembali, dan rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

fungsi kuadrat(x) {
    kembali x * x
}

fungsi sapa(nama) {
    cetak("Halo, ", nama, "!")
}

fungsi ganjil(n) {
    jika n % 2 == 1 {
        kembali benar
    }
    kembali salah
}

variabel angka = 5
cetak("Faktorial ", angka, " = ", faktorial(angka))
cetak("Faktorial 0 = ", faktorial(0))
cetak("Kuadrat 7 = ", kuadrat(7))
sapa("Budi")
cetak("7 ganjil? ", ganjil(7))
cetak("8 ganjil? ", ganjil(8))

# Fungsi sebagai nilai (anonim)
variabel ganda = fungsi(x) {
    kembali x * 2
}
cetak("Dua kali 21 = ", ganda(21))
```

## Run

```powershell
evernight fungsi.eve
```

```
Faktorial 5 = 120
Faktorial 0 = 1
Kuadrat 7 = 49
Halo, Budi!
7 ganjil? benar
8 ganjil? salah
Dua kali 21 = 42
```

The same built-in example:

```powershell
evernight examples/faktorial.eve
```

## Exercises

1. Create a function `sapa_terbalik(a, b)` that prints `b` then `a`.
2. Write a recursive function `pangkat_rekursif(x, n)` (x to the power of n) with
   base case `n == 0` returning `1`.
3. Modify `faktorial` so it prints a message and does `kembali kosong` when given
   a negative number.
