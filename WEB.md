# Desain Website EvernightLanguage

> Perencanaan desain web publik EvernightLanguage.
> Sumber desain awal: **Google Stitch** (2 layar) + keputusan user 2026-09-22.
> Status: **DIBANGUN** — `situs/` hidup lokal; revisi desain menyusul di §9.

## 0. Sumber Desain (Google Stitch)

**Project**: `18139361705705476795` — "Desktop Setup Wizard Interface"

| Layar | Screen ID | Judul |
|-------|-----------|-------|
| Homepage | `9457a23a05ce4a81afee1ea614faa8a4` | Evernight Documentation Homepage - Full-bleed Art |
| Dokumentasi | `e8b358060d224961925a674488f60374` | Evernight Documentation Page |

Kedua layar sudah ada HTML export (Tailwind CDN). Konten placeholder Stitch **bukan sintaks Evernight asli** — wajib diganti sebelum jadi situs.

## 1. Konsep

- Tujuan: wajah publik bahasa EvernightLanguage untuk pengguna & komunitas
- Audiens: programmer Indonesia (pelajar, pemula, profesional)
- Tone: santai profesional; **dwibahasa** — menu switch Indonesia/Inggris di navbar
- Karakter: modern dark untuk hero (kesan "engine"), light untuk dokumentasi (nyaman baca)

## 2. Visual

### Palet (dari desain Stitch)
| Token | Hex | Pemakaian |
|-------|-----|-----------|
| brand purple | `#7F77DD` | aksen utama, aktif nav, logo, tombol |
| lavender | `#C9B8FF` | teks aktif, highlight kode |
| dark bg | `#15151B` / `#0F0F14` | latar homepage |
| banner | `#F1EDFB` | footer banner, kartu intro, active sidebar |
| plum text | `#2E2A3D` | teks di banner terang |
| terracotta | `#A6522F` | link aksen di banner |
| surface light | `#FCF8FF` / `#FAFAF8` | latar dokumentasi |
| code panel | `#15151E` | panel kode gelap |
| outline | `#E6E6EA` | border kartu (light mode) |

### Font
- Homepage: **Inter** (body) + **JetBrains Mono** (kode)
- Dokumentasi: **Geist** (body/UI) + **JetBrains Mono** (kode)
- (Gabungkan: Inter/Geist + JetBrains Mono — pilih satu sans saat implementasi)

### Aset repo
- `assets/logo.png`, `logo_badge.png`, `icon_*.png`, `preview.png`
- Referensi visual: `assets/evernight instaler models.jpeg`
- Brand kit: `BRAND.md`; tema editor: `TEMA.md`

## 3. Struktur Halaman

### 3.1 Homepage (dark, full-bleed)
- **Navbar**: logo "E" ungu + wordmark Evernight | Home (aktif) / About / Documentation | **Ganti Bahasa (switch)** ← tambahan wajib
- **Hero**: background art + overlay gelap; kiri = frosted-glass code editor (tab `main.eve`, traffic lights, UTF-8); kanan = headline + paragraf + pagination 1–5
- **Footer banner**: lavender + tagline + link "Learn more" (terracotta)
- Domain mockup di browser chrome: `evernight-lang.org`

### 3.2 Dokumentasi (light, 20/80)
- **Nav atas**: Home / About / Documentation (aktif = underline ungu)
- **Sidebar 20%** — grup + item (**disesuaikan fitur asli Evernight**, lihat §4)
- **Konten 80%**: judul halaman + pil ‹ Home / Next › + kartu intro "Learn Evernight" + bagian "Why..." + panel kode gelap (tab `main.eve` + tombol Copy) + kartu fitur
- **Chip versi di sidebar footer**: `v0.1.0`

### 3.3 Peta halaman (roadmap konten)
| Halaman | Isi | Sumber di repo |
|---------|-----|----------------|
| Beranda | Hero + quickstart + CTA unduh | `examples/hello.eve`, `paket/` |
| Panduan | Keyword per tabel + contoh | `KEYWORD.md`, `GRAMMAR.md` |
| Tutorial | Latihan bertahap | `examples/` |
| Referensi | 7 modul stdlib | `STDLIB.md` |
| Contoh | Galeri program | `examples/*.eve` |
| Unduh | Paket per versi + SHA256 | `paket/`, `version/` |
| Komunitas | Roadmap, tantangan | `RENCANA.md` |

