# RENCANA.md — Rencana Lengkap EvernightLanguage

> File ini adalah sumber kebenaran (source of truth) untuk arah proyek.
> Baca file ini SETIAP kali sebelum mengerjakan apa pun.

## Identitas Bahasa

- **Nama**: EvernightLanguage
- **Ekstensi file**: `.eve` (contoh: `program.eve`, `utama.eve`)

## Tujuan Bahasa

- Se-mudah Python, se-cepat C++, se-multifungsi JavaScript
- Sintaksis mirip Python, keyword Bahasa Indonesia
- Fungsional + OOP, tipe dinamis, compiler bytecode + VM
- Implementasi: Rust murni (runtime) + TypeScript (hanya ekstensi editor, Fase 4)
- Biner/CLI resmi: **`evernight`** (`evernight <berkas.eve>`; utk paket `ever pkg`) — keputusan 2026-09-10

## Contoh Sintaksis Target

```eve
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

## Keyword Bahasa Indonesia (final)

| English | Indonesia |
|---------|-----------|
| if / else if / else | jika / lainnya_jika / lainnya |
| while / for / in | selama / untuk / dalam |
| break / continue | berhenti / lanjut |
| return | kembali |
| print / read | cetak / baca |
| true / false / null | benar / salah / kosong |
| fn / func | fungsi |
| let / const | variabel / tetap |
| import / from / as | impor / dari / sebagai |
| try / catch / finally / throw / assert | coba / tangkap / akhirnya / lempar / pastikan |
| match / case / default | cocok / kasus / bawaan |
| wildcard / del | _ / hapus (Fase 3 — bersama modul koleksi) |

## Sistem Error

```
BAHAYA: Baris 5 - Pembagian dengan nol!
PERINGATAN: Baris 8 - Variabel 'x' tidak didefinisikan
```

---

## Fase 1: Desain Bahasa
- [x] Definisikan keyword Bahasa Indonesia
- [x] Buat grammar formal (BNF/EBNF)
- [x] Desain tipe data
- [x] Desain sistem error
- [x] Desain standard library

## Fase 2: Implementasi Inti
- [x] Setup proyek Rust
- [x] Lexer (tokenizer)
- [x] Parser (AST generator)
- [x] AST node types
- [x] Bytecode compiler (AST → bytecode)
- [x] VM (bytecode executor)
- [x] Environment (scope variabel)
- [x] Runtime objects
- [x] Error handling system
- [x] Implementasi murni Rust — C dibatalkan permanen (lihat Log Keputusan)

## Fase 3: Standard Library + Identitas Visual
- [x] Modul konsol (cetak/baca) — finalisasi builtin vs `impor konsol`
- [x] Modul string (besar/kecil/bersih/potong/gabung/pisah/ganti/cari)
- [x] Modul matematika (akar/pangkat/bulat_bawah/bulat_atas/pembulatan/bundar/mutlak/abs/acak_antara/log/faktorial/min/max/sin/cos/tan/pi/e)
- [x] Modul daftar (tambah/sisip/hapus/urutkan/balik/unik/jumlah/rata_rata/gabung_larik/iris/cari/ada/peta/saring/lipat/setiap)
- [x] Modul kamus (kunci/nilai/pasangan/ada_kunci/hapus_kunci/dapatkan/setel/gabung_objek)
- [x] Modul sistem (baca_file/tulis_file/ada_file/env(nama,bawaan?)/atur_env/waktu_sekarang/tanggal_sekarang(ts?)/format_tanggal/tunda/selisih_waktu)
- [x] Modul utilitas (adalah_angka/teks/daftar/kamus + ke_boolean/e_boolean/bolean + ke_larik/daftar + salin + angka/teks/kamus) — sinkron `STDLIB.md` (7 kategori)
- [x] Keyword `hapus` (del) — statement `hapus a[i]` / `hapus k["x"]` / `hapus k.nama` (indeks daftar atau kunci kamus)
- [x] Sistem `impor` lintas berkas (lazy-load, cache modul, deteksi circular) — v1: `impor "modul" [sebagai alias]`, ekspor sebagai kamus namespace; `dari` ditunda
- [x] Kode error per modul stdlib — sinkron `ERROR.md` (FUNGSI/MATH/JUMLAH/RUNTIME/ASSERT ditambah; TYPE dirapikan; STACK/MEMORY bertanda rencana)
- [x] Unit test tiap modul stdlib
- [x] Brief & spesifikasi logo resmi (konsep, makna, varian terang/gelap, SVG/PNG) — `LOGO.md` (arah terisi: badge gaya JS + huruf E)
- [x] Desain logo oleh user → review → finalisasi — user kirim `logo1.png`, diproses (`assets/logo/`); user setujui 2026-09-10; master SVG menyusul (utang teknis)
- [x] Icon file `.eve` (16/32/48/256px + `.ico` untuk editor & asosiasi file) — siap di `assets/logo/`
- [x] Brand kit mini (warna brand, slogan, font, lisensi) — final di `BRAND.md` (slogan user 2026-09-10)
- [x] Pasang logo ke README/dokumen (placeholder sampai final) — `README.md` memakai logo

## Fase 4: REPL & CLI + Highlighting Editor
- [x] REPL interaktif (`repl.rs`: prompt `eve>`, deteksi blok multi-baris, perintah `:bantuan`/`:keluar`/`:muat`/`:bersihkan`, `run_incremental`)
- [x] CLI runner (`--run`, `run <berkas.eve>`, `--cek`, `--tokens`, `--ast`, `--bytecode`)
- [x] **`evernight coba.eve` — default-run**: tanpa flag langsung menjalankan (`--run` dan subcommand `run` tetap diterima)
- [x] Subcommand eksplisit: `evernight run <berkas.eve>` (dengan pesan error bila berkas tidak disertakan)
- [x] Biner global / PATH lokal: `cargo install --path crates/evernight_cli` didukung; panduan PATH manual didokumentasikan di README
- [x] Argumen program (`argumen()`, id 75, pemisah `--`, kode keluar)
- [x] Error terminal berwarna + cuplikan baris kode (BAHAYA merah, PERINGATAN kuning, caret `^`, `NO_COLOR`/`--tanpa-warna`)
- [x] `--versi` dan `--bantuan` yang rapi
- [x] Pemetaan token→scope (keyword, string, angka, komentar, fungsi, operator, error) — `TEMA.md` §2
- [x] TextMate grammar `editors/vscode/syntaxes/evernight.tmLanguage.json` — satu sumber kebenaran dari `token.rs`
- [x] Tema warna unik Indonesia "Nusantara" (palet gelap `nusantara-dark.json`) — `TEMA.md` §3.1
- [x] Varian tema terang `nusantara-light.json` — `TEMA.md` §3.2
- [x] Ekstensi VS Code (language id `evernight`, grammar, icon file `.eve`, autocomplete statis ~60 item) — `.vsix` tunggal kompatibel **keluarga VS Code**
- [x] Icon theme `evernight` (bagian `.vsix` yang sama) — `*.eve` tampil ber-icon E di explorer (`icons/evernight-icon-theme.json`)
- [x] Snippet dasar (`fungsi/jika/untuk/coba/cocok` dll, 23 snippet di `snippets/eve.json`)
- [x] Snapshot test highlighting (contoh `test/snapshots/highlight.eve` → scope token di `highlight.eve.scope`)
- [x] Dokumen cara pasang ekstensi lokal (`editors/vscode/README.md`) — panduan F5 Development Host & paket `.vsix`

## Fase 5: Testing & Kualitas Internal (DEV)
- [x] Unit test per-modul (memecah `vm_tests.rs` menjadi 9 berkas `tests/test_*.rs` + `common/mod.rs`)
- [x] Integration test end-to-end + golden test (`tests/golden/` 6 kasus + runner `golden_tests.rs` via library API & `Vm` output capture)
- [x] Test tiap kode error `BAHAYA` dan `PERINGATAN` (`error_codes_test.rs`: DIVISION/TYPE/INDEX/KEY/FUNGSI/VARIABLE/MATH/JUMLAH/ASSERT/RUNTIME/SYNTAX + WKVAR/WKREACH)
- [x] Fuzz test parser (`fuzz_parser.rs`: 15.000 iterasi deterministik anti-panic + batas rekursi `MAX_RECURSION_DEPTH = 64`)
- [x] Benchmark dasar (`benches/benchmark.rs` zero-dep `std::time::Instant`)
- [x] Laporan cakupan test (`TESTING.md`: panduan `cargo-llvm-cov` / `cargo-tarpaulin` + hasil benchmark)
- [x] `CHANGELOG.md` + aturan versi semantik (semver) — versi `[0.1.0]` terisi

## Fase 6: Tooling Lanjutan & Distribusi Sistem (DEV)
- [x] Static CompletionProvider (VS Code) — autocomplete statis (~60 item: keyword/builtin/modul)
- [ ] Language Server dasar (hover, diagnostic, goto-definition, completion) — **dipindah ke Fase 7**
- [x] Formatter (`evernight format`)
- [x] Linter (aturan gaya + warning `WK*` baru)
- [x] Mode debug (`--debug` trace bytecode; DAP menyusul)
- [x] Profiler mini (`--waktu`, hitung opcode)
- [x] Package manager lokal (`ever pkg`: init/jalankan/daftar)
- [x] **Distribusi Compiler & Bundling ke Sistem**:
  - Pengguna akhir **tidak perlu memasang Rust / Cargo** sama sekali; biner dibundel mandiri.
  - **Jalur A (Portabel ZIP)**: Arsip `.zip` berisi `evernight.exe`, `icon.ico`, `PANDUAN.txt` (tanpa butuh hak admin).
  - **Jalur B (Skrip Installer per-user)**: Skrip PowerShell (`install.ps1`/`uninstall.ps1`) menyalin ke `%LocalAppData%\Programs\Evernight\bin`, otomatis mendaftarkan ke PATH `User`, serta mengonfigurasi asosiasi registry berkas `.eve` (`HKCU\Software\Classes`).
  - **Jalur C (Installer GUI, egui/Rust)**: `EvernightLanguage-0.1.0-Setup.exe` — installer mandiri dengan opsi per-user / per-machine untuk rilis publik. *Inno Setup ditinggalkan (keputusan 2026-09-14) karena berbasis Delphi VCL dan tidak mendukung animasi halus.*
  - Struktur folder terpasang:
    ```
    Evernight/
    ├── bin/
    │   ├── evernight.exe           # Biner compiler + VM + REPL + runner
    │   ├── kill.cmd                # `kill evernight system` (CMD)
    │   └── kill.ps1                # `.\kill.ps1` (PowerShell)
    ├── assets/
    │   ├── icon.ico                # Ikon resmi berkas .eve di Explorer
    │   └── logo.png
    ├── docs/
    │   └── PANDUAN.txt
    └── uninstall.exe               # Uninstaller GUI (3 tahap wizard)
    ```
  - **Otomatisasi PATH Sistem**: Pengguna cukup buka terminal baru dan langsung dapat menjalankan `evernight run berkas.eve`, `evernight berkas.eve`, atau `evernight`.
  - **Asosiasi Berkas Windows Instan**: Klik dua kali pada berkas `.eve` langsung mengeksekusi program melalui `evernight.exe "%1"`.
- [ ] Integrasi editor opt-out (installer default mengintegrasikan keluarga VS Code, pengguna bisa tolak via checkbox)
- [ ] Publish `crates.io`
- [ ] CI GitHub Actions (build + test + clippy tiap push)
- [ ] Binary rilis (`--release`, checksum, catatan rilis)
- [ ] **Optimasi Performa Runtime**: Optimasi bytecode VM (threaded code, inline caching, pemangkasan instruksi hot path) untuk menjamin eksekusi cepat.

### Sub-Fase Pelaksanaan Fase 6 (urutan eksekusi)

- **6A — Paket Ekstensi Siap Distribusi** ✅ *(selesai 2026-09-14)*
  - [x] `LICENSE` MIT (Satriyo) di root + salinan ekstensi
  - [x] Lengkapi `package.json` (`publisher: Satriyo`, `license: MIT`, `icon`, `repository`, `keywords`)
  - [x] `.vscodeignore` (kecualikan `src/`, `test/`, `node_modules/`, `*.ts`, maps)
  - [x] Ikon ekstensi `editors/vscode/icons/evernight-icon.png`
  - [x] Build `evernight-language-0.1.0.vsix` (via `vsce package --allow-missing-repository`)
  - [x] **Terpasang & terverifikasi di Antigravity IDE** (`satriyo.evernight-language-0.1.0`)
  - [x] README ekstensi diperbarui (langkah Antigravity IDE + aktivasi tema)
- **6B — Distribusi Compiler Mandiri** ✅ *(selesai 2026-09-14)*
  - [x] `cargo build --release` → `evernight.exe` **1.74 MB statis** (tanpa DLL MinGW; hanya `msvcrt.dll`)
  - [x] Paket portabel `paket/evernight-0.1.0-windows-x64/`: `bin/`, `assets/`, `docs/`, `extensions/` (bundel `.vsix`)
  - [x] `install.ps1` / `install.cmd`: salin ke `%LocalAppData%\Programs\Evernight\`, PATH user (idempoten), asosiasi `.eve` (HKCU), `-Uji` dry-run, `-TanpaAsosiasi`
  - [x] `uninstall.ps1` / `uninstall.cmd`: cabut registry, PATH, folder (trik hapus tertunda bila terkunci)
  - [x] **Uji nyata terverifikasi**: PATH 31→32, `.eve`→`EvernightFile` (label "Evernight files"), klik-ganda `.eve` berhasil menjalankan program, install 2× tetap idempoten, uninstall kembali bersih ke 31 entri
  - [x] ZIP distribusi: `paket/evernight-0.1.0-windows-x64.zip` (~691 KB)
- **6C — Integrasi Otomatis Ekstensi saat Install** ✅ *(selesai 2026-09-14)*
  - [x] Fungsi `Cari-Editor` mendeteksi 6 editor keluarga VS Code (Antigravity, VS Code, VS Code Insiders, Cursor, Windsurf, VSCodium) via CLI/PATH + lokasi instalasi standar
  - [x] Alur **konfirmasi interaktif**: `Pasang ekstensi EvernightLanguage ke editor di atas? (y/t)`
  - [x] Menjalankan `<editor-cli> --install-extension <vsix> --force`; kegagalan tidak menggagalkan instalasi
  - [x] Opsi: `-TanpaEkstensi` (lewati) & `-TanpaKonfirmasi` (tanpa tanya, untuk otomatisasi/6F)
  - [x] `uninstall.ps1`: menawarkan pencopotan ekstensi (`y/t`), opsi `-SimpanEkstensi` & `-TanpaKonfirmasi`
  - [x] `docs/PANDUAN.txt` & `BACA-AKU.txt` diperbarui (pemasangan ekstensi semi-otomatis; manual jadi fallback)
  - [x] **Uji nyata terverifikasi**: deteksi hanya Antigravity (Cursor/Windsurf dilewati meski folder data ada), prompt muncul, `-TanpaEkstensi` melewati, uninstall `-SimpanEkstensi` mempertahankan ekstensi, reguler PATH/registry tetap bersih
  - [x] **Perbaikan bug**: stderr peringatan CLI (`antigravityAnalytics`) tidak lagi dianggap galat fatal (longgarkan `ErrorActionPreference` di sekitar pemanggilan CLI)
- **6D — Tooling Tambahan** ✅ *(selesai 2026-09-16)*
  - [x] **6D-1 Mode debug** — `evernight <berkas> --debug` mencetak trace tiap instruksi bytecode ke `stderr` (`[offset] baris N | NamaOpcode`); `OpCode::name()` sebagai sumber nama; `Vm::debug_trace`
  - [x] **6D-2 Profiler mini** — `evernight <berkas> --waktu` mencetak waktu eksekusi (ms), total instruksi, dan tabel frekuensi opcode terurut + persentase; `Vm::profile_mode` + `Vm::profile_summary()`; counter `[usize; 256]` (zero-dep)
  - [x] **6D-3 Formatter** — crate `evernight_fmt` (berbasis token, komentar dipertahankan, idempoten); `evernight format <berkas>` + `--cek` (dry-run CI) + `--keluar <path>`; 14 test
  - [x] **6D-4 Linter** — crate `evernight_lint` (analisis AST); 7 aturan baru (`WKHURUF`, `WKIMPOR`, `WKPANJANG`, `WKPARAM`, `WKSARANG`, `WKMATI`, `WKMAGIS`) digabung dengan warning compiler; `evernight lint <berkas>`; 11 test
  - [x] **6D-6 Package manager minimal** — `evernight pkg init | jalankan | daftar`; parser JSON zero-dep (`json.rs`); manifest `eve.json`; 7 test
  - **Catatan**: LSP **dipindah ke Fase 7** (keputusan user 2026-09-14) — tidak menghambat rilis; ekstensi sudah punya static completion dari Fase 3.
  - **Perbaikan bug sampingan**: **BOM UTF-8** kini diabaikan lexer & dipertahankan formatter (berkas dari Notepad/PowerShell sebelumnya gagal di-tokenisasi); bantuan global tidak lagi menelan `evernight pkg --bantuan`.
- **6E — CI & Rilis Biner** ✅ *(selesai 2026-09-16)*
  - [x] `git init` + `.gitignore` + `.gitattributes` (normalisasi LF; biner ditandai; `.ps1`/`.cmd` CRLF)
  - [x] `.github/workflows/ci.yml` — `fmt --check` + `clippy -D warnings` + `test --workspace` + verifikasi contoh `.eve` valid & rapi
  - [x] `.github/workflows/rilis.yml` — build `x86_64-pc-windows-gnu` statis, susun paket, ZIP, `SHA256SUMS.txt`, unggah artefak, GitHub Release (draft)
  - [x] `.cargo/config.toml` diubah dari `[build]` ke `[target.x86_64-pc-windows-gnu]` agar `+crt-static` tidak merusak target lain di CI
  - [x] 12 contoh `.eve` + 3 golden dirapikan formatter agar lolos pemeriksaan CI
  - **Catatan**: repositori GitHub belum dibuat (perlu `git remote add` + push oleh user).
- **6F — Installer GUI Windows (egui, Rust)** ✅ *(selesai 2026-09-16)*
  - **Tool**: **egui/eframe 0.36** (Rust) — **bukan Inno Setup** (keputusan user 2026-09-14).
  - **Detail desain lengkap**: lihat **[Addendum Akhir 6D — Desain Installer Evernight](#addendum-akhir-6d--desain-installer-evernight)**.
  - [x] Crate terpisah `installer/` (di luar workspace, profil build sendiri)
  - [x] Panel maskot **28% lebar penuh atas-bawah**; cutout otomatis dari JPEG referensi user
  - [x] Palet dari gambar referensi user (plum/rose); font Plus Jakarta Sans & JetBrains Mono (OFL, dibundel)
  - [x] Bilah judul kustom (ikon + judul + minimize/close, jendela dapat diseret)
  - [x] **Window 1000 × 640** (minimum 880 × 580) — diperlebar dari 760 × 520 agar tidak ada UI terpotong
  - [x] **Satu box per halaman**; isi box HANYA judul + keterangan (teks). Semua kendali interaktif (radio, checkbox, input, progress, log) DI LUAR box
  - [x] **Pemilih folder** dengan tombol **Telusuri...** → dialog folder native Windows (`rfd` 0.17)
  - [x] **Tanpa jendela CMD** — `windows_subsystem = "windows"` saat rilis + `CREATE_NO_WINDOW` pada semua program luar (`reg`, `powershell`, `net`, CLI editor)
  - [x] 6 halaman wizard: Selamat datang → Lisensi → Lokasi tujuan → Siap pasang → Memasang → Selesai
  - [x] **Animasi**: hover tombol 150 ms, tekan 80 ms (mengecil), riak klik 400 ms, progress mengalir, denyut langkah aktif 1,6 s, transisi halaman fade+geser 250 ms
  - [x] Logika pemasangan: salin berkas, PATH, asosiasi `.eve`, deteksi 6 editor + pasang `.vsix`, pintasan Start Menu, registri *Apps & Features*
  - [x] Mode per-user (tanpa UAC) & per-machine (elevasi UAC via relaunch)
  - [x] **Uninstaller** (`uninstall.exe`, salinan installer) — mencabut PATH/registry/folder/ekstensi/pintasan; mengatasi kunci berkas dengan menjalankan diri dari folder temp
  - [x] **Uji nyata terverifikasi**: siklus install → 32 entri PATH + `.eve` aktif + Apps & Features + biner `format`/`lint`/`pkg` jalan; uninstall → kembali bersih ke 31 entri, folder terhapus penuh
  - [x] `installer/build-installer.ps1` (build → payload → installer → `Setup.exe` + SHA256) dengan **penjaga biner basi**
  - [x] Hasil: `installer/dist/EvernightLanguage-0.1.0-Setup.exe` (**8.35 MB** + `SHA256SUMS.txt`)
  - [x] **Pratinjau 6 halaman** untuk tinjauan desain: `installer/aset/tinjau/` + flag `--mulai N`
  - *Poles desain akhir + uji komputer/VM bersih: Fase 7E.*
- **6G — Tema Ikon Lengkap (Symbols + `.eve`) + Kompatibilitas 6 Editor** ✅ *(selesai 2026-09-19)*
  - [x] Tema gabungan dari `miguelsolorio.vscode-symbols` (MIT): 348 SVG `files/` + 104 SVG `folders/` + mapping `.eve` → `_eve` → `evernight-file.png`
  - [x] `package.json`: label tema → **IconStyles** (id tetap `evernight-icons`, path `./icons/evernight-icon-theme.json`)
  - [x] `build-vsix.ps1` dirombak total: patch vsix dasar dengan SEMUA berkas ikon rekursif (bukan 3 entri hardcoded)
  - [x] VSIX baru: **1.264.065 byte**, 720 entri, 352 SVG, tema 98.686 B
  - [x] **Verifikasi lolos**: semua target SVG ada (0 hilang), `.eve`→`_eve` mapping OK, label IconStyles OK
  - [x] **VSIX kompatibel 6 editor**: VS Code, Cursor, Windsurf, VSCodium, Antigravity IDE, Theia — struktur standar, API universal (TextMate, iconThemes, snippets, CompletionProvider), tanpa native module/node_modules
  - [x] Terpasang di Antigravity via CLI (`--install-extension --force`) → **semua ikon kelihatan**
- **6H — Sistem Uninstaller** ✅
  - 3 tahap wizard: **Konfirmasi** (judul "Hapus EvernightLanguage?" + peringatan tak-bisa-batal + daftar komponen yang dicabut dalam box; checkbox "Hapus juga ekstensi editor" di luar box → tombol Kembali + Copot) → **Menghapus** (progress bar + log nyata dari `pasang::copot(hapus_ekstensi, ...)`) → **Selesai** (ringkasan dalam box + tombol Tutup)
  - Navigasi back/forward berfungsi di mode copot (3 langkah: Konfirmasi/Menghapus/Selesai; panel tahapan pakai `LANGKAH_COPOT`)
  - `pasang::copot()` menerima parameter `hapus_ekstensi: bool` — bila false, skip copot ekstensi + reset iconTheme
  - Perintah terminal cepat: `kill evernight System` (CMD via `kill.cmd` di bin/; PowerShell via function di `$PROFILE` dengan `Remove-Item alias:kill` untuk override alias bawaan) — di-embed di installer & dicopy ke `bin/` otomatis saat install; function didaftarkan ke `$PROFILE` otomatis
  - Fondasi lama: `Mode::Copot` + self-copy `--dari-temp` + `pindah_ke_temp()` tetap dipertahankan
- **6I — Sistem Updater (CLI saja, tanpa desain visual)** ✅
  - Perintah: `update Evernight system` (nama kerja dari user) — murni terminal CMD/PowerShell, tanpa GUI/desain installer
  - Sumber: folder `version/` di GitHub (`version/version` = manifes teks satu baris; `version/catatan-<versi>.txt` = deskripsi isi versi; file update + installer masuk folder yang sama)
  - Tanpa update: `Istriku lagi sibuk jangan dingagu` | ada update: tampilkan catatan perubahan dulu, lalu pasang + `terupdate ke versi <versi>, jangan panggil istriku lagi dasar karbit` (verbatim user)
  - Scope: exe + vsix + reinstall ekstensi best-effort; exe terkunci → penimpaan terjadwal; flag `--cek` (dry-run)
  - `evernight system info`: versi + lokasi + ukuran exe (registri) + isi versi (live GitHub, fallback offline)
  - Repo: `Evanight234/EvernightLang` (sudah push; updater live)
- **Sinkronisasi distribusi vsix baru (wajib sebelum rilis ke orang lain)** ✅ *(selesai 2026-09-19)*
  1. Salin vsix 1.264.065 B → `installer/aset/payload/` + `paket/.../extensions/` (timpa 20.797 B)
  2. Regenerasi ZIP paket portabel (±membesar 1,3 MB)
  3. Rebuild `Setup.exe` via `installer/build-installer.ps1`
  4. Verifikasi SHA256 + ukuran + payload; smoke test install→uninstall opsional
  5. Catat SHA/ukuran baru di KONTEKS.md/PROGRES.md

### Addendum Akhir 6D — Desain Installer Evernight

> Keputusan user 2026-09-14. Ini spesifikasi desain resmi installer GUI (6F). Sumber palet: gambar referensi user `assets/evernight instaler models.jpeg` (dieksraksi programatik via PIL). Struktur wizard: adaptasi dari desain Stitch "Desktop Setup Wizard Interface".

### 1. Keputusan arsitektur

| Aspek | Keputusan | Alasan |
|-------|-----------|--------|
| Framework | **egui/eframe (Rust)** | Inno Setup berbasis Delphi VCL → tidak bisa animasi halus, tombol berwarna, transisi halaman |
| Lokasi crate | `installer/` — **di luar workspace** | `.cargo/config.toml` memaksa `+crt-static` + `rust-lld` untuk seluruh workspace; kombinasi ini rapuh dengan `winit`/`eframe`. Compiler inti tetap zero-dep & statis |
| Pengecualian prinsip | **Disadari**: installer memakai dependensi eksternal | Durasi: `evernight_core`/`_vm`/`_cli` tetap **zero-dep**. Pengecualian hanya untuk biner installer |
| Toolchain | **gnu** dulu; fallback `msvc` (`evn-msvc` sudah tersedia) bila `eframe` gagal build | Konsistensi dengan toolchain proyek |
| Judul jendela | **"EvernightLanguage Setup"** | Keputusan user |

### 2. Palet warna (dari gambar referensi user)

| Peran | Hex | Pemakaian |
|-------|-----|-----------|
| Latar jendela | `#37313D` | Latar utama wizard |
| Well / dalam | `#2F2430` | Area lebih dalam (log, progress track) |
| Kartu | `#443C4A` | Kartu konten, panel konten |
| Border | `#7A6074` | Hairline divider |
| Teks utama | `#F8DCE8` | Judul & isi |
| Teks sekunder | `#B49AAB` | Keterangan, label |
| **Aksen** | `#D3A0B9` | Tombol utama, progress fill, langkah aktif |
| Aksen hover | `#DBB8CA` | Keadaan hover |
| Wine / bahaya | `#733543` | Error, peringatan |

