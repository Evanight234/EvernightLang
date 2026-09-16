# TIPE.md — Sistem & Desain Tipe Data EvernightLanguage

> Sumber kebenaran untuk sistem dan spesifikasi tipe data EvernightLanguage.
> Bahasa ini menggunakan sistem **tipe dinamis** dengan manajemen memori **ARC**.

---

## 1. Ringkasan Keputusan Desain

| Aspek | Keputusan |
|-------|-----------|
| **Sistem Pengetikan** | Dinamis (runtime type checking) |
| **Model Angka** | Satu tipe terpadu `angka` (mencakup bulat & desimal/float) |
| **Coercion (Peleburan Tipe)** | **Ketat (Strict)** — tidak ada konversi otomatis antar-tipe berbeda (memicu error `BAHAYA`) |
| **Akses Panjang Teks** | Properti `.panjang` (contoh: `teks.panjang`) |
| **Akses Panjang Daftar** | Fungsi bawaan `panjang()` (contoh: `panjang(daftar)`) |
| **Manajemen Memori** | ARC (Automatic Reference Counting) |

---

## 2. Daftar Tipe Data Bawaan

| Tipe Data | Nama Tipe | Format Literal | Contoh Sintaksis |
|-----------|-----------|----------------|------------------|
| **Angka** | `angka` | `42`, `-10`, `3.14`, `.5` | `variabel x = 42` |
| **Teks** | `teks` | `"..."` atau `'...'` | `variabel pesan = "Halo Dunia"` |
| **Boolean** | `bolean` | `benar` atau `salah` | `variabel aktif = benar` |
| **Kosong** | `kosong` | `kosong` | `variabel nilai = kosong` |
| **Daftar** | `daftar` | `[item1, item2, ...]` | `variabel list = [1, "dua", benar]` |
| **Kamus** | `kamus` | `{kunci: nilai, ...}` | `variabel data = {"nama": "Budi", "umur": 20}` |
| **Fungsi** | `fungsi` | `fungsi(param) { ... }` | `variabel f = fungsi(x) { kembali x * 2 }` |
| **Objek / Kelas** | `objek` | Berdasarkan instansiasi kelas | `variabel mhs = Mahasiswa("Andi")` |

---

## 3. Detail Karakteristik Tipe

### A. Angka (`angka`)
- Menggabungkan representasi bilangan bulat (integer 64-bit) dan pecahan (float 64-bit).
- Operator yang didukung:
  - Penjumlahan: `+`
  - Pengurangan: `-`
  - Perkalian: `*`
  - Pembagian: `/` (pembagian dengan `0` menghasilkan `BAHAYA: Pembagian dengan nol!`)
  - Sisa Bagi (Modulo): `%`
  - Pangkat: `**`
- Operasi perbandingan: `==`, `!=`, `<`, `<=`, `>`, `>=`

### B. Teks (`teks`)
- Deretan karakter UTF-8 yang diapit tanda petik ganda (`"..."`) atau petik tunggal (`'...'`).
- Escape sequence didukung: `\n` (baris baru), `\t` (tab), `\"`, `\'`, `\\`.
- Penggabungan teks: `teks1 + teks2` (hanya berlaku antar `teks`).
- Akses panjang: properti `.panjang` (contoh: `"Halo".panjang` bernilai `4`).
- Pengecekan substring: operator `dalam` (contoh: `"al" dalam "Halo"` bernilai `benar`).

### C. Boolean (`bolean`)
- Bernilai literal `benar` (true) atau `salah` (false).
- Operator logika:
  - `dan` (AND)
  - `atau` (OR)
  - `bukan` (NOT)

### D. Kosong (`kosong`)
- Menandakan ketiadaan nilai (null/nil/None).
- Bernilai literal tunggal: `kosong`.

### E. Daftar (`daftar`)
- Koleksi data terurut dan dinamis (panjang fleksibel, tipe elemen campuran).
- Mengacu penuh pada spesifikasi `ARRAY.md`.
- Akses panjang: fungsi `panjang(daftar)`.
- Method bawaan: `.tambah()`, `.sisip()`, `.hapus()`, `.ambil()`, `.bersihkan()`.

### F. Kamus (`kamus`)
- Koleksi pasangan kunci-nilai (key-value store / map).
- Kunci dapat berupa `teks`, `angka`, atau `bolean`.
- Akses elemen: `kamus[kunci]` atau `kamus.kunci` (jika kunci berupa identifikasi teks valid).
- Pengecekan kunci: `kunci dalam kamus`.

---

## 4. Aturan Konversi Tipe (Type Casting)

### Tidak Ada Implisit Coercion
EvernightLanguage menolak konversi implisit antar-tipe yang berbeda untuk mencegah bug terselubung:
```eve
variabel hasil = 10 + "5"
# BAHAYA: Operasi '+' tidak didukung antara tipe 'angka' dan 'teks'!
```

### Fungsi Konversi Eksplisit
Konversi harus dilakukan secara sadar menggunakan fungsi bawaan:

| Fungsi Konversi | Target Tipe | Contoh Masukan | Hasil |
|-----------------|-------------|----------------|-------|
| `angka(val)` | `angka` | `"123"`, `"3.14"`, `benar` | `123`, `3.14`, `1` |
| `teks(val)` | `teks` | `42`, `benar`, `[1, 2]` | `"42"`, `"benar"`, `"[1, 2]"` |
| `bolean(val)` | `bolean` | `0`, `""`, `[]`, `kosong` | `salah` (falsy) |
| `bolean(val)` | `bolean` | `1`, `"teks"`, `[1]` | `benar` (truthy) |
| `daftar(val)` | `daftar` | `"Halo"` | `["H", "a", "l", "o"]` |

---

## 5. Status Finalisasi

- [x] Tipe primitif & komposit terdefinisi
- [x] Operator per tipe terdata
- [x] Aturan konversi tipe (eksplisit) disepakati
- [x] Sinkronisasi semantik dengan `ARRAY.md` dan `KONSEP.md`
