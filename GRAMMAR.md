# GRAMMAR.md — Tata Bahasa Formal EvernightLanguage

> Dokumen ini mendefinisikan aturan tata bahasa formal (EBNF) untuk EvernightLanguage.
> Setiap program `.eve` yang valid harus mematuhi aturan di bawah ini.

---

## 1. Tata Bahasa Leksikal (Lexical Grammar)

### 1.1 Identifier
```
Identifier ::= [a-zA-Z_][a-zA-Z0-9_]*
```

### 1.2 Literal Angka
```
Number ::= [0-9]+ ("." [0-9]+)?
```

### 1.3 Literal Teks
```
String ::= '"' ([^"\\] | EscapeSequence)* '"'
         | "'" ([^'\\] | EscapeSequence)* "'"

EscapeSequence ::= "\n" | "\t" | "\\\\" | "\\\"" | "\\'"
```

### 1.4 Literal Boolean & Kosong
```
Boolean ::= "benar" | "salah"
Null     ::= "kosong"
```

### 1.5 Komentar
```
Comment ::= "#" [^\n]*
```
Komentar diabaikan oleh parser dan tidak dieksekusi.

---

## 2. Tata Bahasa Pernyataan (Statements & Declarations)

### 2.1 Program
```
Program ::= Statement*
```

### 2.2 Statement
```
Statement ::= VarDeclaration
            | ConstDeclaration
            | FuncDeclaration
            | IfStatement
            | MatchStatement
            | WhileStatement
            | ForStatement
            | ReturnStatement
            | BreakStatement
            | ContinueStatement
            | TryCatchStatement
            | ThrowStatement
            | AssertStatement
            | ImportStatement
            | ExprStatement
            | Block
```

### 2.3 Blok
```
Block ::= "{" Statement* "}"
```

### 2.4 Deklarasi Variabel
```
VarDeclaration ::= "variabel" Identifier ("=" Expression)?
```

### 2.5 Deklarasi Konstanta
```
ConstDeclaration ::= "tetap" Identifier "=" Expression
```

### 2.6 Deklarasi Fungsi
```
FuncDeclaration ::= "fungsi" Identifier "(" ParameterList? ")" Block

ParameterList   ::= Parameter ("," Parameter)*
Parameter       ::= Identifier ("=" Expression)?
```

### 2.7 Percabangan (If-Else)
```
IfStatement  ::= "jika" Expression Block
                 ("lainnya_jika" Expression Block)*
                 ("lainnya" Block)?
```

### 2.8 Pencocokan Pola (Match/Switch)
```
MatchStatement   ::= "cocok" Expression "{" CaseClause* WildcardClause? DefaultClause? "}"
CaseClause       ::= "kasus" Expression Block
WildcardClause   ::= "kasus" "_" Block
DefaultClause    ::= "bawaan" Block
```

### 2.9 Perulangan (Loops)
```
WhileStatement ::= "selama" Expression Block
ForStatement   ::= "untuk" Identifier "dari" Expression "sampai" Expression Block
                 | "untuk" Identifier "dalam" Expression Block
BreakStatement ::= "berhenti"
ContinueStatement ::= "lanjut"
```

### 2.10 Pengembalian Nilai
```
ReturnStatement ::= "kembali" Expression?
```

### 2.11 Penanganan Kesalahan (Error Handling)
```
TryCatchStatement ::= "coba" Block
                      "tangkap" "(" Identifier ")" Block
                      ("akhirnya" Block)?

ThrowStatement    ::= "lempar" Expression
AssertStatement   ::= "pastikan" "(" Expression ("," Expression)? ")"
```

