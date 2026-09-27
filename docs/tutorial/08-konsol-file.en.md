---
judul: "Stage 8: Console Input & Files"
grup: "Tutorial"
urutan: 8
---

# Stage 8: Console Input & Files

## Goal

Accept user input from the terminal (`baca`, `baca_angka`) and save/read file
contents through the `sistem` module.

## Concepts

### Input from the user

```eve
variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Umur: ")
```

- `baca(...)` → returns `teks`
- `baca_angka(...)` → returns `angka` (a non-number input fails)
- `bersihkan()` → clears the screen

### Files (`impor sistem`)

```eve
impor sistem

tulis_file("./catatan.txt", "Isi berkas")
cetak(baca_file("./catatan.txt"))
cetak(ada_file("./catatan.txt"))   # benar
```

| Function | Purpose |
|----------|---------|
| `tulis_file(jalur, teks)` | write / overwrite a file |
| `baca_file(jalur)` | read the whole file contents |
| `ada_file(jalur)` | check whether the file exists |

> **Important rule (sandbox):** file paths may only be inside the program folder
> and **are resolved from the folder where you run the command**. So run `evernight`
> from the same folder where your `.eve` file lives, and write relative paths with
> the `./` prefix, not a bare `data.txt`.

### Time

`waktu_sekarang()` returns a millisecond timestamp; `tunda(0.5)` pauses the program
for half a second.

## Complete Example

```eve
# Masukan pengguna & berkas
impor sistem

variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Umurmu berapa? ")
cetak("Halo ", nama, ", umurmu ", umur, " tahun")

variabel jalur = "./catatan.txt"
tulis_file(jalur, "Halo dari EvernightLanguage!\nNama: " + nama)
cetak("Berkas dibuat: ", ada_file(jalur))
cetak("Isi berkas:")
cetak(baca_file(jalur))

coba {
    baca_file("./tidak_ada.txt")
} tangkap (e) {
    cetak("Gagal membaca: ", e)
}
```

`\n` inside text produces a new line.

## Run

Save it as `catat.eve` **in the same folder**, open a terminal in that folder,
then:

```powershell
evernight catat.eve
```

Type `Budi` then `20`:

```
Siapa namamu? Umurmu berapa? Halo Budi, umurmu 20 tahun
Berkas dibuat: benar
Isi berkas:
Halo dari EvernightLanguage!
Nama: Budi
Gagal membaca: Gagal membaca berkas './tidak_ada.txt': <pesan sistem> (os error 2)
```

The `catatan.txt` file is now saved in the same folder.

Built-in example: `evernight examples/sistem.eve` (run from the repo folder).

## Exercises

1. Ask the user for 3 lines of text, save them to `./pesan.txt`, then print its
   contents back.
2. Create a program that checks `ada_file("./pesan.txt")` and prints the contents
   if it exists, or `"Berkas belum dibuat"` if it does not.
3. Add `cetak("Selesai dalam 1 detik")` + `tunda(1)` at the end of the program.
