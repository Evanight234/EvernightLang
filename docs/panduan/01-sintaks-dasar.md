---
judul: "Struktur Dasar Sintaks"
grup: "Panduan"
urutan: 1
---

# Struktur Dasar Sintaks

Halaman ini menjelaskan anatomi sebuah program EvernightLanguage: cara berkomentar, memisahkan pernyataan, menulis ekspresi, dan menyusun blok kode. Setelah selesai, Anda akan bisa membaca program `.eve` apa pun.

## Anatomi berkas `.eve`

- Berkas program memakai ekstensi **`.eve`** (contoh: `halo.eve`).
- Pernyataan dieksekusi **dari atas ke bawah** (top-down).
- Fungsi `utama()` hanya **opsional** — kalau ada, Anda memanggilnya sendiri di baris terakhir.
- Kata kunci ditulis **huruf kecil semua** dan bersifat *case-sensitive* (`Jika` ≠ `jika`).

```eve
cetak("pernyataan pertama")
cetak("pernyataan kedua")
```

## Komentar

Komentar memakai tanda `#` dan berlaku sampai akhir baris. Komentar dilewati oleh compiler.

```eve
# Ini komentar satu baris
variabel x = 1   # komentar di akhir baris juga boleh
```

## Pemisah pernyataan

**Garis baru (newline) adalah pemisah pernyataan.** Anda tidak perlu menulis titik-koma:

```eve
variabel a = 1
variabel b = 2
cetak(a + b)
```

Titik-koma (`;`) boleh ditulis di akhir pernyataan, tetapi tidak wajib — cukup gunakan satu gaya konsisten (tanpa titik-koma).

## Blok kode

Semua badan `jika`, `selama`, `untuk`, `cocok`, `fungsi`, dan `coba` **wajib** memakai kurung kurawal `{ ... }`:

```eve
jika benar {
    cetak("selalu pakai kurung kurawal")
}
```

Tidak ada aturan indentasi — indentasi hanya untuk keterbacaan.

## Ekspresi

Ekspresi adalah potongan kode yang menghasilkan nilai: literal (`42`, `"teks"`, `benar`), operasi (`+ - * / % **`), pemanggilan fungsi (`faktorial(5)`), pengindeksan (`daftar[0]`), dan akses properti (`teks.panjang`).

### Presedensi operator (dari paling rendah ke paling tinggi)

| Level | Operator | Keterangan |
|------:|----------|------------|
| 1 | `=` `+=` `-=` `*=` `/=` | Penugasan |
| 2 | `atau`, `\|\|` | ATAU logika |
| 3 | `dan`, `&&` | DAN logika |
| 4 | `==` `!=` `sama_dengan` | Kesetaraan |
| 5 | `<` `<=` `>` `>=` `lebih_dari` `kurang_dari` `dalam` | Perbandingan |
| 6 | `+` `-` | Penjumlahan & pengurangan |
| 7 | `*` `/` `%` | Perkalian, pembagian, sisa bagi |
| 8 | `**` | Pangkat (dihitung dari kanan) |
| 9 | `-` (unary), `bukan`, `!` | Negasi |
| 10 | `f(...)`, `a[i]`, `o.p` | Pemanggilan, indeks, properti |
| 11 | literal, identitas, `fungsi(...)` | Elemen dasar |

Karena `*` lebih tinggi dari `+`, ekspresi `1 + 2 * 3` bernilai `7`. Gunakan kurung `()` untuk memaksa urutan: `(1 + 2) * 3` bernilai `9`.

## Contoh program lengkap

Simpan sebagai `dasar.eve`, lalu jalankan: `evernight dasar.eve`

```eve
# 01 - Struktur dasar program EvernightLanguage

# Komentar memakai tanda '#' dan berlaku sampai akhir baris
variabel nama = "Nusantara"
variabel tahun = 2026

cetak("Selamat datang di ", nama, "!")
cetak("Tahun: ", tahun)

# Ekspresi aritmatika & operator
variabel luas = 6 * 7
cetak("Luas = ", luas)
cetak("Sisa bagi = ", luas % 5)
cetak("Pangkat = ", 2 ** 8)
cetak("Gabungan teks: ", "Ever" + "night")
cetak("Perbandingan: ", luas > 40)
cetak("Logika: ", luas > 40 dan tahun > 2000)
cetak("Negasi: ", bukan (luas < 10))

# Blok kode selalu memakai kurung kurawal { }
jika luas > 10 {
    variabel status = "besar"
    cetak("Status: ", status)
}
```

Output:

```text
Selamat datang di Nusantara!
Tahun: 2026
Luas = 42
Sisa bagi = 2
Pangkat = 256
Gabungan teks: Evernight
Perbandingan: benar
Logika: benar
Negasi: benar
Status: besar
```

## Kesalahan umum

- **`//` bukan komentar.** `//` akan dibaca sebagai token `/` yang tidak terduga → `BAHAYA [SYNTAX]: Token tak terduga: '/'`. Gunakan `#`.
- **Lupa `{ }`.** `jika benar` tanpa kurung kurawal → `BAHAYA [SYNTAX]: Diharapkan pembuka blok '{'!`.
- **Tanda petik tidak seimbang.** `"halo` tanpa penutup → `BAHAYA [SYNTAX]: Penutup tanda petik tidak ditemukan!`.
- **`cetak` tanpa tanda kurung.** `cetak "halo"` tidak valid; tulis `cetak("halo")`.
- **Catatan:** `cetak` menyambung tanpa pemisah, jadi tulis spasi sendiri: `cetak("Nilai: ", x)`.
