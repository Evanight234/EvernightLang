# MARCH.md — Spesifikasi Web Notebook Interaktif "March" (.emlbn)

> **Filosofi**: *"Eksplorasi data dan eksperimen Machine Learning tanpa batas dengan antarmuka notebook interaktif modern bertenaga EvernightLanguage, berjalan 100% di atas berkas lokal laptop Anda."*

---

## 1. Ringkasan & Identitas

| Aspek | Spesifikasi Resmi |
|---|---|
| **Nama Aplikasi** | **March** (atau **March Notebook**) |
| **Ekstensi Berkas** | **`.emlbn`** (*Evernight Machine Learning Notebook*) |
| **Gaya Antarmuka** | Berbasis Web interaktif, terinspirasi penuh dari **Google Colab** (Dark Mode) |
| **Bahasa Kernel** | **EvernightLanguage** (`.eve`) + pustaka terpadu **`Matra`** |
| **Penyimpanan Data** | **100% Direktori Lokal Laptop Pengguna** (tanpa upload ke cloud) |
| **Perintah Eksekusi** | `evernight march` atau `evernight march <berkas.emlbn>` |
| **Distribusi** | Terpadu di dalam biner compiler `evernight` tanpa instalasi software eksternal |

---

## 2. Karakteristik Utama & Keunggulan

1. **Akses Data Lokal 100% Nyata**:
   - Berbeda dari Google Colab yang berjalan di server cloud terisolasi (wajib upload file ke `/content/` atau mount Google Drive yang hilang saat sesi selesai), **March berjalan langsung di harddisk laptop pengguna**.
   - Skrip membaca dan menulis berkas nyata:
     ```eve
     impor "matra" sebagai mt

     # Membaca berkas CSV lokal di folder proyek laptop
     variabel df = mt.baca_csv("data_siswa.csv")
     df.kepala()
     ```
   - Semua dataset CSV, berkas gambar, audio, dan bobot model tersimpan aman, privat, dan permanen di harddisk lokal.
2. **Desain Mirip Google Colab**:
   - Palet warna gelap modern khas Google Colab.
   - Tombol eksekusi sel bulat `▶` di kiri sel kode.
   - Tombol aksi sel di kanan atas sel (pindah atas, pindah bawah, edit, salin, hapus).
   - Tombol "+ Kode" dan "+ Teks" yang muncul halus saat hover di antara sel.
   - Tampilan DataFrame interaktif persis tabel Colab: indeks baris tebal, kolom abu-abu terstruktur, zebra striping, dan scrolling halus.
3. **Kernel Evernight VM Stateful**:
   - Variabel, fungsi, impor modul `matra`, dan model pelatihan yang dibuat di sel sebelumnya tetap tersimpan di memori runtime untuk sel-sel berikutnya.
4. **Output Kaya (*Rich Media Rendering*)**:
   - Objek `MatraTabel` (DataFrame) otomatis dirender menjadi tabel HTML interaktif.
   - Objek `MatraPlot` (Grafik) otomatis dirender menjadi grafik SVG vektor resolusi tinggi.
   - Pesan kesalahan `BAHAYA [KODE]` dirender dengan blok visual merah gelap dan penunjuk caret baris kode.

---

## 3. Spesifikasi Desain Antarmuka (Google Colab Style)

Tampilan visual March mengadopsi estetika Google Colab Dark Mode dengan sentuhan aksen Evernight:

### 3.1. Palet Warna (Colab Dark Palette)
* **Latar Belakang Aplikasi**: `#1e1e1e` / `#202124`
* **Latar Belakang Sel (Cell Surface)**: `#292a2d` / `#28292c`
* **Latar Belakang Sel Aktif (Focused Cell)**: `#2f3136`
* **Garis Batas Sel (Border)**: `#3c4043` (fokus: `#4a7cf5` biru Colab)
* **Teks Utama**: `#e8eaed`
* **Teks Sekunder / Header Tabel**: `#9aa0a6`
* **Aksen Tombol Run / Tombol Aktif**: `#8ab4f8` / `#f2994a`
* **Latar Header Tabel DataFrame**: `#303134`
* **Latar Baris Genap Tabel**: `#242528`
* **Warna Teks Indeks Baris (Index Column)**: `#c4c7c5` (font tebal / bold)
* **Error Banner**: `#442726` dengan teks merah terang `#f28b82`

