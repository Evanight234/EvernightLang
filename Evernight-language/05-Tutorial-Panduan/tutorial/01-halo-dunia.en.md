---
judul: "Stage 1: Hello World"
grup: "Tutorial"
urutan: 1
---

# Stage 1: Hello World

## Goal

Run your first EvernightLanguage program: create a `.eve` file, print text to the
screen, and write comments.

## Concepts

- **`.eve` files**: every EvernightLanguage program is saved with the `.eve` extension.
- **Comments**: start with `#` and are ignored by the computer. Use them to explain code.
- **`cetak(...)`**: prints text/numbers to the screen. It can take several values
  at once, separated by commas:

```eve
cetak("Halo", " ", "Dunia")
```

- **`variabel nama = nilai`**: stores data so it can be used again.

```eve
variabel pesan = "Selamat malam"
cetak(pesan)
```

Note: **no semicolon needed** at the end of a line, and `{ }` is used for code
blocks (functions, `if`, loops).

### Running a program

The basic command is just one: `evernight` followed by the file name:

```powershell
evernight halo.eve          # langsung jalankan
evernight halo.eve --cek    # hanya periksa sintaksis
evernight --versi           # lihat versi
```

There is no separate `build` step: the `.eve` file is read, compiled, then executed
directly by the VM.

### Anatomy of one program

```eve
# 1. komentar diabaikan
variabel nama = "Dunia"     # 2. data disimpan
cetak("Halo ", nama)        # 3. hasil dicetak
```

## Complete Example

```eve
# Program pertama EvernightLanguage
variabel pesan = "Halo Dunia dari EvernightLanguage!"
cetak(pesan)

variabel nama = "Evernight"
cetak("Selamat belajar bersama, ", nama, "!")
```

Save it as `halo.eve`.

## Run

```powershell
evernight halo.eve
```

Expected output:

```
Halo Dunia dari EvernightLanguage!
Selamat belajar bersama, Evernight!
```

You can also run the built-in example from the repo folder:

```powershell
evernight examples/hello.eve
```

## Exercises

1. Change the message text into a short introduction about yourself (name + hobby).
2. Add one `umur` (number) variable then print: `Umur saya `, umur.
3. Create a file `sapa.eve` that prints 3 different greeting lines. Try removing
   the double quotes. What error message do you get? (write it down, useful in Stage 9.)
