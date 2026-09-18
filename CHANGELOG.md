# CHANGELOG.md — Riwayat Versi EvernightLanguage

> Format mengikuti [Keep a Changelog](https://keepachangelog.com) + [Semantic Versioning](https://semver.org).
> **Mulai diisi pada Fase 5** (versi semantik). Versi rilis 1.0 ditargetkan setelah Fase 7.

## Format Versi

- **vMAJOR.MINOR.PATCH** (contoh `v0.1.0`)
- Tipe perubahan:
  - `Ditambahkan` (Added) — fitur baru
  - `Diubah` (Changed) — perubahan perilaku/API
  - `Diperbaiki` (Fixed) — bug fix
  - `Dihapus` (Removed) — fitur dibuang
  - `Keamanan` (Security) — patch keamanan

---

## [Belum Dirilis]

### Diubah
- **Revisi desain installer (Fase 6F)**: window diperlebar dari 760x520 menjadi **1000x640** (minimum 880x580) agar tidak ada UI yang terpotong. Setiap halaman kini memakai **satu box** yang isinya hanya judul + keterangan; semua kendali interaktif (radio, checkbox, input, progress, log) diletakkan di luar box. Halaman **Lokasi Tujuan** dan **Siap Pasang** masing-masing disederhanakan dari dua kartu menjadi satu.
- **Installer memakai dialog folder native** lewat crate `rfd` 0.17 (`default-features = false`): tombol **Telusuri...** membuka File Explorer untuk memilih folder tujuan.

### Diperbaiki
- **Jendela CMD pada installer**: biner dibangun sebagai aplikasi console (PE `Subsystem = 3`), sehingga Windows menampilkan jendela CMD hitam. Kini memakai `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` (subsystem 2 saat rilis; build debug tetap menampilkan `eprintln!`). Semua pemanggilan program luar (`reg`, `powershell`, `net`, CLI editor) ditambah `CREATE_NO_WINDOW` agar tidak muncul maupun berkedip saat pemasangan.

### Diperbaiki
- **Ikon `.eve` tidak muncul di code editor**: berkas `icons/evernight-icon-theme.json` menunjuk `../assets/logo/icon_16.png` — relatif di dalam `.vsix` berarti `extension/assets/logo/icon_16.png`, berkas yang TIDAK ikut terpaket (vsix sebelumnya hanya berisi `icons/evernight-icon.png` + theme JSON). Akibatnya editor keluarga VS Code gagal memuat ikon dan diam-diam memakai ikon default. Icon kini dibundel sebagai `icons/evernight-file.png` (16x16) dan dirujuk lokal `./evernight-file.png`.
- File default pada icon theme (`"file": "_eve"`) dihapus agar berkas non-`.eve` tidak semua memakai ikon huruf E; hanya `.eve` yang dikustomisasi.
- Menambahkan `editors/vscode/build-vsix.ps1` (pembangun `.vsix` tanpa vsce/npx; vsce tidak tersedia karena cache npm korup `ECOMPROMISED`, `npx @vscode/vsce` gagal). Skrip mem-patch zip vsix yang ada dan memverifikasi entri ikon.

### Ditambahkan
- **Formatter (`evernight format`, Fase 6D-3)**: crate `evernight_fmt` berbasis token (komentar dipertahankan, idempoten). Opsi `--cek` (dry-run, exit 1 bila belum rapi — untuk CI) dan `--keluar <path>`.
- **Linter (`evernight lint`, Fase 6D-4)**: crate `evernight_lint` berbasis AST. Aturan baru: `WKHURUF` (bukan snake_case), `WKIMPOR` (impor tak terpakai), `WKPANJANG` (fungsi > 50 baris), `WKPARAM` (> 4 parameter), `WKSARANG` (blok kosong), `WKMATI` (kode tak terjangkau), `WKMAGIS` (angka magic). Digabung dengan warning compiler.
- **Package Manager (`evernight pkg`, Fase 6D-6)**: `init` (manifest `eve.json` + kerangka `utama.eve`), `jalankan` (jalankan berkas utama), `daftar` (tampilkan berkas proyek). Parser JSON minimal zero-dep.
- **Mode Debug (Fase 6D-1)**: `evernight <berkas> --debug` mencetak trace tiap instruksi bytecode ke `stderr` (`[offset] baris N | NamaOpcode`). `OpCode::name()` + `Vm::debug_trace`.
- **Profiler Mini (Fase 6D-2)**: `evernight <berkas> --waktu` mencetak waktu eksekusi (ms), total instruksi, dan tabel frekuensi opcode (jumlah + persentase). `Vm::profile_mode` + `Vm::profile_summary()`, counter zero-dep.
- **Installer GUI Windows (Fase 6F)**: `EvernightLanguage-0.1.0-Setup.exe` (8.35 MB) dibangun dengan **egui/eframe** (Rust). Wizard 6 halaman Bahasa Indonesia, panel maskot 28% lebar penuh, palet dari referensi user, font Plus Jakarta Sans & JetBrains Mono dibundel, bilah judul kustom, animasi micro-interaction (hover/tekan/riak/progress/denyut/transisi halaman). Mode per-user (tanpa UAC) / per-machine (UAC). Pemasangan: PATH, asosiasi `.eve`, ekstensi editor, pintasan Start Menu, entri *Apps & Features*. `uninstall.exe` mencabut semuanya dengan bersih.
- **CI & Rilis (Fase 6E)**: `.github/workflows/ci.yml` (fmt + clippy `-D warnings` + test + verifikasi contoh `.eve`) dan `rilis.yml` (build statis, ZIP, `SHA256SUMS.txt`, GitHub Release draft). `.gitignore` + `.gitattributes` ditambahkan.

### Diperbaiki
- **BOM UTF-8**: berkas `.eve` yang disimpan editor Windows (Notepad, `Set-Content -Encoding UTF8`) sebelumnya gagal di-tokenisasi (`Karakter tidak dikenal`). Kini BOM diabaikan lexer dan dipertahankan formatter.
- **Bantuan subcommand**: `evernight pkg --bantuan` (dan subcommand lain) kini menampilkan bantuan subcommand-nya, bukan bantuan utama.
- **Pencopotan installer**: `UninstallString` sempat menunjuk `evernight.exe --uninstall` (flag yang tidak dikenal compiler). Kini memakai `uninstall.exe`; pencopotan juga mengatasi kunci berkas dengan menjalankan diri dari folder temp sehingga folder instalasi terhapus penuh.
- **Paket berisi biner basi**: `installer/build-installer.ps1` kini memverifikasi umur biner compiler terhadap sumber `.rs`, mencegah installer membawa compiler lama.

### Direncanakan
- **Language Server (`evernight-lsp`)**: hover, diagnostic, goto-definition, completion — **dipindah ke Fase 7**.
- **Desain Visual Installer (Fase 7E)**: poles tampilan wizard, penyelarasan tipografi/jarak, uji di komputer/VM bersih.
- **Repositori GitHub**: pembuatan remote + push (perlu dilakukan user).

### Diketahui (Utang Teknis)
- **Code signing** installer belum ada, memicu peringatan SmartScreen.
- **Master SVG logo** belum tersedia (sumber raster saja).
- **Installer belum diuji di komputer bersih** (7E).

---

## [0.1.0] - 2026-09-12

### Ditambahkan
- **Frontend Compiler (`evernight_core`)**: Handwritten Lexer (tokenizer zero-dep) & Pratt Parser pendukung 11 level presedensi operator, pernyataan `jika/lainnya_jika/lainnya`, `selama`, `untuk` (range & collection iteration), `cocok` pattern matching, `coba/tangkap/akhirnya`, `lempar`, `pastikan`, `impor`, `hapus`, dan deklarasi `variabel`/`tetap`/`fungsi`.
- **Backend Bytecode VM (`evernight_vm`)**: Stack-based Bytecode VM + Call Frames, pemanggilan fungsi rekursif, penanganan handler error, penugasan gabungan, dan `Value` berbasis ARC.
- **7 Modul Standard Library**:
  - `konsol` (`cetak`, `baca`, `baca_angka`, `bersihkan`).
  - `string` (`besar`, `kecil`, `bersih`, `potong`, `pecah`, `gabung`, `ganti`, `mengandung`, `mulai_dengan`, `akhir_dengan`, `ulang_teks`, `format`).
  - `matematika` (`akar`, `pangkat`, `bulat_bawah`, `bulat_atas`, `pembulatan`, `bundar`, `mutlak`, `abs`, `acak_antara`, `log`, `faktorial`, `min`, `max`, `sin`, `cos`, `tan`, `pi`, `e`).
  - `daftar` (`tambah`, `sisip`, `hapus`, `urutkan`, `balik`, `unik`, `jumlah`, `rata_rata`, `gabung_larik`, `iris`, `cari`, `ada`, `peta`, `saring`, `lipat`, `setiap`).
  - `kamus` (`kunci`, `nilai`, `pasangan`, `ada_kunci`, `hapus_kunci`, `dapatkan`, `setel`, `gabung_objek`).
  - `sistem` (`baca_file`, `tulis_file`, `ada_file`, `env`, `atur_env`, `waktu_sekarang`, `tanggal_sekarang`, `format_tanggal`, `tunda`, `selisih_waktu`, `argumen`).
  - `utilitas` (`adalah_angka/teks/daftar/kamus`, `ke_boolean`, `ke_larik`, `salin` deep copy, `angka`, `teks`, `kamus`).
- **Sistem Modul `impor`**: Impor lintas berkas lazy-load, cache modul, dan deteksi siklus circular.
- **CLI Runner & REPL (`evernight_cli`)**: Default-run `evernight file.eve`, subcommand `evernight run`, REPL interaktif (`repl.rs`), error terminal berwarna dengan cuplikan baris kode & caret `^`, disassembly `--bytecode`, `--cek`, `--tokens`, `--ast`.
- **Identitas Visual**: Logo resmi (badge gaya JS + huruf E) dan ikon multi-ukuran (`assets/logo/`), serta `BRAND.md` (slogan, font, lisensi, palet).
- **Ekstensi VS Code (`editors/vscode`)**: TextMate syntax grammar (`evernight.tmLanguage.json`), tema Nusantara Gelap & Terang, 23 snippet (`eve.json`), autocomplete statis ~60 item (`extension.ts`), dan ikon berkas `.eve`.
- **Lisensi & Distribusi Ekstensi (Fase 6A)**: Lisensi proyek resmi **MIT** (Satriyo); `package.json` lengkap (publisher `Satriyo`, icon, repository, keywords); `.vscodeignore`; paket `evernight-language-0.1.0.vsix` siap distribusi — terverifikasi terpasang di **Antigravity IDE**.
- **Distribusi Compiler Mandiri (Fase 6B)**: Biner release **statis 1.74 MB** (tanpa dependensi Rust/Cargo/DLL MinGW). Paket portabel `paket/evernight-0.1.0-windows-x64/` + ZIP. Skrip `install.ps1`/`install.cmd` (per-user, tanpa admin: PATH otomatis + asosiasi `.eve` "Evernight files") dan `uninstall.ps1`/`uninstall.cmd`. Terverifikasi: klik-ganda `.eve` menjalankan program, idempoten, uninstall bersih.
- **Integrasi Otomatis Ekstensi (Fase 6C)**: Installer mendeteksi 6 editor keluarga VS Code (Antigravity, VS Code, VS Code Insiders, Cursor, Windsurf, VSCodium) dan **menawarkan** pemasangan ekstensi (`y/t`), lalu memasangnya via `--install-extension --force`. Opsi `-TanpaEkstensi`/`-TanpaKonfirmasi`. Uninstaller menawarkan pencopotan ekstensi (`-SimpanEkstensi` untuk mempertahankan). Dokumentasi paket (`PANDUAN.txt`, `BACA-AKU.txt`) diperbarui.
- **Pengujian & Kualitas (Fase 5)**:
  - Pembagian unit test modular per modul (`tests/test_*.rs`).
  - Golden test suite end-to-end (`tests/golden/`).
  - Uji seluruh kode error fatal & warning (`error_codes_test.rs`).
  - Fuzz testing parser anti-panic deterministik 15.000 iterasi (`fuzz_parser.rs`).
  - Benchmark baseline zero-dependency (`benches/benchmark.rs`).
  - Dokumentasi pengujian & coverage lengkap (`TESTING.md`).

### Diperbaiki
- Penanganan escape backslash `\` pada string lexer ketika mencapai akhir berkas (EOF) yang memicu index out of bounds.
- Batas kedalaman rekursi sintaks parser (`MAX_RECURSION_DEPTH = 64`) untuk mencegah stack overflow pada kurung/operator bersarang ekstrim.
- Bug penjelajahan variabel lokal top-level vs function-level di compiler yang menyebabkan variabel lokal fungsi tanpa parameter sempat dipetakan sebagai global.
- Format variadik pada `format()` untuk string.

---

## Riwayat Rencana (Struktur 8 Fase)

| Fase | Konten | Status |
|------|--------|--------|
| Fase 1 | Desain Bahasa (keyword, grammar, tipe, error, stdlib) | ✅ selesai |
| Fase 2 | Implementasi Inti (lexer→parser→compiler→VM, error handling, 36 test) | ✅ selesai |
| Fase 3 | Standard Library + Identitas Visual (logo, icon `.eve`, brand kit) | ✅ selesai |
| Fase 4 | REPL & CLI + Highlighting Editor (tema "Nusantara", VS Code ext) | ✅ selesai |
| Fase 5 | Testing & Kualitas Internal (modular unit test, golden, fuzz, bench, TESTING.md) | ✅ selesai |
| Fase 6 | Tooling Lanjutan & Distribusi Sistem (LSP, formatter, installer mandiri, PATH) — 6A ✅, 6B ✅, 6C ✅, 6D–6F ⬜ | 🚧 aktif |
| Fase 7 | Stabilisasi & Rilis 1.0 (audit keamanan, bytecode stabil) | ⬜ |
| Fase 8 | Dokumentasi Publik, Website & Komunitas (web WASM, tutorial, Marketplace) | ⬜ |
