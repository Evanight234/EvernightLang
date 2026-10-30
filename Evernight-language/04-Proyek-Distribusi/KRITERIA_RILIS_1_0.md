# Kriteria Rilis 1.0 — EvernightLanguage

Dokumen ini mendefinisikan **kapan** EvernightLanguage dianggap "selesai" untuk rilis 1.0.
Semua item harus terpenuhi sebelum tag `v1.0` dibuat.

---

## 1. Bahasa & Sintaksis

- [x] Keyword Indonesia: `fungsi`, `jika`/`lainnya`, `selama`, `untuk`/`dari`/`sampai`, `kembali`, `cetak`, `baca`, `benar`/`salah`, `kosong`, `variabel`, `tetap`, `impor`/`sebagai`, `coba`/`tangkap`, `lempar`
- [x] Tipe data: `Teks`, `Angka` (f64), `Bolean`, `Kosong`, `Daftar`, `Kamus`, `Fungsi`
- [x] Ekspresi: aritmatika, perbandingan, logika, string concat, indeks, properti `.panjang`
- [x] Kontrol alur: `jika`/`lainnya`, `selama`, `untuk` (range), `coba`/`tangkap`
- [x] Fungsi: deklarasi, parameter, return, rekursi, closure
- [x] Higher-order: `peta`, `saring`, `lipat`, `setiap`
- [x] Impor modul lintas berkas (lazy-load, cache, deteksi siklus)

## 2. Bytecode & VM

- [x] 53 opcode v1 (frozen — tambah/hapus opcode = breaking change)
- [x] `BYTECODE_VERSION = 1` (golden test: `opcode_v1_daftar_lengkap`)
- [x] VM: stack-based, call frames, error handling (try/catch), sandbox path

## 3. Standard Library

- [x] 7 modul: `konsol`, `string`, `matematika`, `daftar`, `kamus`, `sistem`, `utilitas`
- [x] ~60+ fungsi built-in (termasuk fungsi global: `tambah`, `sisip`, `urutkan`, `rata2`, `panjang`, dll)
- [x] Error code per modul + pesan Bahasa Indonesia (`BAHAYA`/`PERINGATAN`)

## 4. CLI & Tooling

- [x] `evernight run <file.eve>` — eksekusi program
- [x] `evernight format <file.eve>` — formatter (idempoten, pertahankan komentar)
- [x] `evernight lint <file.eve>` — linter (aturan WK*)
- [x] `evernight pkg init|jalankan|daftar` — package manager
- [x] `evernight system info` — info instalasi
- [x] `--debug` trace, `--waktu` profiler
- [x] Error output berwarna (BAHAYA merah, PERINGATAN kuning)

## 5. Editor & IDE

- [x] Ekstensi VS Code (TextMate grammar, tema "Nusantara", snippet, icon theme)
- [x] Static CompletionProvider (~60 item)
- [x] Kompatibel: VS Code, Cursor, Windsurf, VSCodium, Antigravity

## 6. Distribusi

- [x] Biner release statis Windows x64 (`+crt-static`)
- [x] Installer GUI (egui/Rust) — wizard per-user/per-machine
- [x] Uninstaller 3 tahap + `kill evernight system`
- [x] Updater CLI + `update evernight system` + `evernight system info`
- [x] Paket portabel ZIP + `paket-rilis.ps1` (otomatis)

## 7. Keamanan & Kualitas

- [x] Sandbox path: `baca_file`/`tulis_file`/`ada_file`/`impor` terkunci ke direktori program
- [x] Deteksi siklus impor
- [x] Unit test ≥160 (semua hijau)
- [x] Clippy zero warning (`-D warnings`)
- [x] Edge-case test (pembagian nol, rekursi dalam, string kosong/panjang, daftar besar, try/catch)

## 8. Dokumentasi

- [ ] README publik (logo, quickstart, status fase)
- [ ] Panduan sintaks (referensi keyword + tipe data)
- [ ] Referensi standard library (semua modul + fungsi)
- [ ] 10+ contoh program (dari "Hello World" sampai FizzBuzz, fibonnaci, file I/O)
- [ ] CHANGELOG per rilis

## 9. Rilis

- [ ] Semua item di atas terpenuhi
- [ ] `cargo test --workspace` hijau
- [ ] `cargo clippy --workspace -- -D warnings` bersih
- [ ] Setup.exe terverifikasi di komputer bersih (install → jalankan → uninstall)
- [ ] Tag `v1.0` + GitHub Release + SHA256SUMS

---

**Catatan**: Item yang belum dicentang (`[ ]`) harus diselesaikan sebelum `v1.0`.
Item yang sudah dicentang (`[x]`) adalah fondasi yang sudah ada.
