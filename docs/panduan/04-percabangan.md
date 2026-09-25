---
judul: "Percabangan: jika dan cocok"
grup: "Panduan"
urutan: 4
---

# Percabangan: jika dan cocok

Halaman ini menjelaskan percabangan `jika` / `lainnya_jika` / `lainnya`, pencocokan nilai `cocok` + `kasus`, serta operator logika `dan`, `atau`, `bukan`.

## `jika` — cabang utama

```eve
jika skor >= 90 {
    cetak("Nilai A")
} lainnya_jika skor >= 75 {
    cetak("Nilai B")
} lainnya {
    cetak("Nilai D")
}
```

- `jika <ekspresi> { ... }`
- `lainnya_jika <ekspresi> { ... }` — bisa diulang (padanan `else if`, ditulis dengan **garis bawah**).
- `lainnya { ... }` — cabang terakhir, tanpa kondisi.
- Semua cabang wajib `{ ... }`. Tidak ada tanda koma/`;` di antara blok.

## Operator logika

| Operator | Padanan simbol | Arti |
|----------|----------------|------|
| `dan` | `&&` | Kedua sisi bernilai benar |
| `atau` | `\|\|` | Salah satu sisi benar |
| `bukan` | `!` | Membalik nilai |

```eve
jika umur >= 17 dan punya_ktp {
    cetak("Boleh masuk")
}
jika bukan aktif {
    cetak("Tidak aktif")
}
```

Operator perbandingan: `==`, `!=`, `<`, `<=`, `>`, `>=`, serta kata `sama_dengan`, `lebih_dari`, `kurang_dari`.

### Kebenaran nilai (*truthiness*)

Nilai dianggap **salah** bila: `salah`, `kosong`, angka `0`, teks kosong `""`, daftar/kamus kosong. Selain itu **benar**.

## `cocok` — pencocokan nilai

```eve
cocok hari {
    kasus "Senin" {
        kembali "Awal pekan yang semangat!"
    }
    kasus _ {
        kembali "Hari biasa"
    }
}
```

Sintaksis persis: `cocok <ekspresi> { <kasus> ... }`

- `kasus <nilai> { ... }` — cabang bila nilai sama (`==`).
- `kasus _ { ... }` — wildcard (menangkap sisanya).
- `bawaan { ... }` — cabang bila tidak ada yang cocok.
- **Tidak ada *fall-through***: begitu satu `kasus` cocok, blok kasus lain dilewati.
- Urutan `kasus _` dan `bawaan` bebas; kalau keduanya ada, `kasus _` yang diprioritaskan.

## Contoh program lengkap

Simpan sebagai `cabang.eve`, lalu jalankan: `evernight cabang.eve`

```eve
# 04 - Percabangan: jika, lainnya_jika, lainnya, dan cocok

variabel skor = 78

# if / else if / else memakai 'jika', 'lainnya_jika', 'lainnya'
jika skor >= 90 {
    cetak("Nilai A")
} lainnya_jika skor >= 75 {
    cetak("Nilai B")
} lainnya_jika skor >= 60 {
    cetak("Nilai C")
} lainnya {
    cetak("Nilai D")
}

# Operator logika: dan, atau, bukan
variabel umur = 20
variabel punya_ktp = benar
jika umur >= 17 dan punya_ktp {
    cetak("Boleh masuk")
}
jika umur < 17 atau bukan punya_ktp {
    cetak("Tidak boleh")
} lainnya {
    cetak("Syarat terpenuhi")
}

# Pencocokan nilai dengan 'cocok' + 'kasus'
fungsi sapa_hari(hari) {
    cocok hari {
        kasus "Senin" {
            kembali "Awal pekan yang semangat!"
        }
        kasus "Sabtu" {
            kembali "Akhir pekan!"
        }
        kasus _ {
            kembali "Hari biasa"
        }
    }
}
cetak(sapa_hari("Senin"))
cetak(sapa_hari("Rabu"))

# 'bawaan' sebagai cabang penampung terakhir
fungsi jenis_biaya(n) {
    cocok n {
        kasus 0 {
            kembali "gratis"
        }
        bawaan {
            kembali "berbayar"
        }
    }
}
cetak(jenis_biaya(0), " / ", jenis_biaya(5))
```

Output:

```text
Nilai B
Boleh masuk
Syarat terpenuhi
Awal pekan yang semangat!
Hari biasa
gratis / berbayar
```

## Kesalahan umum

- **`else if` (spasi)** tidak dikenal — tulis `lainnya_jika`.
- **`else`** ditulis `lainnya`.
- **`cocok` tanpa `{`** → `BAHAYA [SYNTAX]: Diharapkan '{' pada blok 'cocok'!`.
- **Isi `cocok` bukan `kasus`/`bawaan`** → `BAHAYA [SYNTAX]: Diharapkan 'kasus', 'kasus _', atau 'bawaan' ...`.
- **`&&`/`||`/`!` diterima**, tetapi gaya Evernight menulis `dan`/`atau`/`bukan`.
- **Tanda `:` setelah kondisi** (gaya C) tidak ada — langsung `{`.
