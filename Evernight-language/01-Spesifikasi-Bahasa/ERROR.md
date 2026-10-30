# ERROR.md — Sistem Error EvernightLanguage

> Sumber kebenaran untuk spesifikasi, kategori, dan format pesan kesalahan (error handling) di EvernightLanguage.
> Pesan kesalahan disajikan dalam **Bahasa Indonesia** dengan format terstandarisasi.

---

## 1. Format Pesan Standar

Setiap pesan kesalahan atau peringatan mengikuti pola:

```
LEVEL [KODE]: Baris <nomor_baris> (Kolom <nomor_kolom>) - <Deskripsi pesan>
   <baris> | <cuplikan_kode>
           | <penunjuk_caret ^>
```

- **`LEVEL`**: 
  - `BAHAYA` — Kesalahan fatal (merah `\x1b[1;31m`, program langsung berhenti / exit code 1).
  - `PERINGATAN` — Peringatan non-fatal (kuning `\x1b[1;33m`, program tetap berjalan).
- **`[KODE]`**: Kode kategori kesalahan untuk memudahkan penelusuran.
- **`Baris (Kolom)`**: Posisi baris dan kolom kode penyebab masalah.
- **Cuplikan Baris**: Pada CLI (`printer.rs`), baris sumber kode penyebab error ditampilkan dengan nomor baris (cyan `\x1b[36m`) dan penunjuk caret `^` tepat di kolom masalah. Opsi `--tanpa-warna` atau environment variable `NO_COLOR` menonaktifkan warna ANSI.

---

## 2. Kategori PERINGATAN (Non-Fatal)

Peringatan tidak menghentikan jalannya eksekusi program, melainkan memberi tahu pemrogram mengenai konstruksi kode yang tidak optimal atau berpotensi bermasalah.

| Kode | Nama Peringatan | Kapan Terjadi | Contoh Sintaksis | Contoh Pesan |
|------|-----------------|---------------|------------------|--------------|
| `WKFUNG` | Fungsi Kosong | Blok badan fungsi tidak memiliki instruksi | `fungsi hitung() { }` | `PERINGATAN [WKFUNG]: Baris 1 - Fungsi 'hitung' tidak memiliki isi!` |
| `WKCLASS` | Kelas Kosong | Blok badan kelas tidak memiliki method/properti | `class Model { }` | `PERINGATAN [WKCLASS]: Baris 4 - Kelas 'Model' tidak memiliki isi!` |
| `WKVAR` | Variabel Tak Terpakai | Variabel dideklarasikan namun tidak pernah dibaca | `variabel x = 10` | `PERINGATAN [WKVAR]: Baris 2 - Variabel 'x' dideklarasikan tetapi tidak pernah digunakan!` |
| `WKREACH` | Kode Tak Terjangkau | Terdapat instruksi setelah pernyataan `kembali` | `kembali a; cetak(a)` | `PERINGATAN [WKREACH]: Baris 6 - Kode setelah pernyataan 'kembali' tidak akan pernah dieksekusi!` |

---

## 3. Kategori BAHAYA (Fatal / Menghentikan Program)

Kesalahan fatal menghentikan eksekusi runtime atau menggagalkan tahap kompilasi/parsing.

### A. Kesalahan Parsing & Sintaksis (`SYNTAX`)
Terjadi pada fase pembacaan kode sumber (Lexer & Parser).

| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `SYNTAX` | String tidak tertutup | `variabel s = "halo dunia` | `BAHAYA [SYNTAX]: Baris 1 - Penutup tanda petik tidak ditemukan!` |
| `SYNTAX` | Tanda kurung tidak seimbang | `cetak((10 + 5)` | `BAHAYA [SYNTAX]: Baris 3 - Tanda kurung penutup ')' tidak cocok atau kurang!` |
| `SYNTAX` | Token tak terduga | `variabel = 20` | `BAHAYA [SYNTAX]: Baris 2 - Diharapkan nama variabel sebelum '='!` |
| `SYNTAX` | Indentasi / Blok salah | `jika benar` (tanpa `{`) | `BAHAYA [SYNTAX]: Baris 5 - Diharapkan pembuka blok '{'!` |

