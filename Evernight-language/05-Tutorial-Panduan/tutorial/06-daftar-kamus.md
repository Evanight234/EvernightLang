---
judul: "Tahap 6: Daftar & Kamus"
grup: "Tutorial"
urutan: 6
---

# Tahap 6: Daftar & Kamus

## Tujuan

Menyimpan banyak data dalam satu variabel: **daftar** untuk urutan, **kamus** untuk
pasangan kunci–nilai. Contoh program: daftar belanja dan papan skor.

## Konsep

### Daftar

```eve
variabel belanja = ["beras", "telur", "gula"]
```

| Fungsi | Kegunaan |
|--------|----------|
| `tambah(d, x)` | menambah di akhir |
| `hapus(d, x)` | menghapus elemen yang cocok |
| `urutkan(d)` / `balik(d)` | daftar baru terurut / terbalik |
| `gabung_larik(a, b)` | menggabungkan dua daftar |
| `jumlah(d)` / `rata_rata(d)` | total / rata-rata angka |
| `ada(d, x)` | ada atau tidak elemen |
| `d[i]` | akses lewat indeks (mulai dari 0) |
| `d.panjang` | jumlah elemen |

```eve
cetak(belanja[0])          # beras
cetak(belanja.panjang)     # 3
```

### Kamus

```eve
variabel skor = { "Andi": 90, "Budi": 85 }
```

| Fungsi | Kegunaan |
|--------|----------|
| `dapatkan(k, "kunci", bawaan)` | ambil nilai (dengan cadangan) |
| `setel(k, "kunci", nilai)` | tambah / ubah nilai |
| `hapus_kunci(k, "kunci")` | hapus entri |
| `kunci(k)` / `nilai(k)` | daftar semua kunci / nilai |
| `ada_kunci(k, "kunci")` | cek keberadaan kunci |
| `k["kunci"]` | akses langsung |

> `dapatkan` tanpa nilai cadangan akan gagal bila kunci tidak ada — lebih aman selalu
> menyediakan bawaan, misalnya `0`.

## Contoh lengkap

```eve
# Daftar belanja + kamus top skor
variabel belanja = ["beras", "telur", "gula"]
tambah(belanja, "minyak")
cetak("Belanja: ", belanja, " -> ", belanja.panjang, " item")

hapus(belanja, "gula")
cetak("Setelah hapus gula: ", belanja)
cetak("Ada telur? ", ada(belanja, "telur"))

variabel sayur = ["wortel", "bayam"]
cetak("Digabung: ", gabung_larik(belanja, sayur))
cetak("Terurut: ", urutkan(["jeruk", "apel", "mangga"]))

variabel angka_nilai = [80, 95, 70]
cetak("Jumlah: ", jumlah(angka_nilai), " rata-rata: ", rata_rata(angka_nilai))

# Kamus: top skor
variabel skor = { "Andi": 90, "Budi": 85, "Citra": 95 }
setel(skor, "Dewi", 88)
hapus_kunci(skor, "Budi")

cetak("Kunci: ", kunci(skor))
cetak("Nilai: ", nilai(skor))
cetak("Skor Citra: ", dapatkan(skor, "Citra", 0))
cetak("Skor Ani (bawaan 0): ", dapatkan(skor, "Ani", 0))
cetak("Ada kunci Andi: ", ada_kunci(skor, "Andi"))
cetak("Semua skor: ", skor)
```

## Jalankan

```powershell
evernight belanja.eve
```

```
Belanja: [beras, telur, gula, minyak] -> 4 item
Setelah hapus gula: [beras, telur, minyak]
Ada telur? benar
Digabung: [beras, telur, minyak, wortel, bayam]
Terurut: [apel, jeruk, mangga]
Jumlah: 245 rata-rata: 81.66666666666667
Kunci: [Andi, Citra, Dewi]
Nilai: [90, 95, 88]
Skor Citra: 95
Skor Ani (bawaan 0): 0
Ada kunci Andi: benar
Semua skor: {Citra: 95, Andi: 90, Dewi: 88}
```

Catatan: **urutan kunci kamus saat dicetak bisa berbeda tiap kali dijalankan**
(isi kamus tidak disimpan berurutan) — bandingkan nilainya, bukan urutannya.

Contoh bawaan: `evernight examples/daftar.eve` dan `evernight examples/kamus.eve`.

## Latihan

1. Buat daftar `tugas` berisi 3 kegiatan; tambahkan 1, hapus 1, lalu cetak jumlahnya.
2. Simpan data diri di kamus (`nama`, `kota`, `umur`) dan cetak per huruf besar
   memakai `besar()` (Tahap 7).
3. Hitung rata-rata 4 nilai ujian, lalu cetak `"Lulus"` bila rata-ratanya ≥ 75
   (gunakan `jika` dari Tahap 3).
