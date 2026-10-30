---
judul: "Galeri Contoh Program"
grup: "Memulai"
urutan: 5
---

# Galeri Contoh Program

Seluruh contoh di bawah ini tersedia di folder `examples/` dan **semuanya sudah diuji jalan** (keluar `0`).

## Menjalankan

```text
evernight examples/hello.eve
evernight examples/import/main.eve
```

Catatan: jalankan dari **root proyek** (folder tempat folder `examples/` berada), karena jalur relatif program dihitung dari folder program.

## Contoh program inti

| Berkas | Isi | Output singkat |
|--------|-----|----------------|
| `examples/hello.eve` | Program pertama: variabel + `cetak` | `Halo Dunia dari EvernightLanguage!` |
| `examples/faktorial.eve` | Rekursi: `faktorial(5)` | `Faktorial:120` |
| `examples/coba.eve` | Aritmatika dasar | `6` |
| `examples/fitur_baru.eve` | `coba`/`tangkap`/`lempar`, `cocok`, `pastikan`, fungsi `utama()`, operator `+=` | `Total: 15`, `Error: data rusak!`, `Semua berhasil!` |
| `examples/string.eve` | Modul `string`: `besar`, `kecil`, `bersih`, `potong`, `pecah`, `ganti`, `mengandung`, `ulang_teks`, `format` | `besar:   EVERNIGHT LANGUAGE  ` |
| `examples/daftar.eve` | Modul `daftar`: `tambah`, `sisip`, `hapus`, `urutkan`, `balik`, `unik`, `jumlah`, `rata_rata`, `cari`, `ada`, `iris`, `gabung_larik`, `lipat`, `peta`, `saring`, `setiap` | `Setelah tambah: [5, 3, 8, 1, 3, 9]` |
| `examples/kamus.eve` | Modul `kamus`: setel, `dapatkan`, `kunci`, `nilai`, `pasangan`, `ada_kunci`, `hapus`, `gabung_kamus` | `Nama: Budi` |
| `examples/matematika.eve` | Modul `matematika`: `akar`, `pangkat`, `bulat_bawah`, `bulat_atas`, `pembulatan`, `mutlak`, `min`, `max`, `faktorial`, `log`, `sin`, `pi`, `acak_antara` | `akar(144) = 12` |
| `examples/konsol.eve` | Modul `konsol`: `baca`, `baca_angka`, `bersihkan_layar` | `Halo Budi, tahun depan umurmu 18` |
| `examples/sistem.eve` | Modul `sistem`: `waktu_sekarang`, `tanggal_sekarang`, `atur_env`, `ada_env`, `baca_file`, `tulis_file`, `jeda` | `Waktu sekarang (ms): ...` |
| `examples/utilitas.eve` | Modul `utilitas`: `adalah_angka/teks/daftar/kamus`, `ke_boolean`, `ke_larik`, konversi tipe, `salin` | `adalah_angka(42): benar` |

## Contoh lintas berkas

| Berkas | Isi |
|--------|-----|
| `examples/import/helper.eve` | Modul berisi `tambah`, `kali`, `kuadrat` |
| `examples/import/main.eve` | `impor "helper" sebagai mtk` lalu memanggil `mtk.tambah(2, 3)` |

```text
evernight examples/import/main.eve
```

```text
Tambah: 5
Kali: 20
Kuadrat: 9
```

Pelajaran: ekspor sebuah modul berupa **satu kamus**, jadi dipanggil lewat namespace (`mtk.tambah`), bukan nama global — lihat [Panduan Impor](panduan/07-impor.md).

## Dua gaya penulisan yang didukung

```eve
# Gaya eksplisit (disarankan)
fungsi utama() {
    variabel total = 0
    untuk i dari 1 sampai 5 {
        total += i
    }
    cetak("Total: ", total)
}
utama()
```

```eve
# Gaya langsung (top-down, tanpa fungsi utama)
variabel pesan = "Halo Dunia dari EvernightLanguage!"
cetak(pesan)
```

Penugasan ke nama yang belum pernah dideklarasikan akan **membuat variabel baru** (otomatis), tetapi gaya baku yang disarankan tetap menulis `variabel x = ...` agar kode mudah dibaca.

## Contoh lengkap: faktorial

```eve
# examples/faktorial.eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

variabel angka = 5
variabel hasil = faktorial(angka)
cetak("Faktorial:", hasil)
```

```text
Faktorial:120
```

## Kesalahan umum

- **Salah folder** → `BAHAYA [FILE] Berkas ... tidak ditemukan`. Pindah dulu ke root proyek.
- **Membaca `examples/sistem.eve` sebagai referensi akses berkas**: jalurnya relatif folder program (`examples/contoh_berkas.txt`), bukan folder tempat Anda menjalankan perintah.
- **`import`/`require`** tidak ada — bahasa ini memakai `impor`.
- **Isi modul dipanggil tanpa namespace** → `BAHAYA [VARIABLE]`.
