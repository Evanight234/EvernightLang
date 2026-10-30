# GRAMMAR.md - Formal Grammar EvernightLanguage

> This document defines the formal (EBNF) grammar rules for EvernightLanguage.
> Every valid `.eve` program must obey the rules below.

---

## 1. Lexical Grammar

### 1.1 Identifier
```
Identifier ::= [a-zA-Z_][a-zA-Z0-9_]*
```

### 1.2 Number Literals
```
Number ::= [0-9]+ ("." [0-9]+)?
```

### 1.3 Text Literals
```
String ::= '"' ([^"\\] | EscapeSequence)* '"'
         | "'" ([^'\\] | EscapeSequence)* "'"

EscapeSequence ::= "\n" | "\t" | "\\\\" | "\\\"" | "\\'"
```

### 1.4 Boolean & Empty Literals
```
Boolean ::= "benar" | "salah"
Null     ::= "kosong"
```

### 1.5 Comments
```
Comment ::= "#" [^\n]*
```
Comments are ignored by the parser and are never executed.

---

## 2. Statement Grammar (Statements & Declarations)

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

### 2.3 Block
```
Block ::= "{" Statement* "}"
```

### 2.4 Variable Declaration
```
VarDeclaration ::= "variabel" Identifier ("=" Expression)?
```

### 2.5 Constant Declaration
```
ConstDeclaration ::= "tetap" Identifier "=" Expression
```

### 2.6 Function Declaration
```
FuncDeclaration ::= "fungsi" Identifier "(" ParameterList? ")" Block

ParameterList   ::= Parameter ("," Parameter)*
Parameter       ::= Identifier ("=" Expression)?
```

### 2.7 Branching (If-Else)
```
IfStatement  ::= "jika" Expression Block
                 ("lainnya_jika" Expression Block)*
                 ("lainnya" Block)?
```

### 2.8 Pattern Matching (Match/Switch)
```
MatchStatement   ::= "cocok" Expression "{" CaseClause* WildcardClause? DefaultClause? "}"
CaseClause       ::= "kasus" Expression Block
WildcardClause   ::= "kasus" "_" Block
DefaultClause    ::= "bawaan" Block
```

### 2.9 Loops
```
WhileStatement ::= "selama" Expression Block
ForStatement   ::= "untuk" Identifier "dari" Expression "sampai" Expression Block
                 | "untuk" Identifier "dalam" Expression Block
BreakStatement ::= "berhenti"
ContinueStatement ::= "lanjut"
```

### 2.10 Returning Values
```
ReturnStatement ::= "kembali" Expression?
```

### 2.11 Error Handling
```
TryCatchStatement ::= "coba" Block
                      "tangkap" "(" Identifier ")" Block
                      ("akhirnya" Block)?

ThrowStatement    ::= "lempar" Expression
AssertStatement   ::= "pastikan" "(" Expression ("," Expression)? ")"
```

### 2.12 Module System
```
ImportStatement ::= "impor" Identifier ("sebagai" Identifier)?
                 | "impor" ImportList "dari" (String | Identifier)

ImportList      ::= Identifier ("," Identifier)*
```
> **v1 Implementation (2026-09-10):** only the `impor "path/modul" [sebagai alias]`
> form is supported for cross-file `.eve` imports (module names may be
> identifiers or strings). The `impor ImportList dari ...` form is **not yet
> implemented** (deferred). Standard modules (`impor matematika`, etc.) remain
> no-op.

### 2.13 Expression Statement
```
ExprStatement ::= Expression
```

---

## 3. Expression Grammar

### Operator Precedence (lowest to highest)

| Level | Precedence | Operator | Description |
|-------|------------|----------|-------------|
| 1 | Assignment (lowest) | `=`, `+=`, `-=`, `*=`, `/=` | Assignment |
| 2 | Logical OR | `atau`, `\|\|` | Logical OR |
| 3 | Logical AND | `dan`, `&&` | Logical AND |
| 4 | Equality | `==`, `!=`, `sama_dengan` | Equality |
| 5 | Relational | `<`, `<=`, `>`, `>=`, `lebih_dari`, `kurang_dari`, `dalam` | Comparison |
| 6 | Additive | `+`, `-` | Addition & Subtraction |
| 7 | Multiplicative | `*`, `/`, `%` | Multiplication, Division, Modulo |
| 8 | Power | `**` | Power (right-associative) |
| 9 | Unary (prefix) | `-`, `bukan`, `!` | Negation & NOT logic |
| 10 | Postfix (access) | `f(...)`, `a[i]`, `o.p` | Calls, Indexing, Properties |
| 11 | Primary (highest) | Literal, Identifier, Anonymous Function | Basic elements |

### EBNF Precedence Definition

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

## 4. Type & Conversion Grammar

### 4.1 Data Type Literals
```
Literal    ::= Number | String | Boolean | Null | ArrayLiteral | DictLiteral
LiteralType ::= "angka" | "teks" | "bolean" | "kosong" | "daftar" | "kamus" | "fungsi" | "objek"
```

### 4.2 Explicit Conversion Functions
```
TypeCast ::= Identifier "(" Expression ")"
            # angka(expr), teks(expr), bolean(expr), daftar(expr)
```

---

## 5. Main Program Grammar & Execution Flow

### 5.1 Entry Point
EvernightLanguage supports two execution models:
1. **Top-Down Execution**: all statements in a `.eve` file execute from top to
   bottom.
2. **Functional Entry Point**: defined by the `utama()` function as the starting
   point (optional, for large projects).

```
ProgramStart ::= Statement* [FuncDeclaration]
```

### 5.2 Execution Flow
1. The lexer breaks the source into tokens.
2. The parser builds the AST from the rules above.
3. The compiler turns the AST into bytecode.
4. The VM executes the bytecode.
5. Errors are handled according to the `BAHAYA [KODE]` or `PERINGATAN [KODE]`
   format.

---

## 6. Complete Valid Program Examples

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

### Program Example with Error Handling
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

### Program Example with Loops
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

## 7. Finalization Status

- [x] Lexical grammar defined (identifiers, literals, comments)
- [x] All statements defined (declarations, branching, loops, error handling, modules)
- [x] Complete operator precedence (11 levels) with full EBNF
- [x] Valid program examples for verification
- [x] Program execution flow documented

---

## 8. Keyword Table in the Grammar

| Group | Keywords Used in the Grammar |
|-------|-------------------------------|
| **Statements** | `fungsi`, `kembali`, `jika`, `lainnya_jika`, `lainnya`, `selama`, `untuk`, `dari`, `sampai`, `dalam`, `berhenti`, `lanjut`, `cocok`, `kasus`, `bawaan`, `_`, `coba`, `tangkap`, `akhirnya`, `lempar`, `pastikan` |
| **Variables** | `variabel`, `tetap` |
| **Data Types** | `angka`, `teks`, `bolean`, `benar`, `salah`, `kosong` |
| **Logical Operators** | `dan`, `atau`, `bukan`, `sama_dengan`, `lebih_dari`, `kurang_dari` |
| **Modules** | `impor`, `dari`, `sebagai` |
| **I/O** | `cetak`, `baca` |
| **Operators** | `+`, `-`, `*`, `/`, `%`, `**`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `=`, `+=`, `-=`, `*=`, `/=`, `&&`, `\|\|`, `!` |
| **Separators** | `(`, `)`, `{`, `}`, `[`, `]`, `,`, `:`, `.`, `;` |
