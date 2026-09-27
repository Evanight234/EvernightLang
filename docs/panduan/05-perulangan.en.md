---
judul: "Loops: selama, untuk, berhenti, lanjut"
grup: "Panduan"
urutan: 5
---

# Loops: while, for, break, continue

This page explains the three loop forms (`selama`, `untuk ... dari ... sampai ...`,
`untuk ... dalam ...`) plus the exit controls `berhenti` and `lanjut`.

## `selama`: repeat while the condition is true

```eve
variabel hitung = 1
selama hitung <= 5 {
    cetak("hitung: ", hitung)
    hitung = hitung + 1
}
```

Make sure the condition can become `salah`, otherwise the program will never
stop.

## `untuk ... dari ... sampai ...`: number range

```eve
untuk i dari 1 sampai 5 {
    cetak(i)
}
```

- The end bound **is included** (inclusive): `dari 1 sampai 5` produces
  `1, 2, 3, 4, 5`.
- The loop variable is created automatically inside the block.

## `untuk ... dalam ...`: iterate a collection

```eve
variabel buah = ["mangga", "jambu", "kelapa"]
untuk item dalam buah {
    cetak("Buah: ", item)
}
```

Works for text too: `untuk huruf dalam "eve" { ... }`.

## `berhenti`: exit the loop

```eve
untuk i dari 0 sampai 100 {
    cetak("hitung: ", i)
    jika i >= 3 {
        berhenti
    }
}
```

## `lanjut`: move to the next iteration

Most stable when used inside `selama`:

```eve
variabel n = 0
selama n < 6 {
    n = n + 1
    jika n % 2 == 0 {
        lanjut
    }
    cetak("ganjil: ", n)
}
```

## Full program example

Save it as `loop.eve`, then run: `evernight loop.eve`

```eve
# 05 - Perulangan: selama, untuk, berhenti, lanjut

# 'selama' - mengulang selama kondisi bernilai benar
variabel hitung = 1
selama hitung <= 5 {
    cetak("selama: ", hitung)
    hitung = hitung + 1
}

# 'lanjut' paling stabil dipakai di dalam 'selama'
variabel n = 0
selama n < 6 {
    n = n + 1
    jika n % 2 == 0 {
        lanjut
    }
    cetak("ganjil: ", n)
}

# 'untuk ... dari ... sampai ...' - rentang angka, batas akhir IKUT dihitung
variabel total = 0
untuk i dari 1 sampai 5 {
    total = total + i
}
cetak("Jumlah 1 sampai 5 = ", total)

# 'untuk ... dalam ...' - mengiterasi isi daftar
variabel buah = ["mangga", "jambu", "kelapa"]
untuk item dalam buah {
    cetak("Buah: ", item)
}

# 'berhenti' - keluar dari perulangan.
# Letakkan 'berhenti' sebagai pernyataan terakhir di dalam bloknya.
untuk i dari 0 sampai 100 {
    cetak("hitung: ", i)
    jika i >= 3 {
        berhenti
    }
}

# Kombinasi: cari nilai pertama dalam daftar
variabel angka = [7, 3, 9, 1]
variabel ketemu = -1
untuk i dari 0 sampai 3 {
    jika angka[i] == 9 {
        ketemu = i
    }
    jika ketemu >= 0 {
        berhenti
    }
}
cetak("Index nilai 9 = ", ketemu)
```

Output:

```text
selama: 1
selama: 2
selama: 3
selama: 4
selama: 5
ganjil: 1
ganjil: 3
ganjil: 5
Jumlah 1 sampai 5 = 15
Buah: mangga
Buah: jambu
Buah: kelapa
hitung: 0
hitung: 1
hitung: 2
hitung: 3
Index nilai 9 = 2
```

## Common mistakes

- **`for i in range(...)` / `while true`**: that style belongs to another
  language. Here: `untuk i dari 0 sampai 5` and `selama benar { ... }`.
- **Forgetting to update the `selama` condition** → infinite loop.
- **`berhenti`/`lanjut` followed by other code in the same block** → the loop
  variable can no longer be read (`BAHAYA [VARIABLE]`). Put `berhenti`/`lanjut`
  as the **last statement** in its block.
- **`lanjut` inside `untuk ... dari ...` or `untuk ... dalam ...`** is not stable
  in v0.1.0 (the counter does not increment / the loop variable disappears). Use
  `selama` for the `lanjut` pattern.
- **`untuk ... dalam` emits `PERINGATAN [WKVAR] Variabel '(indeks)'/'(koleksi)'`**
  : known compiler noise, safe to ignore; the program still runs normally.
- **`dari ... sampai ...` is not exclusive**: if you want to stop before the
  bound, write the end bound minus 1.
