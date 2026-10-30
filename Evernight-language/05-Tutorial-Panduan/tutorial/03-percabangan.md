---
judul: "Tahap 3: Percabangan"
grup: "Tutorial"
urutan: 3
---

# Tahap 3: Percabangan

## Tujuan

Membuat program yang memilih jalur eksekusi sendiri: genap/ganjil, positif/nol/negatif,
dan gabungan beberapa syarat.

## Konsep

### `jika` — `lainnya_jika` — `lainnya`

```eve
jika nilai > 80 {
    cetak("A")
} lainnya_jika nilai > 60 {
    cetak("B")
} lainnya {
    cetak("C")
}
```

Tanda kurung di sekitar kondisi **tidak dipakai** — langsung diikuti `{ }`.

### Operator perbandingan

`==` (sama dengan), `!=` (tidak sama), `<`, `<=`, `>`, `>=`.

Versi berbasis kata juga sah: `sama_dengan`, `lebih_dari`, `kurang_dari`.

### Operator logika

| Kata | Fungsi |
|------|--------|
| `dan` | kedua syarat harus bernilai `benar` |
| `atau` | cukup satu syarat bernilai `benar` |
| `bukan` | membalik nilai boolean |

```eve
jika umur >= 18 dan punya_ktp { cetak("Boleh") }
jika status == "admin" atau status == "root" { cetak("Istimewa") }
jika bukan (n > 100) { cetak("Di bawah 100") }
```

> Gunakan tanda kurung setelah `bukan` agar urutan perhitungan jelas.

### Modulo `%`

Sisa pembagian — cara paling umum menentukan genap/ganjil: `n % 2 == 0`.

## Contoh lengkap

```eve
variabel n = baca_angka("Masukkan bilangan: ")

jika n % 2 == 0 {
    cetak(n, " adalah bilangan genap")
} lainnya {
    cetak(n, " adalah bilangan ganjil")
}

jika n < 0 {
    cetak("Bilangan negatif")
} lainnya_jika n == 0 {
    cetak("Bilangan nol")
} lainnya {
    cetak("Bilangan positif")
}

jika n >= 0 dan n <= 100 {
    cetak("Rentang: 0..100")
} lainnya {
    cetak("Di luar rentang 0..100")
}

jika n % 2 == 0 atau n % 3 == 0 {
    cetak("Kelipatan 2 atau 3")
}

jika bukan (n > 100) {
    cetak("Nilainya di bawah 100 (bukan)")
}
```

## Jalankan

```powershell
evernight percabangan.eve
```

Ketik `6` saat diminta:

```
Masukkan bilangan: 6 adalah bilangan genap
Bilangan positif
Rentang: 0..100
Kelipatan 2 atau 3
Nilainya di bawah 100 (bukan)
```

Coba lagi dengan `-7` dan `0` untuk melihat cabang lain.

## Latihan

1. Tambahkan cabang khusus: jika `n` habis dibagi 5, cetak `"Kelipatan 5"`.
2. Buat pengecekan nilai ujian: `90` ke atas = A, `80`–`89` = B, di bawah = C.
3. Ubah program menjadi memakai `sama_dengan`/`lebih_dari`/`kurang_dari` — apakah
   hasilnya sama persis?