**Kontras terverifikasi (WCAG AA lolos):** teks utama di latar ≈ **10:1** · aksen di latar ≈ **5.5:1**.

### 3. Tipografi

| Peran | Font | Sumber |
|-------|------|--------|
| UI | **Plus Jakarta Sans** (400/500/600/700) | Google Fonts, OFL |
| Kode / path berkas | **JetBrains Mono** | Google Fonts, OFL |

### 4. Tata letak jendela

```
Jendela: 1000 × 640 px (minimum 880 × 580)

┌──────────────────────────────────────────────────────────────────┐
│ [ikon]  EvernightLanguage Setup            ─   ✕   (bilah judul)  │
├─────────────────┬────────────────────────────────────────────────┤
│                 │  TAHAPAN PEMASANGAN                            │
│   MASKOT        │  ① Selamat datang ② Lisensi ③ Lokasi Tujuan    │
│   panel         │  ④ Siap pasang ⑤ Memasang ⑥ Selesai            │
│   28% lebar     │  ───────────────────────────────────────────   │
│   PENUH         │  ┌──────────────────────────────────────────┐  │
│   atas–bawah    │  │ Box: HANYA judul + keterangan (teks)     │  │
│                 │  └──────────────────────────────────────────┘  │
│  badge versi    │  Kendali interaktif DI LUAR box:               │
│  di bawah       │  ( ) radio  ☑ checkbox  [path] [Telusuri...]   │
│                 │  ▓▓▓▓▓▓▓░░░░ progress (halaman Memasang)       │
│                 │  EvernightLanguage © 2026    [Kembali][Lanjut] │
└─────────────────┴────────────────────────────────────────────────┘
```