### 3.2. Tata Letak (Layout)
```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ [≡] March Notebook   Analisis_Data_Siswa.emlbn   [Disimpan ke harddisk lokal]    RAM O │
├────────┬───────────────────────────────────────────────────────────────────────────────┤
│        │  [+ Kode]   [+ Teks]   [▶ Jalankan Semua]   [⟳ Muat Ulang Kernel]            │
│  [📁]   ├───────────────────────────────────────────────────────────────────────────────┤
│ Berkas │ ┌─[▶]──────────────────────────────────────────────────────────────────[↑][↓][🗑]─┐│
│ Lokal  │ │ impor "matra" sebagai mt                                                    ││
│        │ │                                                                             ││
│ ├ data │ │ variabel link = "data_siswa.csv"                                            ││
│ ├ hasil│ │ variabel df = mt.baca_csv(link)                                            ││
│ └ ...  │ │ df.kepala()                                                                 ││
│        │ └─────────────────────────────────────────────────────────────────────────────┘│
│        │   ┌───┬──────────────────┬──────────────────────┬─────────────┬──────────────┐ │
│        │   │   │ nama_siswa       │ jam_belajar_per_hari │ nilai_ujian │ status       │ │
│        │   ├───┼──────────────────┼──────────────────────┼─────────────┼──────────────┤ │
│        │   │ 0 │ Dewi Firmansyah  │ 3.2                  │ 180.0       │ Lulus        │ │
│        │   │ 1 │ Arif Nugraha     │ 6.2                  │ 250.0       │ Lulus        │ │
│        │   │ 2 │ Raka Santoso     │ 4.6                  │ 999.0       │ Lulus        │ │
│        │   └───┴──────────────────┴──────────────────────┴─────────────┴──────────────┘ │
│        │                                                                               │
│        │                             [+ Kode]    [+ Teks]                              │
│        │                                                                               │
│        │ ┌─[▶]──────────────────────────────────────────────────────────────────[↑][↓][🗑]─┐│
│        │ │ mt.grafik.sebar(df["jam_belajar_per_hari"], df["nilai_ujian"])              ││
│        │ │     .judul("Korelasi Belajar vs Nilai").tampil()                            ││
│        │ └─────────────────────────────────────────────────────────────────────────────┘│
│        │   [ Render Grafis Vektor SVG Responsif & Tajam di Sini ]                      │
└────────┴───────────────────────────────────────────────────────────────────────────────┘
```

---

## 4. Spesifikasi Berkas Format `.emlbn`

Berkas `.emlbn` (*Evernight Machine Learning Notebook*) adalah format berkas terbuka berbasis JSON terstruktur yang menyimpan konten kode, catatan, riwayat urutan eksekusi, serta keluaran visual terakhir.

### Skema Struktur JSON `.emlbn`:
```json
{
  "march_versi": "1.0",
  "bahasa": "EvernightLanguage",
  "nama_notebook": "Eksplorasi Data Siswa",
  "waktu_dibuat": "2026-10-28T09:58:00Z",
  "waktu_diubah": "2026-10-28T10:15:20Z",
  "metadata": {
    "penulis": "Satriyo",
    "folder_kerja": "E:/EvernightLang/proyek_ml",
    "kernel": {
      "nama": "evernight",
      "versi": "0.1.0"
    }
  },
  "sel": [
    {
      "id": "sel_01",
      "tipe": "markdown",
      "sumber": "# Eksplorasi Data Siswa\nAnalisis pengaruh jam belajar terhadap kelulusan."
    },
    {
      "id": "sel_02",
      "tipe": "kode",
      "urutan_eksekusi": 1,
      "sumber": "impor \"matra\" sebagai mt\n\nvariabel link = \"data_siswa.csv\"\nvariabel df = mt.baca_csv(link)\ndf.kepala()",
      "keluaran": {
        "tipe": "tabel_html",
        "data": {
          "kolom": ["nama_siswa", "jam_belajar_per_hari", "kehadiran_persen", "nilai_tugas", "nilai_ujian", "status_kelulusan"],
          "baris": [
            ["Dewi Firmansyah", 3.2, 80.9, 65.0, 180.0, "Lulus"],
            ["Arif Nugraha", 6.2, 76.8, 100.0, 250.0, "Lulus"],
            ["Raka Santoso", 4.6, 91.6, 74.0, 999.0, "Lulus"]
          ]
        }
      }
    },
    {
      "id": "sel_03",
      "tipe": "kode",
      "urutan_eksekusi": 2,
      "sumber": "mt.grafik.sebar(df[\"jam_belajar_per_hari\"], df[\"nilai_ujian\"]).judul(\"Korelasi Belajar vs Ujian\").tampil()",
      "keluaran": {
        "tipe": "grafik_svg",
        "svg": "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 720 420\" ...>...</svg>"
      }
    }
  ]
}
```