### 2.12 Sistem Modul
```
ImportStatement ::= "impor" Identifier ("sebagai" Identifier)?
                 | "impor" ImportList "dari" (String | Identifier)

ImportList      ::= Identifier ("," Identifier)*
```
> **Implementasi v1 (2026-09-10):** hanya bentuk `impor "path/modul" [sebagai alias]` yang didukung untuk berkas `.eve` lintas berkas (nama modul boleh identifier atau string). Bentuk `impor ImportList dari ...` **belum diimplementasikan** (ditunda). Modul standar (`impor matematika`, dll.) tetap no-op.

### 2.13 Pernyataan Ekspresi
```
ExprStatement ::= Expression
```

---

## 3. Tata Bahasa Ekspresi (Expression Grammar)

### Presedensi Operator (dari rendah ke tinggi)

| Level | Presedensi | Operator | Deskripsi |
|-------|------------|----------|-----------|
| 1 | Assignment (paling rendah) | `=`, `+=`, `-=`, `*=`, `/=` | Penugasan |
| 2 | Logical OR | `atau`, `\|\|` | Atau logika |
| 3 | Logical AND | `dan`, `&&` | Dan logika |
| 4 | Equality | `==`, `!=`, `sama_dengan` | Kesetaraan |
| 5 | Relational | `<`, `<=`, `>`, `>=`, `lebih_dari`, `kurang_dari`, `dalam` | Perbandingan |
| 6 | Additive | `+`, `-` | Penjumlahan & Pengurangan |
| 7 | Multiplicative | `*`, `/`, `%` | Perkalian, Pembagian, Modulo |
| 8 | Power | `**` | Pangkat (associative kanan) |
| 9 | Unary (prefix) | `-`, `bukan`, `!` | Negasi & Logika NOT |
| 10 | Postfix (akses) | `f(...)`, `a[i]`, `o.p` | Pemanggilan, Pengindeksan, Properti |
| 11 | Primary (paling tinggi) | Literal, Identifier, Fungsi Anonim | Elemen dasar |

### Definisi EBNF Presedensi

```
Expression       ::= Assignment

Assignment       ::= LogicalOr ("=" Assignment
                               | "+=" Assignment
                               | "-=" Assignment
                               | "*=" Assignment
                               | "/=" Assignment)?

LogicalOr        ::= LogicalAnd (("atau" | "||") LogicalAnd)*
LogicalAnd       ::= Equality (("dan" | "&&") Equality)*

Equality         ::= Relational (("==" | "!=" | "sama_dengan") Relational)*
Relational       ::= Addition (("<" | "<=" | ">" | ">=" | "lebih_dari" | "kurang_dari" | "dalam") Addition)*

Addition         ::= Multiplicative (("+" | "-") Multiplicative)*
Multiplicative   ::= Power (("*" | "/" | "%") Power)*
Power            ::= Unary ("**" Power)?  # Right-associative

Unary            ::= ("-" | "bukan" | "!") Unary
                   | Postfix

Postfix          ::= Primary ( "(" ArgumentList? ")"
                             | "[" Expression (":" Expression? (":" Expression?)?)? "]"
                             | "." Identifier )*

Primary          ::= Number
                   | String
                   | Boolean
                   | Null
                   | Identifier
                   | ArrayLiteral
                   | DictLiteral
                   | FuncExpr
                   | "(" Expression ")"

ArrayLiteral     ::= "[" (Expression ("," Expression)* ","?)? "]"
DictLiteral      ::= "{" (DictEntry ("," DictEntry)* ","?)? "}"
DictEntry        ::= Expression ":" Expression

FuncExpr         ::= "fungsi" "(" ParameterList? ")" Block

ArgumentList     ::= Expression ("," Expression)*
```

---

## 4. Tata Bahasa Tipe & Konversi

### 4.1 Literal Tipe Data
```
Literal    ::= Number | String | Boolean | Null | ArrayLiteral | DictLiteral
LiteralType ::= "angka" | "teks" | "bolean" | "kosong" | "daftar" | "kamus" | "fungsi" | "objek"
```

