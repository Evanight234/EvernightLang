# LOGO.md — Spesifikasi Logo & Identitas Visual EvernightLanguage

> File ini adalah **brief & spesifikasi** untuk desain logo resmi dan icon `.eve`.
> **Logo didesain oleh USER** (bukan AI). Tugas opencode: menyiapkan spesifikasi, checklist, dan menempatkan file hasil desain.
> Bagian yang perlu diisi user ditandai dengan `_`.

---

## 1. Ringkasan (TL;DR)

| Aspek | Spesifikasi |
|-------|-------------|
| Nama | EvernightLanguage (ekstensi `.eve`) |
| Gaya | Logo simbol/ikonografik (bukan cuma teks), mudah dikenali ukuran 16px |
| Varian | Terang + gelap + monokrom |
| Format final | `logo.svg`, `logo.png` (transparan), `icon_*.png` (16/32/48/256), `icon.ico` |
| Penerapan | README, terminal, icon file `.eve` di editor (VS Code), brand kit, situs web |
| Pembuat | User (file `logo1.png`, diproses opencode) |

## 2. Genesis Konteks

- Bahasa pemrograman **Bahasa Indonesia**: "malam kekal" / "malam abadi" adalah makna nama **Evernight**.
- Filosofi produk: *se-mudah Python, se-cepat C++, se-multifungsi JavaScript*.
- Tema warna editor Fase 4: **"Nusantara"** (lihat `TEMA.md`) — palet warna Nusantara mustahil terpisah dari logo.
- **Inspirasi desain:** logo **JavaScript** (badge/kotak solid berisi huruf) — dipakai huruf **"E"** sebagai penghormatan pada JS (keputusan user 2026-09-10).

## 3. Arah Desain

| Dimensi | Keputusan |
|---------|-----------|
| Bentuk | Kotak/badge solid (gaya logo JS) berisi huruf **"E"** serif |
| Makna | "E" = Evernight; bentuk kotak = penghormatan pada logo JavaScript (filosofi *se-multifungsi JavaScript*) |
| Gaya | Flat, komposisi penuh (full-bleed), huruf gelap di atas krem |
| Warna utama | Krem `#F9F5F0` (latar) |
| Warna aksen | Disesuaikan palet `TEMA.md` (menyusul) |
| Font (jika ada teks) | _ (huruf E serif buatan tangan user) |

Catatan desain teknis:
- File sumber user `logo1.png` 500×500, krem solid `(249,245,240)` + E gelap `(31,33,34)`.
- **Stroke E orisinal tipis** (≈2,4% cakupan) → tidak terbaca di 16px. Solusi: master **tipis** untuk ukuran besar (`logo.png`) + **badge tebal** ter-dilasi (≈39% cakupan) untuk icon kecil (`logo_badge.png`) — pola umum favicon.
- Master **SVG** belum tersedia (sumber raster) — utang spesifikasi, menyusul bila user punya sumber vektor.

## 4. Kriteria Wajib (wajib benar, utk teknik)

1. Logo bisa dikenali pada ukuran **16 px** (icon file `.eve`) — ✅ badge tebal terbaca.
2. Format vektor **SVG** sebagai master — ⚠️ belum ada sumber vektor; raster saat ini.
3. PNG transparan untuk raster: ukuran 512×512 minimal (master), plus icon 16/32/48/256 — ✅.
4. `icon.ico` multi-size (16, 32, 48, 256) — ✅.
5. Warna cocok di latar terang (putih) **dan** gelap (#0b1020) — ✅ (transparan + `logo_monokrom`).
6. Logo bebas lisensi (asli buatan user) — **bebas pakai dengan atribusi** (keputusan user 2026-09-10), siap dipakai di ekstensi VS Code (properties `icon`, `themeIcon`) dan installer.

## 5. Deliverables (folder `assets/logo/`)

- [x] `logo1.png` — arsip sumber asli user (500×500, tidak diubah)
- [x] `logo.png` — 512×512 transparan (master orisinal, E tipis)
- [x] `logo_badge.png` — 512×512 badge tebal (untuk icon/ukuran kecil)
- [x] `logo_monokrom.png` — versi satu warna (E putih transparan, untuk latar gelap)
- [x] `icon_16.png`, `icon_32.png`, `icon_48.png`, `icon_256.png` (dari badge)
- [x] `icon.ico` (multisize 16/32/48/256)
- [x] `preview.png` — lembar preview untuk review user (model AI tak bisa melihat gambar)
- [ ] `logo.svg` — master vektor (menyusul, butuh sumber vektor dari user)
- [ ] `sumber_desain.*` — file sumber desain (opsional, mis. `.ai`, `.fig`, `.kra`)

> `icon.ico` dipakai juga sebagai **icon file `.eve`** di Windows Explorer: tipe file tampil sebagai **"Evernight files"** (ProgID `EvernightFile`, `DefaultIcon` → `icon.ico`; lihat `RENCANA.md` Fase 6 Addendum).

## 6. Checklist Proses (Fase 3)

1. [x] **Brief disetujui** — arah desain terisi (badge gaya JS + huruf E)
2. [x] **Sketsa/konsep** — user kirim `logo1.png` (konsep final langsung)
3. [x] **Review bersama** — user setujui 2026-09-10 (cek `preview.png`; palet vs `TEMA.md` ok)
4. [x] **Digitalisasi** — transparan + monokrom + badge tebal (opencode, PIL zero-dep)
5. [x] **Ekspor icon** — PNG 16/32/48/256 + `.ico` multisize
6. [x] **Klik-test** — keterbacaan 16px terverifikasi programatik; uji akhir di Explorer Windows menyusul di Fase 6 (asosiasi `.eve`)
7. [x] **Brand kit** — final: slogan, font, lisensi, palet di `BRAND.md`
8. [~] Finalisasi — logo final; tersisa: master SVG vektor + uji ikon Explorer di Fase 6

## 7. Slogan (final)

Slogan final + opsi cadangan + font + lisensi + palet resmi + aturan pakai: **lihat `BRAND.md`**.

## 8. Aturan Penggunaan

- Logo tidak boleh dimodifikasi proporsinya.
- Varian warna harus memakai palet resmi brand kit (`BRAND.md`).
- Lisensi: **bebas pakai dengan atribusi** (sebut "logo EvernightLanguage") — keputusan user 2026-09-10.

## 9. Brand Kit

Isi lengkap brand kit sekarang di **`BRAND.md`** (slogan final, font sans-serif, lisensi atribusi, palet warna, aturan pakai, peta aset `assets/logo/`). Bagian ini sengaja dikosongkan agar tidak ada duplikasi.