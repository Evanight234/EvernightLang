---
judul: "Tahap 9: Menangani Kesalahan"
grup: "Tutorial"
urutan: 9
---

# Tahap 9: Menangani Kesalahan

## Tujuan

Mencegah program langsung mati saat terjadi masalah: membungkus kode berisiko dengan
`coba` — `tangkap`, memicu error sendiri dengan `lempar`, dan memvalidasi syarat
dengan `pastikan`.

## Konsep

### `coba` — `tangkap`

```eve
coba {
    variabel d = [1, 2]
    cetak(d[9])
} tangkap (e) {
    cetak("Terjadi kesalahan: ", e)
}
```

Kurung setelah `tangkap` **wajib**. Variabel `e` berisi pesan kesalahan berbahasa
Indonesia.

### `lempar("pesan")`

Memicu kesalahan sendiri, biasanya saat data tidak sah:

```eve
coba {
    jika usia < 0 {
        lempar("Usia tidak boleh negatif!")
    }
} tangkap (e) {
    cetak("Error: ", e)
}
```

### `pastikan(kondisi, "pesan")`

Memvalidasi syarat. Bila kondisi `salah`, langsung terjadi kesalahan dengan pesan
tersebut — pasangan sempurna dengan `coba`/`tangkap`:

```eve
coba {
    pastikan(b != 0, "Pembagi tidak boleh nol!")
} tangkap (e) {
    cetak("Gagal: ", e)
}
```

### `akhirnya { }`

Blok yang **selalu** dijalankan, baik berhasil maupun gagal.

### Batasan yang perlu diketahui

Saat ini `tangkap` hanya menangkap kesalahan yang terjadi **di dalam blok `coba`
pada fungsi yang sama**. Kesalahan dari fungsi lain yang dipanggil (termasuk `lempar`
di dalamnya) akan merambat ke atas dan menghentikan program. Karena itu, letakkan
`coba`/`tangkap` di dalam fungsi yang menghasilkan kesalahannya — seperti fungsi
`bagi` di bawah ini.

Kesalahan yang tidak ditangkap akan berhenti dengan pesan `BAHAYA [KODE]: Baris X - ...`.

## Contoh lengkap

```eve
fungsi bagi(a, b) {
    coba {
        pastikan(b != 0, "Pembagi tidak boleh nol!")
        kembali a / b
    } tangkap (e) {
        cetak("Tertangkap: ", e)
        kembali kosong
    }
}

cetak("10 / 2 = ", bagi(10, 2))
cetak("10 / 0 = ", bagi(10, 0))

variabel usia = -5
coba {
    jika usia < 0 {
        lempar("Usia tidak boleh negatif!")
    }
    cetak("Usia diterima: ", usia)
} tangkap (e) {
    cetak("Error: ", e)
}

coba {
    variabel d = [1, 2]
    cetak(d[9])
} tangkap (e) {
    cetak("Error: ", e)
} akhirnya {
    cetak("Blok akhirnya dijalankan")
}

pastikan(1 + 1 == 2, "Hitungan rusak!")
cetak("Semua aman!")
```

## Jalankan

```powershell
evernight coba_tangkap.eve
```

```
10 / 2 = 5
Tertangkap: Pembagi tidak boleh nol!
10 / 0 = kosong
Error: Usia tidak boleh negatif!
Error: Indeks 9 di luar batas daftar [0..1]
Blok akhirnya dijalankan
Semua aman!
```

Coba ubah baris terakhir jadi `pastikan(1 + 1 == 3, "Hitungan rusak!")` — program akan
berhenti dengan `BAHAYA [ASSERT] Baris ... - Hitungan rusak!`.

Contoh bawaan: `evernight examples/fitur_baru.eve`.

## Latihan

1. Buat fungsi `bagi_aman(a, b)` yang mengembalikan `"nol tidak boleh"` ketika `b == 0`.
2. Bungkus pembacaan angka dari pengguna dengan `coba`/`tangkap` agar input yang salah
   tidak menghentikan program.
3. Tulis satu `pastikan` dengan pesan error Anda sendiri, lalu lihat tampilan
   `BAHAYA [ASSERT]`-nya tanpa `coba`.