**Ukuran window** (keputusan user 2026-09-16): **1000 × 640**, minimum **880 × 580**.
Diperlebar dari 760 × 520 karena beberapa UI terpotong. Panel maskot 28%
(≈280 px) menyisakan ≈720 px untuk konten (+32% dari sebelumnya).
Lebar kolom konten dibatasi `LEBAR_MAKS_KONTEN = 760` agar baris teks tetap
nyaman dibaca bila jendela diperbesar pengguna.

**Aturan penataan (keputusan user 2026-09-16):**
- **Di dalam box**: HANYA judul + keterangan berupa teks.
- **Di luar box**: semua kendali interaktif — radio cakupan, checkbox tugas,
  input folder + tombol **Telusuri...**, progress bar, dan log.

**Panel maskot (keputusan user):** lebar **28% dari jendela** (≈280 px), **penuh dari atas ke bawah** (640 px) — seperti "balok berdiri".

Perhitungan crop dari sumber 736×973 (rasio 0.76):
- Target panel rasio 280/640 = 0.44
- Skala berdasarkan tinggi → crop tengah
- Hasil: karakter mengisi penuh tinggi panel (gaya banner), sisi lengan sedikit terpotong
- Area atas (di atas kepala) = warna datar → tempat badge versi

**Pemilih folder:** kotak teks (dapat diketik) + tombol **Telusuri...** yang
membuka dialog folder native Windows lewat crate **`rfd`** 0.17
(`default-features = false` agar tidak menarik backend GTK/X11).

**Tanpa jendela CMD:** biner installer memakai
`#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]`, dan semua
pemanggilan program luar (`reg`, `powershell`, `net`, CLI editor) memakai
`CREATE_NO_WINDOW` — sehingga tidak ada CMD yang muncul atau berkedip.

**Cutout maskot:** flood-fill dari tepi, latar `#37313D` (sudut-sudut gambar seragam ✅), toleransi warna + anti-alias tepi → `installer/aset/maskot.png` transparan.

### 5. Animasi (micro-interaction tombol — keputusan user)

Semua animasi diimplementasikan di `installer/src/anim.rs`, keadaan disimpan di `App`. **Wajib menghormati setelan Windows "Show animations"** (nonaktifkan bila user mematikan animasi OS).

