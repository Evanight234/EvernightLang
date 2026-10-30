---
judul: "Tahap 5: Fungsi"
grup: "Tutorial"
urutan: 5
---

# Tahap 5: Fungsi

## Tujuan

Menulis fungsi sendiri dengan parameter dan nilai kembali, lalu memakai rekursi
sebagai cara menghitung berulang.

## Konsep

### Mendefinisikan fungsi

```eve
fungsi tambah(a, b) {
    kembali a + b
}

cetak(tambah(2, 3))   # 5
```

- `fungsi nama(parameter, ...) { }` — deklarasi.
- `kembali nilai` — mengirim nilai keluar. Tanpa `kembali`, hasilnya `kosong`.
- Fungsi tanpa parameter tetap ditulis dengan kurung kosong: `fungsi sapa() { }`.

### Fungsi tanpa nilai kembali

Fungsi boleh hanya melakukan tindakan (misalnya mencetak):

```eve
fungsi sapa(nama) {
    cetak("Halo, ", nama, "!")
}
sapa("Budi")
```

### Rekursi

Fungsi yang memanggil dirinya sendiri. Wajib ada **basi** (kondisi berhenti):

```eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1            # basi
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
```

`faktorial(5)` → 5 × 4 × 3 × 2 × 1 = `120`.

### Fungsi sebagai nilai

Fungsi anonim boleh disimpan di variabel (kunci rekursi dan pemrograman fungsional):

```eve
variabel ganda = fungsi(x) {
    kembali x * 2
}
cetak(ganda(21))   # 42
```

## Contoh lengkap

```eve
# Fungsi: parameter, kembali, dan rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

fungsi kuadrat(x) {
    kembali x * x
}

fungsi sapa(nama) {
    cetak("Halo, ", nama, "!")
}

fungsi ganjil(n) {
    jika n % 2 == 1 {
        kembali benar
    }
    kembali salah
}

variabel angka = 5
cetak("Faktorial ", angka, " = ", faktorial(angka))
cetak("Faktorial 0 = ", faktorial(0))
cetak("Kuadrat 7 = ", kuadrat(7))
sapa("Budi")
cetak("7 ganjil? ", ganjil(7))
cetak("8 ganjil? ", ganjil(8))

# Fungsi sebagai nilai (anonim)
variabel ganda = fungsi(x) {
    kembali x * 2
}
cetak("Dua kali 21 = ", ganda(21))
```

## Jalankan

```powershell
evernight fungsi.eve
```

```
Faktorial 5 = 120
Faktorial 0 = 1
Kuadrat 7 = 49
Halo, Budi!
7 ganjil? benar
8 ganjil? salah
Dua kali 21 = 42
```

Contoh bawaan yang sama:

```powershell
evernight examples/faktorial.eve
```

## Latihan

1. Buat fungsi `sapa_terbalik(a, b)` yang mencetak `b` lalu `a`.
2. Tulis fungsi rekursi `pangkat_rekursif(x, n)` (x pangkat n) dengan basi `n == 0`
   menghasilkan `1`.
3. Modifikasi `faktorial` agar mencetak pesan dan `kembali kosong` bila diberi
   bilangan negatif.
