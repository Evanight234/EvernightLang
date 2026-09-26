# PROGRES.md — Log Kemajuan Proyek

> Update file ini SETIAP kali menyelesaikan satu task.
> Format: tanggal — task selesai — catatan.

## Status Saat Ini

- Fase aktif: **Fase 6 — Tooling Lanjutan & Distribusi Sistem** — **6H ✅ + 6I ✅**. Berikutnya: **Fase 7 (Stabilisasi & Rilis 1.0)** — **DITUNDA** atas permintaan user.
- Fase selesai: Fase 1 (Desain Bahasa), Fase 2 (Implementasi Inti), Fase 3 (Standard Library + Identitas Visual), Fase 4 (REPL & CLI + Highlighting Editor), Fase 5 (Testing & Kualitas Internal)

## Checklist Progres

### Fase 1: Desain Bahasa
- [x] Definisikan keyword Bahasa Indonesia
- [x] Buat grammar formal (BNF/EBNF)
- [x] Desain tipe data
- [x] Desain sistem error
- [x] Desain standard library

### Fase 2: Implementasi Inti
- [x] Setup proyek Rust (Cargo Workspace: `evernight_core`, `evernight_vm`, `evernight_cli`)
- [x] Lexer (tokenizer)
- [x] AST node types (dipaket dengan parser & kompilasi penuh workspace)
- [x] Parser (AST generator, Pratt/precedence climbing, zero-deps)
- [x] Bytecode compiler (AST → bytecode) — fungsi, if/elif/else, while + break/continue, var, aritmetika
- [x] VM (bytecode executor) — stack VM + call frames, fungsi rekursif berjalan
- [x] Environment (scope variabel) — locals (slot/frame) + globals (HashMap)
- [x] Runtime objects — `Value` ARC (Angka/Teks/Bolean/Kosong/Daftar/Kamus/Fungsi)
- [x] Error handling system (handler `coba/tangkap/lempar/pastikan`, kode error spesifik `VARIABLE`/`DIVISION`/`TYPE`/`INDEX`/`KEY`/`FUNGSI`/`RUNTIME`/`ASSERT` + `PERINGATAN` waktu kompilasi `WKVAR`/`WKREACH`)
- [x] Implementasi murni Rust — **C dibatalkan permanen** (FFI `c_interop/` dihapus dari proyek) + setup TypeScript `editors/vscode`

### Fase 3: Standard Library + Identitas Visual
- [x] Modul konsol (cetak/baca) — finalisasi builtin vs `impor konsol` (baca/baca_angka/bersihkan GLOBAL, impor opsional)
- [x] Modul string (besar/kecil/bersih/potong/pecah/gabung/ganti/mengandung/mulai_dengan/akhir_dengan/ulang_teks/format)
- [x] Modul matematika (akar/pangkat/bulat_bawah/bulat_atas/pembulatan/bundar/mutlak/abs/acak_antara/log/faktorial/min/max/sin/cos/tan/pi/e)
- [x] Modul daftar (tambah/sisip/hapus/urutkan/balik/unik/jumlah/rata_rata/gabung_larik/iris/cari/ada/peta/saring/lipat/setiap)
- [x] Modul kamus (kunci/nilai/pasangan/ada_kunci/hapus_kunci/dapatkan/setel/gabung_objek) + statement `hapus`
- [x] Modul sistem (baca_file/tulis_file/ada_file/env(nama,bawaan?)/atur_env/waktu_sekarang/tanggal_sekarang(ts?)/format_tanggal/tunda/selisih_waktu) — handle API & `argumen()` DITUNDA
- [x] Modul utilitas (adalah_angka/teks/daftar/kamus + ke_boolean/e_boolean/bolean + ke_larik/daftar + salin + angka/teks/kamus) — sinkron `STDLIB.md` (7 kategori)
- [x] Sistem `impor` lintas berkas (lazy-load, cache modul, deteksi circular) — v1: `impor "modul" [sebagai alias]`, ekspor sebagai kamus namespace
- [x] Kode error per modul stdlib — sinkron `ERROR.md` (FUNGSI/MATH/JUMLAH/RUNTIME/ASSERT ditambah; TYPE dirapikan; STACK/MEMORY bertanda rencana)
- [x] Unit test tiap modul stdlib
- [x] Brief & spesifikasi logo resmi (konsep, makna, varian terang/gelap, SVG/PNG) — `LOGO.md` (arah terisi: badge gaya JS + huruf E)
- [x] Desain logo oleh user → review → finalisasi — user kirim `logo1.png`, diproses (`assets/logo/`); user setujui 2026-09-10; master SVG menyusul (utang teknis)
- [x] Icon file `.eve` (16/32/48/256px + `.ico` untuk editor & asosiasi file) — siap di `assets/logo/`
- [x] Brand kit mini (warna brand, slogan, font, lisensi) — final di `BRAND.md` (slogan user 2026-09-10)
- [x] Pasang logo ke README/dokumen (placeholder sampai final) — `README.md` memakai logo

### Fase 4: REPL & CLI + Highlighting Editor
- [x] REPL interaktif (`repl.rs`: prompt `eve>`, deteksi blok multi-baris, perintah `:bantuan`/`:keluar`/`:muat`/`:bersihkan`, `run_incremental`)
- [x] CLI runner (`--run`, `run <berkas.eve>`, `--cek`, `--tokens`, `--ast`, `--bytecode`)
- [x] **`evernight coba.eve` — default-run** (tanpa flag langsung jalan; `--run` dan `evernight run` tetap ada)
- [x] Subcommand eksplisit: `evernight run <berkas.eve>` (dengan pesan error `BAHAYA [ARG]` jika berkas tidak disertakan)
- [x] Biner global / PATH: panduan didokumentasikan di README; installer otomatis direncanakan di Fase 6
- [x] Argumen program (`argumen()`, id 75, pemisah `--`, kode keluar)
- [x] Error terminal berwarna + cuplikan baris kode (BAHAYA merah, PERINGATAN kuning, caret `^`, `NO_COLOR`/`--tanpa-warna`)
- [x] `--versi` dan `--bantuan` yang rapi
- [x] Pemetaan token→scope (keyword, string, angka, komentar, fungsi, operator, error) — `TEMA.md` §2
- [x] TextMate grammar `editors/vscode/syntaxes/evernight.tmLanguage.json` — dari `token.rs`
- [x] Tema warna unik Indonesia "Nusantara" gelap (`nusantara-dark.json`) — `TEMA.md` §3.1
- [x] Varian tema terang `nusantara-light.json` — `TEMA.md` §3.2
- [x] Ekstensi VS Code (`package.json`: language `evernight`, grammar, 2 tema, snippet, icon theme)
- [x] Snippet dasar `snippets/eve.json` (23 snippet: fungsi/jika/selama/untuk/coba/cocok/variabel/impor dll)
- [x] Autocomplete statis ~60 item (`extension.ts`: keyword+builtin+modul stdlib, trigger `." "`)
- [x] Icon theme `.eve` (`icons/evernight-icon-theme.json`, pointer ke `assets/logo/icon_16.png`)
- [x] Build `tsc` bersih (0 error); 8 core + 52 vm = 60/60 test hijau (2026-09-12)
- [x] Snapshot test highlighting (contoh `test/snapshots/highlight.eve` → scope token di `highlight.eve.scope`)
- [x] Dokumen cara pasang ekstensi lokal (`editors/vscode/README.md` siap)

### Fase 5: Testing & Kualitas Internal (DEV)
- [x] Unit test per-modul (9 berkas `tests/test_*.rs`: konsol/string/matematika/daftar/kamus/sistem/utilitas/impor/vm_core + `common/mod.rs`)
- [x] Integration test end-to-end + golden test (`tests/golden/` 6 kasus + `golden_tests.rs`)
- [x] Test tiap kode error `BAHAYA` dan `PERINGATAN` (`error_codes_test.rs`, 12 test)
- [x] Fuzz test parser (15.000 iterasi deterministik, anti-panic)
- [x] Benchmark dasar (`benches/benchmark.rs`, zero-dep `std::time::Instant`)
- [x] Laporan cakupan test (`TESTING.md`)
- [x] `CHANGELOG.md` + aturan versi semantik (semver) — `[0.1.0]` terisi

### Fase 6: Tooling Lanjutan & Distribusi Sistem (DEV)
- [x] Static CompletionProvider (VS Code) — autocomplete statis (~60 item: keyword/builtin/modul)
- [ ] Language Server dasar (hover, diagnostic, goto-definition, completion) — **dipindah ke Fase 7**
- [x] Formatter (`evernight format` + `--cek`)
- [x] Linter (aturan gaya + warning `WK*` baru: WKHURUF/WKIMPOR/WKPANJANG/WKPARAM/WKSARANG/WKMATI/WKMAGIS)
- [x] Mode debug (`--debug` trace bytecode; DAP menyusul) — **6D-1 ✅**
- [x] Profiler mini (`--waktu`, hitung opcode) — **6D-2 ✅**
- [x] Package manager lokal (`ever pkg`: init/jalankan/daftar) — **6D-6 ✅**
- [ ] Distribusi Compiler & Bundling ke Sistem (Jalur A: ZIP portabel ✅ 6B; Jalur B: skrip installer PowerShell ✅ 6B; Jalur C: installer GUI Inno Setup — menyusul; PATH otomatis ✅; asosiasi `.eve` Explorer ✅)
- [ ] Integrasi editor opt-out (installer default mengintegrasikan keluarga VS Code, pengguna bisa tolak via checkbox)
- [ ] Publish `crates.io`
- [ ] CI GitHub Actions (build + test + clippy tiap push)
- [ ] Binary rilis (`--release`, checksum, catatan rilis)
- [ ] Optimasi Performa Runtime (bytecode optimization, inline caching, pemangkasan instruksi hot path)