| Efek | Implementasi |
|------|--------------|
| Hover tombol | Interpolasi warna 150 ms, easing cubic-out |
| Tombol tertekan | Skala 0.97 selama ~80 ms → kembali dengan spring kecil |
| Riak klik | Lingkaran memuai dari titik klik, fade 400 ms |
| Fokus keyboard | Cincin aksen; Tab/Enter/Space |
| Progress bar | Isian bergerak mulus ke target (bukan loncat) |
| Langkah aktif | Denyut halus opasitas 100%↔85%, periode 1.6 s |
| Transisi halaman | Fade + geser naik 8 px, 250 ms |

### 6. Halaman wizard (6 halaman)

| # | Halaman | Isi |
|---|---------|-----|
| 1 | Selamat Datang | Maskot + logo + sambutan Bahasa Indonesia + versi + slogan |
| 2 | Lisensi | MIT (pemegang hak: Satriyo), wajib centang setuju |
| 3 | Lokasi Tujuan | Per-user `%LocalAppData%` (tanpa UAC) / per-machine `%ProgramFiles%` (UAC); dapat diubah |
| 4 | Siap Pasang | Ringkasan + tugas tambahan (checkbox): PATH, asosiasi `.eve`, ekstensi editor, pintasan |
| 5 | Memasang | Progress + status per berkas + log (font mono) |
| 6 | Selesai | "Jalankan REPL" / "Buka Panduan" / buka folder instalasi |

### 7. Struktur folder

```
installer/                      ← crate terpisah, BUKAN anggota workspace
├── Cargo.toml                  # eframe + image
├── src/
│   ├── main.rs                 # entry; flag --uninstall, --diam (silent)
│   ├── app.rs                  # state wizard, routing 6 halaman
│   ├── tema.rs                 # ⭐ SEMUA warna/font/animasi — file milik user
│   ├── anim.rs                 # easing, transisi, riak tombol
│   ├── maskot.rs               # pemuatan & tata letak panel 28%
│   ├── ui/
│   │   ├── mod.rs
│   │   ├── komponen.rs         # tombol, progress, checklist, kartu
│   │   └── halaman.rs          # 6 halaman
│   └── pasang.rs               # salin, PATH, registry, ekstensi, uninstaller
└── aset/
    ├── maskot.png              # cutout transparan
    └── fonts/                  # Jakarta Sans, JetBrains Mono (OFL)
```

> **Prinsip**: `tema.rs` = satu-satunya berkas visual yang perlu disentuh user. Logika di `pasang.rs` tidak terpengaruh.

### 8. Logika pemasangan (pindahan dari `install.ps1` yang sudah terbukti)

- [ ] Salin `bin/`, `assets/`, `docs/`, `extensions/` ke folder tujuan
- [ ] Tambah `evernight.exe` ke PATH (user atau machine)
- [ ] Asosiasi `.eve` → "Evernight files" (HKCU atau HKLM)
- [ ] Deteksi 6 editor keluarga VS Code + tawarkan pemasangan `.vsix`
- [ ] Pintasan Start Menu / Desktop
- [ ] Elevasi UAC (per-machine) — manifest + relaunch diri bila perlu
- [ ] Registrasi *Apps & Features* (`DisplayName`, `DisplayVersion`, `Publisher`, `DisplayIcon`, `UninstallString`, `EstimatedSize`)
- [ ] Flag `--uninstall`: cabut PATH + registry + folder + entri Apps & Features

**Catatan penting**: registri *Apps & Features* per-user (HKCU) tidak tampil di semua versi Windows → **wajib diuji**.

### 9. Risiko & mitigasi

| Risiko | Mitigasi |
|--------|----------|
| `eframe` gagal build di toolchain gnu | Uji di hari pertama; fallback `msvc` |
| Ukuran biner installer membengkak | Target < 8 MB; `opt-level=z` + strip |
| UAC + registri bug senyap | Uji 2 mode × 3 skenario (install / pasang-ulang / uninstall) |
| Antivirus flag biner baru | `CARGO_TARGET_DIR` temp + retry (pola 6B) |
| SmartScreen | Utang (code signing ditunda) |


### Rencana Installer Pengguna Akhir (UX & Distribusi)

Alur distribusi end-user yang ditargetkan:

```
Developer:  cargo build --release → bundel paket → compile installer
                                    ↓
            EvernightLanguage-0.1.0-Setup.exe   (satu berkas, klik dua kali)
            evernight-0.1.0-windows-x64.zip     (fallback portabel, sudah ada)
                                    ↓
End user:   klik dua kali Setup.exe → wizard → "Pasang" → selesai
```

Rincian UX installer:
1. **Halaman Selamat Datang** — logo + sambutan Bahasa Indonesia + versi.
2. **Lisensi** — MIT (pemegang hak: Satriyo).
3. **Tujuan Instalasi** — lokasi folder (dapat diubah).
4. **Komponen** — compiler + dokumentasi + ekstensi editor.
5. **Tugas Tambahan** — PATH, asosiasi `.eve`, ekstensi editor, pintasan.
6. **Proses** — bilah kemajuan + status.
7. **Selesai** — tombol **"Jalankan REPL"** dan **"Buka Panduan"**; opsi buka folder instalasi.

**Deteksi konflik**: bila `.eve` sudah diklaim aplikasi lain, installer meminta konfirmasi sebelum menimpa asosiasi.

**Code signing**: **ditunda** (utang) — installer tanpa sertifikat akan memicu peringatan **SmartScreen** ("Windows protected your PC"). Sertifikat code signing berbayar; direncanakan menyusul.


### Addendum: Asosiasi File Windows (keputusan 2026-09-10)

Ekstensi tetap **`.eve`**; di Windows Explorer tipe file tampil sebagai **"Evernight files"**.

| Registry Key | Nilai |
|--------------|-------|
| `HKCR\.eve` (Default) | `EvernightFile` (ProgID internal) |
| `HKCR\EvernightFile` (Default) | `Evernight files` (teks kolom Type di Explorer) |
| `HKCR\EvernightFile\DefaultIcon` | `%ProgramFiles%\Evernight\assets\logo\icon.ico` |
| `HKCR\EvernightFile\shell\open\command` | `"…\evernight.exe" "%1"` |

Catatan pelaksanaan (Fase 6):
1. Sediakan skrip `.reg`/PowerShell manual untuk menguji asosiasi sebelum installer jadi.
2. Deteksi konflik: bila `.eve` sudah diklaim aplikasi lain, installer meminta konfirmasi sebelum menimpa.
3. Biner defaultnya `evernight.exe` (dijalankan dengan argumen berkas). Di Linux/macOS asosiasi menyusul (MIME/UTI) — tidak menghambat Windows.

## Fase 7: Stabilisasi & Rilis 1.0 (DEV) — **DITUNDA** (keputusan user 2026-09-19)
- [x] Audit keamanan dasar (akses file, batasan sandbox)
- [x] Stabilisasi versi bytecode (kompatibilitas antar-rilis)
- [x] Pengujian ketahanan edge-case + regresi performa
- [x] Definisi kriteria "selesai" rilis 1.0
- [x] **Language Server dasar** (dipindah dari 6D, keputusan 2026-09-14) — hover, diagnostic, goto-definition, completion; biner `evernight-lsp` via JSON-RPC stdio

### Sub-Fase Pelaksanaan Fase 7 (urutan eksekusi)

- **7E — Poles Desain Installer + Uji Pengguna Akhir** ✅ *(desain 6F final apa adanya, tidak ada perubahan visual)*
  - **Status**: kerangka + desain dasar installer sudah jadi di **6F** (selesai 2026-09-16); 7E hanya memoles dan menguji.
  - **Palet**: dari gambar referensi user (lihat Addendum Akhir 6D) — latar `#37313D`, well `#2F2430`, kartu `#443C4A`, teks `#F8DCE8`, aksen `#D3A0B9`. Satu-satunya berkas yang perlu disentuh: `installer/src/tema.rs`.
  - **Aset**: `installer/aset/maskot.png` (cutout), `aset/fonts/` (Plus Jakarta Sans + JetBrains Mono), `aset/logo/`.
  - **Poles yang direncanakan**:
    - [ ] Penyempurnaan tepi cutout maskot & komposisi panel
    - [ ] Penyelarasan jarak/ukuran teks di seluruh halaman
    - [ ] Penyempurnaan animasi (kurva easing, durasi, riak)
    - [ ] Uji keterbacaan & kontras (target WCAG AA)
  - **Uji akhir di lingkungan bersih**: instalasi di komputer/akun Windows lain (atau VM), verifikasi alur wizard, klik-ganda `.eve`, uninstall via *Apps & Features*, dan tampilan visual.
  - **Catatan**: installer 6F memakai egui (Rust), bukan Inno Setup — animasi & tampilan dikendalikan `installer/src/tema.rs` + `anim.rs`.
  - *Catatan: fungsionalitas dasar installer dibangun di **Fase 6F**; 7E hanya memoles desain + uji akhir.*

- **7F — Paket Rilis Per-Versi** ⬜ *(belum — dikerjakan di Fase 7)*
  - **Status**: paket `paket/evernight-0.1.0-windows-x64/` sudah ada (Fase 6B), tapi isi belum lengkap (kurang `kill.*`, `update.*`, `LICENSE`, catatan versi) dan belum ada skrip otomatis untuk merakit paket per-versi.
  - **Tujuan**: user tinggal download paket → ekstrak → install/setup → bahasa + command langsung jalan.
  - **Struktur paket per versi** `evernight-<versi>-windows-x64/`:
    - `bin/` → `evernight.exe`, `kill.cmd`, `kill.ps1`, `update.cmd`, `update.ps1`
    - `setup/` → `Setup.exe` (installer GUI v2)
    - `extensions/` → `.vsix` terbaru
    - `docs/` → `PANDUAN.txt`, `catatan-<versi>.txt`
    - root → `LICENSE` (MIT), `BACA-AKU.txt`, `install`/`uninstall` script, `versi.txt` (manifes)
    - ZIP + SHA256SUMS (3 baris: exe, setup, zip)
  - **Skrip `paket/paket-rilis.ps1`**: rakit folder dari repo (cargo build release + payload + vsix + docs) → ZIP → SHA256. Satu perintah = satu paket konsisten, tanpa manual.
  - **Hubungan dengan updater 6I**: ZIP/bin per versi ini sumber update dari `version/` GitHub.
  - **Bukti pertama**: regenerasi paket 0.1.0 pakai skrip baru.

