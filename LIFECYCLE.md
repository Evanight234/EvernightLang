# LIFECYCLE.md — Rencana Lifecycle Sistem EvernightLanguage

> Dokumen ini mendefinisikan alur eksekusi program EvernightLanguage — dari kode sumber `.eve` hingga selesai dieksekusi.
> Menjadi acuan implementasi runtime (Phase 2).

---

## 1. Tabel Tahap Lifecycle

| No | Nama Tahap | Kapan Jalan | Apa yang Terjadi | Input | Output | Error |
|----|------------|-------------|------------------|-------|--------|-------|
| 1  | Baca Sumber | Saat program dimulai | Membaca file `.eve` dari disk, atau menerima input baris dari REPL | Path file / string | Teks sumber (`String`) | `BAHAYA [FILE]` — berkas tidak ditemukan / izin ditolak |
| 2  | Tokenisasi (Lexer) | Setelah sumber terbaca | Memecah teks menjadi aliran token (keyword, literal, operator, punctuation) | Teks sumber | `Vec<Token>` | `BAHAYA [SYNTAX]` — string tidak tertutup, token tak terduga |
| 3  | Penguraian (Parser) | Setelah token tersedia | Membangun AST (Abstract Syntax Tree) sesuai grammar EBNF di `GRAMMAR.md` | `Vec<Token>` | `AST` | `BAHAYA [SYNTAX]` — kurung tidak seimbang, blok salah |
| 4  | Kompilasi (Compiler) | Setelah AST valid | Mengubah AST menjadi bytecode (sederet instruksi VM) | `AST` | `Vec<Bytecode>` | `BAHAYA [SYNTAX]` — deklarasi ganda, scope error |
| 5  | Inisialisasi VM | Setelah bytecode siap | Membuat environment baru, memuat bytecode, menyiapkan stack & heap | `Vec<Bytecode>` | `VM instance` | `BAHAYA [MEMORY]` — alokasi memori gagal |
| 6  | Eksekusi (VM) | VM terinisialisasi | Menjalankan bytecode instruksi per instruksi; mengelola scope, memanggil fungsi, resolve `impor` (lazy-load) | `VM instance` | Nilai kembalian / efek samping (`cetak`) | `BAHAYA [TYPE]`, `[INDEX]`, `[KEY]`, `[VARIABLE]`, `[STACK]`, `[DIVISION]`, `[NaN]` |
| 7  | Penanganan Error | Kapan saja (seluruh tahap) | Menangkap `BAHAYA` (fatal → hentikan, exit 1) atau `PERINGATAN` (non-fatal → log, lanjutkan) | Info error | Pesan error terformat sesuai `ERROR.md` | Program berhenti (fatal) atau lanjut (warning) |
| 8  | Cleanup | Setelah eksekusi selesai / error | Melepaskan memori (ARC), menutup handle file, membersihkan environment | `VM instance` | Bersih | — |

---

## 2. Diagram Alur

```
                          ┌─ [PERINGATAN] → log, lanjut ke tahap berikutnya
                          │
[1 Baca] → [2 Lexer] → [3 Parser] → [4 Compiler] → [5 VM Init] → [6 Eksekusi] → [8 Cleanup]
    │            │            │            │              │            │
    │            │            │            │              │            └─ [BAHAYA] → panic → exit(1)
    │            │            │            └─ [MEMORY] error              │
    │            │            └─ [SYNTAX] error                           ↓
    │            └─ [SYNTAX] error                              [7 Error Handler]
    └─ [FILE] error (file tidak ditemukan)
```

**Mode eksekusi:**

```
File mode:  Tahap 1 → 2 → 3 → 4 → 5 → 6 → 8   (full pipeline)
REPL mode:  Tahap 2 → 3 → 4 → 6                (per baris/blok, skip baca file & cleanup global)
```

---

## 3. Pertanyaan Desain

1. **Trigger**: apa yang memicu lifecycle ini dimulai?
   - Jawaban: Perintah CLI (`evernight program.eve`) untuk mode file, atau input di REPL. Lexer dijalankan begitu sumber tersedia.

2. **Ulang**: apakah lifecycle bisa berjalan berulang (loop) atau sekali jalan?
   - Jawaban: **Bisa berulang.** Mode REPL menjalankan pipeline parsial per baris/blok input. Mode file berjalan sekali per file.

3. **Berhenti**: apa kondisi lifecycle berakhir — sukses, error, atau keduanya?
   - Jawaban: **Keduanya.** Sukses → exit code 0. `BAHAYA` (fatal) → berhenti, exit code 1. `PERINGATAN` (non-fatal) → tidak menghentikan, program tetap lanjut.

4. **Cleanup**: apakah ada tahap pembersihan (release memori, tutup file) sebelum berakhir?
   - Jawaban: **Ya.** ARC (Automatic Reference Counting) melepaskan memori secara otomatis saat referensi habis. Resource eksternal (file handle) ditutup eksplisit via `tutup()` atau dalam blok `akhirnya`.

---

## Keputusan Desain Lifecycle

| Keputusan | Pilihan | Alasan |
|-----------|---------|--------|
| Model eksekusi | **Hybrid** (REPL + File) | REPL untuk iterasi cepat, File untuk eksekusi penuh |
| Timing `impor` | **Lazy-load** saat VM | Modul di-resolve saat dieksekusi; modul yang tak terpakai tak di-load |
| Strategi error | **Fatal + Warning** | Sesuai `ERROR.md`; `BAHAYA` panic, `PERINGATAN` lanjut |
| Entry point | **Hybrid** (top-down + `utama()`) | Script kecil: top-down; proyek besar: `utama()` |
| Cleanup | **ARC + `akhirnya` block** | Memori otomatis via ARC; resource eksplisit via `akhirnya` |

---

## Status Finalisasi

- [x] Semua tahap sudah diisi dan urut
- [x] Diagram alur sudah dilengkapi
- [x] Pertanyaan desain sudah dijawab

> Lifecycle ini menjadi acuan implementasi runtime EvernightLanguage.
