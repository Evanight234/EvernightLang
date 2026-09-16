# FUNCTION.md — Spesifikasi Fungsi & Eksekusi EvernightLanguage

> Sumber kebenaran untuk aturan deklarasi fungsi, eksekusi, kontrol perulangan, dan daftar fungsi bawaan di EvernightLanguage.

---

## A. Kategori & Jenis Fungsi

| Kategori | Deskripsi | Contoh Sintaksis |
|----------|-----------|------------------|
| **Fungsi Bawaan (Built-in)** | Fungsi global yang selalu tersedia tanpa impor (`cetak`, konversi tipe) | `cetak("Halo")` |
| **Fungsi Pengguna (User-Defined)** | Didefinisikan menggunakan kata kunci `fungsi` dengan nama identifikasi | `fungsi tambah(a, b) { kembali a + b }` |
| **Fungsi Anonim (Lambda)** | Fungsi tanpa nama yang dapat disimpan ke dalam variabel atau dikirim sebagai argumen | `variabel kali_dua = fungsi(x) { kembali x * 2 }` |
| **Penutupan (Closure)** | Fungsi anonim atau bertingkat yang menangkap dan mengingat lingkungan leksikalnya | `fungsi pembuat_tambah(n) { kembali fungsi(x) { kembali x + n } }` |
| **Fungsi Tingkat Tinggi (HOF)** | Fungsi yang menerima fungsi lain sebagai argumen atau mengembalikan fungsi | `peta(data, fungsi(x) { kembali x + 1 })` |

---

## B. Aturan Deklarasi Fungsi (Add Function)

| Aturan | Status | Keterangan & Contoh |
|--------|--------|---------------------|
| **Kata Kunci** | Wajib | Menggunakan `fungsi` di awal definisi |
| **Nama Fungsi** | Wajib (kecuali anonim) | Ditulis dalam huruf kecil / snake_case: `fungsi hitung_total(a, b)` |
| **Parameter** | Opsional | Didaftarkan dalam tanda kurung `()`, dipisahkan tanda koma `,` |
| **Parameter Default** | Didukung | Nilai default ditentukan di akhir daftar parameter: `fungsi sapa(nama, salam = "Halo")` |
| **Blok Badan** | Wajib | Diapit oleh kurung kurawal `{ ... }` |
| **Pengembalian (`kembali`)**| **Wajib Eksplisit** | Fungsi harus memanggil `kembali <nilai>` untuk menghasilkan nilai kembalian |
| **Fungsi Bertingkat (Nested)**| Didukung | Fungsi dapat didefinisikan di dalam badan fungsi lain |
| **Nilai Warga Kelas Satu** | Didukung | Fungsi dapat disimpan di variabel, elemen array, dan dipassing sebagai argumen |

### Contoh Implementasi Lengkap:
```eve
# 1. Fungsi biasa dengan rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

# 2. Fungsi dengan parameter default
fungsi buat_pesan(nama, awalan = "Selamat pagi") {
    kembali awalan + ", " + nama + "!"
}

# 3. Fungsi closure
fungsi buat_penghitung(awal = 0) {
    variabel hitungan = awal
    kembali fungsi() {
        hitungan = hitungan + 1
        kembali hitungan
    }
}
```

---

## C. Kontrol Perulangan (Loop & Iteration)

### C1. Kata Kunci Perulangan Dasar

| Kata Kunci | Pola Sintaksis | Deskripsi |
|------------|----------------|-----------|
| `selama` | `selama <kondisi> { ... }` | Mengulang blok kode selama kondisi bernilai `benar` |
| `untuk` (Rentang) | `untuk i dari <awal> sampai <akhir> { ... }` | Mengulang rentang angka inklusif |
| `untuk` (Iterasi) | `untuk item dalam <daftar> { ... }` | Mengiterasi setiap elemen dalam sebuah koleksi daftar |
| `berhenti` | `berhenti` | Menghentikan paksa dan keluar dari blok perulangan (`break`) |
| `lanjut` | `lanjut` | Melompati iterasi saat ini dan lanjut ke iterasi berikutnya (`continue`) |

Contoh:
```eve
# Loop range dengan henti/lanjut
untuk i dari 0 sampai 10 {
    jika i == 3 {
        lanjut
    }
    jika i == 8 {
        berhenti
    }
    cetak(i)
}
```

---

### C2. Iterasi Fungsional (Modul `daftar`)

| Fungsi | Input | Callback | Output | Deskripsi |
|--------|-------|----------|--------|-----------|
| `peta(lst, fn)` | `daftar`, `fn(item)` | `fn(x) -> hasil` | `daftar` baru | Transformasi elemen (Map) |
| `saring(lst, fn)` | `daftar`, `fn(item)` | `fn(x) -> bolean` | `daftar` tersaring | Filter elemen berdasarkan predikat |
| `lipat(lst, awal, fn)` | `daftar`, `awal`, `fn(acc, item)` | `fn(acc, x) -> acc`| `nilai` | Akumulasi nilai (Reduce/Fold) |
| `setiap(lst, fn)` | `daftar`, `fn(item)` | `fn(x)` | `kosong` | Menjalankan efek samping per elemen |

---

## D. Penanganan Alur Kesalahan (Error Control Flow)

| Pola | Keterangan & Contoh |
|------|---------------------|
| `coba { ... } tangkap(err) { ... }` | Menangkap kesalahan runtime dan menangani tanpa menghentikan sistem |
| `lempar(pesan)` | Memicu error fatal secara sengaja |
| `pastikan(kondisi, pesan)` | Evaluasi assertion; memicu error fatal jika kondisi bernilai `salah` |

---

## E. Status Finalisasi

- [x] Kategori fungsi lengkap didefinisikan (Built-in, User-defined, Anonymous, Closure, HOF)
- [x] Aturan deklarasi fungsi difinalkan (wajib `kembali`, parameter default, nested)
- [x] Perulangan `selama`, `untuk`, `berhenti`, `lanjut` disepakati
- [x] Struktur error flow (`coba`, `tangkap`, `lempar`, `pastikan`) terintegrasi