## Fase 8: Dokumentasi Publik, Website & Komunitas (NON-DEV)
- [ ] README utama publik (logo, quickstart, status fase, fitur)
- [x] Panduan sintaks Bahasa Indonesia per keyword — `docs/panduan/01–07` + `docs/kata-kunci` (KEYWORD.md)
- [x] Tutorial pemula + latihan bertahap — `docs/tutorial/01–09` (tiap contoh diverifikasi jalan; **Tahap 10 mini proyek dihapus 2026-09-27**, keputusan user)
- [ ] Referensi stdlib lengkap (penjelasan tiap fungsi + contoh kode) — `STDLIB.md` sudah ada, kolom contoh menyusul
- [x] 10+ contoh program bertahap — `examples/` (11 program) + galeri `docs/contoh.md`
- [ ] Website resmi + playground online berbasis WASM (eksperimen)
  - [x] **Situs Astro dibangun lokal 2026-09-25** — `situs/`: beranda (desain Stitch + switch ID/EN), 29 halaman docs (sidebar 4 grup, prev/next, plugin link `.md`), halaman Unduh (Setup.exe + ZIP + SHA256), `astro build` 31 halaman
  - [x] **I18n EN chrome menyeluruh 2026-09-27** — `data-en`/`data-en-title`/`data-en-aria` (Base.astro): slide, judul h1, sidebar, prev/next, tombol Jalankan, CobaRun, kuis full EN (36 soal), unduh, footer; build 30 halaman setelah Tahap 10 dihapus
  - [x] **Isi artikel EN (tutorial+panduan) 2026-09-27** — 16 file `.en.md` paralel, render ganda `#isi-id`/`#isi-en` URL tunggal (keputusan user: tutorial 01–09 + panduan 01–07 saja, Memulai/Rujukan tetap ID; blok kode byte-identical); varian EN difilter via `filePath` (loader Astro buang titik pada id)
  - [x] **Isi artikel EN (Reference 9 halaman) 2026-09-27** — keputusan user: **Rujukan saja**, file EN sibling root (`KEYWORD/TIPE/FUNCTION/ARRAY/ERROR/GRAMMAR/STDLIB.en.md`) + `docs/cli.en.md`/`docs/contoh.en.md`, glob `content.config.ts` +`'*.en.md'`; Memulai tetap ID; tabel pesan/kode byte-identik, prosa diterjemah
  - [ ] Deploy **Vercel** + domain — **DITUNDA oleh user 2026-09-27** (statis, tanpa adapter; Root Directory `situs`; menggantikan rencana GH Pages — keputusan user 2026-09-27)
  - [ ] Playground WASM (`evernight_wasm`)
