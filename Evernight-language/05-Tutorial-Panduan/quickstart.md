---
judul: "Mulai dalam 5 Menit"
grup: "Memulai"
urutan: 3
---

# Mulai dalam 5 Menit

Prasyarat: `evernight --versi` sudah menampilkan `EvernightLanguage v0.1.0` (lihat [Instalasi](instalasi.md)).

## Langkah 1 — Tulis program pertama

Buat berkas bernama `halo.eve`:

```eve
# halo.eve
cetak("Halo Dunia dari EvernightLanguage!")
```

## Langkah 2 — Jalankan

```text
evernight halo.eve
```

Output:

```text
Halo Dunia dari EvernightLanguage!
```

Berkas juga bisa dijalankan secara eksplisit dengan subcommand `run`:

```text
evernight run halo.eve
```

## Langkah 3 — Tambahkan input

```eve
# sapa.eve
variabel nama = baca("Siapa namamu? ")
variabel umur = baca_angka("Berapa umurmu? ")
cetak("Halo ", nama, "! Tahun depan umurmu ", umur + 1, " tahun.")
```

```text
evernight sapa.eve
```

```text
Siapa namamu? Rina
Berapa umurmu? 17
Halo Rina! Tahun depan umurmu 18 tahun.
```

## Langkah 4 — Perulangan dan fungsi

```eve
# deret.eve
fungsi deret(n) {
    variabel total = 0
    untuk i dari 1 sampai n {
        total = total + i
    }
    kembali total
}

untuk i dari 1 sampai 5 {
    cetak("1 + ... + ", i, " = ", deret(i))
}
```

```text
evernight deret.eve
```

```text
1 + ... + 1 = 1
1 + ... + 2 = 3
1 + ... + 3 = 6
1 + ... + 4 = 10
1 + ... + 5 = 15
```

## Langkah 5 — Daftar dan kamus

```eve
# toko.eve
variabel belanja = ["beras", "gula", "teh"]
variabel harga = { "beras": 12000, "gula": 15000, "teh": 8000 }

variabel total = 0
untuk item dalam belanja {
    variabel biaya = dapatkan(harga, item, 0)
    cetak(item, " = ", biaya)
    total = total + biaya
}
cetak("Total belanja: ", total, " (", belanja.panjang, " barang)")
```

```text
evernight toko.eve
```

```text
beras = 12000
gula = 15000
teh = 8000
Total belanja: 35000 (3 barang)
```

## Mode REPL — mencoba satu per satu

Tanpa argumen, `evernight` membuka sesi interaktif:

```text
evernight
```

```text
EvernightLanguage REPL (v0.1.0)
Ketik kode Evernight atau  :bantuan  untuk daftar perintah,  :keluar  untuk selesai.

eve> cetak(2 + 3)
5
eve> :keluar
Sampai jumpa!
```

Perintah REPL: `:bantuan` (`:help`, `:?`), `:muat <berkas.eve>`, `:bersihkan`, `:keluar` (`:exit`, `:quit`, `:q`).

## Perintah yang paling sering dipakai

```text
evernight halo.eve            jalankan
evernight halo.eve --cek       periksa sintaksis tanpa menjalankan
evernight format halo.eve      rapikan kode
evernight lint halo.eve        periksa gaya kode
evernight --bantuan            daftar perintah lengkap
```

## Langkah berikutnya

- Pelajari tata bahasa: [Struktur Dasar Sintaks](panduan/01-sintaks-dasar.md) → [Variabel dan Tipe](panduan/02-variabel-tipe.md) → [Fungsi](panduan/03-fungsi.md)
- Lihat program jadi: [Galeri Contoh](contoh.md)
- Detail perintah: [CLI & REPL](cli.md)

## Kesalahan umum

- **Jalankan dari folder yang sama** dengan berkas: `cd folder-anda` dulu, baru `evernight halo.eve`.
- **Kesalahan ketik kata kunci** → `BAHAYA [SYNTAX]`. Ingat: `jika`/`bukan`/`lainnya_jika` (dengan garis bawah), komentar `#`.
- **Campur angka dan teks tanpa konversi** → `BAHAYA [TYPE]`: gunakan `teks(42)` atau `angka("5")`.
- **Perintah `evernight` tidak dikenal** → buka terminal baru (lihat [Instalasi](instalasi.md)).