#### Sub-Fase Pelaksanaan Fase 6
- [x] **6A — Paket Ekstensi Siap Distribusi** (LICENSE MIT, package.json lengkap, `.vscodeignore`, ikon, `.vsix` dibuild, terpasang & terverifikasi di Antigravity IDE, README diperbarui)
- [x] **6B — Distribusi Compiler Mandiri** (biner release statis 1.74 MB, paket portabel `paket/evernight-0.1.0-windows-x64/`, `install.ps1`/`install.cmd`, `uninstall.ps1`/`uninstall.cmd`, PATH + asosiasi `.eve` terverifikasi nyata, ZIP 691 KB)
- [x] **6C — Integrasi Otomatis Ekstensi saat Install** (deteksi 6 editor keluarga VS Code, konfirmasi interaktif y/t, `--install-extension --force`, opsi `-TanpaEkstensi`/`-TanpaKonfirmasi`, uninstall menawarkan pencopotan dengan `-SimpanEkstensi`, dokumentasi paket diperbarui)
- [x] **6D — Tooling Tambahan** — 6D-1 debug ✅, 6D-2 profiler ✅, 6D-3 formatter ✅, 6D-4 linter ✅, 6D-6 `ever pkg` ✅ (LSP dipindah ke Fase 7)
- [x] **6E — CI & Rilis Biner** (`.github/workflows/ci.yml` + `rilis.yml`, `git init`, `.gitignore`/`.gitattributes`, `SHA256SUMS.txt`, contoh `.eve` dirapikan agar lolos CI)
- [x] **6F — Installer GUI Windows (egui/Rust)** — crate terpisah `installer/`, panel maskot 28%, palet dari referensi user, font dibundel, bilah judul kustom, **window 1000x640**, **satu box per halaman** (isi box hanya judul+keterangan), **pemilih folder native (`rfd`) dengan tombol Telusuri**, **tanpa jendela CMD** (`windows_subsystem` + `CREATE_NO_WINDOW`), 6 halaman wizard, animasi micro-interaction, per-user/per-machine, `uninstall.exe` bersih, `EvernightLanguage-0.1.0-Setup.exe` 8.37 MB + SHA256

- [x] **6G — Tema Ikon Lengkap + Kompatibilitas 6 Editor** (352 SVG Symbols, tema 98.686 B, label IconStyles, vsix 1.264.065 B, terpasang & ikon tampil di Antigravity)
- [x] **6H — Sistem Uninstaller** — 3 tahap wizard (Konfirmasi/Menghapus/Selesai), checkbox "Hapus ekstensi editor" di luar box, `pasang::copot(hapus_ekstensi)`, progress+log nyata, navigasi back/forward, `kill evernight system` (CMD: `kill.cmd` di bin/; PowerShell: function di `$PROFILE` + `Remove-Item alias:kill`), 21 test hijau
- [x] **6I — Sistem Updater CLI** (`update evernight system`, folder `version/` GitHub + `catatan-<versi>.txt`, pesan zhongk verbatim, tampilkan perubahan sebelum pasang, exe+vsix+reinstall ekstensi, `--cek` dry-run, `evernight system info`, Setup 0.1.0 revisi 9.59 MB, push + uji live OK)
- [x] **Sinkronisasi vsix baru ke payload + paket + rebuild Setup.exe** (vsix 1.264.065 B, ZIP 1.9 MB, Setup.exe 9.56 MB, SHA `dd89006e...`)

### Fase 7: Stabilisasi & Rilis 1.0 (DEV)
- [x] **7A — Audit Keamanan** (sandbox path: `baca_file`/`tulis_file`/`ada_file`/`impor` terkunci ke direktori program, test sandbox ditolak)
- [x] **7B — Stabilisasi Bytecode** (`BYTECODE_VERSION = 1`, golden test 53 opcode v1, test roundtrip + overlap)
- [x] **7C — Ketahanan** (18 edge-case test: aritmatika, string, daftar, kamus, rekursi 50 level, try/catch, loop nol, sandbox ditolak)
- [x] **7D — Kriteria Rilis 1.0** (`KRITERIA_RILIS_1_0.md`: 9 kategori, terukur + tercentang)

#### Sub-Fase Pelaksanaan Fase 7
- [x] **7E — Desain Visual Final Installer + Uji Pengguna Akhir** (grafis wizard bermerek Nusantara, warna & font kustom, UX hak akses/UAC Bahasa Indonesia, tombol "Jalankan REPL"/"Buka Panduan", uji di komputer/VM bersih) — **FINAL** (desain 6F final, tidak ada perubahan visual)
- [x] **7F — Paket Rilis Per-Versi** (skrip `paket-rilis.ps1`, paket `evernight-<versi>-windows-x64/` isi lengkap: exe, kill, update, vsix, docs, LICENSE, **setup/Setup.exe** + ZIP + SHA256, sumber updater 6I)
- [x] **Language Server dasar** (`evernight-lsp` biner JSON-RPC stdio, zero-dep: evernight_core + serde_json, hover/diagnostic/goto-definition/completion/documentSymbol, kompatibel GNU toolchain)

### Fase 8: Dokumentasi Publik, Website & Komunitas (NON-DEV)
- [ ] README utama publik (logo, quickstart, status fase, fitur)
- [x] **Panduan sintaks** — `docs/panduan/01–07` (struktur, variabel, fungsi, percabangan, perulangan, kesalahan, impor) + `docs/kata-kunci` (KEYWORD.md)
- [x] **Tutorial pemula 10 tahap** — `docs/tutorial/01–10` (akhir: mini proyek kalkulator), tiap contoh kode **diverifikasi jalan** pakai `evernight.exe` (exit 0)
- [ ] Referensi stdlib lengkap (penjelasan tiap fungsi + contoh kode) — `STDLIB.md` sudah ada; kolom contoh menyusul
- [x] **10+ contoh program** — `examples/` (11 program) + galeri `docs/contoh.md`
- [ ] Website resmi + playground online berbasis WASM (eksperimen)
  - [x] **Situs Astro DIBANGUN 2026-09-25** (`situs/`) — beranda port desain Stitch (hero bg, showcase kode faktorial, 5 slide, switch ID/EN `localStorage`), 29 halaman docs (sidebar 4 grup + prev/next + chip versi), halaman Unduh (Setup.exe + ZIP + tabel SHA256), i18n id/en JSON. `astro build` **31 halaman**, preview 5 rute 200 OK, 0 link `.md` nyasar (plugin `remark-md-link.mjs`)
  - [ ] Deploy GitHub Pages (Actions) + domain `evernight-lang.org`
  - [ ] Playground WASM (`evernight_wasm`)