- [ ] Publish ekstensi VS Code ke Marketplace (satu `.vsix` untuk keluarga VS Code)
- [ ] `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, template issue/PR
- [ ] Peta jalan (roadmap) v1.x / v2.0
- [ ] Galeri program komunitas + tantangan pertama ("100 baris pertama")
- [ ] Pengumuman & catatan rilis 1.0
- [ ] Survei pengguna pertama
- [ ] Jadwal pemeliharaan pasca-1.0

## Fase 9: Ekosistem Sains Data & Machine Learning: Matra
> Pustaka terintegrasi *all-in-one* pengganti NumPy, Pandas, Matplotlib, Scikit-learn, Optuna, Imbalanced-learn, Yellowbrick, dan PyTorch dalam satu ekosistem berbahasa Indonesia. Spesifikasi lengkap di `MATRA.md`.

- [x] **9A — `matra.larik` (Tensor & Komputasi Numerik)**: Tipe data array homogen 1D/2D di Rust VM, vektorisasi operasi aritmatika, aljabar linier, transposisi, dan statistik dasar.
- [x] **9B — `matra.tabel` (Manipulasi Data Tabular / DataFrame)**: Struktur data columnar `MatraTabel` (DataFrame), pembaca & penulis CSV zero-dependency, pemfilteran baris/kolom, penanganan nilai kosong (`isi_kosong`/`buang_kosong`), agregasi grup (`kelompokkan`), ringkasan statistik `deskripsi()`, konversi matriks kontinu `ke_larik` untuk ML, dan penampil tabel ASCII berbingkai.
- [x] **9C — `matra.grafik` (Mesin Visualisasi & Diagnostik)**: Pembuat grafik vektor SVG murni (garis, sebar, batang, histogram, matriks konfusi) dan penampil grafik ASCII langsung di terminal tanpa dependensi GUI. Method chaining lengkap (`.judul()`, `.label_x()`, `.label_y()`, `.warna()`, `.simpan_svg()`, `.tampil()`), zero external dependencies.
- [x] **9D — `matra.latih`, `matra.alur` & `matra.uji` (Suite Data Training & Testing)**: Pembagian data latih/uji/validasi berstratifikasi, validasi silang (*K-Fold*), penyeimbang kelas (*SMOTE*), *Pipeline* anti kebocoran data (*anti-leakage*), deteksi pergeseran data (*Data Drift*), dan penyetelan hyperparameter otomatis (*TalaBayes* / Optuna-style).
- [x] **9E — `matra.model` & `matra.metrik` (Machine Learning Klasik & Evaluasi)**: Model ML klasik komprehensif zero-dependency (`RegresiLinear`, `RegresiLogistik`, `KNNKlasifikasi`, `KNNRegresi`, `PohonKeputusan`, `HutanAcak`, `KMeans`), integrasi runtime `Value::Model` dengan method chaining (`latih`, `prediksi`, `prediksi_probabilitas`, `skor`), integrasi pipeline anti kebocoran data `PipaKerja`, serta suite metrik evaluasi lengkap (`akurasi`, `presisi`, `recall`, `f1_skor`, `matriks_konfusi`, `laporan_klasifikasi`, `mse`, `rmse`, `mae`, `r2_skor`, `mape`).
- [x] **9F — `matra.syaraf` & `matra.pelatih` (Deep Learning & Otomatisasi Training)**: Engine diferensiasi otomatis (graf komputasi *Autograd*), lapisan jaringan syaraf tiruan (`Linear`, `Dropout`), aktivasi (`Relu`, `Sigmoid`, `Tanh`, `Softmax`), optimizer (`Adam`, `SGD`), fungsi kerugian (`KerugianMSE`, `KerugianBCE`, `KerugianKategori`), serialisasi bobot berkas `.matra`, serta orkestrator pelatihan otomatis (`Pelatih`, *Early Stopping* `BerhentiAwal`, *Model Checkpointing* `SimpanTerbaik`, dan `PencatatRiwayat`).
- [x] **9H — Web Notebook Interaktif "March" (Format Berkas `.emlbn`)**:
  - Web UI lokal interaktif bergaya **Google Colab** (Dark Mode), sel Kode & Markdown, tombol eksekusi bulat `▶`, dan panel berkas lokal.
  - Editor sel kode interaktif berlapis (Dual-Layer): syntax highlighting bertema Nusantara / VS Code Modern Dark (komentar hijau, string oranye, keyword ungu, deklarasi teal, builtin kuning) dan autocomplete floating popup IntelliSense presisi kursor (dukungan kata kunci, snippets `jk`/`jkl`/`jklj`/`fg`/`cb`, dan fungsi Matra dengan navigasi keyboard).
  - Akses dataset & CSV **100% lokal dari harddisk laptop pengguna** tanpa ketergantungan cloud.
  - Format berkas notebook **`.emlbn`** (*Evernight Machine Learning Notebook*) berbasis JSON terstruktur.
  - Kernel VM incremental stateful (variabel/impor/model bertahan antar-sel).
  - Rendering output interaktif: DataFrame tabel HTML persis tabel Colab, grafik SVG vektor responsif, dan teks konsol.
  - Subcommand CLI: `evernight march [berkas.emlbn]` dan ekspor ke `.eve`.
- [x] **9G — Rilis Ekosistem Matra & Integrasi Updater GitHub (`update evernight system`)**:
  - Bump versi semantik di workspace & biner rilis teroptimasi (`0.2.0`).
  - Catatan rilis di `version/catatan-0.2.0.txt` yang mendokumentasikan fitur sains data & ML `Matra` serta Web Notebook `March`.
  - Sinkronisasi biner rilis `evernight.exe` (3.07 MB), paket distribusi ZIP di `paket/`, dan pembaruan manifes di `version/version` (`0.2.0`).
  - Push ke repositori GitHub `Evanight234/EvernightLang` (commit `36e7ddb`).
  - **Uji coba di terminal Windows / CMD lokal (di `C:\`)**: menjalankan perintah `update evernight system` yang langsung menarik versi `0.2.0` dari GitHub, menimpa instalasi sistem di `%LOCALAPPDATA%\Programs\Evernight\bin\evernight.exe`, dan langsung mengaktifkan fitur `matra` & `march` secara global tanpa instalasi ulang manual. Verifikasi `evernight system info` menampilkan versi 0.2.0 dan pesan khas `"terupdate ke versi 0.2.0, jangan panggil istriku lagi dasar karbit"`.

---

## Struktur Proyek Rencana

```
EvernightLang/
├── Cargo.toml                      # Workspace Root
├── crates/
│   ├── evernight_core/             # Frontend: Token, Lexer, AST, Parser, Errors
│   │   ├── Cargo.toml
│   │   └── src/ (lib.rs, token.rs, lexer.rs, ast.rs, parser.rs, errors.rs)
│   ├── evernight_vm/               # Backend: Bytecode, Compiler, Value, VM, Stdlib
│   │   ├── Cargo.toml
│   │   └── src/ (lib.rs, bytecode.rs, compiler.rs, value.rs, vm.rs, stdlib/)
│   └── evernight_cli/              # Binary Runner & REPL
│       ├── Cargo.toml
│       └── src/ (main.rs, repl.rs)
├── editors/vscode/              # Ekstensi VS Code (TypeScript, Fase 4)
├── tests/
└── examples/
```

> Peta lengkap ada di `STRUKTUR.md`.

## Risiko & Mitigasi

1. Implementasi murni Rust (0 FFI) — C dibatalkan permanen; hot path dioptimasi via Rust (inline, bytecode optimasi)
2. VM lambat → optimasi bytecode (threaded code, inline caching) nanti
3. Stdlib kecil → minimal dulu, tambah sesuai kebutuhan

## Log Keputusan Desain

- 2026-10-30: **Editor Berlapis Syntax Highlighting & Autocomplete IntelliSense Web Notebook March.** Menambahkan editor kode interaktif dual-layer di antarmuka Web March (`crates/evernight_cli/src/march/aset/index.html`). Menghadirkan pewarnaan sintaksis tema Nusantara / VS Code Modern Dark (komentar hijau italic, string oranye, angka mint, keyword kontrol ungu, deklarasi teal, fungsi/builtin/Matra kuning, operator, kurung) serta popup floating dropdown IntelliSense presisi kursor saat pengguna mengetik (misal mengetik `j` memunculkan rekomendasi `jika`, `jk`, `jkl`, `jklj`, `lainnya_jika`). Dilengkapi navigasi keyboard panah atas/bawah, Tab/Enter untuk substitusi cerdas snippet/keyword, dan Escape untuk menutup popup, tanpa dependensi eksternal.

- 2026-10-29: **Peningkatan UX Web Notebook March — Desain Colab Activity Bar & Fitur Pemilihan Folder.** Memisahkan kontrol navigasi panel menjadi Activity Bar 44px dan Drawer 280px (arsitektur Google Colab) sehingga penutupan panel menyembunyikan drawer secara sempurna tanpa menyisakan tombol menumpuk. Menambahkan kemampuan memilih direktori folder penyimpanan berkas `.emlbn` secara dinamis lewat Web UI (modal dialog, tombol native Windows FolderBrowserDialog, input direktori) serta opsi CLI `--folder <DIR>`. Memperbaiki format encoding respon JSON ke `charset=utf-8` untuk mencegah mojibake simbol Markdown.

- 2026-10-28: **Migrasi Dokumentasi Proyek ke Obsidian Vault (`Evernight-language`) & Filter Berkas March (.emlbn).** Berdasarkan keputusan user, seluruh dokumen perencanaan, memori AI, dan spesifikasi teknis dipindahkan ke Obsidian Vault `Evernight-language/` yang berfungsi sebagai "otak & muscle memory" AI Agent dan pengembang manusia. Dilengkapi `PETA-KONSEP.md` sebagai Map of Content berbasis wikilinks dua arah (`[[...]]`) untuk Graph View interaktif Obsidian. Sekaligus bilah berkas March disaring murni hanya menampilkan berkas `.emlbn` lengkap dengan kapabilitas membuka notebook langsung dan membuat notebook baru.

- 2026-10-28: **Fase 9H selesai — Web Notebook Interaktif "March" (Format Berkas `.emlbn`).** Berhasil dibangun subsistem web notebook data science & machine learning interaktif murni Rust (zero external dependencies) di `crates/evernight_cli/src/march/`. Model berkas `.emlbn` (*Evernight Machine Learning Notebook*) berbasis JSON terbuka dengan serializer/parser zero-dependency dan konversi ekspor ke skrip `.eve`. Driver kernel `MarchKernel` mempertahankan status VM antar-sel secara stateful melalui `run_incremental`, mendukung output capture multi-media (tabel DataFrame HTML Colab, plot SVG vektor resolusi tinggi, output konsol teks, dan banner kesalahan `BAHAYA [KODE]`). Antarmuka SPA Google Colab Dark Mode berbasis Vanilla Modern Web (`index.html`) disematkan langsung ke dalam biner release `evernight.exe` melalui `include_str!`, lengkap dengan panel pohon berkas lokal laptop pengguna, editor sel multi-baris dengan auto-resize & keyboard shortcuts (`Shift+Enter`, `Ctrl+Enter`, `Alt+Enter`), tombol run bulat `▶`, serta server HTTP TCP lokal mandiri (`std::net::TcpListener`) di perintah CLI `evernight march`. Teruji 18 unit test biner, 30 CLI integration test hijau, biner release diperbarui, dan percontohan `examples/demo_notebook.emlbn` terverifikasi 100% sukses.

- 2026-10-28: **Fase 9F selesai — Deep Learning, Autograd Engine, Jaringan Syaraf Tiruan & Otomatisasi Training (`matra.syaraf` & `matra.pelatih`).** Implementasi Deep Learning murni Rust (zero-dependency) selesai dibangun di `crates/evernight_vm/src/matra/syaraf.rs` dan `crates/evernight_vm/src/matra/pelatih.rs`. Tipe runtime baru: `Value::ModelSyaraf(Rc<RefCell<MatraJaringan>>)`, `Value::Lapisan(Rc<RefCell<MatraLapisan>>)`, `Value::FungsiKerugian(Rc<MatraLoss>)`, `Value::Pengoptimal(Rc<RefCell<MatraOptimizer>>)`, `Value::Pelatih(Rc<RefCell<MatraPelatih>>)`, dan `Value::Callback(Rc<RefCell<MatraCallback>>)`. Menyediakan lapisan `Linear` (inisialisasi Kaiming/He, forward pass & gradient backward pass analitik), `Dropout` (inverted dropout berskala otomatis), aktivasi non-linear (`Relu`, `Sigmoid`, `Tanh`, `Softmax`), fungsi kerugian (`KerugianMSE`, `KerugianBCE`, `KerugianKategori`), pengoptimal bobot (`SGD` dengan momentum dan `Adam` adaptif momen pertama/kedua dengan bias-correction), serialisasi bobot floating-point presisi tinggi format `.matra` (`.simpan()` dan `.muat()`), serta orkestrator pelatihan otomatis `Pelatih` dengan mini-batch shuffle LCG, evaluasi validasi interaktif per epoch, dan sistem callback modular (`BerhentiAwal`, `SimpanTerbaik` model checkpointing, `PencatatRiwayat` ekspor JSON). Teruji 7/7 unit test integrasi di `test_matra_syaraf.rs` (total 44/44 test Matra lulus 100%), seluruh workspace hijau, biner rilis diperbarui, dan skrip percontohan `examples/matra_syaraf.eve` teruji sempurna.

- 2026-10-28: **Fase 9E selesai — Model Machine Learning Klasik & Metrik Evaluasi Komprehensif (`matra.model` & `matra.metrik`).** Berhasil diimplementasikan pustaka model Machine Learning klasik murni Rust (zero external dependencies) di `crates/evernight_vm/src/matra/model.rs` dan `crates/evernight_vm/src/matra/metrik.rs`. Tipe runtime baru `Value::Model(Rc<RefCell<MatraModel>>)` mendukung antarmuka konsisten: `.latih(X, y)`, `.prediksi(X)`, `.prediksi_probabilitas(X)`, `.skor(X, y)`, `.koefisien`, `.intersep`, `.pusat_klaster`, dan `.inersia`. Algoritma mencakup: `RegresiLinear` (gradient descent & estimasi OLS), `RegresiLogistik` (Sigmoid One-vs-Rest multi-kelas & probabilitas), `KNNKlasifikasi` & `KNNRegresi` (tetangga terdekat Euclidean/Manhattan), `PohonKeputusan` (CART split berdasar Gini Impurity & Variance Reduction), `HutanAcak` (Bagging ensemble pohon keputusan), serta `KMeans` (Lloyd iteration unsupervised clustering). Submodul `matra.metrik` menyediakan evaluasi lengkap: klasifikasi (`akurasi`, `presisi`, `recall`, `f1_skor`, `matriks_konfusi`, tabel format ASCII `laporan_klasifikasi`) dan regresi (`mse`, `rmse`, `mae`, `r2_skor`, `mape`). Terintegrasi penuh dengan `mt.alur.PipaKerja` (`pipa.prediksi(X)`), teruji 7/7 unit test di `test_matra_model.rs` (total 37/37 unit test Matra lulus 100%), seluruh test workspace hijau, dan skrip percontohan `examples/matra_model.eve` terverifikasi sukses.

- 2026-10-28: **Perencanaan Subfase 9H: Web Notebook Interaktif "March" (format `.emlbn`) (keputusan user).** Ditetapkan pembuatan web notebook data science & machine learning bernama **March** dengan format berkas **`.emlbn`** (*Evernight Machine Learning Notebook*). Desain antarmuka mengadopsi estetika **Google Colab** (Dark Mode), sel Kode & Markdown, tombol Run bulat `▶`, dan tabel DataFrame HTML interaktif. Fitur pembeda utama: akses dataset & berkas CSV **100% lokal dari harddisk laptop pengguna** tanpa upload cloud. Spesifikasi lengkap ditulis di `MARCH.md`.
- 2026-10-27: **Fase 9C selesai — Mesin Visualisasi & Diagnostik Vektor SVG + Terminal ASCII `matra.grafik`.** Engine visualisasi data murni Rust (zero-dependency) selesai dibangun di `crates/evernight_vm/src/matra/grafik.rs` dan terintegrasi ke runtime VM melalui tipe `Value::Grafik(Rc<RefCell<MatraPlot>>)`. Mendukung 5 jenis visualisasi: grafik garis (`garis`), sebar (`sebar`), diagram batang (`batang`), histogram frekuensi (`histogram`), dan matriks konfusi (`matriks_konfusi`). Mendukung interoperabilitas dengan list bawaan, `MatraArray`, dan DataFrame `MatraTabel`. Dilengkapi method chaining (`.judul()`, `.label_x()`, `.label_y()`, `.warna()`, `.warna_latar()`, `.grid()`, `.ukuran()`, `.simpan_svg()`, `.tampil()`), ekspor vektor SVG mandiri beresolusi tinggi, serta render instan di terminal konsol berbasis ASCII/Unicode. Teruji 7/7 unit test integrasi di `test_matra_grafik.rs`, seluruh workspace 100% hijau, dan diverifikasi langsung lewat skrip contoh `examples/matra_grafik.eve`.
- 2026-10-27: **Fase 9B selesai — Mesin DataFrame & CSV Parser `matra.tabel`.** Struktur data columnar `MatraTabel` dan tipe runtime `Value::Tabel` terintegrasi penuh di Rust VM. Mendukung pembuatan dari kamus/daftar rekaman/larik 2D, indexing `df["kolom"]` / `df[row]`, mutasi kolom, slicing `kepala(n)` / `ekor(n)`, pemilihan `pilih_kolom` dan pembuangan `kecuali`, pengurutan `urutkan`, filter boolean mask `saring`, deteksi & pengisian nilai kosong `ada_kosong` / `buang_kosong` / `isi_kosong`, kalkulasi ringkasan statistik `deskripsi()`, agregasi multi-fungsi `kelompokkan`, interop tensor 2D `ke_larik` untuk ML, dan pembaca/penulis CSV aman dengan sandbox. Teruji 8/8 test integrasi di `test_matra_tabel.rs` dan skrip contoh `examples/matra_tabel.eve`.
- 2026-10-27: **Penetapan Tahap Akhir Fase 9: Rilis Ekosistem Matra via Updater GitHub (`9G`) (keputusan user).** Di ujung Fase 9 ditetapkan subfase 9G untuk merilis versi baru yang memuat seluruh pustaka `Matra` ke folder `version/` di repositori GitHub, sehingga user dari terminal lokal mana pun di Windows (CMD / PowerShell di direktori `C:\`) cukup menjalankan satu perintah `update evernight system` untuk memperbarui instalasi global secara otomatis langsung dari GitHub.
- 2026-10-27: **Rencana Fase 9: Ekosistem Sains Data & Machine Learning: Matra (keputusan user).** Ditetapkan nama resmi pustaka **`Matra`** dengan konsep *all-in-one* yang menggabungkan dan memodifikasi peran 5 pustaka Python (NumPy, Pandas, Matplotlib, Scikit-learn, PyTorch) ke dalam satu pustaka terpadu berbahasa Indonesia. Menggunakan arsitektur dua lapis (Rust Engine untuk array datar f64/SIMD + sintaks `.eve` untuk abstraksi tingkat tinggi). Spesifikasi lengkap ditulis di `MATRA.md`.
- 2026-09-16: **Revisi desain installer (6F) — keputusan user.** Empat perbaikan: (1) **Window diperlebar** 760×520 → **1000×640** (min 880×580) karena beberapa UI terpotong; (2) **CMD dihilangkan** — `windows_subsystem = "windows"` (kondisional `not(debug_assertions)`) + `CREATE_NO_WINDOW` pada semua `Command` agar `reg`/`powershell`/`net`/CLI editor tidak memunculkan atau mengedipkan jendela console; (3) **pemilih folder native** lewat crate **`rfd` 0.17** (`default-features = false`) dengan tombol **Telusuri...** di halaman Lokasi Tujuan; (4) **satu box per halaman**, isi box hanya judul + keterangan (teks), sedangkan semua kendali interaktif (radio cakupan, checkbox tugas, input folder, progress bar, log) diletakkan **di luar** box. Verifikasi: PE Subsystem = **2 (GUI)**, dialog folder teruji terbuka ("Pilih folder tujuan pemasangan"), siklus install (PATH 31→32, `.eve`, Apps & Features) dan uninstall (folder terhapus penuh, PATH→31) tetap bersih, 17 test installer hijau, ukuran 8.37 MB.
- 2026-09-16: **Fase 6 SELESAI (6A–6F).** Tooling 6D (debug/profiler/formatter/linter/pkg), CI + rilis 6E, dan installer GUI 6F semuanya tuntas. **LSP dipindah ke Fase 7** (tidak menghambat rilis; ekstensi sudah punya static completion). Installer memakai **egui**, bukan Inno Setup. Dua bug penting ditemukan & diperbaiki: **BOM UTF-8** membuat berkas dari editor Windows gagal di-tokenisasi; **UninstallString** sempat menunjuk `evernight.exe --uninstall` (flag yang tidak dikenal compiler) → diganti `uninstall.exe`. Skrip `build-installer.ps1` diberi **penjaga biner basi** setelah installer sempat membawa compiler lama tanpa subcommand baru.
- 2026-09-14: **Fase 6D-1 & 6D-2 selesai — Mode Debug & Profiler.** `--debug` = trace instruksi bytecode ke stderr (observasi murni, tanpa ubah bytecode/kompiler). `--waktu` = profil (waktu ms, total instruksi, frekuensi opcode terurut). Keduanya lewat flag runtime di `Vm` (`debug_trace`, `profile_mode`) + `OpCode::name()`. Counter pakai array `[usize; 256]` — zero-dep. Flag `--debug`/`--waktu` selalu menjalankan program (bukan mode inspeksi-only). Urutan 6D ditetapkan: debug → profiler → formatter → lint → LSP → pkg (LSP termasuk, keputusan user). Package manager versi minimal + dibundel di installer 6F.
- 2026-09-14: **Fase 6C selesai — integrasi otomatis ekstensi editor.** `install.ps1` mendeteksi editor keluarga VS Code via CLI (PATH + lokasi standar), lalu **menawarkan** pemasangan ekstensi (`y/t`); opsi `-TanpaEkstensi` (lewati) dan `-TanpaKonfirmasi` (senyap, dipakai 6F). `uninstall.ps1` menawarkan pencopotan (`y/t`) dengan `-SimpanEkstensi`/`-TanpaKonfirmasi`. Deteksi **tidak** memakai keberadaan folder data (`.cursor`/`.windsurf` bisa ada tanpa aplikasi terpasang). **Bug diperbaiki**: peringatan CLI di stderr (mis. `antigravityAnalytics`) sempat dianggap galat fatal karena `$ErrorActionPreference="Stop"` — kini dilonggarkan di sekitar pemanggilan CLI. Logika `Cari-Editor` akan dipakai ulang di installer GUI 6F.
- 2026-09-14: **Rencana Installer Pengguna Akhir (6F) + Desain Visual (7E).** Alur distribusi end-user ditargetkan memakai **satu berkas `Setup.exe`** (Inno Setup 6) — tanpa ekstrak ZIP atau menjalankan skrip manual. Mode hak akses memakai `PrivilegesRequiredOverridesAllowed=dialog`: pengguna memilih **"Hanya saya"** (per-user, tanpa UAC) atau **"Semua pengguna"** (per-machine, UAC/allow access). Installer mencakup wizard Bahasa Indonesia, opsi PATH/asosiasi `.eve`/ekstensi editor/pintasan, serta uninstaller terdaftar di *Apps & Features*. **Fungsional dibangun di Fase 6F**, **desain visual final + uji di komputer lain di Fase 7E** (agar brand & konten sudah matang saat dipoles). **Code signing ditunda** (utang) — installer tanpa sertifikat memicu peringatan SmartScreen. **6C (auto-ekstensi berbasis skrip) ditunda**, logikanya akan diadaptasi ke dalam installer GUI 6F.
- 2026-09-14: **Fase 6B selesai — distribusi compiler mandiri terverifikasi.** Biner release **1.74 MB statis** (`+crt-static`, tanpa DLL MinGW). Paket `paket/evernight-0.1.0-windows-x64/` berisi `bin/evernight.exe`, `assets/`, `docs/PANDUAN.txt`, `extensions/evernight-language-0.1.0.vsix`, `install.ps1`/`install.cmd`, `uninstall.ps1`/`uninstall.cmd`, `BACA-AKU.txt`. Installer **per-user tanpa admin**: salin ke `%LocalAppData%\Programs\Evernight\`, PATH user idempoten, asosiasi `.eve` di `HKCU\Software\Classes` (ProgID `EvernightFile`, label **"Evernight files"**, perintah `evernight.exe run "%1"`). **Keputusan: uninstaller memakai skrip `.ps1`/`.cmd` (bukan `uninstall.exe`)** — konsisten dengan installer, tanpa perlu compile tool tambahan. Uji nyata: PATH 31→32, klik-ganda `.eve` berjalan, install 2× idempoten, uninstall kembali bersih.
- 2026-09-14: **Lisensi proyek = MIT; publisher ekstensi = `Satriyo` (keputusan user).** Lisensi kode EvernightLanguage resmi **MIT** (nama pemegang hak: Satriyo). Publisher ekstensi editor memakai nama asli **Satriyo** (bukan `evernight`). Fase 6 dipecah menjadi sub-fase **6A–6E** dengan urutan: 6A paket ekstensi `.vsix` → 6B distribusi compiler mandiri → 6C integrasi otomatis ekstensi saat install → 6D tooling tambahan → 6E CI & rilis. **6A selesai**: `.vsix` terpasang & terverifikasi di **Antigravity IDE** (`satriyo.evernight-language-0.1.0`), syntax highlighting `.eve` kini aktif.
- 2026-09-12: **Rencana diperluas dari 7 menjadi 8 fase (keputusan user).** Fase 5, 6, dan 7 difokuskan sepenuhnya pada **pengembangan teknis bahasa pemrograman & sistem** (Fase 5 = Testing & Kualitas Internal; Fase 6 = Tooling & Distribusi Sistem dengan compiler mandiri; Fase 7 = Stabilisasi & Rilis 1.0). Rencana non-pengembangan bahasa (pembuatan website resmi, dokumentasi publik, materi ajar, contoh program komunitas, dan promosi) dipindahkan ke **Fase 8: Dokumentasi Publik, Website & Komunitas**.
- 2026-09-12: **Subcommand eksplisit `evernight run <file>` + Arsitektur Distribusi Mandiri (Fase 6).** Ditambahkan dukungan resmi `evernight run berkas.eve` yang setara dengan default-run. Jika `evernight run` dipanggil tanpa berkas, sistem mengeluarkan error ramah `BAHAYA [ARG]`. Untuk Fase 6 disepakati bahwa distribusi compiler dibundel mandiri: biner `evernight.exe` otomatis masuk ke PATH sistem tanpa mewajibkan pengguna menginstal toolchain Rust/Cargo; berkas `.eve` langsung terhubung di Explorer sehingga dapat dijalankan instan.
- 2026-09-10: **Slogan final (keputusan user):** *"Evernight bukan hanya karakter game tapi bisa menjadi pelajaran bahwa menyukai karakter juga bisa menjadi motivasi."* **Font brand: sans-serif.** **Lisensi logo: bebas pakai dengan atribusi.** Semua dimasukkan ke file baru `BRAND.md` (brand kit final, palet + aturan pakai + peta aset); `LOGO.md` §7–§9 jadi pointer ke sana. **Fase 3 selesai** — satu utang: master SVG vektor logo.
- 2026-09-10: **Liputan editor = keluarga VS Code saja (keputusan user).** Prioritas: VS Code, Cursor, Windsurf, VSCodium (satu `.vsix`, kompatibilitas terus diverifikasi; Antigravity dicek saat eksekusi, bukan diasumsikan). **Installer integrasi editor bersifat opt-out**: default YA (binary+LSP di PATH, `code --install-extension` bila CLI ada), bisa tolak via checkbox; tanpa side-load paksa / utak-atik registry editor pihak ketiga. **Non-goal 1.0:** Sublime Text, Zed, Neovim, JetBrains — YAGNI, menyusul bila ada permintaan nyata. Icon theme `.eve` + extension snippets tetap satu `.vsix` yang sama.
- 2026-09-10: **Logo final-awal: badge gaya JS + huruf E.** User mengirim `logo1.png` (500×500: krem `(249,245,240)` + E serif gelap `(31,33,34)`), terinspirasi logo **JavaScript** (badge solid berisi huruf) — "E" = Evernight, penghormatan pada JS. **Masalah ditemukan:** stroke E orisinal tipis (2,4% cakupan) → tidak terbaca di 16px. **Solusi dua varian:** `logo.png` (master transparan, E tipis, untuk ukuran besar) + `logo_badge.png` (E dilasi-tebalkan, ≈39% cakupan, untuk icon kecil — pola umum favicon). `icon.ico` multisize dari badge; `logo_monokrom` (E putih) untuk latar gelap. **Utang teknis:** master SVG vektor belum ada (sumber raster) — menyusul. Review user via `assets/logo/preview.png` (opencode tak bisa lihat gambar).
- 2026-09-10: **Impor lintas berkas v1 final** — sintaks `impor "path/modul" [sebagai alias]` (identifier juga boleh); ekstensi `.eve` otomatis; path relatif ke direktori berkas utama. Semantik: modul dijalankan **lazy sekali** (cache by path absolut), dieksekusi di frame sendiri, seluruh `fungsi`/`variabel` top-level yang baru diekspos sebagai **`Value::Kamus`** (namespace); akses `alias.fungsi()` / tanpa alias pakai nama dasar berkas (`sub/helper` → `helper`). Siklus → `BAHAYA [FILE]`. Opcode `ImporModul` (0x95). `Vm::with_dir`. **Defer:** bentuk `impor X dari ...` (dari/selektif) ke v2. Catatan desain: global modul tetap berada di map global bersama (modul bisa saling panggil antar-fungsi), jadi nama top-level modul tetap terlihat di scope utama — ponytail, isolasi namespace penuh nanti bila perlu.
- 2026-09-10: **Modul utilitas final** — 13 builtin (id 62–74): `adalah_angka/teks/daftar/kamus` (62–65, arg hilang → `salah`), `ke_boolean`/`e_boolean`/`bolean` (66/67/72 — tiga nama satu makna `is_truthy`), `ke_larik`/`daftar` (68/73 — teks→karakter, daftar→salinan, kamus→kunci diurut alfabetis), `salin` (69 — deep copy rekursif, struktur siklik aman via `HashMap<usize>` visited, fungsi dibagikan), konversi dasar `angka` (70: Angka→identitas, Bolean→1/0, Teks→parse gagal `[NaN]`, lain `[TYPE]`), `teks` (71: semua `to_string`), `kamus` (74: Kamus→klon atau Daftar pasangan `[k,v]` → kamus, malformed → `[TYPE]`). Deviasi dari STDLIB: `bolean()` & `daftar()` (konstruktor) digabung menjadi alias `ke_boolean` & `ke_larik`; `kamus()` terima objek kamus atau daftar pasangan. `impor utilitas` no-op. Contoh `examples/utilitas.eve`.
- 2026-09-10: **Modul sistem final** — 10 builtin (id 52–61): `baca_file/tulis_file/ada_file` (instan `std::fs`, gagal → `[FILE]`), `env(nama, bawaan?)` (argc 1–2; tanpa bawaan → `[ENV]`, non-teks dikonversi), `atur_env`, `waktu_sekarang` (ms UTC), `tanggal_sekarang(ts?)` (tanpa arg = sekarang; tampilan **UTC** ISO `YYYY-MM-DD HH:mm:ss.mmm`), `format_tanggal(ts, pola)` (token `YYYY/MM/DD/HH/mm/ss`, sisanya disalin), `tunda(detik)` (fraksi OK, negatif → `[WAKTU]`), `selisih_waktu` (selisih mutlak). Kode error baru `[ENV]` & `[WAKTU]` → `ERROR.md` §G. **Dua defer (keputusan user):** handle berkas `buka/tutup/baca_baris/tulis` ditunda (API lifecycle belum dibutuhkan; `baca_file`/`tulis_file` menutup 95%) dan `argumen()` ditunda ke Fase 4 (akan dibangun bersama pemisah `--` & kode keluar di CLI). `impor sistem` no-op. Contoh `examples/sistem.eve`.
- 2026-09-10: **Modul daftar & kamus final** — 16 fungsi daftar (id 28–43) & 8 fungsi kamus (id 44–51). Keputusan user: (1) statement `hapus` HANYA untuk **indeks daftar** (`hapus a[i]`, wrap negatif, out-of-range `[INDEX]`) dan **kunci kamus** (`hapus k["x"]` / `hapus k.nama`); target lain = error kompilasi `[SYNTAX]`; **variabel tidak bisa di-hapus** (berbeda dari Python `del`, karena ini sistem statis+dinamis campuran). (2) Method syntax `a.tambah(x)` => **defer** (liabilitas desain karena `tambah` juga keyword-funci builtin global); hanya fungsi global `tambah(a,x)` dll; ARRAY.md diberi catatan. Mutasi in-place via RefCell, `urutkan` hanya homogen, `dapatkan(k, kunci, bawaan?)`, `lipat([], awal)` → awal, komparasi koleksi dalam (`nilai_sama`). `hapus` sebagai keyword juga berfungsi sebagai panggilan fungsi builtin `hapus(lst, item)` (parser disambiguasi: `hapus(...)` → expr statement).
- 2026-09-10: **Modul matematika final** — nama **`max`** (bukan `maks`, per STDLIB); alias ganda **`pembulatan`/`bundar`** & **`mutlak`/`abs`** kedua-duanya berlaku; konstanta **`pi` & `e` global** tanpa impor; **user-fungsi mengalahkan builtin** (`compiler` catat `declared_fns` saat deklarasi, rekursi faktorial user aman); deviasi kecil dari STDLIB: `acak_antara` rentang inklusif & swap bila min>max, `faktorial(i6)` fraksi, faktorial > 170 = error `[JUMLAH]`, `log(0)`/`akar(-4)` = error `[MATH]`.
- 2026-09-10: **Perintah pendek `evernight coba.eve` (Fase 4)** — default-run tanpa flag, buang keharusan `.\target\debug\evernight.exe --run ...`; `--run` tetap kompatibel; biner global via `cargo install`/PATH (spec: checklist Fase 4).
- 2026-09-10: **Nama biner/CLI = `evernight`** (crate tetap `evernight_cli`, biner `[[bin]] name = "evernight"`). Perintah: `evernight file.eve`. **Asosiasi file Windows (final):** ekstensi `.eve`, tipe tampil di Explorer sebagai **"Evernight files"** via ProgID `EvernightFile` + `DefaultIcon` icon.ico + `shell\open\command` → `evernight.exe "%1"`. Spek masuk item Installer Fase 6.
- 2026-09-10: **C dihapus permanen dari proyek.** `c_interop/` (FFI Rust↔C) dibatalkan selamanya. Implementasi = **Rust murni** (runtime/paket) + **TypeScript** (hanya ekstensi editor VS Code, Fase 4). Alasan: `native.c` tidak pernah dibuat (item checklist Fase 2 itu satu-satunya yang kosong), FFI menambah kompleksitas build & distribusi tanpa kebutuhan jelas, dan "se-cepat C++" cukup dicapai via optimasi Rust murni (bytecode optimasi, inline caching). `editors/vscode` scaffolding TypeScript dibuat 2026-09-10.
- 2026-09-10: **Audit Fase 2**: 9/11 item inti terkonfirmasi selesai di kode; 2 sisanya — `c_interop/` (defer, sudah ditandai) dan keyword `hapus` yang ternyata **belum diimplementasikan sama sekali** (tidak ada di lexer/parser/compiler/VM). Keputusan: `hapus` (del) **dipindahkan ke Fase 3** bersama modul koleksi (daftar/kamus) karena semantik hapus menempel pada method koleksi yang baru dibangun di sana. Tabel keyword & checklist Fase 3 diupdate.
- 2026-09-10: **Rencana diperluas dari 5 menjadi 7 fase.** Fase 3 dan 4 masing-masing menambahkan bagian identitas visual & editor: Fase 3 = Standard Library **+ Identitas Visual** (logo resmi, icon file `.eve`, brand kit); Fase 4 = REPL & CLI **+ Highlighting Editor** (grammar TextMate digenerate dari `token.rs`, tema warna unik Indonesia "Nusantara" dengan palet terang/gelap, ekstensi VS Code). Fase 6 & 7 (baru): Tooling Lanjutan (LSP, formatter, linter, debugger, package manager, installer, crates.io, Marketplace, CI) dan Komunitas & Rilis 1.0. Fase 3–7 masing-masing memiliki 12–15 item (sebelumnya cuma 3–6). Direktori proyek ditambah: `assets/logo/`, `editors/vscode/`, `paket/`, `situs/`. File MD baru: `LOGO.md`, `TEMA.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`.
- 2026-09-10: Fase 2 selesai menyeluruh (compiler 4 sub-modul, VM error-handling, 36 test hijau) — checklist Fase 2 dikunci `[x]`, c_interop dibatalkan/didefer.
- 2026-09-08: Keputusan arsitektur implementasi Rust Fase 2: Cargo Workspace (3 crates: `evernight_core`, `evernight_vm`, `evernight_cli`), Handwritten Lexer & Pratt Parser, Zero External Dependencies (pure stdlib), Stack-based Bytecode VM, dan Safe ARC Memory Model (`enum Value` + `Arc<RefCell<T>>`).
- 2026-09-08: Wildcard `_` ditambahkan ke pattern matching `cocok` (Fase 1). `hapus` (del) sudah ada di cadangan Fase 2. `adalah` (is) & `lewati` (pass) diputuskan TIDAK PERLU.
- 2026-09-07: Finalisasi kata kunci resmi di `KEYWORD.md` (`variabel`, `fungsi`, `kembali`, `impor`, error flow, loop keywords).
- 2026-09-07: Spesifikasi standard library final di `STDLIB.md` (7 modul: konsol, string, matematika, daftar, kamus, utilitas, sistem; impor via `impor`, `cetak()` built-in).
- 2026-09-07: Spesifikasi sistem error final di `ERROR.md` (format `BAHAYA [KODE]` / `PERINGATAN [KODE]`, warning fungsi/kelas kosong, NaN fatal langsung).
- 2026-09-07: Spesifikasi tipe data final di `TIPE.md` (angka terpadu, bolean, teks dengan .panjang, daftar dengan panjang(), kamus, strict type error tanpa implicit coercion).
- 2026-09-07: Keyword draft awal ditetapkan (jika/lainnya/selama/untuk/kembali/cetak/benar/salah/kosong).
- 2026-09-07: Tipe dinamis, ARC untuk manajemen memori, interpreter tree-walking dulu.
- 2026-09-07: Tidak pakai compiler — interpreter murni (lexer → parser → evaluator). Makefile dihapus dari struktur awal.
- 2026-09-07: Perubahan arah — eksekusi diganti ke **compiler bytecode + VM** (seperti Python/Lua): `.eve` → AST → bytecode → VM. REPL tetap bisa (compile per baris).
- 2026-09-07: Paradigma diubah: fungsional murni → **fungsional + OOP**. Immutability penuh dihapus. Konsep lengkap di `KONSEP.md`.
