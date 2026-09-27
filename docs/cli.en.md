---
judul: "CLI & REPL"
grup: "Memulai"
urutan: 4
---

# CLI & REPL

The `evernight` binary is compiler + VM + tools in a single file. This page
summarizes the commands, options, and *exit codes*.

## General syntax

```text
evernight                         buka REPL interaktif
evernight <berkas.eve> [opsi]     jalankan berkas (default-run)
evernight run <berkas.eve> [opsi] jalankan berkas (subcommand eksplisit)
evernight <berkas.eve> -- arg1    kirim argumen ke program Anda
evernight --bantuan | -h          panduan bantuan
evernight --versi  | -v           versi (contoh: EvernightLanguage v0.1.0)
```

## Subcommands

| Subcommand | Function |
|------------|----------|
| `run <berkas.eve>` | Run a `.eve` file |
| `format <berkas.eve>` | Tidy indentation & spacing |
| `lint <berkas.eve>` | Check code style (rules `WK*`) |
| `pkg init \| jalankan \| daftar` | Manage projects (`eve.json`, manifest) |
| `system info` | Info about the installed setup |

### `format`

```text
evernight format program.eve              tulis hasil ke aslinya
evernight format program.eve --keluar x.eve   tulis ke berkas lain
evernight format program.eve --cek        hanya cek; keluar 1 bila belum rapi (untuk CI)
```

`#` comments are preserved and the result is idempotent (running it twice
yields the same output).

### `lint`

```text
evernight lint program.eve
```

| Code | Meaning |
|------|---------|
| `WKHURUF` | Name is not `snake_case` |
| `WKIMPOR` | Module imported but unused |
| `WKPANJANG` | Function > 50 lines |
| `WKPARAM` | Function > 4 parameters |
| `WKSARANG` | Empty block |
| `WKMATI` | Unreachable code |
| `WKMAGIS` | "Magic" literal number |
| `WKVAR` / `WKREACH` / `WKFUNG` | Warnings from the compiler |

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

## Execution options

| Option | Function |
|--------|----------|
| `--cek`, `--check` | Check syntax & compilation **without running the VM** |
| `--tokens` | Show the tokens produced by lexing |
| `--ast` | Show the syntax tree (AST) |
| `--bytecode` | Show the bytecode disassembly |
| `--debug` | Trace every instruction to stderr |
| `--waktu` | Profiler: opcode frequency + execution time |
| `--tanpa-warna` | Turn off colored output (also: env `NO_COLOR`) |

Combined example:

```text
evernight program.eve --cek
evernight program.eve --tokens --ast
evernight program.eve --debug --waktu
```

Successful `--cek` output:

```text
Pemeriksaan berhasil: berkas 'program.eve' valid.
```

## Program arguments

```text
evernight program.eve -- arg1 arg2 arg3
```

Inside the program, arguments are read with the built-in `argumen()` function
(returns a list of text).

## REPL (Read-Eval-Print Loop)

Run `evernight` with no arguments:

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

| Command | Function |
|---------|----------|
| `:bantuan`, `:help`, `:?` | REPL help |
| `:muat <berkas.eve>` | Load & run a file |
| `:bersihkan`, `:clear` | Clear the screen |
| `:keluar`, `:exit`, `:quit`, `:q` | Exit |

Block continuation uses the `... ` prompt. Expression results are printed with
`=> `.

## Exit codes

| Code | Meaning |
|-----:|---------|
| `0` | Success |
| `1` | A `BAHAYA` occurred, code was not tidy during `format --cek`, or `lint` found issues |

Because of that, `lint`/`format --cek` can be used directly in CI.

## Complete flow example

```text
evernight --versi
evernight halo.eve
evernight halo.eve --cek
evernight lint halo.eve
evernight format halo.eve
evernight system info
```

## Common mistakes

- **`ever` vs `evernight`**: the official binary name is `evernight`; `ever pkg`
  is just another name for `evernight pkg` still used in some help texts.
- **`--cek` in `format` is not `--cek` in run**: the first checks formatting
  tidiness, the second checks syntax.
- **`--` is required** before program arguments so the CLI does not swallow them.
- **`--debug`/`--waktu` options** write to stderr, so combine them with `2>&1`
  if you want to save the output.
