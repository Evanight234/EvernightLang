# EvernightLanguage — Ekstensi VS Code

Ekstensi resmi untuk bahasa pemrograman **EvernightLanguage** (`.eve`) pada keluarga editor VS Code (VS Code, Cursor, Windsurf, VSCodium, **Antigravity IDE**).

## Fitur Utama

- **Syntax Highlighting Lengkap**: Ditenagai TextMate grammar (`evernight.tmLanguage.json`) yang memetakan seluruh keyword, literal, deklarasi, blok komentar, operator, dan pemisah.
- **Tema Nusantara**:
  - `Evernight Nusantara Gelap`: Nuansa malam biru tua (`#0b1020`), toska laut (`#0f9d8f`), dan oranye batik (`#f2a65a`).
  - `Evernight Nusantara Terang`: Nuansa krem hangat (`#f5f0e8`) dan toska hutan (`#0b7a70`).
- **Snippet Bawaan (23 Snippet)**: Akses cepat struktur `fungsi`, `jika`, `selama`, `untuk`, `coba`, `cocok`, `variabel`, dll.
- **Autocomplete Statis (~60 Item)**: Tanpa dependensi LSP, otomatis menyarankan kata kunci bahasa, fungsi bawaan (seperti `cetak`, `baca`, `akar`, `tambah`, `salin`), dan modul pustaka standar saat mengetik `impor "`.
- **Icon Theme File `.eve`**: Menampilkan ikon resmi Evernight badge E pada tab dan explorer.

---

## Panduan Pemasangan Lokal

### Prasyarat
- Node.js (v18+)
- VS Code, Cursor, Windsurf, VSCodium, atau Antigravity IDE

### 1. Kompilasi Ekstensi
```bash
cd editors/vscode
npm install
npm run compile
```

### 2. Cara Menjalankan Langsung (Development Mode)
1. Buka folder `editors/vscode` di VS Code / Cursor / Antigravity.
2. Tekan tombol `F5` untuk meluncurkan jendela **Extension Development Host**.
3. Buat atau buka berkas berekstensi `.eve` (misal `contoh.eve`). Highlighting, icon, snippet, dan autocomplete langsung aktif.

### 3. Pemasangan Permanen via Paket `.vsix`
1. Install vsce jika belum ada:
   ```bash
   npm install -g @vscode/vsce
   ```
2. Buat bundle paket ekstensi (tanpa repositori git, tambahkan `--allow-missing-repository`):
   ```bash
   cd editors/vscode
   vsce package --allow-missing-repository
   ```
3. Pasang berkas `.vsix` yang dihasilkan:
   - **VS Code**: `code --install-extension evernight-language-0.1.0.vsix`
   - **Cursor**: `cursor --install-extension evernight-language-0.1.0.vsix`
   - **VSCodium**: `codium --install-extension evernight-language-0.1.0.vsix`
   - **Antigravity IDE**:
     ```
     "%LOCALAPPDATA%\Programs\Antigravity IDE\bin\antigravity-ide.cmd" --install-extension evernight-language-0.1.0.vsix --force
     ```
   - Atau lewat menu editor: Tekan `Ctrl+Shift+P` -> ketik `Extensions: Install from VSIX...` -> pilih file.

4. Setelah terpasang, **aktifkan tema** (opsional, agar warna sesuai palet Nusantara):
   - `Ctrl+Shift+P` -> **Preferences: Color Theme** -> pilih **Evernight Nusantara Gelap** / **Terang**.
5. Aktifkan **ikon berkas** (opsional): `Ctrl+Shift+P` -> **Preferences: File Icon Theme** -> pilih **Evernight File Icons**.

> **Catatan**: Ekstensi `evernight` mengaktifkan syntax highlighting begitu berkas `.eve` dibuka. Namun warna spesifik mengikuti tema editor yang aktif — pilih tema Nusantara di langkah 4 untuk tampilan penuh.

---

## Verifikasi Highlighting (Snapshot Test)
Ekstensi menyertakan uji snapshot highlighting pada folder `test/snapshots/`:
- `highlight.eve`: Kode contoh komprehensif seluruh konstruksi sintaksis EvernightLanguage.
- `highlight.eve.scope`: Pemetaan token terhadap TextMate scope yang diharapkan.

Gunakan fitur VS Code `Developer: Inspect Editor Tokens and Scopes` (`Ctrl+Shift+P`) saat membuka `highlight.eve` untuk memverifikasi kecocokan scope.
