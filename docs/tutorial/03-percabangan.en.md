---
judul: "Stage 3: Branching"
grup: "Tutorial"
urutan: 3
---

# Stage 3: Branching

## Goal

Build a program that picks its own execution path: even/odd, positive/zero/negative,
and combinations of several conditions.

## Concepts

### `jika` : `lainnya_jika` : `lainnya`

```eve
jika nilai > 80 {
    cetak("A")
} lainnya_jika nilai > 60 {
    cetak("B")
} lainnya {
    cetak("C")
}
```

Parentheses around the condition are **not used**: `{ }` follows right away.

### Comparison operators

`==` (equal), `!=` (not equal), `<`, `<=`, `>`, `>=`.

Word-based versions are also valid: `sama_dengan`, `lebih_dari`, `kurang_dari`.

### Logic operators

| Word | Function |
|------|----------|
| `dan` | both conditions must be `benar` |
| `atau` | one condition being `benar` is enough |
| `bukan` | flips the boolean value |

```eve
jika umur >= 18 dan punya_ktp { cetak("Boleh") }
jika status == "admin" atau status == "root" { cetak("Istimewa") }
jika bukan (n > 100) { cetak("Di bawah 100") }
```

> Use parentheses after `bukan` so the evaluation order is clear.

### Modulo `%`

The remainder of division, the most common way to check even/odd: `n % 2 == 0`.

## Complete Example

```eve
variabel n = baca_angka("Masukkan bilangan: ")

jika n % 2 == 0 {
    cetak(n, " adalah bilangan genap")
} lainnya {
    cetak(n, " adalah bilangan ganjil")
}

jika n < 0 {
    cetak("Bilangan negatif")
} lainnya_jika n == 0 {
    cetak("Bilangan nol")
} lainnya {
    cetak("Bilangan positif")
}

jika n >= 0 dan n <= 100 {
    cetak("Rentang: 0..100")
} lainnya {
    cetak("Di luar rentang 0..100")
}

jika n % 2 == 0 atau n % 3 == 0 {
    cetak("Kelipatan 2 atau 3")
}

jika bukan (n > 100) {
    cetak("Nilainya di bawah 100 (bukan)")
}
```

## Run

```powershell
evernight percabangan.eve
```

Type `6` when prompted:

```
Masukkan bilangan: 6 adalah bilangan genap
Bilangan positif
Rentang: 0..100
Kelipatan 2 atau 3
Nilainya di bawah 100 (bukan)
```

Try again with `-7` and `0` to see the other branches.

## Exercises

1. Add a special branch: if `n` is divisible by 5, print `"Kelipatan 5"`.
2. Build a grade checker: `90` and above = A, `80`-`89` = B, below = C.
3. Rewrite the program to use `sama_dengan`/`lebih_dari`/`kurang_dari`. Is the
   result exactly the same?
