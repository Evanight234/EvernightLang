# AGENTS.md — Proyek Bahasa Pemrograman Indonesia

## ATURAN WAJIB UNTUK OPENCODE

**Setiap kali menerima prompt baru, WAJIB lakukan ini sebelum menjawab/mengerjakan:**

0. **BACA KONTEKS DULU** — Di sesi baru/sesi hilang, baca `KONTEKS.md` **paling awal** (sebelum file lain) supaya tahu topik yang sedang/sudah dibahas dan tidak ada yang terlewat.
1. **CEK** — Baca ulang file `RENCANA.md`, `PROGRES.md`, dan `KONSEP.md` di root proyek.
2. **UPDATE KONTEKS** — Setiap prompt: perbarui bagian "Konteks Aktif" + log di `KONTEKS.md` (topik yang dibahas, keputusan kecil, langkah berikutnya).
3. **SESUAIKAN** — Pastikan pekerjaan yang diminta user sesuai dengan fase & arah proyek di `RENCANA.md`. Jika user minta sesuatu yang berubah arah, perbarui `RENCANA.md` dulu (dengan konfirmasi user) sebelum mulai.
4. **CATAT** — Setiap kali menyelesaikan satu task, langsung update `PROGRES.md`: centang task selesai (`[x]`), tambah catatan keputusan penting, tambah tanggal.
5. **PERBARUI** — Jika ada keputusan desain baru (mis. keyword ditambah/diubah), tulis ke bagian yang relevan di `RENCANA.md`.

Jangan pernah mengerjakan tanpa membaca `RENCANA.md` dulu. Jangan pernah lupa mengupdate `PROGRES.md` setelah ada kemajuan.

---

## RINGKASAN PROYEK

Bahasa pemrograman baru dengan karakteristik:

| Aspek | Keputusan |
|-------|-----------|
| Tujuan | Se-mudah Python, se-cepat C++, se-multifungsi JavaScript |
| Sintaksis | Mirip Python, tapi **keyword Bahasa Indonesia** |
| Paradigma | Fungsional + OOP (detail di `KONSEP.md`) |
| Tipe | Dinamis (seperti Python/JS) |
| Eksekusi | Compiler bytecode + VM |
| Implementasi | Rust murni + TypeScript (khusus ekstensi editor) |
| Error | Error code + pesan BAHAYA/PERINGATAN dalam Bahasa Indonesia |
| Nama bahasa | **EvernightLanguage** (file: `.eve`) |
| Biner / CLI | `evernight` (berkas `.eve`); package manager `ever pkg` (Fase 6) |
| Asosiasi file | `.eve` → **"Evernight files"** di Windows Explorer (ProgID `EvernightFile`, installer Fase 6) |
| Manajemen memori | ARC (Automatic Reference Counting) — dipilih karena sederhana & modern |

---

## RENCANA.md — Fase-Fase

### Fase 1: Desain Bahasa
- [x] Definisikan keyword Bahasa Indonesia (jika/lainnya/selama/untuk/kembali/cetak/benar/salah/kosong/fungsi/variabel/tetap/_/hapus)
- [x] Buat grammar formal (BNF/EBNF)
- [x] Desain tipe data
- [x] Desain sistem error (BAHAYA/PERINGATAN dalam Bahasa Indonesia)
- [x] Desain standard library

### Fase 2: Implementasi Inti
- [x] Setup proyek Rust (Cargo Workspace)
- [x] Lexer (tokenizer)
- [x] Parser (AST generator)
- [x] AST node types
- [x] Bytecode compiler (AST → bytecode)
- [x] VM (bytecode executor)
- [x] Environment (scope variabel)
- [x] Runtime objects
- [x] Error handling system
- [x] Implementasi murni Rust — C dibatalkan permanen

### Fase 3: Standard Library + Identitas Visual
- [x] Modul konsol, string, matematika, daftar, kamus, sistem, utilitas (7 modul, sinkron `STDLIB.md`)
- [x] Sistem `impor` lintas berkas (lazy-load, cache, deteksi circular)
- [x] Kode error per modul + unit test tiap modul — sinkron `ERROR.md`, 58/58 test hijau
- [x] Brief & spesifikasi logo resmi (`LOGO.md` — user yang desain)
- [x] Desain logo oleh user → review → finalisasi (badge gaya JS + huruf E, `assets/logo/`, user setujui)
- [x] Icon file `.eve` (16/32/48/256px + `.ico`)
- [x] Brand kit mini (warna brand, slogan, font, lisensi) — final `BRAND.md` (slogan user 2026-09-10)

### Fase 4: REPL & CLI + Highlighting Editor
- [x] REPL interaktif + CLI runner (termasuk `evernight run`) + argumen program + error terminal berwarna
- [x] Pemetaan token→scope + TextMate grammar (`evernight.tmLanguage.json` dari `token.rs`)
- [x] Tema warna unik Indonesia "Nusantara" (`TEMA.md`, palet terang/gelap)
- [x] Ekstensi VS Code + snippet + snapshot test — satu `.vsix` untuk **keluarga VS Code** (VS Code/Cursor/Windsurf/VSCodium; Antigravity dicek saat eksekusi) + icon theme `.eve`

