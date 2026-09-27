---
judul: "Stage 6: Lists & Dicts"
grup: "Tutorial"
urutan: 6
---

# Stage 6: Lists & Dicts

## Goal

Store lots of data in one variable: **lists** for order, **dicts** for key-value
pairs. Example programs: a shopping list and a scoreboard.

## Concepts

### List

```eve
variabel belanja = ["beras", "telur", "gula"]
```

| Function | Purpose |
|----------|---------|
| `tambah(d, x)` | append at the end |
| `hapus(d, x)` | remove the matching element |
| `urutkan(d)` / `balik(d)` | new sorted / reversed list |
| `gabung_larik(a, b)` | merge two lists |
| `jumlah(d)` / `rata_rata(d)` | sum / average of numbers |
| `ada(d, x)` | whether an element exists |
| `d[i]` | access by index (starts at 0) |
| `d.panjang` | number of elements |

```eve
cetak(belanja[0])          # beras
cetak(belanja.panjang)     # 3
```

### Dict

```eve
variabel skor = { "Andi": 90, "Budi": 85 }
```

| Function | Purpose |
|----------|---------|
| `dapatkan(k, "kunci", bawaan)` | get a value (with fallback) |
| `setel(k, "kunci", nilai)` | add / change a value |
| `hapus_kunci(k, "kunci")` | delete an entry |
| `kunci(k)` / `nilai(k)` | list of all keys / values |
| `ada_kunci(k, "kunci")` | check whether a key exists |
| `k["kunci"]` | direct access |

> `dapatkan` without a default value fails when the key is missing. It is safer
> to always provide a default, for example `0`.

## Complete Example

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

## Run

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

Note: **the order of dict keys when printed can differ on every run** (dict
contents are not stored in order). Compare the values, not the order.

Built-in examples: `evernight examples/daftar.eve` and `evernight examples/kamus.eve`.

## Exercises

1. Make a `tugas` list with 3 activities; add 1, remove 1, then print its length.
2. Store your personal data in a dict (`nama`, `kota`, `umur`) and print it
   uppercase using `besar()` (Stage 7).
3. Calculate the average of 4 test scores, then print `"Lulus"` if the average is
   75 or above (use `jika` from Stage 3).
