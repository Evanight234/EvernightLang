---
judul: "Menangani Kesalahan"
grup: "Panduan"
urutan: 6
---

# Menangani Kesalahan

Halaman ini menjelaskan blok `coba` / `tangkap` / `akhirnya`, perintah `lempar` dan `pastikan`, format pesan kesalahan, serta kode error yang paling sering muncul.

## Format pesan kesalahan

```text
BAHAYA [KODE]: Baris X (Kolom Y) - Deskripsi masalah!
   X | baris kode sumber
     | ^
```

- **`BAHAYA`** — kesalahan fatal, program berhenti (kode keluar `1`).
- **`PERINGATAN`** — tidak menghentikan program (kuning).
- Di terminal, CLI menambahkan cuplikan baris kode dengan penunjuk `^`.
- Matikan warna dengan `--tanpa-warna` atau environment variable `NO_COLOR`.

## `coba` dan `tangkap`

```eve
coba {
    variabel hasil = 100 / 0
} tangkap (pesan) {
    cetak("Tertangkap: ", pesan)
}
```

- Tanda kurung setelah `tangkap` **wajib**: `tangkap (e) { ... }`.
- Variabel `e` berisi **pesan kesalahan** (teks), bukan objek error.
- Blok `akhirnya { ... }` (opsional) dijalankan setelah `coba`/`tangkap` selesai.
- `coba` hanya menangkap kesalahan yang terjadi **di lingkup/scope yang sama**. Kesalahan yang muncul di dalam fungsi lain tidak tertangkap blok `coba` di berkas luar — bungkus `coba` di dalam fungsi tersebut.

## `lempar` — galat kustom

```eve
coba {
    lempar("Data rusak!")
} tangkap (e) {
    cetak("Tertangkap: ", e)   # Tertangkap: Data rusak!
}
```

`lempar` bisa membawa ekspresi apa pun; teks adalah yang paling umum.

## `pastikan` — asersi

```eve
pastikan(b != 0, "Pembagi tidak boleh nol")
```

- Kurung **wajib**, argumen dipisah koma: `pastikan(kondisi, "pesan")`.
- Bila kondisi `salah` → `BAHAYA [ASSERT]: Pernyataan tidak benar!`.
- Bila ingin ditangkap, bungkus dengan `coba`/`tangkap` di scope yang sama.

## Kode error yang paling sering muncul

| Kode | Level | Pemicu | Contoh pesan |
|------|-------|--------|--------------|
| `SYNTAX` | BAHAYA | Sintaksis salah (tanda petik/kurung/blok) | `Diharapkan pembuka blok '{'!` |
| `TYPE` | BAHAYA | Tipe data tidak cocok | `Operator '+' tidak dapat digunakan antara tipe 'angka' dan 'teks'!` |
| `FUNGSI` | BAHAYA | Jumlah argumen salah / bukan fungsi | `Fungsi 'kali' butuh 2 argumen, diberikan 1` |
| `DIVISION` | BAHAYA | Pembagian dengan nol | `Pembagian dengan nol tidak diizinkan!` |
| `NaN` | BAHAYA | Konversi angka gagal / hasil tak valid | `Gagal mengonversi teks 'bukan_angka' menjadi angka!` |
| `VARIABLE` | BAHAYA | Variabel belum dideklarasikan / ubah konstanta | `Variabel 'x' belum dideklarasikan!` |
| `INDEX` | BAHAYA | Indeks daftar di luar jangkauan | `Indeks 5 di luar jangkauan daftar (panjang: 2)!` |
| `KEY` | BAHAYA | Kunci kamus tidak ada | `Kunci 'kunci_asing' tidak ditemukan dalam kamus!` |
| `ASSERT` | BAHAYA | `pastikan` gagal | `Pernyataan tidak benar!` |
| `RUNTIME` | BAHAYA | `lempar` / galat runtime | pesan dari `lempar` |
| `FILE` | BAHAYA | Berkas hilang, siklus impor, akses ditolak | `Berkas 'modul_hilang.eve' tidak ditemukan!` |
| `MATH` / `JUMLAH` | BAHAYA | Domain matematika salah | `Akar kuadrat tidak terdefinisi untuk angka negatif` |
| `WKVAR` / `WKREACH` / `WKFUNG` | PERINGATAN | Variabel tak terpakai / kode mati / fungsi kosong | `Variabel 'x' dideklarasikan tetapi tidak pernah digunakan!` |

Daftar lengkap ada di `ERROR.md`.

## Contoh program lengkap

Simpan sebagai `kesalahan.eve`, lalu jalankan **dari folder ini**: `evernight kesalahan.eve`

```eve
# 06 - Menangani kesalahan: coba, tangkap, lempar, pastikan

# 'coba' menangkap kesalahan supaya program tetap jalan
coba {
    variabel hasil = 100 / 0
    cetak("Tidak akan tercetak: ", hasil)
} tangkap (pesan) {
    cetak("Tertangkap: ", pesan)
} akhirnya {
    cetak("Blok akhirnya tetap dijalankan")
}

# 'pastikan' memvalidasi syarat; jika salah -> BAHAYA [ASSERT]
coba {
    pastikan(10 > 20, "Sepuluh harus lebih besar dari dua puluh")
} tangkap (e) {
    cetak("Assertion ditangkap: ", e)
}

# 'lempar' memicu kesalahan dengan pesan sendiri.
# Blok coba/tangkap yang menangkapnya harus berada di berkas/scope yang sama.
fungsi setor(nilai) {
    coba {
        jika nilai <= 0 {
            lempar("Nilai setor harus positif!")
        }
        kembali "Setor " + teks(nilai) + " berhasil"
    } tangkap (e) {
        kembali "GAGAL: " + e
    }
}
cetak(setor(50000))
cetak(setor(-1))

# Kegagalan fungsi bawaan juga bisa ditangkap (pakai jalur './...')
coba {
    cetak(baca_file("./berkas_tidak_ada.txt"))
} tangkap (e) {
    cetak("Berkas hilang: ", e)
}
cetak("Program selesai dengan aman.")
```

Output:

```text
Tertangkap: Pembagian dengan nol
Blok akhirnya tetap dijalankan
Assertion ditangkap: Sepuluh harus lebih besar dari dua puluh
Setor 50000 berhasil
GAGAL: Nilai setor harus positif!
Berkas hilang: Gagal membaca berkas './berkas_tidak_ada.txt': The system cannot find the file specified. (os error 2)
Program selesai dengan aman.
```

## Kesalahan umum

- **`tangkap e { }` tanpa kurung** → `BAHAYA [SYNTAX]: Diharapkan '(' setelah kata kunci 'tangkap'!`.
- **`pastikan kondisi` tanpa kurung** → `BAHAYA [SYNTAX]: Diharapkan '(' setelah 'pastikan'!`.
- **`coba` tanpa `tangkap`** → `BAHAYA [SYNTAX]: Diharapkan blok 'tangkap' setelah blok 'coba'!`.
- **Membaca berkas tanpa jalur folder** (`baca_file("data.txt")`) → `BAHAYA [FILE]` yang tidak tertangkap. Tulis `./data.txt` dan jalankan dari folder program.
- **Mengharapkan `coba` di berkas lain menangkap galat fungsi Anda** — pindahkan `coba` ke dalam fungsi tersebut.
