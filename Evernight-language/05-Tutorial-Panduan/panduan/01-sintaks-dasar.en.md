---
judul: "Basic Syntax Structure"
grup: "Panduan"
urutan: 1
---

# Basic Syntax Structure

This page explains the anatomy of an EvernightLanguage program: how to write
comments, separate statements, write expressions, and arrange code blocks. When
you are done, you will be able to read any `.eve` program.

## Anatomy of a `.eve` file

- Program files use the **`.eve`** extension (example: `halo.eve`).
- Statements are executed **top to bottom** (top-down).
- The `utama()` function is only **optional**: if you have one, you call it
  yourself on the last line.
- Keywords are written **all lowercase** and are *case-sensitive* (`Jika` ≠ `jika`).

```eve
cetak("pernyataan pertama")
cetak("pernyataan kedua")
```

## Comments

Comments use the `#` sign and run to the end of the line. Comments are skipped
by the compiler.

```eve
# Ini komentar satu baris
variabel x = 1   # komentar di akhir baris juga boleh
```

## Statement separator

**A newline is the statement separator.** You do not need to write semicolons:

```eve
variabel a = 1
variabel b = 2
cetak(a + b)
```

A semicolon (`;`) may be written at the end of a statement, but it is not
required: just pick one consistent style (without semicolons).

## Code blocks

The bodies of `jika`, `selama`, `untuk`, `cocok`, `fungsi`, and `coba` **must**
use curly braces `{ ... }`:

```eve
jika benar {
    cetak("selalu pakai kurung kurawal")
}
```

There is no indentation rule: indentation is only for readability.

## Expressions

An expression is a piece of code that produces a value: literals (`42`,
`"teks"`, `benar`), operators (`+ - * / % **`), function calls (`faktorial(5)`),
indexing (`daftar[0]`), and property access (`teks.panjang`).

### Operator precedence (lowest to highest)

| Level | Operator | Description |
|------:|----------|-------------|
| 1 | `=` `+=` `-=` `*=` `/=` | Assignment |
| 2 | `atau`, `\|\|` | Logical OR |
| 3 | `dan`, `&&` | Logical AND |
| 4 | `==` `!=` `sama_dengan` | Equality |
| 5 | `<` `<=` `>` `>=` `lebih_dari` `kurang_dari` `dalam` | Comparison |
| 6 | `+` `-` | Addition & subtraction |
| 7 | `*` `/` `%` | Multiplication, division, remainder |
| 8 | `**` | Power (computed right to left) |
| 9 | `-` (unary), `bukan`, `!` | Negation |
| 10 | `f(...)`, `a[i]`, `o.p` | Calls, indexing, properties |
| 11 | literal, identity, `fungsi(...)` | Basic elements |

Because `*` is higher than `+`, the expression `1 + 2 * 3` is `7`. Use
parentheses `()` to force the order: `(1 + 2) * 3` is `9`.

## Full program example

Save it as `dasar.eve`, then run: `evernight dasar.eve`

```eve
# 01 - Struktur dasar program EvernightLanguage

# Komentar memakai tanda '#' dan berlaku sampai akhir baris
variabel nama = "Nusantara"
variabel tahun = 2026

cetak("Selamat datang di ", nama, "!")
cetak("Tahun: ", tahun)

# Ekspresi aritmatika & operator
variabel luas = 6 * 7
cetak("Luas = ", luas)
cetak("Sisa bagi = ", luas % 5)
cetak("Pangkat = ", 2 ** 8)
cetak("Gabungan teks: ", "Ever" + "night")
cetak("Perbandingan: ", luas > 40)
cetak("Logika: ", luas > 40 dan tahun > 2000)
cetak("Negasi: ", bukan (luas < 10))

# Blok kode selalu memakai kurung kurawal { }
jika luas > 10 {
    variabel status = "besar"
    cetak("Status: ", status)
}
```

Output:

```text
Selamat datang di Nusantara!
Tahun: 2026
Luas = 42
Sisa bagi = 2
Pangkat = 256
Gabungan teks: Evernight
Perbandingan: benar
Logika: benar
Negasi: benar
Status: besar
```

## Common mistakes

- **`//` is not a comment.** `//` is read as an unexpected `/` token →
  `BAHAYA [SYNTAX]: Token tak terduga: '/'`. Use `#`.
- **Missing `{ }`.** `jika benar` without curly braces →
  `BAHAYA [SYNTAX]: Diharapkan pembuka blok '{'!`.
- **Unbalanced quotes.** `"halo` with no closing quote →
  `BAHAYA [SYNTAX]: Penutup tanda petik tidak ditemukan!`.
- **`cetak` without parentheses.** `cetak "halo"` is invalid; write `cetak("halo")`.
- **Note:** `cetak` joins values with no separator, so write spaces yourself:
  `cetak("Nilai: ", x)`.
