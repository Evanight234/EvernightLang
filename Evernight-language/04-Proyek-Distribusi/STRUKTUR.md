# STRUKTUR.md — Peta Struktur Project EvernightLanguage

> Peta lengkap seluruh file & direktori proyek.
> Format proyek: **Cargo Workspace (Multi-Crate)** dengan implementasi murni Rust (Zero External Dependencies).

---

## Overview

```
EvernightLang/
│
├── [Dokumen Desain & Manajemen Proyek]        ← semua di root
│   ├── AGENTS.md                 # Aturan untuk AI assistant (opencode)
│   ├── RENCANA.md                # Source of truth arah proyek & fase (7 fase)
│   ├── PROGRES.md                # Log kemajuan harian
│   ├── KONTEKS.md                # Konteks aktif sesi (baca pertama!)
│   ├── KONSEP.md                 # Keputusan konsep bahasa
│   ├── GRAMMAR.md                # Tata bahasa formal EBNF
│   ├── KEYWORD.md                # Daftar kata kunci resmi (~50 keyword)
│   ├── TIPE.md                   # Sistem tipe data
│   ├── ERROR.md                  # Sistem pesan error/warning
│   ├── STDLIB.md                 # Spesifikasi standard library (7 modul)
│   ├── FUNCTION.md               # Spesifikasi fungsi, closure, loop
│   ├── ARRAY.md                  # Spesifikasi tipe daftar
│   ├── LIFECYCLE.md              # Lifecycle alur runtime (8 tahap)
│   ├── LOGO.md                   # Brief & spesifikasi logo + icon + brand kit
│   ├── TEMA.md                   # Spek highlighting & tema warna "Nusantara"
│   ├── CHANGELOG.md              # Log versi (mulai diisi Fase 5)
│   ├── CONTRIBUTING.md           # Panduan kontribusi (Fase 7)
│   ├── CODE_OF_CONDUCT.md        # Kode etik komunitas (Fase 7)
│   └── STRUKTUR.md               # ← File ini (peta struktur)
│
├── Cargo.toml                    # Root Workspace configuration
├── Cargo.lock                    # Lock file dependensi
│
├── crates/
│   ├── evernight_core/           # [Crate 1] Frontend: Lexer, Parser, AST, Errors
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs            # Ekspor modul core
│   │       ├── token.rs          # TokenType enum & Token struct (sumber grammar TextMate)
│   │       ├── lexer.rs          # Handwritten Tokenizer (pelacak baris & kolom)
│   │       ├── ast.rs            # Struktur node AST (Statement, Expression)
│   │       ├── parser.rs         # Handwritten Recursive Descent & Pratt Parser
│   │       └── errors.rs         # Format error BAHAYA [KODE] & PERINGATAN [KODE]
│   │
│   ├── evernight_vm/             # [Crate 2] Backend: Bytecode, VM, Environment, Stdlib
│   │   ├── Cargo.toml            # (depends on evernight_core)
│   │   └── src/
│   │       ├── lib.rs            # Ekspor modul VM
│   │       ├── bytecode.rs       # OpCode enum & Chunk instruksi
│   │       ├── compiler/         # AST → Bytecode Compiler (4 sub-modul)
│   │       │   ├── mod.rs
│   │       │   ├── codegen.rs
│   │       │   ├── expressions.rs
│   │       │   └── statements.rs
│   │       ├── value.rs          # Enum Value runtime & objek ARC (Teks, Daftar, Kamus)
│   │       ├── vm.rs             # Stack-based Virtual Machine (sistem handler error)
│   │       └── stdlib/           # Pustaka Standar bawaan (7 modul) — Fase 3
│   │           ├── mod.rs
│   │           ├── konsol.rs     # cetak, baca, bersihkan
│   │           ├── string_mod.rs # besar, kecil, potong, pecah, gabung
│   │           ├── daftar.rs     # peta, saring, lipat, urutkan
│   │           ├── matematika.rs # akar, pangkat, sin, cos, pi, e
│   │           ├── kamus.rs      # kunci, nilai, pasangan, dapatkan
│   │           ├── utilitas.rs   # pemeriksaan tipe & clone
│   │           └── sistem.rs     # file I/O, timestamp, env
│   │
│   └── evernight_cli/            # [Crate 3] Entry Point: Biner & REPL
│       ├── Cargo.toml            # (depends on core, vm, fmt, lint) — biner `evernight`
│       └── src/
│           ├── main.rs           # CLI entry point (arg parsing zero-deps) → `evernight.exe`
│           ├── repl.rs           # REPL interaktif (Fase 4)
│           ├── printer.rs        # format_error + disassembler bytecode
│           ├── json.rs           # parser/serializer JSON minimal (Fase 6D-6)
│           └── pkg.rs            # `evernight pkg` — package manager minimal
│
│   ├── evernight_fmt/            # [Crate 4] Formatter (Fase 6D-3)
│   │   ├── Cargo.toml
│   │   └── src/lib.rs            # format() + sudah_rapi() — berbasis token, idempoten
│   │
│   └── evernight_lint/           # [Crate 5] Linter (Fase 6D-4)
│       ├── Cargo.toml
│       └── src/lib.rs            # lint() — 7 aturan WK* di atas AST
│
├── assets/                       # Aset identitas visual (Fase 3)
│   ├── evernight instaler models.jpeg  # Gambar referensi user untuk installer
│   └── logo/                     # Logo resmi + icon .eve (16/32/48/256px + .ico)
│       ├── logo.svg / logo.png
│       ├── icon_16.png, icon_32.png, icon_48.png, icon_256.png
│       └── icon.ico
│
├── editors/                      # Dukungan editor (Fase 4)
│   └── vscode/                   # Ekstensi VS Code Evernight
│       ├── package.json          # (language id `evernight`, icon file)
│       ├── syntaxes/
│       │   └── evernight.tmLanguage.json  # grammar digenerate dari token.rs
│       ├── themes/
│       │   ├── nusantara-dark.json
│       │   └── nusantara-light.json
│       ├── snippets/eve.json
│       └── README.md
│
├── paket/                        # Keluaran distribusi (Fase 6)
│   ├── evernight-0.1.0-windows-x64/   # Paket portabel (biner + aset + skrip installer)
│   │   ├── bin/evernight.exe
│   │   ├── assets/ (icon.ico, logo.png, icon_256.png)
│   │   ├── docs/PANDUAN.txt
│   │   ├── extensions/evernight-language-0.1.0.vsix
│   │   ├── install.ps1 / install.cmd
│   │   ├── uninstall.ps1 / uninstall.cmd
│   │   └── BACA-AKU.txt
│   └── evernight-0.1.0-windows-x64.zip
│
├── installer/                    # Installer GUI Rust/egui (Fase 6F/7E)
│   ├── Cargo.toml                # eframe + egui + image + rfd (crate DI LUAR workspace)
│   ├── src/
│   │   ├── main.rs               # entry; flag --uninstall, --diam, --mulai N
│   │   ├── app.rs                # state wizard, routing halaman, bilah judul
│   │   ├── tema.rs               # ⭐ semua warna/font/ukuran window/animasi
│   │   ├── anim.rs               # easing, hover, riak, progress, transisi
│   │   ├── pasang.rs             # logika: PATH, registry, ekstensi, uninstaller
│   │   └── ui/
│   │       ├── mod.rs
│   │       ├── komponen.rs       # tombol, progress, penanda langkah, kartu
│   │       └── halaman.rs        # 6 halaman wizard + pemilih folder (rfd)
│   ├── aset/
│   │   ├── maskot.png            # cutout transparan (panel 28%)
│   │   ├── fonts/                # Jakarta Sans + JetBrains Mono (OFL)
│   │   ├── logo/                 # icon.ico, logo.png
│   │   ├── payload/              # evernight.exe, .vsix, PANDUAN.txt, LICENSE
│   │   └── tinjau/               # pratinjau 6 halaman (hasil uji, 1000x640)
│   ├── alat/cutout_maskot.py     # generator cutout maskot
│   └── build-installer.ps1       # build → payload → Setup.exe + SHA256
│
├── .github/workflows/            # CI & rilis (Fase 6E)
│   ├── ci.yml                    # fmt + clippy + test + verifikasi contoh
│   └── rilis.yml                 # build statis → ZIP → SHA256SUMS → Release
│
├── situs/                        # Website + playground WASM (Fase 8)
│
├── tests/                        # Integration / golden tests
│   └── golden/                   # Kasus uji end-to-end (.eve + .harapan)
│
└── examples/                     # Contoh program .eve
    ├── hello.eve                 # "Halo Dunia"
    ├── faktorial.eve             # Rekursi faktorial
    ├── fitur_baru.eve            # untuk/cocok/coba-tangkap/kamus/daftar
    └── error_handling.eve        # Coba / tangkap / lempar
```

