---
judul: "Stage 10: Matra Ecosystem & Linear Algebra"
grup: "Tutorial"
urutan: 10
---

# Stage 10: Matra Ecosystem & Linear Algebra

## Goal

Explore the unified `Matra` data science ecosystem: create n-dimensional arrays
(vectors and matrices), execute broadcasting arithmetic operations, manipulate
tensor shapes, and compute linear algebra and descriptive statistics.

## Concepts

### Importing the `matra` Library

A single import unlocks complete numerical computing capabilities:

```eve
impor "matra" sebagai mt
```

### Creating Arrays (Vectors & Matrices)

The `mt.larik(...)` function accepts 1D lists or nested 2D lists:

```eve
variabel v = mt.larik([10, 20, 30])
variabel A = mt.larik([
    [1, 2],
    [3, 4]
])
cetak("Bentuk v: ", v.bentuk)
cetak("Bentuk A: ", A.bentuk)
```

The `.bentuk` property returns array dimensions (such as `[3]` for a vector or `[2, 2]` for a matrix).

### Fast Matrix Initialization

Matra provides dedicated builder functions:

```eve
variabel nol2x2 = mt.nol(2, 2)
variabel satu3x3 = mt.satu(3, 3)
variabel rentang = mt.rentang(0, 10, 2)
```

### Arithmetic Operations & Broadcasting

Arithmetic operations between arrays or an array and a scalar automatically apply broadcasting:

```eve
variabel a = mt.larik([1, 2, 3])
variabel b = a * 2 + 5
cetak("Hasil: ", b)
```

### Linear Algebra: Dot Product & Transpose

To calculate dot products between vectors or 2D matrix multiplication:

```eve
variabel X = mt.larik([[1, 2], [3, 4]])
variabel Y = mt.larik([[5, 6], [7, 8]])

variabel Z = X.titik(Y)
variabel T = X.transpos()
cetak("X . Y = ", Z)
cetak("X transpos = ", T)
```

### Descriptive Statistics

Array objects provide built-in native-speed statistical methods:

```eve
variabel data = mt.larik([12, 15, 18, 20, 25])
cetak("Rata-rata: ", data.rata_rata())
cetak("Varians: ", data.varians())
cetak("Deviasi standar: ", data.deviasi_standar())
```

## Complete example

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

## Run

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

## Exercises

1. Create a 3x3 zero matrix using `mt.nol(3, 3)` and set its diagonal elements to 1.
2. Build a vector containing numbers 1 to 100 with `mt.rentang(1, 101, 1)` and compute its mean and standard deviation.
3. Multiply a 2x3 matrix with a 3x2 matrix using `.titik()` and print the `.bentuk` of the resulting matrix.