### B. Kesalahan Tipe (`TYPE`)
Terjadi saat evaluasi ekspresi dengan tipe data yang tidak cocok.

| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `TYPE` | Operasi tipe tidak cocok | `10 + "5"` | `BAHAYA [TYPE]: Baris 4 - Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!` |
| `TYPE` | Akses properti pada tipe salah / `kosong` | `variabel x = kosong; x.nama` | `BAHAYA [TYPE]: Baris 11 - Tidak dapat membaca properti dari nilai 'kosong'!` |

### B.5. Kesalahan Panggilan Fungsi (`FUNGSI`)
Terjadi saat pemanggilan fungsi (arity salah, bukan fungsi, callback).

| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `FUNGSI` | Bukan fungsi | `variabel x = 10; x()` | `BAHAYA [FUNGSI]: Baris 3 - '10' bukan fungsi` |
| `FUNGSI` | Jumlah argumen salah | `fungsi kali(a,b) {}; kali(5)` | `BAHAYA [FUNGSI]: Baris 5 - Fungsi 'kali' butuh 2 argumen, diberikan 1` |
| `FUNGSI` | Arity callback tidak sesuai | `peta([1], fungsi(a,b){})` | `BAHAYA [FUNGSI]: Baris 1 - Fungsi callback harus menerima 1 argumen` |

### C. Kesalahan Nilai Bukan Angka (`NaN`)
Operasi konversi matematika yang tidak sah menghasilkan error fatal langsung (strict mode).

| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `NaN` | Konversi gagal ke angka | `angka("bukan_angka")` | `BAHAYA [NaN]: Baris 2 - Gagal mengonversi teks 'bukan_angka' menjadi angka!` |
| `NaN` | Operasi matematika tak terdefinisi | `0 / 0` | `BAHAYA [NaN]: Baris 5 - Hasil operasi matematika tidak valid (Bukan Angka)!` |

### D. Kesalahan Pembagian (`DIVISION`)
| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `DIVISION` | Pembagian dengan nol | `100 / 0` | `BAHAYA [DIVISION]: Baris 3 - Pembagian dengan nol tidak diizinkan!` |

### E. Kesalahan Indeks & Kunci Koleksi (`INDEX` & `KEY`)
| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `INDEX` | Indeks di luar jangkauan | `[1, 2][5]` | `BAHAYA [INDEX]: Baris 6 - Indeks 5 di luar jangkauan daftar (panjang: 2)!` |
| `KEY` | Kunci tidak ada di kamus | `kamus["kunci_asing"]` | `BAHAYA [KEY]: Baris 8 - Kunci 'kunci_asing' tidak ditemukan dalam kamus!` |

### F. Kesalahan Lingkup Variabel (`VARIABLE`)
| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `VARIABLE` | Variabel tidak terdefinisi | `cetak(nilai_rahasia)` | `BAHAYA [VARIABLE]: Baris 4 - Variabel 'nilai_rahasia' belum dideklarasikan!` |
| `VARIABLE` | Penugasan variabel `tetap` | `tetap PI = 3.14; PI = 3.15` | `BAHAYA [VARIABLE]: Baris 2 - Tidak dapat mengubah nilai konstanta 'tetap' PI!` |