## 4. Menu Sidebar (final — fitur asli Evernight)

Ganti menu placeholder Stitch (Pattern Matching, Concurrency, Memory Model, Optional Types, Modules & Interop — **tidak ada** di bahasa):

```
GETTING STARTED
  Evernight Home
  Introduction
  Installation
  Quickstart

LANGUAGE CORE
  Syntax & Keywords
  Variables & Types
  Functions
  Conditionals          (jika / lainnya)
  Loops & Iteration     (selama / untuk)
  Error Handling        (coba / tangkap / lempar)
  Imports & Modules     (impor)

REFERENCE
  Standard Library      (7 modul: konsol, string, matematika, daftar, kamus, sistem, utilitas)
  CLI Reference         (run, format, lint, pkg, system, --debug, --waktu)
  Examples              (examples/*.eve)
```

## 5. Switch Bahasa (Ganti Bahasa)

**Lokasi**: navbar atas, pojok kanan (setelah link nav). Hanya **homepage** dulu (instruksi user); halaman docs menyusul saat implementasi.

**Bentuk**: toggle switch — track pil horizontal + knob bulat (bunderan).

| State | Posisi knob | Warna track (split horizontal) |
|-------|-------------|-------------------------------|
| **Indonesia (ID)** | **kanan** | **merah atas / putih bawah** (bendera Sang Saka) |
| **Inggris (EN)** | kiri | **hitam atas / putih bawah** |

- Label: teks **"Ganti Bahasa"** di kiri switch (atau ikon globe + switch)
- Default tampil di mockup homepage: **state ID aktif** (knob kanan, merah-putih)
- State EN: dibuat sebagai varian layar terpisah (`stitch_generate_variants`) agar kedua state terdokumentasi

## 6. Koreksi Wajib Sebelum Implementasi

Konten placeholder Stitch ≠ Evernight asli. Checklist koreksi:

- [ ] **Sintaks kode**: ganti Rust/Go (`pub fn`, `let`, `match`, `module app;`, `fun main()`) → Evernight (`fungsi`, `variabel`, `jika`, `cetak`, tanpa `;`)
- [ ] **Contoh kode homepage** (target):
  ```
  fungsi faktorial(n) {
      jika n <= 1 {
          kembali 1
      }
      kembali n * faktorial(n - 1)
  }
  fungsi utama() {
      cetak("Faktorial(5) = ", faktorial(5))
  }
  ```
- [ ] **Contoh kode docs** (target): contoh kecil `fungsi`/`variabel`/`cetak` yang benar
- [ ] **Versi**: `v0.18.4` → **`v0.1.0`** (chip sidebar docs)
- [ ] **Ekstensi file**: `main.ev` → **`main.eve`** (tab kode docs; homepage sudah `.eve`)
- [ ] **Sidebar**: ganti item placeholder → struktur §4
- [ ] **Navbar homepage**: tambah switch Ganti Bahasa §5
- [ ] Teks marketing (Why Evernight dsb.) boleh dipertahankan, nanti diterjemahkan saat mode ID

## 7. Framework (FINAL — keputusan 2026-09-22)

### Keputusan
| Aspek | Pilihan |
|-------|---------|
| Framework | **Astro** (static output, zero-JS default) |
| Styling | **Tailwind** (compat ekspor Stitch HTML) |
| Konten MD | **File MD root repo tetap** (`KEYWORD.md`, `STDLIB.md`, dll.) — tidak diduplikasi ke `situs/` |
| Bahasa | **Toggle client-side, URL tidak berubah** (tanpa `/id/`, `/en/`) |
| Dokumentasi EN | **(a) UI chrome saja** yang switch (navbar, tombol, judul); konten docs tetap ID (MD root) — opsional blok terjemahan nanti |
| Hosting | GitHub Pages (`dist/` via Actions) |
| Non-goal 1.0 | Playground WASM, search penuh, CMS |