- [ ] Publish ekstensi VS Code ke Marketplace (satu `.vsix` untuk keluarga VS Code)
- [ ] `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, template issue/PR
- [ ] Peta jalan (roadmap) v1.x / v2.0
- [ ] Galeri program komunitas + tantangan pertama ("100 baris pertama")
- [ ] Pengumuman & catatan rilis 1.0
- [ ] Survei pengguna pertama
- [ ] Jadwal pemeliharaan pasca-1.0

## Log Aktivitas

- **2026-09-26 (bugfix)**: `/docs/*` 404 = `situs/src/content.config.ts` terhapus → `git restore`. Warning Shiki `eve` hilang: grammar TextMate resmi + tema palet Evernight didaftarkan di `markdown.shikiConfig` (`astro.config.mjs`). Build 31 halaman 0 warning; dev smoke 200.
- **2026-09-26**: **Revisi desain situs menyeluruh (spesifikasi user, `WEB.md` §9).**
  - Navbar global gelap `#15151B` di semua halaman, logo gambar `logo_badge.png`, menu jadi Beranda | Dokumentasi | Unduh (Tutorial dihapus dari navbar), aktif = `#C9B8FF` + underline `#7F77DD`.
  - Footer satu baris copyright (tagline dihapus). Beranda: badge versi dihapus, angka 1-5 jadi **slideshow otomatis 6 detik** (judul + kode contoh ikut berganti, 5 snippet berbeda), string teal, flat tanpa glow. Unduh: badge + subjudul dihapus.
  - Halaman tahap tutorial +2 section: **"Coba Kamu Run"** (editor contenteditable `#17151E` + kursor berkedip + badge Live teal + tombol run simulasi + panel terminal) dipindahkan JS sebelum "Latihan", dan **"Uji Pemahaman"** (kuis 4 soal/tahap, data `situs/src/data/quis.ts` 40 soal diverifikasi subagent) + tombol "Jalankan" pada blok kode pertama.
  - Build 31 halaman, preview 4 rute 200, 14 cek konten hijau. Masih simulasi run (WASM menyusul).
- **2026-09-25**: **Fase 8 — situs Astro + 22 file konten docs DIBANGUN.**
  - Clear diterima → `situs/` scaffold Astro 5.18 + Tailwind 4.3 (install kedua setelah `node_modules` korup/shiki hilang — fix: hapus + install ulang).
  - Beranda port dari HTML export Stitch (`stitch-review/homepage.html`): hero bg `assets/hero.jpg` (diunduh dari URL aida), showcase kode faktorial (highlight manual `tok-*`), 5 slide + pagination, switch ID/EN (knob kanan/merah-putih ↔ kiri/hitam-putih, `localStorage.evernight_lang`, i18n JSON `data-i18n`).
  - Docs: Content Collection glob `../` (22 file `docs/` + 7 MD root), sidebar 4 grup (Memulai/Tutorial/Inti Bahasa/Rujukan, 29 item), prev/next, chip versi dari `version/version`. **Bug diperbaiki**: ID MD root lowercase di glob loader → `petaSlug` kunci kecil (`kata-kunci` dll); baca file via `import.meta.url` gagal saat build → ganti `process.cwd()/..`; link `.md` relatif di docs → plugin `plugins/remark-md-link.mjs`.
  - Konten: subagent menulis `docs/panduan/01–07`, `docs/tutorial/01–10`, `docs/pengenalan|instalasi|quickstart|cli|contoh` — seluruh contoh `.eve` diverifikasi berjalan exit 0. Temuan: komentar `#`, `untuk...sampai` inklusif, `berhenti/lanjut` rusak di `untuk` (bug scope, layak isu), `tangkap` hanya fungsi searah, `d.panjang` (bukan `panjang(d)`), jalur file relatif CWD.
  - Unduh: kartu Cepat (ZIP) + Wizard (Setup.exe), tabel SHA256 dari `paket/SHA256SUMS.txt`, perintah penting. ZIP 0.1.0 rencana `git add -f` (sudah di-gitignore `paket/*.zip`) agar link Unduh hidup.
  - Build final: **31 halaman**, preview `/`, docs, tutorial, unduh, kata-kunci = 200. Belum: GH Pages Actions, domain, playground WASM, edit mockup Stitch (opsional).
- **2026-09-19**: **6G/6H/6I DIDEFINISIKAN + 7E DIBATALKAN.**
  - 7E (poles desain installer) **batal** atas permintaan user — desain 6F final apa adanya; tidak ada perubahan visual.
  - 6G = tema ikon (selesai, tetap). **6H = uninstaller** (hapus PATH+sistem, perintah `kill evernight System`, GUI tunggu mockup Stitch). **6I = updater CLI tanpa GUI** (`update Evernight system`, tarik dari folder GitHub, pesan zhongk jika tidak ada update).
  - Ditemukan: payload installer + paket masih vsix basi 20.797 B (vs 1.264.065 B benar) — Setup.exe kini tidak layak edar; sinkronisasi 5 langkah terjadwal sebelum rilis ke orang lain.
  - **Sinkronisasi SELESAI**: vsix 1.264.065 B → payload + paket; ZIP portabel 1,891,767 byte; `Setup.exe` 9.56 MB, SHA `dd89006e98830dc298fea3098db9bd0784251948689218fb84841ea4162bc7d4`.

- **2026-09-19**: **Ikon editor diperbaiki tuntas + auto-aktif saat install.**
  - Akar masalah kedua: entri `extensions.json` hasil pasang manual memakai path **`/c://c:/Users/...`** (prefix `/c:/` dobel). Editor gagal memuat ekstensi -> tema `evernight-icons` tidak tersedia -> **explorer nol ikon** (semua file). Ini menjelaskan gejala "pakai tema Evernight juga tidak muncul".
  - Perbaikan mesin ini: pasang ulang via CLI editor resmi `--install-extension <vsix> --force` (editor menulis registrasi benar: uuid, path, enabled state). `workbench.iconTheme` sudah `evernight-icons`.
  - **Installer kini auto-aktifkan ikon (fresh install langsung tampil):** `pasang.rs::aktifkan_icon_theme()` menulis `"workbench.iconTheme": "evernight-icons"` ke `settings.json` user tiap editor setelah ekstensi terpasang (peta folder APPDATA per editor). Saat uninstall, `nonaktifkan_icon_theme()` mengembalikan ke `"default"` agar tak jadi "nol ikon". Manipulasi JSON 1 kunci tanpa dependensi; `ganti_icon_theme_nilai()` di-cover 3 unit test.
  - **`install.ps1`/`uninstall.ps1` disamakan:** fungsi `Aktifkan-Ikon` (set ke `evernight-icons`) dan `Reset-Ikon` (set `default`) untuk jalur CLI/distribusi portabel; sintaks 2 skrip lolos parser PS. ZIP paket diregenerasi (758 KB).
  - Verifikasi: **20 test installer hijau**, clippy `-D warnings` bersih; Setup.exe baru di `installer/dist/` (8.37 MB, SHA `da187a3c...`).
  - **[TAMBAHAN SIANG INI]** Tema ikon digabung total: 352 SVG Symbols (249 files + 104 folders) + `_eve` → PNG E. `package.json` label → **IconStyles** (id tetap `evernight-icons`). `build-vsix.ps1` dirombak: patch vsix dasar dengan SEMUA berkas ikon rekursif (bukan 3 entri hardcoded). VSIX baru 1.264.065 byte, 720 entri. **Verifikasi lolos**: semua target SVG ada (0 hilang), tema 98.686 B, `.eve`→`_eve` mapping OK. **VSIX kompatibel 6 editor** (VS Code, Cursor, Windsurf, VSCodium, Antigravity, Theia) — struktur standar, API universal, tanpa native module. Terpasang di Antigravity → **semua ikon kelihatan**.

- **2026-09-18**: **Revisi ikon file `.eve` (Fix penting).**
  - Akar masalah: `editors/vscode/icons/evernight-icon-theme.json` menunjuk `../assets/logo/icon_16.png`, yang relatif terhadap isi `.vsix` = `extension/assets/logo/icon_16.png` — berkas TIDAK ikut terpaket, jadi editor keluarga VS Code gagal memuat ikon dan memakai default.
  - Perbaikan: `icon_16.png` disalin menjadi `editors/vscode/icons/evernight-file.png` (16x16, dibundel), theme JSON menunjuk `./evernight-file.png`. Default `"file"` di hapus agar non-`.eve` tidak ikut jadi ikon E.
  - Alat: `editors/vscode/build-vsix.ps1` baru (patch zip vsix + verifikasi entri). vsce global tidak ada; `npx @vscode/vsce` gagal (npm cache ECOMPROMISED).
  - Pemasangan: vsix dipasang langsung ke `C:/Users/LENOVO/.antigravity/extensions/satriyo.evernight-language-0.1.0` + didaftarkan di `extensions.json` (valid, 15 entri). `workbench.iconTheme` di ubah `vscode-icons` -> `evernight-icons` (keputusan user). Semua antigravity process masih berjalan — perlu restart penuh user.
  - Distribusi disinkron: vsix baru (20.797 byte) di `paket/.../extensions/` + `installer/aset/payload/`; `Setup.exe` dibangun ulang (8.37 MB, SHA `bd370021...`).

- **2026-09-16**: **Revisi desain installer (6F) — 4 perbaikan sesuai permintaan user.**
  - **Window diperlebar** 760x520 -> **1000x640** (minimum 880x580). Panel maskot 28% (~280 px) menyisakan ~720 px konten (+32%). Lebar kolom konten dibatasi `LEBAR_MAKS_KONTEN = 760` agar baris teks tetap nyaman dibaca. Konstanta ukuran kini terpusat di `tema.rs`.
  - **CMD dihilangkan**: akar masalah ditemukan dengan memeriksa header PE — biner ber-`Subsystem = 3` (CONSOLE). Diperbaiki dengan `#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]` sehingga rilis memakai GUI (subsystem 2) sementara build debug tetap bisa menampilkan `eprintln!`. Ditambah `CREATE_NO_WINDOW` (0x08000000) lewat helper `tanpa_jendela()` pada semua `Command`: `reg`, `powershell`, `net`, dan CLI editor.
  - **Pemilih folder native**: dependensi **`rfd` 0.17** (`default-features = false`) + tombol **Telusuri...** di halaman Lokasi Tujuan, membuka dialog folder Windows asli. Kotak teks tetap dapat diketik; ada teks bantuan (folder belum dipilih / sudah ada / akan dibuat).
  - **Satu box per halaman**: halaman **Lokasi Tujuan** (dari 2 kartu -> 1) dan **Siap Pasang** (dari 2 kartu -> 1). Isi box HANYA judul + keterangan (teks). Semua kendali interaktif — radio cakupan, checkbox tugas, input folder + Telusuri, progress bar, dan log — diletakkan **di luar** box. Halaman Memasang: progress/animasi di luar, box berisi catatan/log teks.
  - **Verifikasi**: PE Subsystem = **2 (GUI)** pada installer dan Setup.exe; sweep klik mengonfirmasi dialog rfd terbuka dengan judul "Pilih folder tujuan pemasangan"; hanya jendela `EvernightLanguage Setup` yang muncul (tanpa CMD); siklus install (PATH 31->32, `.eve`, Apps & Features, biner `format`/`lint` jalan) dan uninstall (folder terhapus penuh, PATH->31, registry bersih) tetap benar; **17 test installer hijau**; 6 pratinjau (1000x640) dibuat ulang dan terverifikasi 1 box per halaman.

- **2026-09-16**: **FASE 6 SELESAI SELURUHNYA (6D-3, 6D-4, 6D-6, 6E, 6F).**
  - **6D-3 Formatter** — crate baru `evernight_fmt`. Pendekatan **berbasis token**, bukan AST: lexer membuang komentar, jadi formatter berbasis AST akan menghapusnya. Formatter memproses baris demi baris, mempertahankan komentar, dan **idempoten**. CLI: `evernight format <berkas>`, `--cek` (dry-run, exit 1 bila belum rapi — untuk CI), `--keluar <path>`. 14 test unit; 11 contoh `.eve` terverifikasi tetap valid & tetap menghasilkan output sama (selisih output hanya dari `HashMap` yang tidak deterministik & contoh butuh input, bukan akibat formatter).
  - **6D-4 Linter** — crate baru `evernight_lint` (analisis AST). 7 aturan baru: `WKHURUF` (bukan snake_case), `WKIMPOR` (impor tak dipakai), `WKPANJANG` (fungsi > 50 baris), `WKPARAM` (> 4 parameter), `WKSARANG` (blok kosong), `WKMATI` (kode tak terjangkau), `WKMAGIS` (angka magic). Digabung dengan warning compiler (`WKVAR`/`WKREACH`/`WKFUNG`) di `evernight lint <berkas>`. 11 test.
  - **6D-6 Package manager** — `evernight pkg init | jalankan | daftar`. Parser JSON minimal zero-dep (`json.rs`) untuk manifest `eve.json`. `init` membuat manifest + kerangka `utama.eve`; `jalankan` menjalankan berkas utama (dari manifest / `utama.eve` / satu-satunya `.eve`); `daftar` menampilkan berkas terdaftar + seluruh `.eve` (rekursif, melewati `target`/`.git`/`node_modules`). 7 test.
  - **6E CI & rilis** — `git init` (`main`), `.gitignore` (target/, node_modules, dist, zip; `.vsix` paket sengaja dilacak), `.gitattributes` (LF default; biner ditandai; `.ps1`/`.cmd` CRLF). `ci.yml`: `fmt --check` + `clippy -D warnings` + `test --workspace` + verifikasi 12 contoh `.eve` valid & rapi. `rilis.yml`: build statis `x86_64-pc-windows-gnu`, ZIP, `SHA256SUMS.txt`, artefak, GitHub Release draft. **`.cargo/config.toml` diubah dari `[build]` ke `[target.x86_64-pc-windows-gnu]`** agar `+crt-static` tidak merusak target lain di CI. 12 contoh + 3 golden dirapikan agar lolos CI.
  - **6F Installer GUI (egui)** — crate terpisah `installer/` DI LUAR workspace (agar `+crt-static` tidak bentrok dengan `winit`/`eframe`). Struktur: `src/tema.rs` (semua warna/font/animasi — file milik user), `anim.rs`, `app.rs`, `ui/{komponen,halaman}.rs`, `pasang.rs` (logika sistem). Panel maskot **28% lebar penuh atas-bawah**, maskot di-cutout otomatis dari JPEG referensi user (flood-fill berbasis konektivitas, menghindari lubang di area gelap). Palet dari referensi user (plum/rose) + Plus Jakarta Sans & JetBrains Mono (OFL). **Bilah judul kustom** (dekorasi native dimatikan). **6 halaman**: Selamat datang → Lisensi → Lokasi tujuan → Siap pasang → Memasang → Selesai. **Animasi**: hover 150 ms, tekan 80 ms (skala 0.97), riak klik 400 ms, progress mengalir, denyut langkah aktif 1,6 s, transisi halaman fade+geser 250 ms.
  - **Uji nyata installer (siklus penuh)**: install → PATH 31→32, asosiasi `.eve` aktif (`Evernight files`), entri *Apps & Features* terdaftar (DisplayName/Version/Publisher/UninstallString/EstimatedSize), biner terpasang menjalankan `--versi`/`run`/`format`/`lint`/`debug`; uninstall → folder terhapus penuh, PATH kembali 31, registry & pintasan bersih.
  - **Bug penting yang ditemukan & diperbaiki**:
    1. **BOM UTF-8** — berkas `.eve` dari Notepad/PowerShell gagal di-tokenisasi (`Karakter tidak dikenal: '\u{FEFF}'`). Lexer kini mengabaikan BOM; formatter mempertahankannya. Plus test regresi.
    2. **`UninstallString` salah** — sempat menunjuk `evernight.exe --uninstall`, padahal compiler tidak mengenal flag itu (exit 1). Diganti `uninstall.exe` (salinan installer) — pola standar. Sekaligus installer menyalin dirinya saat pemasangan.
    3. **Kunci berkas saat uninstall** — `uninstall.exe` tidak dapat menghapus folder tempat ia berjalan. Solusi: proses induk menyalin diri ke folder temp, menjalankan pencopotan dari sana, lalu menghapus folder instalasi yang sudah kosong.
    4. **Bantuan global menelan subcommand** — `evernight pkg --bantuan` menampilkan bantuan utama. Kini flag `-h`/`--bantuan` diabaikan bila argumen pertama adalah subcommand.
    5. **Biner basi di paket** — installer sempat membawa compiler lama tanpa subcommand baru, karena fallback memakai `target/release` yang tidak dibangun ulang. `build-installer.ps1` kini memiliki **penjaga biner basi** (membandingkan waktu ubah biner dengan sumber `.rs` terbaru).
  - **Hasil**: `installer/dist/EvernightLanguage-0.1.0-Setup.exe` (8.35 MB) + `SHA256SUMS.txt`; pratinjau 6 halaman di `installer/aset/tinjau/`; flag `--mulai N` untuk meninjau halaman tertentu. Seluruh test workspace + 11 test installer hijau; `cargo fmt --check` bersih; clippy 0 warning.
  - Dokumentasi `RENCANA.md`, `PROGRES.md`, `KONTEKS.md`, `CHANGELOG.md`, `AGENTS.md` disinkronkan.
- **2026-09-14**: **FASE 6D-1 & 6D-2 SELESAI — Mode Debug & Profiler Mini.**
  - **6D-1 (`--debug`)**: `Vm` dapat field `pub debug_trace: bool`; di loop `execute()` setiap instruksi dicetak ke stderr dengan format `[offset] baris N | NamaOpcode`. Ditambah `OpCode::name()` sebagai sumber nama yang dapat dibaca (dipakai bersama profiler). Observasi murni — bytecode & kompiler tidak berubah.
  - **6D-2 (`--waktu`)**: `Vm` dapat field `pub profile_mode: bool` + counter `[usize; 256]` (zero-dep, indeks = byte opcode). `Vm::profile_summary()` mengembalikan `(waktu_ms, total_instruksi, daftar opcode terurut)`. CLI mencetak tabel: waktu eksekusi, total instruksi, dan frekuensi per opcode + persentase.
  - **Interaksi flag**: `--debug`/`--waktu` selalu menjalankan program (tidak dianggap mode inspeksi-only), sehingga bisa digabung dengan `--bytecode`.
  - **Bantuan CLI** diperbarui (`--debug`, `--waktu`).
  - **Test**: 2 test baru di `cli_tests.rs` (`test_cli_profiler`, `test_cli_debug_trace`) + assertion baru di `test_cli_bantuan`. Seluruh workspace hijau; clippy 0 warning.
  - **Perbaikan sampingan**: `tests/golden/01_halo.eve` ternyata tidak punya berkas `.harapan` (membuat `golden_tests` gagal sebelum perubahan 6D) — ditambahkan `01_halo.harapan`.
  - Dokumentasi `RENCANA.md`, `PROGRES.md`, `KONTEKS.md`, `CHANGELOG.md`, `AGENTS.md` disinkronkan.
- **2026-09-14**: **FASE 6C SELESAI — Integrasi Otomatis Ekstensi saat Install.**
  - **Deteksi editor**: fungsi `Cari-Editor` mengenali 6 editor keluarga VS Code (Antigravity IDE, Visual Studio Code, VS Code Insiders, Cursor, Windsurf, VSCodium) lewat CLI di PATH lalu lokasi instalasi standar. **Tidak** memakai keberadaan folder data (`.cursor`/`.windsurf`) karena folder itu bisa ada tanpa aplikasi terpasang.
  - **Alur interaktif**: installer menampilkan editor yang terdeteksi lalu bertanya `Pasang ekstensi EvernightLanguage ke editor di atas? (y/t)`, kemudian menjalankan `<editor-cli> --install-extension <vsix> --force`.
  - **Opsi baru**: `-TanpaEkstensi` (lewati), `-TanpaKonfirmasi` (senyap — dipakai nanti oleh installer GUI 6F).
  - **`uninstall.ps1`**: menawarkan pencopotan ekstensi (`y/t`), opsi `-SimpanEkstensi` (biarkan) & `-TanpaKonfirmasi` (otomatis copot).
  - **Dokumentasi paket**: `docs/PANDUAN.txt` (pemasangan ekstensi kini semi-otomatis + bagian manual sebagai fallback) & `BACA-AKU.txt` diperbarui.
  - **Perbaikan bug**: `$ErrorActionPreference="Stop"` membuat peringatan stderr CLI editor (`antigravityAnalytics ... NOT registered`) dianggap galat fatal walau ekstensi **berhasil** terpasang. Kini `ErrorActionPreference` dilonggarkan sementara di sekitar pemanggilan CLI, dengan deteksi keberhasilan berbasis keluaran + exit code.
  - **Verifikasi nyata**: deteksi menemukan Antigravity (5 editor lain benar dilewati); prompt muncul & berfungsi; `-TanpaEkstensi` melewati langkah; `-TanpaKonfirmasi` memasang otomatis; uninstall `-SimpanEkstensi` mempertahankan ekstensi; `-TanpaKonfirmasi` mencopot ekstensi; PATH 32↔31, registry & folder bersih sesuai kondisi. ZIP paket dibangun ulang (~694 KB).
  - Dokumentasi `RENCANA.md`, `PROGRES.md`, `KONTEKS.md`, `CHANGELOG.md`, `AGENTS.md` disinkronkan.
- **2026-09-14**: **Rencana Installer & Desain Ditambahkan (dokumen).**
  - **Konteks**: alur distribusi saat ini masih memerlukan ekstrak ZIP lalu menjalankan skrip manual — belum praktis untuk pengguna umum.
  - **Tambahan 6F — Installer GUI (Inno Setup 6)**: satu berkas `Setup.exe`, mode per-user (tanpa UAC) / per-machine (UAC) yang dapat dipilih pengguna, wizard Bahasa Indonesia, opsi PATH/asosiasi `.eve`/ekstensi editor/pintasan, uninstaller terdaftar di *Apps & Features*.
  - **Tambahan 7E — Desain Visual Final + Uji**: grafis wizard bermerek (palet Nusantara), warna & font kustom via Pascal, UX hak akses Bahasa Indonesia, tombol "Jalankan REPL"/"Buka Panduan", uji di komputer/VM bersih.
  - **Penundaan**: **6C** (auto-ekstensi berbasis skrip) ditunda — logikanya akan diadaptasi ke installer GUI 6F. **Code signing ditunda** (utang, SmartScreen).
  - Dokumentasi (`RENCANA.md`, `PROGRES.md`, `KONTEKS.md`, `CHANGELOG.md`, `AGENTS.md`) disinkronkan.
- **2026-09-14**: **FASE 6B SELESAI — Distribusi Compiler Mandiri.**
  - **Build release**: `cargo build --release -p evernight_cli` → `evernight.exe` **1.74 MB** (vs 9.79 MB debug). **Terverifikasi statis**: tidak ada dependensi `libgcc`/`libwinpthread`/`libstdc++`; hanya `msvcrt.dll`. Pengguna akhir **tidak perlu Rust/Cargo**.
  - **Paket portabel** `paket/evernight-0.1.0-windows-x64/`: `bin/evernight.exe`, `assets/` (ikon+logo), `docs/PANDUAN.txt`, `extensions/evernight-language-0.1.0.vsix`, `install.ps1`, `install.cmd`, `uninstall.ps1`, `uninstall.cmd`, `BACA-AKU.txt`.
  - **`install.ps1`** (per-user, **tanpa admin**): salin ke `%LocalAppData%\Programs\Evernight\`; tambah PATH user **idempoten**; asosiasi `.eve` di `HKCU\Software\Classes` (ProgID `EvernightFile`, label **"Evernight files"**, `DefaultIcon`, `shell\open\command` = `evernight.exe run "%1"`, entri `Applications\evernight.exe`); `SHChangeNotify` refresh shell; flag `-Uji` (dry-run) & `-TanpaAsosiasi`.
  - **`uninstall.ps1`**: hapus registry, cabut PATH, hapus folder — dengan **penghapusan tertunda** bila berkas terkunci (antivirus).
  - **Verifikasi nyata di sistem**: PATH 31→32 entri; registry lengkap terpasang; `evernight --versi` via PATH → `EvernightLanguage v0.1.0`; `evernight run` → output benar; **klik-ganda berkas `.eve` berhasil menjalankan program** (diuji via Shell + berkas bukti); install ke-2 **idempoten** (tetap 32 entri); uninstall **kembali bersih** tepat ke 31 entri, semua registry & folder terhapus.
  - **ZIP distribusi**: `paket/evernight-0.1.0-windows-x64.zip` (~691 KB).
  - **Keputusan**: uninstaller memakai skrip `.ps1`/`.cmd`, bukan `uninstall.exe` (RENCANA disesuaikan).
  - Dokumentasi (`RENCANA.md`, `PROGRES.md`, `KONTEKS.md`, `CHANGELOG.md`, `AGENTS.md`, `docs/PANDUAN.txt`) disinkronkan.
- **2026-09-14**: **FASE 6A SELESAI — Paket Ekstensi Siap Distribusi.**
  - **Keputusan user**: lisensi proyek = **MIT** (pemegang hak: Satriyo); publisher ekstensi = **Satriyo** (nama asli); editor uji coba = **Antigravity IDE**; eksekusi Fase 6 dipecah menjadi sub-fase 6A–6E dan dikerjakan **urut**.
  - **`LICENSE`** MIT dibuat di root + salinan `editors/vscode/LICENSE`.
  - **`package.json`** dilengkapi: `publisher: Satriyo`, `license: MIT`, `icon`, `repository`/`homepage`/`bugs`, `keywords`, `categories` (Programming Languages/Themes/Snippets).
  - **`.vscodeignore`** dibuat — mengecualikan `src/`, `test/`, `node_modules/`, `*.ts`, source maps dari paket.
  - **`.vsix` berhasil dibuild**: `evernight-language-0.1.0.vsix` (~20 KB) berisi grammar, 2 tema, snippet, icon theme, `out/extension.js`, README, LICENSE.
  - **Terpasang & terverifikasi di Antigravity IDE**: `satriyo.evernight-language-0.1.0` (via `antigravity-ide.cmd --install-extension`). Syntax highlighting `.eve` kini aktif.
  - **README ekstensi** diperbarui: langkah instalasi Antigravity IDE, aktivasi tema Nusantara & icon theme.
  - Dokumentasi (`RENCANA.md`, `PROGRES.md`, `KONTEKS.md`) disinkronkan.
- **2026-09-12**: **FASE 5 SELESAI — Testing & Kualitas Internal.**
  - **5.1 Unit test modular**: `vm_tests.rs` (52 test monolitik) dipecah menjadi 9 berkas `crates/evernight_vm/tests/test_*.rs` (konsol, string, matematika, daftar, kamus, sistem, utilitas, impor, vm_core) + `common/mod.rs` (helper bersama). Semua 52 test tetap hijau.
  - **5.2 VM output capture**: field opt-in `captured_output: Option<Vec<String>>` di `Vm` + method `set_capture_output()` / `take_captured_output()` / `get_captured_output()`. `Cetak` dan `Bersihkan` menulis ke buffer saat aktif, perilaku normal tidak berubah. Test `vm_output_capture` ditambahkan.
  - **5.3 Golden test**: folder `tests/golden/` dengan 6 pasangan `.eve`/`.harapan` (halo, faktorial, perulangan, koleksi, error handling, kondisi/cocok). Runner `golden_tests.rs` memakai library API + output capture (tanpa spawn exe, bebas lock AV). Normalisasi CRLF/LF & whitespace.
  - **5.4 Test kode error**: `error_codes_test.rs` — 12 test memverifikasi `DIVISION`, `TYPE`, `INDEX`, `KEY`, `FUNGSI`, `VARIABLE`, `MATH`, `JUMLAH`, `ASSERT`, `RUNTIME`, `SYNTAX`, `WKVAR`/`WKREACH`.
  - **5.5 Fuzz test**: `fuzz_parser.rs` — 10.000 iterasi token acak + 5.000 raw bytes, LCG deterministik, `catch_unwind`. **Menemukan 2 bug nyata**: (a) index out of bounds di `Lexer::advance()` saat escape `\` di EOF, (b) stack overflow parser pada input bersarang ekstrim.
  - **5.6 Benchmark**: `benches/benchmark.rs` (`harness = false`, zero-dep). Hasil: kompilasi ~31.8 µs, faktorial(20) ~2.0 µs, peta+saring ~6.3 µs, loop 10k ~1.96 ms.
  - **5.7 TESTING.md**: dokumentasi struktur test, cara menjalankan, hasil benchmark, panduan coverage.
  - **5.8 CHANGELOG.md**: versi `[0.1.0]` terisi (Ditambahkan/Diperbaiki) + tabel 8 fase.
  - **Perbaikan bug (temuan fuzzing/test)**: `Lexer::advance()` aman di EOF; `MAX_RECURSION_DEPTH = 64` di Parser (`statement`/`assignment`/`power`/`unary`); pemisahan scope lokal fungsi (`is_function`) vs global; `format()` variadik `{0}`/`{1}`.
  - **Status**: seluruh test workspace **HIJAU**, `cargo clippy --workspace --all-targets -- -D warnings` **0 warning**.
- **2026-09-12**: **Rencana Proyek Diperluas Menjadi 8 Fase.**
  - **Pemisahan Fokus**: Fase 5, 6, dan 7 difokuskan murni pada rekayasa teknis bahasa dan performa/distribusi sistem (Fase 5 = Testing & Kualitas; Fase 6 = Tooling & Distribusi Mandiri; Fase 7 = Stabilisasi & Rilis 1.0).
  - **Fase 8 Baru**: Menampung semua item non-pengembangan bahasa (website resmi WASM, dokumentasi publik, materi ajar 10+ contoh program, promosi Marketplace, dan komunitas).
  - `RENCANA.md`, `PROGRES.md`, `AGENTS.md`, `KONTEKS.md` disinkronkan.
- **2026-09-12**: **FASE 4 SELESAI SELURUHNYA.**
  - **Subcommand `evernight run <file>`**: ditambahkan subcommand eksplisit di CLI; `evernight run` tanpa berkas memberikan error `BAHAYA [ARG]` + panduan bantuan.
  - **Uji Highlighting & Dokumentasi Ekstensi**: snapshot test dibuat di `editors/vscode/test/snapshots/highlight.eve` & `highlight.eve.scope`; panduan instalasi lokal `.vsix` & Development Host lengkap di `editors/vscode/README.md`.
  - **Spesifikasi Distribusi Fase 6**: rancangan pembundelan biner `evernight.exe` mandiri (tanpa Rust/Cargo bagi pengguna akhir) + otomatisasi PATH sistem + asosiasi berkas `.eve` didokumentasikan di `RENCANA.md`.
  - **Status Pengujian**: 10 CLI integration test + 52 VM test + 8 core test = **70/70 test HIJAU**, clippy **0 warning**.
- **2026-09-10**: **Fase 4A SELESAI — CLI Runner & REPL Interaktif.**
  - **CLI Runner**: default-run `evernight <berkas.eve>` tanpa flag, `--run` eksplisit, `--cek` (pemeriksaan sintaks & kompilasi tanpa eksekusi VM), `--tokens`, `--ast`, `--bytecode` (disassembly instruksi VM lengkap), `--versi` / `-v`, `--bantuan` / `-h`, `--tanpa-warna` (juga menghormati `NO_COLOR`).
  - **Argumen Program**: pemisah `--` memisahkan argumen program yang diberikan ke fungsi bawaan `argumen()` (id 75 di VM, mengembalikan `daftar`).
  - **Error Berwarna & Cuplikan Baris**: modul `printer.rs` memformat `BAHAYA [KODE]` merah & `PERINGATAN [KODE]` kuning beserta baris sumber dan penunjuk caret `^` di kolom yang bermasalah.
  - **REPL Interaktif (`repl.rs`)**: dijalankan dengan `evernight` tanpa argumen; prompt `eve> ` dan `... ` untuk multi-baris (deteksi pasangan kurung `{`, `(`, `[` tak seimbang); perintah `:bantuan`, `:keluar`, `:muat <berkas>`, `:bersihkan`; menggunakan `Vm::run_incremental` sehingga variabel & impor persisten di sesi REPL.
  - **Unit & Integration Tests**: 8 test baru di `cli_tests.rs` (versi, bantuan, cek, default-run, argumen, bytecode, file not found, syntax error) + 2 unit test di `vm_tests.rs` (`vm_run_incremental`, `vm_argumen_builtin`) + 3 test di `printer.rs`. Seluruh 68 test workspace **HIJAU**, clippy **0 warning**.
  - `STDLIB.md`, `RENCANA.md`, `PROGRES.md`, `KONTEKS.md` disinkronkan.
- **2026-09-10**: **Fase 3 SELESAI — brand kit final + `BRAND.md` baru.** Keputusan user: slogan = *"Evernight bukan hanya karakter game tapi bisa menjadi pelajaran bahwa menyukai karakter juga bisa menjadi motivasi."*; font = sans-serif; lisensi = bebas pakai dengan atribusi. `BRAND.md` dibuat (satu sumber kebenaran brand: slogan + cadangan, font, lisensi, palet resmi, aturan pakai, peta aset `assets/logo/`). `LOGO.md` §7–§9 diringkas jadi pointer ke `BRAND.md` (anti-duplikasi), checklist item 7 `[x]`. RENCANA/PROGRES/AGENTS brand kit `[x]`, README `[x]`. **Semua item Fase 3 hijau** — utang satu: master SVG vektor (sumber raster saja). Lanjut: **Fase 4** (REPL & CLI + tema Nusantara + ekstensi VS Code keluarga).
- **2026-09-10**: **Fase 3 — LOGO FINAL + brand kit draft + README placeholder + keputusan liputan editor.** User setujui logo (review `preview.png` OK). Item Fase 3 `[~]` difinalisasi: desain `[x]`, icon `.eve` `[x]`, brand kit `[~]` (draft warna `LOGO.md` §9 — font/slogan/lisensi menunggu user), README `[~]` (placeholder baru `README.md` dengan logo). **Keputusan editor (user): keluarga VS Code saja** — `.vsix` tunggal (VS Code/Cursor/Windsurf/VSCodium; Antigravity dicek saat eksekusi), Marketplace satu upload + installer integrasi editor **opt-out** (default YA, bisa tolak), non-goal 1.0: Sublime/Zed/Neovim/JetBrains. Ditulis ke `RENCANA.md` (Fase 4 item ekstensi + icon theme, Fase 6 installer/publish/non-goal, Log Keputusan) + `KONTEKS.md`. Tanpa perubahan kode Rust (58/58 tetap hijau, clippy 0).
- **2026-09-10**: **Fase 3 — LOGO DIPROSES (item desain & icon).** User kirim `logo1.png` (500×500: badge krem `(249,245,240)` + E serif gelap, inspirasi logo JS). Diproses zero-dep (PIL, script di temp): **unpremultiply** krem → `logo.png` (master transparan, E tipis, 512px); **E putih** untuk latar gelap → `logo_monokrom.png`; **badge tebal** via dilatasi mask (MaxFilter 3×3 ×14 iterasi) → `logo_badge.png` (≈39% cakupan vs orisinal 2,4% — stroke orisinal tak terbaca di 16px, badge jawab syarat; pola umum favicon); turunan `icon_16/32/48/256.png` + `icon.ico` multisize; `preview.png` untuk review user. **Klik-test programatik:** badge 16px cakupan 39,5% + ASCII 16px terbaca jelas sebagai "E". Utang: master **SVG vektor** (sumber raster) menyusul. `LOGO.md` §1–6 diupdate, `RENCANA.md` logo → `[x]` brief, `[~]` desain wait-review, `[~]` icon (SVG pending). **Blokir:** review user `preview.png` → finalisasi + brand kit.

- **2026-09-10**: **Fase 3 — kode error stdlib disinkron + unit test lengkap.** EROR.md:** baris `TYPE` (bukan fungsi, arity salah) yang salah terdokumen **dipindah ke kode aktual `FUNGSI`**; §B.5 baru (3 kasus: bukan fungsi `'{}' bukan fungsi`, arity user-fungsi `Fungsi 'kali' butuh 2 argumen, diberikan 1`, arity callback `Fungsi callback harus menerima N argumen`). §G ditambah: `RUNTIME` (lempar kustom, melewati batas bytecode, builtin tak dikenal), `MATH` (akar(-4), log(0)), `JUMLAH` (faktorial negatif/fraksi, maksimal 170, rata_rata([])), `ASSERT` (`pastikan` gagal → "Pernyataan tidak benar!"). `STACK`/`MEMORY` ditandai *(direncanakan — belum di-raise)*. §4 status finalisasi ditambah 2 baris. **Unit test:** test baru `konsol_cetak_bersihkan_nan` menutup 3 celah konsol (cetak→Kosong, bersihkan→Kosong, baca_angka("abc")→BAHAYA [NaN]). **58/58 HIJAU** (8 core + 50 vm), clippy 0. **Fase 3 stdlib + impor + error/admin LENGKAP** — tersisa: Identitas Visual (logo user). `ERROR.md`, `RENCANA.md`, `PROGRES.md`, `AGENTS.md` disinkron.
- **2026-09-10**: **Fase 3 — IMPOR LINTAS BERKAS SELESAI (v1).** Opcode baru `ImporModul` (0x95); compiler mengubah `ImportStatement`: modul standar tetap no-op, selain itu emit `Konstanta(path)` + `ImporModul` + `SimpanGlobalNama(alias/nama-dasar)`. VM: field baru `entry_dir`/`import_cache`/`importing`; `CallFrame` bawa `module_path` + `module_before`; `Henti` kini pop frame dan (bila frame modul) mengumpulkan global baru sebagai `Value::Kamus` ekspor → cache → push. `Vm::with_dir(dir)`; CLI mengoper direktori berkas utama. Modul dijalankan lazy sekali (cache), deteksi siklus → `BAHAYA [FILE]`, berkas hilang → `[FILE]`. Sintaks: `impor "helper" [sebagai h]` → `h.tambah(...)`; subdirektori `impor "sub/helper"`. `dari` ditunda. Test: 7 test baru (`impor_lintas_berkas_alias`, `..._tanpa_alias`, `..._subdirektori`, `impor_modul_diimpor_ganda`, `impor_fungsi_memakai_fungsi_modul_lain`, `impor_sirkular_dideteksi`, `impor_file_tidak_ada`) → **57/57 HIJAU** (8 core + 49 vm), clippy 0. Contoh `examples/import/main.eve` + `helper.eve` diverifikasi via CLI → `Tambah: 5 / Kali: 20 / Kuadrat: 9`. `STDLIB.md`, `GRAMMAR.md`, `ERROR.md`, `RENCANA.md`, `PROGRES.md`, `KONTEKS.md` disinkron. Catatan: modul diekspor sebagai kamus, sisa global modul tetap ada di map global bersama (dokumentasi ponytail).
- **2026-09-10**: **Rencana Fase 6 diperluas — Static CompletionProvider.** Item baru ditambah ke `RENCANA.md` Fase 6: VS Code extension akan punya `CompletionProvider` statis (~60 item: keyword, fungsi bawaan, modul) dengan prefix match otomatis saat user ketik. Mekanisme: `vscode.languages.registerCompletionItemProvider` di `extension.ts`, tanpa LSP. Zero dependency ke compiler/VM. `KONTEKS.md` disinkron.
- **2026-09-10**: **Fase 3 — Modul UTILITAS SELESAI.** 13 builtin (id 62–74). `adalah_angka/teks/daftar/kamus` pemeriksaan tipe (arg hilang → `salah`); `ke_boolean`/`e_boolean`/`bolean` alias `is_truthy`; `ke_larik`/`daftar` konversi ke daftar (teks→karakter, daftar→salinan, kamus→kunci diurut alfabetis); `salin` **deep copy** (rekursif, siklik aman via `HashMap<usize>` visited, fungsi dibagikan — helper free `salin_nilai`, pakai `Rc::as_ptr` untuk identitas); konversi dasar `angka()` (Bolean→1/0, Teks parse gagal → `BAHAYA [NaN]`), `teks()` (semua `to_string`), `kamus()` (klon objek atau Daftar pasangan `[k,v]`, malformed → `[TYPE]`). Deviasi dari STDLIB: `bolean`≡`ke_boolean`, `daftar`≡`ke_larik`. `impor utilitas` no-op. Test: 6 test baru → **50/50 HIJAU** (8 core + 42 vm), clippy 0 warning. Contoh `examples/utilitas.eve` diverifikasi (deep-copy nested, `salin` tak mengubah asli) + regresi 8 contoh lain OK. `STDLIB.md` §F, `RENCANA.md`, `PROGRES.md` disinkron. **Fase 3 modul stdlib LENGKAP (7/7)** — tersisa: impor lintas berkas, kode error/centang admin, lalu Identitas Visual (menunggu logo user).
- **2026-09-10**: **Fase 3 — Modul SISTEM SELESAI.** 10 builtin (id 52–61): `baca_file`/`tulis_file`/`ada_file` (instan via `std::fs`, IO gagal → `BAHAYA [FILE]`), `env(nama, bawaan?)` (argc 1–2, hilang tanpa bawaan → `BAHAYA [ENV]`), `atur_env`, `waktu_sekarang` (ms UTC), `tanggal_sekarang(ts?)` (tanpa arg = sekarang; ISO UTC via algoritma Hinnant zero-dep, bukan `chrono`), `format_tanggal(ts, pola)` (token `YYYY MM DD HH mm ss`, sisanya disalin apa adanya), `tunda(detik)` (desimal, negatif → `[WAKTU]`), `selisih_waktu` (selisih mutlak). Kode error baru `[ENV]` & `[WAKTU]` disinkron ke `ERROR.md`. **Defer (keputusan user):** handle berkas `buka/tutup/baca_baris/tulis` ditunda; `argumen()` ditunda ke Fase 4 (bersama pemisah `--` CLI — main.rs belum diubah). `impor sistem` no-op. Test: 4 test baru (`sistem_waktu_dan_selisih`, `sistem_berkas`, `sistem_env`, `sistem_tunda`) → **44/44 HIJAU** (8 core + 36 vm), clippy 0 warning. Contoh `examples/sistem.eve` diverifikasi (waktu/env/file/tunda + tangkap `[FILE]`). `STDLIB.md` §G, `ERROR.md`, `RENCANA.md`, `PROGRES.md` disinkron.
- **2026-09-10**: **Fase 3 — Modul DAFTAR, KAMUS + statement `hapus` SELESAI.** 16 fungsi daftar (`tambah/sisip/hapus/urutkan/balik/unik/jumlah/rata_rata/gabung_larik/iris/cari/ada/peta/saring/lipat/setiap`, id 28–43) + 8 fungsi kamus (`kunci/nilai/pasangan/ada_kunci/hapus_kunci/dapatkan/setel/gabung_objek`, id 44–51). **Keyword baru `hapus`** (statement): `hapus a[i]` (daftar, indeks buat wrap negatif, out-of-range → `BAHAYA [INDEX]`), `hapus k["kunci"]` & `hapus k.nama` (kamus, kunci hilang diam-diam → `Kosong`); target bukan indeks/kunci → error kompilasi `[SYNTAX]`. Konflik nama `hapus` sebagai fungsi (builtin id 30) diselesaikan: parser memetakan token `Hapus` → `Identifier("hapus")` di `primary()`, dan `hapus_statement` mengembalikan `ExprStatement` bila bentuknya `hapus(daftar, nilai)` (panggilan fungsi). Mutasi daftar in-place (via RefCell → `Kosong`), fungsi lain mengembalikan daftar baru; `urutkan` hanya daftar homogen angka/teks (`[TYPE]`); `jumlah`/`rata_rata` hanya angka (`[TYPE]`, `rata_rata([])` → `[JUMLAH]`); `iris` clamp seperti `potong`; `cari` → indeks atau `-1`; `ada` → `benar/salah`; `dapatkan` argc 2–3 (opsional nilai bawaan, kunci hilang tanpa bawaan → `[KEY]`); `gabung_objek` (kunci b menang). **Higher-order** `peta/saring/lipat/setiap` memakai **pending-callback driver** di VM (`PendingCback`, nested callbacks didukung via stack pending; `lipat([])` → nilai awal; `setiap` → `Kosong`; arity callback salah → `[FUNGSI]`; kesalahan di dalam callback merambat ke `coba/tangkap` pemanggil). Helper `nilai_sama` (perbandingan dalam, rekur tidak dalam fungsi/kamus). `impor daftar`/`impor kamus` no-op. Test: 14 test baru di `vm_tests.rs` → **40/40 HIJAU** (8 core + 32 vm), clippy 0 warning. Contoh `examples/daftar.eve` & `examples/kamus.eve` diverifikasi + regresi `matematika/string/faktorial/fitur_baru` hijau.
- **2026-09-10**: **Fase 3 — Modul MATEMATIKA SELESAI** (fungsi global, keputusan user). 16 fungsi + 2 konstanta: `akar/pangkat/bulat_bawah/bulat_atas/pembulatan/bundar/mutlak/abs/acak_antara/log/faktorial/min/max/sin/cos/tan` + konstanta `pi`/`e`. Pakai **opcode `PanggilBuiltin` yang sama** (id 12–27; total 28 builtin). Keputusan penamaan: **`max`** (bukan `maks`, per STDLIB); alias ganda `pembulatan`/`bundar` dan `mutlak`/`abs` keduanya diterima; `pi` & `e` tersedia global tanpa impor (constant-folding di compiler: `std::f64::consts`). **User-fungsi kini mengalahkan builtin** (fitur baru): `compiler` men-catat nama fungsi deklarasi (`declared_fns`) sebelum kompilasi badan — rekursi `faktorial` user tidak lagi dicegat builtin (test `user_fungsi_mengalahkan_builtin`). Domain error via `self.raise` (kode `[MATH]` akar(-4)/log(0), `[JUMLAH]` faktorial negatif/fraksi/ext 170). `acak_antara` = LCG `u64` thread-local berbasis seed waktu (zero-dependency; swap min>max, rentang inklusif). `impor matematika` no-op. Test Rust ditulis ulang penuh di `vm_tests.rs` (18 vm + 8 core = **26/26 hijau**, clippy 0 warning) — mencakup matematika + regresi string/konsol + faktorial rekursif. Contoh `examples/matematika.eve` diverifikasi.
- **2026-09-10**: **Fase 3 — Modul STRING SELESAI** (fungsi global, keputusan user). 12 fungsi: `besar/kecil/bersih/potong/pecah/gabung/ganti/mengandung/mulai_dengan/akhir_dengan/ulang_teks/format`. Mekanisme **1 opcode generik `PanggilBuiltin` (0x94)** — operand `id(u8)`+`argc(u8)`, dispatch tabel di VM; dipakai ulang seluruh stdlib masa depan (daftar/kamus/matematika/dll). Semua error tipe via `self.raise` (bisa ditangkap `coba/tangkap`). `impor string` no-op (join `konsol`). 7 test string baru → **49/49 test** (8 core + 41 vm) hijau, clippy 0 warning. Contoh `examples/string.eve` diverifikasi: `besar/potong/pecah/gabung/ganti/mengandung/ulang/format` semua benar.
- **2026-09-10**: **Biner resmi = `evernight` + spek asosiasi file final.** `[[bin]] name = "evernight"` di `evernight_cli` → `target/debug/evernight.exe` (crate tetap `evernight_cli`). Terverifikasi: `evernight.exe --run examples/hello.eve` → `Halo Dunia dari EvernightLanguage!`. Keputusan asosiasi Windows: `.eve` → **"Evernight files"** (ProgID `EvernightFile`, `DefaultIcon` = icon.ico, `shell\open\command` = `evernight.exe "%1"`) — Addendum di `RENCANA.md` Fase 6.
- **2026-09-10**: **Fase 3 dimulai — Modul konsol SELESAI** (keputusan user: semua global builtin). `baca(pesan?)` + `baca_angka(pesan?)` (prompt opsional 0/1 argumen; input bukan angka → `BAHAYA [NaN]`), `bersihkan()` (ANSI clear). Opcode baru: `Baca` (kini bawa operand jumlah arg), `BacaAngka` 0x92, `Bersihkan` 0x93. `Vm` punya `input_queue` (`push_input`) untuk test input stdin. `impor konsol`→ no-op tanpa warning (modul lain tetap warning FILE). 6 test baru (total 42 = 8 core + 34 vm) hijau, clippy 0 warning kode. Contoh `examples/konsol.eve` diverifikasi via CLI (piped stdin): `Halo Budi, tahun depan umurmu 26` + ANSI clear. `STDLIB.md` §A diupdate.
- **2026-09-10**: **C dihapus permanen.** `c_interop/` (FFI Rust↔C) keluar dari RENCANA/AGENTS/KONSEP/STRUKTUR/PROGRES. Implementasi = **Rust murni** + **TypeScript** (hanya ekstensi editor). Scaffolding TypeScript dibuat di `editors/vscode/` (`package.json`, `tsconfig.json`, `src/extension.ts`). Item checklist Fase 2 diganti `[x] Implementasi murni Rust`.
- **2026-09-10**: **Audit Fase 2** — verifikasi checklist ke kode aktual: 9/11 item inti terkonfirmasi selesai (lexer/parser/compiler/VM/value/error handling semua ada). Dua temuan: (1) `c_interop/` tetap defer; (2) keyword **`hapus` (del) belum diimplementasikan** (tidak ada di lexer/parser/compiler/VM walau dijanjikan "Fase 2" di tabel keyword). Keputusan (Opsi B): `hapus` dipindah ke **Fase 3** bersama modul koleksi daftar/kamus. `RENCANA.md` (tabel keyword + checklist Fase 3 + log) diupdate.
- **2026-09-10**: **Rencana diperluas dari 5 menjadi 7 fase** (keputusan user). Fase 3 = Standard Library + **Identitas Visual** (brief logo `LOGO.md` untuk user desain, icon `.eve`, brand kit); Fase 4 = REPL & CLI + **Highlighting Editor** (tema unik "Nusantara", grammar TextMate dari `token.rs`, ekstensi VS Code); Fase 6 & 7 baru: Tooling (LSP, formatter, linter, package manager, installer, publish) + Komunitas & Rilis 1.0. Fase 3–7 kini 12–15 item tiap fase. File MD baru dibuat: `LOGO.md`, `TEMA.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`. `RENCANA.md`, `PROGRES.md`, `AGENTS.md`, `STRUKTUR.md`, `KONTEKS.md` disinkronkan.
- **2026-09-10**: **Compiler di-refactor (4 sub-modul) + backend fitur lengkap + error handling + 36 test HIJAU.** `compiler.rs` → `compiler/{mod,codegen,expressions,statements}.rs`. Semua statement berfungsi: `untuk` (range `dari..sampai` + `dalam` daftar/teks), `cocok` (kasus + wildcard `_` + `bawaan`), `coba/tangkap` + `lempar` + `pastikan`, penugasan gabungan (`+=` `-=` `*=` `/=`), assign ke indeks/properti, array/dict literal, lambda. Start VM ditulis ulang dengan sistem handler error + kode `BAHAYA` spesifik; compiler issue `PERINGATAN` (WKVAR, WKREACH) sebelum run. **36/36 test** (8 core + 28 vm integration) hijau, clippy 0 warning. Contoh baru `examples/fitur_baru.eve` (untuk, kamus, daftar, cocok, coba/tangkap, pastikan) berjalan penuh. Perbaikan kunci: `LompatJikaSalah` pop kondisi (buang `Pop` ekstra yang merusak slot lokal), for-loop init sebelum `start_ip`, `LoopInfo.continue_ip`, handler `BuatHandler` di-patch setelah `Lompat skip` (tangkapan), match gunakan `LompatJikaSalah` tanpa `Pop` di lintasan belum-cocok, `SimpanIndeks`/`SimpanKunci` menyimpan + kembalikan nilai, kamus bisa di-indeks `k["x"]`. `evernight_vm` selesai: `value.rs` (Value ARC `Fungsi` ditambahkan), `bytecode.rs` (OpCode 0x00–0xFF + Chunk), `compiler.rs` (AST→bytecode), `vm.rs` (stack VM + call frames; `Panggil`/`Kembali` beneran, rekursi jalan). Contoh diverifikasi: `examples/hello.eve --run` → `Halo Dunia dari EvernightLanguage!`, `examples/faktorial.eve --run` → `Faktorial:120`. `cargo build --workspace` hijau, `cargo test --workspace` 8/8 hijau. Editor: perbaikan besar di compiler (resolve local pakai forward index — bug reversed, global handler kini pakai operand u16 yang benar, `cetak` tidak push dummy, `Henti` return `Kosong`) dan if/elif/else dirombak bersih + scope handling + break/continue.
- **2026-09-10**: Blocker AV (McAfee Framework Host) muncul berkala saat menautkan/menjalankan exe baru (`Access is denied`, os error 5) — transient, retry langsung berhasil. Ini pola yang sama dengan karantina `evernight_cli.exe` (os error 225) sebelumnya.
- **2026-09-09**: **rustc 1.98.1 final berfungsi!** Jalur penyelesaian: unduh MSI `rust-1.98.1-x86_64-pc-windows-msvc.msi` (314MB) via WinGet (berhasil, berbeda dari rustup yang timeout) → ekstrak dengan `msiexec /a` (admin install gagal error 1925) → `rustup toolchain link evn-msvc` → ternyata `rustc_driver-*.dll` (192MB) baru selesai terekstrak belakangan (sempat salah didiagnosis DLL not found). MSVC tetap tak bisa link: butuh `link.exe` (VS Build Tools) + import libs (kernel32.lib dll) yang tidak terpasang.
- **2026-09-09**: **Switch ke toolchain GNU.** `rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal` berhasil (unduhan 4+7 komponen, timeout berkali-kali tapi rustup resume `.partial`). gcc MinGW tidak perlu: pakai linker bawaan `rust-lld` + mode statik `-C target-feature=+crt-static` (sudah diproven berhasil). Terpasang di `.cargo/config.toml` workspace.
- **2026-09-09**: **`cargo build --workspace` & `cargo test --workspace` HIJAU.** 3 unit test lexer lolos (operators, keywords+identifiers, string escapes). `ast.rs` + `parser.rs` (Pratt parser lengkap: ekspresi, pernyataan, `cocok`, `coba/tangkap/akhirnya`, loop, `pastikan`, impor, dll) ditulis & dikompilasi tanpa error.
- **2026-09-09**: **Blocker baru — Antivirus pihak ketiga** (McAfee/Reason Cybersecurity, bukan Defender) mengkarantina `target\debug\evernight_cli.exe` saat dijalankan: "file contains a virus or potentially unwanted software" (os error 225). Test exe di `target\debug\deps\` tidak diblokir. Diperlukan proses allowlist/exclusion AV untuk bisa `cargo run` & REPL. `cargo build`/`cargo test` tetap jalan.
- **2026-09-08**: Toolchain installer `Rustlang.Rustup` (Rustup 1.29.1, Cargo 1.98.1) berhasil dipasang melalui winget. Komponen `rustc` sedang menunggu penyelesaian unduhan stabil dari mirror server.
- **2026-09-08**: Setup Cargo Workspace selesai (`evernight_core`, `evernight_vm`, `evernight_cli`). Implementasi `token.rs`, `errors.rs`, `lexer.rs` (handwritten zero-deps dengan pelacakan baris/kolom) dan wiring `evernight_cli` selesai.
- **2026-09-08**: Keputusan arsitektur implementasi Rust Fase 2 ditetapkan: Cargo Workspace (`evernight_core`, `evernight_vm`, `evernight_cli`), Handwritten Lexer & Pratt Parser, Zero Dependencies, Stack-based VM, Safe ARC Value Model. `STRUKTUR.md` & `RENCANA.md` disinkronkan.
- **2026-09-08**: `LIFECYCLE.md` difinalisasi — 8 tahap runtime (baca→lexer→parser→compiler→VM init→eksekusi→error→cleanup), diagram alur, keputusan desain (hybrid REPL+File, lazy-load impor, ARC+`akhirnya`).
- **2026-09-08**: Wildcard `_` ditambahkan ke pattern matching `cocok` — sinkron ke `KEYWORD.md`, `GRAMMAR.md`, `RENCANA.md`, `KONSEP.md`, `AGENTS.md`. `adalah` (is) & `lewati` (pass) diputuskan tidak perlu.
- **2026-09-08**: `STRUKTUR.md` dibuat — peta lengkap struktur project (Rust implementation). Struktur proyek di `RENCANA.md` diupdate.
- **2026-09-08**: `GRAMMAR.md` selesai dibuat — tata bahasa formal EBNF untuk EvernightLanguage (11 level presedensi operator, seluruh pernyataan, deklarasi, dan contoh program).
- **2026-09-08**: `KEYWORD.md` difinalisasi — kata kunci resmi Fase 1 lengkap (`variabel`, `fungsi`, `kembali`, `impor`, `dari`, `sebagai`, `coba`, `tangkap`, `lempar`, dll).
- **2026-09-07**: `FUNCTION.md` dan pembaruan `STDLIB.md` selesai difinalisasi — mencakup ~35 utilitas fungsi baru, penamaan `besar`/`kecil`, closure, dan penanganan alur `coba`/`tangkap`.
- **2026-09-07**: `STDLIB.md` selesai difinalisasi — perancangan 7 modul standar (`konsol`, `string`, `matematika`, `daftar`, `kamus`, `utilitas`, `sistem`), sistem impor `impor`, dan kontrol berkas.
- **2026-09-07**: `ERROR.md` selesai difinalisasi — spesifikasi format `BAHAYA [KODE]` dan `PERINGATAN [KODE]`, daftar peringatan dan runtime error lengkap.
- **2026-09-07**: `TIPE.md` selesai difinalisasi — spesifikasi tipe data dinamis, model `angka` tunggal, strict type coercion, dan konversi eksplisit.
- **2026-09-07**: File `KONTEKS.md` dibuat sebagai prioritas baca pertama antar-sesi, dan aturan diperbarui di `AGENTS.md`.
- **2026-09-07**: Perubahan arah eksekusi — interpreter tree-walking diganti **compiler bytecode + VM** (model Python/Lua). Fase 2, struktur proyek, dan risiko diupdate di `RENCANA.md`; `AGENTS.md` & `KONSEP.md` disinkronkan.
- **2026-09-07**: `ARRAY.md` difinalisasi — semantik daftar 1:1 dengan Python (indexing 0 + negatif, slicing, method `tambah`/`sisip`/`hapus`/`ambil`, error `BAHAYA` out of bounds).
- **2026-09-07**: `FUNCTION.md` dibuat (kerangka kosong: function, add function, loop function — user isi sendiri).
- **2026-09-07**: `LIFECYCLE.md` dibuat (kerangka kosong, sistem custom — user isi sendiri).
- **2026-09-07**: `KONSEP.md` dibuat (terisi keputusan + rekomendasi konsep). Paradigma diubah: fungsional murni → fungsional + OOP, immutability penuh dihapus.
- **2026-09-07**: File `KEYWORD.md` dibuat sebagai template — user akan mengisi keyword final sendiri, lalu lanjut ke grammar.
- **2026-09-07**: `makefile` dihapus (awalnya untuk rencana compiler, sekarang pakai interpreter murni).
- **2026-09-07**: Nama bahasa ditentukan: **EvernightLanguage**, ekstensi file `.eve`.
- **2026-09-07**: Proyek dimulai. Rencana lengkap dibuat di `RENCANA.md`. Struktur awal: `Cargo.toml` + `src/` (masih kosong). Belum ada kode ditulis.
