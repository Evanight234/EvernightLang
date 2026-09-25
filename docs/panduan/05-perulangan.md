---
judul: "Perulangan: selama, untuk, berhenti, lanjut"
grup: "Panduan"
urutan: 5
---

# Perulangan: selama, untuk, berhenti, lanjut

Halaman ini menjelaskan tiga bentuk perulangan (`selama`, `untuk ... dari ... sampai ...`, `untuk ... dalam ...`) serta kontrol keluar `berhenti` dan `lanjut`.

## `selama` — ulang selama kondisi benar

```eve
variabel hitung = 1
selama hitung <= 5 {
    cetak("hitung: ", hitung)
    hitung = hitung + 1
}
```

Pastikan kondisi bisa menjadi `salah`, kalau tidak program tidak akan berhenti.

## `untuk ... dari ... sampai ...` — rentang angka

```eve
untuk i dari 1 sampai 5 {
    cetak(i)
}
```

- Batas akhir **ikut dihitung** (inklusif): `dari 1 sampai 5` menghasilkan `1, 2, 3, 4, 5`.
- Variabel loop dibuat otomatis di dalam blok.

## `untuk ... dalam ...` — iterasi koleksi

```eve
variabel buah = ["mangga", "jambu", "kelapa"]
untuk item dalam buah {
    cetak("Buah: ", item)
}
```

Bisa juga untuk teks: `untuk huruf dalam "eve" { ... }`.

## `berhenti` — keluar dari perulangan

```eve
untuk i dari 0 sampai 100 {
    cetak("hitung: ", i)
    jika i >= 3 {
        berhenti
    }
}
```

## `lanjut` — lanjut ke iterasi berikutnya

Paling stabil dipakai di dalam `selama`:

```eve
variabel n = 0
selama n < 6 {
    n = n + 1
    jika n % 2 == 0 {
        lanjut
    }
    cetak("ganjil: ", n)
}
```

## Contoh program lengkap

Simpan sebagai `loop.eve`, lalu jalankan: `evernight loop.eve`

```eve
# 05 - Perulangan: selama, untuk, berhenti, lanjut

# 'selama' - mengulang selama kondisi bernilai benar
variabel hitung = 1
selama hitung <= 5 {
    cetak("selama: ", hitung)
    hitung = hitung + 1
}

# 'lanjut' paling stabil dipakai di dalam 'selama'
variabel n = 0
selama n < 6 {
    n = n + 1
    jika n % 2 == 0 {
        lanjut
    }
    cetak("ganjil: ", n)
}

# 'untuk ... dari ... sampai ...' - rentang angka, batas akhir IKUT dihitung
variabel total = 0
untuk i dari 1 sampai 5 {
    total = total + i
}
cetak("Jumlah 1 sampai 5 = ", total)

# 'untuk ... dalam ...' - mengiterasi isi daftar
variabel buah = ["mangga", "jambu", "kelapa"]
untuk item dalam buah {
    cetak("Buah: ", item)
}

# 'berhenti' - keluar dari perulangan.
# Letakkan 'berhenti' sebagai pernyataan terakhir di dalam bloknya.
untuk i dari 0 sampai 100 {
    cetak("hitung: ", i)
    jika i >= 3 {
        berhenti
    }
}

# Kombinasi: cari nilai pertama dalam daftar
variabel angka = [7, 3, 9, 1]
variabel ketemu = -1
untuk i dari 0 sampai 3 {
    jika angka[i] == 9 {
        ketemu = i
    }
    jika ketemu >= 0 {
        berhenti
    }
}
cetak("Index nilai 9 = ", ketemu)
```

Output:

```text
selama: 1
selama: 2
selama: 3
selama: 4
selama: 5
ganjil: 1
ganjil: 3
ganjil: 5
Jumlah 1 sampai 5 = 15
Buah: mangga
Buah: jambu
Buah: kelapa
hitung: 0
hitung: 1
hitung: 2
hitung: 3
Index nilai 9 = 2
```

## Kesalahan umum

- **`for i in range(...)` / `while true`** — gaya itu bahasa lain. Di sini: `untuk i dari 0 sampai 5` dan `selama benar { ... }`.
- **Lupa memperbarui kondisi `selama`** → perulangan tak berujung.
- **`berhenti`/`lanjut` diikuti kode lain di blok yang sama** → variabel loop tidak terbaca lagi (`BAHAYA [VARIABLE]`). Letakkan `berhenti`/`lanjut` sebagai **pernyataan terakhir** dalam bloknya.
- **`lanjut` di dalam `untuk ... dari ...` atau `untuk ... dalam ...`** belum stabil pada v0.1.0 (counter tidak bertambah / variabel loop hilang). Gunakan `selama` untuk pola `lanjut`.
- **`untuk ... dalam` memunculkan `PERINGATAN [WKVAR] Variabel '(indeks)'/'(koleksi)'`** — kebisingan compiler yang diketahui dan aman diabaikan; program tetap berjalan normal.
- **`dari ... sampai ...` bukan eksklusif** — kalau mau berhenti sebelum batas, tulis batas akhir dikurangi 1.
