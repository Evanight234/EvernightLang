# Desain Website EvernightLanguage

> Perencanaan desain web publik EvernightLanguage.
> Sumber desain awal: **Google Stitch** (2 layar) + keputusan user 2026-09-22.
> Status: **PLANNING** — implementasi HTML menyusul.

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

## 7. Teknis (implementasi nanti)

- HTML polos + CSS (Tailwind CDN dari export Stitch boleh dipakai awal; evaluasi sebelum production)
- Hosting: GitHub Pages (branch `main` / folder `docs/`)
- Playground WASM: **eksperimen, ditunda pasca-1.0**
- Pipeline: tiap rilis → update versi + daftar paket di halaman Unduh dari `version/` + `paket/`

## 8. Langkah Eksekusi (setelah planning disetujui)

1. `stitch_edit_screens` Homepage — tambah switch §5 + kode Evernight §6
2. `stitch_edit_screens` Docs — versi, `.eve`, sidebar §4, kode Evernight §6
3. `stitch_generate_variants` Homepage — state EN switch (opsional)
4. Verifikasi `stitch_get_screen` → screenshot kedua layar
5. Update MD ini (tandai selesai) + `KONTEKS.md` + commit/push
6. Ekspor HTML → mulai situs di repo (Fase 8)
