---
judul: "Tahap 4: Perulangan"
grup: "Tutorial"
urutan: 4
---

# Tahap 4: Perulangan

## Tujuan

Mengulang pekerjaan dengan `untuk` dan `selama`, lalu mengendalikan alur memakai
`berhenti` dan `lanjut` lewat program deret bilangan.

## Konsep

### `untuk i dari A sampai B`

Menghitung dari `A` sampai `B` — **kedua ujung ikut** dihitung.

```eve
untuk i dari 1 sampai 5 {
    cetak(i)      # 1, 2, 3, 4, 5
}
```

### `untuk x dalam koleksi`

Mengiterasi isi daftar (atau karakter teks):

```eve
untuk buah dalam ["apel", "jeruk"] {
    cetak(buah)
}
```

> Saat ini kompilator menampilkan `PERINGATAN [WKVAR]` yang keliru untuk perintah
> ini. Peringatan itu aman diabaikan — program tetap berjalan benar.

### `selama kondisi { }`

Berulang selama kondisi masih `benar`. Kita harus mengubah sendiri penghitungnya.

```eve
variabel i = 0
selama i < 3 {
    i = i + 1
    cetak(i)
}
```

### `berhenti` dan `lanjut`

- `berhenti` — keluar dari perulangan.
- `lanjut` — langsung ke iterasi berikutnya (lewati sisa blok).

Keduanya dipakai di dalam perulangan `selama`:

```eve
selama i < 10 {
    i = i + 1
    jika i % 2 != 0 {
        lanjut          # lewati ganjil
    }
    jika i > 6 {
        berhenti        # berhenti di 8
    }
    cetak(i)
}
```

## Contoh lengkap

```eve
# Deret bilangan
cetak("Deret 1 sampai 5:")
untuk i dari 1 sampai 5 {
    cetak(i, " ")
}
cetak("")

variabel jumlah = 0
untuk i dari 1 sampai 5 {
    jumlah = jumlah + i
}
cetak("Jumlah 1..5 = ", jumlah)

# Berhenti & lanjut dipakai di dalam selama
variabel langkah = 0
variabel total = 0
selama langkah < 20 {
    langkah = langkah + 1
    jika langkah % 2 != 0 {
        lanjut
    }
    jika langkah > 12 {
        berhenti
    }
    total = total + langkah
}
cetak("Jumlah bilangan genap sampai 12 = ", total)

# Iterasi isi daftar
variabel buah = ["apel", "mangga", "jeruk"]
untuk b dalam buah {
    cetak("Buah: ", b)
}
```

## Jalankan

```powershell
evernight deret.eve
```

```
Deret 1 sampai 5:
1 
2 
3 
4 
5 

Jumlah 1..5 = 15
Jumlah bilangan genap sampai 12 = 42
Buah: apel
Buah: mangga
Buah: jeruk
```

Sebelum baris `Buah: apel` Anda mungkin melihat 2 baris `PERINGATAN [WKVAR]` — abaikan,
itu bug kecil kompilator untuk `untuk ... dalam`, bukan kesalahan program Anda.

## Latihan

1. Cetak deret mundur dari 10 sampai 1 dengan `untuk`.
2. Hitung jumlah bilangan dari 1 sampai 100 **hanya** yang kelipatan 3, memakai
   `selama` + `lanjut`.
3. Ubah bagian `berhenti` agar berhenti tepat setelah total melewati 30. Berapa
   nilai `total` akhirnya?
