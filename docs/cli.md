---
judul: "CLI & REPL"
grup: "Memulai"
urutan: 4
---

# CLI & REPL

Biner `evernight` adalah compiler + VM + alat bantu dalam satu berkas. Halaman ini merangkum perintah, opsi, dan kode keluar (*exit code*).

## Sintaks umum

```text
evernight                         buka REPL interaktif
evernight <berkas.eve> [opsi]     jalankan berkas (default-run)
evernight run <berkas.eve> [opsi] jalankan berkas (subcommand eksplisit)
evernight <berkas.eve> -- arg1    kirim argumen ke program Anda
evernight --bantuan | -h          panduan bantuan
evernight --versi  | -v           versi (contoh: EvernightLanguage v0.1.0)
```

## Subcommand

| Subcommand | Fungsi |
|------------|--------|
| `run <berkas.eve>` | Jalankan berkas `.eve` |
| `format <berkas.eve>` | Rapikan indentasi & spasi |
| `lint <berkas.eve>` | Periksa gaya kode (aturan `WK*`) |
| `pkg init \| jalankan \| daftar` | Kelola proyek (`eve.json`, manifest) |
| `system info` | Info instalasi terpasang |

### `format`

```text
evernight format program.eve              tulis hasil ke aslinya
evernight format program.eve --keluar x.eve   tulis ke berkas lain
evernight format program.eve --cek        hanya cek; keluar 1 bila belum rapi (untuk CI)
```

Komentar `#` dipertahankan, hasilnya idempoten (diulang dua kali hasilnya sama).

### `lint`

```text
evernight lint program.eve
```

| Kode | Arti |
|------|------|
| `WKHURUF` | Nama bukan `snake_case` |
| `WKIMPOR` | Modul diimpor tapi tidak dipakai |
| `WKPANJANG` | Fungsi > 50 baris |
| `WKPARAM` | Fungsi > 4 parameter |
| `WKSARANG` | Blok kosong |
| `WKMATI` | Kode tak terjangkau |
| `WKMAGIS` | Angka literal "magic" |
| `WKVAR` / `WKREACH` / `WKFUNG` | Peringatan dari compiler |

### `pkg`

```text
evernight pkg init          buat proyek baru (eve.json + utama.eve)
evernight pkg jalankan      jalankan proyek di folder saat ini
evernight pkg daftar        daftar dependensi proyek
```

### `system`

```text
evernight system info       versi, lokasi instalasi, ukuran biner, catatan rilis
```

## Opsi eksekusi

| Opsi | Fungsi |
|------|--------|
| `--cek`, `--check` | Periksa sintaksis & kompilasi **tanpa menjalankan VM** |
| `--tokens` | Tampilkan token hasil lexing |
| `--ast` | Tampilkan pohon sintaksis (AST) |
| `--bytecode` | Tampilkan disassembly bytecode |
| `--debug` | Trace setiap instruksi ke stderr |
| `--waktu` | Profiler: frekuensi opcode + waktu eksekusi |
| `--tanpa-warna` | Matikan output berwarna (juga: env `NO_COLOR`) |

Contoh gabungan:

```text
evernight program.eve --cek
evernight program.eve --tokens --ast
evernight program.eve --debug --waktu
```

Keluaran `--cek` yang berhasil:

```text
Pemeriksaan berhasil: berkas 'program.eve' valid.
```

## Argumen program

```text
evernight program.eve -- arg1 arg2 arg3
```

Di dalam program, argumen diambil dengan fungsi bawaan `argumen()` (mengembalikan daftar teks).

## REPL (Read-Eval-Print Loop)

Jalankan `evernight` tanpa argumen:

```text
EvernightLanguage REPL (v0.1.0)
Ketik kode Evernight atau  :bantuan  untuk daftar perintah,  :keluar  untuk selesai.

eve> cetak(2 + 3)
5
eve> variabel x = 10
eve> x * 2
20
eve> :keluar
Sampai jumpa!
```

| Perintah | Fungsi |
|----------|--------|
| `:bantuan`, `:help`, `:?` | Bantuan REPL |
| `:muat <berkas.eve>` | Muat & jalankan berkas |
| `:bersihkan`, `:clear` | Bersihkan layar |
| `:keluar`, `:exit`, `:quit`, `:q` | Keluar |

Lanjutan blok memakai prompt `... `. Hasil ekspresi dicetak dengan `=> `.

## Kode keluar (*exit code*)

| Kode | Arti |
|-----:|------|
| `0` | Berhasil |
| `1` | Ada `BAHAYA`, kode belum rapi saat `format --cek`, atau ada temuan `lint` |

Karena itu `lint`/`format --cek` bisa langsung dipakai di CI.

## Contoh alur lengkap

```text
evernight --versi
evernight halo.eve
evernight halo.eve --cek
evernight lint halo.eve
evernight format halo.eve
evernight system info
```

## Kesalahan umum

- **`ever` vs `evernight`** — nama biner resmi adalah `evernight`; `ever pkg` hanyalah nama lain untuk `evernight pkg` yang masih dipakai di sebagian teks bantuan.
- **`--cek` di `format` bukan `--cek` di run** — yang pertama cek kerapian format, yang kedua cek sintaksis.
- **`--` wajib** sebelum argumen program agar tidak tertelan oleh CLI.
- **Opsi `--debug`/`--waktu`** menulis ke stderr, jadi gabungkan dengan `2>&1` bila ingin menyimpannya.
