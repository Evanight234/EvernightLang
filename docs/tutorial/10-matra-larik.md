---
judul: "Tahap 10: Ekosistem Matra & Aljabar Linier"
grup: "Tutorial"
urutan: 10
---

# Tahap 10: Ekosistem Matra & Aljabar Linier

## Tujuan

Mengenal ekosistem sains data terpadu `Matra`: membuat larik n-dimensi (vektor & matriks),
menjalankan operasi aritmatika *broadcasting*, manipulasi bentuk tensor, dan kalkulasi
aljabar linier serta statistik dasar.

## Konsep

### Mengimpor Pustaka `matra`

Cukup satu kali impor untuk membuka seluruh kapabilitas komputasi numerik:

```eve
impor "matra" sebagai mt
```

### Membuat Larik (Vektor & Matriks)

Fungsi `mt.larik(...)` menerima daftar 1D maupun daftar bersarang 2D:

```eve
variabel v = mt.larik([10, 20, 30])
variabel A = mt.larik([
    [1, 2],
    [3, 4]
])
cetak("Bentuk v: ", v.bentuk)
cetak("Bentuk A: ", A.bentuk)
```

Properti `.bentuk` mengembalikan dimensi larik (misal `[3]` untuk vektor atau `[2, 2]` untuk matriks).

### Inisialisasi Matriks Cepat

Matra menyediakan fungsi pembuat matriks khusus:

```eve
variabel nol2x2 = mt.nol(2, 2)
variabel satu3x3 = mt.satu(3, 3)
variabel rentang = mt.rentang(0, 10, 2)
```

### Operasi Aritmatika & *Broadcasting*

Operasi aritmatika antar-larik atau larik dengan skalar bekerja secara otomatis (*broadcasting*):

```eve
variabel a = mt.larik([1, 2, 3])
variabel b = a * 2 + 5
cetak("Hasil: ", b)
```

### Aljabar Linier: Perkalian Titik & Transposisi

Untuk menghitung perkalian titik (*dot product*) antar vektor atau perkalian matriks 2D:

```eve
variabel X = mt.larik([[1, 2], [3, 4]])
variabel Y = mt.larik([[5, 6], [7, 8]])

variabel Z = X.titik(Y)
variabel T = X.transpos()
cetak("X . Y = ", Z)
cetak("X transpos = ", T)
```

### Statistik Deskriptif

Objek larik memiliki method statistik bawaan berkecepatan native:

```eve
variabel data = mt.larik([12, 15, 18, 20, 25])
cetak("Rata-rata: ", data.rata_rata())
cetak("Varians: ", data.varians())
cetak("Deviasi standar: ", data.deviasi_standar())
```

## Contoh lengkap

```eve
impor "matra" sebagai mt

# 1. Membuat dua matriks 2x2
variabel A = mt.larik([
    [1, 2],
    [3, 4]
])

variabel B = mt.larik([
    [5, 6],
    [7, 8]
])

# 2. Operasi aritmatika elemen per elemen
variabel tambah = A + B
cetak("A + B = ", tambah)

# 3. Perkalian matriks (dot product)
variabel hasil_kali = A.titik(B)
cetak("A . B = ", hasil_kali)

# 4. Transposisi dan statistik
variabel A_t = A.transpos()
cetak("A Transpos = ", A_t)
cetak("Nilai minimum A = ", A.min())
cetak("Nilai maksimum A = ", A.max())
cetak("Rata-rata elemen A = ", A.rata_rata())
```

## Jalankan

```powershell
evernight coba_matra.eve
```

```
A + B = Larik([[6, 8], [10, 12]])
A . B = Larik([[19, 22], [43, 50]])
A Transpos = Larik([[1, 3], [2, 4]])
Nilai minimum A = 1
Nilai maksimum A = 4
Rata-rata elemen A = 2.5
```

## Latihan

1. Buat matriks identitas 3x3 menggunakan `mt.nol(3, 3)`, lalu ubah elemen diagonalnya menjadi 1.
2. Buat vektor berisi angka 1 sampai 100 dengan `mt.rentang(1, 101, 1)` dan hitung rata-rata serta deviasi standarnya.
3. Kalikan matriks 2x3 dengan matriks 3x2 menggunakan method `.titik()`, lalu cetak ukuran bentuk (`.bentuk`) hasilnya.
