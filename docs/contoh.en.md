---
judul: "Example Program Gallery"
grup: "Memulai"
urutan: 5
---

# Example Program Gallery

All of the examples below live in the `examples/` folder and **have all been
tested to run** (exit code `0`).

## Running

```text
evernight examples/hello.eve
evernight examples/import/main.eve
```

Note: run from the **project root** (the folder that contains the `examples/`
folder), because program-relative paths are resolved from the program's folder.

## Core program examples

| File | Contents | Short output |
|------|----------|--------------|
| `examples/hello.eve` | First program: variables + `cetak` | `Halo Dunia dari EvernightLanguage!` |
| `examples/faktorial.eve` | Recursion: `faktorial(5)` | `Faktorial:120` |
| `examples/coba.eve` | Basic arithmetic | `6` |
| `examples/fitur_baru.eve` | `coba`/`tangkap`/`lempar`, `cocok`, `pastikan`, function `utama()`, operator `+=` | `Total: 15`, `Error: data rusak!`, `Semua berhasil!` |
| `examples/string.eve` | `string` module: `besar`, `kecil`, `bersih`, `potong`, `pecah`, `ganti`, `mengandung`, `ulang_teks`, `format` | `besar:   EVERNIGHT LANGUAGE  ` |
| `examples/daftar.eve` | `daftar` module: `tambah`, `sisip`, `hapus`, `urutkan`, `balik`, `unik`, `jumlah`, `rata_rata`, `cari`, `ada`, `iris`, `gabung_larik`, `lipat`, `peta`, `saring`, `setiap` | `Setelah tambah: [5, 3, 8, 1, 3, 9]` |
| `examples/kamus.eve` | `kamus` module: setel, `dapatkan`, `kunci`, `nilai`, `pasangan`, `ada_kunci`, `hapus`, `gabung_kamus` | `Nama: Budi` |
| `examples/matematika.eve` | `matematika` module: `akar`, `pangkat`, `bulat_bawah`, `bulat_atas`, `pembulatan`, `mutlak`, `min`, `max`, `faktorial`, `log`, `sin`, `pi`, `acak_antara` | `akar(144) = 12` |
| `examples/konsol.eve` | `konsol` module: `baca`, `baca_angka`, `bersihkan_layar` | `Halo Budi, tahun depan umurmu 18` |
| `examples/sistem.eve` | `sistem` module: `waktu_sekarang`, `tanggal_sekarang`, `atur_env`, `ada_env`, `baca_file`, `tulis_file`, `jeda` | `Waktu sekarang (ms): ...` |
| `examples/utilitas.eve` | `utilitas` module: `adalah_angka/teks/daftar/kamus`, `ke_boolean`, `ke_larik`, type conversion, `salin` | `adalah_angka(42): benar` |

## Cross-file example

| File | Contents |
|------|----------|
| `examples/import/helper.eve` | Module containing `tambah`, `kali`, `kuadrat` |
| `examples/import/main.eve` | `impor "helper" sebagai mtk` then calls `mtk.tambah(2, 3)` |

```text
evernight examples/import/main.eve
```

```text
Tambah: 5
Kali: 20
Kuadrat: 9
```

Takeaway: a module's exports form **one dict**, so they are called through a
namespace (`mtk.tambah`), not as global names: see the
[Import Guide](panduan/07-impor.md).

## Two supported writing styles

```eve
# Gaya eksplisit (disarankan)
fungsi utama() {
    variabel total = 0
    untuk i dari 1 sampai 5 {
        total += i
    }
    cetak("Total: ", total)
}
utama()
```

```eve
# Gaya langsung (top-down, tanpa fungsi utama)
variabel pesan = "Halo Dunia dari EvernightLanguage!"
cetak(pesan)
```

Assigning to a name that was never declared **creates a new variable**
(automatically), but the recommended default style still writes
`variabel x = ...` so the code stays readable.

## Complete example: factorial

```eve
# examples/faktorial.eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

variabel angka = 5
variabel hasil = faktorial(angka)
cetak("Faktorial:", hasil)
```

```text
Faktorial:120
```

## Common mistakes

- **Wrong folder** → `BAHAYA [FILE] Berkas ... tidak ditemukan`. Move to the
  project root first.
- **Reading `examples/sistem.eve` as a file-access reference**: its paths are
  relative to the program's folder (`examples/contoh_berkas.txt`), not the
  folder where you run the command.
- **`import`/`require` do not exist**: this language uses `impor`.
- **Calling module contents without a namespace** → `BAHAYA [VARIABLE]`.
