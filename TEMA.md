# TEMA.md — Spek Highlighting & Tema "Nusantara"

> Spesifikasi warna syntax highlighting untuk EvernightLanguage (Fase 4).
> Palet warna **final diputuskan user** (bagian `_`). Nama tema unik Indonesia: **Nusantara**.
> Bagian yang perlu diisi user ditandai dengan `_`.

---

## 1. Sumber Kebenaran (anti-drift)

- Grammar **generated dari `token.rs`** (`crates/evernight_core/src/token.rs`) — satu sumber keyword/operator.
- Pemetaan token → **TextMate scope** harus sinkron dengan daftar keyword di `KEYWORD.md`.
- File hasil: `editors/vscode/syntaxes/evernight.tmLanguage.json` (bisa digenerate via script/tooling kecil di `crates/evernight_cli` atau generator tersendiri).

## 2. Pemetaan Token → Scope (kerangka)

| Kategori | Contoh token Evernight | TextMate scope |
|----------|------------------------|----------------|
| Keyword kontrol | `jika`, `lainnya`, `selama`, `untuk`, `berhenti`, `lanjut`, `kembali`, `cocok`, `kasus`, `bawaan`, `coba`, `tangkap`, `akhirnya`, `lempar`, `pastikan`, `impor`, `dari`, `sebagai` | `keyword.control.eve` |
| Keyword deklarasi | `fungsi`, `variabel`, `tetap`, `kelas`, `objek` | `keyword.declaration.eve` |
| Literal boolean/null | `benar`, `salah`, `kosong`, `_` | `constant.language.eve` |
| String | `"..."`, `'...'`, template | `string.quoted.eve`, `string.interpolated.eve` |
| Angka | `123`, `3.14`, `0x`, `1e5` | `constant.numeric.eve` |
| Komentar | `# ...` (dan varian) | `comment.line.eve` |
| Nama fungsi (deklarasi) | setelah `fungsi` | `entity.name.function.eve` |
| Nama variabel (deklarasi) | setelah `variabel`/`tetap` | `variable.other.eve` |
| Parameter | di dalam `( )` fungsi | `variable.parameter.eve` |
| Operator | `+ - * / % == != < > <= >= && \|\| ! ` | `keyword.operator.eve` |
| Interpolasi/String | properti `.panjang`, pemanggilan method | `support.function.eve` / `entity.other.attribute-name.eve` |
| Pemisah sulit | `{ } ( ) [ ] , ; :` | `punctuation.*.eve` |
| Error/peringatan | `BAHAYA …`, `PERINGATAN …` pada output | `markup.bold` / scope terminal |

> Tabel ini sementara — akan dikunci saat ekstensi VS Code dibuat di Fase 4.

## 3. Palet "Nusantara" (draft — user yang pungut palet final)

Konsep: warna-warna yang nuansa Indonesia (alam tropis, kain, laut, gunung, malam).

### 3.1 Tema Gelap (default) — `nusantara-dark`

| Elemen | Waktu | Warna hex |
|--------|-------|-----------|
| Latar editor | `editor.background` | `#0b1020` |
| Teks umum | `foreground` | `#d4cfbe` |
| Keyword | keyword | `#0f9d8f` (toska laut) |
| String | string | `#7ec98f` (hijau tropis) |
| Angka | angka | `#f2a65a` (oranye batik) |
| Komentar | komentar | `#3d5a6a` (biru gelap, italic) |
| Fungsi | fungsi | `#e8c97a` (kuning keemasan) |
| Operator | operator | `#56b6c2` (biru toska) |
| Seleksi | `selectionBackground` | `#0f9d8f40` |
| Line highlight | `editor.lineHighlightBackground` | `#131a30` |

Palet rujukan:
- `#0b1020` (latar malam biru tua)
- Hijau tropis / toska laut `#0f9d8f` (rujukan)
- Oranye batik `#f2a65a` (rujukan)
- Merah cabai `#d93025` (rujukan — untuk error/BAHAYA)

### 3.2 Tema Terang — `nusantara-light`

| Elemen | Warna hex |
|--------|-----------|
| Latar editor | `#f5f0e8` (krem hangat) |
| Teks umum | `#2a2a2a` |
| Keyword | `#0b7a70` (toska tua) |
| String | `#2e7d52` (hijau hutan) |
| Angka | `#d9730a` (oranye tanah) |
| Komentar | `#a09070` (coklat muda, italic) |
| Fungsi | `#b5640a` (coklat oranye) |
| Operator | `#2a6db5` (biru langit) |

## 4. Konvensi Warna Error/Status CLI (Fase 4)

| Level | Arti | Warna terminal |
|-------|------|----------------|
| `BAHAYA [KODE]` | Error fatal | merah (`\x1b[31m`) |
| `PERINGATAN [KODE]` | Warning | kuning (`\x1b[33m`) |
| Info/`cetak()` | Output normal | default |
| Debug/`--debug` | Trace bytecode | abu (`\x1b[90m`) |

## 5. Snippet Dasar (draft for `editors/vscode/snippets/eve.json`)

| Prefix | Snippet |
|--------|---------|
| `fg` | `fungsi {Nama}(${1:param}) {` → `}` |
| `jk` | `jika (${1:kondisi}) { }` |
| `st` | `selama (${1:kondisi}) { }` |
| `utk` | `untuk ${1:i} dari ${2:1} sampai ${3:n} { }` |
| `cb` | `coba { } tangkap (${1:e}) { }` |
| `ck` | `cocok (${1:nilai}) { kasus ${2:_} { } }` |

## 6. Checklist Fase 4 (Highlighting)

1. [x] Pemetaan token→scope dimfinalkan (section 2)
2. [x] Grammar TextMate dari `token.rs` berfungsi
3. [x] `evernight.tmLanguage.json` di-generate + valid — `editors/vscode/syntaxes/`
4. [x] Palet gelap "Nusantara" final — `editors/vscode/themes/nusantara-dark.json`
5. [x] Palet terang final — `editors/vscode/themes/nusantara-light.json`
6. [x] Tema dimasukkan ke ekstensi VS Code (`package.json` `themes`)
7. [x] Snippet `.json` siap — `editors/vscode/snippets/eve.json` (23 snippet)
8. [ ] Snapshot test: contoh `.eve` di-highlight dengan scope yang benar
9. [ ] README ekstensi (cara pasang lokal)