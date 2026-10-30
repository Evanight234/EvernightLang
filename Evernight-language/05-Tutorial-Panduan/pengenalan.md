---
judul: "Pengenalan EvernightLanguage"
grup: "Memulai"
urutan: 1
---

# Pengenalan EvernightLanguage

EvernightLanguage (berkas `.eve`) adalah bahasa pemrograman dengan **sintaksis dan pesan error berbahasa Indonesia**, dibuat untuk semudah Python, secepat C++, dan sefleksibel JavaScript.

## Contoh terkecil

```eve
cetak("Halo Dunia dari EvernightLanguage!")
```

Simpan sebagai `halo.eve`, lalu:

```text
evernight halo.eve
```

## Karakteristik utama

| Aspek | Keputusan |
|-------|-----------|
| Sintaksis | Mirip Python: indentasi untuk keterbacaan, blok `{ }` wajib |
| Kata kunci | Bahasa Indonesia (`jika`, `lainnya_jika`, `selama`, `untuk`, `kembali`, `cetak`, `cocok`, `kasus`, `fungsi`, `variabel`, `tetap`, `impor`) |
| Tipe data | Dinamis, 8 tipe: `angka`, `teks`, `bolean`, `kosong`, `daftar`, `kamus`, `fungsi`, `objek` |
| Model eksekusi | Compiler ke **bytecode** lalu dijalankan **VM** (bukan interpreter baris per baris) |
| Manajemen memori | **ARC** (*Automatic Reference Counting*) |
| Paradigma | Fungsional + OOP |
| Ekstensi berkas | `.eve` |
| Bahasa implementasi | Rust (compiler + VM + CLI), TypeScript (ekstensi editor) |
| Kesalahan | `BAHAYA` dan `PERINGATAN` berbahasa Indonesia, lengkap dengan kode (`[TYPE]`, `[FUNGSI]`, ...) dan nomor baris |
| Lisensi | MIT |

## Alur sebuah program

```text
berkas .eve  ->  Lexer (token)  ->  Parser (AST)  ->  Compiler (bytecode)  ->  VM (hasil)
```

Anda tidak perlu tahu detail alur ini untuk mulai menulis program, tetapi perintah `--tokens`, `--ast`, dan `--bytecode` bisa menampilkannya kalau ingin belajar lebih dalam (lihat [CLI & REPL](cli.md)).

## Anatomi sebuah program

```eve
# Komentar memakai '#'
tetap NAMA = "Evernight"

fungsi sapa(nama) {
    kembali "Halo, " + nama + "!"
}

jika benar {
    cetak(sapa(NAMA))
}
```

- Pernyataan dieksekusi dari atas ke bawah.
- Fungsi `utama()` bersifat opsional — tidak ada titik masuk paksa.
- Titik-koma (`;`) tidak wajib.
- Gaya penulisan disarankan: nama variabel & fungsi `snake_case`, konstanta `HURUF_BESAR`.

## Pustaka standar

Tujuh modul sudah tersedia tanpa instalasi tambahan:

`konsol`, `string`, `matematika`, `daftar`, `kamus`, `sistem`, `utilitas`

Seluruh fungsinya sudah tersedia sebagai fungsi global (`cetak`, `baca`, `akar`, `besar`, `peta`, `tambah`, ...), jadi `impor matematika` hanya penanda.

## Peta dokumentasi

| Saya ingin... | Baca |
|---------------|------|
| Memasang / memeriksa instalasi | [Instalasi](instalasi.md) |
| Menulis program pertama | [Mulai dalam 5 menit](quickstart.md) |
| Belajar tata bahasa langkah demi langkah | [Struktur Dasar Sintaks](panduan/01-sintaks-dasar.md) |
| Mencari perintah terminal | [CLI & REPL](cli.md) |
| Melihat contoh program jadi | [Galeri Contoh](contoh.md) |

## Langkah berikutnya

1. [Instalasi](instalasi.md) — pasang `evernight`.
2. [Mulai dalam 5 menit](quickstart.md) — jalankan program pertama Anda.
