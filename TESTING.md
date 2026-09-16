# TESTING.md — Panduan & Laporan Pengujian EvernightLanguage

> Dokumen resmi strategi pengujian, verifikasi kualitas internal, benchmark performa, dan pengukuran cakupan kode (Fase 5).

---

## 1. Arsitektur & Struktur Pengujian

Pengujian EvernightLanguage terbagi menjadi beberapa lapisan terisolasi untuk memastikan stabilitas dan keandalan sistem:

```
EvernightLang/
├── crates/
│   ├── evernight_core/
│   │   └── tests/
│   │       └── fuzz_parser.rs          # Fuzzing anti-panic (15.000 iterasi deterministik)
│   ├── evernight_vm/
│   │   ├── tests/
│   │   │   ├── common/mod.rs           # Shared test fixtures & runner helpers
│   │   │   ├── test_konsol.rs          # Unit test modul konsol & I/O
│   │   │   ├── test_string.rs          # Unit test modul string
│   │   │   ├── test_matematika.rs      # Unit test modul matematika & trigonometri
│   │   │   ├── test_daftar.rs          # Unit test operasi array & higher-order
│   │   │   ├── test_kamus.rs           # Unit test operasi dictionary & keyword hapus
│   │   │   ├── test_sistem.rs          # Unit test waktu, berkas, env
│   │   │   ├── test_utilitas.rs        # Unit test tipe, konversi, deep copy
│   │   │   ├── test_impor.rs           # Unit test impor lintas berkas & circular check
│   │   │   └── test_vm_core.rs         # Unit test incremental VM, argumen, output capture
│   │   └── benches/
│   │       └── benchmark.rs            # Baseline benchmark performa (Instant zero-dep)
│   └── evernight_cli/
│       └── tests/
│           ├── cli_tests.rs            # Integration test perintah CLI & flag runner
│           ├── error_codes_test.rs     # Verifikasi pesan error BAHAYA & PERINGATAN
│           └── golden_tests.rs         # Runner Golden Test end-to-end
└── tests/
    └── golden/                         # Berkas uji end-to-end (.eve & .harapan)
        ├── 01_halo.eve / .harapan
        ├── 02_faktorial.eve / .harapan
        ├── 03_perulangan.eve / .harapan
        ├── 04_koleksi.eve / .harapan
        ├── 05_error_handling.eve / .harapan
        └── 06_kondisi_cocok.eve / .harapan
```

---

## 2. Cara Menjalankan Pengujian

### A. Seluruh Workspace
```bash
cargo test --workspace
```

### B. Unit Test per Crate
```bash
# Frontend Core (Lexer & Parser)
cargo test -p evernight_core

# Backend VM & Standard Library (52 tests modular)
cargo test -p evernight_vm

# CLI Runner & Integration
cargo test -p evernight_cli
```

### C. Golden Tests End-to-End
```bash
cargo test -p evernight_cli --test golden_tests
```

### D. Fuzz Testing Parser Anti-Panic (15.000 Iterasi)
```bash
cargo test -p evernight_core --test fuzz_parser -- --nocapture
```

### E. Verifikasi Seluruh Kode Kesalahan (Error Codes)
```bash
cargo test -p evernight_cli --test error_codes_test
```

---

## 3. Hasil Benchmark Baseline (Performa)

Dijalankan menggunakan target `[[bench]]` murni (zero external dependency) berbasis `std::time::Instant`:

```bash
cargo bench -p evernight_vm
```

Hasil pengukuran pada mesin pengembang (Windows x86_64):

| Pengujian | Rata-rata (avg) | Median | Min | Max | Sampel (N) |
|---|---|---|---|---|---|
| **Kompilasi (Lex + Parse + Codegen)** | 34.28 µs | **31.80 µs** | 18.90 µs | 127.90 µs | 500 |
| **Eksekusi Rekursi Faktorial(20)** | 2.29 µs | **2.00 µs** | 1.50 µs | 13.00 µs | 1.000 |
| **Operasi Koleksi (peta + saring)** | 6.48 µs | **6.30 µs** | 6.00 µs | 39.30 µs | 1.000 |
| **Eksekusi Loop (10k iterasi)** | 2.28 ms | **1.96 ms** | 1.88 ms | 6.23 ms | 100 |

*Catatan: 1 mikrodetik (µs) = 0.001 milidetik (ms). Waktu kompilasi dan eksekusi fungsi dasar berada di skala mikrodetik, membuktikan efisiensi model Stack VM dan Pratt Parser.*

---

## 4. Pengukuran Cakupan Kode (Code Coverage)

### Menggunakan `cargo-llvm-cov` (Direkomendasikan)

Prasyarat komponen LLVM:
```bash
rustup component add llvm-tools-preview
cargo install cargo-llvm-cov
```

Jalankan laporan cakupan dalam terminal:
```bash
cargo llvm-cov --workspace
```

Atau hasilkan laporan visual HTML:
```bash
cargo llvm-cov --workspace --html
# Buka target/llvm-cov/html/index.html di peramban
```

### Menggunakan `cargo-tarpaulin` (Alternatif)
```bash
cargo install cargo-tarpaulin
cargo tarpaulin --workspace --out Html
```

---

## 5. Ringkasan Status Kualitas

- **Total Test Otomatis**: **85+ tests** (8 core + 52 vm + 10 cli integration + 12 error codes + 1 golden runner (6 e2e suites) + 2 fuzzing suites).
- **Hasil Pengujian**: **100% HIJAU (PASSED)**.
- **Linter & Warning**: **`cargo clippy` 0 warning** (`-D warnings`).
- **Ketahanan Parser**: Lolos 15.000 iterasi fuzzing input acak tanpa panic (dilindungi batas kedalaman rekursi `MAX_RECURSION_DEPTH = 64`).
