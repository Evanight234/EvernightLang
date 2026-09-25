---
judul: "Tahap 10: Mini Proyek — Kalkulator Interaktif"
grup: "Tutorial"
urutan: 10
---

# Tahap 10: Mini Proyek — Kalkulator Interaktif

## Tujuan

Menggabungkan seluruh tahap 1–9 dalam satu program utuh: menu interaktif dengan
`baca()`, percabangan `jika`/`lainnya_jika`, perulangan `selama`, fungsi, daftar
riwayat, konversi teks, dan penanganan kesalahan.

## Konsep

Program ini bekerja seperti ini:

1. **`selama pilihan != "6"`** — menu terus ditampilkan sampai pengguna memilih keluar.
2. **`baca(...)`** membaca pilihan; **`baca_angka(...)`** membaca kedua operand.
3. **`jika` / `lainnya_jika`** memilih operasi (pengganti `cocok` sederhana).
4. **`coba` / `tangkap` + `pastikan`** menahan kesalahan pembagian dengan nol.
5. **Daftar `riwayat`** menyimpan setiap perhitungan sebagai teks, ditampilkan ulang
   pada menu 5.

Kunci kecil yang dipakai:

```eve
pastikan(pilihan != "4" atau b != 0, "Pembagi tidak boleh nol!")
teks(a) + " + " + teks(b)   # angka harus diubah ke teks dulu
tambah(riwayat, catatan)     # daftar (Tahap 6)
```

## Contoh lengkap

```eve
# Kalkulator interaktif sederhana (gabungan tahap 1 - 9)
variabel riwayat = []
variabel pilihan = ""

fungsi tampilkan_menu() {
    cetak("========= KALKULATOR =========")
    cetak("1. Jumlah (+)     2. Kurang (-)")
    cetak("3. Kali (*)       4. Bagi (/)")
    cetak("5. Lihat riwayat  6. Keluar")
    cetak("==============================")
}

fungsi simbol_operasi(op) {
    jika op == "1" { kembali "+" }
    lainnya_jika op == "2" { kembali "-" }
    lainnya_jika op == "3" { kembali "*" }
    lainnya { kembali "/" }
}

selama pilihan != "6" {
    tampilkan_menu()
    pilihan = baca("Pilih menu: ")

    jika pilihan == "1" atau pilihan == "2" atau pilihan == "3" atau pilihan == "4" {
        variabel a = baca_angka("  Bilangan pertama : ")
        variabel b = baca_angka("  Bilangan kedua   : ")
        coba {
            pastikan(pilihan != "4" atau b != 0, "Pembagi tidak boleh nol!")
            variabel hasil = 0
            jika pilihan == "1" {
                hasil = a + b
            } lainnya_jika pilihan == "2" {
                hasil = a - b
            } lainnya_jika pilihan == "3" {
                hasil = a * b
            } lainnya {
                hasil = a / b
            }
            variabel catatan = teks(a) + " " + simbol_operasi(pilihan) + " " + teks(b) + " = " + teks(hasil)
            tambah(riwayat, catatan)
            cetak("  Hasil: ", hasil)
        } tangkap (e) {
            cetak("  Gagal: ", e)
        }
    } lainnya_jika pilihan == "5" {
        jika riwayat.panjang == 0 {
            cetak("  Riwayat masih kosong")
        } lainnya {
            cetak("  === Riwayat ===")
            untuk i dari 0 sampai riwayat.panjang - 1 {
                cetak("  ", riwayat[i])
            }
        }
    } lainnya_jika pilihan == "6" {
        cetak("Riwayat tersimpan: ", riwayat.panjang, " operasi")
        cetak("Sampai jumpa!")
    } lainnya {
        cetak("  Pilihan tidak dikenal, coba lagi")
    }
}
```

## Jalankan

Simpan sebagai `kalkulator.eve`, buka terminal di folder itu, lalu:

```powershell
evernight kalkulator.eve
```

Coba urutan input ini (ketik tiap baris lalu tekan Enter):

```
1
10
4
3
6
7
4
10
0
5
9
6
```

Output (menu dicetak ulang tiap iterasi; berikut potongan pentingnya):

```
========= KALKULATOR =========
...
==============================
Pilih menu:   Bilangan pertama :   Bilangan kedua   :   Hasil: 14
========= KALKULATOR =========
...
Pilih menu:   Bilangan pertama :   Bilangan kedua   :   Hasil: 42
========= KALKULATOR =========
...
Pilih menu:   Bilangan pertama :   Bilangan kedua   :   Gagal: Pembagi tidak boleh nol!
========= KALKULATOR =========
...
Pilih menu:   === Riwayat ===
  10 + 4 = 14
  6 * 7 = 42
========= KALKULATOR =========
...
Pilih menu:   Pilihan tidak dikenal, coba lagi
========= KALKULATOR =========
...
Pilih menu: Riwayat tersimpan: 2 operasi
Sampai jumpa!
```

## Latihan

1. Tambahkan operasi **sisa bagi (%)** sebagai pilihan baru.
2. Tambahkan menu **"Hapus riwayat"** yang mengosongkan daftar (petunjuk: buat daftar
   baru `riwayat = []`).
3. Simpan riwayat ke berkas `./riwayat.txt` dengan `tulis_file` (Tahap 8) setiap kali
   ada perhitungan baru.
4. Tantangan: tampilkan riwayat dari yang terbaru memakai `balik(riwayat)`.
