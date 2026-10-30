---
judul: "Stage 4: Loops"
grup: "Tutorial"
urutan: 4
---

# Stage 4: Loops

## Goal

Repeat work with `untuk` and `selama`, then control the flow with `berhenti` and
`lanjut` through a number series program.

## Concepts

### `untuk i dari A sampai B`

Counts from `A` to `B`: **both ends are included**.

```eve
untuk i dari 1 sampai 5 {
    cetak(i)      # 1, 2, 3, 4, 5
}
```

### `untuk x dalam koleksi`

Iterates over the contents of a list (or the characters of a text):

```eve
untuk buah dalam ["apel", "jeruk"] {
    cetak(buah)
}
```

> Right now the compiler shows a wrong `PERINGATAN [WKVAR]` for this statement.
> The warning is safe to ignore: the program still runs correctly.

### `selama kondisi { }`

Repeats while the condition is still `benar`. You must update the counter yourself.

```eve
variabel i = 0
selama i < 3 {
    i = i + 1
    cetak(i)
}
```

### `berhenti` and `lanjut`

- `berhenti`: exits the loop.
- `lanjut`: goes straight to the next iteration (skips the rest of the block).

Both are used inside a `selama` loop:

```eve
selama i < 10 {
    i = i + 1
    jika i % 2 != 0 {
        lanjut          # lewati ganjil
    }
    jika i > 6 {
        berhenti        # berhenti di 8
    }
    cetak(i)
}
```

## Complete Example

```eve
# Deret bilangan
cetak("Deret 1 sampai 5:")
untuk i dari 1 sampai 5 {
    cetak(i, " ")
}
cetak("")

variabel jumlah = 0
untuk i dari 1 sampai 5 {
    jumlah = jumlah + i
}
cetak("Jumlah 1..5 = ", jumlah)

# Berhenti & lanjut dipakai di dalam selama
variabel langkah = 0
variabel total = 0
selama langkah < 20 {
    langkah = langkah + 1
    jika langkah % 2 != 0 {
        lanjut
    }
    jika langkah > 12 {
        berhenti
    }
    total = total + langkah
}
cetak("Jumlah bilangan genap sampai 12 = ", total)

# Iterasi isi daftar
variabel buah = ["apel", "mangga", "jeruk"]
untuk b dalam buah {
    cetak("Buah: ", b)
}
```

## Run

```powershell
evernight deret.eve
```

```
Deret 1 sampai 5:
1 
2 
3 
4 
5 

Jumlah 1..5 = 15
Jumlah bilangan genap sampai 12 = 42
Buah: apel
Buah: mangga
Buah: jeruk
```

Before the `Buah: apel` line you may see 2 `PERINGATAN [WKVAR]` lines. Ignore
them: it is a small compiler bug for `untuk ... dalam`, not an error in your program.

## Exercises

1. Print a countdown from 10 down to 1 with `untuk`.
2. Sum the numbers from 1 to 100, **only** the multiples of 3, using `selama` +
   `lanjut`.
3. Change the `berhenti` part so it stops right after the total passes 30. What
   is the final value of `total`?
