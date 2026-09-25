---
judul: "Fungsi, Parameter, dan Kembali"
grup: "Panduan"
urutan: 3
---

# Fungsi, Parameter, dan Kembali

Halaman ini menjelaskan cara mendeklarasikan fungsi dengan `fungsi`, meneruskan nilai lewat parameter, mengembalikan nilai dengan `kembali`, serta memakai fungsi sebagai nilai (lambda/closure).

## Deklarasi fungsi

```eve
fungsi nama_fungsi(a, b) {
    kembali a + b
}
```

- Kata kunci `fungsi` (bukan `fn`/`def`).
- Nama fungsi disarankan `snake_case` (aturan linter `WKHURUF`).
- Daftar parameter dipisah koma; tanpa parameter tetap ditulis `()`.
- Badan wajib memakai `{ ... }`.

## Mengembalikan nilai

```eve
fungsi tambah(a, b) {
    kembali a + b
}
```

- `kembali` adalah kata kunci pengembalian nilai (bukan `return`).
- Fungsi **tanpa `kembali`** akan mengembalikan `kosong`.
- `kembali` tanpa nilai (di dalam `{ }` terakhir) juga menghasilkan `kosong`.
- Kode setelah `kembali` tidak akan pernah jalan → peringatan `PERINGATAN [WKREACH]`.

## Parameter

```eve
fungsi sapa(nama, salam = "Halo") {
    kembali salam + ", " + nama + "!"
}
```

- Nilai awal parameter **didukung sintaksisnya** (`salam = "Halo"`).
- **Catatan v0.1.0:** pemanggilan tetap wajib menyediakan **seluruh argumen** — `sapa("Budi")` akan error `BAHAYA [FUNGSI]: Fungsi 'sapa' butuh 2 argumen, diberikan 1`. Tulis `sapa("Budi", "Halo")`.
- Jumlah argumen harus pas (lebih banyak pun error) — lihat `BAHAYA [FUNGSI]`.

## Rekursi

```eve
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
```

## Fungsi sebagai nilai

Fungsi adalah nilai biasa: bisa disimpan di variabel, dimasukkan ke daftar, dan dikirim sebagai argumen (*first-class function*).

```eve
variabel kali_dua = fungsi(x) { kembali x * 2 }   # fungsi anonim / lambda
cetak(kali_dua(21))                                # 42
cetak(peta([1, 2, 3], kali_dua))                   # [2, 4, 6]
```

Fungsi tingkat tinggi yang tersedia: `peta`, `saring`, `lipat`, `setiap` (lihat [Panduan Percabangan](04-percabangan.md) dan STDLIB).

## Fungsi bersarang (closure)

```eve
variabel pengali = 3
fungsi buat_kali() {
    kembali fungsi(x) {
        kembali x * pengali
    }
}
cetak(buat_kali()(7))   # 21
```

Fungsi di dalam fungsi bisa membaca **variabel global**. Pada v0.1.0 penangkapan variabel *lokal* milik fungsi luar (parameter/variabel di dalam badan) **belum didukung** — jangan mengandalkannya dulu.

## Contoh program lengkap

Simpan sebagai `fungsi.eve`, lalu jalankan: `evernight fungsi.eve`

```eve
# 03 - Fungsi, parameter, dan fungsi sebagai nilai

# Fungsi bernama dengan deklarasi 'fungsi'
fungsi tambah(a, b) {
    kembali a + b
}
cetak("tambah(4, 6) = ", tambah(4, 6))

# Deklarasi parameter dengan nilai awal (sintaksis didukung).
# Catatan v0.1.0: pemanggilan tetap wajib menyediakan SEMUA argumen.
fungsi sapa(nama, salam = "Halo") {
    kembali salam + ", " + nama + "!"
}
cetak(sapa("Budi", "Halo"))
cetak(sapa("Ani", "Selamat malam"))

# Rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}
cetak("faktorial(6) = ", faktorial(6))

# Fungsi anonim (lambda) disimpan ke variabel
variabel kali_dua = fungsi(x) { kembali x * 2 }
cetak("kali_dua(21) = ", kali_dua(21))

# Fungsi sebagai nilai (first-class): dikirim sebagai argumen
cetak("peta: ", peta([1, 2, 3], kali_dua))
cetak("saring: ", saring([1, 2, 3, 4], fungsi(x) { kembali x % 2 == 0 }))
cetak("lipat: ", lipat([1, 2, 3, 4], 0, fungsi(acc, x) { kembali acc + x }))

# Fungsi tanpa 'kembali' mengembalikan kosong
fungsi sapa_langsung(nama) {
    cetak("Langsung: ", nama)
}
variabel hasil_kosong = sapa_langsung("Dunia")
cetak("Hasil fungsi tanpa kembali: ", hasil_kosong)

# Fungsi bersarang dapat membaca variabel GLOBAL
variabel pengali = 3
fungsi buat_kali() {
    kembali fungsi(x) {
        kembali x * pengali
    }
}
cetak("fungsi bersarang: ", buat_kali()(7))
```

Output:

```text
tambah(4, 6) = 10
Halo, Budi!
Selamat malam, Ani!
faktorial(6) = 720
kali_dua(21) = 42
peta: [2, 4, 6]
saring: [2, 4]
lipat: 10
Langsung: Dunia
Hasil fungsi tanpa kembali: kosong
fungsi bersarang: 21
```

## Kesalahan umum

- **Jumlah argumen tidak pas** → `BAHAYA [FUNGSI]: Fungsi 'x' butuh 2 argumen, diberikan 1`.
- **Memanggil yang bukan fungsi** (misal `variabel x = 10` lalu `x()`) → `BAHAYA [FUNGSI]: '10' bukan fungsi`.
- **Lupa `kembali`** → fungsi mengembalikan `kosong`, bukan error. Ini sering bikin hasil "kosong".
- **Membaca variabel lokal fungsi lain dari dalam lambda** → `BAHAYA [VARIABLE] ... tidak didefinisikan`. Pakai variabel global untuk v0.1.0.
- **Fungsi kosong** `fungsi f() { }` → `PERINGATAN [WKFUNG]`.