---

## Penjelasan Crate Workspace

### 1. `crates/evernight_core` (Frontend)
Tanggung jawab: Membaca teks program mentah hingga menghasilkan AST yang valid dan siap dikompilasi.
- **`token.rs`**: Representasi seluruh kata kunci (~50 kata), literal, dan operator.
- **`lexer.rs`**: Pemindai karakter manual tanpa library luar, mencatat baris dan kolom persis.
- **`ast.rs`**: Struktur data pohon sintaksis abstrak.
- **`parser.rs`**: Parser kombinasi Recursive Descent (statement) dan Pratt Parser (ekspresi operator).
- **`errors.rs`**: Formatter pesan kesalahan standar `BAHAYA [KODE]` dan `PERINGATAN [KODE]`.

### 2. `crates/evernight_vm` (Backend & Runtime)
Tanggung jawab: Mengubah AST menjadi bytecode dan mengeksekusinya di stack-based VM.
- **`bytecode.rs`**: Definisi instruksi VM (OpCode, Konstanta, Chunk).
- **`compiler.rs`**: Mengubah node AST menjadi instruksi bytecode berurutan.
- **`value.rs`**: Tipe data runtime `Value` yang aman menggunakan `Arc<RefCell<...>>` untuk koleksi di heap.
- **`environment.rs`**: Manajemen tabel simbol dan variabel lokal/global.
- **`vm.rs`**: Mesin virtual berbasis stack yang mengeksekusi instruksi bytecode.
- **`stdlib/`**: Modul bawaan bahasa (`konsol`, `string`, `daftar`, `matematika`, `kamus`, `utilitas`, `sistem`).

