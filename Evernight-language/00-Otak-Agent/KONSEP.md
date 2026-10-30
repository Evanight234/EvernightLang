# KONSEP.md — Konsep Pemrograman EvernightLanguage

> Sumber keputusan konsep bahasa. Pelengkap `RENCANA.md` dan `KEYWORD.md`.

## 1. Keputusan yang Sudah Ada

| Aspek | Keputusan |
|-------|-----------|
| Nama | EvernightLanguage |
| Ekstensi file | `.eve` |
| Paradigma | Fungsional + OOP |
| Tipe | Dinamis |
| Eksekusi | Compiler bytecode + VM (seperti Python/Lua; `.eve` → AST → bytecode → VM) |
| Biner / CLI | `evernight` (crate `evernight_cli`, biner `evernight`) |
| Asosiasi file | `.eve` → "Evernight files" (Windows, ProgID `EvernightFile`) — Fase 6 |
| Manajemen memori | ARC (Automatic Reference Counting) |
| Error | Pesan `BAHAYA` / `PERINGATAN` dalam Bahasa Indonesia |
| Implementasi | Rust murni (runtime) + TypeScript (ekstensi editor) |

## 2. Konsep yang Direkomendasikan

| Konsep | Status | Alasan |
|--------|--------|--------|
| First-class function | dipakai | Fungsi jadi nilai: bisa disimpan di variabel, dikirim sebagai argumen |
| Closure | dipakai | Fungsi mengingat scope tempat ia dibuat |
| Rekursi | dipakai | Cara utama perulangan di sisi fungsional |
| Lexical scope | dipakai | Scope ditentukan posisi kode, standar & sederhana |
| `cetak` sebagai efek samping | dipakai (pengecualian) | Tanpa ini bahasa tidak bisa I/O |
| OOP (class/object) | dipakai | User meminta — class untuk model data & method |
| Loop `selama`/`untuk` | opsional | Bentrok dengan gaya fungsional; default: rekursi dulu |
| Pattern matching | Fase 1 (`cocok`, `kasus`, `bawaan`, `_`) | Struktur `cocok` + wildcard `_` didukung di Fase 1 |
| Tail-call optimization | nanti | Performa rekursi dalam |
| Immutability penuh | tidak | Dihapus oleh keputusan user |

## 3. Pertanyaan Desain (isi nanti)

1. **Entry point**: `utama()` atau file `.eve` dieksekusi dari atas ke bawah?
   - Jawaban: _
2. **Loop**: `selama`/`untuk` tetap ada, atau cukup rekursi?
   - Jawaban: _
3. **OOP model**: class seperti Python, atau struktur data + method terpisah?
   - Jawaban: _
4. **`peta`/`saring`/`lipat`**: keyword bawaan atau stdlib?
   - Jawaban: _