### 7.1 Struktur folder `situs/`
```
situs/
├── astro.config.mjs          # tailwind integration, site URL
├── package.json              # astro, @astrojs/tailwind, tailwindcss
├── public/assets/            # logo, icon (symlink/copy dari ../assets)
├── src/
│   ├── layouts/Base.astro    # navbar + switch bahasa + footer
│   ├── layouts/Docs.astro    # sidebar 20% + konten 80%
│   ├── pages/
│   │   ├── index.astro       # homepage (dari ekspor Stitch)
│   │   ├── unduh.astro
│   │   └── docs/[...slug].astro  # render MD → halaman docs
│   ├── components/           # Navbar, Sidebar, SwitchBahasa, CodePanel, KartuFitur
│   ├── content/
│   │   ├── docs/             # hubungan ke MD root (lihat §7.3)
│   │   └── i18n/
│   │       ├── id.json       # teks UI Indonesia
│   │       └── en.json       # teks UI Inggris
│   └── styles/global.css
```

### 7.2 Routing & i18n (tanpa ganti URL)
- URL tunggal: `/`, `/docs/sintaks`, `/unduh` — sama untuk ID & EN
- Mekanisme:
  - `<html data-lang="id">` (default; `localStorage.evernight_lang`)
  - Elemen terjemahan: `<span data-i18n="nav.unduh">` → JS ganti teks dari `id.json`/`en.json`
  - Konten MD = Bahasa Indonesia; switch hanya ubah UI chrome (lihat keputusan di atas)
  - Switch component: sesuai §5 (ID: knob kanan merah-putih; EN: kiri hitam-putih), tulis `localStorage`, sinkron semua tab
- **Tidak ada** middleware/redirect/`hreflang` path

### 7.3 Konten MD root → Astro
- **Astro Content Layer** (`glob()` loader) menunjuk ke `../KEYWORD.md` dsb. — file MD **tidak dipindah/diduplikasi**
- Frontmatter minimal: judul, slug, grup sidebar
- Mapping awal (sesuai §4):
  - `KEYWORD.md` → Syntax & Keywords
  - `TIPE.md` → Variables & Types
  - `FUNCTION.md` → Functions
  - `STDLIB.md` → Standard Library + modul
  - `ERROR.md` → Error Handling
  - `GRAMMAR.md` → Syntax detail
  - `examples/*.eve` → halaman Examples (code block)
- Fallback jika glob di luar `src/` bermasalah: build step copy-in ke `src/content/docs/` (MD root tetap sumber kebenaran)

### 7.4 Data rilis
- Build time: baca `../version/version` + daftar isi `../paket/` → variabel halaman Unduh + chip versi sidebar (`v0.1.0`)
- Regenerasi otomatis saat commit ke `version/` (GH Action)

### 7.5 Pipeline GitHub Pages
```yaml
# .github/workflows/situs.yml
on: push [main, paths: situs/** | version/** | *.md]
jobs: build (npm ci → astro build) → deploy (actions/deploy-pages)
```
- Output: `situs/dist/` → GitHub Pages dari Actions

### 7.6 Integrasi Stitch
1. Ekspor HTML 2 layar → pecah jadi komponen Astro (`Hero`, `CodeEditor`, `Sidebar`, `SwitchBahasa`)
2. Tailwind CDN → `@astrojs/tailwind` (config di `tailwind.config.mjs`)
3. Koreksi §6 (sintaks, v0.1.0, `.eve`, sidebar) dilakukan saat pemindahan ke komponen — `stitch_edit_screens` jadi opsional

### 7.7 Tahapan eksekusi — **CLEAR DIBERIKAN, EKSEKUSI SELESAI 2026-09-25**
1. [x] Scaffold `situs/` (Astro 5.18 + Tailwind 4.3 + struktur §7.1)
2. [x] Port homepage Stitch → `index.astro` + `SwitchBahasa` (state ID/EN, 5 slide, kode faktorial asli)
3. [x] Content layer MD root + `docs/[...slug].astro` + sidebar §4 (+ plugin `plugins/remark-md-link.mjs` ubah link `.md` relatif → `/docs/slug/`)
4. [x] Halaman Unduh (baca `version/version` + `paket/SHA256SUMS.txt` via `process.cwd()` — `import.meta.url` tidak reliabel saat build)
5. [ ] GH Action deploy Pages (**belum**)
6. [ ] (Opsional) `stitch_edit_screens` — mockup Stitch biarkan apa adanya (**belum**)

**Status build 2026-09-25**: `astro build` → **31 halaman** (1 beranda + 1 unduh + 29 docs: 22 file `docs/` baru + 7 MD root), preview 200 OK, 0 link `.md` nyasar. Konten docs (panduan 7 + tutorial 10 + memulai 5) ditulis subagent, seluruh contoh kode **diverifikasi jalan** via `evernight.exe`.