### G. Kesalahan Runtime & Sistem (`RUNTIME`, `MATH`, `JUMLAH`, `ASSERT`, `FILE`, `ENV`, `WAKTU`, `STACK`, `MEMORY`)
| Kode | Kasus Kesalahan | Contoh Pemicu | Contoh Pesan |
|------|-----------------|---------------|--------------|
| `RUNTIME` | Program melewati batas bytecode | Error internal VM | `BAHAYA [RUNTIME]: Baris 0 - Program melewati batas bytecode` |
| `RUNTIME` | Lempar error kustom | `lempar "gagal!"` | `BAHAYA [RUNTIME]: Baris 1 - gagal!` |
| `RUNTIME` | Builtin tak dikenal | Error internal | `BAHAYA [RUNTIME]: Baris 0 - Builtin tak dikenal` |
| `MATH` | Akar kuadrat negatif | `akar(-4)` | `BAHAYA [MATH]: Baris 1 - Akar kuadrat tidak terdefinisi untuk angka negatif` |
| `MATH` | Logaritma tidak terdefinisi | `log(0)` | `BAHAYA [MATH]: Baris 1 - Logaritma tidak terdefinisi untuk angka nol atau negatif` |
| `JUMLAH` | Faktorial bilangan negatif/fraksi | `faktorial(-1)` | `BAHAYA [JUMLAH]: Baris 1 - Faktorial hanya untuk bilangan bulat tak-negatif` |
| `JUMLAH` | Faktorial melebihi batas | `faktorial(171)` | `BAHAYA [JUMLAH]: Baris 1 - Faktorial melebihi batas angka (maksimal 170)` |
| `JUMLAH` | Rata-rata daftar kosong | `rata_rata([])` | `BAHAYA [JUMLAH]: Baris 1 - rata_rata dari daftar kosong tidak terdefinisi` |
| `ASSERT` | Pernyataan `pastikan` gagal | `pastikan 1 == 2` | `BAHAYA [ASSERT]: Baris 1 - Pernyataan tidak benar!` |
| `FILE` | Berkas tidak ditemukan | `impor modul_hilang` | `BAHAYA [FILE]: Baris 1 - Berkas 'modul_hilang.eve' tidak ditemukan!` |
| `FILE` | Siklus impor terdeteksi | `a.eve` impor `b.eve`, `b.eve` impor `a.eve` | `BAHAYA [FILE]: Baris 1 - Siklus impor terdeteksi saat memuat 'a'!` |
| `FILE` | Izin akses ditolak | Membaca berkas tanpa izin | `BAHAYA [FILE]: Baris 3 - Akses ke berkas ditolak (Permission Denied)!` |
| `FILE` | Baca/tulis berkas gagal | `baca_file("data.txt")` (tak ada) | `BAHAYA [FILE]: Baris 5 - Gagal membaca berkas 'data.txt': No such file or directory (os error 2)!` |
| `ENV` | Variabel lingkungan tidak ada | `env("DB_HOST")` tanpa bawaan | `BAHAYA [ENV]: Baris 2 - Variabel lingkungan 'DB_HOST' tidak ditemukan!` |
| `WAKTU` | Fungsi waktu menerima nilai tidak valid | `tunda(-1)` | `BAHAYA [WAKTU]: Baris 4 - tunda tidak menerima nilai negatif!` |
| `STACK` | Kedalaman rekursi melampaui batas *(direncanakan — belum di-raise)* | Rekursi tak hingga | `BAHAYA [STACK]: Batas kedalaman rekursi tercapai (Stack Overflow)!` |
| `MEMORY` | Alokasi memori gagal *(direncanakan — belum di-raise)* | Alokasi daftar melebihi batas | `BAHAYA [MEMORY]: Kehabisan memori (Out of Memory)!` |

---

## 4. Status Finalisasi

- [x] Standar format pesan (`BAHAYA [KODE]` dan `PERINGATAN [KODE]`)
- [x] Kategori warning (fungsi kosong, kelas kosong, variabel menganggur)
- [x] Kategori fatal error runtime & parsing
- [x] Penanganan `NaN` sebagai error fatal langsung disepakati
- [x] Sinkron kode error terpakai di stdlib: `FUNGSI` (panggilan fungsi), `MATH` (domain akar/log), `JUMLAH` (faktorial/rata_rata), `RUNTIME` (lempar/bytecode), `ASSERT` (pastikan) — 2026-09-10
- [ ] `STACK`/`MEMORY` akan di-raise saat implementasi batas rekursi & alokasi