---

## 5. Arsitektur Komponen Teknis March

### 5.1. Backend & Server Lokal (Rust Murni)
- **Komponen CLI**: Terintegrasi pada biner `evernight` (`crates/evernight_cli`).
- **Server HTTP & WebSocket**: Server lokal ringan tanpa dependensi eksternal (menggunakan `std::net::TcpListener` murni Rust) yang melayani permintaan di `127.0.0.1:8888`.
- **Pengendali State Kernel**: Menjalankan session `Vm::run_incremental()` per notebook aktif. Variabel, objek `MatraArray`, `MatraTabel`, dan modul `impor` tetap berada di RAM selama server March berjalan.
- **Autosave Engine**: Secara otomatis menyimpan pembaruan konten dan output sel kembali ke berkas `.emlbn` di harddisk lokal pengguna setiap kali eksekusi sel selesai atau setiap 15 detik.

### 5.2. Jembatan Akses Berkas Lokal (*Local File Bridge*)
- Saat server March dijalankan di direktori kerja (misal: `E:\ProyekML`), server menetapkan direktori tersebut sebagai root folder kerja.
- Endpoint API `/api/berkas` menyediakan daftar isi direktori lokal untuk panel penjelajah berkas di bilah kiri UI.
- Semua fungsi I/O seperti `mt.baca_csv()`, `baca_file()`, `tulis_file()`, dan `simpan_svg()` secara alami memiliki akses membaca dan menulis langsung ke file system laptop lokal.

### 5.3. Frontend Web UI (Colab-like Single Page Application)
- **Editor Sel**: Monaco Editor / CodeMirror ringan dengan syntax highlighting EvernightLanguage (`.eve`).
- **Pintasan Keyboard (Shortcuts)**:
  - `Shift + Enter`: Menjalankan sel aktif dan fokus ke sel berikutnya.
  - `Ctrl + Enter`: Menjalankan sel aktif di tempat.
  - `Alt + Enter`: Menjalankan sel aktif dan menyisipkan sel baru di bawahnya.
  - `Esc` lalu `A`: Menyisipkan sel di atas (*Above*).
  - `Esc` lalu `B`: Menyisipkan sel di bawah (*Below*).
  - `Esc` lalu `D D`: Menghapus sel aktif (*Delete*).

---

## 6. Integrasi Perintah CLI & Asosiasi Windows

1. **Menjalankan March**:
   ```powershell
   # Membuka March di direktori saat ini
   evernight march

   # Membuka March pada port tertentu
   evernight march --port 8080

   # Membuka langsung berkas notebook tertentu
   evernight march eksperimen.emlbn
   ```
2. **Ekspor Notebook ke Skrip Standar**:
   ```powershell
   # Mengonversi notebook .emlbn menjadi berkas skrip .eve biasa
   evernight march ekspor eksperimen.emlbn --ke-eve
   ```
3. **Asosiasi Berkas Windows**:
   - Berkas berekstensi `.emlbn` diasosiasikan di Windows Explorer dengan ProgID `MarchNotebookFile`.
   - Klik ganda pada berkas `.emlbn` di Windows Explorer akan otomatis menyalakan server March dan membuka notebook tersebut di browser default.

---

## 7. Rencana Tahapan Eksekusi Subfase 9H (March Notebook)

Pengerjaan March Notebook diintegrasikan ke dalam Fase 9 sebagai **Subfase 9H**:

- [ ] **9H-1: Format Berkas & Serializer `.emlbn`**: Struktur data internal di Rust (`MarchNotebook`, `MarchCell`), parser dan serializer JSON zero-dependency, serta utilitas ekspor `.emlbn` ke skrip `.eve`.
- [ ] **9H-2: Kernel Server & Driver Eksekusi Stateful**: Server TCP/HTTP lokal di `evernight_cli`, endpoint eksekusi sel via `Vm::run_incremental`, output capture terstruktur (HTML, SVG, Teks, Error).
- [ ] **9H-3: Web UI Frontend March (Google Colab Theme)**: Template antarmuka web Colab Dark Mode terpadu (HTML, CSS, JS) yang di-embed langsung ke dalam biner release, renderer tabel DataFrame HTML, renderer plot SVG, dan file tree lokal.
- [ ] **9H-4: CLI Subcommand `evernight march` & Asosiasi File `.emlbn`**: Wiring CLI runner, peluncuran otomatis browser default (`rundll32 url.dll,FileProtocolHandler`), dan registrasi asosiasi Windows.
