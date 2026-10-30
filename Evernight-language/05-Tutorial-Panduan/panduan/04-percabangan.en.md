---
judul: "Branching: jika and cocok"
grup: "Panduan"
urutan: 4
---

# Branching: if and match

This page explains `jika` / `lainnya_jika` / `lainnya` branching, value matching
with `cocok` + `kasus`, and the logical operators `dan`, `atau`, `bukan`.

## `jika`: the main branch

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
- `lainnya_jika <ekspresi> { ... }`: may be repeated (the `else if` equivalent,
  written with an **underscore**).
- `lainnya { ... }`: the final branch, no condition.
- Every branch requires `{ ... }`. No comma or `;` between blocks.

## Logical operators

| Operator | Symbol equivalent | Meaning |
|----------|-------------------|---------|
| `dan` | `&&` | Both sides are `benar` |
| `atau` | `\|\|` | Either side is `benar` |
| `bukan` | `!` | Flips the value |

```eve
jika umur >= 17 dan punya_ktp {
    cetak("Boleh masuk")
}
jika bukan aktif {
    cetak("Tidak aktif")
}
```

Comparison operators: `==`, `!=`, `<`, `<=`, `>`, `>=`, plus the words
`sama_dengan`, `lebih_dari`, `kurang_dari`.

### Truthiness

A value counts as **false** when it is: `salah`, `kosong`, the number `0`, the
empty text `""`, or an empty list/dict. Everything else is **true**.

## `cocok`: value matching

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

Exact syntax: `cocok <ekspresi> { <kasus> ... }`

- `kasus <nilai> { ... }`: branch when the value matches (`==`).
- `kasus _ { ... }`: wildcard (catches the rest).
- `bawaan { ... }`: branch when nothing matched.
- **No fall-through**: once one `kasus` matches, the other case blocks are skipped.
- The order of `kasus _` and `bawaan` is free; when both exist, `kasus _` takes
  priority.

## Full program example

Save it as `cabang.eve`, then run: `evernight cabang.eve`

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

## Common mistakes

- **`else if` (with a space)** is not recognized: write `lainnya_jika`.
- **`else`** is written `lainnya`.
- **`cocok` without `{`** → `BAHAYA [SYNTAX]: Diharapkan '{' pada blok 'cocok'!`.
- **Body of `cocok` not `kasus`/`bawaan`** →
  `BAHAYA [SYNTAX]: Diharapkan 'kasus', 'kasus _', atau 'bawaan' ...`.
- **`&&`/`||`/`!` are accepted**, but Evernight style writes
  `dan`/`atau`/`bukan`.
- **A `:` after the condition** (C style) does not exist: go straight to `{`.
