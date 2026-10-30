---
judul: "Tahap 1: Halo Dunia"
grup: "Tutorial"
urutan: 1
---

# Tahap 1: Halo Dunia

## Tujuan

Menjalankan program EvernightLanguage pertama Anda: membuat berkas `.eve`, mencetak
teks ke layar, dan menulis komentar.

## Konsep

- **Berkas `.eve`** — setiap program EvernightLanguage disimpan dengan akhiran `.eve`.
- **Komentar** — diawali `#` dan diabaikan komputer. Pakai untuk menjelaskan kode.
- **`cetak(...)`** — menampilkan teks/angka ke layar. Boleh menerima beberapa nilai
  sekaligus, dipisah koma:

```eve
cetak("Halo", " ", "Dunia")
```

- **`variabel nama = nilai`** — menyimpan data supaya bisa dipakai berulang.

```eve
variabel pesan = "Selamat malam"
cetak(pesan)
```

Perhatikan: **tidak perlu titik koma** di akhir baris, dan tanda `{ }` dipakai untuk
blok kode (fungsi, `jika`, perulangan).

### Menjalankan program

Perintah dasarnya satu saja — `evernight` diikuti nama berkas:

```powershell
evernight halo.eve          # langsung jalankan
evernight halo.eve --cek    # hanya periksa sintaksis
evernight --versi           # lihat versi
```

Tidak ada tahap `build` terpisah: berkas `.eve` dibaca, dikompilasi, lalu langsung
dieksekusi oleh VM.

### Susunan satu program

```eve
# 1. komentar diabaikan
variabel nama = "Dunia"     # 2. data disimpan
cetak("Halo ", nama)        # 3. hasil dicetak
```

## Contoh lengkap

```eve
# Program pertama EvernightLanguage
variabel pesan = "Halo Dunia dari EvernightLanguage!"
cetak(pesan)

variabel nama = "Evernight"
cetak("Selamat belajar bersama, ", nama, "!")
```

Simpan sebagai `halo.eve`.

## Jalankan

```powershell
evernight halo.eve
```

Output yang diharapkan:

```
Halo Dunia dari EvernightLanguage!
Selamat belajar bersama, Evernight!
```

Bisa juga menjalankan contoh bawaan dari folder repo:

```powershell
evernight examples/hello.eve
```

## Latihan

1. Ubah teks pesan menjadi perkenalan singkat tentang diri Anda (nama + hobi).
2. Tambahkan satu variabel `umur` (angka) lalu cetak: `Umur saya `, umur.
3. Buat berkas `sapa.eve` yang mencetak 3 baris sapaan berbeda. Coba hapus tanda
   petik ganda — apa pesan errornya? (catat pesannya, berguna di Tahap 9.)