**Catatan implementasi (pelajaran)**:
- ID file MD root di glob loader **lowercase** (`KEYWORD.md` → `keyword`) — `petaSlug` wajib kunci huruf kecil.
- `{` mentah di template `.astro` (mis. blok kode) = ekspresi JS → pindahkan ke frontmatter + `set:html`.
- Nav docs v1: Beranda | Tutorial | Dokumentasi | Unduh (About digabung ke docs/pengenalan).
- Konten `.md` di dalam halaman docs memakai h1 pertama disembunyikan CSS (judul dirender layout dari frontmatter `judul`).

## 8. Langkah Eksekusi Stitch (setelah planning disetujui)

1. `stitch_edit_screens` Homepage — tambah switch §5 + kode Evernight §6
2. `stitch_edit_screens` Docs — versi, `.eve`, sidebar §4, kode Evernight §6
3. `stitch_generate_variants` Homepage — state EN switch (opsional)
4. Verifikasi `stitch_get_screen` → screenshot kedua layar
5. Update MD ini (tandai selesai) + `KONTEKS.md` + commit/push
6. Ekspor HTML → mulai situs di repo (Fase 8)

---

---

## 9. Revisi Desain Menyeluruh (keputusan user 2026-09-26)

Berlaku di SEMUA halaman (Beranda, Dokumentasi, Unduh):

- **Navbar**: latar hitam `#15151B` di semua halaman (termasuk docs/unduh). Logo = gambar `logo_badge.png` (32-36px, rounded) + "Evernight" tebal putih — **bukan** kotak "E" CSS. Menu: **Beranda | Dokumentasi | Unduh** (menu Tutorial DIHAPUS dari navbar; sidebar docs tetap punya grup Tutorial). Aktif = teks `#C9B8FF` + garis bawah `#7F77DD`. Tanpa pencarian.
- **Palet tambahan**: ungu tua `#534AB7` (hover/aksen), teal `#0F6E56` (badge Live, aksen benar), krem `#FAEEDA`, pink lembut `#ED93B1` (nama fungsi), latar panel kode `#15151B`–`#1A1E33`.
- **Footer**: HANYA satu baris tengah "© 2026 EvernightLanguage · Lisensi MIT" — tagline dihapus.
- **Beranda**: badge `v0.1.0 · Windows x64` dihapus. **Tanpa kotak angka 1-5 sama sekali** — slideshow sepenuhnya **otomatis tiap 4 detik** (judul + paragraf + KODE contoh di panel ikut berganti; 5 contoh kode berbeda, loop terus, tanpa kontrol manual). String kode teal, keyword ungu muda, angka amber, fungsi pink, komentar abu. Tanpa glow (flat).
- **Unduh**: badge versi + subjudul dihapus; hero langsung ke judul, lalu dua kartu.
- **Template tahap tutorial** (urutan): judul/isi + blok kode dengan tombol **Jalankan** → section **"Coba Kamu Run"** (kartu border 10px, header + badge Live teal, editor gelap `#17151E` contenteditable + kursor berkedip + border terang, tombol ungu "Coba Kamu Run ▶", panel terminal hitam pekat `$ evernight main.eve` + output) yang **dipindahkan JS sebelum section Latihan** → Latihan → section **"Uji Pemahaman"** (kuis ungu: subteks, progres "Soal 1 dari 4", 4 opsi radio flat, tombol "Kirim Jawaban »", skor + ulangi; data `situs/src/data/quis.ts` 10 tahap x 4 soal) → navigasi sebelumnya/berikutnya di bawah.
- **Gaya**: flat, minim shadow, tanpa neon/glow, radius 8-10px.

Status §9: **SELESAI diimplementasikan 2026-09-26** (build 31 halaman, preview 200; kotak angka dihapus + interval 4 detik pada revisi lanjutan hari yang sama). Run "Coba Kamu Run" kini memakai **mini-interpreter JS** (`situs/src/lib/eve-run.mjs`, cek `situs/scripts/cek-eve-run.mjs`) yang mengeksekusi teks terkini di editor — pengganti aslinya `evernight_wasm` menyusul. Editor = overlay textarea + highlight dinamis + tombol reset. Blok "Output yang diharapkan" semua tutorial terverifikasi vs `evernight.exe` (checker `situs/scripts/cek-output-tutorial.mjs`; satu kesalahan di Tahap 4 diperbaiki).
