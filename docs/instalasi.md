---
judul: "Instalasi"
grup: "Memulai"
urutan: 2
---

# Instalasi

EvernightLanguage 0.1.0 berjalan di Windows (x64) dalam bentuk biner tunggal — **tidak perlu Rust, Cargo, atau hak administrator sistem** untuk mode per pengguna.

## 1. Dapatkan paket

Paket resmi: `paket/evernight-0.1.0-windows-x64/` (atau arsip `...-windows-x64.zip`). Isi:

```text
bin/evernight.exe        compiler + VM
bin/kill.cmd, kill.ps1   pencopot
bin/update.cmd, update.ps1  pembaruan
setup/Setup.exe          installer GUI (wizard)
extensions/              ekstensi editor (.vsix)
docs/PANDUAN.txt         panduan pengguna
docs/catatan-0.1.0.txt   catatan rilis
LICENSE, versi.txt, BACA-AKU.txt
```

## 2. Pasang (pilih salah satu)

### A. Wizard instalasi GUI (disarankan)

Klik dua kali **`setup/Setup.exe`**, ikuti wizard, selesai.

### B. Tanpa instalasi (portabel)

Tambahkan folder `bin` ke variabel `PATH` pengguna secara manual, atau jalankan dengan jalur penuh:

```text
C:\folder-paket\bin\evernight.exe halo.eve
```

> **Catatan:** `docs/PANDUAN.txt` bawaan masih menyebut `install.cmd`/`install.ps1`, sedangkan paket 0.1.0 hanya berisi `setup/Setup.exe` + `bin/`. Gunakan wizard, atau pakai mode portabel di atas.

## 3. Verifikasi

**Buka terminal BARU** (agar PATH terbaca), lalu:

```text
evernight --versi
```

Harus muncul:

```text
EvernightLanguage v0.1.0
```

Info instalasi lebih lengkap:

```text
evernight system info
```

```text
EvernightLanguage v0.1.0
Lokasi: C:\Users\<Anda>\AppData\Local\Programs\Evernight
Ukuran evernight.exe: 1871 KB
...
```

Bila perintah `evernight` tidak dikenal, tutup terminal lama dan buka terminal baru; kalau masih gagal, pakai jalur penuh `...\bin\evernight.exe`.

## 4. Ekstensi editor (opsional)

Ekstensi memberi warna sintaksis, tema "Evernight Nusantara", snippet, autocomplete, dan ikon `.eve`. Berkasnya:

```text
extensions\evernight-language-0.1.0.vsix
```

- **Otomatis** — wizard/instalasi menawarkan pemasangan ke editor keluarga VS Code yang terdeteksi: Antigravity IDE, VS Code, VS Code Insiders, Cursor, Windsurf, VSCodium.
- **Manual lewat terminal:**

```text
code --install-extension "C:\folder-paket\extensions\evernight-language-0.1.0.vsix"
```

- **Manual lewat menu:** `Ctrl+Shift+P` → `Extensions: Install from VSIX...` → pilih berkasnya.

Aktifkan tema (opsional): `Ctrl+Shift+P` → `Preferences: Color Theme` → **Evernight Nusantara Gelap**.

## 5. Pembaruan

```text
update evernight system
```

Pembaruan memakai daftar versi di GitHub dan menampilkan catatan rilis sebelum mengganti biner.

## 6. Pencopotan

```text
kill evernight system
```

Menghapus program dari `%LocalAppData%\Programs\Evernight`, entri PATH, asosiasi berkas `.eve`, serta menawarkan pencopotan ekstensi editor.

## Masalah umum

| Gejala | Penyebab | Solusi |
|--------|----------|--------|
| `'evernight' is not recognized` | PATH belum terbaca | Buka terminal baru; atau pakai jalur penuh |
| `BAHAYA [FILE] Berkas tidak ditemukan` | SALAH | Lihat [Mulai dalam 5 menit](quickstart.md) |
| Warna tidak muncul | dukungan ANSI mati | Gunakan Windows Terminal; atau `--tanpa-warna` |
| `.eve` tidak terbuka dengan klik dua kali | asosiasi belum terpasang | Pasang lewat `setup/Setup.exe` |
| Ekstensi tidak ada di editor | editor tidak terdeteksi | Pasang `.vsix` secara manual (langkah 4) |