### 4.2 Fungsi Konversi Eksplisit
```
TypeCast ::= Identifier "(" Expression ")"
            # angka(expr), teks(expr), bolean(expr), daftar(expr)
```

---

## 5. Tata Bahasa Program Utama & Alur Eksekusi

### 5.1 Titik Masuk (Entry Point)
EvernightLanguage mendukung dua model eksekusi:
1. **Eksekusi Top-Down**: Seluruh pernyataan dalam file `.eve` dieksekusi dari atas ke bawah.
2. **Entry Point Fungsional**: Didefinisikan oleh fungsi `utama()` sebagai titik awal (opsional, untuk proyek besar).

```
ProgramStart ::= Statement* [FuncDeclaration]
```

### 5.2 Alur Eksekusi
1. Lexer memecah sumber menjadi token.
2. Parser membangun AST berdasarkan aturan di atas.
3. Compiler mengubah AST menjadi bytecode.
4. VM mengeksekusi bytecode.
5. Error ditangani sesuai format `BAHAYA [KODE]` atau `PERINGATAN [KODE]`.

---

## 6. Contoh Program Lengkap yang Valid

```eve
# Program Faktorial dengan Rekursi
fungsi faktorial(n) {
    jika n <= 1 {
        kembali 1
    } lainnya {
        kembali n * faktorial(n - 1)
    }
}

# Program Utama
fungsi utama() {
    variabel angka = 5
    variabel hasil = faktorial(angka)
    cetak("Faktorial dari " + teks(angka) + " adalah " + teks(hasil))
}

# Panggil fungsi utama jika ini file yang dijalankan
utama()
```

### Contoh Program dengan Error Handling
```eve
impor matematika sebagai mtk

fungsi bagi(a, b) {
    coba {
        pastikan(b != 0, "Pembagi tidak boleh nol!")
        kembali a / b
    } tangkap(err) {
        cetak("Error: " + err)
        kembali kosong
    }
}
```

### Contoh Program dengan Perulangan
```eve
fungsi hitung_ganjil(limit) {
    variabel i = 0
    selama i < limit {
        i = i + 1
        jika i % 2 == 0 {
            lanjut
        }
        cetak(i)
        jika i >= 10 {
            berhenti
        }
    }
}

hitung_ganjil(20)
```

---

## 7. Status Finalisasi

- [x] Tata bahasa leksikal terdefinisi (identifier, literal, komentar)
- [x] Semua pernyataan (statements) terdefinisi (deklarasi, percabangan, perulangan, error handling, modul)
- [x] Presedensi operator lengkap (11 level) dengan EBNF penuh
- [x] Contoh program valid untuk verifikasi
- [x] Alur eksekusi program terdokumentasi

---

## 8. Tabel Kata Kunci dalam Grammar

| Kelompok | Kata Kunci yang Digunakan dalam Grammar |
|----------|----------------------------------------|
| **Pernyataan** | `fungsi`, `kembali`, `jika`, `lainnya_jika`, `lainnya`, `selama`, `untuk`, `dari`, `sampai`, `dalam`, `berhenti`, `lanjut`, `cocok`, `kasus`, `bawaan`, `_`, `coba`, `tangkap`, `akhirnya`, `lempar`, `pastikan` |
| **Variabel** | `variabel`, `tetap` |
| **Tipe Data** | `angka`, `teks`, `bolean`, `benar`, `salah`, `kosong` |
| **Operator Logika** | `dan`, `atau`, `bukan`, `sama_dengan`, `lebih_dari`, `kurang_dari` |
| **Modul** | `impor`, `dari`, `sebagai` |
| **I/O** | `cetak`, `baca` |
| **Operator** | `+`, `-`, `*`, `/`, `%`, `**`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `=`, `+=`, `-=`, `*=`, `/=`, `&&`, `\|\|`, `!` |
| **Pemisah** | `(`, `)`, `{`, `}`, `[`, `]`, `,`, `:`, `.`, `;` |