### 3. `crates/evernight_cli` (Runner & REPL)
Tanggung jawab: Binary executable utama yang dipanggil pengguna lewat terminal.
- **`main.rs`**: Membaca opsi baris perintah (`evernight berkas.eve`).
- **`repl.rs`**: Antarmuka REPL interaktif untuk pengujian baris per baris.

---

## Keputusan Arsitektur Terpilih

| Aspek | Pilihan | Alasan |
|-------|---------|--------|
| **Struktur Proyek** | Cargo Workspace (3 crates) | Modularitas tinggi: Core, VM, dan CLI terpisah rapi |
| **Lexer & Parser** | Handwritten (Manual) | Kontrol penuh pesan error Bahasa Indonesia & nomor baris/kolom |
| **Dependensi Eksternal** | Zero Dependencies (Stdlib Rust murni) | Sangat cepat di-compile, bebas bloat, portabel |
| **Virtual Machine** | Stack-based VM | Sederhana, andal, mudah dipetakan dari AST |
| **Model Memori (ARC)** | Safe Enum `Value` + `Arc<RefCell<T>>` | 100% Safe Rust, sesuai spesifikasi ARC di KONSEP.md |

---

## Urutan Pengerjaan Implementasi (Fase 2)

```
1. Setup Workspace Root & Crate Skeletons (evernight_core, evernight_vm, evernight_cli)
2. evernight_core::token    → Definisikan TokenType & Token
3. evernight_core::errors   → Definisikan sistem error BAHAYA/PERINGATAN
4. evernight_core::lexer    → Implementasi tokenizer manual + unit test
5. evernight_cli::main      → Wire CLI awal untuk verifikasi tokenization
6. evernight_core::ast      → Definisikan struktur node AST
7. evernight_core::parser   → Implementasi parser manual (Pratt parsing)
8. evernight_vm::bytecode   → Definisikan OpCode & Chunk
9. evernight_vm::value      → Definisikan Enum Value & ARC
10. evernight_vm::compiler  → Kompilasi AST ke bytecode
11. evernight_vm::vm        → Eksekusi instruksi di Virtual Machine
12. evernight_vm::stdlib    → Implementasi 7 modul bawaan
```
> `c_interop/` (FFI Rust↔C) dihapus permanen — implementasi murni Rust. Edisi asli: 14 langkah tanpa item 13.

---

## Status

- [x] Dokumen desain (Fase 1) — semua file `.md` di root
- [x] Setup Cargo Workspace & Crate Skeletons
- [x] Token & Lexer (`evernight_core`)
- [x] AST & Parser (`evernight_core`)
- [x] Bytecode & Compiler (`evernight_vm`)
- [x] Value & VM (`evernight_vm`)
- [x] Implementasi murni Rust (C dibatalkan permanen)
- [x] Setup TypeScript (`editors/vscode`) — scaffolding ekstensi
- [x] Biner `evernight` ([[bin]] di `evernight_cli`) — `evernight.exe`
- [ ] Standard Library (`evernight_vm::stdlib`) — Fase 3
- [ ] Identitas visual (`assets/logo`, `LOGO.md`) — Fase 3
- [ ] CLI lengkap & REPL (`evernight_cli`) — Fase 4
- [ ] Asosiasi `.eve` → "Evernight files" (ProgID `EvernightFile`, installer) — Fase 6
- [ ] Highlighting & ekstensi editor (`editors/vscode`, `TEMA.md`) — Fase 4
- [ ] Tooling & distribusi (`paket`, `situs`, publish) — Fase 6
- [ ] Komunitas & Rilis 1.0 — Fase 7
