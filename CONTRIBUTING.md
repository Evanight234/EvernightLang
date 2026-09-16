# CONTRIBUTING.md — Panduan Kontribusi

> Kerangka awal — **dilengkapi & difinalkan pada Fase 7**. Disarankan disalin dari template komunitas (Contributor Covenant / GitHub).
> Selalu cek `RENCANA.md` dan `KONTEKS.md` sebelum mulai, karena arah proyek berubah tiap fase.

---

## 1. Cara Berkontribusi

- [ ] Laporkan bug / ide: buka **issue** (template: [Fase 7] link template issue)
- [ ] Perbaikan kecil / typo: langsung **Pull Request**
- [ ] Fitur besar: buka issue dulu → diskusi → baru implementasi

## 2. Alur Kontribusi

1. `fork` + `clone` repositori
2. Buat cabang: `fitur/deskripsi-pendek` atau `perbaikan/nama-bug`
3. Jalankan `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace --all-targets` — semua wajib hijau
4. Ikuti standar kode (zero-deps, error Bahasa Indonesia, komentar minimal)
5. Kirim PR → review → merge

## 3. Standar Kode

- **Zero external dependencies** (kecuali disepakati bersama di issue).
- Semua pesan error/warning: format `BAHAYA [KODE]` / `PERINGATAN [KODE]` Bahasa Indonesia.
- Komentar hanya bila menjelaskan "mengapa", bukan "apa".
- Tulis test untuk setiap logika non-trivial.
- Update dokumentasi terkait (`RENCANA.md`, `ERROR.md`, `TEMA.md`, dst.) bila fitur mengubah spesifikasi.

## 4. Definisi "Selesai" (Definition of Done)

- [ ] Build hijau (`cargo build --workspace`)
- [ ] Test hijau (`cargo test --workspace`)
- [ ] Clippy 0 warning (`cargo clippy --workspace --all-targets`)
- [ ] Ada test untuk perubahan (bila relevan)
- [ ] Dokumentasi diupdate bila perlu

## 5. Struktur Respons

| Permintaan | Tindakan |
|------------|----------|
| Bug tanpa penjelasan | Tanya info reproduksi (versi, OS, kode contoh) |
| Fitur baru | Diskusi scope & dampak pada fase |
| Dokumentasi min | Cek akurasi isi `RENCANA.md`/spesifikasi |