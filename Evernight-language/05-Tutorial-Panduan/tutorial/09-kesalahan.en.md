---
judul: "Stage 9: Handling Errors"
grup: "Tutorial"
urutan: 9
---

# Stage 9: Handling Errors

## Goal

Stop the program from dying right away when something goes wrong: wrap risky code
with `coba` / `tangkap`, throw your own errors with `lempar`, and validate
conditions with `pastikan`.

## Concepts

### `coba` : `tangkap`

```eve
coba {
    variabel d = [1, 2]
    cetak(d[9])
} tangkap (e) {
    cetak("Terjadi kesalahan: ", e)
}
```

The parentheses after `tangkap` are **required**. Variable `e` holds the error
message in Indonesian.

### `lempar("pesan")`

Throw your own error, usually when the data is invalid:

```eve
coba {
    jika usia < 0 {
        lempar("Usia tidak boleh negatif!")
    }
} tangkap (e) {
    cetak("Error: ", e)
}
```

### `pastikan(kondisi, "pesan")`

Validate a condition. When the condition is `salah`, an error with that message
fires right away, a perfect pair with `coba`/`tangkap`:

```eve
coba {
    pastikan(b != 0, "Pembagi tidak boleh nol!")
} tangkap (e) {
    cetak("Gagal: ", e)
}
```

### `akhirnya { }`

A block that **always** runs, whether it succeeds or fails.

### Limitations to know about

Right now `tangkap` only catches errors that happen **inside the `coba` block in
the same function**. Errors from other functions that get called (including a
`lempar` inside them) propagate upward and stop the program. So place
`coba`/`tangkap` inside the function that produces the error, like the `bagi`
function below.

Uncaught errors stop the program with a `BAHAYA [KODE]: Baris X - ...` message.

## Complete Example

```eve
fungsi bagi(a, b) {
    coba {
        pastikan(b != 0, "Pembagi tidak boleh nol!")
        kembali a / b
    } tangkap (e) {
        cetak("Tertangkap: ", e)
        kembali kosong
    }
}

cetak("10 / 2 = ", bagi(10, 2))
cetak("10 / 0 = ", bagi(10, 0))

variabel usia = -5
coba {
    jika usia < 0 {
        lempar("Usia tidak boleh negatif!")
    }
    cetak("Usia diterima: ", usia)
} tangkap (e) {
    cetak("Error: ", e)
}

coba {
    variabel d = [1, 2]
    cetak(d[9])
} tangkap (e) {
    cetak("Error: ", e)
} akhirnya {
    cetak("Blok akhirnya dijalankan")
}

pastikan(1 + 1 == 2, "Hitungan rusak!")
cetak("Semua aman!")
```

## Run

```powershell
evernight coba_tangkap.eve
```

```
10 / 2 = 5
Tertangkap: Pembagi tidak boleh nol!
10 / 0 = kosong
Error: Usia tidak boleh negatif!
Error: Indeks 9 di luar batas daftar [0..1]
Blok akhirnya dijalankan
Semua aman!
```

Try changing the last line to `pastikan(1 + 1 == 3, "Hitungan rusak!")`. The
program will stop with `BAHAYA [ASSERT] Baris ... - Hitungan rusak!`.

Built-in example: `evernight examples/fitur_baru.eve`.

## Exercises

1. Create a function `bagi_aman(a, b)` that returns `"nol tidak boleh"` when
   `b == 0`.
2. Wrap the number reading from the user with `coba`/`tangkap` so bad input does
   not stop the program.
3. Write one `pastikan` with your own error message, then look at its
   `BAHAYA [ASSERT]` display without `coba`.