### Fase 5: Testing & Kualitas Internal
- [x] Unit test per-modul + integration/golden test di semua crate
- [x] Fuzz test parser + benchmark baseline
- [x] `CHANGELOG.md` + versi semantik + laporan cakupan test

### Fase 6: Tooling Lanjutan & Distribusi Sistem
- [x] Static CompletionProvider (VS Code) — autocomplete statis ~60 item
- [x] Paket ekstensi `.vsix` (6A) — publisher **Satriyo**, lisensi **MIT**, terpasang & terverifikasi di **Antigravity IDE**
- [x] Distribusi compiler mandiri (6B) — biner statis 1.74 MB, paket `paket/evernight-0.1.0-windows-x64/` + ZIP, `install.ps1`/`uninstall.ps1`, PATH otomatis, asosiasi `.eve` terverifikasi
- [x] Integrasi otomatis ekstensi (6C) — deteksi 6 editor keluarga VS Code, konfirmasi interaktif, `-TanpaEkstensi`/`-TanpaKonfirmasi`, uninstall menawarkan pencopotan
- [x] **Installer GUI (6F)** — `EvernightLanguage-0.1.0-Setup.exe` (egui/Rust, **bukan** Inno Setup), window **1000x640**, **satu box per halaman** (isi box hanya judul+keterangan), **pemilih folder native (rfd) + tombol Telusuri**, **tanpa CMD** (`windows_subsystem` + `CREATE_NO_WINDOW`), pilihan per-user/per-machine (UAC), wizard 6 halaman, panel maskot 28%, animasi mikro, `uninstall.exe` bersih
- [x] Mode debug (6D-1) — `--debug` trace instruksi bytecode ke stderr
- [x] Profiler mini (6D-2) — `--waktu` frekuensi opcode + waktu eksekusi
- [x] Formatter (6D-3) — `evernight format` + `--cek` (komentar dipertahankan; idempoten)
- [x] Linter (6D-4) — `evernight lint` + 7 aturan `WK*` baru
- [x] Package manager (6D-6) — `evernight pkg init | jalankan | daftar`
- [x] CI & rilis biner (6E) — GitHub Actions + `SHA256SUMS.txt`
- [ ] Language Server dasar (`evernight-lsp`) — **dipindah ke Fase 7**
- [ ] Publish `crates.io` + repositori GitHub (remote + push) + optimasi performa runtime

### Fase 7: Stabilisasi & Rilis 1.0
- [ ] Audit keamanan dasar (akses file, sandbox)
- [ ] Stabilisasi versi bytecode (kompatibilitas antar-rilis) + kriteria "selesai" rilis 1.0
- [ ] **7E — Desain visual final installer** (grafis wizard bermerek Nusantara, UX hak akses, uji di komputer bersih)

### Fase 8: Dokumentasi Publik, Website & Komunitas
- [ ] README utama publik, panduan sintaks, tutorial, referensi stdlib, 10+ contoh program
- [ ] Website resmi + playground WASM + publish ekstensi VS Code ke Marketplace
- [ ] `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, template issue/PR, peta jalan, galeri komunitas, tantangan pertama
- [ ] Pengumuman rilis 1.0, survei pengguna, jadwal pemeliharaan

---

## CONTOH SINTAKSIS TARGET

```
fungsi faktorial(n) {
    jika n == 0 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

fungsi utama() {
    cetak("Hasil: ", faktorial(5))
}
```

Pesan error:
```
BAHAYA: Baris 5 - Pembagian dengan nol!
PERINGATAN: Baris 8 - Variabel 'x' tidak didefinisikan
```

---

## KEPUTUSAN PENTING (log)

- 2026-09-10: **Nama biner resmi `evernight`** (keputusan user). Asosiasi file Windows: `.eve` → ProGID `EvernightFile`, kolom Type Explorer = **"Evernight files"** (spek di RENCANA Fase 6 Addendum).

- 2026-09-07: Proyek dimulai. Fase 1 di awal. Struktur awal: Cargo.toml + src/ (masih kosong).
- 2026-09-07: Model eksekusi final: **compiler bytecode + VM** (`.eve` → AST → bytecode → VM).
- 2026-09-10: **Rencana diperluas dari 5 menjadi 7 fase.** Fase 3 + Identitas Visual (logo, icon `.eve`, brand kit); Fase 4 + Highlighting Editor (tema "Nusantara", grammar TextMate dari `token.rs`, ekstensi VS Code); Fase 6 & 7 baru (Tooling/Distribusi, Komunitas/Rilis). File MD baru: `LOGO.md`, `TEMA.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.
